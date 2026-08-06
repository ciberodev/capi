//! Initial semantic analysis: scopes, symbols, and name resolution.

use std::collections::BTreeMap;
use std::fmt::{self, Write};

use capi_diagnostics::{Diagnostic, DiagnosticCode, DiagnosticLabel};
use capi_hir::{
    dump_hir, Hir, HirBlockId, HirExprId, HirExprKind, HirId, HirImportId, HirItemId, HirItemKind,
    HirLocalId, HirMemberId, HirMemberKind, HirParamId, HirPatternId, HirPatternKind, HirStmtId,
    HirStmtKind, HirTypeRefId, HirTypeRefKind, UnresolvedPath,
};
use capi_lexer::{LiteralKind, Operator};
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
id_type!(TypeId);
id_type!(GenericParamId);
id_type!(CoercionId);

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
    BuiltinType(String),
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

/// Primitive type categories known by the Stage 4 checker.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrimitiveType {
    Bool,
    Char,
    Int,
    UInt,
    Float,
    Double,
    String,
}

/// Internal type category.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeKind {
    Primitive(PrimitiveType),
    Unit,
    Nominal(SymbolId),
    ObjectId(TypeId),
    Function(FunctionSignature),
    GenericParam(GenericParamId),
    GenericInstance { base: SymbolId, args: Vec<TypeId> },
    Tuple(Vec<TypeId>),
    Array(TypeId),
    Unknown(u32),
    Error,
}

/// Type origin used by diagnostics and dumps.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeOrigin {
    Builtin,
    Declared(SymbolId),
    Inferred(HirId),
    Constructed,
    ErrorRecovery,
}

/// Type properties used by the checker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeProperties {
    pub has_identity: bool,
    pub copyable: bool,
    pub polymorphic: bool,
}

/// Interned type data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeInfo {
    pub id: TypeId,
    pub kind: TypeKind,
    pub origin: TypeOrigin,
    pub properties: TypeProperties,
}

/// Built-in type handles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BuiltinTypes {
    pub bool_: TypeId,
    pub char_: TypeId,
    pub int: TypeId,
    pub uint: TypeId,
    pub float: TypeId,
    pub double: TypeId,
    pub string: TypeId,
    pub unit: TypeId,
    pub error: TypeId,
}

/// Type interner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeInterner {
    types: Vec<TypeInfo>,
    index: BTreeMap<TypeKind, TypeId>,
    builtins: BuiltinTypes,
}

impl Default for TypeInterner {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeInterner {
    /// Creates a type interner with built-ins registered in deterministic order.
    pub fn new() -> Self {
        let mut interner = Self {
            types: Vec::new(),
            index: BTreeMap::new(),
            builtins: BuiltinTypes {
                bool_: TypeId::from_raw(0),
                char_: TypeId::from_raw(0),
                int: TypeId::from_raw(0),
                uint: TypeId::from_raw(0),
                float: TypeId::from_raw(0),
                double: TypeId::from_raw(0),
                string: TypeId::from_raw(0),
                unit: TypeId::from_raw(0),
                error: TypeId::from_raw(0),
            },
        };
        let bool_ = interner.intern_with_origin(
            TypeKind::Primitive(PrimitiveType::Bool),
            TypeOrigin::Builtin,
        );
        let char_ = interner.intern_with_origin(
            TypeKind::Primitive(PrimitiveType::Char),
            TypeOrigin::Builtin,
        );
        let int = interner
            .intern_with_origin(TypeKind::Primitive(PrimitiveType::Int), TypeOrigin::Builtin);
        let uint = interner.intern_with_origin(
            TypeKind::Primitive(PrimitiveType::UInt),
            TypeOrigin::Builtin,
        );
        let float = interner.intern_with_origin(
            TypeKind::Primitive(PrimitiveType::Float),
            TypeOrigin::Builtin,
        );
        let double = interner.intern_with_origin(
            TypeKind::Primitive(PrimitiveType::Double),
            TypeOrigin::Builtin,
        );
        let string = interner.intern_with_origin(
            TypeKind::Primitive(PrimitiveType::String),
            TypeOrigin::Builtin,
        );
        let unit = interner.intern_with_origin(TypeKind::Unit, TypeOrigin::Builtin);
        let error = interner.intern_with_origin(TypeKind::Error, TypeOrigin::ErrorRecovery);
        interner.builtins = BuiltinTypes {
            bool_,
            char_,
            int,
            uint,
            float,
            double,
            string,
            unit,
            error,
        };
        interner
    }

    /// Interns a type.
    pub fn intern(&mut self, kind: TypeKind) -> TypeId {
        self.intern_with_origin(kind, TypeOrigin::Constructed)
    }

    /// Interns a type with origin for first creation.
    pub fn intern_with_origin(&mut self, kind: TypeKind, origin: TypeOrigin) -> TypeId {
        if let Some(id) = self.index.get(&kind) {
            return *id;
        }
        let id = TypeId::from_raw(self.types.len() as u32);
        let properties = type_properties(&kind);
        self.types.push(TypeInfo {
            id,
            kind: kind.clone(),
            origin,
            properties,
        });
        self.index.insert(kind, id);
        id
    }

    /// Returns type data.
    pub fn get(&self, id: TypeId) -> &TypeInfo {
        &self.types[id.raw() as usize]
    }

    /// Returns all interned types.
    pub fn types(&self) -> &[TypeInfo] {
        &self.types
    }

    /// Returns built-in handles.
    pub const fn builtins(&self) -> BuiltinTypes {
        self.builtins
    }
}

fn type_properties(kind: &TypeKind) -> TypeProperties {
    match kind {
        TypeKind::Nominal(_) => TypeProperties {
            has_identity: true,
            copyable: false,
            polymorphic: true,
        },
        TypeKind::ObjectId(_) => TypeProperties {
            has_identity: false,
            copyable: true,
            polymorphic: true,
        },
        TypeKind::Error | TypeKind::Unknown(_) => TypeProperties {
            has_identity: false,
            copyable: false,
            polymorphic: false,
        },
        _ => TypeProperties {
            has_identity: false,
            copyable: true,
            polymorphic: false,
        },
    }
}

/// Function or method signature.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionSignature {
    pub params: Vec<TypeId>,
    pub return_type: TypeId,
    pub receiver: Option<TypeId>,
    pub generic_params: Vec<GenericParamId>,
}

/// Generic parameter data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenericParam {
    pub id: GenericParamId,
    pub owner: HirId,
    pub name: String,
    pub span: Span,
}

/// Type table produced by Stage 4.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TypeTable {
    pub symbol_types: BTreeMap<SymbolId, TypeId>,
    pub type_refs: BTreeMap<HirTypeRefId, TypeId>,
    pub items: BTreeMap<HirItemId, TypeId>,
    pub members: BTreeMap<HirMemberId, TypeId>,
    pub params: BTreeMap<HirParamId, TypeId>,
    pub locals: BTreeMap<HirLocalId, TypeId>,
    pub exprs: BTreeMap<HirExprId, TypeId>,
    pub stmts: BTreeMap<HirStmtId, TypeId>,
    pub patterns: BTreeMap<HirPatternId, TypeId>,
    pub signatures: BTreeMap<SymbolId, FunctionSignature>,
    pub generic_params: Vec<GenericParam>,
}

/// Coercion category.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoercionKind {
    Identity,
    Upcast(Vec<TypeId>),
    Error,
}

/// Applied coercion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coercion {
    pub id: CoercionId,
    pub kind: CoercionKind,
    pub source: TypeId,
    pub target: TypeId,
    pub origin: HirId,
}

/// Coercions applied by type checking.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CoercionTable {
    pub by_hir: BTreeMap<HirId, Coercion>,
}

/// Resolved call data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallResolution {
    pub call: HirExprId,
    pub target: SymbolId,
    pub signature: FunctionSignature,
    pub applied_coercions: Vec<CoercionId>,
}

/// Call resolution table.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CallResolutionTable {
    pub calls: BTreeMap<HirExprId, CallResolution>,
}

/// Type checking state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeCheckState {
    Checked,
    CheckedWithErrors,
    Blocked,
}

/// Type checking output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeCheckOutput {
    semantic: SemanticOutput,
    interner: TypeInterner,
    types: TypeTable,
    coercions: CoercionTable,
    calls: CallResolutionTable,
    diagnostics: Vec<Diagnostic>,
    state: TypeCheckState,
}

impl TypeCheckOutput {
    /// Returns Stage 3 semantic output.
    pub const fn semantic(&self) -> &SemanticOutput {
        &self.semantic
    }

    /// Returns type interner.
    pub const fn interner(&self) -> &TypeInterner {
        &self.interner
    }

    /// Returns type tables.
    pub const fn types(&self) -> &TypeTable {
        &self.types
    }

    /// Returns applied coercions.
    pub const fn coercions(&self) -> &CoercionTable {
        &self.coercions
    }

    /// Returns resolved calls.
    pub const fn calls(&self) -> &CallResolutionTable {
        &self.calls
    }

    /// Returns diagnostics from name analysis and type checking.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Returns final type checking state.
    pub const fn state(&self) -> TypeCheckState {
        self.state
    }
}

/// Runs Stage 3 and Stage 4 semantic analysis.
pub fn check_types(hir: &Hir) -> TypeCheckOutput {
    let semantic = analyze_names(hir);
    let mut checker = TypeChecker::new(hir, semantic);
    checker.check()
}

struct TypeChecker<'a> {
    hir: &'a Hir,
    semantic: SemanticOutput,
    interner: TypeInterner,
    types: TypeTable,
    coercions: CoercionTable,
    calls: CallResolutionTable,
    diagnostics: Vec<Diagnostic>,
    next_coercion: u32,
    generic_params: BTreeMap<(HirId, String), GenericParamId>,
}

impl<'a> TypeChecker<'a> {
    fn new(hir: &'a Hir, semantic: SemanticOutput) -> Self {
        Self {
            hir,
            semantic,
            interner: TypeInterner::new(),
            types: TypeTable::default(),
            coercions: CoercionTable::default(),
            calls: CallResolutionTable::default(),
            diagnostics: Vec::new(),
            next_coercion: 0,
            generic_params: BTreeMap::new(),
        }
    }

    fn check(&mut self) -> TypeCheckOutput {
        self.collect_generic_params();
        self.collect_nominal_types();
        self.collect_signatures();
        self.check_items();

        let mut diagnostics = self.semantic.diagnostics.clone();
        diagnostics.extend(self.diagnostics.clone());
        let state = if self.semantic.diagnostics.iter().any(|diagnostic| {
            diagnostic
                .code()
                .is_some_and(|code| code.as_str() == "SEM0001" || code.as_str() == "SEM0003")
        }) {
            TypeCheckState::Blocked
        } else if diagnostics.is_empty() {
            TypeCheckState::Checked
        } else {
            TypeCheckState::CheckedWithErrors
        };

        TypeCheckOutput {
            semantic: self.semantic.clone(),
            interner: self.interner.clone(),
            types: self.types.clone(),
            coercions: self.coercions.clone(),
            calls: self.calls.clone(),
            diagnostics,
            state,
        }
    }

    fn builtins(&self) -> BuiltinTypes {
        self.interner.builtins()
    }

    fn collect_generic_params(&mut self) {
        for item in &self.hir.items {
            match &item.kind {
                HirItemKind::Function(function) => {
                    self.register_generic_params(HirId::Item(item.id), &function.generics);
                }
                HirItemKind::Class(ty) | HirItemKind::Interface(ty) | HirItemKind::Trait(ty) => {
                    self.register_generic_params(HirId::Item(item.id), &ty.generics);
                }
                _ => {}
            }
        }
        for member in &self.hir.members {
            match &member.kind {
                HirMemberKind::Method(function) => {
                    self.register_generic_params(HirId::Member(member.id), &function.generics);
                }
                HirMemberKind::Constructor(constructor) => {
                    let _ = constructor;
                }
                _ => {}
            }
        }
    }

    fn register_generic_params(&mut self, owner: HirId, generics: &[capi_hir::HirName]) {
        let mut names = BTreeMap::<&str, Span>::new();
        for generic in generics {
            if let Some(previous) = names.insert(generic.text.as_str(), generic.span) {
                self.push_type_diagnostic(
                    "TYPE0008",
                    format!("duplicate generic parameter `{}`", generic.text),
                    generic.span,
                    "duplicate generic parameter",
                )
                .with_label(DiagnosticLabel::secondary(
                    previous,
                    "previous generic parameter is here",
                ));
            }
            let id = GenericParamId::from_raw(self.types.generic_params.len() as u32);
            self.types.generic_params.push(GenericParam {
                id,
                owner,
                name: generic.text.clone(),
                span: generic.span,
            });
            self.generic_params
                .insert((owner, generic.text.clone()), id);
        }
    }

    fn collect_nominal_types(&mut self) {
        let nominal_symbols = self
            .semantic
            .symbols
            .symbols
            .iter()
            .filter(|symbol| {
                matches!(
                    symbol.kind,
                    SymbolKind::Class | SymbolKind::Interface | SymbolKind::Trait
                )
            })
            .map(|symbol| (symbol.id, symbol.declaration))
            .collect::<Vec<_>>();

        for (symbol, declaration) in nominal_symbols {
            let ty = self
                .interner
                .intern_with_origin(TypeKind::Nominal(symbol), TypeOrigin::Declared(symbol));
            self.types.symbol_types.insert(symbol, ty);
            if let DeclarationSite::Item(item) = declaration {
                self.types.items.insert(item, ty);
            }
        }
    }

    fn collect_signatures(&mut self) {
        let symbols = self.semantic.symbols.symbols.clone();
        for symbol in symbols {
            match symbol.declaration {
                DeclarationSite::Item(item) if symbol.kind == SymbolKind::Function => {
                    let HirItemKind::Function(function) = &self.hir.items[item.raw() as usize].kind
                    else {
                        continue;
                    };
                    let signature = self.function_signature(function, None);
                    let ty = self.interner.intern(TypeKind::Function(signature.clone()));
                    self.types.signatures.insert(symbol.id, signature);
                    self.types.symbol_types.insert(symbol.id, ty);
                    self.types.items.insert(item, ty);
                }
                DeclarationSite::Member(member)
                    if matches!(symbol.kind, SymbolKind::Method | SymbolKind::Constructor) =>
                {
                    let receiver = self.receiver_for_member(member);
                    let signature = match &self.hir.members[member.raw() as usize].kind {
                        HirMemberKind::Method(function) => {
                            self.function_signature(function, receiver)
                        }
                        HirMemberKind::Constructor(constructor) => {
                            self.constructor_signature(constructor, receiver)
                        }
                        _ => continue,
                    };
                    let ty = self.interner.intern(TypeKind::Function(signature.clone()));
                    self.types.signatures.insert(symbol.id, signature);
                    self.types.symbol_types.insert(symbol.id, ty);
                    self.types.members.insert(member, ty);
                }
                DeclarationSite::Param(raw) if symbol.kind == SymbolKind::Param => {
                    let id = HirParamId::from_raw(raw);
                    let ty = self.param_type(id);
                    self.types.symbol_types.insert(symbol.id, ty);
                    self.types.params.insert(id, ty);
                }
                DeclarationSite::Local(_) if symbol.kind == SymbolKind::Local => {}
                DeclarationSite::Member(member) if symbol.kind == SymbolKind::Field => {
                    let ty = self.field_type(member);
                    self.types.symbol_types.insert(symbol.id, ty);
                    self.types.members.insert(member, ty);
                }
                _ => {}
            }
        }
    }

    fn receiver_for_member(&self, member: HirMemberId) -> Option<TypeId> {
        let member_scope = self
            .semantic
            .scopes
            .enclosing_scope(HirId::Member(member))?;
        let owner = self
            .semantic
            .scopes
            .get(member_scope)
            .parent
            .and_then(|type_scope| self.semantic.scopes.get(type_scope).parent)
            .and_then(|scope| {
                self.semantic
                    .symbols
                    .symbols
                    .iter()
                    .find(|symbol| {
                        symbol.declaring_scope == scope
                            && matches!(
                                symbol.kind,
                                SymbolKind::Class | SymbolKind::Interface | SymbolKind::Trait
                            )
                    })
                    .map(|symbol| symbol.id)
            })?;
        self.types.symbol_types.get(&owner).copied()
    }

    fn function_signature(
        &mut self,
        function: &capi_hir::HirFunction,
        receiver: Option<TypeId>,
    ) -> FunctionSignature {
        let params = function
            .params
            .iter()
            .map(|param| self.param_type(*param))
            .collect::<Vec<_>>();
        let return_type = function
            .return_type
            .map(|ty| self.type_ref(ty))
            .unwrap_or_else(|| self.builtins().unit);
        FunctionSignature {
            params,
            return_type,
            receiver,
            generic_params: Vec::new(),
        }
    }

    fn constructor_signature(
        &mut self,
        constructor: &capi_hir::HirConstructor,
        receiver: Option<TypeId>,
    ) -> FunctionSignature {
        let params = constructor
            .params
            .iter()
            .map(|param| self.param_type(*param))
            .collect::<Vec<_>>();
        FunctionSignature {
            params,
            return_type: receiver.unwrap_or_else(|| self.builtins().unit),
            receiver,
            generic_params: Vec::new(),
        }
    }

    fn param_type(&mut self, id: HirParamId) -> TypeId {
        if let Some(ty) = self.types.params.get(&id) {
            return *ty;
        }
        let param = &self.hir.params[id.raw() as usize];
        let ty = if let Some(type_ref) = param.ty {
            self.type_ref(type_ref)
        } else {
            self.push_type_diagnostic(
                "TYPE0009",
                format!("parameter `{}` requires an explicit type", param.name.text),
                param.name.span,
                "missing parameter type",
            );
            self.builtins().error
        };
        self.types.params.insert(id, ty);
        ty
    }

    fn field_type(&mut self, member: HirMemberId) -> TypeId {
        if let Some(ty) = self.types.members.get(&member) {
            return *ty;
        }
        let ty = match &self.hir.members[member.raw() as usize].kind {
            HirMemberKind::Field(field) => {
                field.ty.map(|ty| self.type_ref(ty)).unwrap_or_else(|| {
                    field
                        .initializer
                        .map(|expr| self.expr_type(expr, None))
                        .unwrap_or_else(|| {
                            self.push_type_diagnostic(
                                "TYPE0001",
                                format!(
                                    "field `{}` requires a type or initializer",
                                    field.name.text
                                ),
                                field.name.span,
                                "cannot infer field type",
                            );
                            self.builtins().error
                        })
                })
            }
            _ => self.builtins().error,
        };
        self.types.members.insert(member, ty);
        ty
    }

    fn check_items(&mut self) {
        for item in &self.hir.items {
            match &item.kind {
                HirItemKind::Function(function) => {
                    let expected = function
                        .return_type
                        .map(|ty| self.type_ref(ty))
                        .unwrap_or_else(|| self.builtins().unit);
                    if let Some(body) = function.body {
                        self.check_block(body, expected);
                    }
                }
                HirItemKind::Class(ty) | HirItemKind::Interface(ty) | HirItemKind::Trait(ty) => {
                    self.check_type_item(ty);
                }
                HirItemKind::Const(konst) => self.check_const(konst),
                HirItemKind::Let(local) => {
                    self.local_type(*local);
                }
                _ => {}
            }
        }
    }

    fn check_type_item(&mut self, ty: &capi_hir::HirTypeItem) {
        if let Some(extends) = ty.extends {
            self.type_ref(extends);
        }
        for implemented in &ty.implements {
            self.type_ref(*implemented);
        }
        for used in &ty.uses {
            self.type_ref(*used);
        }
        for member in &ty.members {
            match &self.hir.members[member.raw() as usize].kind {
                HirMemberKind::Field(field) => {
                    let expected = self.field_type(*member);
                    if let Some(initializer) = field.initializer {
                        let actual = self.expr_type(initializer, Some(expected));
                        self.require_compatible(
                            expected,
                            actual,
                            HirId::Expr(initializer),
                            self.hir.exprs[initializer.raw() as usize].origin.span,
                        );
                    }
                }
                HirMemberKind::Method(function) => {
                    let expected = function
                        .return_type
                        .map(|ty| self.type_ref(ty))
                        .unwrap_or_else(|| self.builtins().unit);
                    if let Some(body) = function.body {
                        self.check_block(body, expected);
                    }
                }
                HirMemberKind::Constructor(constructor) => {
                    if let Some(body) = constructor.body {
                        self.check_block(body, self.builtins().unit);
                    }
                }
                HirMemberKind::Const(konst) => self.check_const(konst),
                HirMemberKind::Let(local) => {
                    self.local_type(*local);
                }
                _ => {}
            }
        }
    }

    fn check_const(&mut self, konst: &capi_hir::HirConst) {
        let expected = konst.ty.map(|ty| self.type_ref(ty));
        if let Some(initializer) = konst.initializer {
            let actual = self.expr_type(initializer, expected);
            if let Some(expected) = expected {
                self.require_compatible(
                    expected,
                    actual,
                    HirId::Expr(initializer),
                    self.hir.exprs[initializer.raw() as usize].origin.span,
                );
            }
        }
    }

    fn check_block(&mut self, id: HirBlockId, return_type: TypeId) {
        let block = &self.hir.blocks[id.raw() as usize];
        for stmt in &block.stmts {
            self.check_stmt(*stmt, return_type);
        }
    }

    fn check_stmt(&mut self, id: HirStmtId, return_type: TypeId) {
        let stmt = &self.hir.stmts[id.raw() as usize];
        let unit = self.builtins().unit;
        self.types.stmts.insert(id, unit);
        match &stmt.kind {
            HirStmtKind::Local(local) => {
                self.local_type(*local);
            }
            HirStmtKind::Expr(expr) => {
                self.expr_type(*expr, None);
            }
            HirStmtKind::Return(value) => {
                let actual = value
                    .map(|expr| self.expr_type(expr, Some(return_type)))
                    .unwrap_or(unit);
                self.require_compatible(return_type, actual, HirId::Stmt(id), stmt.origin.span);
            }
            HirStmtKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let bool_ = self.builtins().bool_;
                let condition_ty = self.expr_type(*condition, Some(bool_));
                self.require_compatible(
                    bool_,
                    condition_ty,
                    HirId::Expr(*condition),
                    self.hir.exprs[condition.raw() as usize].origin.span,
                );
                self.check_block(*then_branch, return_type);
                if let Some(else_branch) = else_branch {
                    self.check_block(*else_branch, return_type);
                }
            }
            HirStmtKind::While { condition, body } => {
                let bool_ = self.builtins().bool_;
                let condition_ty = self.expr_type(*condition, Some(bool_));
                self.require_compatible(
                    bool_,
                    condition_ty,
                    HirId::Expr(*condition),
                    self.hir.exprs[condition.raw() as usize].origin.span,
                );
                self.check_block(*body, return_type);
            }
            HirStmtKind::Block(block) => self.check_block(*block, return_type),
            HirStmtKind::Match { expression, arms } => {
                let expected = self.expr_type(*expression, None);
                for arm in arms {
                    self.pattern_type(arm.pattern, expected);
                    for stmt in &arm.stmts {
                        self.check_stmt(*stmt, return_type);
                    }
                }
            }
            _ => {}
        }
    }

    fn type_ref(&mut self, id: HirTypeRefId) -> TypeId {
        if let Some(ty) = self.types.type_refs.get(&id) {
            return *ty;
        }
        let type_ref = &self.hir.type_refs[id.raw() as usize];
        let ty = match &type_ref.kind {
            HirTypeRefKind::Path(path) => {
                self.path_type(HirId::TypeRef(id), path, type_ref.origin.span)
            }
            HirTypeRefKind::Generic { base, arguments } => {
                let base_ty = self.path_type_with_arity_check(
                    HirId::TypeRef(id),
                    base,
                    type_ref.origin.span,
                    false,
                );
                let args = arguments
                    .iter()
                    .map(|argument| self.type_ref(*argument))
                    .collect::<Vec<_>>();
                match self.symbol_for_hir(HirId::TypeRef(id)) {
                    Some(base_symbol) => {
                        let expected = self.generic_arity(base_symbol);
                        if expected != args.len() {
                            self.push_type_diagnostic(
                                "TYPE0008",
                                format!(
                                    "generic type `{}` expects {expected} argument(s), got {}",
                                    base.segments
                                        .last()
                                        .map(|segment| segment.text.as_str())
                                        .unwrap_or("<unknown>"),
                                    args.len()
                                ),
                                type_ref.origin.span,
                                "invalid generic arity",
                            );
                            self.builtins().error
                        } else if self.is_error(base_ty)
                            || args.iter().any(|arg| self.is_error(*arg))
                        {
                            self.builtins().error
                        } else {
                            self.interner.intern(TypeKind::GenericInstance {
                                base: base_symbol,
                                args,
                            })
                        }
                    }
                    None => self.builtins().error,
                }
            }
            HirTypeRefKind::Array { element, .. } => {
                let element = self.type_ref(*element);
                if self.is_error(element) {
                    self.builtins().error
                } else {
                    self.interner.intern(TypeKind::Array(element))
                }
            }
            HirTypeRefKind::Tuple(elements) => {
                let elements = elements
                    .iter()
                    .map(|element| self.type_ref(*element))
                    .collect::<Vec<_>>();
                if elements.iter().any(|element| self.is_error(*element)) {
                    self.builtins().error
                } else {
                    self.interner.intern(TypeKind::Tuple(elements))
                }
            }
            HirTypeRefKind::Error(_) => self.builtins().error,
        };
        self.types.type_refs.insert(id, ty);
        ty
    }

    fn path_type(&mut self, hir_id: HirId, path: &UnresolvedPath, span: Span) -> TypeId {
        self.path_type_with_arity_check(hir_id, path, span, true)
    }

    fn path_type_with_arity_check(
        &mut self,
        hir_id: HirId,
        path: &UnresolvedPath,
        span: Span,
        reject_unapplied_generic: bool,
    ) -> TypeId {
        if path.segments.len() == 1 {
            if let Some(ty) = self.builtin_type(&path.segments[0].text) {
                return ty;
            }
            if let Some(param) = self.generic_param_in_scope(hir_id, &path.segments[0].text) {
                return self.interner.intern(TypeKind::GenericParam(param));
            }
        }

        let binding = self.semantic.bindings.by_hir.get(&hir_id).cloned();
        match binding {
            Some(ResolvedBinding::Symbol(symbol)) => {
                let ty = self
                    .types
                    .symbol_types
                    .get(&symbol)
                    .copied()
                    .unwrap_or_else(|| {
                        self.push_type_diagnostic(
                            "TYPE0002",
                            "symbol is not a type",
                            span,
                            "expected a type",
                        );
                        self.builtins().error
                    });
                let arity = self.generic_arity(symbol);
                if reject_unapplied_generic && arity > 0 {
                    self.push_type_diagnostic(
                        "TYPE0008",
                        format!(
                            "generic type `{}` expects {arity} argument(s), got 0",
                            path.segments
                                .last()
                                .map(|segment| segment.text.as_str())
                                .unwrap_or("<unknown>")
                        ),
                        span,
                        "invalid generic arity",
                    );
                    self.builtins().error
                } else {
                    ty
                }
            }
            Some(ResolvedBinding::BuiltinType(name)) => {
                self.builtin_type(&name).unwrap_or(self.builtins().error)
            }
            Some(ResolvedBinding::Ambiguous(_)) => self.builtins().error,
            Some(ResolvedBinding::NotFound) | None => {
                if path.segments.len() == 1 {
                    self.push_type_diagnostic(
                        "TYPE0002",
                        format!("unknown type `{}`", path.segments[0].text),
                        span,
                        "type not found",
                    );
                }
                self.builtins().error
            }
        }
    }

    fn local_type(&mut self, id: HirLocalId) -> TypeId {
        if let Some(ty) = self.types.locals.get(&id) {
            return *ty;
        }
        let local = &self.hir.locals[id.raw() as usize];
        let explicit = local.ty.map(|ty| self.type_ref(ty));
        let inferred = local.initializer.map(|expr| self.expr_type(expr, None));
        let ty = match (explicit, inferred) {
            (Some(expected), Some(actual)) => {
                self.require_compatible(expected, actual, HirId::Local(id), local.origin.span);
                expected
            }
            (Some(expected), None) => expected,
            (None, Some(actual)) => actual,
            (None, None) => {
                self.push_type_diagnostic(
                    "TYPE0001",
                    format!("cannot infer type of local `{}`", local.name.text),
                    local.name.span,
                    "type annotation or initializer required",
                );
                self.builtins().error
            }
        };
        self.types.locals.insert(id, ty);
        if let Some(symbol) = self.symbol_for_declaration(DeclarationSite::Local(id)) {
            self.types.symbol_types.insert(symbol, ty);
        }
        ty
    }

    fn expr_type(&mut self, id: HirExprId, expected: Option<TypeId>) -> TypeId {
        if let Some(ty) = self.types.exprs.get(&id) {
            return *ty;
        }
        let expr = &self.hir.exprs[id.raw() as usize];
        let ty = match &expr.kind {
            HirExprKind::Literal { kind, .. } => self.literal_type(*kind, expected),
            HirExprKind::Path(_) => self.path_expr_type(id),
            HirExprKind::This => self.builtins().error,
            HirExprKind::New { ty, arguments } => {
                let nominal = self.type_ref(*ty);
                for argument in arguments {
                    self.expr_type(*argument, None);
                }
                if self.is_error(nominal) {
                    self.builtins().error
                } else {
                    self.interner.intern(TypeKind::ObjectId(nominal))
                }
            }
            HirExprKind::Call { callee, arguments } => self.call_type(id, *callee, arguments),
            HirExprKind::Member { base, member } => self.member_type(id, *base, member),
            HirExprKind::Index { base, index } => {
                self.expr_type(*index, Some(self.builtins().int));
                let base_ty = self.expr_type(*base, None);
                match self.interner.get(base_ty).kind.clone() {
                    TypeKind::Array(element) => element,
                    TypeKind::Error => self.builtins().error,
                    _ => {
                        self.push_type_diagnostic(
                            "TYPE0002",
                            "indexed expression is not an array",
                            expr.origin.span,
                            "not indexable",
                        );
                        self.builtins().error
                    }
                }
            }
            HirExprKind::Unary { op, expr: inner } => {
                self.unary_type(*op, *inner, expr.origin.span)
            }
            HirExprKind::Binary { left, op, right } => {
                self.binary_type(*left, *op, *right, expr.origin.span)
            }
            HirExprKind::Assign { target, value } => {
                let expected = self.expr_type(*target, None);
                let actual = self.expr_type(*value, Some(expected));
                self.require_compatible(
                    expected,
                    actual,
                    HirId::Expr(*value),
                    self.hir.exprs[value.raw() as usize].origin.span,
                );
                self.builtins().unit
            }
            HirExprKind::Tuple(elements) => {
                let elements = elements
                    .iter()
                    .map(|element| self.expr_type(*element, None))
                    .collect::<Vec<_>>();
                if elements.iter().any(|element| self.is_error(*element)) {
                    self.builtins().error
                } else {
                    self.interner.intern(TypeKind::Tuple(elements))
                }
            }
            HirExprKind::Array(elements) => {
                let first = elements
                    .first()
                    .map(|element| self.expr_type(*element, None));
                if let Some(expected) = first {
                    for element in elements.iter().skip(1) {
                        let actual = self.expr_type(*element, Some(expected));
                        self.require_compatible(
                            expected,
                            actual,
                            HirId::Expr(*element),
                            self.hir.exprs[element.raw() as usize].origin.span,
                        );
                    }
                    self.interner.intern(TypeKind::Array(expected))
                } else {
                    self.push_type_diagnostic(
                        "TYPE0001",
                        "cannot infer type of empty array",
                        expr.origin.span,
                        "empty array needs type context",
                    );
                    self.builtins().error
                }
            }
            HirExprKind::Error(_) => self.builtins().error,
        };
        if let Some(expected) = expected {
            self.require_compatible(expected, ty, HirId::Expr(id), expr.origin.span);
        }
        self.types.exprs.insert(id, ty);
        ty
    }

    fn literal_type(&mut self, kind: LiteralKind, expected: Option<TypeId>) -> TypeId {
        let builtins = self.builtins();
        match kind {
            LiteralKind::Integer => expected
                .filter(|ty| {
                    matches!(
                        self.interner.get(*ty).kind,
                        TypeKind::Primitive(PrimitiveType::Int | PrimitiveType::UInt)
                    )
                })
                .unwrap_or(builtins.int),
            LiteralKind::Float => expected
                .filter(|ty| {
                    matches!(
                        self.interner.get(*ty).kind,
                        TypeKind::Primitive(PrimitiveType::Float | PrimitiveType::Double)
                    )
                })
                .unwrap_or(builtins.double),
            LiteralKind::Char => builtins.char_,
            LiteralKind::String => builtins.string,
            LiteralKind::Bool => builtins.bool_,
        }
    }

    fn path_expr_type(&mut self, id: HirExprId) -> TypeId {
        let binding = self.semantic.bindings.by_hir.get(&HirId::Expr(id)).cloned();
        match binding {
            Some(ResolvedBinding::Symbol(symbol)) => self
                .types
                .symbol_types
                .get(&symbol)
                .copied()
                .unwrap_or_else(|| {
                    let entry = self.semantic.symbols.symbol(symbol);
                    match entry.declaration {
                        DeclarationSite::Local(local) => self.local_type(local),
                        DeclarationSite::Param(raw) => self.param_type(HirParamId::from_raw(raw)),
                        _ => self.builtins().error,
                    }
                }),
            Some(ResolvedBinding::Ambiguous(_)) | Some(ResolvedBinding::NotFound) | None => {
                self.builtins().error
            }
            Some(ResolvedBinding::BuiltinType(_)) => self.builtins().error,
        }
    }

    fn call_type(&mut self, id: HirExprId, callee: HirExprId, arguments: &[HirExprId]) -> TypeId {
        let Some(target) = self.call_target(callee) else {
            self.push_type_diagnostic(
                "TYPE0005",
                "callee is not callable",
                self.hir.exprs[callee.raw() as usize].origin.span,
                "not callable",
            );
            return self.builtins().error;
        };
        let Some(signature) = self.types.signatures.get(&target).cloned() else {
            self.push_type_diagnostic(
                "TYPE0005",
                "callee has no callable signature",
                self.hir.exprs[callee.raw() as usize].origin.span,
                "not callable",
            );
            return self.builtins().error;
        };
        if signature.params.len() != arguments.len() {
            self.push_type_diagnostic(
                "TYPE0005",
                format!(
                    "call expects {} argument(s), got {}",
                    signature.params.len(),
                    arguments.len()
                ),
                self.hir.exprs[id.raw() as usize].origin.span,
                "invalid argument count",
            );
            return self.builtins().error;
        }
        let mut applied_coercions = Vec::new();
        for (argument, expected) in arguments.iter().zip(signature.params.iter()) {
            let actual = self.expr_type(*argument, Some(*expected));
            if let Some(coercion) = self.compatibility_coercion(
                *expected,
                actual,
                HirId::Expr(*argument),
                self.hir.exprs[argument.raw() as usize].origin.span,
            ) {
                applied_coercions.push(coercion.id);
            }
        }
        self.calls.calls.insert(
            id,
            CallResolution {
                call: id,
                target,
                signature: signature.clone(),
                applied_coercions,
            },
        );
        signature.return_type
    }

    fn call_target(&mut self, callee: HirExprId) -> Option<SymbolId> {
        match &self.hir.exprs[callee.raw() as usize].kind {
            HirExprKind::Path(_) => match self.semantic.bindings.by_hir.get(&HirId::Expr(callee)) {
                Some(ResolvedBinding::Symbol(symbol)) => Some(*symbol),
                _ => None,
            },
            HirExprKind::Member { base, member } => {
                let base_ty = self.expr_type(*base, None);
                self.lookup_member(base_ty, &member.text, SymbolKind::Method)
            }
            _ => None,
        }
    }

    fn member_type(
        &mut self,
        _id: HirExprId,
        base: HirExprId,
        member: &capi_hir::HirName,
    ) -> TypeId {
        let base_ty = self.expr_type(base, None);
        if let Some(symbol) = self.lookup_member(base_ty, &member.text, SymbolKind::Field) {
            self.types
                .symbol_types
                .get(&symbol)
                .copied()
                .unwrap_or_else(|| self.builtins().error)
        } else if let Some(symbol) = self.lookup_member(base_ty, &member.text, SymbolKind::Method) {
            self.types
                .symbol_types
                .get(&symbol)
                .copied()
                .unwrap_or_else(|| self.builtins().error)
        } else {
            self.push_type_diagnostic(
                "TYPE0005",
                format!("type has no member `{}`", member.text),
                member.span,
                "member not found",
            );
            self.builtins().error
        }
    }

    fn unary_type(&mut self, op: Operator, inner: HirExprId, span: Span) -> TypeId {
        let ty = self.expr_type(inner, None);
        match op {
            Operator::Bang => {
                self.require_compatible(self.builtins().bool_, ty, HirId::Expr(inner), span);
                self.builtins().bool_
            }
            Operator::Minus => {
                if self.is_numeric(ty) {
                    ty
                } else {
                    self.push_type_diagnostic(
                        "TYPE0006",
                        "unary `-` requires a numeric operand",
                        span,
                        "not numeric",
                    );
                    self.builtins().error
                }
            }
            _ => self.builtins().error,
        }
    }

    fn binary_type(
        &mut self,
        left: HirExprId,
        op: Operator,
        right: HirExprId,
        span: Span,
    ) -> TypeId {
        let left_ty = self.expr_type(left, None);
        let right_ty = self.expr_type(right, Some(left_ty));
        match op {
            Operator::Plus
            | Operator::Minus
            | Operator::Star
            | Operator::Slash
            | Operator::Percent => {
                if self.is_numeric(left_ty)
                    && self.require_compatible(left_ty, right_ty, HirId::Expr(right), span)
                {
                    left_ty
                } else {
                    self.push_type_diagnostic(
                        "TYPE0006",
                        "binary arithmetic requires matching numeric operands",
                        span,
                        "invalid operands",
                    );
                    self.builtins().error
                }
            }
            Operator::EqualEqual
            | Operator::BangEqual
            | Operator::Less
            | Operator::LessEqual
            | Operator::Greater
            | Operator::GreaterEqual => {
                self.require_compatible(left_ty, right_ty, HirId::Expr(right), span);
                self.builtins().bool_
            }
            Operator::AmpAmp | Operator::PipePipe => {
                let bool_ = self.builtins().bool_;
                self.require_compatible(bool_, left_ty, HirId::Expr(left), span);
                self.require_compatible(bool_, right_ty, HirId::Expr(right), span);
                bool_
            }
            _ => self.builtins().error,
        }
    }

    fn pattern_type(&mut self, id: HirPatternId, expected: TypeId) {
        self.types.patterns.insert(id, expected);
        let pattern = &self.hir.patterns[id.raw() as usize];
        if let HirPatternKind::Constructor { fields, .. } = &pattern.kind {
            for field in fields {
                self.pattern_type(*field, expected);
            }
        }
    }

    fn require_compatible(
        &mut self,
        expected: TypeId,
        actual: TypeId,
        origin: HirId,
        span: Span,
    ) -> bool {
        if expected == actual || self.is_error(expected) || self.is_error(actual) {
            return true;
        }
        if let Some(coercion) = self.compatibility_coercion(expected, actual, origin, span) {
            self.coercions.by_hir.insert(origin, coercion);
            true
        } else {
            self.push_type_diagnostic(
                "TYPE0003",
                format!(
                    "type mismatch: expected {}, found {}",
                    self.type_display(expected),
                    self.type_display(actual)
                ),
                span,
                "incompatible type",
            );
            false
        }
    }

    fn compatibility_coercion(
        &mut self,
        expected: TypeId,
        actual: TypeId,
        origin: HirId,
        _span: Span,
    ) -> Option<Coercion> {
        if expected == actual || self.is_error(expected) || self.is_error(actual) {
            return None;
        }
        self.subtype_path(actual, expected).map(|path| {
            let id = CoercionId::from_raw(self.next_coercion);
            self.next_coercion += 1;
            Coercion {
                id,
                kind: CoercionKind::Upcast(path),
                source: actual,
                target: expected,
                origin,
            }
        })
    }

    fn subtype_path(&self, actual: TypeId, expected: TypeId) -> Option<Vec<TypeId>> {
        if actual == expected {
            return Some(vec![actual]);
        }
        match (
            self.interner.get(actual).kind.clone(),
            self.interner.get(expected).kind.clone(),
        ) {
            (TypeKind::ObjectId(actual_inner), TypeKind::ObjectId(expected_inner)) => self
                .subtype_path(actual_inner, expected_inner)
                .map(|_| vec![actual, expected]),
            (TypeKind::Nominal(actual_symbol), TypeKind::Nominal(expected_symbol)) => self
                .nominal_subtype_path(actual_symbol, expected_symbol)
                .map(|symbols| {
                    symbols
                        .into_iter()
                        .filter_map(|symbol| self.types.symbol_types.get(&symbol).copied())
                        .collect::<Vec<_>>()
                }),
            _ => None,
        }
    }

    fn nominal_subtype_path(&self, actual: SymbolId, expected: SymbolId) -> Option<Vec<SymbolId>> {
        if actual == expected {
            return Some(vec![actual]);
        }
        let mut current = actual;
        let mut path = vec![actual];
        for _ in 0..self.semantic.symbols.symbols.len() {
            let next = self.extends_symbol(current);
            if let Some(next) = next {
                path.push(next);
                if next == expected {
                    return Some(path);
                }
                current = next;
            } else {
                break;
            }
        }
        if self.implements_symbol(actual, expected) {
            return Some(vec![actual, expected]);
        }
        None
    }

    fn extends_symbol(&self, symbol: SymbolId) -> Option<SymbolId> {
        let DeclarationSite::Item(item) = self.semantic.symbols.symbol(symbol).declaration else {
            return None;
        };
        let HirItemKind::Class(ty) = &self.hir.items[item.raw() as usize].kind else {
            return None;
        };
        ty.extends
            .and_then(|extends| self.symbol_for_hir(HirId::TypeRef(extends)))
    }

    fn implements_symbol(&self, actual: SymbolId, expected: SymbolId) -> bool {
        let DeclarationSite::Item(item) = self.semantic.symbols.symbol(actual).declaration else {
            return false;
        };
        let HirItemKind::Class(ty) = &self.hir.items[item.raw() as usize].kind else {
            return false;
        };
        ty.implements
            .iter()
            .filter_map(|implemented| self.symbol_for_hir(HirId::TypeRef(*implemented)))
            .any(|symbol| symbol == expected)
    }

    fn lookup_member(&self, receiver: TypeId, name: &str, kind: SymbolKind) -> Option<SymbolId> {
        let nominal = match self.interner.get(receiver).kind {
            TypeKind::ObjectId(inner) => match self.interner.get(inner).kind {
                TypeKind::Nominal(symbol) => symbol,
                _ => return None,
            },
            TypeKind::Nominal(symbol) => symbol,
            _ => return None,
        };
        let DeclarationSite::Item(item) = self.semantic.symbols.symbol(nominal).declaration else {
            return None;
        };
        let type_scope = self.semantic.scopes.owned_scope(ScopeOwner::Item(item))?;
        let member_scope = self
            .semantic
            .scopes
            .get(type_scope)
            .children
            .iter()
            .copied()
            .find(|scope| self.semantic.scopes.get(*scope).kind == ScopeKind::Member)?;
        self.semantic
            .symbols
            .lookup_local(member_scope, Namespace::Member, name)
            .and_then(|set| {
                let symbol = set.primary;
                (self.semantic.symbols.symbol(symbol).kind == kind).then_some(symbol)
            })
    }

    fn symbol_for_hir(&self, id: HirId) -> Option<SymbolId> {
        match self.semantic.bindings.by_hir.get(&id) {
            Some(ResolvedBinding::Symbol(symbol)) => Some(*symbol),
            _ => None,
        }
    }

    fn symbol_for_declaration(&self, declaration: DeclarationSite) -> Option<SymbolId> {
        self.semantic
            .symbols
            .symbols
            .iter()
            .find(|symbol| symbol.declaration == declaration)
            .map(|symbol| symbol.id)
    }

    fn generic_param_in_scope(&self, hir_id: HirId, name: &str) -> Option<GenericParamId> {
        let mut scope = match hir_id {
            HirId::TypeRef(id) => self.semantic.scopes.enclosing_scope(HirId::TypeRef(id)),
            HirId::Expr(id) => self.semantic.scopes.enclosing_scope(HirId::Expr(id)),
            _ => None,
        };
        while let Some(current) = scope {
            let owner = self.semantic.scopes.get(current).owner;
            let owner_hir = match owner {
                ScopeOwner::Item(item) => Some(HirId::Item(item)),
                ScopeOwner::Member(member) => Some(HirId::Member(member)),
                _ => None,
            };
            if let Some(owner_hir) = owner_hir {
                if let Some(param) = self.generic_params.get(&(owner_hir, name.to_string())) {
                    return Some(*param);
                }
            }
            scope = self.semantic.scopes.get(current).parent;
        }
        None
    }

    fn generic_arity(&self, symbol: SymbolId) -> usize {
        let owner = match self.semantic.symbols.symbol(symbol).declaration {
            DeclarationSite::Item(item) => HirId::Item(item),
            DeclarationSite::Member(member) => HirId::Member(member),
            _ => return 0,
        };
        self.types
            .generic_params
            .iter()
            .filter(|param| param.owner == owner)
            .count()
    }

    fn builtin_type(&self, name: &str) -> Option<TypeId> {
        let builtins = self.builtins();
        match name {
            "Bool" => Some(builtins.bool_),
            "Char" => Some(builtins.char_),
            "Int" => Some(builtins.int),
            "UInt" => Some(builtins.uint),
            "Float" => Some(builtins.float),
            "Double" => Some(builtins.double),
            "String" => Some(builtins.string),
            "Unit" => Some(builtins.unit),
            _ => None,
        }
    }

    fn is_numeric(&self, ty: TypeId) -> bool {
        matches!(
            self.interner.get(ty).kind,
            TypeKind::Primitive(
                PrimitiveType::Int
                    | PrimitiveType::UInt
                    | PrimitiveType::Float
                    | PrimitiveType::Double
            )
        )
    }

    fn is_error(&self, ty: TypeId) -> bool {
        matches!(self.interner.get(ty).kind, TypeKind::Error)
    }

    fn type_display(&self, ty: TypeId) -> String {
        match &self.interner.get(ty).kind {
            TypeKind::Primitive(primitive) => format!("{primitive:?}"),
            TypeKind::Unit => "Unit".to_string(),
            TypeKind::Nominal(symbol) => self.semantic.symbols.symbol(*symbol).name.clone(),
            TypeKind::ObjectId(inner) => format!("ObjectId<{}>", self.type_display(*inner)),
            TypeKind::Function(_) => "<function>".to_string(),
            TypeKind::GenericParam(param) => self
                .types
                .generic_params
                .get(param.raw() as usize)
                .map(|param| param.name.clone())
                .unwrap_or_else(|| "<generic>".to_string()),
            TypeKind::GenericInstance { base, args } => format!(
                "{}<{}>",
                self.semantic.symbols.symbol(*base).name,
                args.iter()
                    .map(|arg| self.type_display(*arg))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            TypeKind::Tuple(elements) => format!(
                "({})",
                elements
                    .iter()
                    .map(|element| self.type_display(*element))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            TypeKind::Array(element) => format!("[{}]", self.type_display(*element)),
            TypeKind::Unknown(raw) => format!("?{raw}"),
            TypeKind::Error => "<error>".to_string(),
        }
    }

    fn push_type_diagnostic(
        &mut self,
        code: &str,
        message: impl Into<String>,
        span: Span,
        label: impl Into<String>,
    ) -> Diagnostic {
        let diagnostic = Diagnostic::error(message)
            .with_code(DiagnosticCode::new(code))
            .with_primary_span(span)
            .with_label(DiagnosticLabel::primary(span, label));
        self.diagnostics.push(diagnostic.clone());
        diagnostic
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
        if namespace == Namespace::Type
            && path.segments.len() == 1
            && is_builtin_type_name(&path.segments[0].text)
        {
            self.bindings.by_hir.insert(
                hir_id,
                ResolvedBinding::BuiltinType(path.segments[0].text.clone()),
            );
            return;
        }
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

fn is_builtin_type_name(name: &str) -> bool {
    matches!(
        name,
        "Bool" | "Char" | "Int" | "UInt" | "Float" | "Double" | "String" | "Unit"
    )
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
            ResolvedBinding::BuiltinType(name) => format!("builtin_type({name})"),
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
