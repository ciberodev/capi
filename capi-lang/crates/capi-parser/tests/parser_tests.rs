use capi_ast::{dump_ast, Decl, Expr, MemberDecl, Stmt, TypeSyntax};
use capi_lexer::{lex, Operator};
use capi_parser::{parse, ParseOutput};
use capi_source::SourceMap;

struct Parsed {
    output: ParseOutput,
    sources: SourceMap,
    text: String,
}

impl Parsed {
    fn ast_dump(&self) -> String {
        dump_ast(self.output.ast(), &self.sources)
    }

    fn span_for(&self, needle: &str) -> (u32, u32) {
        let start = self
            .text
            .find(needle)
            .unwrap_or_else(|| panic!("expected source to contain {needle:?}"));
        let end = start + needle.len();
        (start as u32, end as u32)
    }

    fn span_for_after(&self, after: &str, needle: &str) -> (u32, u32) {
        let base = self
            .text
            .find(after)
            .unwrap_or_else(|| panic!("expected source to contain {after:?}"));
        let relative = self.text[base..]
            .find(needle)
            .unwrap_or_else(|| panic!("expected source after {after:?} to contain {needle:?}"));
        let start = base + relative;
        let end = start + needle.len();
        (start as u32, end as u32)
    }
}

fn parse_text(text: &str) -> Parsed {
    let mut sources = SourceMap::default();
    let source = sources.add_file("parser-test.cap", text);
    let lexed = lex(source, text);
    assert!(
        lexed.diagnostics().is_empty(),
        "unexpected lexer diagnostics: {:?}",
        lexed.diagnostics()
    );
    let output = parse(source, lexed.tokens(), &sources);
    Parsed {
        output,
        sources,
        text: text.to_string(),
    }
}

fn parse_valid(text: &str) -> Parsed {
    let parsed = parse_text(text);
    assert!(
        parsed.output.diagnostics().is_empty(),
        "unexpected parser diagnostics: {:?}",
        parsed.output.diagnostics()
    );
    parsed
}

fn assert_ast_dump_matches(source: &str, expected: &str) {
    let parsed = parse_text(source);
    assert_eq!(parsed.ast_dump(), expected);
}

#[test]
fn parses_declarations() {
    let parsed = parse_valid(
        r#"
        module banco.contas;
        import banco.Cliente;
        const PI = 3;
        let global = 1;
        function main() {}
        interface I {}
        trait T {}
        class C {}
        "#,
    );

    let root = parsed.output.ast().root();
    assert!(root.module.is_some());
    assert_eq!(root.imports.len(), 1);
    assert_eq!(root.declarations.len(), 6);
    assert!(matches!(root.declarations[0], Decl::Const(_)));
    assert!(matches!(root.declarations[1], Decl::Let(_)));
    assert!(matches!(root.declarations[2], Decl::Function(_)));
    assert!(matches!(root.declarations[3], Decl::Interface(_)));
    assert!(matches!(root.declarations[4], Decl::Trait(_)));
    assert!(matches!(root.declarations[5], Decl::Class(_)));
}

#[test]
fn parses_expressions() {
    let parsed = parse_valid(
        r#"
        function main() {
            let a = new Cliente("Ana");
            let b = cliente.nome().chars[0];
            let c = [1, 2, 3];
            let d = (a, b);
            let e = -value;
        }
        "#,
    );

    let Decl::Function(function) = &parsed.output.ast().root().declarations[0] else {
        panic!("expected function");
    };
    let body = function.body.as_ref().expect("function body");
    assert!(matches!(
        local_initializer(&body.statements[0]),
        Some(Expr::New(_))
    ));
    assert!(matches!(
        local_initializer(&body.statements[1]),
        Some(Expr::Index(_))
    ));
    assert!(matches!(
        local_initializer(&body.statements[2]),
        Some(Expr::Array { .. })
    ));
    assert!(matches!(
        local_initializer(&body.statements[3]),
        Some(Expr::Tuple { .. })
    ));
    assert!(matches!(
        local_initializer(&body.statements[4]),
        Some(Expr::Unary(_))
    ));
}

#[test]
fn parses_operator_precedence() {
    let parsed = parse_valid("function main() { let x = a + b * c == d || e; }");

    let initializer = function_local_initializer(&parsed, 0);
    let Expr::Binary(or_expr) = initializer else {
        panic!("expected top-level binary expression");
    };
    assert_eq!(or_expr.op, Operator::PipePipe);

    let Expr::Binary(eq_expr) = &*or_expr.left else {
        panic!("expected equality expression on left side of ||");
    };
    assert_eq!(eq_expr.op, Operator::EqualEqual);

    let Expr::Binary(add_expr) = &*eq_expr.left else {
        panic!("expected addition before equality");
    };
    assert_eq!(add_expr.op, Operator::Plus);

    let Expr::Binary(mul_expr) = &*add_expr.right else {
        panic!("expected multiplication nested under addition");
    };
    assert_eq!(mul_expr.op, Operator::Star);
}

#[test]
fn parses_types() {
    let parsed = parse_valid(
        r#"
        function f(
            a : Int32,
            b : banco.Cliente,
            c : List<Map<String, Cliente>>,
            d : Int32[10],
            e : (String, Int32)
        ) : Optional<Cliente> {}
        "#,
    );

    let Decl::Function(function) = &parsed.output.ast().root().declarations[0] else {
        panic!("expected function");
    };
    assert_eq!(function.params.len(), 5);
    assert!(matches!(function.params[0].ty, Some(TypeSyntax::Path(_))));
    assert!(matches!(
        function.params[2].ty,
        Some(TypeSyntax::Generic { .. })
    ));
    assert!(matches!(
        function.params[3].ty,
        Some(TypeSyntax::Array { .. })
    ));
    assert!(matches!(
        function.params[4].ty,
        Some(TypeSyntax::Tuple { .. })
    ));
    assert!(matches!(
        function.return_type,
        Some(TypeSyntax::Generic { .. })
    ));
}

#[test]
fn parses_classes() {
    let parsed = parse_valid(
        r#"
        public class Cliente<T> extends Pessoa implements Autenticavel, Nomeavel uses Logavel {
            private nome : String;
            constructor(nome : String) { this.nome = nome; }
            function getNome() : String { return nome; }
        }
        "#,
    );

    let Decl::Class(class) = &parsed.output.ast().root().declarations[0] else {
        panic!("expected class");
    };
    assert_eq!(class.name.text, "Cliente");
    assert_eq!(class.generic_params.len(), 1);
    assert!(class.extends.is_some());
    assert_eq!(class.implements.len(), 2);
    assert_eq!(class.uses.len(), 1);
    assert_eq!(class.members.len(), 3);
    assert!(matches!(class.members[0], MemberDecl::Field(_)));
    assert!(matches!(class.members[1], MemberDecl::Constructor(_)));
    assert!(matches!(class.members[2], MemberDecl::Method(_)));
}

#[test]
fn reports_syntax_errors() {
    let parsed = parse_text("function () { let x = ; }");

    assert!(!parsed.output.diagnostics().is_empty());
    assert_has_code(&parsed, "PARSE0002");
    assert_has_code(&parsed, "PARSE0006");
    assert!(parsed
        .output
        .diagnostics()
        .iter()
        .any(|diagnostic| diagnostic.message().contains("expected function name")));
}

#[test]
fn classifies_syntax_diagnostics() {
    let unexpected = parse_text("@");
    assert_has_code(&unexpected, "PARSE0004");

    let unclosed = parse_text("function main() { let x = 1;");
    assert_has_code(&unclosed, "PARSE0003");

    let missing_type = parse_text("function f(value) {}");
    assert_has_code(&missing_type, "PARSE0005");

    let missing_separator = parse_text("function f(a : Int32 b : String) {}");
    assert_has_code(&missing_separator, "PARSE0007");
}

#[test]
fn recovers_after_syntax_errors() {
    let parsed = parse_text(
        r#"
        function main() {
            let x = ;
            let y = 1;
            a + ;
            return y;
        }
        "#,
    );

    assert!(!parsed.output.diagnostics().is_empty());
    let Decl::Function(function) = &parsed.output.ast().root().declarations[0] else {
        panic!("expected function");
    };
    let body = function.body.as_ref().expect("function body");
    assert_eq!(body.statements.len(), 4);
    assert!(matches!(body.statements[1], Stmt::Let(_)));
    assert!(matches!(body.statements[3], Stmt::Return { .. }));
}

#[test]
fn builds_ast_with_spans_and_dump() {
    let parsed = parse_text("function main() { let x = 1 + 2; }");

    assert!(parsed.output.diagnostics().is_empty());
    let root = parsed.output.ast().root();
    assert_eq!(root.span.start().raw(), 0);
    assert_eq!(root.span.end().raw(), 34);

    let dump = parsed.ast_dump();
    assert!(dump.contains("CompilationUnit span=0..34"));
    assert!(dump.contains("FunctionDecl name=main"));
    assert!(dump.contains("LocalLet name=x"));
    assert!(dump.contains("BinaryExpr op=Plus"));
    assert!(!dump.contains("0x"));
}

#[test]
fn ast_dump_matches_basic_golden() {
    assert_ast_dump_matches(
        include_str!("fixtures/ast_dump/basic.cap"),
        include_str!("fixtures/ast_dump/basic.ast"),
    );
}

#[test]
fn ast_dump_matches_recovery_golden() {
    let parsed = parse_text(include_str!("fixtures/ast_dump/recovery.cap"));

    assert!(!parsed.output.diagnostics().is_empty());
    assert_eq!(
        parsed.ast_dump(),
        include_str!("fixtures/ast_dump/recovery.ast")
    );
}

#[test]
fn preserves_spans_for_specific_ast_nodes() {
    let source = "function main(value : List<String>) { let result = service.call(value) + 1; let field = (service).field; }";
    let parsed = parse_valid(source);

    let Decl::Function(function) = &parsed.output.ast().root().declarations[0] else {
        panic!("expected function");
    };
    assert_span(&parsed, function.span, source);
    assert_span(&parsed, function.name.span, "main");

    let param = &function.params[0];
    assert_span(&parsed, param.span, "value : List<String>");
    assert_span(
        &parsed,
        param.ty.as_ref().expect("param type").span(),
        "List<String>",
    );

    let body = function.body.as_ref().expect("function body");
    assert_span(
        &parsed,
        body.span,
        "{ let result = service.call(value) + 1; let field = (service).field; }",
    );

    let Stmt::Let(local) = &body.statements[0] else {
        panic!("expected local let");
    };
    assert_span(&parsed, local.span, "let result = service.call(value) + 1;");
    assert_span(&parsed, local.name.span, "result");

    let Some(Expr::Binary(binary)) = &local.initializer else {
        panic!("expected binary initializer");
    };
    assert_span(&parsed, binary.span, "service.call(value) + 1");

    let Expr::Call(call) = &*binary.left else {
        panic!("expected call expression");
    };
    assert_span(&parsed, call.span, "service.call(value)");
    assert_span(&parsed, call.callee.span(), "service.call");

    let Expr::Literal(literal) = &*binary.right else {
        panic!("expected integer literal");
    };
    assert_span(&parsed, literal.span, "1");

    let Stmt::Let(field_local) = &body.statements[1] else {
        panic!("expected second local let");
    };
    let Some(Expr::Member(member)) = &field_local.initializer else {
        panic!("expected member expression");
    };
    assert_span(&parsed, member.span, "(service).field");
    assert_span_after(&parsed, "(service).", member.member.span, "field");
}

#[test]
fn preserves_span_for_error_node() {
    let parsed = parse_text("function main() { let result = ; }");

    let Decl::Function(function) = &parsed.output.ast().root().declarations[0] else {
        panic!("expected function");
    };
    let body = function.body.as_ref().expect("function body");
    let Stmt::Let(local) = &body.statements[0] else {
        panic!("expected local let");
    };
    let Some(Expr::Error(error)) = &local.initializer else {
        panic!("expected error initializer");
    };

    assert_eq!(error.span.start().raw(), parsed.span_for(";").0);
    assert_eq!(error.span.end().raw(), parsed.span_for(";").1);
}

fn function_local_initializer(parsed: &Parsed, index: usize) -> &Expr {
    let Decl::Function(function) = &parsed.output.ast().root().declarations[0] else {
        panic!("expected function");
    };
    let body = function.body.as_ref().expect("function body");
    local_initializer(&body.statements[index]).expect("local initializer")
}

fn local_initializer(statement: &Stmt) -> Option<&Expr> {
    let Stmt::Let(decl) = statement else {
        return None;
    };
    decl.initializer.as_ref()
}

fn assert_has_code(parsed: &Parsed, expected: &str) {
    assert!(
        parsed.output.diagnostics().iter().any(|diagnostic| {
            diagnostic
                .code()
                .is_some_and(|code| code.as_str() == expected)
        }),
        "expected diagnostic code {expected}, got {:?}",
        parsed.output.diagnostics()
    );
}

fn assert_span(parsed: &Parsed, span: capi_source::Span, needle: &str) {
    let (start, end) = parsed.span_for(needle);
    assert_span_range(span, (start, end), needle);
}

fn assert_span_after(parsed: &Parsed, after: &str, span: capi_source::Span, needle: &str) {
    let (start, end) = parsed.span_for_after(after, needle);
    assert_span_range(span, (start, end), needle);
}

fn assert_span_range(span: capi_source::Span, expected: (u32, u32), needle: &str) {
    assert_eq!(
        (span.start().raw(), span.end().raw()),
        expected,
        "unexpected span for {needle:?}"
    );
}
