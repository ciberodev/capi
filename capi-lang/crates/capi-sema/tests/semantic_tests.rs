use std::fs;
use std::path::PathBuf;

use capi_diagnostics::{Diagnostic, LabelStyle, Severity};
use capi_hir::{dump_hir, Hir, HirExprKind, HirId, HirPatternKind, HirTypeRefKind};
use capi_lexer::lex;
use capi_lowering::lower_ast;
use capi_parser::parse;
use capi_sema::{
    analyze_names, dump_resolved_hir, Namespace, ResolvedBinding, ScopeKind, SemanticOutput,
    SymbolKind,
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

fn diagnostic_codes(diagnostics: &[Diagnostic]) -> Vec<&str> {
    diagnostics
        .iter()
        .filter_map(|diagnostic| diagnostic.code().map(|code| code.as_str()))
        .collect()
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
