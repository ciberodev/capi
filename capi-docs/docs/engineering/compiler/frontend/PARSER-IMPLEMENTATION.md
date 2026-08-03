# Parser Implementation

**Projeto:** Linguagem Capi  
**Documento:** PARSER-IMPLEMENTATION  
**Status:** Aprovado  
**Stage:** Stage 2 — Parser e AST  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o contrato de engenharia para a implementação do parser da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- entradas e saídas do parser;
- organização interna recomendada;
- contrato do cursor de tokens;
- ordem de parsing da unidade de compilação;
- parsing de módulos, imports, declarações, tipos, comandos e expressões;
- estratégia de precedência e associatividade;
- integração com AST, spans e diagnósticos;
- pontos de integração com recuperação sintática;
- dump de AST exigido pelo Stage 2;
- testes mínimos para validar a implementação.

---

## 2. Escopo

Este documento cobre:

- implementação inicial do parser;
- contrato do crate ou módulo de parser;
- consumo de `TokenStream`;
- construção de AST;
- emissão de diagnósticos sintáticos;
- parsing do subconjunto sintático inicial do Stage 2;
- precedência de operadores;
- integração com `--emit ast`;
- limites de responsabilidade do parser.

Este documento não cobre:

- modelo completo da AST;
- modelo completo de tokens;
- algoritmo detalhado de recuperação de erros;
- lowering de AST para HIR;
- resolução de nomes;
- escopos;
- inferência ou verificação de tipos;
- semântica de operadores;
- formato definitivo de snapshots de teste;
- formatação oficial da linguagem.

Esses temas pertencem a:

- `AST-MODEL.md`;
- `TOKEN-MODEL.md`;
- `PARSER-RECOVERY.md`;
- `AST-LOWERING.md`;
- `HIR-MODEL.md`;
- `NAME-RESOLUTION.md`;
- `TYPE-MODEL.md`;
- `PARSER-TESTS.md`.

---

## 3. Princípios

A implementação do parser deve seguir estes princípios:

- responsabilidade única: transformar tokens em AST;
- independência de HIR, resolução de nomes, tipos, MIR, runtime e backends;
- determinismo para a mesma sequência de tokens;
- ausência de panic para entrada de usuário malformada;
- diagnósticos sintáticos estruturados;
- preservação de spans em todos os nós relevantes;
- produção de AST parcial quando a recuperação for possível;
- precedência e associatividade refletidas na forma da AST;
- nenhuma validação semântica durante o parsing;
- simplicidade de implementação no Stage 2.

---

## 4. Papel no Pipeline

Fluxo conceitual:

```text
SourceFile
    ↓
Lexer
    ↓
TokenStream + diagnósticos léxicos
    ↓
Parser
    ↓
AST + diagnósticos sintáticos
    ↓
Dump de AST ou lowering para HIR
```

O parser recebe tokens já produzidos pelo lexer. Ele não deve abrir arquivos, ler texto diretamente como fonte primária, resolver módulos ou construir HIR.

O parser pode consultar `SourceMap` apenas para necessidades auxiliares, como recuperar lexemas de identificadores ou produzir diagnósticos mais precisos. Decisões sintáticas devem ser tomadas a partir de tokens.

---

## 5. Entradas

Entrada conceitual:

```rust
pub struct ParseInput<'a> {
    pub source_id: SourceId,
    pub tokens: &'a [Token],
}
```

A implementação pode receber `TokenStream`, slice de tokens, iterador indexável ou estrutura equivalente, desde que preserve:

- ordem dos tokens;
- acesso a lookahead;
- spans de cada token;
- token EOF;
- integração com coletor de diagnósticos.

Regras:

- comentários e whitespace não devem chegar ao parser por padrão;
- o parser deve aceitar tokens de erro léxico apenas se o lexer os produzir como parte da sequência;
- diagnósticos léxicos já emitidos não devem ser reinterpretados como diagnósticos sintáticos;
- EOF deve existir e ser consumível como sentinela.

---

## 6. Saídas

Saída conceitual:

```rust
pub struct ParseOutput {
    pub ast: Ast,
    pub diagnostics: Vec<Diagnostic>,
}
```

Ou interface equivalente:

```rust
pub fn parse(input: ParseInput, diagnostics: &mut DiagnosticBag) -> Ast;
```

Regras:

- código sintaticamente válido deve produzir AST sem diagnósticos sintáticos de erro;
- código sintaticamente inválido deve produzir diagnósticos estruturados;
- quando houver recuperação, a AST pode conter `AstErrorNode`;
- erro sintático não deve ser retornado por `panic`;
- se erro léxico bloqueador já impedir sequência útil de tokens, o driver pode interromper antes do parser.

---

## 7. Organização do Crate

O Stage 2 deve criar ou consolidar um componente dedicado ao parser, preferencialmente um crate como:

```text
capi-parser
```

Responsabilidades do componente:

- expor API pública de parsing;
- consumir tokens definidos pelo frontend;
- construir nós definidos pelo modelo de AST;
- emitir diagnósticos sintáticos;
- implementar precedência de operadores;
- fornecer dados suficientes para o dump de AST;
- manter testes unitários de parsing.

Dependências permitidas:

- `capi-source`;
- `capi-diagnostics`;
- `capi-lexer` ou crate que exponha `Token`;
- `capi-ast` ou módulo equivalente;
- `capi-common`, se necessário;
- biblioteca padrão Rust.

O parser não deve depender de:

- HIR;
- resolução de nomes;
- checagem de tipos;
- ownership;
- MIR;
- codegen;
- driver como dependência interna.

O driver pode depender do parser para executar `capic --emit ast`.

---

## 8. Estrutura Interna do Parser

Estado conceitual:

```rust
struct Parser<'a> {
    source: SourceId,
    tokens: &'a [Token],
    cursor: TokenCursor<'a>,
    diagnostics: DiagnosticSink,
    ast: AstBuilder,
}
```

Componentes recomendados:

| Componente | Responsabilidade |
| --- | --- |
| `TokenCursor` | Navegação determinística sobre tokens. |
| `Parser` | Coordenação das rotinas de parsing. |
| `AstBuilder` | Criação centralizada de nós e spans. |
| `DiagnosticSink` | Emissão de diagnósticos sintáticos. |
| `Recovery` | Sincronização após erro recuperável. |
| `Precedence` | Tabela de precedência e associatividade. |

A implementação pode fundir componentes quando isso reduzir complexidade, desde que preserve responsabilidades claras.

---

## 9. Cursor de Tokens

O parser deve acessar tokens por meio de um cursor ou abstração equivalente.

Contrato conceitual:

```rust
struct TokenCursor<'a> {
    tokens: &'a [Token],
    index: usize,
}
```

Operações mínimas:

```rust
fn current(&self) -> &Token;
fn nth(&self, n: usize) -> &Token;
fn at(&self, kind: TokenKind) -> bool;
fn bump(&mut self) -> Token;
fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError>;
fn is_eof(&self) -> bool;
```

Regras:

- `current()` em fim de sequência deve retornar EOF, não panic;
- lookahead além do fim deve retornar EOF;
- `bump()` não deve avançar além de EOF de forma observável;
- `expect()` deve emitir ou permitir emitir diagnóstico quando o token esperado estiver ausente;
- o cursor não deve consultar HIR, símbolos ou tipos.

---

## 10. Estratégia de Parsing

A estratégia de referência para o Stage 2 é:

```text
Recursive descent para módulos, declarações, tipos e comandos.
Pratt parser ou precedence climbing para expressões.
```

Essa estratégia é recomendada porque:

- a sintaxe inicial é próxima de linguagens OO modernas;
- declarações e comandos são naturalmente recursivos;
- expressões exigem controle claro de precedência;
- a implementação é simples de testar e depurar.

Outras estratégias são permitidas se preservarem:

- AST funcionalmente equivalente;
- spans equivalentes;
- diagnósticos compatíveis;
- dump determinístico;
- integração com recuperação sintática.

---

## 11. Ordem de Parsing da Unidade

A unidade de compilação deve ser processada nesta ordem:

```text
1. declaração opcional de módulo
2. zero ou mais imports
3. zero ou mais declarações de alto nível
4. EOF
```

Contrato conceitual:

```rust
fn parse_compilation_unit(&mut self) -> CompilationUnit;
```

Regras:

- módulo, quando presente, deve aparecer antes de imports e declarações;
- imports devem aparecer antes de declarações de alto nível;
- tokens após uma declaração completa devem ser interpretados como início de nova declaração ou erro;
- EOF deve ser esperado explicitamente;
- tokens inesperados em nível superior devem gerar diagnóstico e recuperação até próximo início plausível de declaração.

---

## 12. Parsing de Módulo e Imports

### 12.1 Módulo

Contrato conceitual:

```rust
fn parse_module_decl(&mut self) -> ModuleDecl;
```

Forma sintática inicial:

```text
module path ;
```

Regras:

- `module` inicia declaração de módulo;
- o caminho deve ser parseado como sequência de identificadores separados por `.`;
- `;` encerra a declaração;
- o span cobre de `module` até `;`;
- o parser não valida correspondência entre módulo e caminho físico do arquivo.

### 12.2 Imports

Contrato conceitual:

```rust
fn parse_import_decl(&mut self) -> ImportDecl;
```

Forma sintática inicial:

```text
import path ;
import path . * ;
```

Regras:

- `import` inicia declaração de import;
- import com wildcard deve ser distinguido na AST;
- o parser não valida existência, visibilidade ou ciclos de dependência;
- imports fora da região inicial devem gerar diagnóstico sintático ou ser tratados conforme recuperação.

---

## 13. Parsing de Declarações

Contrato conceitual:

```rust
fn parse_decl(&mut self) -> Decl;
```

O parser deve reconhecer pelo menos:

- funções;
- classes;
- interfaces;
- traits;
- constantes;
- declarações globais com `let`, se permitidas no subconjunto;
- nós de erro para declarações malformadas.

Ordem recomendada:

```text
1. atributos
2. modificadores
3. palavra-chave principal da declaração
4. cabeçalho específico
5. corpo, assinatura ou inicializador
```

Regras:

- atributos e modificadores devem ser preservados na AST;
- duplicidade ou combinação inválida pode ser diagnosticada apenas quando for regra sintática clara;
- significado de visibilidade, abstração, `static`, `final` e `override` não pertence ao parser;
- a ordem textual das declarações deve ser preservada;
- declaração incompleta deve tentar produzir `Decl::Error` ou declaração parcial com nó de erro.

---

## 14. Atributos e Modificadores

### 14.1 Atributos

Contrato conceitual:

```rust
fn parse_attributes(&mut self) -> Vec<Attribute>;
```

Forma sintática inicial:

```text
@Name
@Name(arg1, arg2)
@module.Name(key = value)
```

Regras:

- atributos aparecem antes de modificadores e declarações;
- argumentos devem ser preservados como expressões ou pares sintáticos, conforme `AST-MODEL.md`;
- o parser não interpreta efeitos de atributos;
- atributos malformados devem usar recuperação local até `)`, nova linha relevante, `{`, `;` ou início de declaração.

### 14.2 Modificadores

Contrato conceitual:

```rust
fn parse_modifiers(&mut self) -> Vec<Modifier>;
```

Modificadores iniciais:

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

- modificadores devem preservar ordem textual;
- o parser pode aceitar lista ampla e deixar validação semântica para fases posteriores;
- modificador em posição impossível deve gerar diagnóstico sintático;
- um modificador isolado sem declaração deve produzir erro recuperável.

---

## 15. Classes, Interfaces e Traits

### 15.1 Classes

Contrato conceitual:

```rust
fn parse_class_decl(&mut self, prefix: DeclPrefix) -> ClassDecl;
```

Forma sintática inicial:

```text
class Name GenericParams? ExtendsClause? ImplementsClause? UsesClause? ClassBody
```

Regras:

- `class` deve ser seguida por identificador;
- parâmetros genéricos devem ser preservados quando presentes;
- `extends`, `implements` e `uses` são cláusulas sintáticas;
- o corpo da classe é delimitado por `{` e `}`;
- membros são parseados até `}` ou EOF;
- corpo vazio é válido sintaticamente;
- o parser não valida herança simples, existência de tipos ou conflitos de membros.

### 15.2 Interfaces

Contrato conceitual:

```rust
fn parse_interface_decl(&mut self, prefix: DeclPrefix) -> InterfaceDecl;
```

Regras:

- `interface` deve ser seguida por identificador;
- corpo é delimitado por `{` e `}`;
- assinaturas de funções sem corpo devem ser aceitas quando a gramática permitir;
- o parser não valida se o membro é permitido semanticamente em interface.

### 15.3 Traits

Contrato conceitual:

```rust
fn parse_trait_decl(&mut self, prefix: DeclPrefix) -> TraitDecl;
```

Regras:

- `trait` deve ser seguida por identificador;
- corpo é delimitado por `{` e `}`;
- métodos com corpo e assinaturas sem corpo podem ser preservados conforme gramática;
- o parser não valida regras de estado persistente ou composição.

### 15.4 Membros

Contrato conceitual:

```rust
fn parse_member_decl(&mut self) -> MemberDecl;
```

Membros iniciais:

- campos;
- métodos;
- construtores;
- constantes;
- nós de erro.

Regras:

- campos preservam nome, tipo e inicializador quando presentes;
- métodos usam a mesma rotina de funções com contexto de membro;
- `constructor` deve produzir nó próprio;
- erro em membro deve recuperar até `;`, `}`, ou próximo início plausível de membro.

---

## 16. Funções e Construtores

### 16.1 Funções

Contrato conceitual:

```rust
fn parse_function_decl(&mut self, prefix: DeclPrefix) -> FunctionDecl;
```

Forma sintática inicial:

```text
function name GenericParams? ( ParamList? ) ReturnType? FunctionBody?
```

Regras:

- `function` deve ser seguida por identificador;
- parâmetros são delimitados por `(` e `)`;
- tipo de retorno explícito é introduzido por `:`;
- retorno omitido deve permanecer omitido na AST;
- corpo é bloco quando presente;
- assinatura sem corpo deve exigir `;` se a gramática do contexto permitir;
- o parser não valida sobrecarga, tipo de retorno, recursão ou despacho.

### 16.2 Construtores

Contrato conceitual:

```rust
fn parse_constructor_decl(&mut self, prefix: DeclPrefix) -> ConstructorDecl;
```

Forma sintática inicial:

```text
constructor ( ParamList? ) Block
```

Regras:

- construtor não deve ser convertido em função no parser;
- construtor deve preservar modificadores quando presentes;
- corpo deve ser parseado como bloco;
- validações sobre chamada de superconstrutor pertencem a fases posteriores.

### 16.3 Parâmetros

Contrato conceitual:

```rust
fn parse_param_list(&mut self) -> Vec<Param>;
fn parse_param(&mut self) -> Param;
```

Forma sintática inicial:

```text
name : Type
name : Type = Expr
```

Regras:

- vírgulas separam parâmetros;
- trailing comma pode ser aceita somente se a gramática aprovar;
- parâmetros devem preservar ordem textual;
- tipo ausente deve gerar diagnóstico sintático se obrigatório no contexto;
- valor padrão é preservado como expressão quando permitido.

---

## 17. Parsing de Tipos

Contrato conceitual:

```rust
fn parse_type(&mut self) -> TypeSyntax;
```

Tipos iniciais:

- nome simples;
- caminho qualificado;
- tipo genérico;
- array;
- tupla;
- tipo funcional, quando a gramática do subconjunto exigir;
- nó de erro.

Regras:

- tipos primitivos são parseados como nomes sintáticos;
- argumentos genéricos são delimitados por `<` e `>`;
- arrays preservam elemento e tamanho sintático quando presente;
- tuplas preservam ordem dos elementos;
- o parser não resolve aliases, tipos primitivos, bounds ou variância.

### 17.1 Ambiguidade entre `<` e Operador

Em contexto de tipo, `<` e `>` devem ser interpretados como delimitadores de argumentos genéricos quando a gramática permitir.

Em contexto de expressão, `<` e `>` devem ser interpretados como operadores relacionais, exceto em posições sintáticas explicitamente reconhecidas como argumentos genéricos de chamada ou construção.

A implementação deve manter essa distinção por contexto, não por consulta semântica.

---

## 18. Blocos e Comandos

### 18.1 Blocos

Contrato conceitual:

```rust
fn parse_block(&mut self) -> Block;
```

Regras:

- bloco inicia com `{` e termina com `}`;
- comandos são parseados até `}` ou EOF;
- bloco vazio produz lista vazia;
- span do bloco inclui delimitadores;
- EOF dentro de bloco gera diagnóstico de delimitador ausente.

### 18.2 Comandos

Contrato conceitual:

```rust
fn parse_stmt(&mut self) -> Stmt;
```

Comandos iniciais:

- declaração local `let`;
- constante local `const`;
- expressão como comando;
- `return`;
- `if`;
- `switch`;
- `match`;
- `while`;
- `for`;
- `foreach`, se a palavra-chave estiver disponível no token model;
- `break`;
- `continue`;
- bloco;
- bloco `unsafe`;
- nó de erro.

Regras:

- comandos simples devem consumir `;`;
- comandos compostos delimitados por bloco não precisam de `;` final;
- `return` deve distinguir presença e ausência de expressão;
- `break` e `continue` não resolvem destino;
- declaração local preserva tipo opcional e inicializador;
- o parser não valida alcançabilidade, escopo, inicialização ou retorno obrigatório.

---

## 19. Controle de Fluxo

### 19.1 `if`

Forma sintática inicial:

```text
if ( Expr ) StmtOrBlock else StmtOrBlock
```

Regras:

- condição deve ser expressão delimitada por parênteses;
- ramo `else` é opcional;
- `else if` é representado como `else` contendo outro `IfStmt` ou forma equivalente;
- tipo booleano da condição não é verificado pelo parser.

### 19.2 `switch`

Forma sintática inicial:

```text
switch ( Expr ) { Case* Default? }
```

Regras:

- casos preservam ordem textual;
- `default` deve ser distinguido de `case`;
- duplicidade de casos e exaustividade não pertencem ao parser.

### 19.3 Laços

Formas iniciais:

```text
while ( Expr ) StmtOrBlock
for ( Init? ; Expr? ; Expr? ) StmtOrBlock
foreach ( PatternOrName in Expr ) StmtOrBlock
```

Regras:

- componentes ausentes do `for` devem ser representados explicitamente;
- `foreach` preserva variável ou padrão e expressão iterada;
- o parser não verifica se a expressão é iterável.

### 19.4 `match`

Forma sintática inicial:

```text
match ( Expr ) { MatchArm* }
```

Regras:

- cada braço deve preservar padrão e corpo;
- `case` inicia braço quando a gramática assim definir;
- exaustividade e compatibilidade de padrões pertencem à análise semântica.

---

## 20. Expressões

Expressões devem ser parseadas com precedência e associatividade determinísticas.

Contrato conceitual:

```rust
fn parse_expr(&mut self) -> Expr;
fn parse_expr_bp(&mut self, min_bp: BindingPower) -> Expr;
```

A implementação pode usar Pratt parser, precedence climbing ou níveis recursivos separados.

### 20.1 Expressões Primárias

Primárias iniciais:

- literais;
- identificadores;
- `this`;
- agrupamento com `(` e `)`;
- tupla;
- array literal;
- criação com `new`;
- `match` como expressão, se permitido;
- nó de erro.

Regras:

- literais preservam token e span;
- identificadores são nomes sintáticos;
- agrupamento explícito deve aparecer na AST quando necessário para rastreabilidade;
- tupla de um elemento deve seguir a regra sintática definida pela linguagem;
- `new` preserva tipo e argumentos.

### 20.2 Pós-fixos

Pós-fixos iniciais:

- chamada `callee(args)`;
- argumentos genéricos explícitos, quando permitidos;
- acesso a membro `base.name`;
- indexação `base[index]`;
- conversão por chamada, como `valor.as<Int32>()`, preservada como acesso e chamada.

Regras:

- pós-fixos associam à esquerda;
- cadeias devem preservar ordem;
- o parser não resolve se chamada é função, método, construtor ou conversão.

### 20.3 Prefixos

Prefixos iniciais:

```text
!
-
+
```

`unsafe` pode iniciar bloco ou construção própria somente em contexto definido pela gramática.

### 20.4 Binários

Operadores binários iniciais:

```text
*
/
%
+
-
<
<=
>
>=
==
!=
===
&&
||
=
```

Atribuição deve possuir precedência baixa e associatividade à direita quando aceita como expressão.

### 20.5 Precedência Inicial

Tabela inicial recomendada, da maior para a menor precedência:

| Nível | Operadores ou construções | Associatividade |
| --- | --- | --- |
| 1 | chamada, acesso a membro, indexação | esquerda |
| 2 | prefixos `!`, `-`, `+` | direita |
| 3 | `*`, `/`, `%` | esquerda |
| 4 | `+`, `-` | esquerda |
| 5 | `<`, `<=`, `>`, `>=` | esquerda |
| 6 | `==`, `!=`, `===` | esquerda |
| 7 | `&&` | esquerda |
| 8 | `||` | esquerda |
| 9 | `=` | direita |

Essa tabela deve ser ajustada se o Documento 04 ou decisão posterior aprovar precedência mais específica. Enquanto isso, ela materializa a regra de precedência tradicional da família C para o Stage 2.

### 20.6 Condicional Ternário

Se o subconjunto do Stage 2 aceitar `cond ? a : b`, o parser deve tratá-lo como operador de precedência baixa, acima de atribuição e abaixo de `||`, com associatividade à direita.

Se a gramática ainda não aprovar essa construção, o parser deve rejeitar `?` com diagnóstico sintático.

---

## 21. Caminhos e Nomes

Contrato conceitual:

```rust
fn parse_identifier(&mut self) -> Identifier;
fn parse_path(&mut self) -> Path;
```

Regras:

- identificador deve vir de token `Identifier` ou palavra-chave contextual permitida;
- palavras reservadas não devem ser aceitas como identificador comum;
- caminho é sequência de segmentos separados por `.`;
- import wildcard deve usar rotina própria para aceitar `*`;
- span do caminho cobre do primeiro ao último segmento ou wildcard;
- resolução do caminho pertence a fases posteriores.

---

## 22. Delimitadores e Listas

O parser deve usar rotinas comuns para listas delimitadas.

Contrato conceitual:

```rust
fn parse_delimited_list<T>(
    open: TokenKind,
    close: TokenKind,
    separator: TokenKind,
    parse_item: impl FnMut(&mut Parser) -> T,
) -> Vec<T>;
```

Listas comuns:

- parâmetros `(...)`;
- argumentos de chamada `(...)`;
- argumentos genéricos `<...>`;
- elementos de array `[...]`;
- corpo de bloco `{...}`;
- membros de classe/interface/trait `{...}`;

Regras:

- delimitador ausente deve gerar diagnóstico;
- separador ausente deve gerar diagnóstico recuperável quando possível;
- item vazio deve ser permitido apenas quando a gramática permitir;
- trailing separator deve seguir regra explícita por contexto.

---

## 23. Spans

O parser é responsável por combinar spans de tokens em spans de nós.

Regras:

- nó folha usa span do token correspondente;
- nó composto usa span do primeiro token da construção até o último token consumido;
- delimitadores fazem parte do span quando delimitam a construção;
- item recuperado por inserção sintética usa span vazio no ponto de inserção;
- nó de erro usa span da região problemática;
- EOF usa span vazio no fim da fonte.

Operação conceitual:

```rust
fn span_from(start: Token, end: Token) -> Span;
fn empty_span_at(token: Token) -> Span;
```

Spans inválidos por bug interno devem ser tratados como erro interno de compilador.

---

## 24. Diagnósticos Sintáticos

O parser deve emitir diagnósticos estruturados por meio da infraestrutura comum.

Categorias iniciais:

- token inesperado;
- token obrigatório ausente;
- declaração incompleta;
- tipo incompleto;
- expressão incompleta;
- delimitador não fechado;
- separador ausente;
- construção não suportada pelo subconjunto atual;
- tokens restantes após fim esperado.

Contrato conceitual:

```rust
fn error_expected(&mut self, expected: SyntaxExpectation, found: &Token);
fn error_unexpected(&mut self, found: &Token);
```

Regras:

- todo diagnóstico deve ter span primário quando possível;
- diagnósticos não devem depender apenas de strings;
- mensagens humanas podem ser ajustadas sem alterar categorias;
- erro sintático esperado de usuário não é `InternalError`;
- pânico não controlado não substitui diagnóstico.

Detalhes de sincronização e continuidade pertencem a `PARSER-RECOVERY.md`.

---

## 25. Recuperação Sintática

Este documento define apenas pontos de integração com recuperação.

O parser deve chamar recuperação quando:

- `expect()` falhar para token obrigatório;
- uma declaração não puder ser reconhecida;
- delimitador de lista ou bloco estiver ausente;
- expressão estiver incompleta;
- tipo estiver incompleto;
- token inesperado aparecer em posição estrutural.

Pontos de sincronização iniciais:

```text
;
}
)
]
case
default
function
class
interface
trait
let
const
return
if
while
for
match
EOF
```

Regras:

- recuperação deve evitar loops infinitos;
- ao menos um token deve ser consumido quando não houver progresso;
- nós de erro devem ser explícitos quando ocuparem posição estrutural;
- diagnósticos devem continuar sendo emitidos após recuperação segura.

O algoritmo normativo de recuperação deve ser definido em `PARSER-RECOVERY.md`.

---

## 26. AST Builder

A criação de nós deve ser centralizada quando possível.

Responsabilidades do builder:

- atribuir `AstNodeId`, quando usado;
- construir nós com spans válidos;
- preservar ordem de filhos;
- criar nós de erro;
- evitar referências para dados temporários do parser;
- facilitar dumps determinísticos.

Contrato conceitual:

```rust
struct AstBuilder {
    next_id: u32,
}
```

Regras:

- IDs devem ser atribuídos de forma determinística;
- builder não deve realizar validação semântica;
- builder não deve consultar tabela de símbolos;
- builder pode internar identificadores via infraestrutura comum, se disponível.

---

## 27. Dump de AST

O Stage 2 exige resultado demonstrável por:

```bash
capic --emit ast arquivo.capi
```

Responsabilidades do parser para o dump:

- produzir AST completa o suficiente para ser percorrida;
- preservar spans e categorias;
- expor nós de erro explicitamente;
- manter ordem determinística de filhos;
- evitar dados não determinísticos.

O driver ou componente de dump pode ser separado do parser. O parser não deve formatar diretamente a saída final se houver módulo de dumps compartilhado.

Requisitos mínimos do dump:

- raiz `CompilationUnit`;
- módulos e imports;
- declarações e membros;
- comandos e expressões;
- tipos sintáticos;
- spans;
- nós de erro;
- ausência de dados semânticos.

---

## 28. Limites de Responsabilidade

O parser não deve:

- verificar se nomes existem;
- construir escopos;
- criar símbolos semânticos;
- inferir tipos;
- verificar compatibilidade de tipos;
- resolver overload;
- validar herança, implementação de interface ou composição de trait;
- verificar exaustividade de `match`;
- verificar ownership, borrow ou Domains;
- executar avaliação constante;
- gerar HIR, MIR ou código.

O parser pode:

- diagnosticar violações sintáticas;
- preservar modificadores e atributos;
- distinguir formas sintáticas;
- organizar precedência de expressões;
- produzir AST parcial;
- preservar spans para fases posteriores.

---

## 29. Determinismo

Para a mesma sequência de tokens, mesma versão do compilador e mesmas opções relevantes:

- a AST deve ser funcionalmente equivalente;
- diagnósticos sintáticos devem ter mesma categoria e spans equivalentes;
- ordem dos nós deve ser igual;
- dump de AST deve ser igual;
- IDs exibidos em dumps devem ser determinísticos.

O parser não deve depender de:

- ordem de hash maps não determinísticos;
- endereços de memória;
- paralelismo sem ordenação explícita;
- locale do sistema;
- caminho absoluto do ambiente, exceto quando o dump explicitamente exibir fonte.

---

## 30. Testes Obrigatórios

Os testes de implementação do parser devem cobrir:

- unidade de compilação mínima;
- módulo e imports;
- imports com wildcard;
- funções com e sem retorno explícito;
- parâmetros;
- classes vazias;
- classes com campos, métodos e construtores;
- interfaces;
- traits;
- modificadores;
- atributos;
- tipos nomeados e qualificados;
- tipos genéricos;
- arrays e tuplas;
- blocos;
- comandos simples;
- `if` e `else`;
- `while`, `for` e `foreach` quando habilitado;
- `switch`;
- `match`;
- literais;
- chamadas;
- acesso a membro;
- indexação;
- `new`;
- operadores prefixados;
- operadores binários por nível de precedência;
- associatividade;
- atribuição;
- erros de delimitador;
- token inesperado;
- declaração incompleta;
- expressão incompleta;
- recuperação sem loop infinito;
- dump determinístico de AST.

Testes de parser não devem depender de HIR, resolução de nomes, tipo ou backend.

---

## 31. Critérios de Aceite do Stage 2

Para este documento ser considerado implementado:

- existe API de parsing consumindo tokens do lexer;
- o parser produz AST conforme `AST-MODEL.md`;
- módulos, imports, declarações, classes, funções, tipos, comandos e expressões do subconjunto inicial são aceitos;
- precedência de operadores está implementada;
- spans são preservados nos nós relevantes;
- erros sintáticos produzem diagnósticos estruturados;
- recuperação básica continua após erros recuperáveis;
- `capic --emit ast arquivo.capi` funciona;
- dump de AST é determinístico;
- testes obrigatórios de parser passam.

---

## 32. Relações Normativas

Este documento depende diretamente de:

- Documento 04 — Sintaxe da Linguagem;
- Documento 14 — Lexer e Tokens;
- Documento 15 — Parser e AST;
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial;
- `TOKEN-MODEL.md`;
- `AST-MODEL.md`;
- `SPANS-AND-LOCATIONS.md`;
- `DIAGNOSTIC-DATA-MODEL.md`;
- `DIAGNOSTIC-ARCHITECTURE.md`.

Este documento orienta diretamente:

- `PARSER-RECOVERY.md`;
- `AST-LOWERING.md`;
- `PARSER-TESTS.md`;
- implementação de `capic --emit ast`.
