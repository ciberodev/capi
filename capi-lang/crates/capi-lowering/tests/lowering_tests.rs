use capi_hir::{
    dump_hir, Hir, HirExprKind, HirId, HirItemKind, HirMemberKind, HirModulePath, HirPatternKind,
    HirStmtKind, HirTypeRefKind,
};
use capi_lexer::{Keyword, Operator};
use capi_lowering::lower_ast;
use capi_parser::parse;
use capi_source::{SourceId, SourceMap, Span};

struct Lowered {
    hir: Hir,
    ast_to_hir_entries: Vec<(Span, HirId)>,
    sources: SourceMap,
}

fn lower_valid(text: &str) -> Lowered {
    let mut sources = SourceMap::default();
    let source = sources.add_file("lowering-test.cap", text);
    let lexed = capi_lexer::lex(source, text);
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
    assert!(
        lowered.diagnostics().is_empty(),
        "unexpected lowering diagnostics: {:?}",
        lowered.diagnostics()
    );
    assert!(!lowered.blocked());
    let ast_to_hir_entries = lowered.ast_to_hir().entries().to_vec();
    let hir = lowered.hir().expect("HIR should be produced").clone();

    Lowered {
        hir,
        ast_to_hir_entries,
        sources,
    }
}

fn span_for(text: &str, needle: &str) -> (u32, u32) {
    let start = text
        .find(needle)
        .unwrap_or_else(|| panic!("expected source to contain {needle:?}"));
    (start as u32, (start + needle.len()) as u32)
}

#[test]
fn lowers_minimal_unit_with_implicit_module() {
    let lowered = lower_valid("");

    assert_eq!(lowered.hir.units.len(), 1);
    assert!(matches!(
        lowered.hir.units[0].module,
        HirModulePath::Implicit
    ));
    assert!(lowered.hir.units[0].imports.is_empty());
    assert!(lowered.hir.units[0].items.is_empty());
    assert!(lowered.hir.units[0].valid);
}

#[test]
fn lowers_module_imports_and_top_level_items() {
    let lowered = lower_valid(
        r#"module app.main;
import app.util;
import app.prelude.*;
const VERSION = 1;
let global = VERSION;
function main(value : Cliente) : Cliente { return value; }
class Cliente {}
interface Nomeavel {}
trait Logavel {}
"#,
    );

    let unit = &lowered.hir.units[0];
    assert!(matches!(unit.module, HirModulePath::Explicit(_)));
    assert_eq!(unit.imports.len(), 2);
    assert!(!lowered.hir.imports[0].wildcard);
    assert!(lowered.hir.imports[1].wildcard);
    assert_eq!(unit.items.len(), 6);
    assert!(matches!(lowered.hir.items[0].kind, HirItemKind::Const(_)));
    assert!(matches!(lowered.hir.items[1].kind, HirItemKind::Let(_)));
    assert!(matches!(
        lowered.hir.items[2].kind,
        HirItemKind::Function(_)
    ));
    assert!(matches!(lowered.hir.items[3].kind, HirItemKind::Class(_)));
    assert!(matches!(
        lowered.hir.items[4].kind,
        HirItemKind::Interface(_)
    ));
    assert!(matches!(lowered.hir.items[5].kind, HirItemKind::Trait(_)));
}

#[test]
fn lowers_class_members_modifiers_attributes_and_types() {
    let lowered = lower_valid(
        r#"@entity()
public class Cliente<T> extends Pessoa implements Nomeavel uses Logavel {
    private nome : String;
    const KIND : String = "cliente";
    let cached = nome;
    constructor(nome : String) { this.nome = nome; }
    function getNome() : String { return nome; }
}
"#,
    );

    let HirItemKind::Class(class) = &lowered.hir.items[0].kind else {
        panic!("expected class item");
    };
    assert_eq!(class.name.text, "Cliente");
    assert_eq!(class.generics[0].text, "T");
    assert!(class.extends.is_some());
    assert_eq!(class.implements.len(), 1);
    assert_eq!(class.uses.len(), 1);
    assert_eq!(class.modifiers[0].keyword, Keyword::Public);
    assert_eq!(class.attributes[0].path.segments[0].text, "entity");
    assert_eq!(class.members.len(), 5);

    assert!(matches!(
        lowered.hir.members[class.members[0].raw() as usize].kind,
        HirMemberKind::Field(_)
    ));
    assert!(matches!(
        lowered.hir.members[class.members[1].raw() as usize].kind,
        HirMemberKind::Const(_)
    ));
    assert!(matches!(
        lowered.hir.members[class.members[2].raw() as usize].kind,
        HirMemberKind::Let(_)
    ));
    assert!(matches!(
        lowered.hir.members[class.members[3].raw() as usize].kind,
        HirMemberKind::Constructor(_)
    ));
    assert!(matches!(
        lowered.hir.members[class.members[4].raw() as usize].kind,
        HirMemberKind::Method(_)
    ));
}

#[test]
fn lowers_statement_and_expression_forms() {
    let lowered = lower_valid(
        r#"function main() {
    let a = new Cliente(1);
    const b = [1, 2, 3];
    if (a) { call(a); } else if (b) { call(b); } else { call(0); }
    while (a) { break; }
    for (let i = 0; i; i = i + 1) { continue; }
    switch (a) { case 1: call(a); default: call(b); }
    match (a) { case Cliente(value): value; case _: b; }
    (a).member[0] = (a, b);
    -a + b;
}
"#,
    );

    assert!(lowered
        .hir
        .stmts
        .iter()
        .any(|stmt| matches!(stmt.kind, HirStmtKind::If { .. })));
    assert!(lowered
        .hir
        .stmts
        .iter()
        .any(|stmt| matches!(stmt.kind, HirStmtKind::While { .. })));
    assert!(lowered
        .hir
        .stmts
        .iter()
        .any(|stmt| matches!(stmt.kind, HirStmtKind::For { .. })));
    assert!(lowered
        .hir
        .stmts
        .iter()
        .any(|stmt| matches!(stmt.kind, HirStmtKind::Switch { .. })));
    assert!(lowered
        .hir
        .stmts
        .iter()
        .any(|stmt| matches!(stmt.kind, HirStmtKind::Match { .. })));
    assert!(lowered
        .hir
        .exprs
        .iter()
        .any(|expr| matches!(expr.kind, HirExprKind::New { .. })));
    assert!(lowered
        .hir
        .exprs
        .iter()
        .any(|expr| matches!(expr.kind, HirExprKind::Call { .. })));
    assert!(lowered
        .hir
        .exprs
        .iter()
        .any(|expr| matches!(expr.kind, HirExprKind::Member { .. })));
    assert!(lowered
        .hir
        .exprs
        .iter()
        .any(|expr| matches!(expr.kind, HirExprKind::Index { .. })));
    assert!(lowered.hir.exprs.iter().any(|expr| {
        matches!(
            expr.kind,
            HirExprKind::Binary {
                op: Operator::Plus,
                ..
            }
        )
    }));
    assert!(lowered
        .hir
        .exprs
        .iter()
        .any(|expr| matches!(expr.kind, HirExprKind::Tuple(_))));
    assert!(lowered
        .hir
        .patterns
        .iter()
        .any(|pattern| matches!(pattern.kind, HirPatternKind::Constructor { .. })));
    assert!(lowered
        .hir
        .patterns
        .iter()
        .any(|pattern| matches!(pattern.kind, HirPatternKind::Wildcard)));
}

#[test]
fn lowers_type_forms_without_resolution() {
    let lowered = lower_valid(
        r#"function f(
    a : Cliente,
    b : List<Map<String, Cliente>>,
    c : Int32[10],
    d : (String, Int32)
) : Optional<Cliente> {}
"#,
    );

    assert!(lowered
        .hir
        .type_refs
        .iter()
        .any(|ty| matches!(ty.kind, HirTypeRefKind::Path(_))));
    assert!(lowered
        .hir
        .type_refs
        .iter()
        .any(|ty| matches!(ty.kind, HirTypeRefKind::Generic { .. })));
    assert!(lowered
        .hir
        .type_refs
        .iter()
        .any(|ty| matches!(ty.kind, HirTypeRefKind::Array { .. })));
    assert!(lowered
        .hir
        .type_refs
        .iter()
        .any(|ty| matches!(ty.kind, HirTypeRefKind::Tuple(_))));
}

#[test]
fn preserves_origin_source_spans_and_ast_to_hir_map() {
    let source = "function main() { let value = 1; value; }";
    let lowered = lower_valid(source);
    let (start, end) = span_for(source, "let value = 1;");
    let local = lowered
        .hir
        .locals
        .iter()
        .find(|local| local.name.text == "value")
        .expect("local should exist");

    assert_eq!(local.origin.source, SourceId::from_raw(0));
    assert_eq!(
        (
            local.origin.span.start().raw(),
            local.origin.span.end().raw()
        ),
        (start, end)
    );
    assert_eq!(
        lowered.sources.span_text(local.origin.span),
        Some("let value = 1;")
    );
    assert!(lowered
        .ast_to_hir_entries
        .iter()
        .any(|(span, id)| *id == HirId::Local(local.id)
            && (span.start().raw(), span.end().raw()) == (start, end)));
}

#[test]
fn assigns_deterministic_typed_ids_and_textual_order() {
    let first = lower_valid("function a() {} function b() { let x = 1; let y = x; }");
    let second = lower_valid("function a() {} function b() { let x = 1; let y = x; }");

    assert_eq!(dump_hir(&first.hir), dump_hir(&second.hir));
    assert_eq!(first.hir.items[0].id.raw(), 0);
    assert_eq!(first.hir.items[1].id.raw(), 1);
    assert_eq!(first.hir.locals[0].id.raw(), 0);
    assert_eq!(first.hir.locals[1].id.raw(), 1);
    assert_eq!(first.hir.items[0].origin.span.start().raw(), 0);
    assert!(dump_hir(&first.hir).contains("Function id=0 name=a"));
    assert!(dump_hir(&first.hir).contains("Function id=1 name=b"));
    assert!(!dump_hir(&first.hir).contains("0x"));
}

#[test]
fn blocks_lowering_when_ast_contains_error_node() {
    let mut sources = SourceMap::default();
    let source = sources.add_file("invalid.cap", "function main() { let value = ; }");
    let lexed = capi_lexer::lex(source, sources.get(source).unwrap().text());
    let parsed = parse(source, lexed.tokens(), &sources);
    assert!(
        !parsed.diagnostics().is_empty(),
        "parser should produce syntax diagnostics"
    );

    let lowered = lower_ast(parsed.ast(), &sources);

    assert!(lowered.blocked());
    assert!(lowered.hir().is_none());
    assert!(lowered.diagnostics().iter().any(|diagnostic| diagnostic
        .code()
        .is_some_and(|code| code.as_str() == "HIR0001")));
    assert!(lowered
        .ast_to_hir()
        .entries()
        .iter()
        .any(|(_, id)| matches!(id, HirId::Error(_))));
}
