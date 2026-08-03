//! Abstract syntax tree for the Capi compiler.

use std::fmt::{self, Write};

use capi_lexer::{Keyword, LiteralKind, Operator};
use capi_source::{SourceId, SourceMap, Span};

/// A parsed Capi source file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ast {
    root: CompilationUnit,
}

impl Ast {
    /// Creates an AST from its root node.
    pub const fn new(root: CompilationUnit) -> Self {
        Self { root }
    }

    /// Returns the root compilation unit.
    pub const fn root(&self) -> &CompilationUnit {
        &self.root
    }
}

/// A source file syntax tree root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompilationUnit {
    pub source: SourceId,
    pub module: Option<ModuleDecl>,
    pub imports: Vec<ImportDecl>,
    pub declarations: Vec<Decl>,
    pub span: Span,
}

/// A module declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleDecl {
    pub path: Path,
    pub span: Span,
}

/// An import declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportDecl {
    pub path: Path,
    pub wildcard: bool,
    pub span: Span,
}

/// A dotted path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path {
    pub segments: Vec<Identifier>,
    pub span: Span,
}

/// A user-written identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub text: String,
    pub span: Span,
}

/// A declaration attribute.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute {
    pub path: Path,
    pub arguments: Vec<Expr>,
    pub span: Span,
}

/// A declaration modifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Modifier {
    pub keyword: Keyword,
    pub span: Span,
}

/// A declaration prefix shared by items and members.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeclPrefix {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
}

/// A top-level declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Decl {
    Function(FunctionDecl),
    Class(ClassDecl),
    Interface(InterfaceDecl),
    Trait(TraitDecl),
    Const(ConstDecl),
    Let(LetDecl),
    Error(AstErrorNode),
}

/// A class declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassDecl {
    pub prefix: DeclPrefix,
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub extends: Option<TypeSyntax>,
    pub implements: Vec<TypeSyntax>,
    pub uses: Vec<TypeSyntax>,
    pub members: Vec<MemberDecl>,
    pub span: Span,
}

/// An interface declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InterfaceDecl {
    pub prefix: DeclPrefix,
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub members: Vec<MemberDecl>,
    pub span: Span,
}

/// A trait declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraitDecl {
    pub prefix: DeclPrefix,
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub members: Vec<MemberDecl>,
    pub span: Span,
}

/// A member declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MemberDecl {
    Field(FieldDecl),
    Method(FunctionDecl),
    Constructor(ConstructorDecl),
    Const(ConstDecl),
    Let(LetDecl),
    Error(AstErrorNode),
}

/// A field declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldDecl {
    pub prefix: DeclPrefix,
    pub name: Identifier,
    pub ty: Option<TypeSyntax>,
    pub initializer: Option<Expr>,
    pub span: Span,
}

/// A function or method declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionDecl {
    pub prefix: DeclPrefix,
    pub name: Identifier,
    pub generic_params: Vec<GenericParam>,
    pub params: Vec<Param>,
    pub return_type: Option<TypeSyntax>,
    pub body: Option<Block>,
    pub span: Span,
}

/// A constructor declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstructorDecl {
    pub prefix: DeclPrefix,
    pub params: Vec<Param>,
    pub body: Option<Block>,
    pub span: Span,
}

/// A constant declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstDecl {
    pub prefix: DeclPrefix,
    pub name: Identifier,
    pub ty: Option<TypeSyntax>,
    pub initializer: Option<Expr>,
    pub span: Span,
}

/// A let declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetDecl {
    pub prefix: DeclPrefix,
    pub name: Identifier,
    pub ty: Option<TypeSyntax>,
    pub initializer: Option<Expr>,
    pub span: Span,
}

/// A function parameter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub name: Identifier,
    pub ty: Option<TypeSyntax>,
    pub default_value: Option<Expr>,
    pub span: Span,
}

/// A generic parameter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenericParam {
    pub name: Identifier,
    pub span: Span,
}

/// A syntactic type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeSyntax {
    Path(Path),
    Generic {
        base: Path,
        arguments: Vec<TypeSyntax>,
        span: Span,
    },
    Array {
        element: Box<TypeSyntax>,
        size: Option<Box<Expr>>,
        span: Span,
    },
    Tuple {
        elements: Vec<TypeSyntax>,
        span: Span,
    },
    Error(AstErrorNode),
}

/// A block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Stmt>,
    pub span: Span,
}

/// A statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt {
    Let(LetDecl),
    Const(ConstDecl),
    Expr { expr: Expr, span: Span },
    Return { value: Option<Expr>, span: Span },
    If(IfStmt),
    Switch(SwitchStmt),
    Match(MatchStmt),
    While(WhileStmt),
    For(ForStmt),
    Foreach(ForeachStmt),
    Break(Span),
    Continue(Span),
    UnsafeBlock(Block),
    Block(Block),
    Error(AstErrorNode),
}

/// An if statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
    pub span: Span,
}

/// A switch statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwitchStmt {
    pub expression: Expr,
    pub cases: Vec<SwitchCase>,
    pub span: Span,
}

/// A switch case.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwitchCase {
    pub label: Option<Expr>,
    pub statements: Vec<Stmt>,
    pub span: Span,
}

/// A match statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchStmt {
    pub expression: Expr,
    pub arms: Vec<MatchArm>,
    pub span: Span,
}

/// A match arm.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub statements: Vec<Stmt>,
    pub span: Span,
}

/// A while statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
    pub span: Span,
}

/// A for statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForStmt {
    pub initializer: Option<Box<Stmt>>,
    pub condition: Option<Expr>,
    pub increment: Option<Expr>,
    pub body: Box<Stmt>,
    pub span: Span,
}

/// A foreach statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForeachStmt {
    pub binding: Pattern,
    pub iterable: Expr,
    pub body: Box<Stmt>,
    pub span: Span,
}

/// A pattern.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pattern {
    Path(Path),
    Constructor {
        path: Path,
        fields: Vec<Pattern>,
        span: Span,
    },
    Literal(LiteralExpr),
    Wildcard(Span),
    Error(AstErrorNode),
}

/// An expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Literal(LiteralExpr),
    Name(Path),
    This(Span),
    New(NewExpr),
    Call(CallExpr),
    Member(MemberExpr),
    Index(IndexExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Assign(AssignExpr),
    Group { expr: Box<Expr>, span: Span },
    Tuple { elements: Vec<Expr>, span: Span },
    Array { elements: Vec<Expr>, span: Span },
    Error(AstErrorNode),
}

/// A literal expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiteralExpr {
    pub kind: LiteralKind,
    pub lexeme: String,
    pub span: Span,
}

/// A new expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewExpr {
    pub ty: Box<TypeSyntax>,
    pub arguments: Vec<Expr>,
    pub span: Span,
}

/// A call expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub arguments: Vec<Expr>,
    pub span: Span,
}

/// A member access expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemberExpr {
    pub base: Box<Expr>,
    pub member: Identifier,
    pub span: Span,
}

/// An index expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexExpr {
    pub base: Box<Expr>,
    pub index: Box<Expr>,
    pub span: Span,
}

/// A unary expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnaryExpr {
    pub op: Operator,
    pub expr: Box<Expr>,
    pub span: Span,
}

/// A binary expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: Operator,
    pub right: Box<Expr>,
    pub span: Span,
}

/// An assignment expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssignExpr {
    pub target: Box<Expr>,
    pub value: Box<Expr>,
    pub span: Span,
}

/// A syntax error placeholder in the AST.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AstErrorNode {
    pub message: String,
    pub span: Span,
}

impl TypeSyntax {
    /// Returns this type span.
    pub const fn span(&self) -> Span {
        match self {
            Self::Path(path) => path.span,
            Self::Generic { span, .. } | Self::Array { span, .. } | Self::Tuple { span, .. } => {
                *span
            }
            Self::Error(error) => error.span,
        }
    }
}

impl Expr {
    /// Returns this expression span.
    pub const fn span(&self) -> Span {
        match self {
            Self::Literal(expr) => expr.span,
            Self::Name(path) => path.span,
            Self::This(span)
            | Self::Group { span, .. }
            | Self::Tuple { span, .. }
            | Self::Array { span, .. } => *span,
            Self::New(expr) => expr.span,
            Self::Call(expr) => expr.span,
            Self::Member(expr) => expr.span,
            Self::Index(expr) => expr.span,
            Self::Unary(expr) => expr.span,
            Self::Binary(expr) => expr.span,
            Self::Assign(expr) => expr.span,
            Self::Error(error) => error.span,
        }
    }
}

impl Stmt {
    /// Returns this statement span.
    pub const fn span(&self) -> Span {
        match self {
            Self::Let(decl) => decl.span,
            Self::Const(decl) => decl.span,
            Self::Expr { span, .. }
            | Self::Return { span, .. }
            | Self::Break(span)
            | Self::Continue(span) => *span,
            Self::If(stmt) => stmt.span,
            Self::Switch(stmt) => stmt.span,
            Self::Match(stmt) => stmt.span,
            Self::While(stmt) => stmt.span,
            Self::For(stmt) => stmt.span,
            Self::Foreach(stmt) => stmt.span,
            Self::UnsafeBlock(block) | Self::Block(block) => block.span,
            Self::Error(error) => error.span,
        }
    }
}

/// Dumps an AST in deterministic text format.
pub fn dump_ast(ast: &Ast, sources: &SourceMap) -> String {
    let mut dumper = AstDumper {
        output: String::new(),
        sources,
    };
    dumper.compilation_unit(ast.root(), 0);
    dumper.output
}

struct AstDumper<'a> {
    output: String,
    sources: &'a SourceMap,
}

impl AstDumper<'_> {
    fn line(&mut self, indent: usize, args: fmt::Arguments<'_>) {
        for _ in 0..indent {
            self.output.push_str("  ");
        }
        self.output.write_fmt(args).expect("write to String");
        self.output.push('\n');
    }

    fn span(&self, span: Span) -> String {
        format!("{}..{}", span.start().raw(), span.end().raw())
    }

    fn compilation_unit(&mut self, unit: &CompilationUnit, indent: usize) {
        self.line(
            indent,
            format_args!("CompilationUnit span={}", self.span(unit.span)),
        );
        if let Some(module) = &unit.module {
            self.line(
                indent + 1,
                format_args!(
                    "ModuleDecl path={} span={}",
                    self.path(&module.path),
                    self.span(module.span)
                ),
            );
        }
        for import in &unit.imports {
            let suffix = if import.wildcard { ".*" } else { "" };
            self.line(
                indent + 1,
                format_args!(
                    "ImportDecl path={}{} span={}",
                    self.path(&import.path),
                    suffix,
                    self.span(import.span)
                ),
            );
        }
        for declaration in &unit.declarations {
            self.decl(declaration, indent + 1);
        }
    }

    fn decl(&mut self, decl: &Decl, indent: usize) {
        match decl {
            Decl::Function(decl) => self.function("FunctionDecl", decl, indent),
            Decl::Class(decl) => {
                self.line(
                    indent,
                    format_args!(
                        "ClassDecl name={} span={}",
                        decl.name.text,
                        self.span(decl.span)
                    ),
                );
                self.prefix(&decl.prefix, indent + 1);
                if let Some(extends) = &decl.extends {
                    self.ty("Extends", extends, indent + 1);
                }
                for implements in &decl.implements {
                    self.ty("Implements", implements, indent + 1);
                }
                for used in &decl.uses {
                    self.ty("Uses", used, indent + 1);
                }
                for member in &decl.members {
                    self.member(member, indent + 1);
                }
            }
            Decl::Interface(decl) => {
                self.line(
                    indent,
                    format_args!(
                        "InterfaceDecl name={} span={}",
                        decl.name.text,
                        self.span(decl.span)
                    ),
                );
                self.prefix(&decl.prefix, indent + 1);
                for member in &decl.members {
                    self.member(member, indent + 1);
                }
            }
            Decl::Trait(decl) => {
                self.line(
                    indent,
                    format_args!(
                        "TraitDecl name={} span={}",
                        decl.name.text,
                        self.span(decl.span)
                    ),
                );
                self.prefix(&decl.prefix, indent + 1);
                for member in &decl.members {
                    self.member(member, indent + 1);
                }
            }
            Decl::Const(decl) => self.const_decl("ConstDecl", decl, indent),
            Decl::Let(decl) => self.let_decl("LetDecl", decl, indent),
            Decl::Error(error) => self.error(error, indent),
        }
    }

    fn member(&mut self, member: &MemberDecl, indent: usize) {
        match member {
            MemberDecl::Field(decl) => {
                self.line(
                    indent,
                    format_args!(
                        "FieldDecl name={} span={}",
                        decl.name.text,
                        self.span(decl.span)
                    ),
                );
                self.prefix(&decl.prefix, indent + 1);
                if let Some(ty) = &decl.ty {
                    self.ty("Type", ty, indent + 1);
                }
                if let Some(initializer) = &decl.initializer {
                    self.expr(initializer, indent + 1);
                }
            }
            MemberDecl::Method(decl) => self.function("MethodDecl", decl, indent),
            MemberDecl::Constructor(decl) => {
                self.line(
                    indent,
                    format_args!("ConstructorDecl span={}", self.span(decl.span)),
                );
                self.prefix(&decl.prefix, indent + 1);
                for param in &decl.params {
                    self.param(param, indent + 1);
                }
                if let Some(body) = &decl.body {
                    self.block(body, indent + 1);
                }
            }
            MemberDecl::Const(decl) => self.const_decl("MemberConstDecl", decl, indent),
            MemberDecl::Let(decl) => self.let_decl("MemberLetDecl", decl, indent),
            MemberDecl::Error(error) => self.error(error, indent),
        }
    }

    fn function(&mut self, label: &str, decl: &FunctionDecl, indent: usize) {
        self.line(
            indent,
            format_args!(
                "{label} name={} span={}",
                decl.name.text,
                self.span(decl.span)
            ),
        );
        self.prefix(&decl.prefix, indent + 1);
        for param in &decl.params {
            self.param(param, indent + 1);
        }
        if let Some(ty) = &decl.return_type {
            self.ty("ReturnType", ty, indent + 1);
        } else {
            self.line(indent + 1, format_args!("ReturnType <omitted>"));
        }
        if let Some(body) = &decl.body {
            self.block(body, indent + 1);
        } else {
            self.line(indent + 1, format_args!("Body <omitted>"));
        }
    }

    fn const_decl(&mut self, label: &str, decl: &ConstDecl, indent: usize) {
        self.line(
            indent,
            format_args!(
                "{label} name={} span={}",
                decl.name.text,
                self.span(decl.span)
            ),
        );
        if let Some(ty) = &decl.ty {
            self.ty("Type", ty, indent + 1);
        }
        if let Some(initializer) = &decl.initializer {
            self.expr(initializer, indent + 1);
        }
    }

    fn let_decl(&mut self, label: &str, decl: &LetDecl, indent: usize) {
        self.line(
            indent,
            format_args!(
                "{label} name={} span={}",
                decl.name.text,
                self.span(decl.span)
            ),
        );
        if let Some(ty) = &decl.ty {
            self.ty("Type", ty, indent + 1);
        }
        if let Some(initializer) = &decl.initializer {
            self.expr(initializer, indent + 1);
        }
    }

    fn prefix(&mut self, prefix: &DeclPrefix, indent: usize) {
        for attribute in &prefix.attributes {
            self.line(
                indent,
                format_args!(
                    "Attribute path={} span={}",
                    self.path(&attribute.path),
                    self.span(attribute.span)
                ),
            );
        }
        for modifier in &prefix.modifiers {
            self.line(
                indent,
                format_args!(
                    "Modifier {:?} span={}",
                    modifier.keyword,
                    self.span(modifier.span)
                ),
            );
        }
    }

    fn param(&mut self, param: &Param, indent: usize) {
        self.line(
            indent,
            format_args!(
                "Param name={} span={}",
                param.name.text,
                self.span(param.span)
            ),
        );
        if let Some(ty) = &param.ty {
            self.ty("Type", ty, indent + 1);
        }
        if let Some(default_value) = &param.default_value {
            self.expr(default_value, indent + 1);
        }
    }

    fn block(&mut self, block: &Block, indent: usize) {
        self.line(indent, format_args!("Block span={}", self.span(block.span)));
        for statement in &block.statements {
            self.stmt(statement, indent + 1);
        }
    }

    fn stmt(&mut self, stmt: &Stmt, indent: usize) {
        match stmt {
            Stmt::Let(decl) => self.let_decl("LocalLet", decl, indent),
            Stmt::Const(decl) => self.const_decl("LocalConst", decl, indent),
            Stmt::Expr { expr, span } => {
                self.line(indent, format_args!("ExprStmt span={}", self.span(*span)));
                self.expr(expr, indent + 1);
            }
            Stmt::Return { value, span } => {
                self.line(indent, format_args!("ReturnStmt span={}", self.span(*span)));
                if let Some(value) = value {
                    self.expr(value, indent + 1);
                }
            }
            Stmt::If(stmt) => {
                self.line(indent, format_args!("IfStmt span={}", self.span(stmt.span)));
                self.expr(&stmt.condition, indent + 1);
                self.stmt(&stmt.then_branch, indent + 1);
                if let Some(else_branch) = &stmt.else_branch {
                    self.stmt(else_branch, indent + 1);
                }
            }
            Stmt::Switch(stmt) => {
                self.line(
                    indent,
                    format_args!("SwitchStmt span={}", self.span(stmt.span)),
                );
                self.expr(&stmt.expression, indent + 1);
                for case in &stmt.cases {
                    self.line(
                        indent + 1,
                        format_args!("SwitchCase span={}", self.span(case.span)),
                    );
                    if let Some(label) = &case.label {
                        self.expr(label, indent + 2);
                    } else {
                        self.line(indent + 2, format_args!("Default"));
                    }
                    for statement in &case.statements {
                        self.stmt(statement, indent + 2);
                    }
                }
            }
            Stmt::Match(stmt) => {
                self.line(
                    indent,
                    format_args!("MatchStmt span={}", self.span(stmt.span)),
                );
                self.expr(&stmt.expression, indent + 1);
                for arm in &stmt.arms {
                    self.line(
                        indent + 1,
                        format_args!("MatchArm span={}", self.span(arm.span)),
                    );
                    self.pattern(&arm.pattern, indent + 2);
                    for statement in &arm.statements {
                        self.stmt(statement, indent + 2);
                    }
                }
            }
            Stmt::While(stmt) => {
                self.line(
                    indent,
                    format_args!("WhileStmt span={}", self.span(stmt.span)),
                );
                self.expr(&stmt.condition, indent + 1);
                self.stmt(&stmt.body, indent + 1);
            }
            Stmt::For(stmt) => {
                self.line(
                    indent,
                    format_args!("ForStmt span={}", self.span(stmt.span)),
                );
                if let Some(initializer) = &stmt.initializer {
                    self.stmt(initializer, indent + 1);
                }
                if let Some(condition) = &stmt.condition {
                    self.expr(condition, indent + 1);
                }
                if let Some(increment) = &stmt.increment {
                    self.expr(increment, indent + 1);
                }
                self.stmt(&stmt.body, indent + 1);
            }
            Stmt::Foreach(stmt) => {
                self.line(
                    indent,
                    format_args!("ForeachStmt span={}", self.span(stmt.span)),
                );
                self.pattern(&stmt.binding, indent + 1);
                self.expr(&stmt.iterable, indent + 1);
                self.stmt(&stmt.body, indent + 1);
            }
            Stmt::Break(span) => {
                self.line(indent, format_args!("BreakStmt span={}", self.span(*span)))
            }
            Stmt::Continue(span) => {
                self.line(
                    indent,
                    format_args!("ContinueStmt span={}", self.span(*span)),
                );
            }
            Stmt::UnsafeBlock(block) => {
                self.line(
                    indent,
                    format_args!("UnsafeBlock span={}", self.span(block.span)),
                );
                self.block(block, indent + 1);
            }
            Stmt::Block(block) => self.block(block, indent),
            Stmt::Error(error) => self.error(error, indent),
        }
    }

    fn ty(&mut self, label: &str, ty: &TypeSyntax, indent: usize) {
        match ty {
            TypeSyntax::Path(path) => self.line(
                indent,
                format_args!(
                    "{label} Path {} span={}",
                    self.path(path),
                    self.span(path.span)
                ),
            ),
            TypeSyntax::Generic {
                base,
                arguments,
                span,
            } => {
                self.line(
                    indent,
                    format_args!(
                        "{label} Generic base={} span={}",
                        self.path(base),
                        self.span(*span)
                    ),
                );
                for argument in arguments {
                    self.ty("TypeArg", argument, indent + 1);
                }
            }
            TypeSyntax::Array {
                element,
                size,
                span,
            } => {
                self.line(
                    indent,
                    format_args!("{label} Array span={}", self.span(*span)),
                );
                self.ty("Element", element, indent + 1);
                if let Some(size) = size {
                    self.expr(size, indent + 1);
                }
            }
            TypeSyntax::Tuple { elements, span } => {
                self.line(
                    indent,
                    format_args!("{label} Tuple span={}", self.span(*span)),
                );
                for element in elements {
                    self.ty("Element", element, indent + 1);
                }
            }
            TypeSyntax::Error(error) => self.error(error, indent),
        }
    }

    fn expr(&mut self, expr: &Expr, indent: usize) {
        match expr {
            Expr::Literal(literal) => self.line(
                indent,
                format_args!(
                    "Literal {:?} \"{}\" span={}",
                    literal.kind,
                    literal.lexeme.escape_default(),
                    self.span(literal.span)
                ),
            ),
            Expr::Name(path) => self.line(
                indent,
                format_args!("Name {} span={}", self.path(path), self.span(path.span)),
            ),
            Expr::This(span) => self.line(indent, format_args!("This span={}", self.span(*span))),
            Expr::New(expr) => {
                self.line(
                    indent,
                    format_args!("NewExpr span={}", self.span(expr.span)),
                );
                self.ty("Type", &expr.ty, indent + 1);
                for argument in &expr.arguments {
                    self.expr(argument, indent + 1);
                }
            }
            Expr::Call(expr) => {
                self.line(
                    indent,
                    format_args!("CallExpr span={}", self.span(expr.span)),
                );
                self.expr(&expr.callee, indent + 1);
                for argument in &expr.arguments {
                    self.expr(argument, indent + 1);
                }
            }
            Expr::Member(expr) => {
                self.line(
                    indent,
                    format_args!(
                        "MemberExpr member={} span={}",
                        expr.member.text,
                        self.span(expr.span)
                    ),
                );
                self.expr(&expr.base, indent + 1);
            }
            Expr::Index(expr) => {
                self.line(
                    indent,
                    format_args!("IndexExpr span={}", self.span(expr.span)),
                );
                self.expr(&expr.base, indent + 1);
                self.expr(&expr.index, indent + 1);
            }
            Expr::Unary(expr) => {
                self.line(
                    indent,
                    format_args!("UnaryExpr op={:?} span={}", expr.op, self.span(expr.span)),
                );
                self.expr(&expr.expr, indent + 1);
            }
            Expr::Binary(expr) => {
                self.line(
                    indent,
                    format_args!("BinaryExpr op={:?} span={}", expr.op, self.span(expr.span)),
                );
                self.expr(&expr.left, indent + 1);
                self.expr(&expr.right, indent + 1);
            }
            Expr::Assign(expr) => {
                self.line(
                    indent,
                    format_args!("AssignExpr span={}", self.span(expr.span)),
                );
                self.expr(&expr.target, indent + 1);
                self.expr(&expr.value, indent + 1);
            }
            Expr::Group { expr, span } => {
                self.line(indent, format_args!("GroupExpr span={}", self.span(*span)));
                self.expr(expr, indent + 1);
            }
            Expr::Tuple { elements, span } => {
                self.line(indent, format_args!("TupleExpr span={}", self.span(*span)));
                for element in elements {
                    self.expr(element, indent + 1);
                }
            }
            Expr::Array { elements, span } => {
                self.line(indent, format_args!("ArrayExpr span={}", self.span(*span)));
                for element in elements {
                    self.expr(element, indent + 1);
                }
            }
            Expr::Error(error) => self.error(error, indent),
        }
    }

    fn pattern(&mut self, pattern: &Pattern, indent: usize) {
        match pattern {
            Pattern::Path(path) => self.line(
                indent,
                format_args!(
                    "Pattern Path {} span={}",
                    self.path(path),
                    self.span(path.span)
                ),
            ),
            Pattern::Constructor { path, fields, span } => {
                self.line(
                    indent,
                    format_args!(
                        "Pattern Constructor {} span={}",
                        self.path(path),
                        self.span(*span)
                    ),
                );
                for field in fields {
                    self.pattern(field, indent + 1);
                }
            }
            Pattern::Literal(literal) => self.line(
                indent,
                format_args!(
                    "Pattern Literal {:?} \"{}\" span={}",
                    literal.kind,
                    literal.lexeme.escape_default(),
                    self.span(literal.span)
                ),
            ),
            Pattern::Wildcard(span) => {
                self.line(
                    indent,
                    format_args!("Pattern Wildcard span={}", self.span(*span)),
                );
            }
            Pattern::Error(error) => self.error(error, indent),
        }
    }

    fn error(&mut self, error: &AstErrorNode, indent: usize) {
        self.line(
            indent,
            format_args!(
                "Error message=\"{}\" span={}",
                error.message.escape_default(),
                self.span(error.span)
            ),
        );
    }

    fn path(&self, path: &Path) -> String {
        path.segments
            .iter()
            .map(|segment| {
                if segment.text.is_empty() {
                    self.sources
                        .span_text(segment.span)
                        .map_or_else(|| "<missing>".to_string(), ToString::to_string)
                } else {
                    segment.text.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(".")
    }
}
