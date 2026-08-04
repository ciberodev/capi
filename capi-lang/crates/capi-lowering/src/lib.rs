//! AST-to-HIR lowering.

use capi_ast as ast;
use capi_diagnostics::{Diagnostic, DiagnosticCode, DiagnosticLabel};
use capi_hir::*;
use capi_source::{SourceMap, Span};

/// Deterministic AST-to-HIR relation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AstToHirMap {
    entries: Vec<(Span, HirId)>,
}

impl AstToHirMap {
    /// Records a relation between an AST span and a HIR element.
    pub fn insert(&mut self, span: Span, hir: HirId) {
        self.entries.push((span, hir));
    }

    /// Returns all recorded entries.
    pub fn entries(&self) -> &[(Span, HirId)] {
        &self.entries
    }
}

/// Output of AST lowering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AstLoweringOutput {
    hir: Option<Hir>,
    ast_to_hir: AstToHirMap,
    diagnostics: Vec<Diagnostic>,
    blocked: bool,
}

impl AstLoweringOutput {
    /// Returns lowered HIR.
    pub const fn hir(&self) -> Option<&Hir> {
        self.hir.as_ref()
    }

    /// Returns the AST-to-HIR map.
    pub const fn ast_to_hir(&self) -> &AstToHirMap {
        &self.ast_to_hir
    }

    /// Returns lowering diagnostics.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Returns true when lowering was blocked.
    pub const fn blocked(&self) -> bool {
        self.blocked
    }

    /// Consumes this output into parts.
    pub fn into_parts(self) -> (Option<Hir>, AstToHirMap, Vec<Diagnostic>, bool) {
        (self.hir, self.ast_to_hir, self.diagnostics, self.blocked)
    }
}

/// Lowers AST to initial HIR.
pub fn lower_ast(ast: &ast::Ast, _sources: &SourceMap) -> AstLoweringOutput {
    Lowerer::new(ast).lower()
}

struct Lowerer<'a> {
    ast: &'a ast::Ast,
    hir: Hir,
    ast_to_hir: AstToHirMap,
    diagnostics: Vec<Diagnostic>,
    blocked: bool,
}

impl<'a> Lowerer<'a> {
    fn new(ast: &'a ast::Ast) -> Self {
        Self {
            ast,
            hir: Hir::default(),
            ast_to_hir: AstToHirMap::default(),
            diagnostics: Vec::new(),
            blocked: false,
        }
    }

    fn lower(mut self) -> AstLoweringOutput {
        self.lower_unit(self.ast.root());
        let hir = (!self.blocked).then_some(self.hir);
        AstLoweringOutput {
            hir,
            ast_to_hir: self.ast_to_hir,
            diagnostics: self.diagnostics,
            blocked: self.blocked,
        }
    }

    fn origin(&self, span: Span) -> HirOrigin {
        HirOrigin {
            source: span.source(),
            span,
        }
    }

    fn lower_unit(&mut self, unit: &ast::CompilationUnit) -> HirUnitId {
        let id = HirUnitId::from_raw(self.hir.units.len() as u32);
        let module = unit
            .module
            .as_ref()
            .map(|module| HirModulePath::Explicit(self.path(&module.path)))
            .unwrap_or(HirModulePath::Implicit);
        let imports = unit
            .imports
            .iter()
            .map(|import| self.lower_import(import))
            .collect::<Vec<_>>();
        let items = unit
            .declarations
            .iter()
            .map(|decl| self.lower_decl(decl))
            .collect::<Vec<_>>();
        self.hir.units.push(HirUnit {
            id,
            source: unit.source,
            module,
            imports,
            items,
            origin: self.origin(unit.span),
            valid: !self.blocked,
        });
        self.ast_to_hir.insert(unit.span, HirId::Unit(id));
        id
    }

    fn lower_import(&mut self, import: &ast::ImportDecl) -> HirImportId {
        let id = HirImportId::from_raw(self.hir.imports.len() as u32);
        self.hir.imports.push(HirImport {
            id,
            path: self.path(&import.path),
            wildcard: import.wildcard,
            origin: self.origin(import.span),
        });
        self.ast_to_hir.insert(import.span, HirId::Import(id));
        id
    }

    fn lower_decl(&mut self, decl: &ast::Decl) -> HirItemId {
        let id = HirItemId::from_raw(self.hir.items.len() as u32);
        let (kind, span) = match decl {
            ast::Decl::Function(decl) => (HirItemKind::Function(self.function(decl)), decl.span),
            ast::Decl::Class(decl) => (HirItemKind::Class(self.type_item(decl)), decl.span),
            ast::Decl::Interface(decl) => {
                (HirItemKind::Interface(self.interface_item(decl)), decl.span)
            }
            ast::Decl::Trait(decl) => (HirItemKind::Trait(self.trait_item(decl)), decl.span),
            ast::Decl::Const(decl) => (HirItemKind::Const(self.const_decl(decl)), decl.span),
            ast::Decl::Let(decl) => (HirItemKind::Let(self.local(decl, true)), decl.span),
            ast::Decl::Error(error) => (HirItemKind::Error(self.error(error)), error.span),
        };
        self.hir.items.push(HirItem {
            id,
            kind,
            origin: self.origin(span),
        });
        self.ast_to_hir.insert(span, HirId::Item(id));
        id
    }

    fn type_item(&mut self, decl: &ast::ClassDecl) -> HirTypeItem {
        HirTypeItem {
            name: self.name(&decl.name),
            generics: self.generics(&decl.generic_params),
            extends: decl.extends.as_ref().map(|ty| self.ty(ty)),
            implements: decl.implements.iter().map(|ty| self.ty(ty)).collect(),
            uses: decl.uses.iter().map(|ty| self.ty(ty)).collect(),
            members: decl
                .members
                .iter()
                .map(|member| self.lower_member(member))
                .collect(),
            modifiers: self.modifiers(&decl.prefix),
            attributes: self.attributes(&decl.prefix),
        }
    }

    fn interface_item(&mut self, decl: &ast::InterfaceDecl) -> HirTypeItem {
        HirTypeItem {
            name: self.name(&decl.name),
            generics: self.generics(&decl.generic_params),
            extends: None,
            implements: Vec::new(),
            uses: Vec::new(),
            members: decl
                .members
                .iter()
                .map(|member| self.lower_member(member))
                .collect(),
            modifiers: self.modifiers(&decl.prefix),
            attributes: self.attributes(&decl.prefix),
        }
    }

    fn trait_item(&mut self, decl: &ast::TraitDecl) -> HirTypeItem {
        HirTypeItem {
            name: self.name(&decl.name),
            generics: self.generics(&decl.generic_params),
            extends: None,
            implements: Vec::new(),
            uses: Vec::new(),
            members: decl
                .members
                .iter()
                .map(|member| self.lower_member(member))
                .collect(),
            modifiers: self.modifiers(&decl.prefix),
            attributes: self.attributes(&decl.prefix),
        }
    }

    fn lower_member(&mut self, member: &ast::MemberDecl) -> HirMemberId {
        let id = HirMemberId::from_raw(self.hir.members.len() as u32);
        let (kind, span) = match member {
            ast::MemberDecl::Field(decl) => (HirMemberKind::Field(self.field(decl)), decl.span),
            ast::MemberDecl::Method(decl) => {
                (HirMemberKind::Method(self.function(decl)), decl.span)
            }
            ast::MemberDecl::Constructor(decl) => (
                HirMemberKind::Constructor(self.constructor(decl)),
                decl.span,
            ),
            ast::MemberDecl::Const(decl) => {
                (HirMemberKind::Const(self.const_decl(decl)), decl.span)
            }
            ast::MemberDecl::Let(decl) => (HirMemberKind::Let(self.local(decl, true)), decl.span),
            ast::MemberDecl::Error(error) => (HirMemberKind::Error(self.error(error)), error.span),
        };
        self.hir.members.push(HirMember {
            id,
            kind,
            origin: self.origin(span),
        });
        self.ast_to_hir.insert(span, HirId::Member(id));
        id
    }

    fn function(&mut self, decl: &ast::FunctionDecl) -> HirFunction {
        HirFunction {
            name: self.name(&decl.name),
            generics: self.generics(&decl.generic_params),
            params: decl.params.iter().map(|param| self.param(param)).collect(),
            return_type: decl.return_type.as_ref().map(|ty| self.ty(ty)),
            body: decl.body.as_ref().map(|block| self.block(block)),
            modifiers: self.modifiers(&decl.prefix),
            attributes: self.attributes(&decl.prefix),
        }
    }

    fn constructor(&mut self, decl: &ast::ConstructorDecl) -> HirConstructor {
        HirConstructor {
            params: decl.params.iter().map(|param| self.param(param)).collect(),
            body: decl.body.as_ref().map(|block| self.block(block)),
            modifiers: self.modifiers(&decl.prefix),
            attributes: self.attributes(&decl.prefix),
        }
    }

    fn field(&mut self, decl: &ast::FieldDecl) -> HirField {
        HirField {
            name: self.name(&decl.name),
            ty: decl.ty.as_ref().map(|ty| self.ty(ty)),
            initializer: decl.initializer.as_ref().map(|expr| self.expr(expr)),
            modifiers: self.modifiers(&decl.prefix),
            attributes: self.attributes(&decl.prefix),
        }
    }

    fn const_decl(&mut self, decl: &ast::ConstDecl) -> HirConst {
        HirConst {
            name: self.name(&decl.name),
            ty: decl.ty.as_ref().map(|ty| self.ty(ty)),
            initializer: decl.initializer.as_ref().map(|expr| self.expr(expr)),
            modifiers: self.modifiers(&decl.prefix),
            attributes: self.attributes(&decl.prefix),
        }
    }

    fn param(&mut self, param: &ast::Param) -> HirParamId {
        let id = HirParamId::from_raw(self.hir.params.len() as u32);
        let name = self.name(&param.name);
        let ty = param.ty.as_ref().map(|ty| self.ty(ty));
        let default_value = param.default_value.as_ref().map(|expr| self.expr(expr));
        let origin = self.origin(param.span);
        self.hir.params.push(HirParam {
            id,
            name,
            ty,
            default_value,
            origin,
        });
        self.ast_to_hir.insert(param.span, HirId::Param(id));
        id
    }

    fn local(&mut self, decl: &ast::LetDecl, mutable: bool) -> HirLocalId {
        let id = HirLocalId::from_raw(self.hir.locals.len() as u32);
        let name = self.name(&decl.name);
        let ty = decl.ty.as_ref().map(|ty| self.ty(ty));
        let initializer = decl.initializer.as_ref().map(|expr| self.expr(expr));
        let origin = self.origin(decl.span);
        self.hir.locals.push(HirLocal {
            id,
            name,
            mutable,
            ty,
            initializer,
            origin,
        });
        self.ast_to_hir.insert(decl.span, HirId::Local(id));
        id
    }

    fn local_const(&mut self, decl: &ast::ConstDecl) -> HirLocalId {
        let id = HirLocalId::from_raw(self.hir.locals.len() as u32);
        let name = self.name(&decl.name);
        let ty = decl.ty.as_ref().map(|ty| self.ty(ty));
        let initializer = decl.initializer.as_ref().map(|expr| self.expr(expr));
        let origin = self.origin(decl.span);
        self.hir.locals.push(HirLocal {
            id,
            name,
            mutable: false,
            ty,
            initializer,
            origin,
        });
        self.ast_to_hir.insert(decl.span, HirId::Local(id));
        id
    }

    fn block(&mut self, block: &ast::Block) -> HirBlockId {
        let id = HirBlockId::from_raw(self.hir.blocks.len() as u32);
        self.hir.blocks.push(HirBlock {
            id,
            stmts: Vec::new(),
            origin: self.origin(block.span),
        });
        let stmts = block
            .statements
            .iter()
            .map(|stmt| self.stmt(stmt))
            .collect::<Vec<_>>();
        self.hir.blocks[id.raw() as usize] = HirBlock {
            id,
            stmts,
            origin: self.origin(block.span),
        };
        self.ast_to_hir.insert(block.span, HirId::Block(id));
        id
    }

    fn stmt_block(&mut self, stmt: &ast::Stmt) -> HirBlockId {
        match stmt {
            ast::Stmt::Block(block) | ast::Stmt::UnsafeBlock(block) => self.block(block),
            _ => {
                let id = HirBlockId::from_raw(self.hir.blocks.len() as u32);
                self.hir.blocks.push(HirBlock {
                    id,
                    stmts: Vec::new(),
                    origin: self.origin(stmt.span()),
                });
                let stmt_id = self.stmt(stmt);
                self.hir.blocks[id.raw() as usize] = HirBlock {
                    id,
                    stmts: vec![stmt_id],
                    origin: self.origin(stmt.span()),
                };
                id
            }
        }
    }

    fn stmt(&mut self, stmt: &ast::Stmt) -> HirStmtId {
        let id = HirStmtId::from_raw(self.hir.stmts.len() as u32);
        let span = stmt.span();
        self.hir.stmts.push(HirStmt {
            id,
            kind: HirStmtKind::Break,
            origin: self.origin(span),
        });
        let kind = match stmt {
            ast::Stmt::Let(decl) => HirStmtKind::Local(self.local(decl, true)),
            ast::Stmt::Const(decl) => HirStmtKind::Local(self.local_const(decl)),
            ast::Stmt::Expr { expr, .. } => HirStmtKind::Expr(self.expr(expr)),
            ast::Stmt::Return { value, .. } => {
                HirStmtKind::Return(value.as_ref().map(|expr| self.expr(expr)))
            }
            ast::Stmt::Break(_) => HirStmtKind::Break,
            ast::Stmt::Continue(_) => HirStmtKind::Continue,
            ast::Stmt::If(stmt) => HirStmtKind::If {
                condition: self.expr(&stmt.condition),
                then_branch: self.stmt_block(&stmt.then_branch),
                else_branch: stmt
                    .else_branch
                    .as_ref()
                    .map(|branch| self.stmt_block(branch)),
            },
            ast::Stmt::While(stmt) => HirStmtKind::While {
                condition: self.expr(&stmt.condition),
                body: self.stmt_block(&stmt.body),
            },
            ast::Stmt::For(stmt) => HirStmtKind::For {
                initializer: stmt.initializer.as_ref().map(|stmt| self.stmt(stmt)),
                condition: stmt.condition.as_ref().map(|expr| self.expr(expr)),
                increment: stmt.increment.as_ref().map(|expr| self.expr(expr)),
                body: self.stmt_block(&stmt.body),
            },
            ast::Stmt::Foreach(stmt) => HirStmtKind::For {
                initializer: None,
                condition: Some(self.expr(&stmt.iterable)),
                increment: None,
                body: self.stmt_block(&stmt.body),
            },
            ast::Stmt::Switch(stmt) => HirStmtKind::Switch {
                expression: self.expr(&stmt.expression),
                cases: stmt
                    .cases
                    .iter()
                    .map(|case| HirSwitchCase {
                        label: case.label.as_ref().map(|expr| self.expr(expr)),
                        stmts: case.statements.iter().map(|stmt| self.stmt(stmt)).collect(),
                        origin: self.origin(case.span),
                    })
                    .collect(),
            },
            ast::Stmt::Match(stmt) => HirStmtKind::Match {
                expression: self.expr(&stmt.expression),
                arms: stmt
                    .arms
                    .iter()
                    .map(|arm| HirMatchArm {
                        pattern: self.pattern(&arm.pattern),
                        stmts: arm.statements.iter().map(|stmt| self.stmt(stmt)).collect(),
                        origin: self.origin(arm.span),
                    })
                    .collect(),
            },
            ast::Stmt::UnsafeBlock(block) | ast::Stmt::Block(block) => {
                HirStmtKind::Block(self.block(block))
            }
            ast::Stmt::Error(error) => HirStmtKind::Error(self.error(error)),
        };
        self.hir.stmts[id.raw() as usize] = HirStmt {
            id,
            kind,
            origin: self.origin(span),
        };
        self.ast_to_hir.insert(span, HirId::Stmt(id));
        id
    }

    fn expr(&mut self, expr: &ast::Expr) -> HirExprId {
        if let ast::Expr::Group { expr, .. } = expr {
            return self.expr(expr);
        }
        let id = HirExprId::from_raw(self.hir.exprs.len() as u32);
        let span = expr.span();
        self.hir.exprs.push(HirExpr {
            id,
            kind: HirExprKind::Tuple(Vec::new()),
            origin: self.origin(span),
        });
        let kind = match expr {
            ast::Expr::Literal(literal) => HirExprKind::Literal {
                kind: literal.kind,
                lexeme: literal.lexeme.clone(),
            },
            ast::Expr::Name(path) => HirExprKind::Path(self.path(path)),
            ast::Expr::This(_) => HirExprKind::This,
            ast::Expr::New(expr) => HirExprKind::New {
                ty: self.ty(&expr.ty),
                arguments: expr.arguments.iter().map(|expr| self.expr(expr)).collect(),
            },
            ast::Expr::Call(expr) => HirExprKind::Call {
                callee: self.expr(&expr.callee),
                arguments: expr.arguments.iter().map(|expr| self.expr(expr)).collect(),
            },
            ast::Expr::Member(expr) => HirExprKind::Member {
                base: self.expr(&expr.base),
                member: self.name(&expr.member),
            },
            ast::Expr::Index(expr) => HirExprKind::Index {
                base: self.expr(&expr.base),
                index: self.expr(&expr.index),
            },
            ast::Expr::Unary(expr) => HirExprKind::Unary {
                op: expr.op,
                expr: self.expr(&expr.expr),
            },
            ast::Expr::Binary(expr) => HirExprKind::Binary {
                left: self.expr(&expr.left),
                op: expr.op,
                right: self.expr(&expr.right),
            },
            ast::Expr::Assign(expr) => HirExprKind::Assign {
                target: self.expr(&expr.target),
                value: self.expr(&expr.value),
            },
            ast::Expr::Tuple { elements, .. } => {
                HirExprKind::Tuple(elements.iter().map(|expr| self.expr(expr)).collect())
            }
            ast::Expr::Array { elements, .. } => {
                HirExprKind::Array(elements.iter().map(|expr| self.expr(expr)).collect())
            }
            ast::Expr::Group { .. } => {
                unreachable!("group expressions return before HIR allocation")
            }
            ast::Expr::Error(error) => HirExprKind::Error(self.error(error)),
        };
        self.hir.exprs[id.raw() as usize] = HirExpr {
            id,
            kind,
            origin: self.origin(span),
        };
        self.ast_to_hir.insert(span, HirId::Expr(id));
        id
    }

    fn ty(&mut self, ty: &ast::TypeSyntax) -> HirTypeRefId {
        let id = HirTypeRefId::from_raw(self.hir.type_refs.len() as u32);
        let span = ty.span();
        self.hir.type_refs.push(HirTypeRef {
            id,
            kind: HirTypeRefKind::Tuple(Vec::new()),
            origin: self.origin(span),
        });
        let kind = match ty {
            ast::TypeSyntax::Path(path) => HirTypeRefKind::Path(self.path(path)),
            ast::TypeSyntax::Generic {
                base, arguments, ..
            } => HirTypeRefKind::Generic {
                base: self.path(base),
                arguments: arguments.iter().map(|ty| self.ty(ty)).collect(),
            },
            ast::TypeSyntax::Array { element, size, .. } => HirTypeRefKind::Array {
                element: self.ty(element),
                size: size.as_ref().map(|expr| self.expr(expr)),
            },
            ast::TypeSyntax::Tuple { elements, .. } => {
                HirTypeRefKind::Tuple(elements.iter().map(|ty| self.ty(ty)).collect())
            }
            ast::TypeSyntax::Error(error) => HirTypeRefKind::Error(self.error(error)),
        };
        self.hir.type_refs[id.raw() as usize] = HirTypeRef {
            id,
            kind,
            origin: self.origin(span),
        };
        self.ast_to_hir.insert(span, HirId::TypeRef(id));
        id
    }

    fn pattern(&mut self, pattern: &ast::Pattern) -> HirPatternId {
        let id = HirPatternId::from_raw(self.hir.patterns.len() as u32);
        self.hir.patterns.push(HirPattern {
            id,
            kind: HirPatternKind::Wildcard,
            origin: self.origin(Self::pattern_span(pattern)),
        });
        let (kind, span) = match pattern {
            ast::Pattern::Path(path) => (HirPatternKind::Path(self.path(path)), path.span),
            ast::Pattern::Constructor { path, fields, span } => (
                HirPatternKind::Constructor {
                    path: self.path(path),
                    fields: fields.iter().map(|pattern| self.pattern(pattern)).collect(),
                },
                *span,
            ),
            ast::Pattern::Literal(literal) => (
                HirPatternKind::Literal {
                    kind: literal.kind,
                    lexeme: literal.lexeme.clone(),
                },
                literal.span,
            ),
            ast::Pattern::Wildcard(span) => (HirPatternKind::Wildcard, *span),
            ast::Pattern::Error(error) => (HirPatternKind::Error(self.error(error)), error.span),
        };
        self.hir.patterns[id.raw() as usize] = HirPattern {
            id,
            kind,
            origin: self.origin(span),
        };
        self.ast_to_hir.insert(span, HirId::Pattern(id));
        id
    }

    fn modifiers(&self, prefix: &ast::DeclPrefix) -> Vec<HirModifier> {
        prefix
            .modifiers
            .iter()
            .map(|modifier| HirModifier {
                keyword: modifier.keyword,
                span: modifier.span,
            })
            .collect()
    }

    fn pattern_span(pattern: &ast::Pattern) -> Span {
        match pattern {
            ast::Pattern::Path(path) => path.span,
            ast::Pattern::Constructor { span, .. } | ast::Pattern::Wildcard(span) => *span,
            ast::Pattern::Literal(literal) => literal.span,
            ast::Pattern::Error(error) => error.span,
        }
    }

    fn attributes(&mut self, prefix: &ast::DeclPrefix) -> Vec<HirAttribute> {
        prefix
            .attributes
            .iter()
            .map(|attribute| HirAttribute {
                path: self.path(&attribute.path),
                arguments: attribute
                    .arguments
                    .iter()
                    .map(|expr| self.expr(expr))
                    .collect(),
                origin: self.origin(attribute.span),
            })
            .collect()
    }

    fn generics(&self, generics: &[ast::GenericParam]) -> Vec<HirName> {
        generics
            .iter()
            .map(|generic| self.name(&generic.name))
            .collect()
    }

    fn name(&self, identifier: &ast::Identifier) -> HirName {
        HirName {
            text: identifier.text.clone(),
            span: identifier.span,
        }
    }

    fn path(&self, path: &ast::Path) -> UnresolvedPath {
        UnresolvedPath {
            segments: path
                .segments
                .iter()
                .map(|segment| self.name(segment))
                .collect(),
            span: path.span,
        }
    }

    fn error(&mut self, error: &ast::AstErrorNode) -> HirErrorId {
        self.blocked = true;
        let id = HirErrorId::from_raw(self.hir.errors.len() as u32);
        self.hir.errors.push(HirError {
            id,
            message: error.message.clone(),
            origin: self.origin(error.span),
        });
        self.ast_to_hir.insert(error.span, HirId::Error(id));
        self.diagnostics.push(
            Diagnostic::error(format!("cannot lower invalid AST node: {}", error.message))
                .with_code(DiagnosticCode::new("HIR0001"))
                .with_primary_span(error.span)
                .with_label(DiagnosticLabel::primary(
                    error.span,
                    "invalid syntax reached HIR lowering",
                )),
        );
        id
    }
}
