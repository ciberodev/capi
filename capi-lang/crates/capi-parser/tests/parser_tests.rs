use capi_ast::{dump_ast, Decl, Expr, MemberDecl, Pattern, Stmt, TypeSyntax};
use capi_diagnostics::{LabelStyle, Severity};
use capi_lexer::{lex, Keyword, LiteralKind, Operator};
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
fn parses_empty_unit_and_preserves_source_id() {
    let parsed = parse_valid("");
    let root = parsed.output.ast().root();

    assert_eq!(root.source.raw(), 0);
    assert!(root.module.is_none());
    assert!(root.imports.is_empty());
    assert!(root.declarations.is_empty());
    assert_eq!(root.span.start().raw(), 0);
    assert_eq!(root.span.end().raw(), 0);
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
fn parses_unit_order_and_wildcard_imports() {
    let parsed = parse_valid(
        r#"
        module banco.contas;
        import banco.Cliente;
        import banco.*;
        function main() {}
        class Conta {}
        "#,
    );

    let root = parsed.output.ast().root();
    assert!(root.module.is_some());
    assert_eq!(root.imports.len(), 2);
    assert_eq!(root.imports[0].path.segments[1].text, "Cliente");
    assert!(!root.imports[0].wildcard);
    assert_eq!(root.imports[1].path.segments[0].text, "banco");
    assert!(root.imports[1].wildcard);
    assert_eq!(root.declarations.len(), 2);
    assert!(matches!(root.declarations[0], Decl::Function(_)));
    assert!(matches!(root.declarations[1], Decl::Class(_)));
}

#[test]
fn parses_module_only_unit_and_preserves_module_span() {
    let parsed = parse_valid("module banco.contas;");
    let root = parsed.output.ast().root();

    assert!(root.module.is_some());
    assert!(root.imports.is_empty());
    assert!(root.declarations.is_empty());
    assert_span(&parsed, root.span, "module banco.contas;");
    assert_span(
        &parsed,
        root.module.as_ref().expect("module").span,
        "module banco.contas;",
    );
    assert_span(
        &parsed,
        root.module.as_ref().expect("module").path.span,
        "banco.contas",
    );
}

#[test]
fn preserves_import_order_and_spans() {
    let parsed = parse_valid(
        r#"
        module banco;
        import banco.Cliente;
        import banco.*;
        "#,
    );
    let root = parsed.output.ast().root();

    assert_eq!(root.imports.len(), 2);
    assert_eq!(root.imports[0].path.segments[1].text, "Cliente");
    assert_eq!(root.imports[1].path.segments[0].text, "banco");
    assert_span(&parsed, root.imports[0].span, "import banco.Cliente;");
    assert_span(&parsed, root.imports[0].path.span, "banco.Cliente");
    assert_span(&parsed, root.imports[1].span, "import banco.*;");
}

#[test]
fn parses_declaration_prefixes_generics_and_global_items() {
    let parsed = parse_valid(
        r#"
        @service("accounts")
        public final function make<T>(value : T = 1) : T {}
        @data()
        private class Box<T> {}
        "#,
    );

    let root = parsed.output.ast().root();
    let Decl::Function(function) = &root.declarations[0] else {
        panic!("expected function");
    };
    assert_eq!(function.prefix.attributes.len(), 1);
    assert_eq!(
        function.prefix.attributes[0].path.segments[0].text,
        "service"
    );
    assert_eq!(function.prefix.attributes[0].arguments.len(), 1);
    assert_eq!(
        function
            .prefix
            .modifiers
            .iter()
            .map(|modifier| modifier.keyword)
            .collect::<Vec<_>>(),
        vec![Keyword::Public, Keyword::Final]
    );
    assert_eq!(function.generic_params[0].name.text, "T");
    assert!(function.params[0].default_value.is_some());
    assert!(function.return_type.is_some());

    let Decl::Class(class) = &root.declarations[1] else {
        panic!("expected class");
    };
    assert_eq!(class.prefix.attributes.len(), 1);
    assert_eq!(class.prefix.modifiers[0].keyword, Keyword::Private);
    assert_eq!(class.generic_params[0].name.text, "T");
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
fn parses_statement_forms() {
    let parsed = parse_valid(
        r#"
        function main() {
            {}
            let value = 1;
            const fixed = value;
            value;
            return;
            return value;
            break;
            continue;
            if (value) { value; } else if (fixed) { fixed; } else { 0; }
            switch (value) { case 1: value; default: fixed; }
            while (value) { break; }
            for (let i = 0; i; i = i + 1) { continue; }
            foreach (item in value) { item; }
            match (value) { case Cliente(item): item; case _: value; }
            unsafe { value; }
        }
        "#,
    );

    let Decl::Function(function) = &parsed.output.ast().root().declarations[0] else {
        panic!("expected function");
    };
    let body = function.body.as_ref().expect("function body");
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::Block(_))));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::Let(_))));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::Const(_))));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::Expr { .. })));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::Return { value: None, .. })));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::Return { value: Some(_), .. })));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::Break(_))));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::Continue(_))));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::If(_))));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::Switch(_))));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::While(_))));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::For(_))));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::Foreach(_))));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::Match(_))));
    assert!(body
        .statements
        .iter()
        .any(|stmt| matches!(stmt, Stmt::UnsafeBlock(_))));
}

#[test]
fn parses_expression_forms_and_literals() {
    let parsed = parse_valid(
        r#"
        function main() {
            let a = 1;
            let b = 3.14;
            let c = "text";
            let d = 'x';
            let e = true;
            let f = this;
            let g = ();
            let h = (a);
            let i = (a, b);
            let j = call();
            let k = call(a, b);
            let l = call(a)(b);
            let m = (object).field;
            let n = array[0];
            let o = new Cliente(a);
            let p = !a;
            let q = a = b = c;
        }
        "#,
    );

    let Decl::Function(function) = &parsed.output.ast().root().declarations[0] else {
        panic!("expected function");
    };
    let body = function.body.as_ref().expect("function body");
    assert!(matches!(
        local_initializer(&body.statements[0]),
        Some(Expr::Literal(literal)) if literal.kind == LiteralKind::Integer
    ));
    assert!(matches!(
        local_initializer(&body.statements[1]),
        Some(Expr::Literal(literal)) if literal.kind == LiteralKind::Float
    ));
    assert!(matches!(
        local_initializer(&body.statements[2]),
        Some(Expr::Literal(literal)) if literal.kind == LiteralKind::String
    ));
    assert!(matches!(
        local_initializer(&body.statements[3]),
        Some(Expr::Literal(literal)) if literal.kind == LiteralKind::Char
    ));
    assert!(matches!(
        local_initializer(&body.statements[4]),
        Some(Expr::Literal(literal)) if literal.kind == LiteralKind::Bool
    ));
    assert!(matches!(
        local_initializer(&body.statements[5]),
        Some(Expr::Name(path)) if path.segments[0].text == "this"
    ));
    assert!(matches!(
        local_initializer(&body.statements[6]),
        Some(Expr::Tuple { elements, .. }) if elements.is_empty()
    ));
    assert!(matches!(
        local_initializer(&body.statements[7]),
        Some(Expr::Group { .. })
    ));
    assert!(matches!(
        local_initializer(&body.statements[8]),
        Some(Expr::Tuple { elements, .. }) if elements.len() == 2
    ));
    assert!(matches!(
        local_initializer(&body.statements[9]),
        Some(Expr::Call(call)) if call.arguments.is_empty()
    ));
    assert!(matches!(
        local_initializer(&body.statements[10]),
        Some(Expr::Call(call)) if call.arguments.len() == 2
    ));
    assert!(matches!(
        local_initializer(&body.statements[11]),
        Some(Expr::Call(call)) if matches!(&*call.callee, Expr::Call(_))
    ));
    assert!(matches!(
        local_initializer(&body.statements[12]),
        Some(Expr::Member(_))
    ));
    assert!(matches!(
        local_initializer(&body.statements[13]),
        Some(Expr::Index(_))
    ));
    assert!(matches!(
        local_initializer(&body.statements[14]),
        Some(Expr::New(_))
    ));
    assert!(matches!(
        local_initializer(&body.statements[15]),
        Some(Expr::Unary(unary)) if unary.op == Operator::Bang
    ));
    assert!(matches!(
        local_initializer(&body.statements[16]),
        Some(Expr::Assign(assign)) if matches!(&*assign.value, Expr::Assign(_))
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
fn parses_each_precedence_level_and_postfix_associativity() {
    let parsed = parse_valid(
        "function main() { let x = a || b && c == d < e + f * -g; let y = (object).field[0](arg); }",
    );

    let x = function_local_initializer(&parsed, 0);
    let Expr::Binary(or_expr) = x else {
        panic!("expected || expression");
    };
    assert_eq!(or_expr.op, Operator::PipePipe);
    let Expr::Binary(and_expr) = &*or_expr.right else {
        panic!("expected && expression");
    };
    assert_eq!(and_expr.op, Operator::AmpAmp);
    let Expr::Binary(eq_expr) = &*and_expr.right else {
        panic!("expected equality expression");
    };
    assert_eq!(eq_expr.op, Operator::EqualEqual);
    let Expr::Binary(lt_expr) = &*eq_expr.right else {
        panic!("expected relational expression");
    };
    assert_eq!(lt_expr.op, Operator::Less);
    let Expr::Binary(add_expr) = &*lt_expr.right else {
        panic!("expected additive expression");
    };
    assert_eq!(add_expr.op, Operator::Plus);
    let Expr::Binary(mul_expr) = &*add_expr.right else {
        panic!("expected multiplicative expression");
    };
    assert_eq!(mul_expr.op, Operator::Star);
    assert!(matches!(&*mul_expr.right, Expr::Unary(_)));

    let y = function_local_initializer(&parsed, 1);
    let Expr::Call(call) = y else {
        panic!("expected call expression");
    };
    let Expr::Index(index) = &*call.callee else {
        panic!("expected index before call");
    };
    assert!(matches!(&*index.base, Expr::Member(_)));
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
fn parses_array_type_without_size_when_allowed() {
    let parsed = parse_valid("function f(values : Int32[]) {}");

    let Decl::Function(function) = &parsed.output.ast().root().declarations[0] else {
        panic!("expected function");
    };
    let Some(TypeSyntax::Array { size, span, .. }) = &function.params[0].ty else {
        panic!("expected array type");
    };

    assert!(size.is_none());
    assert_span(&parsed, *span, "Int32[]");
}

#[test]
fn parses_interface_trait_and_member_signatures() {
    let parsed = parse_valid(
        r#"
        interface Nomeavel {
            function nome() : String;
            function apelido() : String;
        }
        trait Logavel {
            function log(message : String);
            function enabled() : Bool { return true; }
        }
        "#,
    );

    let root = parsed.output.ast().root();
    let Decl::Interface(interface) = &root.declarations[0] else {
        panic!("expected interface");
    };
    assert_eq!(interface.members.len(), 2);
    assert!(matches!(interface.members[0], MemberDecl::Method(_)));
    let MemberDecl::Method(signature) = &interface.members[0] else {
        panic!("expected method signature");
    };
    assert!(signature.body.is_none());
    assert!(signature.return_type.is_some());

    let Decl::Trait(trait_decl) = &root.declarations[1] else {
        panic!("expected trait");
    };
    assert_eq!(trait_decl.members.len(), 2);
    let MemberDecl::Method(default_method) = &trait_decl.members[1] else {
        panic!("expected default trait method");
    };
    assert!(default_method.body.is_some());
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
fn reports_structured_syntax_diagnostics() {
    let parsed = parse_text("function f(value) { let x = ; }");

    assert!(!parsed.output.diagnostics().is_empty());
    for diagnostic in parsed.output.diagnostics() {
        assert_eq!(diagnostic.severity(), Severity::Error);
        let span = diagnostic
            .primary_span()
            .expect("parser diagnostic should have primary span");
        assert!(
            diagnostic
                .labels()
                .iter()
                .any(|label| label.span() == span && label.style() == LabelStyle::Primary),
            "parser diagnostic should include primary label"
        );
    }
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
fn reports_additional_negative_syntax_cases() {
    for (source, code) in [
        ("public", "PARSE0004"),
        ("@decorator()", "PARSE0004"),
        ("function", "PARSE0002"),
        ("function main(a : Int32,, b : Int32) {}", "PARSE0002"),
        ("function main(a : Int32 b : Int32) {}", "PARSE0007"),
        ("function main() :", "PARSE0002"),
        ("function main(value :) {}", "PARSE0002"),
        ("function main(value : List<) {}", "PARSE0002"),
        ("function main(value : List<String) {}", "PARSE0002"),
        ("function main(value : Map<String Cliente>) {}", "PARSE0007"),
        ("function main(value : Int32[10) {}", "PARSE0002"),
        ("class C { constructor(a : Int32); }", "PARSE0004"),
        ("class C { field : Int32 }", "PARSE0002"),
        ("class C { function () {} }", "PARSE0002"),
        ("function main() { call(,); }", "PARSE0006"),
        ("function main() { array[0; }", "PARSE0002"),
        ("function main() { if (value { return; } }", "PARSE0002"),
        ("function main() { return }", "PARSE0002"),
        ("function main() { a + ; }", "PARSE0006"),
        ("function main() { match (x) { case : x; } }", "PARSE0001"),
    ] {
        let parsed = parse_text(source);
        assert_has_code(&parsed, code);
    }
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
fn recovers_from_top_level_member_and_type_errors() {
    let parsed = parse_text(
        r#"
        @
        function () { let broken = ; }
        function ok(value : List<String) {
            let y = 1;
        }
        class C {
            @
            function good() {}
        }
        "#,
    );

    assert!(!parsed.output.diagnostics().is_empty());
    assert!(parsed
        .output
        .ast()
        .root()
        .declarations
        .iter()
        .any(|decl| matches!(decl, Decl::Function(function) if function.name.text == "ok")));
    assert!(parsed
        .output
        .ast()
        .root()
        .declarations
        .iter()
        .any(|decl| matches!(decl, Decl::Class(_))));
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
fn recovers_missing_function_body_and_continues_with_later_declaration() {
    let parsed = parse_text(
        r#"
        function broken()
        function ok() {}
        "#,
    );

    assert!(!parsed.output.diagnostics().is_empty());
    assert!(parsed
        .output
        .ast()
        .root()
        .declarations
        .iter()
        .any(|decl| matches!(decl, Decl::Function(function) if function.name.text == "ok")));
}

#[test]
fn parses_patterns_in_match_and_foreach() {
    let parsed = parse_valid(
        r#"
        function main(value : Cliente) {
            foreach (item in value) { item; }
            match (value) {
                case Cliente(name, _): name;
                case 1: value;
            }
        }
        "#,
    );

    let Decl::Function(function) = &parsed.output.ast().root().declarations[0] else {
        panic!("expected function");
    };
    let body = function.body.as_ref().expect("function body");
    let Stmt::Foreach(foreach) = &body.statements[0] else {
        panic!("expected foreach");
    };
    assert!(matches!(foreach.binding, Pattern::Path(_)));

    let Stmt::Match(match_stmt) = &body.statements[1] else {
        panic!("expected match");
    };
    assert_eq!(match_stmt.arms.len(), 2);
    assert!(matches!(
        match_stmt.arms[0].pattern,
        Pattern::Constructor { .. }
    ));
    assert!(matches!(match_stmt.arms[1].pattern, Pattern::Literal(_)));
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
fn preserves_spans_after_unicode_text() {
    let source = "function main() { let texto = \"ç\"; let valor = 1; }";
    let parsed = parse_valid(source);

    let Decl::Function(function) = &parsed.output.ast().root().declarations[0] else {
        panic!("expected function");
    };
    let body = function.body.as_ref().expect("function body");
    let Stmt::Let(second) = &body.statements[1] else {
        panic!("expected second let");
    };

    assert_span(&parsed, second.name.span, "valor");
    assert_span_after(
        &parsed,
        "valor = ",
        second.initializer.as_ref().unwrap().span(),
        "1",
    );
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
