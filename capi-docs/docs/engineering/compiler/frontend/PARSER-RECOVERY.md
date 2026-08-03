# Parser Recovery

**Projeto:** Linguagem Capi  
**Documento:** PARSER-RECOVERY  
**Status:** Aprovado  
**Stage:** Stage 2 — Parser e AST  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o contrato de engenharia para recuperação de erros sintáticos no parser da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- quando o parser deve tentar recuperar;
- como diagnósticos sintáticos são emitidos durante recuperação;
- como tokens ausentes e inesperados são tratados;
- quais pontos de sincronização devem ser usados;
- como nós de erro aparecem na AST;
- como evitar cascatas excessivas de diagnósticos;
- quais limites tornam a recuperação insegura;
- quais testes validam a recuperação no Stage 2.

---

## 2. Escopo

Este documento cobre:

- recuperação de erros sintáticos;
- sincronização por contexto;
- inserção sintética de tokens ausentes;
- descarte controlado de tokens inesperados;
- construção de `AstErrorNode`;
- continuidade de parsing após erro recuperável;
- diagnósticos de parser relacionados à recuperação;
- critérios para bloquear fases posteriores;
- testes de recuperação.

Este documento não cobre:

- erros léxicos;
- modelo completo de tokens;
- modelo completo de AST;
- algoritmo normal de parsing;
- resolução de nomes;
- checagem de tipos;
- recuperação semântica;
- renderização visual final de diagnósticos;
- políticas de IDE/LSP para edição incremental.

Esses temas pertencem a:

- `TOKEN-MODEL.md`;
- `AST-MODEL.md`;
- `PARSER-IMPLEMENTATION.md`;
- `DIAGNOSTIC-DATA-MODEL.md`;
- `DIAGNOSTIC-ARCHITECTURE.md`;
- `DIAGNOSTIC-STYLE-GUIDE.md`;
- `NAME-RESOLUTION.md`;
- `TYPE-CHECKING-PIPELINE.md`;
- `LSP-ARCHITECTURE.md`.

---

## 3. Princípios

A recuperação sintática deve seguir estes princípios:

- entrada malformada de usuário não deve causar panic;
- o primeiro diagnóstico deve apontar para a causa mais próxima do erro;
- recuperação existe para reportar erros adicionais, não para aceitar código inválido;
- AST parcial deve preservar estrutura suficiente para dumps e diagnósticos;
- nós de erro devem ser explícitos;
- a recuperação deve ser determinística;
- a recuperação deve evitar loops infinitos;
- a recuperação deve limitar diagnósticos em cascata;
- o parser não deve inventar semântica para reparar código;
- quando a recuperação for insegura, o parser deve parar de forma controlada.

---

## 4. Conceitos

| Conceito | Responsabilidade |
| --- | --- |
| `SyntaxExpectation` | Descreve o que o parser esperava em determinado ponto. |
| `RecoveryContext` | Contexto sintático atual usado para escolher sincronização. |
| `RecoverySet` | Conjunto de tokens onde o parser pode retomar. |
| `SyntheticToken` | Token ausente tratado como inserido conceitualmente. |
| `SkippedToken` | Token descartado para retomar parsing. |
| `AstErrorNode` | Nó inserido na AST para preservar posição estrutural de erro. |
| `RecoveryResult` | Resultado operacional de uma tentativa de recuperação. |

Os nomes finais podem variar na implementação. As responsabilidades são obrigatórias.

---

## 5. Categorias de Erros Recuperáveis

O parser deve tentar recuperar, quando seguro, das seguintes categorias:

- token obrigatório ausente;
- token inesperado;
- delimitador não fechado;
- separador ausente em lista;
- declaração incompleta;
- membro incompleto;
- tipo incompleto;
- expressão incompleta;
- comando incompleto;
- padrão incompleto;
- tokens extras após construção válida;
- EOF inesperado dentro de construção aberta.

Nem todo erro dessas categorias é recuperável em todo contexto. A recuperação deve considerar se existe ponto de sincronização plausível e se a AST parcial resultante ainda preserva estrutura útil.

---

## 6. Categorias Não Recuperáveis

O parser pode interromper a análise de forma controlada quando:

- a sequência de tokens não contém EOF;
- há erro interno de cursor ou span inválido;
- o número de diagnósticos excede limite configurado;
- a recuperação não avança o cursor após tentativa obrigatória;
- EOF ocorre dentro de estrutura tão incompleta que não há subárvore útil;
- a sequência contém grande região inválida sem ponto de sincronização;
- erro léxico bloqueador já tornou a tokenização inutilizável.

Interromper de forma controlada significa retornar AST parcial, diagnósticos acumulados e marcador de erro bloqueador quando possível. Não significa panic.

---

## 7. Interface de Recuperação

Interface conceitual:

```rust
fn recover(
    &mut self,
    context: RecoveryContext,
    expected: SyntaxExpectation,
) -> RecoveryResult;
```

Resultado conceitual:

```rust
pub enum RecoveryResult {
    InsertedSyntheticToken(SyntheticToken),
    SkippedTo(TokenKind),
    BuiltErrorNode(AstErrorNode),
    ReachedEof,
    Unrecoverable,
}
```

A implementação pode decompor essa interface em funções menores, como:

- `expect_or_insert`;
- `skip_until`;
- `recover_decl`;
- `recover_stmt`;
- `recover_expr`;
- `recover_delimited`.

O contrato obrigatório é que toda tentativa de recuperação seja explícita, avance ou pare, e emita diagnóstico estruturado quando o erro for de usuário.

---

## 8. Expectativas Sintáticas

`SyntaxExpectation` deve representar a expectativa de forma estruturada.

Contrato conceitual:

```rust
pub enum SyntaxExpectation {
    Token(TokenKind),
    OneOf(Vec<TokenKind>),
    Identifier,
    Path,
    Type,
    Expression,
    Statement,
    Declaration,
    Member,
    Pattern,
    Delimiter { open: TokenKind, close: TokenKind },
}
```

Regras:

- expectativa deve ser usada para diagnóstico e decisão de recuperação;
- mensagem humana não deve ser a única representação da expectativa;
- conjuntos grandes devem ser descritos por categoria, como `Expression`;
- expectativa não deve carregar informação semântica.

---

## 9. Contextos de Recuperação

`RecoveryContext` indica onde o parser está quando encontra o erro.

Contextos iniciais:

```rust
pub enum RecoveryContext {
    TopLevel,
    ModuleDecl,
    ImportDecl,
    Declaration,
    ClassBody,
    InterfaceBody,
    TraitBody,
    Member,
    FunctionSignature,
    ParamList,
    Type,
    Block,
    Statement,
    Expression,
    ArgumentList,
    GenericArgumentList,
    ArrayLiteral,
    MatchBody,
    SwitchBody,
    Pattern,
}
```

Regras:

- cada contexto deve possuir conjunto de sincronização próprio;
- contextos aninhados podem herdar tokens de sincronização de contextos externos;
- delimitadores de fechamento do contexto externo devem ser tratados como pontos seguros;
- EOF sempre sincroniza, mas normalmente encerra a recuperação.

---

## 10. Conjuntos de Sincronização

Conjuntos de sincronização são tokens nos quais o parser pode retomar com risco controlado.

### 10.1 Conjunto Global

```text
EOF
;
}
```

### 10.2 Top-Level

```text
module
import
function
class
interface
trait
const
let
@
public
protected
private
internal
abstract
sealed
final
static
unsafe
EOF
```

### 10.3 Corpo de Classe, Interface e Trait

```text
function
constructor
const
let
@
public
protected
private
internal
abstract
final
static
override
unsafe
}
EOF
```

### 10.4 Bloco e Comandos

```text
let
const
return
if
switch
match
while
for
foreach
break
continue
unsafe
{
}
;
EOF
```

### 10.5 Expressões

```text
;
,
)
]
}
:
case
default
EOF
```

### 10.6 Tipos

```text
,
)
]
>
{
;
=
EOF
```

### 10.7 Listas Delimitadas

Para listas delimitadas, o conjunto deve incluir:

- separador da lista;
- delimitador de fechamento;
- tokens de sincronização do contexto externo;
- EOF.

Exemplo para argumentos de chamada:

```text
,
)
;
}
EOF
```

---

## 11. Estratégias de Recuperação

A implementação deve usar as estratégias abaixo, nesta ordem preferencial quando aplicável.

### 11.1 Inserção Sintética

Usada quando um token obrigatório está ausente, mas o token atual indica que o parser já está no próximo elemento válido.

Exemplos:

- `;` ausente antes do início de nova declaração;
- `)` ausente antes de `{` em assinatura de função;
- `]` ausente antes de `;` em indexação ou array;
- `}` ausente antes de EOF.

Regras:

- inserir token sintético não consome token real;
- deve emitir diagnóstico de token ausente;
- span sintético deve ser vazio no ponto de inserção;
- o token sintético não deve aparecer como lexema real;
- dumps devem indicar ausência ou nó de erro quando relevante.

### 11.2 Descarte de Token Inesperado

Usado quando o token atual não pode iniciar nenhuma construção válida no contexto atual.

Regras:

- deve emitir diagnóstico de token inesperado;
- deve consumir pelo menos um token;
- pode agrupar sequência descartada em um único nó de erro;
- deve parar ao encontrar token do conjunto de sincronização;
- não deve descartar EOF.

### 11.3 Sincronização por Delimitador

Usada em listas, blocos e agrupamentos.

Regras:

- se o delimitador de fechamento aparece, o parser deve retomar após ou no fechamento conforme o contexto;
- delimitador de fechamento inesperado deve encerrar o contexto atual quando plausível;
- delimitador de abertura sem fechamento deve gerar diagnóstico no span da abertura ou EOF;
- delimitadores aninhados devem ser contados quando o descarte atravessar subestruturas.

### 11.4 Nó de Erro Estrutural

Usado quando uma posição obrigatória da AST não pode ser preenchida por nó válido.

Exemplos:

- expressão ausente em `let x = ;`;
- tipo ausente em `let x : = 1;`;
- declaração inválida dentro de classe;
- padrão inválido em `case`.

Regras:

- deve produzir `AstErrorNode`;
- deve emitir diagnóstico correspondente;
- deve preservar span da região problemática;
- deve permitir que o pai continue estruturalmente válido.

---

## 12. Algoritmo Geral

Algoritmo conceitual:

```text
ao encontrar erro sintático:
  1. emitir diagnóstico estruturado
  2. verificar se inserção sintética é segura
  3. se segura, retornar token/nó sintético
  4. se não, calcular RecoverySet do contexto
  5. descartar tokens até encontrar ponto de sincronização ou EOF
  6. se a posição exige nó, construir AstErrorNode
  7. retomar parsing no ponto encontrado
  8. se nenhum avanço ocorreu, consumir um token ou marcar unrecoverable
```

Invariante obrigatória:

```text
cada tentativa de recuperação deve avançar o parser,
produzir substituto sintético,
ou encerrar a análise de forma controlada.
```

---

## 13. Recuperação de Declarações

Erros em declaração devem sincronizar em:

```text
;
}
function
class
interface
trait
const
let
@
public
protected
private
internal
abstract
sealed
final
static
unsafe
EOF
```

Regras:

- declaração com palavra-chave reconhecida mas corpo incompleto deve tentar produzir declaração parcial;
- declaração sem palavra-chave reconhecida deve produzir `Decl::Error`;
- `}` encerra corpo externo e não deve ser consumido pela recuperação de declaração, salvo quando o contexto decidir;
- `;` pode ser consumido para finalizar declaração malformada;
- tokens até o próximo início de declaração podem ser agrupados em um único erro.

Exemplos:

```capi
function () {
}
```

Deve gerar diagnóstico de identificador ausente e produzir função parcial ou declaração de erro.

```capi
public sealed {
}
```

Deve gerar diagnóstico de declaração esperada após modificadores e sincronizar no bloco ou próximo início de declaração.

---

## 14. Recuperação de Membros

Erros em membro devem sincronizar em:

```text
;
}
function
constructor
const
let
@
public
protected
private
internal
abstract
final
static
override
unsafe
EOF
```

Regras:

- erro em membro não deve descartar `}` do corpo da classe/interface/trait;
- membro malformado deve virar `MemberDecl::Error`;
- se `function` ou `constructor` forem reconhecidos, o parser deve tentar preservar assinatura parcial;
- campo sem `;` pode inserir `;` sintético antes do próximo membro.

---

## 15. Recuperação de Assinaturas

Erros em assinatura de função ou construtor devem priorizar preservação do corpo.

Sincronização:

```text
)
{
;
}
EOF
```

Regras:

- parâmetro incompleto deve produzir `Param` com nó de erro ou ser omitido com diagnóstico;
- vírgula ausente entre parâmetros deve gerar diagnóstico se dois parâmetros adjacentes forem reconhecíveis;
- `)` ausente pode ser inserido antes de `{` ou `;`;
- tipo de retorno incompleto deve produzir `TypeSyntax::Error`;
- corpo `{ ... }` não deve ser descartado por erro na assinatura quando puder ser reconhecido.

Exemplo:

```capi
function f(a : Int32 b : String) {
}
```

Deve diagnosticar separador ausente entre parâmetros e preservar ambos os parâmetros quando possível.

---

## 16. Recuperação de Tipos

Erros em tipo devem sincronizar em:

```text
,
)
]
>
{
;
=
EOF
```

Regras:

- tipo ausente em posição obrigatória deve produzir `TypeSyntax::Error`;
- `>` ausente em tipo genérico pode ser inserido antes de `)`, `{`, `;` ou `=`;
- argumento genérico inválido deve ser substituído por erro sem descartar a lista inteira quando houver vírgula ou `>`;
- array com `]` ausente deve inserir fechamento antes de `;`, `,`, `)` ou `=`;
- o parser não deve tentar resolver se o tipo existe.

Exemplos:

```capi
let x : List< = value;
```

Deve produzir tipo genérico parcial com argumento de erro e diagnóstico.

```capi
function f() : {
}
```

Deve diagnosticar tipo de retorno ausente e preservar o corpo.

---

## 17. Recuperação de Comandos

Erros em comandos devem sincronizar em:

```text
;
}
let
const
return
if
switch
match
while
for
foreach
break
continue
unsafe
{
EOF
```

Regras:

- comando malformado deve produzir `Stmt::Error`;
- `;` encerra comando malformado e pode ser consumido;
- `}` encerra bloco externo e não deve ser consumido pelo comando malformado;
- comando simples sem `;` pode inserir `;` antes de novo comando ou `}`;
- erro dentro de comando composto deve preservar bloco ou subcomando quando possível.

Exemplos:

```capi
return
let x = 1;
```

Se `return` exige `;` ou expressão na mesma construção, deve diagnosticar fim de comando ausente e retomar em `let`.

```capi
if (x > 0 {
    return;
}
```

Deve diagnosticar `)` ausente antes de `{` e preservar o bloco.

---

## 18. Recuperação de Expressões

Erros em expressões devem sincronizar em:

```text
;
,
)
]
}
:
case
default
EOF
```

Regras:

- expressão ausente em posição obrigatória deve produzir `Expr::Error`;
- operador binário sem lado direito deve produzir expressão parcial com lado direito de erro;
- token que encerra contexto externo não deve ser consumido pela expressão;
- delimitador de agrupamento ausente pode ser inserido;
- parser de expressão deve interromper ao encontrar token com binding power menor que o mínimo atual;
- recuperação de expressão não deve atravessar `;` ou `}` salvo por contexto explícito.

Exemplos:

```capi
let x = ;
```

Deve produzir inicializador `Expr::Error` e continuar após `;`.

```capi
let x = a + ;
```

Deve produzir `BinaryExpr` com lado direito de erro ou `Expr::Error` cobrindo a operação incompleta.

```capi
foo(1, , 3);
```

Deve produzir argumento de erro entre vírgulas e preservar os demais argumentos.

---

## 19. Recuperação de Listas

Listas delimitadas devem ser recuperadas localmente para preservar o máximo de itens válidos.

Listas cobertas:

- parâmetros;
- argumentos;
- argumentos genéricos;
- elementos de array;
- elementos de tupla;
- membros de corpo;
- comandos de bloco;
- braços de `match`;
- casos de `switch`.

Regras:

- item inválido entre separadores deve gerar nó de erro local;
- separador ausente entre dois itens reconhecíveis deve gerar diagnóstico e continuar;
- delimitador de fechamento ausente deve ser inserido quando o token atual pertencer ao contexto externo;
- item vazio deve ser aceito somente quando a gramática permitir;
- sequência de vírgulas repetidas deve limitar diagnósticos.

Exemplo:

```capi
function f(a : Int32,, b : String) {
}
```

Deve diagnosticar item ausente ou vírgula extra e preservar `a` e `b`.

---

## 20. Recuperação de `switch` e `match`

### 20.1 `switch`

Sincronização em corpo de `switch`:

```text
case
default
}
EOF
```

Regras:

- `case` malformado deve gerar nó de erro para o caso;
- corpo de caso deve sincronizar no próximo `case`, `default` ou `}`;
- `default` duplicado pode ser deixado para validação posterior, salvo se a gramática tratar como erro sintático;
- `}` encerra o switch e não deve ser consumido por recuperação de caso.

### 20.2 `match`

Sincronização em corpo de `match`:

```text
case
}
EOF
```

Regras:

- padrão ausente deve produzir `Pattern::Error`;
- corpo ausente deve produzir erro no braço;
- erro em um braço deve sincronizar no próximo `case` ou `}`;
- exaustividade não pertence ao parser.

---

## 21. EOF Inesperado

EOF inesperado deve produzir diagnóstico específico.

Regras:

- EOF dentro de bloco deve diagnosticar `}` ausente;
- EOF dentro de lista deve diagnosticar delimitador de fechamento ausente;
- EOF após operador binário deve diagnosticar expressão ausente;
- EOF após modificadores deve diagnosticar declaração ausente;
- EOF deve encerrar a recuperação;
- o parser não deve tentar consumir além de EOF.

Spans de EOF devem ser spans vazios no fim da fonte.

---

## 22. Controle de Cascata

Recuperação ruim pode gerar diagnósticos derivados de um único erro. A implementação deve limitar cascatas.

Mecanismos obrigatórios:

- limite máximo de diagnósticos por arquivo;
- supressão de diagnósticos repetidos no mesmo span e categoria;
- modo de recuperação ativo para evitar múltiplos erros até sincronização;
- agrupamento de tokens descartados em um único diagnóstico quando apropriado;
- detecção de ausência de progresso.

Limite inicial recomendado:

```text
100 diagnósticos sintáticos por arquivo
```

O limite pode ser configurável futuramente. Ao atingir o limite, o parser deve emitir diagnóstico de excesso de erros e encerrar parsing de forma controlada.

---

## 23. Progresso e Loops Infinitos

Toda rotina de recuperação deve preservar progresso.

Invariante:

```text
antes de iniciar recuperação, registre cursor.index;
ao terminar, cursor.index deve ser maior,
ou um token sintético deve ter sido produzido,
ou a análise deve ser encerrada.
```

Se a recuperação retorna ao mesmo índice sem substituto sintético:

- consumir um token inesperado, se não for EOF;
- ou retornar `Unrecoverable`, se for EOF;
- emitir diagnóstico interno somente se a situação indicar bug do parser.

Esse controle deve ser testado com entradas adversariais.

---

## 24. AST Parcial e Nós de Erro

`AstErrorNode` deve ser usado quando um nó obrigatório não puder ser construído.

Posições permitidas:

- declaração;
- membro;
- parâmetro;
- tipo;
- comando;
- expressão;
- padrão;
- braço de `match` ou caso de `switch`, se modelado assim.

Contrato conceitual:

```rust
pub struct AstErrorNode {
    span: Span,
    expected: Vec<SyntaxExpectation>,
    found: Option<TokenKind>,
}
```

Regras:

- o nó de erro deve apontar para diagnóstico correspondente;
- o nó de erro não torna o programa válido;
- o dump da AST deve exibir o nó de erro explicitamente;
- lowering deve tratar AST com erro como entrada bloqueada ou parcialmente analisável conforme `AST-LOWERING.md`;
- nós de erro não devem carregar significado semântico.

---

## 25. Diagnósticos

Diagnósticos de recuperação devem usar a infraestrutura comum.

Categorias iniciais:

| Categoria | Exemplo |
| --- | --- |
| `ExpectedToken` | `)` ausente. |
| `ExpectedOneOf` | esperava `;` ou `}`. |
| `UnexpectedToken` | token não inicia declaração. |
| `UnclosedDelimiter` | `{` sem `}`. |
| `MissingSeparator` | vírgula ausente entre parâmetros. |
| `MissingExpression` | expressão ausente após `=`. |
| `MissingType` | tipo ausente após `:`. |
| `IncompleteDeclaration` | declaração termina antes do corpo. |
| `TooManySyntaxErrors` | limite de diagnósticos atingido. |

Regras:

- diagnóstico deve possuir span primário quando possível;
- quando houver abertura sem fechamento, o span primário pode ser o delimitador aberto e o EOF pode aparecer como label secundária;
- mensagens devem evitar afirmar semântica;
- sugestões podem ser usadas para tokens ausentes simples;
- diagnósticos devem ser estáveis para testes.

---

## 26. Interação com Tokens de Erro Léxico

Se o lexer produzir token `Error`, o parser deve tratá-lo como token inesperado no contexto sintático.

Regras:

- o parser não deve duplicar diagnóstico léxico já emitido;
- pode emitir diagnóstico sintático adicional somente quando o token de erro impede reconhecer a construção;
- recuperação deve consumir ou sincronizar a partir do token de erro;
- token de erro pode compor span de `AstErrorNode`.

Se o driver decidir não chamar o parser após erro léxico bloqueador, este documento não exige recuperação sintática.

---

## 27. Determinismo

Para a mesma sequência de tokens:

- a mesma recuperação deve ser escolhida;
- os mesmos tokens devem ser descartados;
- os mesmos nós de erro devem ser criados;
- diagnósticos devem ter mesma categoria e spans equivalentes;
- dump de AST parcial deve ser determinístico.

A recuperação não deve depender de:

- endereços de memória;
- ordem não determinística de coleções;
- paralelismo sem ordenação;
- locale do sistema;
- mensagens renderizadas.

---

## 28. Testes Obrigatórios

Os testes de recuperação devem cobrir:

- token inesperado em top-level;
- declaração sem nome;
- modificador sem declaração;
- função sem `)`;
- função sem corpo;
- parâmetro sem tipo;
- vírgula ausente entre parâmetros;
- vírgula duplicada em parâmetros;
- classe sem `}`;
- membro inválido dentro de classe;
- campo sem `;`;
- tipo genérico sem `>`;
- tipo ausente após `:`;
- bloco sem `}`;
- `if` sem `)`;
- `return` incompleto;
- `let x = ;`;
- operador binário sem lado direito;
- chamada com argumento ausente;
- array ou indexação sem `]`;
- `switch` com `case` malformado;
- `match` com padrão ausente;
- EOF inesperado em lista;
- sequência longa de tokens inválidos;
- limite de diagnósticos;
- ausência de loop infinito;
- dump de AST parcial com nós de erro.

Cada teste deve validar:

- diagnósticos emitidos;
- spans principais;
- ponto de retomada;
- presença ou ausência de `AstErrorNode`;
- continuidade após erro quando esperada.

---

## 29. Critérios de Aceite do Stage 2

Para este documento ser considerado implementado:

- erros sintáticos comuns produzem diagnósticos estruturados;
- parser continua após erros recuperáveis;
- recuperação nunca entra em loop infinito;
- tokens ausentes simples podem ser tratados por inserção sintética;
- tokens inesperados são descartados até ponto seguro;
- nós de erro aparecem na AST parcial;
- dumps exibem nós de erro de forma determinística;
- limite de diagnósticos impede cascatas excessivas;
- testes obrigatórios de recuperação passam.

---

## 30. Relações Normativas

Este documento depende diretamente de:

- Documento 15 — Parser e AST;
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial;
- `TOKEN-MODEL.md`;
- `AST-MODEL.md`;
- `PARSER-IMPLEMENTATION.md`;
- `SPANS-AND-LOCATIONS.md`;
- `DIAGNOSTIC-DATA-MODEL.md`;
- `DIAGNOSTIC-ARCHITECTURE.md`;
- `DIAGNOSTIC-STYLE-GUIDE.md`.

Este documento orienta diretamente:

- `PARSER-TESTS.md`;
- `AST-LOWERING.md`;
- implementação de recuperação no parser;
- dumps de AST parcial em `capic --emit ast`.
