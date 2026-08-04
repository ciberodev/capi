//! High-level intermediate representation.

use std::fmt::{self, Write};

use capi_lexer::{Keyword, LiteralKind, Operator};
use capi_source::{SourceId, Span};

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(u32);

        impl $name {
            /// Creates an id from a raw value.
            pub const fn from_raw(raw: u32) -> Self {
                Self(raw)
            }

            /// Returns the raw id value.
            pub const fn raw(self) -> u32 {
                self.0
            }
        }
    };
}

id_type!(HirUnitId);
id_type!(HirImportId);
id_type!(HirItemId);
id_type!(HirMemberId);
id_type!(HirParamId);
id_type!(HirLocalId);
id_type!(HirBlockId);
id_type!(HirStmtId);
id_type!(HirExprId);
id_type!(HirTypeRefId);
id_type!(HirPatternId);
id_type!(HirErrorId);

/// Generic HIR id for maps and dumps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HirId {
    Unit(HirUnitId),
    Import(HirImportId),
    Item(HirItemId),
    Member(HirMemberId),
    Param(HirParamId),
    Local(HirLocalId),
    Block(HirBlockId),
    Stmt(HirStmtId),
    Expr(HirExprId),
    TypeRef(HirTypeRefId),
    Pattern(HirPatternId),
    Error(HirErrorId),
}

/// HIR origin data used by diagnostics and dumps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HirOrigin {
    pub source: SourceId,
    pub span: Span,
}

/// HIR root.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Hir {
    pub units: Vec<HirUnit>,
    pub imports: Vec<HirImport>,
    pub items: Vec<HirItem>,
    pub members: Vec<HirMember>,
    pub params: Vec<HirParam>,
    pub locals: Vec<HirLocal>,
    pub blocks: Vec<HirBlock>,
    pub stmts: Vec<HirStmt>,
    pub exprs: Vec<HirExpr>,
    pub type_refs: Vec<HirTypeRef>,
    pub patterns: Vec<HirPattern>,
    pub errors: Vec<HirError>,
}

/// A lowered source unit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirUnit {
    pub id: HirUnitId,
    pub source: SourceId,
    pub module: HirModulePath,
    pub imports: Vec<HirImportId>,
    pub items: Vec<HirItemId>,
    pub origin: HirOrigin,
    pub valid: bool,
}

/// Module path for a unit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HirModulePath {
    Explicit(UnresolvedPath),
    Implicit,
}

/// A pending import.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirImport {
    pub id: HirImportId,
    pub path: UnresolvedPath,
    pub wildcard: bool,
    pub origin: HirOrigin,
}

/// A name written by the user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirName {
    pub text: String,
    pub span: Span,
}

/// A path that has not been resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnresolvedPath {
    pub segments: Vec<HirName>,
    pub span: Span,
}

/// A top-level item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirItem {
    pub id: HirItemId,
    pub kind: HirItemKind,
    pub origin: HirOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HirItemKind {
    Function(HirFunction),
    Class(HirTypeItem),
    Interface(HirTypeItem),
    Trait(HirTypeItem),
    Const(HirConst),
    Let(HirLocalId),
    Error(HirErrorId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirTypeItem {
    pub name: HirName,
    pub generics: Vec<HirName>,
    pub extends: Option<HirTypeRefId>,
    pub implements: Vec<HirTypeRefId>,
    pub uses: Vec<HirTypeRefId>,
    pub members: Vec<HirMemberId>,
    pub modifiers: Vec<HirModifier>,
    pub attributes: Vec<HirAttribute>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirMember {
    pub id: HirMemberId,
    pub kind: HirMemberKind,
    pub origin: HirOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HirMemberKind {
    Field(HirField),
    Method(HirFunction),
    Constructor(HirConstructor),
    Const(HirConst),
    Let(HirLocalId),
    Error(HirErrorId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirField {
    pub name: HirName,
    pub ty: Option<HirTypeRefId>,
    pub initializer: Option<HirExprId>,
    pub modifiers: Vec<HirModifier>,
    pub attributes: Vec<HirAttribute>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirFunction {
    pub name: HirName,
    pub generics: Vec<HirName>,
    pub params: Vec<HirParamId>,
    pub return_type: Option<HirTypeRefId>,
    pub body: Option<HirBlockId>,
    pub modifiers: Vec<HirModifier>,
    pub attributes: Vec<HirAttribute>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirConstructor {
    pub params: Vec<HirParamId>,
    pub body: Option<HirBlockId>,
    pub modifiers: Vec<HirModifier>,
    pub attributes: Vec<HirAttribute>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirConst {
    pub name: HirName,
    pub ty: Option<HirTypeRefId>,
    pub initializer: Option<HirExprId>,
    pub modifiers: Vec<HirModifier>,
    pub attributes: Vec<HirAttribute>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirParam {
    pub id: HirParamId,
    pub name: HirName,
    pub ty: Option<HirTypeRefId>,
    pub default_value: Option<HirExprId>,
    pub origin: HirOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirLocal {
    pub id: HirLocalId,
    pub name: HirName,
    pub mutable: bool,
    pub ty: Option<HirTypeRefId>,
    pub initializer: Option<HirExprId>,
    pub origin: HirOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirModifier {
    pub keyword: Keyword,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirAttribute {
    pub path: UnresolvedPath,
    pub arguments: Vec<HirExprId>,
    pub origin: HirOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirBlock {
    pub id: HirBlockId,
    pub stmts: Vec<HirStmtId>,
    pub origin: HirOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirStmt {
    pub id: HirStmtId,
    pub kind: HirStmtKind,
    pub origin: HirOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HirStmtKind {
    Local(HirLocalId),
    Expr(HirExprId),
    Return(Option<HirExprId>),
    Break,
    Continue,
    If {
        condition: HirExprId,
        then_branch: HirBlockId,
        else_branch: Option<HirBlockId>,
    },
    While {
        condition: HirExprId,
        body: HirBlockId,
    },
    For {
        initializer: Option<HirStmtId>,
        condition: Option<HirExprId>,
        increment: Option<HirExprId>,
        body: HirBlockId,
    },
    Switch {
        expression: HirExprId,
        cases: Vec<HirSwitchCase>,
    },
    Match {
        expression: HirExprId,
        arms: Vec<HirMatchArm>,
    },
    Block(HirBlockId),
    Error(HirErrorId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirSwitchCase {
    pub label: Option<HirExprId>,
    pub stmts: Vec<HirStmtId>,
    pub origin: HirOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirMatchArm {
    pub pattern: HirPatternId,
    pub stmts: Vec<HirStmtId>,
    pub origin: HirOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirExpr {
    pub id: HirExprId,
    pub kind: HirExprKind,
    pub origin: HirOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HirExprKind {
    Literal {
        kind: LiteralKind,
        lexeme: String,
    },
    Path(UnresolvedPath),
    This,
    New {
        ty: HirTypeRefId,
        arguments: Vec<HirExprId>,
    },
    Call {
        callee: HirExprId,
        arguments: Vec<HirExprId>,
    },
    Member {
        base: HirExprId,
        member: HirName,
    },
    Index {
        base: HirExprId,
        index: HirExprId,
    },
    Unary {
        op: Operator,
        expr: HirExprId,
    },
    Binary {
        left: HirExprId,
        op: Operator,
        right: HirExprId,
    },
    Assign {
        target: HirExprId,
        value: HirExprId,
    },
    Tuple(Vec<HirExprId>),
    Array(Vec<HirExprId>),
    Error(HirErrorId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirTypeRef {
    pub id: HirTypeRefId,
    pub kind: HirTypeRefKind,
    pub origin: HirOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HirTypeRefKind {
    Path(UnresolvedPath),
    Generic {
        base: UnresolvedPath,
        arguments: Vec<HirTypeRefId>,
    },
    Array {
        element: HirTypeRefId,
        size: Option<HirExprId>,
    },
    Tuple(Vec<HirTypeRefId>),
    Error(HirErrorId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirPattern {
    pub id: HirPatternId,
    pub kind: HirPatternKind,
    pub origin: HirOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HirPatternKind {
    Path(UnresolvedPath),
    Constructor {
        path: UnresolvedPath,
        fields: Vec<HirPatternId>,
    },
    Literal {
        kind: LiteralKind,
        lexeme: String,
    },
    Wildcard,
    Error(HirErrorId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HirError {
    pub id: HirErrorId,
    pub message: String,
    pub origin: HirOrigin,
}

/// Dumps initial HIR in deterministic text format.
pub fn dump_hir(hir: &Hir) -> String {
    HirDumper {
        output: String::new(),
        hir,
    }
    .dump()
}

struct HirDumper<'a> {
    output: String,
    hir: &'a Hir,
}

impl HirDumper<'_> {
    fn dump(mut self) -> String {
        for unit in &self.hir.units {
            self.unit(unit, 0);
        }
        self.output
    }

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

    fn path(&self, path: &UnresolvedPath) -> String {
        path.segments
            .iter()
            .map(|segment| segment.text.as_str())
            .collect::<Vec<_>>()
            .join(".")
    }

    fn unit(&mut self, unit: &HirUnit, indent: usize) {
        let module = match &unit.module {
            HirModulePath::Explicit(path) => self.path(path),
            HirModulePath::Implicit => "<implicit>".to_string(),
        };
        self.line(
            indent,
            format_args!(
                "Unit unit{} source={} module={} span={}",
                unit.id.raw(),
                unit.source.raw(),
                module,
                self.span(unit.origin.span)
            ),
        );
        for import in &unit.imports {
            self.import(*import, indent + 1);
        }
        for item in &unit.items {
            self.item(*item, indent + 1);
        }
    }

    fn import(&mut self, id: HirImportId, indent: usize) {
        let import = &self.hir.imports[id.raw() as usize];
        let suffix = if import.wildcard { ".*" } else { "" };
        self.line(
            indent,
            format_args!(
                "Import import{} path={}{} span={}",
                id.raw(),
                self.path(&import.path),
                suffix,
                self.span(import.origin.span)
            ),
        );
    }

    fn item(&mut self, id: HirItemId, indent: usize) {
        let item = &self.hir.items[id.raw() as usize];
        match &item.kind {
            HirItemKind::Function(function) => {
                self.function("Function", id.raw(), function, indent)
            }
            HirItemKind::Class(ty) => self.type_item("Class", id.raw(), ty, indent),
            HirItemKind::Interface(ty) => self.type_item("Interface", id.raw(), ty, indent),
            HirItemKind::Trait(ty) => self.type_item("Trait", id.raw(), ty, indent),
            HirItemKind::Const(konst) => self.const_decl("Const", id.raw(), konst, indent),
            HirItemKind::Let(local) => self.line(
                indent,
                format_args!("ItemLet item{} local{}", id.raw(), local.raw()),
            ),
            HirItemKind::Error(error) => self.error(*error, indent),
        }
    }

    fn type_item(&mut self, label: &str, raw: u32, ty: &HirTypeItem, indent: usize) {
        self.line(
            indent,
            format_args!("{label} item{raw} name={} symbol=<pending>", ty.name.text),
        );
        for member in &ty.members {
            self.member(*member, indent + 1);
        }
    }

    fn member(&mut self, id: HirMemberId, indent: usize) {
        let member = &self.hir.members[id.raw() as usize];
        match &member.kind {
            HirMemberKind::Field(field) => self.line(
                indent,
                format_args!(
                    "Field member{} name={} symbol=<pending>",
                    id.raw(),
                    field.name.text
                ),
            ),
            HirMemberKind::Method(function) => {
                self.function("Method", id.raw(), function, indent);
            }
            HirMemberKind::Constructor(constructor) => self.line(
                indent,
                format_args!(
                    "Constructor member{} params={} symbol=<pending>",
                    id.raw(),
                    constructor.params.len()
                ),
            ),
            HirMemberKind::Const(konst) => self.const_decl("MemberConst", id.raw(), konst, indent),
            HirMemberKind::Let(local) => self.line(
                indent,
                format_args!("MemberLet member{} local{}", id.raw(), local.raw()),
            ),
            HirMemberKind::Error(error) => self.error(*error, indent),
        }
    }

    fn function(&mut self, label: &str, raw: u32, function: &HirFunction, indent: usize) {
        self.line(
            indent,
            format_args!(
                "{label} id={} name={} params={} symbol=<pending>",
                raw,
                function.name.text,
                function.params.len()
            ),
        );
        if let Some(body) = function.body {
            self.block(body, indent + 1);
        }
    }

    fn const_decl(&mut self, label: &str, raw: u32, konst: &HirConst, indent: usize) {
        self.line(
            indent,
            format_args!(
                "{label} id={} name={} symbol=<pending>",
                raw, konst.name.text
            ),
        );
    }

    fn block(&mut self, id: HirBlockId, indent: usize) {
        let block = &self.hir.blocks[id.raw() as usize];
        self.line(
            indent,
            format_args!(
                "Block block{} span={}",
                id.raw(),
                self.span(block.origin.span)
            ),
        );
        for stmt in &block.stmts {
            self.stmt(*stmt, indent + 1);
        }
    }

    fn stmt(&mut self, id: HirStmtId, indent: usize) {
        let stmt = &self.hir.stmts[id.raw() as usize];
        match &stmt.kind {
            HirStmtKind::Local(local) => {
                let local = &self.hir.locals[local.raw() as usize];
                self.line(
                    indent,
                    format_args!(
                        "Local local{} name={} symbol=<pending>",
                        local.id.raw(),
                        local.name.text
                    ),
                );
            }
            HirStmtKind::Expr(expr) => self.expr(*expr, indent),
            HirStmtKind::Return(value) => {
                self.line(indent, format_args!("Return stmt{}", id.raw()));
                if let Some(value) = value {
                    self.expr(*value, indent + 1);
                }
            }
            HirStmtKind::Block(block) => self.block(*block, indent),
            other => self.line(indent, format_args!("{:?} stmt{}", other, id.raw())),
        }
    }

    fn expr(&mut self, id: HirExprId, indent: usize) {
        let expr = &self.hir.exprs[id.raw() as usize];
        match &expr.kind {
            HirExprKind::Literal { kind, lexeme } => self.line(
                indent,
                format_args!(
                    "Literal expr{} {:?} \"{}\"",
                    id.raw(),
                    kind,
                    lexeme.escape_default()
                ),
            ),
            HirExprKind::Path(path) => self.line(
                indent,
                format_args!(
                    "Path expr{} {} binding=<pending>",
                    id.raw(),
                    self.path(path)
                ),
            ),
            other => self.line(indent, format_args!("{:?} expr{}", other, id.raw())),
        }
    }

    fn error(&mut self, id: HirErrorId, indent: usize) {
        let error = &self.hir.errors[id.raw() as usize];
        self.line(
            indent,
            format_args!(
                "Error error{} \"{}\" span={}",
                id.raw(),
                error.message.escape_default(),
                self.span(error.origin.span)
            ),
        );
    }
}
