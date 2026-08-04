# HIR Model

**Projeto:** Linguagem Capi  
**Documento:** HIR-MODEL  
**Status:** Aprovado  
**Stage:** Stage 3 — HIR e resolução de nomes  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o modelo de engenharia da Representação Semântica de Alto Nível (*High-level Intermediate Representation* — HIR) da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- a estrutura conceitual da HIR;
- as categorias obrigatórias de elementos HIR;
- o contrato de IDs internos;
- a rastreabilidade entre código-fonte, AST e HIR;
- os estados de enriquecimento semântico;
- os dados que pertencem à HIR inicial;
- os dados adicionados por resolução de nomes e fases posteriores;
- as invariantes que devem ser preservadas;
- os requisitos para dumps determinísticos;
- os testes mínimos exigidos para o Stage 3.

A HIR é a representação comum usada pelas fases semânticas do frontend. Ela nasce do lowering da AST e permanece independente da AST, da MIR, do backend, da ABI e da plataforma de destino.

---

## 2. Escopo

Este documento cobre:

- unidade HIR;
- módulos e imports pendentes;
- itens de alto nível;
- classes, interfaces, traits e seus membros;
- funções, métodos, construtores e parâmetros;
- tipos pendentes;
- blocos, comandos, expressões e padrões;
- IDs HIR tipados;
- origem, spans e mapeamento AST-HIR;
- slots de enriquecimento semântico;
- representação de erros HIR;
- invariantes estruturais;
- interface esperada para resolução de nomes, tipos e lowering para MIR;
- formato conceitual de dump HIR.

Este documento não cobre:

- algoritmo de lowering AST-HIR;
- regras detalhadas de resolução de nomes;
- representação completa de símbolos;
- modelo completo de escopos;
- inferência e verificação de tipos;
- regras de ownership;
- análise de Domains;
- lowering para MIR;
- layout de objetos;
- geração de código;
- ABI.

Esses temas pertencem a:

- `AST-LOWERING.md`;
- `SYMBOL-MODEL.md`;
- `SCOPE-MODEL.md`;
- `NAME-RESOLUTION.md`;
- `TYPE-MODEL.md`;
- `TYPE-INFERENCE.md`;
- `OWNERSHIP-MODEL.md`;
- `DOMAIN-IMPLEMENTATION.md`;
- `MIR-MODEL.md`;
- `MIR-LOWERING.md`;
- `OBJECT-MODEL.md`;
- `ABI-IMPLEMENTATION.md`.

---

## 3. Posição no Pipeline

A HIR é produzida imediatamente após o parser construir a AST.

Fluxo conceitual:

```text
SourceFile
    ↓
Lexer
    ↓
TokenStream
    ↓
Parser
    ↓
AST
    ↓
AST Lowering
    ↓
HIR inicial
    ↓
Resolução de nomes
    ↓
HIR com símbolos resolvidos
    ↓
Inferência e verificação de tipos
    ↓
HIR tipada
    ↓
Verificação semântica
    ↓
HIR validada
    ↓
Lowering para MIR
```

No Stage 3, a implementação deve produzir pelo menos:

- HIR inicial a partir de AST válida;
- IDs internos determinísticos;
- estrutura suficiente para módulos, imports, itens, blocos e expressões;
- campos ou tabelas auxiliares para anexar resultados de resolução de nomes;
- dump determinístico para `capic --emit hir`.

---

## 4. Princípios

O modelo de HIR deve seguir estes princípios:

- a HIR representa semântica de alto nível, não gramática;
- a AST não deve ser modificada para construir ou enriquecer HIR;
- a HIR inicial não deve conter nomes resolvidos;
- a HIR inicial não deve conter tipos inferidos;
- cada elemento HIR relevante deve possuir identidade própria;
- IDs HIR não devem reutilizar IDs da AST;
- spans e origem AST devem ser preservados quando aplicável;
- normalizações devem preservar significado;
- fases posteriores devem enriquecer a HIR de forma controlada;
- a representação deve ser determinística;
- dumps não devem depender de endereços de memória nem ordem instável de mapas;
- a HIR deve ser independente de MIR, backend, runtime, ABI e layout físico de objetos.

---

## 5. Entidades Conceituais

| Entidade | Responsabilidade |
| --- | --- |
| `Hir` | Artefato raiz da representação semântica de uma unidade ou conjunto de unidades. |
| `HirUnitId` | Identificador de unidade HIR. |
| `HirUnit` | Unidade HIR produzida para um arquivo-fonte ou módulo raiz de compilação. |
| `HirId` | Identificador genérico de elemento HIR quando uma API genérica for necessária. |
| `HirItemId` | Identificador de item HIR. |
| `HirImportId` | Identificador de import HIR. |
| `HirMemberId` | Identificador de membro de classe, interface ou trait. |
| `HirParamId` | Identificador de parâmetro HIR. |
| `HirLocalId` | Identificador de declaração local HIR. |
| `HirStmtId` | Identificador de comando HIR. |
| `HirExprId` | Identificador de expressão HIR. |
| `HirTypeRefId` | Identificador de referência de tipo pendente. |
| `HirPatternId` | Identificador de padrão HIR. |
| `HirBlockId` | Identificador de bloco HIR. |
| `HirErrorId` | Identificador de erro estrutural preservado na HIR. |
| `HirOrigin` | Origem de um elemento HIR no código-fonte e na AST. |
| `UnresolvedPath` | Caminho textual ainda não resolvido. |
| `ResolvedRef` | Referência semântica preenchida pela resolução de nomes. |
| `HirError` | Marcador explícito para erro estrutural ou nó inválido. |

Os nomes concretos podem variar na implementação. O contrato obrigatório é a presença dessas responsabilidades e das invariantes descritas neste documento.

---

## 6. Identidade e IDs

Todo elemento HIR que possa receber informação semântica posterior deve possuir identidade estável dentro da HIR.

Contrato conceitual:

```rust
pub struct HirUnitId(u32);
pub struct HirItemId(u32);
pub struct HirImportId(u32);
pub struct HirMemberId(u32);
pub struct HirParamId(u32);
pub struct HirLocalId(u32);
pub struct HirStmtId(u32);
pub struct HirExprId(u32);
pub struct HirTypeRefId(u32);
pub struct HirPatternId(u32);
pub struct HirBlockId(u32);
pub struct HirErrorId(u32);
```

Regras:

- IDs devem ser únicos dentro da categoria correspondente;
- IDs devem ser atribuídos de forma determinística;
- IDs não devem codificar significado semântico;
- IDs não devem depender de endereço de memória;
- IDs não precisam ser estáveis entre versões do compilador;
- IDs não precisam ser estáveis entre programas diferentes;
- a ordem de criação deve ser documentada pela implementação;
- APIs públicas internas devem preferir IDs tipados a índices crus.

Quando uma API precisar aceitar múltiplas categorias de elemento, pode existir um identificador somado:

```rust
pub enum HirId {
    Unit(HirUnitId),
    Item(HirItemId),
    Import(HirImportId),
    Member(HirMemberId),
    Param(HirParamId),
    Local(HirLocalId),
    Stmt(HirStmtId),
    Expr(HirExprId),
    TypeRef(HirTypeRefId),
    Pattern(HirPatternId),
    Block(HirBlockId),
    Error(HirErrorId),
}
```

`HirId` não substitui IDs tipados em APIs específicas.

---

## 7. Origem e Rastreabilidade

Cada elemento HIR relevante deve preservar origem suficiente para diagnósticos e ferramentas.

Contrato conceitual:

```rust
pub struct HirOrigin {
    pub source: SourceId,
    pub span: Span,
    pub ast_node: Option<AstNodeId>,
}
```

Regras:

- `source` deve identificar o arquivo-fonte original;
- `span` deve apontar para a construção principal que originou o elemento;
- `ast_node` deve apontar para o nó AST principal quando houver correspondência direta;
- elementos sintéticos devem usar origem sintética explícita ou origem representativa;
- normalizações que combinam múltiplos nós devem preservar origem principal e permitir origens auxiliares quando necessário;
- elementos gerados a partir de erro devem preservar a região problemática quando possível;
- nenhum enriquecimento posterior pode apagar a rastreabilidade inicial.

Um nó AST pode gerar zero, um ou vários elementos HIR. Um elemento HIR pode representar uma forma canônica derivada de mais de um nó AST.

---

## 8. Estrutura Raiz

A HIR deve possuir uma raiz capaz de representar a unidade compilada e os elementos semanticamente relevantes produzidos pelo lowering.

Contrato conceitual:

```rust
pub struct Hir {
    pub units: Vec<HirUnitId>,
    pub imports: HirArena<HirImport>,
    pub items: HirArena<HirItem>,
    pub members: HirArena<HirMember>,
    pub params: HirArena<HirParam>,
    pub locals: HirArena<HirLocal>,
    pub blocks: HirArena<HirBlock>,
    pub stmts: HirArena<HirStmt>,
    pub exprs: HirArena<HirExpr>,
    pub type_refs: HirArena<HirTypeRef>,
    pub patterns: HirArena<HirPattern>,
    pub errors: HirArena<HirError>,
}
```

Este contrato é conceitual. A implementação pode usar arenas, vetores, mapas ordenados, árvores, grafos ou outra estrutura equivalente.

Regras:

- a HIR deve permitir traversal completo sem consultar tokens;
- a HIR deve permitir traversal sem depender da estrutura física da AST;
- listas com ordem textual relevante devem preservar essa ordem;
- a raiz deve permitir encontrar módulos, imports e itens de cada unidade;
- a HIR parcial, quando suportada, deve ser marcada explicitamente como inválida.

---

## 9. Unidade, Módulos e Imports

### 9.1 Unidade HIR

Cada `CompilationUnit` da AST deve produzir uma unidade HIR.

Contrato conceitual:

```rust
pub struct HirUnit {
    pub source: SourceId,
    pub module: HirModulePath,
    pub imports: Vec<HirImportId>,
    pub items: Vec<HirItemId>,
    pub origin: HirOrigin,
    pub state: HirValidity,
}
```

Regras:

- `source` deve ser preservado da AST;
- `module` pode ser explícito ou implícito conforme a sessão de compilação;
- imports e itens preservam ordem textual;
- a unidade não deve validar existência de módulos importados;
- a unidade não deve resolver o caminho físico do módulo.

### 9.2 Caminho de Módulo

O caminho de módulo deve permanecer pendente na HIR inicial.

Contrato conceitual:

```rust
pub enum HirModulePath {
    Explicit(UnresolvedPath),
    Implicit,
}
```

Regras:

- caminhos explícitos preservam segmentos e span;
- módulo implícito deve ser representado de forma distinguível;
- validação de consistência entre módulo e sistema de arquivos pertence à toolchain ou resolução apropriada.

### 9.3 Imports

Imports devem ser representados como entradas próprias ou como itens HIR distinguíveis.

Contrato conceitual:

```rust
pub struct HirImport {
    pub path: UnresolvedPath,
    pub kind: HirImportKind,
    pub alias: Option<HirName>,
    pub origin: HirOrigin,
    pub resolution: Option<ResolvedImport>,
}

pub enum HirImportKind {
    Named,
    Wildcard,
}
```

Regras:

- imports permanecem não resolvidos na HIR inicial;
- wildcard deve ser distinguível de identificador comum;
- aliases preservam nome e span;
- erros de import inexistente pertencem à resolução de nomes.

---

## 10. Nomes e Caminhos Pendentes

Identificadores e caminhos usados em declarações, tipos e expressões devem ser representados sem resolução na HIR inicial.

Contrato conceitual:

```rust
pub struct HirName {
    pub symbol: Symbol,
    pub span: Span,
}

pub struct UnresolvedPath {
    pub segments: Vec<HirName>,
    pub span: Span,
}
```

Regras:

- `Symbol` representa texto internado, não símbolo resolvido;
- caminhos qualificados preservam ordem dos segmentos;
- caminhos vazios são inválidos e devem produzir `HirError` ou bloqueio;
- a HIR inicial não deve decidir se um caminho aponta para módulo, tipo, função, campo, método, variável, constante ou namespace;
- palavras-chave primitivas usadas como nomes de tipos podem permanecer como caminhos pendentes ou usar forma própria de tipo primitivo pendente, conforme a implementação.

---

## 11. Itens

Itens representam declarações semanticamente relevantes.

Contrato conceitual:

```rust
pub enum HirItem {
    Function(HirFunction),
    Class(HirClass),
    Interface(HirInterface),
    Trait(HirTrait),
    Const(HirConst),
    TypeAlias(HirTypeAlias),
    Import(HirImportId),
    Error(HirErrorId),
}
```

Regras:

- cada declaração válida deve produzir um item HIR;
- cada item deve possuir `HirItemId`;
- nome declarado deve ser preservado;
- modificadores e atributos devem permanecer pendentes;
- duplicidade de nomes não deve ser diagnosticada durante criação da HIR;
- visibilidade efetiva e validade de combinação de modificadores pertencem à análise semântica;
- itens devem expor local suficiente para criação de símbolos no Stage 3.

---

## 12. Modificadores e Atributos

Modificadores e atributos são preservados na HIR como dados pendentes.

Contrato conceitual:

```rust
pub struct HirModifier {
    pub kind: HirModifierKind,
    pub origin: HirOrigin,
}

pub struct HirAttribute {
    pub path: UnresolvedPath,
    pub args: Vec<HirExprId>,
    pub origin: HirOrigin,
}
```

Regras:

- o lowering pode converter modificadores conhecidos para enum;
- modificadores desconhecidos ou malformados devem produzir erro estrutural se chegaram à AST;
- duplicatas podem permanecer para diagnóstico posterior;
- atributos não devem executar efeitos durante o lowering;
- `@domain` e atributos semelhantes devem permanecer como metadados pendentes até a fase responsável.

---

## 13. Classes, Interfaces e Traits

### 13.1 Classes

Contrato conceitual:

```rust
pub struct HirClass {
    pub name: HirName,
    pub generics: Vec<HirGenericParam>,
    pub modifiers: Vec<HirModifier>,
    pub attributes: Vec<HirAttribute>,
    pub extends: Option<HirTypeRefId>,
    pub implements: Vec<HirTypeRefId>,
    pub uses_traits: Vec<HirTypeRefId>,
    pub members: Vec<HirMemberId>,
    pub origin: HirOrigin,
    pub symbol: Option<SymbolId>,
}
```

Regras:

- superclasse, interfaces e traits permanecem como tipos pendentes;
- membros preservam ordem textual;
- `symbol` é preenchido pela resolução de nomes;
- layout, vtables, herança válida e inicialização pertencem a fases posteriores.

### 13.2 Interfaces

Interfaces devem preservar nome, parâmetros genéricos, modificadores, atributos e membros.

Regras:

- membros não devem ser validados quanto à permissividade semântica nessa etapa;
- relações de implementação pertencem à análise semântica posterior;
- símbolos são anexados somente após resolução.

### 13.3 Traits

Traits devem preservar nome, parâmetros genéricos, modificadores, atributos e membros.

Regras:

- composição, conflitos e restrições de estado persistente não pertencem ao modelo inicial;
- uso de traits em classes deve permanecer como referência pendente.

### 13.4 Membros

Contrato conceitual:

```rust
pub enum HirMember {
    Field(HirField),
    Method(HirFunction),
    Constructor(HirConstructor),
    Const(HirConst),
    Error(HirErrorId),
}
```

Regras:

- campos preservam nome, tipo pendente e inicializador opcional;
- métodos podem compartilhar a estrutura de funções, com contexto de pertencimento;
- construtores devem ser distinguíveis de funções comuns;
- acesso a `this`, campos e métodos não deve ser resolvido na HIR inicial.

---

## 14. Funções, Métodos e Construtores

Contrato conceitual:

```rust
pub struct HirFunction {
    pub name: HirName,
    pub generics: Vec<HirGenericParam>,
    pub params: Vec<HirParamId>,
    pub return_type: Option<HirTypeRefId>,
    pub body: Option<HirBlockId>,
    pub modifiers: Vec<HirModifier>,
    pub attributes: Vec<HirAttribute>,
    pub origin: HirOrigin,
    pub symbol: Option<SymbolId>,
}

pub struct HirParam {
    pub name: HirName,
    pub type_ref: Option<HirTypeRefId>,
    pub default_value: Option<HirExprId>,
    pub origin: HirOrigin,
    pub symbol: Option<SymbolId>,
}
```

Regras:

- retorno omitido pode permanecer ausente ou virar referência pendente explícita para `Unit`;
- função sem corpo deve ser representada como assinatura;
- parâmetros preservam ordem textual;
- tipos de parâmetros permanecem pendentes;
- corpos são blocos HIR;
- símbolos de função e parâmetros são preenchidos pela resolução de nomes;
- overload, tipo de retorno obrigatório, `override`, `static` e assinatura efetiva pertencem a fases posteriores.

Construtores devem possuir forma distinguível:

```rust
pub struct HirConstructor {
    pub params: Vec<HirParamId>,
    pub body: Option<HirBlockId>,
    pub modifiers: Vec<HirModifier>,
    pub attributes: Vec<HirAttribute>,
    pub origin: HirOrigin,
    pub symbol: Option<SymbolId>,
}
```

Regras:

- construtor não deve inventar identificador de usuário;
- chamadas a outros construtores ou superclasse não são validadas no modelo inicial.

---

## 15. Tipos Pendentes

Referências de tipo na HIR inicial representam tipos escritos ou implícitos ainda não resolvidos.

Contrato conceitual:

```rust
pub enum HirTypeRef {
    Path(UnresolvedPath),
    PrimitivePending(HirPrimitiveTypeName),
    Generic {
        base: HirTypeRefId,
        args: Vec<HirTypeRefId>,
    },
    Array {
        element: HirTypeRefId,
        size: Option<HirExprId>,
    },
    Tuple(Vec<HirTypeRefId>),
    Function {
        params: Vec<HirTypeRefId>,
        ret: HirTypeRefId,
    },
    Error(HirErrorId),
}
```

Regras:

- todo tipo escrito deve preservar span e origem;
- `PrimitivePending` é opcional; a implementação pode representar tipos primitivos como `Path`;
- argumentos genéricos preservam ordem;
- bounds genéricos permanecem pendentes;
- arrays não devem avaliar tamanho constante nessa etapa;
- alias, subtipagem, variância, coerções e existência do tipo não são verificados na HIR inicial;
- resolução final para `TypeId` pertence ao Stage 4.

---

## 16. Blocos e Comandos

Blocos representam sequências ordenadas de comandos.

Contrato conceitual:

```rust
pub struct HirBlock {
    pub stmts: Vec<HirStmtId>,
    pub origin: HirOrigin,
    pub scope: Option<ScopeId>,
}
```

Regras:

- comandos preservam ordem textual;
- bloco vazio é válido;
- `scope` é preenchido ou associado pela fase de escopos;
- blocos implícitos devem possuir origem representativa.

Contrato conceitual de comandos:

```rust
pub enum HirStmt {
    Local(HirLocalId),
    Expr(HirExprId),
    Return(Option<HirExprId>),
    Break(Option<HirExprId>),
    Continue,
    If(HirIf),
    While(HirWhile),
    For(HirFor),
    Switch(HirSwitch),
    Match(HirMatch),
    Block(HirBlockId),
    Error(HirErrorId),
}
```

Regras:

- declarações locais preservam nome, mutabilidade, tipo pendente e inicializador;
- controle de fluxo preserva estrutura sem validar tipos;
- `else if` pode ser normalizado como `else` contendo outro `if`;
- corpo único pode ser normalizado para bloco implícito;
- alvo de `break` e `continue` não é resolvido nessa etapa;
- alcançabilidade não é calculada na HIR inicial.

Declarações locais devem possuir identidade própria quando puderem ser alvo de resolução.

Contrato conceitual:

```rust
pub struct HirLocal {
    pub name: HirName,
    pub mutable: bool,
    pub type_ref: Option<HirTypeRefId>,
    pub initializer: Option<HirExprId>,
    pub origin: HirOrigin,
    pub symbol: Option<SymbolId>,
}
```

---

## 17. Expressões

Expressões HIR preservam a estrutura semântica necessária para resolução, tipagem e lowering.

Contrato conceitual:

```rust
pub enum HirExpr {
    Literal(HirLiteral),
    Path(UnresolvedPath, Option<ResolvedRef>),
    This(HirOrigin),
    Call(HirCall),
    MemberAccess(HirMemberAccess),
    Index(HirIndex),
    New(HirNew),
    Unary(HirUnary),
    Binary(HirBinary),
    Assign(HirAssign),
    Block(HirBlockId),
    If(HirIfExpr),
    Match(HirMatchExpr),
    Error(HirErrorId),
}
```

Regras:

- literais preservam categoria, valor normalizado quando disponível e span;
- nomes e caminhos permanecem não resolvidos na HIR inicial;
- chamadas preservam callee e argumentos;
- acesso a membro preserva base e nome do membro;
- indexação preserva base e índice;
- `new` preserva tipo pendente e argumentos;
- operadores preservam operador sintático;
- atribuição preserva lado esquerdo e direito;
- agrupamento sintático redundante pode ser removido;
- overload de operadores, coerções, tipos e avaliação constante não pertencem à HIR inicial.

---

## 18. Padrões, Switch e Match

Padrões devem ser representados como estruturas pendentes até que tipos e nomes sejam conhecidos.

Contrato conceitual:

```rust
pub enum HirPattern {
    Wildcard(HirOrigin),
    Binding(HirName, Option<SymbolId>),
    Path(UnresolvedPath, Option<ResolvedRef>),
    Literal(HirLiteral),
    Tuple(Vec<HirPatternId>),
    Constructor {
        path: UnresolvedPath,
        args: Vec<HirPatternId>,
    },
    Error(HirErrorId),
}
```

Regras:

- padrões preservam forma e ordem dos subpadrões;
- bindings recebem símbolo apenas durante resolução;
- exaustividade não é verificada no Stage 3;
- compatibilidade entre padrão e tipo analisado pertence ao Stage 4 ou fase semântica posterior.

`switch` e `match` devem preservar expressão analisada, casos ou braços, padrões, corpos e spans.

---

## 19. Enriquecimento Semântico

A HIR evolui por associação de dados produzidos por fases específicas.

Estados conceituais:

```text
Initial
HIR criada a partir da AST, com nomes e tipos pendentes.

NamesResolved
Símbolos, imports, escopos e referências resolvidas foram anexados.

Typed
Tipos internos foram anexados a expressões, itens e referências de tipo.

Validated
Verificações semânticas obrigatórias foram executadas.

Lowerable
HIR possui dados suficientes para lowering para MIR.
```

Regras:

- fases devem registrar explicitamente quais dados adicionam;
- uma fase não deve sobrescrever dados de outra sem contrato específico;
- dados ausentes devem ser representados de forma explícita (`None`, estado pendente ou erro);
- a HIR não deve fingir resolução bem-sucedida usando símbolos sentinela;
- elementos inválidos devem manter erro associado para evitar cascatas desnecessárias;
- a transição de estado deve ser determinística.

---

## 20. Slots de Resolução

Elementos que dependem de resolução de nomes devem possuir slot explícito para o resultado.

Contrato conceitual:

```rust
pub enum ResolvedRef {
    Module(ModuleId),
    Item(HirItemId),
    Member(HirMemberId),
    Local(HirLocalId),
    Param(HirParamId),
    Type(TypeId),
    Error(HirErrorId),
}
```

Regras:

- a HIR inicial deve deixar slots vazios;
- resolução de nomes preenche slots de forma monotônica;
- conflitos e ambiguidades devem apontar para erro estruturado;
- referências resolvidas devem apontar para identidades internas, não para texto;
- o modelo de símbolo detalhado pertence a `SYMBOL-MODEL.md`.

---

## 21. Erros HIR e Validade Parcial

A HIR deve representar explicitamente situações em que a AST não permite elemento semântico válido.

Contrato conceitual:

```rust
pub enum HirValidity {
    Valid,
    Invalid,
}

pub struct HirError {
    pub kind: HirErrorKind,
    pub origin: HirOrigin,
}
```

Categorias iniciais:

- nó AST de erro em posição obrigatória;
- ausência de span obrigatório;
- path vazio;
- declaração incompleta;
- tipo sintático incompleto;
- expressão incompleta;
- construção ainda não suportada pelo lowering;
- violação de invariante estrutural da AST.

Regras:

- HIR inválida não deve ser tratada como semanticamente válida;
- para compilação normal, erro sintático bloqueador deve impedir análise semântica completa;
- ferramentas podem solicitar HIR parcial se a implementação suportar esse modo;
- dumps de HIR parcial devem exibir marcadores de erro;
- diagnósticos do parser não devem ser apagados nem substituídos por erros HIR genéricos.

---

## 22. Normalizações Permitidas

A HIR pode normalizar:

- agrupamentos sintáticos redundantes;
- `else if` para `else` contendo `if`;
- corpo de comando único para bloco implícito;
- função sem corpo para assinatura;
- retorno omitido para ausência explícita ou `Unit` pendente;
- paths sintáticos para `UnresolvedPath`;
- modificadores para flags ou enum pendente;
- atributos para metadados pendentes;
- padrões equivalentes para forma canônica.

Toda normalização deve preservar:

- significado;
- span útil;
- origem AST;
- ordem observável;
- capacidade de emitir diagnóstico preciso.

---

## 23. Normalizações Proibidas

A HIR inicial não deve:

- resolver nomes;
- criar símbolos definitivos;
- inferir tipos;
- verificar subtipagem;
- aplicar coerções;
- selecionar overload;
- avaliar constantes;
- expandir biblioteca padrão;
- validar ownership;
- aplicar efeitos de Domains;
- decidir layout de objetos;
- transformar `new` em alocação concreta;
- introduzir MIR;
- depender de backend ou ABI.

---

## 24. Invariantes

Uma HIR válida deve obedecer às seguintes invariantes:

- todo ID referenciado existe na arena ou tabela correspondente;
- todo elemento relevante possui origem;
- todo span real pertence a um `SourceId` conhecido;
- listas ordenadas preservam ordem textual ou ordem canônica documentada;
- elementos pendentes são distinguíveis de elementos resolvidos;
- elementos com erro possuem `HirError` associado;
- a AST não é necessária para percorrer a HIR;
- a AST continua disponível apenas como origem e suporte a ferramentas;
- HIR inicial não contém resultado de fases posteriores;
- HIR enriquecida preserva IDs e origem da HIR inicial.

Após resolução de nomes, invariantes adicionais devem ser verificadas pelos documentos de símbolos, escopos e resolução.

---

## 25. Interface com AST Lowering

O lowering deve produzir HIR compatível com este modelo. A implementação do lowering pertence a `capi-lowering`; `capi-hir` define apenas o modelo HIR e utilitários diretamente ligados a ele.

Entrada conceitual:

```rust
pub struct AstLoweringInput<'a> {
    pub ast: &'a Ast,
    pub source_map: &'a SourceMap,
}
```

Saída conceitual:

```rust
pub struct AstLoweringOutput {
    pub hir: Option<Hir>,
    pub ast_to_hir: AstToHirMap,
    pub diagnostics: Vec<Diagnostic>,
    pub blocked: bool,
}
```

Regras:

- AST válida deve produzir HIR inicial válida;
- AST com erro bloqueador pode impedir produção de HIR;
- AST parcial pode produzir HIR inválida somente em modo que suporte ferramentas;
- o lowering deve preencher IDs, origens, spans e elementos pendentes;
- o mapeamento AST-HIR pertence ao lowering e não deve ser necessário para percorrer ou analisar a HIR;
- o lowering não deve preencher símbolos resolvidos nem tipos finais.

---

## 26. Interface com Resolução de Nomes

A HIR deve fornecer à resolução de nomes:

- unidades e módulos;
- imports pendentes;
- itens nomeados;
- membros nomeados;
- parâmetros e declarações locais;
- blocos e escopos lexicais inferíveis;
- caminhos não resolvidos;
- referências de tipo pendentes;
- padrões com bindings;
- spans e origens para diagnósticos.

A resolução de nomes deve devolver ou anexar:

- símbolos declarados;
- escopos;
- referências resolvidas;
- erros de duplicidade;
- erros de nome inexistente;
- erros de ambiguidade;
- estado de resolução da HIR.

Detalhes de tabelas de símbolos, escopos e algoritmo de resolução pertencem a `SYMBOL-MODEL.md`, `SCOPE-MODEL.md` e `NAME-RESOLUTION.md`.

---

## 27. Interface com Tipagem

A HIR deve fornecer à fase de tipos:

- funções e assinaturas;
- parâmetros;
- tipos pendentes;
- expressões;
- operadores;
- chamadas;
- construtores;
- membros;
- padrões;
- referências resolvidas quando exigidas.

A fase de tipos deve anexar ou produzir:

- `TypeId` para tipos resolvidos;
- tipos de expressões;
- tipos de declarações locais;
- assinatura efetiva de funções;
- resultado de coerções permitidas;
- diagnósticos de incompatibilidade.

A HIR inicial não deve exigir que esses dados existam.

---

## 28. Interface com MIR Lowering

Ao final da análise semântica, a HIR validada deve ser suficiente para lowering para MIR.

A HIR lowerable deve fornecer:

- corpo de funções e métodos;
- blocos e controle de fluxo;
- referências resolvidas;
- tipos necessários para seleção de operações;
- chamadas já identificadas;
- acesso a membros validado;
- inicializações relevantes;
- padrões já verificados ou transformáveis.

O lowering para MIR não deve depender:

- da AST;
- dos tokens;
- de parse recovery;
- de nomes textuais quando identidade resolvida for obrigatória;
- de decisões de backend.

---

## 29. Diagnósticos

A HIR não é responsável por diagnosticar semântica por conta própria, mas deve carregar dados suficientes para diagnósticos precisos.

Regras:

- cada diagnóstico semântico deve poder apontar para span relevante;
- diagnósticos de resolução devem usar origem de paths, nomes e declarações;
- diagnósticos de tipo devem usar origem de expressões e referências de tipo;
- diagnósticos sobre declarações duplicadas devem conseguir apontar para declaração atual e declaração anterior;
- marcadores `HirError` devem evitar cascatas quando um erro estrutural já foi registrado;
- dumps e testes não devem depender da renderização final dos diagnósticos.

---

## 30. Dump HIR

O comando `capic --emit hir arquivo.capi` deve produzir dump textual determinístico quando implementado no Stage 3.

Requisitos:

- exibir unidade e módulo;
- exibir imports;
- exibir itens e membros em ordem estável;
- exibir assinaturas;
- exibir blocos, comandos, expressões e padrões;
- exibir tipos pendentes como pendentes;
- exibir caminhos não resolvidos como não resolvidos;
- exibir símbolos resolvidos somente quando o dump for executado após resolução;
- exibir spans em formato estável;
- exibir marcadores de erro quando houver HIR parcial;
- não exibir endereços de memória;
- não depender de ordem instável de `HashMap`;
- não exigir backend, MIR nem geração de código.

Exemplo conceitual:

```text
unit source=0 module=<implicit> span=0..24
  item fn main id=item0 span=0..24 symbol=<pending>
    params []
    return <pending Unit>
    block id=block0 span=16..24
      stmt expr id=stmt0
        literal Int value=1 span=18..19 type=<pending>
```

O formato final pertence à implementação, desde que seja legível, determinístico e adequado a testes de snapshot.

---

## 31. Determinismo

Para a mesma AST, mesma versão do compilador e mesmas opções:

- a HIR inicial deve ser funcionalmente equivalente;
- IDs devem ser atribuídos de modo determinístico;
- traversal deve produzir ordem estável;
- diagnósticos estruturais devem ser determinísticos;
- dumps devem ser determinísticos;
- mapas internos usados no dump devem ser ordenados antes de impressão.

A implementação não deve depender de:

- endereços de memória;
- ordem aleatória de hash;
- paralelismo sem junção ordenada;
- locale do sistema;
- caminhos absolutos não normalizados.

---

## 32. Testes Obrigatórios

Testes do modelo de HIR no Stage 3 devem cobrir:

- unidade mínima;
- módulo implícito;
- módulo explícito;
- import simples;
- import wildcard;
- função livre;
- função sem corpo;
- função com retorno omitido;
- parâmetros com tipo;
- parâmetros sem tipo quando permitido pelo subconjunto;
- classe vazia;
- classe com campo;
- classe com método;
- classe com construtor;
- interface;
- trait;
- modificadores;
- atributos;
- tipo nomeado;
- tipo genérico;
- array;
- tupla;
- tipo função;
- declaração local `let`;
- declaração local `const`;
- bloco vazio;
- bloco com múltiplos comandos;
- `if`;
- `else if` normalizado;
- `while`;
- `for`;
- `switch`;
- `match`;
- padrões básicos;
- literal;
- caminho não resolvido;
- chamada;
- acesso a membro;
- indexação;
- `new`;
- operadores unários;
- operadores binários;
- atribuição;
- agrupamento removido sem perda de span útil;
- preservação de spans;
- preservação de origem AST;
- IDs determinísticos;
- HIR com erro bloqueador;
- HIR parcial marcada como inválida, se suportada;
- dump determinístico.

Testes de HIR inicial não devem exigir resolução de nomes, inferência de tipos, ownership, Domains, MIR ou backend.

---

## 33. Critérios de Aceite

Este documento é considerado aprovado para orientar a implementação do Stage 3 quando:

- define as entidades conceituais obrigatórias da HIR;
- define o contrato de IDs HIR;
- define origem e rastreabilidade;
- define estado inicial e enriquecimento progressivo;
- separa dados pendentes de dados resolvidos;
- define normalizações permitidas e proibidas;
- define invariantes estruturais;
- define interfaces com lowering, resolução de nomes, tipagem e MIR lowering;
- define requisitos de dump;
- define testes obrigatórios.

A implementação correspondente será considerada concluída quando:

- AST válida produzir HIR inicial;
- HIR inicial preservar spans e origem AST;
- HIR inicial não depender da estrutura física da AST;
- nomes e tipos permanecerem pendentes antes das fases apropriadas;
- símbolos possuírem identidade interna estável após resolução;
- `capic --emit hir arquivo.capi` produzir dump determinístico;
- erros estruturais forem representados ou bloquearem a HIR de forma explícita;
- todos os testes obrigatórios do Stage 3 passarem.

---

## 34. Relações Normativas

Este documento depende diretamente de:

- Documento 02 — Sistema de Tipos;
- Documento 04 — Sintaxe da Linguagem;
- Documento 05 — Semântica Operacional;
- Documento 06 — Arquitetura do Compilador;
- Documento 13 — Estrutura do Compilador;
- Documento 15 — Parser e AST;
- Documento 16 — HIR;
- Documento 17 — Resolução de Nomes;
- Documento 18 — Inferência e Verificação de Tipos;
- Documento 21 — MIR e Fluxo de Controle;
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial;
- `AST-MODEL.md`;
- `AST-LOWERING.md`;
- `TOKEN-MODEL.md`;
- `SPANS-AND-LOCATIONS.md`;
- `DIAGNOSTIC-DATA-MODEL.md`.

Este documento orienta diretamente:

- `SYMBOL-MODEL.md`;
- `SCOPE-MODEL.md`;
- `NAME-RESOLUTION.md`;
- `TYPE-MODEL.md`;
- `TYPE-INFERENCE.md`;
- `TYPE-CHECKING-PIPELINE.md`;
- `SEMANTIC-TESTS.md`;
- `MIR-LOWERING.md`;
- implementação de `capic --emit hir`.
