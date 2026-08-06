use std::fs;
use std::path::PathBuf;

use capi_diagnostics::{Diagnostic, LabelStyle, Severity};
use capi_hir::{dump_hir, Hir, HirExprKind, HirId, HirPatternKind, HirTypeRefKind};
use capi_lexer::lex;
use capi_lowering::lower_ast;
use capi_parser::parse;
use capi_sema::{
    analyze_names, check_types, dump_resolved_hir, Namespace, PrimitiveType, ResolvedBinding,
    ScopeKind, SemanticOutput, SymbolId, SymbolKind, TypeCheckOutput, TypeCheckState, TypeKind,
    TypeOrigin,
};
use capi_source::SourceMap;

struct SemanticFixture {
    hir: Hir,
    semantic: SemanticOutput,
}

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

fn analyze_fixture(path: &str) -> SemanticFixture {
    let text = fs::read_to_string(workspace_fixture(path)).expect("fixture should be readable");
    analyze_text(path, &text)
}

fn analyze_text(path: &str, text: &str) -> SemanticFixture {
    let mut sources = SourceMap::default();
    let source = sources.add_file(path, text);
    let lexed = lex(source, text);
    assert!(
        lexed.diagnostics().is_empty(),
        "unexpected lexer diagnostics: {:?}",
        lexed.diagnostics()
    );

    let parsed = parse(source, lexed.tokens(), &sources);
    assert!(
        parsed.diagnostics().is_empty(),
        "unexpected parser diagnostics: {:?}",
        parsed.diagnostics()
    );

    let lowered = lower_ast(parsed.ast(), &sources);
    let (hir, _ast_to_hir, diagnostics, blocked) = lowered.into_parts();
    assert!(!blocked, "lowering should not be blocked");
    assert!(
        diagnostics.is_empty(),
        "unexpected lowering diagnostics: {diagnostics:?}"
    );
    let hir = hir.expect("HIR should be produced");
    let semantic = analyze_names(&hir);
    SemanticFixture { hir, semantic }
}

fn check_text(path: &str, text: &str) -> TypeCheckOutput {
    let mut sources = SourceMap::default();
    let source = sources.add_file(path, text);
    let lexed = lex(source, text);
    assert!(
        lexed.diagnostics().is_empty(),
        "unexpected lexer diagnostics: {:?}",
        lexed.diagnostics()
    );

    let parsed = parse(source, lexed.tokens(), &sources);
    assert!(
        parsed.diagnostics().is_empty(),
        "unexpected parser diagnostics: {:?}",
        parsed.diagnostics()
    );

    let lowered = lower_ast(parsed.ast(), &sources);
    let (hir, _ast_to_hir, diagnostics, blocked) = lowered.into_parts();
    assert!(!blocked, "lowering should not be blocked");
    assert!(
        diagnostics.is_empty(),
        "unexpected lowering diagnostics: {diagnostics:?}"
    );
    let hir = hir.expect("HIR should be produced");
    check_types(&hir)
}

fn diagnostic_codes(diagnostics: &[Diagnostic]) -> Vec<&str> {
    diagnostics
        .iter()
        .filter_map(|diagnostic| diagnostic.code().map(|code| code.as_str()))
        .collect()
}

fn typecheck_codes(output: &TypeCheckOutput) -> Vec<&str> {
    diagnostic_codes(output.diagnostics())
}

fn assert_typecheck_has_code(output: &TypeCheckOutput, expected: &str) {
    let codes = typecheck_codes(output);
    assert!(
        codes.contains(&expected),
        "expected diagnostic code {expected}, got {codes:?}"
    );
}

fn assert_has_code(semantic: &SemanticOutput, expected: &str) {
    let codes = diagnostic_codes(semantic.diagnostics());
    assert!(
        codes.contains(&expected),
        "expected diagnostic code {expected}, got {codes:?}"
    );
}

fn diagnostics_with_code<'a>(semantic: &'a SemanticOutput, expected: &str) -> Vec<&'a Diagnostic> {
    semantic
        .diagnostics()
        .iter()
        .filter(|diagnostic| {
            diagnostic
                .code()
                .is_some_and(|code| code.as_str() == expected)
        })
        .collect()
}

fn assert_structured_semantic_diagnostic(semantic: &SemanticOutput, expected: &str) {
    let diagnostics = diagnostics_with_code(semantic, expected);
    assert!(
        !diagnostics.is_empty(),
        "expected diagnostic code {expected}, got {:?}",
        diagnostic_codes(semantic.diagnostics())
    );

    for diagnostic in diagnostics {
        assert_eq!(diagnostic.severity(), Severity::Error);
        let primary = diagnostic
            .primary_span()
            .expect("semantic diagnostic should have primary span");
        assert!(
            diagnostic
                .labels()
                .iter()
                .any(|label| label.span() == primary && label.style() == LabelStyle::Primary),
            "semantic diagnostic should label the primary span"
        );
    }
}

fn has_symbol(
    semantic: &SemanticOutput,
    name: &str,
    kind: SymbolKind,
    namespace: Namespace,
) -> bool {
    semantic
        .symbols()
        .symbols
        .iter()
        .any(|symbol| symbol.name == name && symbol.kind == kind && symbol.namespace == namespace)
}

fn symbol_id(
    semantic: &SemanticOutput,
    name: &str,
    kind: SymbolKind,
    namespace: Namespace,
) -> SymbolId {
    semantic
        .symbols()
        .symbols
        .iter()
        .find(|symbol| symbol.name == name && symbol.kind == kind && symbol.namespace == namespace)
        .map(|symbol| symbol.id)
        .unwrap_or_else(|| panic!("expected symbol `{name}` with kind {kind:?}"))
}

fn resolved_symbol_names(fixture: &SemanticFixture) -> Vec<&str> {
    fixture
        .semantic
        .bindings()
        .by_hir
        .values()
        .filter_map(|binding| match binding {
            ResolvedBinding::Symbol(symbol) => Some(
                fixture.semantic.symbols().symbols[symbol.raw() as usize]
                    .name
                    .as_str(),
            ),
            _ => None,
        })
        .collect()
}

fn resolvable_hir_ids(hir: &Hir) -> Vec<HirId> {
    let mut ids = Vec::new();

    ids.extend(hir.imports.iter().map(|import| HirId::Import(import.id)));

    ids.extend(hir.exprs.iter().filter_map(|expr| match &expr.kind {
        HirExprKind::Path(_) => Some(HirId::Expr(expr.id)),
        _ => None,
    }));

    ids.extend(hir.type_refs.iter().filter_map(|ty| match &ty.kind {
        HirTypeRefKind::Path(_) | HirTypeRefKind::Generic { .. } => Some(HirId::TypeRef(ty.id)),
        _ => None,
    }));

    ids.extend(
        hir.patterns
            .iter()
            .filter_map(|pattern| match &pattern.kind {
                HirPatternKind::Constructor { .. } => Some(HirId::Pattern(pattern.id)),
                HirPatternKind::Path(path) if path.segments.len() > 1 => {
                    Some(HirId::Pattern(pattern.id))
                }
                _ => None,
            }),
    );

    ids.sort();
    ids.dedup();
    ids
}

#[test]
fn accepts_stage3_semantic_pass_fixtures() {
    for path in [
        "tests/semantic/pass/basic.cap",
        "tests/semantic/pass/scopes-and-symbols.cap",
        "tests/semantic/pass/patterns.cap",
    ] {
        let fixture = analyze_fixture(path);

        assert!(
            fixture.semantic.diagnostics().is_empty(),
            "{path} should not produce semantic diagnostics: {:?}",
            fixture.semantic.diagnostics()
        );
        assert!(
            fixture
                .semantic
                .symbols()
                .symbols
                .iter()
                .any(|symbol| symbol.name == "main" && symbol.namespace == Namespace::Value),
            "{path} should declare function main in value namespace"
        );
        assert!(
            fixture
                .semantic
                .bindings()
                .by_hir
                .values()
                .any(|binding| matches!(binding, ResolvedBinding::Symbol(_))),
            "{path} should resolve at least one name binding"
        );
    }
}

#[test]
fn resolves_all_names_in_initial_subset_pass_fixtures() {
    for path in [
        "tests/semantic/pass/basic.cap",
        "tests/semantic/pass/scopes-and-symbols.cap",
        "tests/semantic/pass/patterns.cap",
    ] {
        let fixture = analyze_fixture(path);
        assert!(
            fixture.semantic.diagnostics().is_empty(),
            "{path} should not produce semantic diagnostics: {:?}",
            fixture.semantic.diagnostics()
        );

        for id in resolvable_hir_ids(&fixture.hir) {
            assert!(
                matches!(
                    fixture.semantic.bindings().by_hir.get(&id),
                    Some(ResolvedBinding::Symbol(_))
                ),
                "{path} should resolve {id:?}, got {:?}",
                fixture.semantic.bindings().by_hir.get(&id)
            );
        }
    }
}

#[test]
fn rejects_stage3_semantic_fail_fixtures() {
    for (path, code) in [
        ("tests/semantic/fail/duplicate-function.cap", "SEM0001"),
        ("tests/semantic/fail/duplicate-class.cap", "SEM0001"),
        ("tests/semantic/fail/duplicate-param.cap", "SEM0001"),
        ("tests/semantic/fail/duplicate-local.cap", "SEM0001"),
        ("tests/semantic/fail/duplicate-field.cap", "SEM0001"),
        ("tests/semantic/fail/duplicate-method.cap", "SEM0001"),
        (
            "tests/semantic/fail/duplicate-pattern-binding.cap",
            "SEM0001",
        ),
        ("tests/semantic/fail/unresolved-reference.cap", "SEM0002"),
        ("tests/semantic/fail/unresolved-type.cap", "SEM0002"),
        ("tests/semantic/fail/namespace-mismatch.cap", "SEM0002"),
        ("tests/semantic/fail/ambiguous-reference.cap", "SEM0003"),
    ] {
        let fixture = analyze_fixture(path);

        assert_has_code(&fixture.semantic, code);
    }
}

#[test]
fn builds_scope_graph_for_stage3_constructs() {
    let fixture = analyze_fixture("tests/semantic/pass/scopes-and-symbols.cap");
    let scopes = fixture.semantic.scopes();

    assert_eq!(
        scopes.scopes[scopes.root.raw() as usize].kind,
        ScopeKind::Global
    );
    assert!(scopes
        .scopes
        .iter()
        .any(|scope| scope.kind == ScopeKind::Module));
    assert!(scopes
        .scopes
        .iter()
        .any(|scope| scope.kind == ScopeKind::Type));
    assert!(scopes
        .scopes
        .iter()
        .any(|scope| scope.kind == ScopeKind::Member));
    assert!(scopes
        .scopes
        .iter()
        .any(|scope| scope.kind == ScopeKind::Function));
    assert!(scopes
        .scopes
        .iter()
        .any(|scope| scope.kind == ScopeKind::Constructor));
    assert!(scopes
        .scopes
        .iter()
        .any(|scope| scope.kind == ScopeKind::Block));
    assert!(scopes
        .scopes
        .iter()
        .all(|scope| scope.id.raw() as usize <= scopes.scopes.len()));
    assert!(
        scopes
            .scopes
            .iter()
            .filter(|scope| scope.kind == ScopeKind::Block)
            .count()
            >= 6
    );
}

#[test]
fn registers_symbols_for_stage3_declarations() {
    let fixture = analyze_fixture("tests/semantic/pass/scopes-and-symbols.cap");
    let semantic = &fixture.semantic;

    assert!(has_symbol(
        semantic,
        "util",
        SymbolKind::Import,
        Namespace::Module
    ));
    assert!(has_symbol(
        semantic,
        "Text",
        SymbolKind::Class,
        Namespace::Type
    ));
    assert!(has_symbol(
        semantic,
        "Cliente",
        SymbolKind::Class,
        Namespace::Type
    ));
    assert!(has_symbol(
        semantic,
        "Nomeavel",
        SymbolKind::Interface,
        Namespace::Type
    ));
    assert!(has_symbol(
        semantic,
        "Logavel",
        SymbolKind::Trait,
        Namespace::Type
    ));
    assert!(has_symbol(
        semantic,
        "nome",
        SymbolKind::Field,
        Namespace::Member
    ));
    assert!(has_symbol(
        semantic,
        "getNome",
        SymbolKind::Method,
        Namespace::Member
    ));
    assert!(has_symbol(
        semantic,
        "constructor",
        SymbolKind::Constructor,
        Namespace::Member
    ));
    assert!(has_symbol(
        semantic,
        "DEFAULT",
        SymbolKind::Const,
        Namespace::Value
    ));
    assert!(has_symbol(
        semantic,
        "param",
        SymbolKind::Param,
        Namespace::Value
    ));
    assert!(has_symbol(
        semantic,
        "globalValue",
        SymbolKind::Local,
        Namespace::Value
    ));
}

#[test]
fn registers_pattern_bindings() {
    let fixture = analyze_fixture("tests/semantic/pass/patterns.cap");

    assert!(has_symbol(
        &fixture.semantic,
        "bound",
        SymbolKind::PatternBinding,
        Namespace::Value
    ));
    assert!(resolved_symbol_names(&fixture).contains(&"bound"));
}

#[test]
fn resolves_values_types_functions_and_shadowing() {
    let fixture = analyze_fixture("tests/semantic/pass/scopes-and-symbols.cap");
    let names = resolved_symbol_names(&fixture);

    assert!(names.contains(&"Cliente"));
    assert!(names.contains(&"Text"));
    assert!(names.contains(&"DEFAULT"));
    assert!(names.contains(&"helper"));
    assert!(names.contains(&"param"));
    assert!(names.contains(&"globalValue"));
    assert!(names.contains(&"inner"));
}

#[test]
fn records_ambiguous_bindings_for_conflicting_symbols() {
    let fixture = analyze_fixture("tests/semantic/fail/ambiguous-reference.cap");

    assert!(
        fixture.semantic.bindings().by_hir.values().any(
            |binding| matches!(binding, ResolvedBinding::Ambiguous(symbols) if symbols.len() == 2)
        ),
        "ambiguous reference should retain both candidate symbols"
    );
}

#[test]
fn reports_structured_semantic_diagnostics() {
    for (path, code) in [
        ("tests/semantic/fail/duplicate-local.cap", "SEM0001"),
        ("tests/semantic/fail/unresolved-reference.cap", "SEM0002"),
        ("tests/semantic/fail/ambiguous-reference.cap", "SEM0003"),
    ] {
        let fixture = analyze_fixture(path);
        assert_structured_semantic_diagnostic(&fixture.semantic, code);
    }

    let duplicate = analyze_fixture("tests/semantic/fail/duplicate-local.cap");
    let duplicate_diagnostic = diagnostics_with_code(&duplicate.semantic, "SEM0001")[0];
    assert!(
        duplicate_diagnostic
            .labels()
            .iter()
            .any(|label| label.style() == LabelStyle::Secondary),
        "duplicate diagnostic should include previous declaration label"
    );
}

#[test]
fn semantic_diagnostics_are_deterministic() {
    let first = analyze_fixture("tests/semantic/fail/ambiguous-reference.cap");
    let second = analyze_fixture("tests/semantic/fail/ambiguous-reference.cap");
    let first_shape = first
        .semantic
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            (
                diagnostic.code().map(|code| code.as_str()).unwrap_or(""),
                diagnostic.message(),
                diagnostic
                    .primary_span()
                    .map(|span| (span.start().raw(), span.end().raw())),
            )
        })
        .collect::<Vec<_>>();
    let second_shape = second
        .semantic
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            (
                diagnostic.code().map(|code| code.as_str()).unwrap_or(""),
                diagnostic.message(),
                diagnostic
                    .primary_span()
                    .map(|span| (span.start().raw(), span.end().raw())),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(first_shape, second_shape);
}

#[test]
fn hir_dump_is_stable_for_basic_fixture() {
    let fixture = analyze_fixture("tests/semantic/pass/basic.cap");

    assert_eq!(
        dump_hir(&fixture.hir),
        include_str!("../../../tests/semantic/snapshots/basic.initial-hir.snap")
    );
    assert_eq!(
        dump_resolved_hir(&fixture.hir, &fixture.semantic),
        include_str!("../../../tests/semantic/snapshots/basic.hir.snap")
    );
    assert!(!dump_hir(&fixture.hir).contains("0x"));
}

#[test]
fn stage4_infers_local_and_literal_types() {
    let output = check_text(
        "stage4-infer.cap",
        r#"
        function main() {
            let value = 1;
            value;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    let int = output.interner().builtins().int;
    assert!(
        output.types().locals.values().any(|ty| *ty == int),
        "local initializer should infer Int"
    );
    assert!(output
        .interner()
        .types()
        .iter()
        .any(|info| matches!(info.kind, TypeKind::Primitive(PrimitiveType::Int))));
}

#[test]
fn stage4_reports_type_mismatch() {
    let output = check_text(
        "stage4-mismatch.cap",
        r#"
        function main() {
            let value : Bool = 1;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    let codes = typecheck_codes(&output);
    assert!(
        codes.contains(&"TYPE0003"),
        "expected TYPE0003, got {codes:?}"
    );
    assert_eq!(
        codes.iter().filter(|code| **code == "TYPE0003").count(),
        1,
        "local initializer mismatch should produce a single TYPE0003"
    );
}

#[test]
fn stage4_reports_incompatible_assignment() {
    let output = check_text(
        "stage4-assignment-mismatch.cap",
        r#"
        function main() {
            let value : Int = 1;
            value = true;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0003");
}

#[test]
fn stage4_reports_structured_type_diagnostics() {
    let output = check_text(
        "stage4-structured-type-diagnostic.cap",
        r#"
        function main() {
            let value : Bool = 1;
        }
        "#,
    );

    let diagnostic = output
        .diagnostics()
        .iter()
        .find(|diagnostic| {
            diagnostic
                .code()
                .is_some_and(|code| code.as_str() == "TYPE0003")
        })
        .expect("expected TYPE0003");
    assert_eq!(diagnostic.severity(), Severity::Error);
    let primary = diagnostic
        .primary_span()
        .expect("type diagnostic should have primary span");
    assert!(diagnostic
        .labels()
        .iter()
        .any(|label| label.span() == primary && label.style() == LabelStyle::Primary));
}

#[test]
fn stage4_rejects_null_as_type_name() {
    let output = check_text(
        "stage4-null-type.cap",
        r#"
        function main(value : null) {
            value;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "SEM0002");
    assert_typecheck_has_code(&output, "TYPE0002");
}

#[test]
fn stage4_rejects_value_symbol_used_as_type() {
    let output = check_text(
        "stage4-value-as-type.cap",
        r#"
        function helper() {}

        function main(value : helper) {
            value;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "SEM0002");
    assert_typecheck_has_code(&output, "TYPE0002");
}

#[test]
fn stage4_resolves_calls_and_argument_types() {
    let output = check_text(
        "stage4-call.cap",
        r#"
        function id(value : Int) : Int {
            return value;
        }

        function main() {
            let result = id(1);
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    assert_eq!(output.calls().calls.len(), 1);
}

#[test]
fn stage4_accepts_reflexive_subtyping_without_coercion() {
    let output = check_text(
        "stage4-reflexive-subtyping.cap",
        r#"
        class Animal {}

        function take(value : Animal) : Animal {
            return value;
        }

        function main(animal : Animal) {
            take(animal);
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    assert!(
        output.coercions().by_hir.is_empty(),
        "same-type argument should not need a coercion"
    );
}

#[test]
fn stage4_records_upcast_coercions() {
    let output = check_text(
        "stage4-upcast.cap",
        r#"
        class Animal {}
        class Dog extends Animal {}

        function take(animal : Animal) : Animal {
            return animal;
        }

        function main(dog : Dog) {
            take(dog);
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    assert!(
        !output.coercions().by_hir.is_empty(),
        "upcast argument should be registered as a coercion"
    );
}

#[test]
fn stage4_rejects_generic_application_with_missing_argument() {
    let output = check_text(
        "stage4-generics-missing-argument.cap",
        r#"
        class Box<T> {}

        function main(value : Box) {
            value;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0008");
}

#[test]
fn stage4_reports_invalid_generic_arity() {
    let output = check_text(
        "stage4-generics.cap",
        r#"
        class Box<T> {}

        function main(value : Box<Int, Bool>) {
            value;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    let codes = typecheck_codes(&output);
    assert!(
        codes.contains(&"TYPE0008"),
        "expected TYPE0008, got {codes:?}"
    );
}

#[test]
fn stage4_registers_builtin_types_deterministically() {
    let first = check_text("stage4-builtins-a.cap", "function main() {}");
    let second = check_text("stage4-builtins-b.cap", "function main() {}");

    assert_eq!(first.state(), TypeCheckState::Checked);
    assert_eq!(second.state(), TypeCheckState::Checked);

    let first_builtins = first.interner().builtins();
    let second_builtins = second.interner().builtins();
    assert_eq!(first_builtins, second_builtins);
    assert_eq!(
        first.interner().get(first_builtins.bool_).kind,
        TypeKind::Primitive(PrimitiveType::Bool)
    );
    assert_eq!(
        first.interner().get(first_builtins.int).kind,
        TypeKind::Primitive(PrimitiveType::Int)
    );
    assert_eq!(
        first.interner().get(first_builtins.unit).kind,
        TypeKind::Unit
    );
    assert_eq!(
        first.interner().get(first_builtins.error).kind,
        TypeKind::Error
    );
}

#[test]
fn stage4_preserves_explicit_local_type() {
    let output = check_text(
        "stage4-explicit-local.cap",
        r#"
        function main() {
            let value : Int = 1;
            value;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    assert!(output
        .types()
        .locals
        .values()
        .all(|ty| *ty == output.interner().builtins().int));
}

#[test]
fn stage4_shares_type_id_for_repeated_builtin_type_refs() {
    let output = check_text(
        "stage4-repeated-int.cap",
        r#"
        function add(left : Int, right : Int) : Int {
            return left;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    let int = output.interner().builtins().int;
    let int_refs = output
        .types()
        .type_refs
        .values()
        .filter(|ty| **ty == int)
        .count();
    assert_eq!(int_refs, 3);
}

#[test]
fn stage4_keeps_distinct_nominal_types_separate() {
    let output = check_text(
        "stage4-distinct-nominal.cap",
        r#"
        class Left {}
        class Right {}

        function main(left : Left, right : Right) {
            left;
            right;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    let left = symbol_id(
        output.semantic(),
        "Left",
        SymbolKind::Class,
        Namespace::Type,
    );
    let right = symbol_id(
        output.semantic(),
        "Right",
        SymbolKind::Class,
        Namespace::Type,
    );
    assert_ne!(
        output.types().symbol_types[&left],
        output.types().symbol_types[&right]
    );
}

#[test]
fn stage4_infers_tuple_and_array_types() {
    let output = check_text(
        "stage4-composite-infer.cap",
        r#"
        function main() {
            let pair = (1, true);
            let values = [1, 2];
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    let int = output.interner().builtins().int;
    let bool_ = output.interner().builtins().bool_;
    assert!(output.interner().types().iter().any(
        |info| matches!(&info.kind, TypeKind::Tuple(elements) if elements == &vec![int, bool_])
    ));
    assert!(output
        .interner()
        .types()
        .iter()
        .any(|info| matches!(info.kind, TypeKind::Array(element) if element == int)));
}

#[test]
fn stage4_interns_equivalent_tuples_once() {
    let output = check_text(
        "stage4-equivalent-tuples.cap",
        r#"
        function main() {
            let first = (1, true);
            let second = (2, false);
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    let tuple_count = output
        .interner()
        .types()
        .iter()
        .filter(|info| matches!(info.kind, TypeKind::Tuple(_)))
        .count();
    assert_eq!(tuple_count, 1);
}

#[test]
fn stage4_keeps_tuple_order_significant() {
    let output = check_text(
        "stage4-tuple-order.cap",
        r#"
        function main() {
            let first = (1, true);
            let second = (false, 2);
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    let tuple_count = output
        .interner()
        .types()
        .iter()
        .filter(|info| matches!(info.kind, TypeKind::Tuple(_)))
        .count();
    assert_eq!(tuple_count, 2);
}

#[test]
fn stage4_interns_equivalent_function_signatures_once() {
    let output = check_text(
        "stage4-equivalent-signatures.cap",
        r#"
        function first(value : Int) : Int {
            return value;
        }

        function second(value : Int) : Int {
            return value;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    let function_count = output
        .interner()
        .types()
        .iter()
        .filter(|info| matches!(info.kind, TypeKind::Function(_)))
        .count();
    assert_eq!(function_count, 1);
}

#[test]
fn stage4_records_type_properties_and_origins() {
    let output = check_text(
        "stage4-properties-origins.cap",
        r#"
        class Account {}

        function main(account : Account) {
            let value = 1;
            account;
            value;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    let account = symbol_id(
        output.semantic(),
        "Account",
        SymbolKind::Class,
        Namespace::Type,
    );
    let account_ty = output.types().symbol_types[&account];
    let account_info = output.interner().get(account_ty);
    assert_eq!(account_info.origin, TypeOrigin::Declared(account));
    assert!(account_info.properties.has_identity);
    assert!(!account_info.properties.copyable);

    let int_info = output.interner().get(output.interner().builtins().int);
    assert_eq!(int_info.origin, TypeOrigin::Builtin);
    assert!(!int_info.properties.has_identity);
    assert!(int_info.properties.copyable);
}

#[test]
fn stage4_maps_type_refs_through_symbols_to_type_ids() {
    let output = check_text(
        "stage4-type-ref-map.cap",
        r#"
        class Account {}

        function main(account : Account) : Account {
            return account;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    let account = symbol_id(
        output.semantic(),
        "Account",
        SymbolKind::Class,
        Namespace::Type,
    );
    let account_ty = output.types().symbol_types[&account];
    let mapped_refs = output
        .semantic()
        .bindings()
        .by_hir
        .iter()
        .filter_map(|(hir_id, binding)| match (hir_id, binding) {
            (HirId::TypeRef(type_ref), ResolvedBinding::Symbol(symbol)) if *symbol == account => {
                Some(type_ref)
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(mapped_refs.len(), 2);
    assert!(mapped_refs
        .iter()
        .all(|type_ref| output.types().type_refs[type_ref] == account_ty));
}

#[test]
fn stage4_reports_empty_array_inference_failure() {
    let output = check_text(
        "stage4-empty-array.cap",
        r#"
        function main() {
            let values = [];
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0001");
}

#[test]
fn stage4_accepts_compatible_assignment() {
    let output = check_text(
        "stage4-assignment-ok.cap",
        r#"
        function main() {
            let value : Int = 1;
            value = 2;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    assert!(output
        .types()
        .exprs
        .values()
        .any(|ty| *ty == output.interner().builtins().unit));
}

#[test]
fn stage4_materializes_unit_for_empty_blocks() {
    let output = check_text(
        "stage4-empty-block.cap",
        r#"
        function main() {
            {}
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    assert!(output
        .types()
        .stmts
        .values()
        .all(|ty| *ty == output.interner().builtins().unit));
}

#[test]
fn stage4_does_not_leave_unknown_types_in_checked_program() {
    let output = check_text(
        "stage4-no-unknown.cap",
        r#"
        class Account {}

        function id(value : Int) : Int {
            return value;
        }

        function main(account : Account) {
            let value = id(1);
            let pair = (value, true);
            account;
            pair;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    assert!(!output
        .interner()
        .types()
        .iter()
        .any(|info| matches!(info.kind, TypeKind::Unknown(_))));
}

#[test]
fn stage4_checks_return_compatibility() {
    let output = check_text(
        "stage4-return-ok.cap",
        r#"
        function id(value : Int) : Int {
            return value;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
}

#[test]
fn stage4_blocks_when_semantic_input_is_ambiguous() {
    let output = check_text(
        "stage4-blocked.cap",
        r#"
        function main() {}
        function main() {}
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Blocked);
    assert_typecheck_has_code(&output, "SEM0001");
}

#[test]
fn stage4_type_diagnostics_are_deterministic() {
    let first = check_text(
        "stage4-deterministic-a.cap",
        r#"
        function main() : Bool {
            return 1;
        }
        "#,
    );
    let second = check_text(
        "stage4-deterministic-b.cap",
        r#"
        function main() : Bool {
            return 1;
        }
        "#,
    );
    let first_shape = first
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            (
                diagnostic.code().map(|code| code.as_str()).unwrap_or(""),
                diagnostic.message(),
                diagnostic
                    .primary_span()
                    .map(|span| (span.start().raw(), span.end().raw())),
            )
        })
        .collect::<Vec<_>>();
    let second_shape = second
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            (
                diagnostic.code().map(|code| code.as_str()).unwrap_or(""),
                diagnostic.message(),
                diagnostic
                    .primary_span()
                    .map(|span| (span.start().raw(), span.end().raw())),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(first.state(), TypeCheckState::CheckedWithErrors);
    assert_eq!(first_shape, second_shape);
}

#[test]
fn stage4_reports_return_mismatch() {
    let output = check_text(
        "stage4-return-mismatch.cap",
        r#"
        function main() : Bool {
            return 1;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0003");
}

#[test]
fn stage4_reports_unit_return_mismatch() {
    let output = check_text(
        "stage4-unit-return-mismatch.cap",
        r#"
        function main() : Int {
            return;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0003");
}

#[test]
fn stage4_requires_boolean_control_conditions() {
    let output = check_text(
        "stage4-control-condition.cap",
        r#"
        function main() {
            if (1) {
                return;
            }

            while (1) {
                return;
            }
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    let codes = typecheck_codes(&output);
    assert!(
        codes.iter().filter(|code| **code == "TYPE0003").count() >= 2,
        "expected TYPE0003 for if and while conditions, got {codes:?}"
    );
}

#[test]
fn stage4_reports_call_arity_mismatch() {
    let output = check_text(
        "stage4-call-arity.cap",
        r#"
        function id(value : Int) : Int {
            return value;
        }

        function main() {
            id();
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0005");
}

#[test]
fn stage4_reports_call_argument_mismatch() {
    let output = check_text(
        "stage4-call-argument.cap",
        r#"
        function id(value : Int) : Int {
            return value;
        }

        function main() {
            id(true);
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0003");
}

#[test]
fn stage4_rejects_non_callable_callee() {
    let output = check_text(
        "stage4-non-callable.cap",
        r#"
        function main() {
            let value = 1;
            value();
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0005");
}

#[test]
fn stage4_accepts_transitive_class_upcast() {
    let output = check_text(
        "stage4-transitive-upcast.cap",
        r#"
        class Living {}
        class Animal extends Living {}
        class Dog extends Animal {}

        function take(value : Living) : Living {
            return value;
        }

        function main(dog : Dog) {
            take(dog);
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    assert!(
        !output.coercions().by_hir.is_empty(),
        "transitive upcast should be registered"
    );
}

#[test]
fn stage4_rejects_incompatible_nominal_types() {
    let output = check_text(
        "stage4-incompatible-nominal.cap",
        r#"
        class Cat {}
        class Dog {}

        function take(cat : Cat) {
            return;
        }

        function main(dog : Dog) {
            take(dog);
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0003");
}

#[test]
fn stage4_rejects_implicit_downcast() {
    let output = check_text(
        "stage4-downcast.cap",
        r#"
        class Animal {}
        class Dog extends Animal {}

        function take(value : Dog) {
            return;
        }

        function main(animal : Animal) {
            take(animal);
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0003");
    assert!(
        output.coercions().by_hir.is_empty(),
        "implicit downcast must not register an upcast coercion"
    );
}

#[test]
fn stage4_accepts_class_as_implemented_interface() {
    let output = check_text(
        "stage4-interface-upcast.cap",
        r#"
        interface Pet {}
        class Dog implements Pet {}

        function take(value : Pet) : Pet {
            return value;
        }

        function main(dog : Dog) {
            take(dog);
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    assert!(
        !output.coercions().by_hir.is_empty(),
        "interface upcast should be registered"
    );
}

#[test]
fn stage4_rejects_class_as_unimplemented_interface() {
    let output = check_text(
        "stage4-interface-mismatch.cap",
        r#"
        interface Pet {}
        class Dog {}

        function take(value : Pet) {
            return;
        }

        function main(dog : Dog) {
            take(dog);
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0003");
}

#[test]
fn stage4_rejects_implicit_numeric_coercion() {
    let output = check_text(
        "stage4-no-numeric-coercion.cap",
        r#"
        function main() {
            let value : Double = 1;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0003");
}

#[test]
fn stage4_accepts_generic_application_with_correct_arity() {
    let output = check_text(
        "stage4-generics-ok.cap",
        r#"
        class Box<T> {}

        function main(value : Box<Int>) {
            value;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    let int = output.interner().builtins().int;
    assert!(output.interner().types().iter().any(
        |info| matches!(&info.kind, TypeKind::GenericInstance { args, .. } if args == &vec![int])
    ));
}

#[test]
fn stage4_distinguishes_generic_instances_by_arguments() {
    let output = check_text(
        "stage4-generics-distinct.cap",
        r#"
        class Box<T> {}

        function main(left : Box<Int>, right : Box<Bool>) {
            left;
            right;
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::Checked);
    assert!(
        output.diagnostics().is_empty(),
        "{:?}",
        output.diagnostics()
    );
    let instances = output
        .interner()
        .types()
        .iter()
        .filter(|info| matches!(info.kind, TypeKind::GenericInstance { .. }))
        .count();
    assert_eq!(instances, 2);
}

#[test]
fn stage4_rejects_generic_invariance_violation() {
    let output = check_text(
        "stage4-generics-invariant.cap",
        r#"
        class Animal {}
        class Dog extends Animal {}
        class Box<T> {}

        function take(value : Box<Animal>) {
            return;
        }

        function main(value : Box<Dog>) {
            take(value);
        }
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0003");
}

#[test]
fn stage4_reports_duplicate_generic_parameter() {
    let output = check_text(
        "stage4-duplicate-generic.cap",
        r#"
        class Box<T, T> {}
        "#,
    );

    assert_eq!(output.state(), TypeCheckState::CheckedWithErrors);
    assert_typecheck_has_code(&output, "TYPE0008");
}
