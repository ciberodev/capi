//! Initial semantic analysis: scopes, symbols, and name resolution.

use std::collections::BTreeMap;
use std::fmt::{self, Write};

use capi_diagnostics::{Diagnostic, DiagnosticCode, DiagnosticLabel};
use capi_hir::{
    dump_hir, Hir, HirBlockId, HirExprId, HirExprKind, HirId, HirImportId, HirItemId, HirItemKind,
    HirLocalId, HirMemberId, HirMemberKind, HirParamId, HirPatternId, HirPatternKind, HirStmtId,
    HirStmtKind, HirTypeRefId, HirTypeRefKind, UnresolvedPath,
};
use capi_source::Span;

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

id_type!(ScopeId);
id_type!(SymbolId);
id_type!(NameRefId);

/// Namespace used by initial name resolution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Namespace {
    Value,
    Type,
    Module,
    Member,
}

/// Scope category.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopeKind {
    Global,
    Module,
    Type,
    Member,
    Function,
    Constructor,
    Block,
    Pattern,
    Error,
}

/// HIR owner of a scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeOwner {
    Global,
    Unit(u32),
    Item(HirItemId),
    Member(HirMemberId),
    Block(HirBlockId),
    Stmt(HirStmtId),
    Pattern(HirPatternId),
}

/// Scope data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScopeData {
    pub id: ScopeId,
    pub kind: ScopeKind,
    pub owner: ScopeOwner,
    pub parent: Option<ScopeId>,
    pub children: Vec<ScopeId>,
    pub span: Option<Span>,
}

/// Scope graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScopeGraph {
    pub root: ScopeId,
    pub scopes: Vec<ScopeData>,
    enclosing: BTreeMap<HirId, ScopeId>,
    owned: BTreeMap<ScopeOwner, ScopeId>,
}

impl ScopeGraph {
    /// Returns the enclosing scope for a HIR id.
    pub fn enclosing_scope(&self, id: HirId) -> Option<ScopeId> {
        self.enclosing.get(&id).copied()
    }

    /// Returns the scope owned by a HIR owner.
    pub fn owned_scope(&self, owner: ScopeOwner) -> Option<ScopeId> {
        self.owned.get(&owner).copied()
    }

    fn get(&self, id: ScopeId) -> &ScopeData {
        &self.scopes[id.raw() as usize]
    }
}

/// Symbol category.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    Module,
    Import,
    Class,
    Interface,
    Trait,
    Function,
    Method,
    Constructor,
    Field,
    Const,
    Param,
    Local,
    PatternBinding,
}

/// HIR declaration site for a symbol.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeclarationSite {
    Import(HirImportId),
    Item(HirItemId),
    Member(HirMemberId),
    Param(u32),
    Local(HirLocalId),
    Pattern(HirPatternId),
}

/// Symbol entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolEntry {
    pub id: SymbolId,
    pub name: String,
    pub kind: SymbolKind,
    pub namespace: Namespace,
    pub declaring_scope: ScopeId,
    pub declaration: DeclarationSite,
    pub span: Span,
}

/// Symbols sharing one name in one scope and namespace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolSet {
    pub primary: SymbolId,
    pub conflicts: Vec<SymbolId>,
}

/// Symbol table.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SymbolTable {
    pub symbols: Vec<SymbolEntry>,
    pub scopes: BTreeMap<ScopeId, BTreeMap<(Namespace, String), SymbolSet>>,
}

impl SymbolTable {
    fn insert(&mut self, entry: SymbolEntry) -> Option<SymbolId> {
        let key = (entry.namespace, entry.name.clone());
        let scope_symbols = self.scopes.entry(entry.declaring_scope).or_default();
        let previous = if let Some(set) = scope_symbols.get_mut(&key) {
            set.conflicts.push(entry.id);
            Some(set.primary)
        } else {
            scope_symbols.insert(
                key,
                SymbolSet {
                    primary: entry.id,
                    conflicts: Vec::new(),
                },
            );
            None
        };
        self.symbols.push(entry);
        previous
    }

    fn symbol(&self, id: SymbolId) -> &SymbolEntry {
        &self.symbols[id.raw() as usize]
    }

    fn lookup_local(&self, scope: ScopeId, namespace: Namespace, name: &str) -> Option<&SymbolSet> {
        self.scopes
            .get(&scope)
            .and_then(|symbols| symbols.get(&(namespace, name.to_string())))
    }
}

/// Binding produced by name resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResolvedBinding {
    Symbol(SymbolId),
    Ambiguous(Vec<SymbolId>),
    NotFound,
}

/// Name binding table.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NameBindingTable {
    pub by_hir: BTreeMap<HirId, ResolvedBinding>,
}

/// Semantic output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticOutput {
    scopes: ScopeGraph,
    symbols: SymbolTable,
    bindings: NameBindingTable,
    diagnostics: Vec<Diagnostic>,
}

impl SemanticOutput {
    /// Returns scopes.
    pub const fn scopes(&self) -> &ScopeGraph {
        &self.scopes
    }

    /// Returns symbols.
    pub const fn symbols(&self) -> &SymbolTable {
        &self.symbols
    }

    /// Returns bindings.
    pub const fn bindings(&self) -> &NameBindingTable {
        &self.bindings
    }

    /// Returns diagnostics.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

/// Runs Stage 3 semantic analysis.
pub fn analyze_names(hir: &Hir) -> SemanticOutput {
    let scopes = ScopeBuilder::new(hir).build();
    let mut resolver = Resolver {
        hir,
        scopes,
        symbols: SymbolTable::default(),
        bindings: NameBindingTable::default(),
        diagnostics: Vec::new(),
    };
    resolver.register_symbols();
    resolver.resolve_all();
    SemanticOutput {
        scopes: resolver.scopes,
        symbols: resolver.symbols,
        bindings: resolver.bindings,
        diagnostics: resolver.diagnostics,
    }
}

struct ScopeBuilder<'a> {
    hir: &'a Hir,
    scopes: Vec<ScopeData>,
    enclosing: BTreeMap<HirId, ScopeId>,
    owned: BTreeMap<ScopeOwner, ScopeId>,
}

impl<'a> ScopeBuilder<'a> {
    fn new(hir: &'a Hir) -> Self {
        Self {
            hir,
            scopes: Vec::new(),
            enclosing: BTreeMap::new(),
            owned: BTreeMap::new(),
        }
    }

    fn build(mut self) -> ScopeGraph {
        let root = self.push_scope(ScopeKind::Global, ScopeOwner::Global, None, None);
        for unit in &self.hir.units {
            let unit_scope = self.push_scope(
                ScopeKind::Module,
                ScopeOwner::Unit(unit.id.raw()),
                Some(root),
                Some(unit.origin.span),
            );
            self.enclosing.insert(HirId::Unit(unit.id), unit_scope);
            for import in &unit.imports {
                self.enclosing.insert(HirId::Import(*import), unit_scope);
            }
            for item in &unit.items {
                self.item(*item, unit_scope);
            }
        }
        ScopeGraph {
            root,
            scopes: self.scopes,
            enclosing: self.enclosing,
            owned: self.owned,
        }
    }

    fn push_scope(
        &mut self,
        kind: ScopeKind,
        owner: ScopeOwner,
        parent: Option<ScopeId>,
        span: Option<Span>,
    ) -> ScopeId {
        let id = ScopeId::from_raw(self.scopes.len() as u32);
        self.scopes.push(ScopeData {
            id,
            kind,
            owner,
            parent,
            children: Vec::new(),
            span,
        });
        if let Some(parent) = parent {
            self.scopes[parent.raw() as usize].children.push(id);
        }
        self.owned.insert(owner, id);
        id
    }

    fn item(&mut self, id: HirItemId, parent: ScopeId) {
        self.enclosing.insert(HirId::Item(id), parent);
        let item = &self.hir.items[id.raw() as usize];
        match &item.kind {
            HirItemKind::Function(function) => {
                let scope = self.push_scope(
                    ScopeKind::Function,
                    ScopeOwner::Item(id),
                    Some(parent),
                    Some(item.origin.span),
                );
                self.function_signature(function, scope);
                if let Some(body) = function.body {
                    self.block(body, scope);
                }
            }
            HirItemKind::Class(ty) | HirItemKind::Interface(ty) | HirItemKind::Trait(ty) => {
                if let Some(extends) = ty.extends {
                    self.ty(extends, parent);
                }
                for implemented in &ty.implements {
                    self.ty(*implemented, parent);
                }
                for used in &ty.uses {
                    self.ty(*used, parent);
                }
                let type_scope = self.push_scope(
                    ScopeKind::Type,
                    ScopeOwner::Item(id),
                    Some(parent),
                    Some(item.origin.span),
                );
                let member_scope = self.push_scope(
                    ScopeKind::Member,
                    ScopeOwner::Item(id),
                    Some(type_scope),
                    Some(item.origin.span),
                );
                for member in &ty.members {
                    self.member(*member, member_scope);
                }
            }
            HirItemKind::Let(local) => {
                self.enclosing.insert(HirId::Local(*local), parent);
                self.local(*local, parent);
            }
            HirItemKind::Const(konst) => {
                self.const_decl(konst, parent);
            }
            _ => {}
        }
    }

    fn member(&mut self, id: HirMemberId, parent: ScopeId) {
        self.enclosing.insert(HirId::Member(id), parent);
        let member = &self.hir.members[id.raw() as usize];
        match &member.kind {
            HirMemberKind::Method(function) => {
                let scope = self.push_scope(
                    ScopeKind::Function,
                    ScopeOwner::Member(id),
                    Some(parent),
                    Some(member.origin.span),
                );
                self.function_signature(function, scope);
                if let Some(body) = function.body {
                    self.block(body, scope);
                }
            }
            HirMemberKind::Constructor(constructor) => {
                let scope = self.push_scope(
                    ScopeKind::Constructor,
                    ScopeOwner::Member(id),
                    Some(parent),
                    Some(member.origin.span),
                );
                for param in &constructor.params {
                    self.param(*param, scope);
                }
                if let Some(body) = constructor.body {
                    self.block(body, scope);
                }
            }
            HirMemberKind::Let(local) => {
                self.enclosing.insert(HirId::Local(*local), parent);
                self.local(*local, parent);
            }
            HirMemberKind::Field(field) => {
                if let Some(ty) = field.ty {
                    self.ty(ty, parent);
                }
                if let Some(initializer) = field.initializer {
                    self.expr(initializer, parent);
                }
            }
            HirMemberKind::Const(konst) => {
                self.const_decl(konst, parent);
            }
            _ => {}
        }
    }

    fn block(&mut self, id: HirBlockId, parent: ScopeId) {
        let block = &self.hir.blocks[id.raw() as usize];
        let scope = self.push_scope(
            ScopeKind::Block,
            ScopeOwner::Block(id),
            Some(parent),
            Some(block.origin.span),
        );
        self.enclosing.insert(HirId::Block(id), scope);
        for stmt in &block.stmts {
            self.stmt(*stmt, scope);
        }
    }

    fn stmt(&mut self, id: HirStmtId, scope: ScopeId) {
        self.enclosing.insert(HirId::Stmt(id), scope);
        let stmt = &self.hir.stmts[id.raw() as usize];
        match &stmt.kind {
            HirStmtKind::Local(local) => {
                self.enclosing.insert(HirId::Local(*local), scope);
                self.local(*local, scope);
            }
            HirStmtKind::Expr(expr) | HirStmtKind::Return(Some(expr)) => self.expr(*expr, scope),
            HirStmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.expr(*condition, scope);
                self.block(*then_branch, scope);
                if let Some(else_branch) = else_branch {
                    self.block(*else_branch, scope);
                }
            }
            HirStmtKind::While { condition, body } => {
                self.expr(*condition, scope);
                self.block(*body, scope);
            }
            HirStmtKind::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                if let Some(initializer) = initializer {
                    self.stmt(*initializer, scope);
                }
                if let Some(condition) = condition {
                    self.expr(*condition, scope);
                }
                if let Some(increment) = increment {
                    self.expr(*increment, scope);
                }
                self.block(*body, scope);
            }
            HirStmtKind::Switch { expression, cases } => {
                self.expr(*expression, scope);
                for case in cases {
                    if let Some(label) = case.label {
                        self.expr(label, scope);
                    }
                    for stmt in &case.stmts {
                        self.stmt(*stmt, scope);
                    }
                }
            }
            HirStmtKind::Match { expression, arms } => {
                self.expr(*expression, scope);
                for arm in arms {
                    self.pattern(arm.pattern, scope);
                    for stmt in &arm.stmts {
                        self.stmt(*stmt, scope);
                    }
                }
            }
            HirStmtKind::Block(block) => self.block(*block, scope),
            _ => {}
        }
    }

    fn expr(&mut self, id: HirExprId, scope: ScopeId) {
        self.enclosing.insert(HirId::Expr(id), scope);
        let expr = &self.hir.exprs[id.raw() as usize];
        match &expr.kind {
            HirExprKind::New { ty, arguments } => {
                self.ty(*ty, scope);
                for argument in arguments {
                    self.expr(*argument, scope);
                }
            }
            HirExprKind::Call { callee, arguments } => {
                self.expr(*callee, scope);
                for argument in arguments {
                    self.expr(*argument, scope);
                }
            }
            HirExprKind::Member { base, .. } => self.expr(*base, scope),
            HirExprKind::Index { base, index } => {
                self.expr(*base, scope);
                self.expr(*index, scope);
            }
            HirExprKind::Unary { expr, .. } => self.expr(*expr, scope),
            HirExprKind::Binary { left, right, .. } => {
                self.expr(*left, scope);
                self.expr(*right, scope);
            }
            HirExprKind::Assign { target, value } => {
                self.expr(*target, scope);
                self.expr(*value, scope);
            }
            HirExprKind::Tuple(exprs) | HirExprKind::Array(exprs) => {
                for expr in exprs {
                    self.expr(*expr, scope);
                }
            }
            _ => {}
        }
    }

    fn function_signature(&mut self, function: &capi_hir::HirFunction, scope: ScopeId) {
        for param in &function.params {
            self.param(*param, scope);
        }
        if let Some(return_type) = function.return_type {
            self.ty(return_type, scope);
        }
    }

    fn param(&mut self, id: HirParamId, scope: ScopeId) {
        self.enclosing.insert(HirId::Param(id), scope);
        let param = &self.hir.params[id.raw() as usize];
        if let Some(ty) = param.ty {
            self.ty(ty, scope);
        }
        if let Some(default_value) = param.default_value {
            self.expr(default_value, scope);
        }
    }

    fn local(&mut self, id: HirLocalId, scope: ScopeId) {
        let local = &self.hir.locals[id.raw() as usize];
        if let Some(ty) = local.ty {
            self.ty(ty, scope);
        }
        if let Some(initializer) = local.initializer {
            self.expr(initializer, scope);
        }
    }

    fn const_decl(&mut self, konst: &capi_hir::HirConst, scope: ScopeId) {
        if let Some(ty) = konst.ty {
            self.ty(ty, scope);
        }
        if let Some(initializer) = konst.initializer {
            self.expr(initializer, scope);
        }
    }

    fn ty(&mut self, id: HirTypeRefId, scope: ScopeId) {
        self.enclosing.insert(HirId::TypeRef(id), scope);
        let ty = &self.hir.type_refs[id.raw() as usize];
        match &ty.kind {
            HirTypeRefKind::Generic { arguments, .. } => {
                for argument in arguments {
                    self.ty(*argument, scope);
                }
            }
            HirTypeRefKind::Array { element, size } => {
                self.ty(*element, scope);
                if let Some(size) = size {
                    self.expr(*size, scope);
                }
            }
            HirTypeRefKind::Tuple(elements) => {
                for element in elements {
                    self.ty(*element, scope);
                }
            }
            HirTypeRefKind::Path(_) | HirTypeRefKind::Error(_) => {}
        }
    }

    fn pattern(&mut self, id: HirPatternId, scope: ScopeId) {
        self.enclosing.insert(HirId::Pattern(id), scope);
        let pattern = &self.hir.patterns[id.raw() as usize];
        if let HirPatternKind::Constructor { fields, .. } = &pattern.kind {
            for field in fields {
                self.pattern(*field, scope);
            }
        }
    }
}

struct Resolver<'a> {
    hir: &'a Hir,
    scopes: ScopeGraph,
    symbols: SymbolTable,
    bindings: NameBindingTable,
    diagnostics: Vec<Diagnostic>,
}

impl Resolver<'_> {
    fn register_symbols(&mut self) {
        for unit in &self.hir.units {
            let scope = self
                .scopes
                .enclosing_scope(HirId::Unit(unit.id))
                .unwrap_or(self.scopes.root);
            for import in &unit.imports {
                self.register_import(*import, scope);
            }
            for item in &unit.items {
                self.register_item(*item, scope);
            }
        }
    }

    fn register_import(&mut self, id: HirImportId, scope: ScopeId) {
        let import = &self.hir.imports[id.raw() as usize];
        if let Some(name) = import.path.segments.last() {
            let symbol = self.insert_symbol(
                name.text.clone(),
                SymbolKind::Import,
                Namespace::Module,
                scope,
                DeclarationSite::Import(id),
                name.span,
            );
            self.bindings
                .by_hir
                .insert(HirId::Import(id), ResolvedBinding::Symbol(symbol));
        }
    }

    fn register_item(&mut self, id: HirItemId, scope: ScopeId) {
        let item = &self.hir.items[id.raw() as usize];
        match &item.kind {
            HirItemKind::Function(function) => {
                self.insert_symbol(
                    function.name.text.clone(),
                    SymbolKind::Function,
                    Namespace::Value,
                    scope,
                    DeclarationSite::Item(id),
                    function.name.span,
                );
                let function_scope = self
                    .scopes
                    .owned_scope(ScopeOwner::Item(id))
                    .unwrap_or(scope);
                self.register_params(&function.params, function_scope);
                if let Some(body) = function.body {
                    self.register_block(body);
                }
            }
            HirItemKind::Class(ty) | HirItemKind::Interface(ty) | HirItemKind::Trait(ty) => {
                let kind = match &item.kind {
                    HirItemKind::Class(_) => SymbolKind::Class,
                    HirItemKind::Interface(_) => SymbolKind::Interface,
                    HirItemKind::Trait(_) => SymbolKind::Trait,
                    _ => unreachable!(),
                };
                self.insert_symbol(
                    ty.name.text.clone(),
                    kind,
                    Namespace::Type,
                    scope,
                    DeclarationSite::Item(id),
                    ty.name.span,
                );
                let member_scope = ty
                    .members
                    .first()
                    .and_then(|member| self.scopes.enclosing_scope(HirId::Member(*member)))
                    .or_else(|| self.scopes.owned_scope(ScopeOwner::Item(id)))
                    .unwrap_or(scope);
                for member in &ty.members {
                    self.register_member(*member, member_scope);
                }
            }
            HirItemKind::Const(konst) => {
                self.insert_symbol(
                    konst.name.text.clone(),
                    SymbolKind::Const,
                    Namespace::Value,
                    scope,
                    DeclarationSite::Item(id),
                    konst.name.span,
                );
            }
            HirItemKind::Let(local) => self.register_local(*local),
            _ => {}
        }
    }

    fn register_member(&mut self, id: HirMemberId, scope: ScopeId) {
        let member = &self.hir.members[id.raw() as usize];
        match &member.kind {
            HirMemberKind::Field(field) => {
                self.insert_symbol(
                    field.name.text.clone(),
                    SymbolKind::Field,
                    Namespace::Member,
                    scope,
                    DeclarationSite::Member(id),
                    field.name.span,
                );
            }
            HirMemberKind::Method(function) => {
                self.insert_symbol(
                    function.name.text.clone(),
                    SymbolKind::Method,
                    Namespace::Member,
                    scope,
                    DeclarationSite::Member(id),
                    function.name.span,
                );
                let function_scope = self
                    .scopes
                    .owned_scope(ScopeOwner::Member(id))
                    .unwrap_or(scope);
                self.register_params(&function.params, function_scope);
                if let Some(body) = function.body {
                    self.register_block(body);
                }
            }
            HirMemberKind::Constructor(constructor) => {
                self.insert_symbol(
                    "constructor".to_string(),
                    SymbolKind::Constructor,
                    Namespace::Member,
                    scope,
                    DeclarationSite::Member(id),
                    member.origin.span,
                );
                let constructor_scope = self
                    .scopes
                    .owned_scope(ScopeOwner::Member(id))
                    .unwrap_or(scope);
                self.register_params(&constructor.params, constructor_scope);
                if let Some(body) = constructor.body {
                    self.register_block(body);
                }
            }
            HirMemberKind::Const(konst) => {
                self.insert_symbol(
                    konst.name.text.clone(),
                    SymbolKind::Const,
                    Namespace::Member,
                    scope,
                    DeclarationSite::Member(id),
                    konst.name.span,
                );
            }
            HirMemberKind::Let(local) => {
                self.register_local(*local);
            }
            _ => {}
        }
    }

    fn register_params(&mut self, params: &[capi_hir::HirParamId], scope: ScopeId) {
        for param in params {
            let param_data = &self.hir.params[param.raw() as usize];
            self.insert_symbol(
                param_data.name.text.clone(),
                SymbolKind::Param,
                Namespace::Value,
                scope,
                DeclarationSite::Param(param.raw()),
                param_data.name.span,
            );
        }
    }

    fn register_block(&mut self, id: HirBlockId) {
        let block = &self.hir.blocks[id.raw() as usize];
        for stmt in &block.stmts {
            let stmt = &self.hir.stmts[stmt.raw() as usize];
            match &stmt.kind {
                HirStmtKind::Local(local) => self.register_local(*local),
                HirStmtKind::Block(block) => self.register_block(*block),
                HirStmtKind::If {
                    then_branch,
                    else_branch,
                    ..
                } => {
                    self.register_block(*then_branch);
                    if let Some(else_branch) = else_branch {
                        self.register_block(*else_branch);
                    }
                }
                HirStmtKind::While { body, .. } | HirStmtKind::For { body, .. } => {
                    self.register_block(*body);
                }
                HirStmtKind::Switch { cases, .. } => {
                    for case in cases {
                        for stmt in &case.stmts {
                            self.register_stmt_locals(*stmt);
                        }
                    }
                }
                HirStmtKind::Match { arms, .. } => {
                    for arm in arms {
                        self.register_pattern(arm.pattern);
                        for stmt in &arm.stmts {
                            self.register_stmt_locals(*stmt);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn register_stmt_locals(&mut self, id: HirStmtId) {
        let stmt = &self.hir.stmts[id.raw() as usize];
        if let HirStmtKind::Local(local) = stmt.kind {
            self.register_local(local);
        }
    }

    fn register_local(&mut self, id: HirLocalId) {
        let local = &self.hir.locals[id.raw() as usize];
        if let Some(scope) = self.scopes.enclosing_scope(HirId::Local(id)) {
            self.insert_symbol(
                local.name.text.clone(),
                SymbolKind::Local,
                Namespace::Value,
                scope,
                DeclarationSite::Local(id),
                local.name.span,
            );
        }
    }

    fn register_pattern(&mut self, id: HirPatternId) {
        let pattern = &self.hir.patterns[id.raw() as usize];
        match &pattern.kind {
            HirPatternKind::Path(path) if path.segments.len() == 1 => {
                if let Some(scope) = self.scopes.enclosing_scope(HirId::Pattern(id)) {
                    let name = &path.segments[0];
                    self.insert_symbol(
                        name.text.clone(),
                        SymbolKind::PatternBinding,
                        Namespace::Value,
                        scope,
                        DeclarationSite::Pattern(id),
                        name.span,
                    );
                }
            }
            HirPatternKind::Constructor { fields, .. } => {
                for field in fields {
                    self.register_pattern(*field);
                }
            }
            _ => {}
        }
    }

    fn insert_symbol(
        &mut self,
        name: String,
        kind: SymbolKind,
        namespace: Namespace,
        scope: ScopeId,
        declaration: DeclarationSite,
        span: Span,
    ) -> SymbolId {
        let id = SymbolId::from_raw(self.symbols.symbols.len() as u32);
        let entry = SymbolEntry {
            id,
            name: name.clone(),
            kind,
            namespace,
            declaring_scope: scope,
            declaration,
            span,
        };
        if let Some(previous) = self.symbols.insert(entry) {
            self.diagnostics.push(
                Diagnostic::error(format!("duplicate symbol `{name}`"))
                    .with_code(DiagnosticCode::new("SEM0001"))
                    .with_primary_span(span)
                    .with_label(DiagnosticLabel::primary(span, "duplicate declaration"))
                    .with_label(DiagnosticLabel::secondary(
                        self.symbols.symbol(previous).span,
                        "previous declaration is here",
                    )),
            );
        }
        id
    }

    fn resolve_all(&mut self) {
        for ty in &self.hir.type_refs {
            self.resolve_type(ty.id);
        }
        for expr in &self.hir.exprs {
            self.resolve_expr(expr.id);
        }
        for pattern in &self.hir.patterns {
            self.resolve_pattern(pattern.id);
        }
    }

    fn resolve_type(&mut self, id: HirTypeRefId) {
        let ty = &self.hir.type_refs[id.raw() as usize];
        match &ty.kind {
            HirTypeRefKind::Path(path) => {
                self.resolve_path(HirId::TypeRef(id), path, Namespace::Type, ty.origin.span);
            }
            HirTypeRefKind::Generic {
                base, arguments, ..
            } => {
                self.resolve_path(HirId::TypeRef(id), base, Namespace::Type, ty.origin.span);
                for argument in arguments {
                    self.resolve_type(*argument);
                }
            }
            HirTypeRefKind::Array { element, size } => {
                self.resolve_type(*element);
                if let Some(size) = size {
                    self.resolve_expr(*size);
                }
            }
            HirTypeRefKind::Tuple(elements) => {
                for element in elements {
                    self.resolve_type(*element);
                }
            }
            HirTypeRefKind::Error(_) => {}
        }
    }

    fn resolve_expr(&mut self, id: HirExprId) {
        let expr = &self.hir.exprs[id.raw() as usize];
        match &expr.kind {
            HirExprKind::Path(path) => {
                self.resolve_path(HirId::Expr(id), path, Namespace::Value, expr.origin.span);
            }
            HirExprKind::New { ty, arguments } => {
                self.resolve_type(*ty);
                for argument in arguments {
                    self.resolve_expr(*argument);
                }
            }
            HirExprKind::Call { callee, arguments } => {
                self.resolve_expr(*callee);
                for argument in arguments {
                    self.resolve_expr(*argument);
                }
            }
            HirExprKind::Member { base, .. } => {
                self.resolve_expr(*base);
            }
            HirExprKind::Index { base, index } => {
                self.resolve_expr(*base);
                self.resolve_expr(*index);
            }
            HirExprKind::Unary { expr, .. } => self.resolve_expr(*expr),
            HirExprKind::Binary { left, right, .. } => {
                self.resolve_expr(*left);
                self.resolve_expr(*right);
            }
            HirExprKind::Assign { target, value } => {
                self.resolve_expr(*target);
                self.resolve_expr(*value);
            }
            HirExprKind::Tuple(exprs) | HirExprKind::Array(exprs) => {
                for expr in exprs {
                    self.resolve_expr(*expr);
                }
            }
            _ => {}
        }
    }

    fn resolve_pattern(&mut self, id: HirPatternId) {
        let pattern = &self.hir.patterns[id.raw() as usize];
        match &pattern.kind {
            HirPatternKind::Constructor { path, fields } => {
                self.resolve_path(
                    HirId::Pattern(id),
                    path,
                    Namespace::Type,
                    pattern.origin.span,
                );
                for field in fields {
                    self.resolve_pattern(*field);
                }
            }
            HirPatternKind::Path(path) if path.segments.len() > 1 => {
                self.resolve_path(
                    HirId::Pattern(id),
                    path,
                    Namespace::Value,
                    pattern.origin.span,
                );
            }
            _ => {}
        }
    }

    fn resolve_path(
        &mut self,
        hir_id: HirId,
        path: &UnresolvedPath,
        namespace: Namespace,
        span: Span,
    ) {
        let Some(last) = path.segments.last() else {
            self.bindings
                .by_hir
                .insert(hir_id, ResolvedBinding::NotFound);
            return;
        };
        let Some(mut scope) = self.scopes.enclosing_scope(hir_id) else {
            self.bindings
                .by_hir
                .insert(hir_id, ResolvedBinding::NotFound);
            return;
        };

        loop {
            if let Some(set) = self.symbols.lookup_local(scope, namespace, &last.text) {
                if set.conflicts.is_empty() {
                    self.bindings
                        .by_hir
                        .insert(hir_id, ResolvedBinding::Symbol(set.primary));
                } else {
                    let mut candidates = vec![set.primary];
                    candidates.extend(set.conflicts.iter().copied());
                    self.diagnostics.push(
                        Diagnostic::error(format!("ambiguous reference `{}`", self.path(path)))
                            .with_code(DiagnosticCode::new("SEM0003"))
                            .with_primary_span(span)
                            .with_label(DiagnosticLabel::primary(span, "ambiguous reference")),
                    );
                    self.bindings
                        .by_hir
                        .insert(hir_id, ResolvedBinding::Ambiguous(candidates));
                }
                return;
            }

            let data = self.scopes.get(scope);
            if let Some(parent) = data.parent {
                scope = parent;
            } else {
                break;
            }
        }

        self.diagnostics.push(
            Diagnostic::error(format!("unresolved name `{}`", self.path(path)))
                .with_code(DiagnosticCode::new("SEM0002"))
                .with_primary_span(span)
                .with_label(DiagnosticLabel::primary(span, "name not found in scope")),
        );
        self.bindings
            .by_hir
            .insert(hir_id, ResolvedBinding::NotFound);
    }

    fn path(&self, path: &UnresolvedPath) -> String {
        path.segments
            .iter()
            .map(|segment| segment.text.as_str())
            .collect::<Vec<_>>()
            .join(".")
    }
}

/// Dumps HIR with scope, symbol, and binding data.
pub fn dump_resolved_hir(hir: &Hir, semantic: &SemanticOutput) -> String {
    let mut output = dump_hir(hir);
    let mut dumper = SemaDumper {
        output: String::new(),
        semantic,
    };
    dumper.scopes();
    dumper.symbols();
    dumper.bindings();
    output.push_str(&dumper.output);
    output
}

struct SemaDumper<'a> {
    output: String,
    semantic: &'a SemanticOutput,
}

impl SemaDumper<'_> {
    fn line(&mut self, indent: usize, args: fmt::Arguments<'_>) {
        for _ in 0..indent {
            self.output.push_str("  ");
        }
        self.output.write_fmt(args).expect("write to String");
        self.output.push('\n');
    }

    fn scopes(&mut self) {
        self.line(0, format_args!("Scopes"));
        for scope in &self.semantic.scopes.scopes {
            let parent = scope
                .parent
                .map(|parent| format!("scope{}", parent.raw()))
                .unwrap_or_else(|| "<none>".to_string());
            self.line(
                1,
                format_args!(
                    "scope{} kind={:?} parent={} owner={:?}",
                    scope.id.raw(),
                    scope.kind,
                    parent,
                    scope.owner
                ),
            );
        }
    }

    fn symbols(&mut self) {
        self.line(0, format_args!("Symbols"));
        for symbol in &self.semantic.symbols.symbols {
            self.line(
                1,
                format_args!(
                    "sym{} kind={:?} ns={:?} name={} scope=scope{} decl={:?}",
                    symbol.id.raw(),
                    symbol.kind,
                    symbol.namespace,
                    symbol.name,
                    symbol.declaring_scope.raw(),
                    symbol.declaration
                ),
            );
        }
    }

    fn bindings(&mut self) {
        self.line(0, format_args!("Bindings"));
        for (hir, binding) in &self.semantic.bindings.by_hir {
            self.line(1, format_args!("{hir:?} -> {}", self.binding(binding)));
        }
    }

    fn binding(&self, binding: &ResolvedBinding) -> String {
        match binding {
            ResolvedBinding::Symbol(symbol) => format!("sym{}", symbol.raw()),
            ResolvedBinding::Ambiguous(symbols) => format!(
                "ambiguous({})",
                symbols
                    .iter()
                    .map(|symbol| format!("sym{}", symbol.raw()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            ResolvedBinding::NotFound => "not_found".to_string(),
        }
    }
}
