//! Syntax parser for the Capi compiler.

use capi_ast::{
    Ast, AstErrorNode, Attribute, BinaryExpr, Block, CallExpr, ClassDecl, CompilationUnit,
    ConstDecl, ConstructorDecl, Decl, DeclPrefix, Expr, FieldDecl, ForStmt, ForeachStmt,
    FunctionDecl, GenericParam, Identifier, IfStmt, ImportDecl, IndexExpr, InterfaceDecl, LetDecl,
    LiteralExpr, MatchArm, MatchStmt, MemberDecl, MemberExpr, Modifier, ModuleDecl, NewExpr, Param,
    Path, Pattern, Stmt, SwitchCase, SwitchStmt, TraitDecl, TypeSyntax, UnaryExpr, WhileStmt,
};
use capi_diagnostics::{Diagnostic, DiagnosticCode, DiagnosticLabel};
use capi_lexer::{Delimiter, Keyword, Operator, Token, TokenKind};
use capi_source::{SourceId, SourceMap, Span};

/// Output of a parsing run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseOutput {
    ast: Ast,
    diagnostics: Vec<Diagnostic>,
}

impl ParseOutput {
    /// Returns the parsed AST.
    pub const fn ast(&self) -> &Ast {
        &self.ast
    }

    /// Returns parser diagnostics.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Consumes this output into parts.
    pub fn into_parts(self) -> (Ast, Vec<Diagnostic>) {
        (self.ast, self.diagnostics)
    }
}

/// Parses a token sequence into an AST.
pub fn parse(source: SourceId, tokens: &[Token], sources: &SourceMap) -> ParseOutput {
    Parser::new(source, tokens, sources).parse()
}

struct Parser<'a> {
    source: SourceId,
    tokens: &'a [Token],
    sources: &'a SourceMap,
    cursor: usize,
    diagnostics: Vec<Diagnostic>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParseDiagnosticKind {
    UnexpectedToken,
    ExpectedToken,
    UnclosedDelimiter,
    IncompleteDeclaration,
    MissingType,
    MissingExpression,
    MissingSeparator,
}

impl ParseDiagnosticKind {
    const fn code(self) -> &'static str {
        match self {
            Self::UnexpectedToken => "PARSE0001",
            Self::ExpectedToken => "PARSE0002",
            Self::UnclosedDelimiter => "PARSE0003",
            Self::IncompleteDeclaration => "PARSE0004",
            Self::MissingType => "PARSE0005",
            Self::MissingExpression => "PARSE0006",
            Self::MissingSeparator => "PARSE0007",
        }
    }
}

impl<'a> Parser<'a> {
    fn new(source: SourceId, tokens: &'a [Token], sources: &'a SourceMap) -> Self {
        Self {
            source,
            tokens,
            sources,
            cursor: 0,
            diagnostics: Vec::new(),
        }
    }

    fn parse(mut self) -> ParseOutput {
        let root = self.parse_compilation_unit();
        ParseOutput {
            ast: Ast::new(root),
            diagnostics: self.diagnostics,
        }
    }

    fn parse_compilation_unit(&mut self) -> CompilationUnit {
        let start = self.current().span();
        let module = if self.at_keyword(Keyword::Module) {
            Some(self.parse_module_decl())
        } else {
            None
        };

        let mut imports = Vec::new();
        while self.at_keyword(Keyword::Import) {
            imports.push(self.parse_import_decl());
        }

        let mut declarations = Vec::new();
        while !self.at_eof() {
            if self.at_keyword(Keyword::Import) {
                self.error_here(
                    ParseDiagnosticKind::UnexpectedToken,
                    "import declarations must appear before declarations",
                );
                imports.push(self.parse_import_decl());
                continue;
            }

            let before = self.cursor;
            declarations.push(self.parse_decl());
            if self.cursor == before {
                self.error_here(
                    ParseDiagnosticKind::IncompleteDeclaration,
                    "expected declaration",
                );
                self.bump();
            }
        }

        let end = self.current().span();
        CompilationUnit {
            source: self.source,
            module,
            imports,
            declarations,
            span: self.merge(start, end),
        }
    }

    fn parse_module_decl(&mut self) -> ModuleDecl {
        let start = self.expect_keyword(Keyword::Module, "expected `module`");
        let path = self.parse_path();
        let end = self.expect_delimiter(
            Delimiter::Semicolon,
            "expected `;` after module declaration",
        );
        ModuleDecl {
            span: self.merge(start, end),
            path,
        }
    }

    fn parse_import_decl(&mut self) -> ImportDecl {
        let start = self.expect_keyword(Keyword::Import, "expected `import`");
        let path = self.parse_path();
        let mut wildcard = false;
        if self.eat_delimiter(Delimiter::Dot).is_some() {
            if self.eat_operator(Operator::Star).is_some() {
                wildcard = true;
            } else {
                self.error_here(
                    ParseDiagnosticKind::ExpectedToken,
                    "expected `*` after `.` in import",
                );
            }
        }
        let end = self.expect_delimiter(
            Delimiter::Semicolon,
            "expected `;` after import declaration",
        );
        ImportDecl {
            path,
            wildcard,
            span: self.merge(start, end),
        }
    }

    fn parse_decl(&mut self) -> Decl {
        let prefix = self.parse_decl_prefix();
        match self.current().kind() {
            TokenKind::Keyword(Keyword::Function) => {
                Decl::Function(self.parse_function_decl(prefix))
            }
            TokenKind::Keyword(Keyword::Class) => Decl::Class(self.parse_class_decl(prefix)),
            TokenKind::Keyword(Keyword::Interface) => {
                Decl::Interface(self.parse_interface_decl(prefix))
            }
            TokenKind::Keyword(Keyword::Trait) => Decl::Trait(self.parse_trait_decl(prefix)),
            TokenKind::Keyword(Keyword::Const) => Decl::Const(self.parse_const_decl(prefix, true)),
            TokenKind::Keyword(Keyword::Let) => Decl::Let(self.parse_let_decl(prefix, true)),
            _ => {
                let error = self.error_node_here(
                    ParseDiagnosticKind::IncompleteDeclaration,
                    "expected declaration",
                );
                self.recover_top_level();
                Decl::Error(error)
            }
        }
    }

    fn parse_decl_prefix(&mut self) -> DeclPrefix {
        let mut prefix = DeclPrefix::default();
        while self.at_delimiter(Delimiter::At) {
            prefix.attributes.push(self.parse_attribute());
        }
        while let TokenKind::Keyword(keyword) = self.current().kind() {
            if is_modifier(*keyword) {
                let token = self.bump();
                prefix.modifiers.push(Modifier {
                    keyword: *keyword,
                    span: token.span(),
                });
            } else {
                break;
            }
        }
        prefix
    }

    fn parse_attribute(&mut self) -> Attribute {
        let start = self.expect_delimiter(Delimiter::At, "expected `@`");
        let path = self.parse_path();
        let mut arguments = Vec::new();
        let mut end = path.span;
        if self.eat_delimiter(Delimiter::LeftParen).is_some() {
            if !self.at_delimiter(Delimiter::RightParen) && !self.at_eof() {
                loop {
                    arguments.push(self.parse_expr());
                    if self.eat_delimiter(Delimiter::Comma).is_none() {
                        break;
                    }
                    if self.at_delimiter(Delimiter::RightParen) {
                        break;
                    }
                }
            }
            end = self.expect_delimiter(
                Delimiter::RightParen,
                "expected `)` after attribute arguments",
            );
        }
        Attribute {
            path,
            arguments,
            span: self.merge(start, end),
        }
    }

    fn parse_class_decl(&mut self, prefix: DeclPrefix) -> ClassDecl {
        let start = self.expect_keyword(Keyword::Class, "expected `class`");
        let name = self.parse_identifier("expected class name");
        let generic_params = self.parse_generic_params();
        let extends = if self.eat_keyword(Keyword::Extends).is_some() {
            Some(self.parse_type())
        } else {
            None
        };
        let implements = self.parse_type_clause(Keyword::Implements);
        let uses = self.parse_type_clause(Keyword::Uses);
        let (members, end) = self.parse_member_body();
        ClassDecl {
            prefix,
            name,
            generic_params,
            extends,
            implements,
            uses,
            members,
            span: self.merge(start, end),
        }
    }

    fn parse_interface_decl(&mut self, prefix: DeclPrefix) -> InterfaceDecl {
        let start = self.expect_keyword(Keyword::Interface, "expected `interface`");
        let name = self.parse_identifier("expected interface name");
        let generic_params = self.parse_generic_params();
        let (members, end) = self.parse_member_body();
        InterfaceDecl {
            prefix,
            name,
            generic_params,
            members,
            span: self.merge(start, end),
        }
    }

    fn parse_trait_decl(&mut self, prefix: DeclPrefix) -> TraitDecl {
        let start = self.expect_keyword(Keyword::Trait, "expected `trait`");
        let name = self.parse_identifier("expected trait name");
        let generic_params = self.parse_generic_params();
        let (members, end) = self.parse_member_body();
        TraitDecl {
            prefix,
            name,
            generic_params,
            members,
            span: self.merge(start, end),
        }
    }

    fn parse_type_clause(&mut self, keyword: Keyword) -> Vec<TypeSyntax> {
        if self.eat_keyword(keyword).is_none() {
            return Vec::new();
        }
        let mut types = Vec::new();
        loop {
            types.push(self.parse_type());
            if self.eat_delimiter(Delimiter::Comma).is_none() {
                if matches!(self.current().kind(), TokenKind::Identifier) {
                    self.error_here(
                        ParseDiagnosticKind::MissingSeparator,
                        "expected `,` between types",
                    );
                    continue;
                }
                break;
            }
        }
        types
    }

    fn parse_member_body(&mut self) -> (Vec<MemberDecl>, Span) {
        let open = self.expect_delimiter(Delimiter::LeftBrace, "expected `{` before type body");
        let mut members = Vec::new();
        while !self.at_delimiter(Delimiter::RightBrace) && !self.at_eof() {
            let before = self.cursor;
            members.push(self.parse_member_decl());
            if self.cursor == before {
                self.bump();
            }
        }
        let close = if self.at_delimiter(Delimiter::RightBrace) {
            self.bump().span()
        } else {
            self.error_at(
                ParseDiagnosticKind::UnclosedDelimiter,
                open,
                "unclosed type body",
            );
            self.empty_at_current()
        };
        (members, close)
    }

    fn parse_member_decl(&mut self) -> MemberDecl {
        let prefix = self.parse_decl_prefix();
        match self.current().kind() {
            TokenKind::Keyword(Keyword::Function) => {
                MemberDecl::Method(self.parse_function_decl(prefix))
            }
            TokenKind::Keyword(Keyword::Constructor) => {
                MemberDecl::Constructor(self.parse_constructor_decl(prefix))
            }
            TokenKind::Keyword(Keyword::Const) => {
                MemberDecl::Const(self.parse_const_decl(prefix, true))
            }
            TokenKind::Keyword(Keyword::Let) => MemberDecl::Let(self.parse_let_decl(prefix, true)),
            TokenKind::Identifier => MemberDecl::Field(self.parse_field_decl(prefix)),
            _ => {
                let error = self.error_node_here(
                    ParseDiagnosticKind::IncompleteDeclaration,
                    "expected member declaration",
                );
                self.recover_member();
                MemberDecl::Error(error)
            }
        }
    }

    fn parse_function_decl(&mut self, prefix: DeclPrefix) -> FunctionDecl {
        let start = self.expect_keyword(Keyword::Function, "expected `function`");
        let name = self.parse_identifier("expected function name");
        let generic_params = self.parse_generic_params();
        let params = self.parse_param_list();
        let return_type = if self.eat_delimiter(Delimiter::Colon).is_some() {
            Some(self.parse_type())
        } else {
            None
        };
        let body = if self.at_delimiter(Delimiter::LeftBrace) {
            Some(self.parse_block())
        } else {
            self.expect_delimiter(
                Delimiter::Semicolon,
                "expected function body or `;` after signature",
            );
            None
        };
        let end = body
            .as_ref()
            .map_or_else(|| self.previous_span(), |body| body.span);
        FunctionDecl {
            prefix,
            name,
            generic_params,
            params,
            return_type,
            body,
            span: self.merge(start, end),
        }
    }

    fn parse_constructor_decl(&mut self, prefix: DeclPrefix) -> ConstructorDecl {
        let start = self.expect_keyword(Keyword::Constructor, "expected `constructor`");
        let params = self.parse_param_list();
        let body = if self.at_delimiter(Delimiter::LeftBrace) {
            Some(self.parse_block())
        } else {
            self.error_here(
                ParseDiagnosticKind::IncompleteDeclaration,
                "expected constructor body",
            );
            None
        };
        let end = body
            .as_ref()
            .map_or_else(|| self.previous_span(), |body| body.span);
        ConstructorDecl {
            prefix,
            params,
            body,
            span: self.merge(start, end),
        }
    }

    fn parse_field_decl(&mut self, prefix: DeclPrefix) -> FieldDecl {
        let start = self.current().span();
        let name = self.parse_identifier("expected field name");
        let ty = if self.eat_delimiter(Delimiter::Colon).is_some() {
            Some(self.parse_type())
        } else {
            None
        };
        let initializer = if self.eat_operator(Operator::Equal).is_some() {
            Some(self.parse_expr())
        } else {
            None
        };
        let end =
            self.expect_delimiter(Delimiter::Semicolon, "expected `;` after field declaration");
        FieldDecl {
            prefix,
            name,
            ty,
            initializer,
            span: self.merge(start, end),
        }
    }

    fn parse_const_decl(&mut self, prefix: DeclPrefix, semicolon: bool) -> ConstDecl {
        let start = self.expect_keyword(Keyword::Const, "expected `const`");
        let name = self.parse_identifier("expected constant name");
        let ty = if self.eat_delimiter(Delimiter::Colon).is_some() {
            Some(self.parse_type())
        } else {
            None
        };
        let initializer = if self.eat_operator(Operator::Equal).is_some() {
            Some(self.parse_expr())
        } else {
            self.error_here(
                ParseDiagnosticKind::MissingExpression,
                "expected initializer for constant",
            );
            None
        };
        let end = if semicolon {
            self.expect_delimiter(
                Delimiter::Semicolon,
                "expected `;` after constant declaration",
            )
        } else {
            self.previous_span()
        };
        ConstDecl {
            prefix,
            name,
            ty,
            initializer,
            span: self.merge(start, end),
        }
    }

    fn parse_let_decl(&mut self, prefix: DeclPrefix, semicolon: bool) -> LetDecl {
        let start = self.expect_keyword(Keyword::Let, "expected `let`");
        let name = self.parse_identifier("expected variable name");
        let ty = if self.eat_delimiter(Delimiter::Colon).is_some() {
            Some(self.parse_type())
        } else {
            None
        };
        let initializer = if self.eat_operator(Operator::Equal).is_some() {
            Some(self.parse_expr())
        } else {
            None
        };
        let end = if semicolon {
            self.expect_delimiter(
                Delimiter::Semicolon,
                "expected `;` after variable declaration",
            )
        } else {
            self.previous_span()
        };
        LetDecl {
            prefix,
            name,
            ty,
            initializer,
            span: self.merge(start, end),
        }
    }

    fn parse_param_list(&mut self) -> Vec<Param> {
        self.expect_delimiter(Delimiter::LeftParen, "expected `(` before parameter list");
        let mut params = Vec::new();
        while !self.at_delimiter(Delimiter::RightParen) && !self.at_eof() {
            let start = self.current().span();
            let name = self.parse_identifier("expected parameter name");
            let ty = if self.eat_delimiter(Delimiter::Colon).is_some() {
                Some(self.parse_type())
            } else {
                self.error_here(
                    ParseDiagnosticKind::MissingType,
                    "expected `:` and parameter type",
                );
                None
            };
            let default_value = if self.eat_operator(Operator::Equal).is_some() {
                Some(self.parse_expr())
            } else {
                None
            };
            let end = default_value.as_ref().map_or_else(
                || ty.as_ref().map_or(name.span, TypeSyntax::span),
                Expr::span,
            );
            params.push(Param {
                name,
                ty,
                default_value,
                span: self.merge(start, end),
            });
            if self.eat_delimiter(Delimiter::Comma).is_none() {
                if matches!(self.current().kind(), TokenKind::Identifier) {
                    self.error_here(
                        ParseDiagnosticKind::MissingSeparator,
                        "expected `,` between parameters",
                    );
                    continue;
                }
                break;
            }
        }
        self.expect_delimiter(Delimiter::RightParen, "expected `)` after parameter list");
        params
    }

    fn parse_generic_params(&mut self) -> Vec<GenericParam> {
        if self.eat_operator(Operator::Less).is_none() {
            return Vec::new();
        }
        let mut params = Vec::new();
        while !self.at_operator(Operator::Greater) && !self.at_eof() {
            let name = self.parse_identifier("expected generic parameter name");
            params.push(GenericParam {
                span: name.span,
                name,
            });
            if self.eat_delimiter(Delimiter::Comma).is_none() {
                break;
            }
        }
        self.expect_operator(Operator::Greater, "expected `>` after generic parameters");
        params
    }

    fn parse_type(&mut self) -> TypeSyntax {
        if self.at_delimiter(Delimiter::LeftParen) {
            return self.parse_tuple_type();
        }
        let base = self.parse_path();
        let mut ty = if self.eat_operator(Operator::Less).is_some() {
            let mut arguments = Vec::new();
            while !self.at_operator(Operator::Greater) && !self.at_eof() {
                arguments.push(self.parse_type());
                if self.eat_delimiter(Delimiter::Comma).is_none() {
                    break;
                }
            }
            let close =
                self.expect_operator(Operator::Greater, "expected `>` after type arguments");
            TypeSyntax::Generic {
                span: self.merge(base.span, close),
                base,
                arguments,
            }
        } else {
            TypeSyntax::Path(base)
        };
        while self.eat_delimiter(Delimiter::LeftBracket).is_some() {
            let size = if self.at_delimiter(Delimiter::RightBracket) {
                None
            } else {
                Some(self.parse_expr())
            };
            let close =
                self.expect_delimiter(Delimiter::RightBracket, "expected `]` after array type");
            ty = TypeSyntax::Array {
                span: self.merge(ty.span(), close),
                element: Box::new(ty),
                size: size.map(Box::new),
            };
        }
        ty
    }

    fn parse_tuple_type(&mut self) -> TypeSyntax {
        let start = self.expect_delimiter(Delimiter::LeftParen, "expected `(`");
        let mut elements = Vec::new();
        if !self.at_delimiter(Delimiter::RightParen) {
            loop {
                elements.push(self.parse_type());
                if self.eat_delimiter(Delimiter::Comma).is_none() {
                    break;
                }
            }
        }
        let close = self.expect_delimiter(Delimiter::RightParen, "expected `)` after tuple type");
        TypeSyntax::Tuple {
            elements,
            span: self.merge(start, close),
        }
    }

    fn parse_block(&mut self) -> Block {
        let start = self.expect_delimiter(Delimiter::LeftBrace, "expected `{` before block");
        let mut statements = Vec::new();
        while !self.at_delimiter(Delimiter::RightBrace) && !self.at_eof() {
            let before = self.cursor;
            statements.push(self.parse_stmt());
            if self.cursor == before {
                self.bump();
            }
        }
        let close = if self.at_delimiter(Delimiter::RightBrace) {
            self.bump().span()
        } else {
            self.error_at(
                ParseDiagnosticKind::UnclosedDelimiter,
                start,
                "unclosed block",
            );
            self.empty_at_current()
        };
        Block {
            statements,
            span: self.merge(start, close),
        }
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.current().kind() {
            TokenKind::Keyword(Keyword::Let) => {
                Stmt::Let(self.parse_let_decl(DeclPrefix::default(), true))
            }
            TokenKind::Keyword(Keyword::Const) => {
                Stmt::Const(self.parse_const_decl(DeclPrefix::default(), true))
            }
            TokenKind::Keyword(Keyword::Return) => self.parse_return_stmt(),
            TokenKind::Keyword(Keyword::If) => Stmt::If(self.parse_if_stmt()),
            TokenKind::Keyword(Keyword::Switch) => Stmt::Switch(self.parse_switch_stmt()),
            TokenKind::Keyword(Keyword::Match) => Stmt::Match(self.parse_match_stmt()),
            TokenKind::Keyword(Keyword::While) => Stmt::While(self.parse_while_stmt()),
            TokenKind::Keyword(Keyword::For) => Stmt::For(self.parse_for_stmt()),
            TokenKind::Identifier if self.current_lexeme() == "foreach" => {
                Stmt::Foreach(self.parse_foreach_stmt())
            }
            TokenKind::Keyword(Keyword::Break) => {
                let start = self.bump().span();
                let end = self.expect_delimiter(Delimiter::Semicolon, "expected `;` after `break`");
                Stmt::Break(self.merge(start, end))
            }
            TokenKind::Keyword(Keyword::Continue) => {
                let start = self.bump().span();
                let end =
                    self.expect_delimiter(Delimiter::Semicolon, "expected `;` after `continue`");
                Stmt::Continue(self.merge(start, end))
            }
            TokenKind::Keyword(Keyword::Unsafe) => {
                let start = self.bump().span();
                let block = self.parse_block();
                Stmt::UnsafeBlock(Block {
                    span: self.merge(start, block.span),
                    statements: block.statements,
                })
            }
            TokenKind::Delimiter(Delimiter::LeftBrace) => Stmt::Block(self.parse_block()),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_return_stmt(&mut self) -> Stmt {
        let start = self.expect_keyword(Keyword::Return, "expected `return`");
        let value = if self.at_delimiter(Delimiter::Semicolon) {
            None
        } else {
            Some(self.parse_expr())
        };
        let end =
            self.expect_delimiter(Delimiter::Semicolon, "expected `;` after return statement");
        Stmt::Return {
            value,
            span: self.merge(start, end),
        }
    }

    fn parse_if_stmt(&mut self) -> IfStmt {
        let start = self.expect_keyword(Keyword::If, "expected `if`");
        let condition = self.parse_parenthesized_expr("if condition");
        let then_branch = Box::new(self.parse_stmt());
        let else_branch = if self.eat_keyword(Keyword::Else).is_some() {
            Some(Box::new(self.parse_stmt()))
        } else {
            None
        };
        let end = else_branch
            .as_ref()
            .map_or_else(|| then_branch.span(), |branch| branch.span());
        IfStmt {
            condition,
            then_branch,
            else_branch,
            span: self.merge(start, end),
        }
    }

    fn parse_switch_stmt(&mut self) -> SwitchStmt {
        let start = self.expect_keyword(Keyword::Switch, "expected `switch`");
        let expression = self.parse_parenthesized_expr("switch expression");
        self.expect_delimiter(Delimiter::LeftBrace, "expected `{` before switch body");
        let mut cases = Vec::new();
        while !self.at_delimiter(Delimiter::RightBrace) && !self.at_eof() {
            let case_start = self.current().span();
            let label = if self.eat_keyword(Keyword::Case).is_some() {
                let expr = self.parse_expr();
                self.expect_delimiter(Delimiter::Colon, "expected `:` after case label");
                Some(expr)
            } else if self.eat_keyword(Keyword::Default).is_some() {
                self.expect_delimiter(Delimiter::Colon, "expected `:` after default label");
                None
            } else {
                self.error_here(
                    ParseDiagnosticKind::ExpectedToken,
                    "expected `case` or `default`",
                );
                self.bump();
                continue;
            };
            let mut statements = Vec::new();
            while !self.at_keyword(Keyword::Case)
                && !self.at_keyword(Keyword::Default)
                && !self.at_delimiter(Delimiter::RightBrace)
                && !self.at_eof()
            {
                statements.push(self.parse_stmt());
            }
            let end = statements.last().map_or(case_start, Stmt::span);
            cases.push(SwitchCase {
                label,
                statements,
                span: self.merge(case_start, end),
            });
        }
        let close = self.expect_delimiter(Delimiter::RightBrace, "expected `}` after switch body");
        SwitchStmt {
            expression,
            cases,
            span: self.merge(start, close),
        }
    }

    fn parse_match_stmt(&mut self) -> MatchStmt {
        let start = self.expect_keyword(Keyword::Match, "expected `match`");
        let expression = self.parse_parenthesized_expr("match expression");
        self.expect_delimiter(Delimiter::LeftBrace, "expected `{` before match body");
        let mut arms = Vec::new();
        while !self.at_delimiter(Delimiter::RightBrace) && !self.at_eof() {
            let arm_start = self.expect_keyword(Keyword::Case, "expected `case` in match body");
            let pattern = self.parse_pattern();
            self.expect_delimiter(Delimiter::Colon, "expected `:` after match pattern");
            let mut statements = Vec::new();
            while !self.at_keyword(Keyword::Case)
                && !self.at_delimiter(Delimiter::RightBrace)
                && !self.at_eof()
            {
                statements.push(self.parse_stmt());
            }
            let end = statements
                .last()
                .map_or_else(|| pattern_span(&pattern), Stmt::span);
            arms.push(MatchArm {
                pattern,
                statements,
                span: self.merge(arm_start, end),
            });
        }
        let close = self.expect_delimiter(Delimiter::RightBrace, "expected `}` after match body");
        MatchStmt {
            expression,
            arms,
            span: self.merge(start, close),
        }
    }

    fn parse_while_stmt(&mut self) -> WhileStmt {
        let start = self.expect_keyword(Keyword::While, "expected `while`");
        let condition = self.parse_parenthesized_expr("while condition");
        let body = Box::new(self.parse_stmt());
        WhileStmt {
            span: self.merge(start, body.span()),
            condition,
            body,
        }
    }

    fn parse_for_stmt(&mut self) -> ForStmt {
        let start = self.expect_keyword(Keyword::For, "expected `for`");
        self.expect_delimiter(Delimiter::LeftParen, "expected `(` after `for`");
        let initializer = if self.at_delimiter(Delimiter::Semicolon) {
            self.bump();
            None
        } else if self.at_keyword(Keyword::Let) {
            Some(Box::new(Stmt::Let(
                self.parse_let_decl(DeclPrefix::default(), true),
            )))
        } else if self.at_keyword(Keyword::Const) {
            Some(Box::new(Stmt::Const(
                self.parse_const_decl(DeclPrefix::default(), true),
            )))
        } else {
            let expr = self.parse_expr();
            let end =
                self.expect_delimiter(Delimiter::Semicolon, "expected `;` after for initializer");
            Some(Box::new(Stmt::Expr {
                span: self.merge(expr.span(), end),
                expr,
            }))
        };
        let condition = if self.at_delimiter(Delimiter::Semicolon) {
            self.bump();
            None
        } else {
            let condition = self.parse_expr();
            self.expect_delimiter(Delimiter::Semicolon, "expected `;` after for condition");
            Some(condition)
        };
        let increment = if self.at_delimiter(Delimiter::RightParen) {
            None
        } else {
            Some(self.parse_expr())
        };
        self.expect_delimiter(Delimiter::RightParen, "expected `)` after for clauses");
        let body = Box::new(self.parse_stmt());
        ForStmt {
            span: self.merge(start, body.span()),
            initializer,
            condition,
            increment,
            body,
        }
    }

    fn parse_foreach_stmt(&mut self) -> ForeachStmt {
        let start = self.expect_contextual_identifier("foreach", "expected `foreach`");
        self.expect_delimiter(Delimiter::LeftParen, "expected `(` after `foreach`");
        let binding = self.parse_pattern();
        self.expect_contextual_identifier("in", "expected `in` in foreach statement");
        let iterable = self.parse_expr();
        self.expect_delimiter(Delimiter::RightParen, "expected `)` after foreach header");
        let body = Box::new(self.parse_stmt());
        ForeachStmt {
            span: self.merge(start, body.span()),
            binding,
            iterable,
            body,
        }
    }

    fn parse_expr_stmt(&mut self) -> Stmt {
        let expr = self.parse_expr();
        let end = self.expect_delimiter(Delimiter::Semicolon, "expected `;` after expression");
        Stmt::Expr {
            span: self.merge(expr.span(), end),
            expr,
        }
    }

    fn parse_parenthesized_expr(&mut self, context: &str) -> Expr {
        self.expect_delimiter(
            Delimiter::LeftParen,
            format!("expected `(` before {context}"),
        );
        let expr = self.parse_expr();
        self.expect_delimiter(
            Delimiter::RightParen,
            format!("expected `)` after {context}"),
        );
        expr
    }

    fn parse_pattern(&mut self) -> Pattern {
        match self.current().kind() {
            TokenKind::Literal(kind) => {
                let token = self.bump();
                Pattern::Literal(LiteralExpr {
                    kind: *kind,
                    lexeme: self.lexeme(token.span()),
                    span: token.span(),
                })
            }
            TokenKind::Identifier => {
                let path = self.parse_path();
                if self.eat_delimiter(Delimiter::LeftParen).is_some() {
                    let mut fields = Vec::new();
                    while !self.at_delimiter(Delimiter::RightParen) && !self.at_eof() {
                        fields.push(self.parse_pattern());
                        if self.eat_delimiter(Delimiter::Comma).is_none() {
                            break;
                        }
                    }
                    let close =
                        self.expect_delimiter(Delimiter::RightParen, "expected `)` after pattern");
                    Pattern::Constructor {
                        span: self.merge(path.span, close),
                        path,
                        fields,
                    }
                } else if path.segments.len() == 1 && path.segments[0].text == "_" {
                    Pattern::Wildcard(path.span)
                } else {
                    Pattern::Path(path)
                }
            }
            _ => Pattern::Error(
                self.error_node_here(ParseDiagnosticKind::UnexpectedToken, "expected pattern"),
            ),
        }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_expr_bp(0)
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> Expr {
        let mut left = self.parse_prefix_expr();

        loop {
            left = if self.at_delimiter(Delimiter::LeftParen)
                || self.at_delimiter(Delimiter::Dot)
                || self.at_delimiter(Delimiter::LeftBracket)
            {
                self.parse_postfix_expr(left)
            } else {
                break;
            };
        }

        loop {
            let Some((op, left_bp, right_bp)) = self.current_binary_op() else {
                break;
            };
            if left_bp < min_bp {
                break;
            }
            self.bump();
            let right = self.parse_expr_bp(right_bp);
            let span = self.merge(left.span(), right.span());
            left = if op == Operator::Equal {
                Expr::Assign(capi_ast::AssignExpr {
                    target: Box::new(left),
                    value: Box::new(right),
                    span,
                })
            } else {
                Expr::Binary(BinaryExpr {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                    span,
                })
            };
        }

        left
    }

    fn parse_prefix_expr(&mut self) -> Expr {
        if let TokenKind::Operator(op @ (Operator::Bang | Operator::Minus | Operator::Plus)) =
            self.current().kind()
        {
            let start = self.bump().span();
            let expr = self.parse_expr_bp(8);
            return Expr::Unary(UnaryExpr {
                op: *op,
                span: self.merge(start, expr.span()),
                expr: Box::new(expr),
            });
        }
        self.parse_primary_expr()
    }

    fn parse_postfix_expr(&mut self, base: Expr) -> Expr {
        if self.eat_delimiter(Delimiter::LeftParen).is_some() {
            let mut arguments = Vec::new();
            while !self.at_delimiter(Delimiter::RightParen) && !self.at_eof() {
                arguments.push(self.parse_expr());
                if self.eat_delimiter(Delimiter::Comma).is_none() {
                    break;
                }
            }
            let close =
                self.expect_delimiter(Delimiter::RightParen, "expected `)` after arguments");
            return Expr::Call(CallExpr {
                span: self.merge(base.span(), close),
                callee: Box::new(base),
                arguments,
            });
        }

        if self.eat_delimiter(Delimiter::Dot).is_some() {
            let member = self.parse_identifier("expected member name after `.`");
            return Expr::Member(MemberExpr {
                span: self.merge(base.span(), member.span),
                base: Box::new(base),
                member,
            });
        }

        if self.eat_delimiter(Delimiter::LeftBracket).is_some() {
            let index = self.parse_expr();
            let close = self.expect_delimiter(Delimiter::RightBracket, "expected `]` after index");
            return Expr::Index(IndexExpr {
                span: self.merge(base.span(), close),
                base: Box::new(base),
                index: Box::new(index),
            });
        }

        base
    }

    fn parse_primary_expr(&mut self) -> Expr {
        match self.current().kind() {
            TokenKind::Literal(kind) => {
                let token = self.bump();
                Expr::Literal(LiteralExpr {
                    kind: *kind,
                    lexeme: self.lexeme(token.span()),
                    span: token.span(),
                })
            }
            TokenKind::Identifier => Expr::Name(self.parse_path()),
            TokenKind::Keyword(Keyword::New) => self.parse_new_expr(),
            TokenKind::Delimiter(Delimiter::LeftParen) => self.parse_group_or_tuple_expr(),
            TokenKind::Delimiter(Delimiter::LeftBracket) => self.parse_array_expr(),
            _ => Expr::Error(self.error_node_here(
                ParseDiagnosticKind::MissingExpression,
                "expected expression",
            )),
        }
    }

    fn parse_new_expr(&mut self) -> Expr {
        let start = self.expect_keyword(Keyword::New, "expected `new`");
        let ty = self.parse_type();
        let mut arguments = Vec::new();
        let mut end = ty.span();
        if self.eat_delimiter(Delimiter::LeftParen).is_some() {
            while !self.at_delimiter(Delimiter::RightParen) && !self.at_eof() {
                arguments.push(self.parse_expr());
                if self.eat_delimiter(Delimiter::Comma).is_none() {
                    break;
                }
            }
            end = self.expect_delimiter(
                Delimiter::RightParen,
                "expected `)` after constructor arguments",
            );
        }
        Expr::New(NewExpr {
            ty: Box::new(ty),
            arguments,
            span: self.merge(start, end),
        })
    }

    fn parse_group_or_tuple_expr(&mut self) -> Expr {
        let start = self.expect_delimiter(Delimiter::LeftParen, "expected `(`");
        if self.at_delimiter(Delimiter::RightParen) {
            let close = self.bump().span();
            return Expr::Tuple {
                elements: Vec::new(),
                span: self.merge(start, close),
            };
        }
        let first = self.parse_expr();
        if self.eat_delimiter(Delimiter::Comma).is_some() {
            let mut elements = vec![first];
            while !self.at_delimiter(Delimiter::RightParen) && !self.at_eof() {
                elements.push(self.parse_expr());
                if self.eat_delimiter(Delimiter::Comma).is_none() {
                    break;
                }
            }
            let close = self.expect_delimiter(Delimiter::RightParen, "expected `)` after tuple");
            Expr::Tuple {
                elements,
                span: self.merge(start, close),
            }
        } else {
            let close =
                self.expect_delimiter(Delimiter::RightParen, "expected `)` after expression");
            Expr::Group {
                expr: Box::new(first),
                span: self.merge(start, close),
            }
        }
    }

    fn parse_array_expr(&mut self) -> Expr {
        let start = self.expect_delimiter(Delimiter::LeftBracket, "expected `[`");
        let mut elements = Vec::new();
        while !self.at_delimiter(Delimiter::RightBracket) && !self.at_eof() {
            elements.push(self.parse_expr());
            if self.eat_delimiter(Delimiter::Comma).is_none() {
                break;
            }
        }
        let close =
            self.expect_delimiter(Delimiter::RightBracket, "expected `]` after array literal");
        Expr::Array {
            elements,
            span: self.merge(start, close),
        }
    }

    fn current_binary_op(&self) -> Option<(Operator, u8, u8)> {
        let TokenKind::Operator(op) = self.current().kind() else {
            return None;
        };
        match op {
            Operator::Star | Operator::Slash | Operator::Percent => Some((*op, 7, 8)),
            Operator::Plus | Operator::Minus => Some((*op, 6, 7)),
            Operator::Less | Operator::LessEqual | Operator::Greater | Operator::GreaterEqual => {
                Some((*op, 5, 6))
            }
            Operator::EqualEqual | Operator::BangEqual | Operator::EqualEqualEqual => {
                Some((*op, 4, 5))
            }
            Operator::AmpAmp => Some((*op, 3, 4)),
            Operator::PipePipe => Some((*op, 2, 3)),
            Operator::Equal => Some((*op, 1, 1)),
            _ => None,
        }
    }

    fn parse_identifier(&mut self, message: impl Into<String>) -> Identifier {
        if matches!(self.current().kind(), TokenKind::Identifier) {
            let token = self.bump();
            Identifier {
                text: self.lexeme(token.span()),
                span: token.span(),
            }
        } else {
            let span = self.current().span();
            self.error_here(ParseDiagnosticKind::ExpectedToken, message);
            Identifier {
                text: String::new(),
                span,
            }
        }
    }

    fn parse_path(&mut self) -> Path {
        let first = self.parse_identifier("expected identifier");
        let mut span = first.span;
        let mut segments = vec![first];
        while self.eat_delimiter(Delimiter::Dot).is_some() {
            let segment = self.parse_identifier("expected identifier after `.`");
            span = self.merge(span, segment.span);
            segments.push(segment);
        }
        Path { segments, span }
    }

    fn current(&self) -> &'a Token {
        self.tokens
            .get(self.cursor)
            .or_else(|| self.tokens.last())
            .expect("parser requires EOF token")
    }

    fn bump(&mut self) -> &'a Token {
        let token = self.current();
        if !matches!(token.kind(), TokenKind::Eof) {
            self.cursor += 1;
        }
        token
    }

    fn at_eof(&self) -> bool {
        matches!(self.current().kind(), TokenKind::Eof)
    }

    fn at_keyword(&self, keyword: Keyword) -> bool {
        matches!(self.current().kind(), TokenKind::Keyword(found) if *found == keyword)
    }

    fn at_operator(&self, operator: Operator) -> bool {
        matches!(self.current().kind(), TokenKind::Operator(found) if *found == operator)
    }

    fn at_delimiter(&self, delimiter: Delimiter) -> bool {
        matches!(self.current().kind(), TokenKind::Delimiter(found) if *found == delimiter)
    }

    fn eat_keyword(&mut self, keyword: Keyword) -> Option<Span> {
        self.at_keyword(keyword).then(|| self.bump().span())
    }

    fn eat_operator(&mut self, operator: Operator) -> Option<Span> {
        self.at_operator(operator).then(|| self.bump().span())
    }

    fn eat_delimiter(&mut self, delimiter: Delimiter) -> Option<Span> {
        self.at_delimiter(delimiter).then(|| self.bump().span())
    }

    fn expect_keyword(&mut self, keyword: Keyword, message: impl Into<String>) -> Span {
        if self.at_keyword(keyword) {
            self.bump().span()
        } else {
            self.error_here(ParseDiagnosticKind::ExpectedToken, message);
            self.empty_at_current()
        }
    }

    fn expect_operator(&mut self, operator: Operator, message: impl Into<String>) -> Span {
        if self.at_operator(operator) {
            self.bump().span()
        } else {
            self.error_here(ParseDiagnosticKind::ExpectedToken, message);
            self.empty_at_current()
        }
    }

    fn expect_delimiter(&mut self, delimiter: Delimiter, message: impl Into<String>) -> Span {
        if self.at_delimiter(delimiter) {
            self.bump().span()
        } else {
            self.error_here(ParseDiagnosticKind::ExpectedToken, message);
            self.empty_at_current()
        }
    }

    fn expect_contextual_identifier(&mut self, text: &str, message: impl Into<String>) -> Span {
        if matches!(self.current().kind(), TokenKind::Identifier) && self.current_lexeme() == text {
            self.bump().span()
        } else {
            self.error_here(ParseDiagnosticKind::ExpectedToken, message);
            self.empty_at_current()
        }
    }

    fn error_node_here(
        &mut self,
        kind: ParseDiagnosticKind,
        message: impl Into<String>,
    ) -> AstErrorNode {
        let message = message.into();
        let span = self.current().span();
        self.error_at(kind, span, message.clone());
        AstErrorNode { message, span }
    }

    fn error_here(&mut self, kind: ParseDiagnosticKind, message: impl Into<String>) {
        self.error_at(kind, self.current().span(), message);
    }

    fn error_at(&mut self, kind: ParseDiagnosticKind, span: Span, message: impl Into<String>) {
        let message = message.into();
        self.diagnostics.push(
            Diagnostic::error(message.clone())
                .with_code(DiagnosticCode::new(kind.code()))
                .with_primary_span(span)
                .with_label(DiagnosticLabel::primary(span, message)),
        );
    }

    fn recover_top_level(&mut self) {
        while !self.at_eof() && !self.is_top_level_start() {
            if self.at_delimiter(Delimiter::Semicolon) {
                self.bump();
                break;
            }
            self.bump();
        }
    }

    fn recover_member(&mut self) {
        while !self.at_eof() && !self.at_delimiter(Delimiter::RightBrace) && !self.is_member_start()
        {
            if self.at_delimiter(Delimiter::Semicolon) {
                self.bump();
                break;
            }
            self.bump();
        }
    }

    fn is_top_level_start(&self) -> bool {
        matches!(
            self.current().kind(),
            TokenKind::Keyword(
                Keyword::Function
                    | Keyword::Class
                    | Keyword::Interface
                    | Keyword::Trait
                    | Keyword::Const
                    | Keyword::Let
                    | Keyword::Public
                    | Keyword::Private
                    | Keyword::Protected
                    | Keyword::Abstract
                    | Keyword::Sealed
                    | Keyword::Final
                    | Keyword::Static
                    | Keyword::Unsafe
            ) | TokenKind::Delimiter(Delimiter::At)
        )
    }

    fn is_member_start(&self) -> bool {
        self.is_top_level_start()
            || matches!(
                self.current().kind(),
                TokenKind::Keyword(Keyword::Constructor) | TokenKind::Identifier
            )
    }

    fn merge(&self, start: Span, end: Span) -> Span {
        start.merge(end).unwrap_or(start)
    }

    fn previous_span(&self) -> Span {
        if self.cursor == 0 {
            self.current().span()
        } else {
            self.tokens[self.cursor - 1].span()
        }
    }

    fn empty_at_current(&self) -> Span {
        let span = self.current().span();
        Span::new_unchecked(self.source, span.start(), span.start())
    }

    fn lexeme(&self, span: Span) -> String {
        self.sources
            .span_text(span)
            .map_or_else(String::new, ToString::to_string)
    }

    fn current_lexeme(&self) -> String {
        self.lexeme(self.current().span())
    }
}

fn is_modifier(keyword: Keyword) -> bool {
    matches!(
        keyword,
        Keyword::Public
            | Keyword::Private
            | Keyword::Protected
            | Keyword::Abstract
            | Keyword::Sealed
            | Keyword::Final
            | Keyword::Static
            | Keyword::Override
            | Keyword::Unsafe
    )
}

fn pattern_span(pattern: &Pattern) -> Span {
    match pattern {
        Pattern::Path(path) => path.span,
        Pattern::Constructor { span, .. } | Pattern::Wildcard(span) => *span,
        Pattern::Literal(literal) => literal.span,
        Pattern::Error(error) => error.span,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use capi_lexer::lex;
    use capi_source::SourceMap;

    fn parse_text(text: &str) -> ParseOutput {
        let mut sources = SourceMap::default();
        let source = sources.add_file("test.cap", text);
        let lexed = lex(source, text);
        assert!(lexed.diagnostics().is_empty(), "{:?}", lexed.diagnostics());
        parse(source, lexed.tokens(), &sources)
    }

    #[test]
    fn parses_module_import_and_function() {
        let output =
            parse_text("module banco.contas;\nimport banco.Cliente;\nfunction main() {}\n");

        assert!(
            output.diagnostics().is_empty(),
            "{:?}",
            output.diagnostics()
        );
        assert!(output.ast().root().module.is_some());
        assert_eq!(output.ast().root().imports.len(), 1);
        assert_eq!(output.ast().root().declarations.len(), 1);
    }

    #[test]
    fn parses_class_with_constructor_and_method() {
        let output = parse_text(
            "public class Cliente { private nome : String; constructor(nome : String) { this.nome = nome; } function getNome() : String { return nome; } }",
        );

        assert!(
            output.diagnostics().is_empty(),
            "{:?}",
            output.diagnostics()
        );
        let Decl::Class(class) = &output.ast().root().declarations[0] else {
            panic!("expected class");
        };
        assert_eq!(class.members.len(), 3);
    }

    #[test]
    fn respects_operator_precedence() {
        let output = parse_text("function main() { let x = a + b * c; }");

        assert!(
            output.diagnostics().is_empty(),
            "{:?}",
            output.diagnostics()
        );
        let Decl::Function(function) = &output.ast().root().declarations[0] else {
            panic!("expected function");
        };
        let Some(body) = &function.body else {
            panic!("expected body");
        };
        let Stmt::Let(decl) = &body.statements[0] else {
            panic!("expected let");
        };
        let Some(Expr::Binary(binary)) = &decl.initializer else {
            panic!("expected binary");
        };
        assert_eq!(binary.op, Operator::Plus);
        assert!(matches!(&*binary.right, Expr::Binary(nested) if nested.op == Operator::Star));
    }

    #[test]
    fn recovers_missing_expression() {
        let output = parse_text("function main() { let x = ; let y = 1; }");

        assert!(!output.diagnostics().is_empty());
        let Decl::Function(function) = &output.ast().root().declarations[0] else {
            panic!("expected function");
        };
        let body = function.body.as_ref().expect("body");
        assert_eq!(body.statements.len(), 2);
    }

    #[test]
    fn parses_foreach_contextual_statement() {
        let output = parse_text("function main() { foreach (item in items) { process(item); } }");

        assert!(
            output.diagnostics().is_empty(),
            "{:?}",
            output.diagnostics()
        );
        let Decl::Function(function) = &output.ast().root().declarations[0] else {
            panic!("expected function");
        };
        let body = function.body.as_ref().expect("body");
        assert!(matches!(body.statements[0], Stmt::Foreach(_)));
    }
}
