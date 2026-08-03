# AST Model

**Projeto:** Linguagem Capi  
**Documento:** AST-MODEL  
**Status:** Aprovado  
**Stage:** Stage 2 — Parser e AST  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o modelo de engenharia da Árvore Sintática Abstrata (*Abstract Syntax Tree* — AST) produzida pelo parser da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- a estrutura conceitual da AST;
- as categorias obrigatórias de nós sintáticos;
- o contrato comum de spans e rastreabilidade;
- a separação entre sintaxe e semântica;
- os limites entre AST, parser e HIR;
- a representação de nós incompletos ou recuperados;
- os requisitos para dumps determinísticos;
- os testes mínimos do modelo de AST no Stage 2.

---

## 2. Escopo

Este documento cobre:

- unidade de compilação;
- módulo e imports;
- declarações;
- membros de classes, interfaces e traits;
- parâmetros, modificadores e anotações;
- tipos sintáticos;
- blocos e comandos;
- expressões;
- padrões sintáticos usados por `match`;
- literais, nomes e caminhos;
- nós de erro e recuperação;
- invariantes estruturais da AST;
- interface esperada para lowering, diagnósticos, dumps e testes.

Este documento não cobre:

- algoritmo do parser;
- gramática formal completa;
- estratégia de recuperação sintática;
- resolução de nomes;
- escopos;
- inferência ou verificação de tipos;
- modelo de HIR;
- semântica de operadores;
- layout de objetos;
- geração de código;
- formatação oficial de código.

Esses temas pertencem a:

- `PARSER-IMPLEMENTATION.md`;
- `PARSER-RECOVERY.md`;
- `AST-LOWERING.md`;
- `HIR-MODEL.md`;
- `NAME-RESOLUTION.md`;
- `TYPE-MODEL.md`;
- `OBJECT-MODEL.md`;
- `MIR-MODEL.md`.

---

## 3. Princípios

O modelo de AST deve seguir estes princípios:

- a AST é a representação sintática oficial produzida pelo parser;
- cada nó representa uma construção sintática reconhecida;
- a AST não contém resolução de nomes, símbolos, tipos inferidos ou escopos;
- a AST preserva spans suficientes para diagnósticos e ferramentas;
- a AST preserva a ordem sintática de declarações, comandos, membros, argumentos e parâmetros;
- detalhes puramente léxicos, comentários e espaços em branco não fazem parte da AST por padrão;
- a AST é conceitualmente imutável após construída;
- a construção da HIR usa a AST como entrada, sem modificá-la;
- entradas inválidas podem produzir AST parcial com nós de erro;
- dumps de AST devem ser determinísticos.

---

## 4. Entidades Conceituais

| Entidade | Responsabilidade |
| --- | --- |
| `Ast` | Artefato raiz produzido para uma unidade de compilação. |
| `AstNodeId` | Identificador estável de nó dentro de uma AST. |
| `AstNode` | Nó sintático com categoria, span e filhos conceituais. |
| `CompilationUnit` | Raiz sintática de um arquivo-fonte Capi. |
| `Path` | Nome simples ou qualificado preservado sintaticamente. |
| `Identifier` | Identificador escrito pelo usuário. |
| `Modifier` | Qualificador sintático como `public`, `static` ou `override`. |
| `Attribute` | Anotação sintática aplicada a uma declaração. |
| `Decl` | Declaração de alto nível ou membro. |
| `TypeSyntax` | Representação sintática de um tipo. |
| `Stmt` | Comando ou item de bloco. |
| `Expr` | Expressão sintática. |
| `Pattern` | Padrão sintático usado em `match` e construções correlatas. |
| `AstErrorNode` | Nó de preservação estrutural para erro recuperado. |

Os nomes finais podem variar na implementação. O contrato obrigatório é a presença dessas responsabilidades e a preservação das invariantes deste documento.

---

## 5. Estrutura Raiz

Cada arquivo-fonte processado pelo parser deve produzir uma AST correspondente a uma unidade de compilação.

Contrato conceitual:

```rust
pub struct Ast {
    root: CompilationUnit,
    diagnostics: Vec<Diagnostic>,
}

pub struct CompilationUnit {
    source: SourceId,
    module: Option<ModuleDecl>,
    imports: Vec<ImportDecl>,
    declarations: Vec<Decl>,
    span: Span,
}
```

Regras:

- a raiz deve apontar para a `SourceId` da fonte analisada;
- `span` da unidade deve cobrir a região sintática processada;
- `module` representa a declaração explícita de módulo quando presente;
- `imports` preservam a ordem textual;
- `declarations` preservam a ordem textual;
- diagnósticos sintáticos podem acompanhar a AST, mas não substituem nós de erro quando a estrutura parcial for útil.

O parser deve produzir no máximo uma `CompilationUnit` por arquivo-fonte. A coordenação entre múltiplos arquivos pertence à sessão de compilação e às fases de resolução.

---

## 6. Identidade de Nós

A implementação deve oferecer uma forma estável de referenciar nós dentro de uma AST.

Contrato conceitual:

```rust
pub struct AstNodeId(u32);
```

Regras:

- o identificador deve ser único dentro da AST que o contém;
- o identificador não precisa ser estável entre execuções diferentes;
- o identificador não deve codificar significado semântico;
- fases posteriores podem usar `AstNodeId` para mapear diagnósticos, HIR e dados auxiliares;
- a ausência de identificador público é permitida somente se a implementação fornecer mecanismo equivalente de referência interna.

`AstNodeId` não substitui `Span`. Identidade interna e origem no código-fonte são responsabilidades distintas.

---

## 7. Contrato Comum dos Nós

Todo nó sintático relevante deve possuir:

- categoria sintática;
- span real ou sintético;
- relação estrutural com seus filhos;
- origem recuperável para diagnósticos quando aplicável.

Contrato conceitual:

```rust
pub trait AstNode {
    fn id(&self) -> AstNodeId;
    fn span(&self) -> Span;
}
```

A implementação pode usar `enum`, `structs` por categoria, arena tipada, árvore verde/vermelha ou outra representação equivalente. A escolha física não altera os contratos deste documento.

### 7.1 Spans dos Nós

Regras:

- nós derivados de tokens reais devem possuir span real;
- spans devem usar intervalo half-open `[start, end)`;
- o span de nó composto deve cobrir da primeira à última parte sintática relevante;
- delimitadores pertencentes à construção devem fazer parte do span do nó composto;
- identificadores e literais devem preservar seu span próprio;
- nós sintéticos devem usar span sintético explícito ou span vazio em posição relevante;
- nós de erro devem cobrir a região problemática quando possível.

Exemplos:

| Construção | Span esperado |
| --- | --- |
| `function f() {}` | de `function` até `}` |
| `a + b` | de `a` até `b` |
| `(a + b)` | de `(` até `)` |
| `Cliente<String>` | de `Cliente` até `>` |
| EOF recuperado | span vazio no ponto de inserção |

### 7.2 Imutabilidade

Após a construção, a AST deve ser tratada como conceitualmente imutável.

Regras:

- lowering para HIR não deve alterar a AST;
- resolução de nomes não deve escrever símbolos diretamente na AST;
- verificação de tipos não deve anexar tipos inferidos aos nós;
- caches auxiliares devem ficar fora da estrutura canônica da AST ou ser explicitamente derivados e invalidáveis;
- ferramentas podem manter índices externos baseados em `AstNodeId` e `Span`.

---

## 8. Nomes, Caminhos e Identificadores

Identificadores representam nomes escritos pelo usuário.

Contrato conceitual:

```rust
pub struct Identifier {
    text: Symbol,
    span: Span,
}

pub struct Path {
    segments: Vec<Identifier>,
    span: Span,
}
```

Regras:

- a AST pode armazenar símbolo internado para o texto do identificador;
- o lexema original deve permanecer recuperável pelo `SourceMap`;
- caminhos qualificados devem preservar a ordem dos segmentos;
- a AST não deve resolver se um caminho aponta para módulo, tipo, função, campo ou variável;
- wildcard de import, quando presente, deve ser representado como componente sintático próprio, não como identificador normal.

Exemplos de caminhos:

```text
banco.contas.Cliente
Cliente
banco.financeiro.*
```

---

## 9. Módulos e Imports

### 9.1 Módulo

Contrato conceitual:

```rust
pub struct ModuleDecl {
    path: Path,
    span: Span,
}
```

Regras:

- a AST preserva apenas a declaração sintática de módulo;
- o parser não verifica se o caminho corresponde ao diretório do arquivo;
- ausência de módulo explícito deve ser representada como `None` ou equivalente.

### 9.2 Imports

Contrato conceitual:

```rust
pub struct ImportDecl {
    path: ImportPath,
    span: Span,
}
```

`ImportPath` deve distinguir import de item específico e import com wildcard.

O parser não verifica existência, visibilidade, ciclos ou acessibilidade de módulos importados.

---

## 10. Declarações

Declarações introduzem construções sintáticas nomeadas ou estruturais.

Contrato conceitual:

```rust
pub enum Decl {
    Function(FunctionDecl),
    Class(ClassDecl),
    Interface(InterfaceDecl),
    Trait(TraitDecl),
    Const(ConstDecl),
    GlobalLet(LetDecl),
    Error(AstErrorNode),
}
```

A lista pode crescer conforme a gramática oficial evoluir. No Stage 2, a implementação deve cobrir pelo menos as construções exigidas pelo plano: módulos, declarações, classes, funções, tipos, expressões e comandos.

### 10.1 Cabeçalho Comum

Declarações que aceitam metadados devem preservar:

- atributos;
- modificadores;
- nome;
- parâmetros genéricos quando presentes;
- span total da declaração.

Contrato conceitual:

```rust
pub struct DeclHeader {
    attributes: Vec<Attribute>,
    modifiers: Vec<Modifier>,
    name: Identifier,
    generic_params: Vec<GenericParam>,
    span: Span,
}
```

### 10.2 Modificadores

Modificadores são elementos sintáticos como:

```text
public
protected
private
internal
abstract
sealed
final
static
override
unsafe
```

Regras:

- a AST preserva modificadores na ordem textual;
- o parser pode diagnosticar duplicação ou posicionamento sintaticamente inválido quando a gramática exigir;
- o significado de cada modificador pertence às fases semânticas.

### 10.3 Atributos

Anotações devem ser preservadas como estrutura sintática.

Contrato conceitual:

```rust
pub struct Attribute {
    name: Path,
    arguments: Vec<AttributeArgument>,
    span: Span,
}
```

O parser não interpreta o efeito de atributos como `@domain`.

---

## 11. Classes, Interfaces e Traits

### 11.1 Classes

Contrato conceitual:

```rust
pub struct ClassDecl {
    header: DeclHeader,
    extends: Option<TypeSyntax>,
    implements: Vec<TypeSyntax>,
    uses: Vec<TypeSyntax>,
    members: Vec<MemberDecl>,
    span: Span,
}
```

Regras:

- `extends`, `implements` e `uses` são relações sintáticas;
- a AST não valida existência ou compatibilidade dos tipos referenciados;
- membros devem preservar a ordem textual;
- o corpo vazio deve ser representado como lista vazia, não como ausência de corpo.

### 11.2 Interfaces

Contrato conceitual:

```rust
pub struct InterfaceDecl {
    header: DeclHeader,
    members: Vec<MemberDecl>,
    span: Span,
}
```

O parser deve preservar assinaturas de métodos e demais membros permitidos pela gramática, sem validar se o membro é semanticamente permitido para interface.

### 11.3 Traits

Contrato conceitual:

```rust
pub struct TraitDecl {
    header: DeclHeader,
    members: Vec<MemberDecl>,
    span: Span,
}
```

Traits podem conter membros com implementação padrão quando a sintaxe permitir. A validação de estado persistente ou regras de composição pertence às fases semânticas.

### 11.4 Membros

Contrato conceitual:

```rust
pub enum MemberDecl {
    Field(FieldDecl),
    Method(FunctionDecl),
    Constructor(ConstructorDecl),
    Const(ConstDecl),
    Error(AstErrorNode),
}
```

Campos devem preservar nome, tipo opcional quando a gramática permitir, inicializador opcional quando permitido, modificadores e span.

---

## 12. Funções, Construtores e Parâmetros

### 12.1 Funções

Contrato conceitual:

```rust
pub struct FunctionDecl {
    header: DeclHeader,
    params: Vec<Param>,
    return_type: Option<TypeSyntax>,
    body: Option<Block>,
    span: Span,
}
```

Regras:

- retorno omitido deve ser representado como ausência sintática, não como tipo `Unit` resolvido;
- funções sem corpo, quando aceitas pela gramática, usam `body: None`;
- funções com corpo vazio usam `body: Some(Block { statements: [] })`;
- sobrecarga não é resolvida na AST.

### 12.2 Construtores

Contrato conceitual:

```rust
pub struct ConstructorDecl {
    modifiers: Vec<Modifier>,
    params: Vec<Param>,
    body: Block,
    span: Span,
}
```

Construtores não devem ser convertidos em funções comuns dentro da AST. Qualquer normalização pertence ao lowering.

### 12.3 Parâmetros

Contrato conceitual:

```rust
pub struct Param {
    name: Identifier,
    type_annotation: Option<TypeSyntax>,
    default_value: Option<Expr>,
    span: Span,
}
```

O parser preserva anotações de tipo e valores padrão quando a gramática permitir. A validação de obrigatoriedade e compatibilidade pertence às fases posteriores.

---

## 13. Tipos Sintáticos

`TypeSyntax` representa a forma escrita de um tipo.

Contrato conceitual:

```rust
pub enum TypeSyntax {
    Named(NamedType),
    Generic(GenericType),
    Array(ArrayType),
    Tuple(TupleType),
    Function(FunctionType),
    Optional(OptionalType),
    Error(AstErrorNode),
}
```

Regras:

- tipos primitivos como `Int32`, `Bool`, `String` e `Unit` são nomes sintáticos até a resolução;
- `List<Cliente>` deve preservar nome e argumentos de tipo;
- `Int32[10]` deve preservar elemento e tamanho sintático quando presente;
- tuplas preservam ordem dos elementos;
- tipos funcionais preservam parâmetros e retorno sintáticos;
- o parser não verifica existência, alias, subtipagem, variância ou restrições genéricas.

### 13.1 Generics

Contrato conceitual:

```rust
pub struct GenericParam {
    name: Identifier,
    bounds: Vec<TypeSyntax>,
    span: Span,
}

pub struct GenericType {
    base: Path,
    arguments: Vec<TypeSyntax>,
    span: Span,
}
```

Restrições como `where T implements Entidade`, quando reconhecidas pela gramática, devem ser preservadas como nós sintáticos próprios. Sua interpretação pertence à análise semântica.

---

## 14. Blocos e Comandos

### 14.1 Blocos

Contrato conceitual:

```rust
pub struct Block {
    statements: Vec<Stmt>,
    span: Span,
}
```

Regras:

- o span do bloco inclui os delimitadores `{` e `}`;
- comandos preservam ordem textual;
- bloco vazio deve usar lista vazia;
- escopo lexical não é representado como entidade semântica na AST.

### 14.2 Comandos

Contrato conceitual:

```rust
pub enum Stmt {
    Let(LetDecl),
    Const(ConstDecl),
    Expr(ExprStmt),
    Return(ReturnStmt),
    If(IfStmt),
    Switch(SwitchStmt),
    Match(MatchStmt),
    While(WhileStmt),
    For(ForStmt),
    Foreach(ForeachStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    UnsafeBlock(UnsafeBlock),
    Block(Block),
    Error(AstErrorNode),
}
```

Regras:

- comandos de declaração local devem preservar tipo opcional e inicializador;
- `return;` deve distinguir ausência de expressão;
- `break` e `continue` não carregam destino semântico;
- `unsafe` é apenas marcação sintática nesta fase;
- alcance, inicialização, alcançabilidade e retorno obrigatório não são verificados pela AST.

---

## 15. Expressões

`Expr` representa construções sintáticas que produzem valores ou participam de computação.

Contrato conceitual:

```rust
pub enum Expr {
    Literal(LiteralExpr),
    Name(NameExpr),
    This(ThisExpr),
    New(NewExpr),
    Call(CallExpr),
    Member(MemberExpr),
    Index(IndexExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Assign(AssignExpr),
    Group(GroupExpr),
    Tuple(TupleExpr),
    Array(ArrayExpr),
    Conditional(ConditionalExpr),
    Cast(CastExpr),
    Match(MatchExpr),
    Error(AstErrorNode),
}
```

A implementação pode separar ou agrupar variantes, desde que preserve a estrutura sintática e a precedência correta.

### 15.1 Literais

Literais devem preservar:

- categoria léxica;
- lexema recuperável via span;
- valor normalizado quando já produzido pelo lexer;
- span do literal.

O parser não deve executar avaliação constante.

### 15.2 Nomes e Acessos

Regras:

- `cliente` é expressão de nome;
- `cliente.nome` é acesso a membro;
- `cliente.nome()` é chamada cujo callee é acesso a membro;
- `valor.as<Int32>()` deve preservar chamada, argumentos genéricos e acesso;
- a AST não resolve se um nome é variável local, campo, função, tipo ou módulo.

### 15.3 Operadores

Expressões unárias e binárias devem preservar o operador sintático e os operandos.

Contrato conceitual:

```rust
pub struct BinaryExpr {
    left: Box<Expr>,
    op: BinaryOp,
    right: Box<Expr>,
    span: Span,
}
```

O agrupamento da árvore deve refletir precedência e associatividade definidas pela sintaxe da linguagem. O significado do operador pertence às fases semânticas.

### 15.4 Criação de Objetos

`new Cliente("Gabriel")` deve ser preservado como construção sintática própria.

Contrato conceitual:

```rust
pub struct NewExpr {
    ty: TypeSyntax,
    arguments: Vec<Expr>,
    span: Span,
}
```

A AST não verifica construtores, alocação, Domains ou ownership.

---

## 16. Switch, Match e Padrões

### 16.1 Switch

`switch` deve preservar:

- expressão de seleção;
- casos na ordem textual;
- caso `default`, quando presente;
- blocos ou listas de comandos associadas a cada caso;
- spans de cada caso.

### 16.2 Match

`match` deve preservar:

- expressão analisada;
- lista de braços;
- padrão de cada braço;
- corpo de cada braço;
- spans.

Contrato conceitual:

```rust
pub struct MatchArm {
    pattern: Pattern,
    body: MatchArmBody,
    span: Span,
}
```

### 16.3 Padrões

Contrato conceitual:

```rust
pub enum Pattern {
    Name(Identifier),
    Path(Path),
    Constructor { path: Path, fields: Vec<Pattern>, span: Span },
    Literal(LiteralExpr),
    Wildcard(Span),
    Error(AstErrorNode),
}
```

Exaustividade, compatibilidade com classes seladas, `Optional` e `Result` pertencem às fases semânticas.

---

## 17. Nós de Erro e AST Parcial

O parser pode produzir nós de erro para preservar estrutura após recuperação sintática.

Contrato conceitual:

```rust
pub struct AstErrorNode {
    span: Span,
    expected: Vec<SyntaxExpectation>,
    found: Option<TokenKind>,
}
```

Regras:

- nó de erro representa falha sintática recuperada;
- nó de erro não deve ocultar diagnóstico correspondente;
- nó de erro deve possuir span útil quando possível;
- nó de erro pode ocupar a posição de declaração, membro, tipo, comando, expressão ou padrão;
- lowering deve decidir se a AST parcial pode avançar ou se deve bloquear fases posteriores;
- dumps devem representar nós de erro de forma explícita.

A presença de nós de erro não torna o programa válido. Ela apenas permite continuidade de análise e melhores diagnósticos.

---

## 18. Separação entre AST e Semântica

A AST não deve conter:

- `SymbolId` resolvido;
- `DefId`;
- `TypeId`;
- tipo inferido;
- escopo lexical resolvido;
- tabela de símbolos;
- resultado de overload resolution;
- informação de borrow checking;
- domínio ou região inferida;
- layout de objeto;
- bloco de MIR.

Permitido na AST:

- texto ou símbolo internado de identificador;
- categoria sintática;
- spans;
- listas de filhos;
- operadores sintáticos;
- modificadores escritos;
- anotações escritas;
- delimitadores relevantes quando necessários para rastreabilidade;
- marcadores de erro sintático.

Qualquer enriquecimento semântico deve ser produzido por HIR ou por mapas externos associados a fases posteriores.

---

## 19. Interface com o Parser

O parser é o único produtor canônico da AST no Stage 2.

Entrada conceitual:

```rust
pub fn parse(tokens: TokenStream, diagnostics: &mut DiagnosticBag) -> Ast;
```

Regras:

- o parser consome tokens produzidos pelo lexer;
- a AST não deve depender de acesso direto ao texto-fonte para decisões sintáticas;
- o parser pode consultar lexemas por span quando necessário para identificadores e literais;
- diagnósticos de parser devem usar a infraestrutura comum;
- a AST produzida deve ser utilizável mesmo quando houver diagnósticos recuperáveis.

---

## 20. Interface com HIR e Lowering

A AST é a entrada formal do lowering para HIR.

Regras:

- lowering não deve depender da sequência original de tokens;
- lowering não deve modificar a AST;
- lowering deve usar spans da AST para preservar rastreabilidade;
- lowering pode criar IDs semânticos próprios;
- normalizações semânticas, como retorno `Unit` implícito, pertencem ao lowering ou à HIR, não ao parser;
- construções sintáticas diferentes que sejam semanticamente equivalentes podem ser normalizadas somente após a AST.

Exemplos:

| AST | Lowering/HIR |
| --- | --- |
| retorno omitido em função | retorno `Unit` explícito, se aplicável |
| `Path` sintático | referência resolvível |
| modificadores textuais | flags semânticas validadas |
| `new Cliente()` | construção semântica de objeto |

---

## 21. Dumps de AST

O Stage 2 exige resultado demonstrável por:

```bash
capic --emit ast arquivo.capi
```

O dump da AST deve ser determinístico.

Requisitos:

- mesma entrada e mesma versão do compilador produzem o mesmo dump;
- nós aparecem em ordem sintática;
- spans aparecem em formato estável;
- identificadores e literais aparecem de forma recuperável;
- nós de erro aparecem explicitamente;
- dados semânticos não aparecem no dump de AST;
- endereços de memória, ponteiros e IDs não determinísticos não devem aparecer;
- quando IDs forem exibidos, devem ser atribuídos em ordem determinística.

Formato textual sugerido:

```text
CompilationUnit span=0..42
  ModuleDecl path=banco.contas span=0..20
  FunctionDecl name=main span=22..42
    Params
    ReturnType <omitted>
    Block span=38..42
```

O formato final pode ser ajustado por `COMPILER-DUMP-FLAGS.md`, desde que preserve estes requisitos.

---

## 22. Invariantes

A AST válida para fases posteriores deve obedecer:

- todo nó acessível a partir da raiz pertence à mesma AST;
- todo nó possui span real, sintético ou de erro válido;
- a ordem de listas corresponde à ordem textual;
- nenhum filho obrigatório válido é nulo;
- ausência sintática é representada por `Option`, lista vazia ou variante explícita;
- nós de erro são explícitos;
- expressões binárias refletem precedência e associatividade;
- blocos preservam seus comandos;
- declarações preservam seus membros;
- tipos sintáticos não são convertidos em tipos semânticos;
- a AST não contém referências para dados temporários do parser.

Violação dessas invariantes por bug interno deve ser tratada como erro interno do compilador, não como erro de usuário.

---

## 23. Testes Obrigatórios

Os testes do modelo de AST devem cobrir:

- construção de unidade de compilação vazia ou mínima;
- declaração de módulo;
- imports simples e com wildcard;
- declarações de função;
- parâmetros e retorno omitido;
- classes com campos, métodos e construtores;
- interfaces e traits;
- modificadores e atributos;
- tipos nomeados, genéricos, arrays e tuplas;
- blocos vazios e com comandos;
- `let`, `const`, `return`, `break` e `continue`;
- `if`, `while`, `for`, `foreach`, `switch` e `match`;
- literais e nomes;
- chamadas, acesso a membro e indexação;
- precedência e associatividade de operadores;
- criação com `new`;
- nós de erro em declaração, tipo, comando e expressão;
- preservação de spans;
- determinismo do dump.

Testes de AST devem ser independentes de resolução de nomes, tipos, ownership, MIR e backend.

---

## 24. Critérios de Aceite do Stage 2

Para este documento ser considerado implementado no Stage 2:

- o parser produz `CompilationUnit` para código sintaticamente válido;
- a AST cobre o subconjunto sintático inicial definido para o stage;
- todos os nós relevantes preservam spans;
- a AST não carrega dados semânticos;
- erros recuperáveis produzem diagnósticos e nós de erro quando útil;
- o dump de AST é determinístico;
- testes de AST passam junto com os testes de parser;
- `capic --emit ast arquivo.capi` demonstra a árvore produzida.

---

## 25. Relações Normativas

Este documento depende diretamente de:

- Documento 04 — Sintaxe da Linguagem;
- Documento 14 — Lexer e Tokens;
- Documento 15 — Parser e AST;
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial;
- `SPANS-AND-LOCATIONS.md`;
- `TOKEN-MODEL.md`;
- `DIAGNOSTIC-DATA-MODEL.md`.

Este documento orienta diretamente:

- `PARSER-IMPLEMENTATION.md`;
- `PARSER-RECOVERY.md`;
- `AST-LOWERING.md`;
- `PARSER-TESTS.md`;
- `HIR-MODEL.md`.
