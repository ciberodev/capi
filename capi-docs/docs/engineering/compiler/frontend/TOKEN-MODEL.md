# Token Model

**Projeto:** Linguagem Capi  
**Documento:** TOKEN-MODEL  
**Status:** Aprovado  
**Stage:** Stage 1 — Fontes, diagnósticos e lexer  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o modelo de engenharia dos tokens produzidos pelo lexer da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- a estrutura conceitual de `Token`;
- o contrato de `TokenKind`;
- a relação entre token, lexema e span;
- as categorias léxicas iniciais;
- a representação de palavras-chave, identificadores, literais, operadores e delimitadores;
- o tratamento de comentários e espaços em branco;
- o token EOF;
- a interface esperada para parser, dumps e diagnósticos.

---

## 2. Escopo

Este documento cobre:

- tokens produzidos pelo lexer;
- categorias de tokens;
- lexemas preservados;
- valores léxicos normalizados quando aplicável;
- spans associados a tokens;
- palavras reservadas;
- identificadores;
- literais;
- operadores;
- delimitadores;
- comentários;
- espaços em branco;
- token inválido e recuperação;
- requisitos de teste para o modelo de tokens.

Este documento não cobre:

- algoritmo do lexer;
- gramática do parser;
- construção de AST;
- resolução de nomes;
- inferência de tipos;
- semântica de operadores;
- representação final de valores constantes;
- renderização final de diagnósticos.

Esses temas pertencem a:

- `LEXER-IMPLEMENTATION.md`;
- `PARSER-IMPLEMENTATION.md`;
- `AST-MODEL.md`;
- `NAME-RESOLUTION.md`;
- `TYPE-MODEL.md`;
- `DIAGNOSTIC-DATA-MODEL.md`;
- `DIAGNOSTIC-ARCHITECTURE.md`.

---

## 3. Princípios

O modelo de tokens deve seguir estes princípios:

- tokens são a interface formal entre lexer e parser;
- cada token possui categoria explícita;
- cada token possui span;
- lexemas devem ser recuperáveis a partir de `SourceMap` e `Span`;
- o lexer não interpreta semântica;
- o parser não acessa detalhes internos do lexer;
- comentários e espaços em branco não devem ser enviados ao parser por padrão;
- a sequência de tokens deve ser determinística;
- a sequência deve terminar com EOF;
- entradas inválidas devem produzir diagnóstico estruturado e, quando útil, token de erro.

---

## 4. Estrutura de `Token`

Contrato conceitual:

```rust
pub struct Token {
    kind: TokenKind,
    span: Span,
}
```

Campos adicionais podem existir quando houver necessidade real:

- valor literal normalizado;
- símbolo internado;
- flags de recuperação;
- metadados para dumps.

O token não deve armazenar cópia obrigatória do lexema. O lexema deve ser recuperável por `SourceMap::span_text` ou API equivalente.

---

## 5. Componentes Conceituais

| Componente | Responsabilidade |
| --- | --- |
| `kind` | Categoria léxica reconhecida. |
| `span` | Origem do token no código-fonte. |
| lexema | Texto original recuperado pelo span. |
| valor | Valor normalizado opcional para literais. |
| metadados | Informações auxiliares sem semântica de linguagem. |

O lexema preservado é sempre o texto escrito pelo usuário. Valor normalizado não substitui lexema.

---

## 6. `TokenKind`

`TokenKind` identifica a categoria do token.

Contrato conceitual:

```rust
pub enum TokenKind {
    Identifier,
    Keyword(Keyword),
    Literal(LiteralKind),
    Operator(Operator),
    Delimiter(Delimiter),
    Eof,
    Error,
}
```

A implementação pode escolher enum plano, enums aninhados ou representação compacta. O contrato obrigatório é que o parser consiga distinguir todas as categorias necessárias sem consultar texto cru para decisões básicas.

---

## 7. Lexema

O lexema é a sequência exata de texto reconhecida no código-fonte.

Regras:

- deve ser preservado por meio de span;
- não deve ser normalizado silenciosamente;
- deve incluir delimitadores em literais de string e char;
- deve incluir todos os caracteres de operadores compostos;
- deve respeitar offsets em bytes definidos por `SPANS-AND-LOCATIONS.md`;
- deve permanecer recuperável para dumps e diagnósticos.

Exemplos:

| Código | Categoria | Lexema |
| --- | --- | --- |
| `let` | keyword | `let` |
| `saldo` | identifier | `saldo` |
| `123` | integer literal | `123` |
| `"abc"` | string literal | `"abc"` |
| `>=` | operator | `>=` |
| `{` | delimiter | `{` |

---

## 8. Span

Todo token produzido pelo lexer deve possuir span.

Regras:

- tokens reais usam span real;
- EOF usa span vazio no fim da fonte;
- token de erro deve cobrir a região problemática quando possível;
- comentários descartados ainda devem atualizar a posição dos tokens seguintes;
- spans devem usar intervalo half-open `[start, end)`;
- spans devem apontar para fronteiras válidas de UTF-8 quando o lexema for recuperável.

Tokens não devem armazenar linha e coluna como autoridade primária. Linha e coluna são derivadas pelo `SourceMap`.

---

## 9. Palavras-Chave

Palavras-chave são identificadores reservados pela linguagem.

O lexer deve reconhecer uma sequência compatível com identificador e, antes de emitir `Identifier`, verificar se o lexema corresponde a palavra reservada.

Lista inicial conforme Documento 04:

```text
abstract
break
case
class
const
constructor
continue
default
else
extends
false
final
for
function
if
implements
import
interface
let
match
module
new
override
private
protected
public
return
sealed
static
switch
trait
true
unsafe
uses
while
```

Contrato conceitual:

```rust
pub enum Keyword {
    Abstract,
    Break,
    Case,
    Class,
    Const,
    Constructor,
    Continue,
    Default,
    Else,
    Extends,
    False,
    Final,
    For,
    Function,
    If,
    Implements,
    Import,
    Interface,
    Let,
    Match,
    Module,
    New,
    Override,
    Private,
    Protected,
    Public,
    Return,
    Sealed,
    Static,
    Switch,
    Trait,
    True,
    Unsafe,
    Uses,
    While,
}
```

`true` e `false` são palavras reservadas. A implementação pode classificá-las como `Keyword(True/False)` ou `Literal(Bool)`, desde que o parser receba uma categoria inequívoca e o contrato seja aplicado de forma consistente.

---

## 10. Identificadores

Identificadores representam nomes definidos pelo usuário.

Regras iniciais conforme Documento 04:

- podem conter letras;
- podem conter dígitos;
- podem conter `_`;
- não podem iniciar por dígito;
- não podem ser palavra reservada;
- são case-sensitive;
- devem preservar o texto original.

O lexer não deve:

- resolver o identificador;
- verificar declaração prévia;
- atribuir símbolo;
- inferir tipo;
- normalizar Unicode;
- aplicar regra semântica de visibilidade.

Interning de identificadores é permitido como otimização, mas não deve alterar o contrato observável do token.

---

## 11. Literais

Literais representam valores escritos diretamente no código-fonte.

Categorias iniciais:

```rust
pub enum LiteralKind {
    Integer,
    Float,
    Char,
    String,
    Bool,
}
```

Regras:

- o lexema original deve ser preservado;
- o span deve cobrir o literal inteiro;
- valores normalizados são opcionais no token;
- erros de literal devem produzir diagnóstico estruturado;
- literal inválido não deve causar panic;
- a linguagem não possui literal `null` ou `nil`.

`Bool` pode ser modelado como literal ou keyword especial. A decisão deve ser consistente com o parser.

---

## 12. Valores Normalizados

Alguns literais podem carregar valor normalizado.

Exemplos:

- inteiro convertido para representação numérica;
- float convertido ou preparado para parsing posterior;
- string com escapes interpretados;
- char como valor Unicode escalar;
- booleano como `true` ou `false`.

No Stage 1, é aceitável que o token carregue apenas `LiteralKind` e span, deixando a conversão completa para fase posterior ou helper do lexer.

Quando houver valor normalizado, ele não deve substituir:

- lexema original;
- span;
- diagnóstico sobre formato inválido.

---

## 13. Operadores

Operadores são tokens reconhecidos por forma textual.

Contrato conceitual:

```rust
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AmpAmp,
    PipePipe,
    EqualEqualEqual,
    PlusPlus,
    MinusMinus,
    Arrow,
}
```

Lista inicial derivada da sintaxe e exemplos:

| Lexema | Categoria |
| --- | --- |
| `+` | soma ou operador unário conforme parser |
| `-` | subtração ou operador unário conforme parser |
| `*` | multiplicação ou wildcard/import glob conforme parser |
| `/` | divisão ou início de comentário conforme lexer |
| `%` | resto |
| `=` | atribuição ou inicialização |
| `==` | igualdade lógica |
| `===` | identidade |
| `!=` | diferença |
| `<` | menor ou delimitador de generic conforme parser |
| `<=` | menor ou igual |
| `>` | maior ou delimitador de generic conforme parser |
| `>=` | maior ou igual |
| `&&` | and lógico |
| `||` | or lógico |
| `!` | negação lógica |
| `++` | incremento, se mantido pela sintaxe |
| `--` | decremento, se mantido pela sintaxe |
| `->` | seta reservada para evolução ou sintaxe futura |

O lexer deve aplicar maximal munch: quando mais de um operador compartilha prefixo, a maior sequência válida deve ser emitida.

O lexer não decide precedência, associatividade, aridade ou validade semântica do operador.

---

## 14. Delimitadores

Delimitadores organizam a estrutura do programa.

Contrato conceitual:

```rust
pub enum Delimiter {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Semicolon,
    Colon,
    Question,
    At,
}
```

Lista inicial:

| Lexema | Uso típico |
| --- | --- |
| `(` | chamadas, agrupamento, parâmetros |
| `)` | fechamento correspondente |
| `{` | bloco |
| `}` | fechamento de bloco |
| `[` | reservado para indexação ou tipos futuros |
| `]` | fechamento correspondente |
| `,` | separação |
| `.` | acesso a membro ou módulo |
| `;` | fim de declaração |
| `:` | anotação de tipo, case |
| `?` | ternário ou optional conforme parser |
| `@` | anotações e diretivas |

O parser decide o papel sintático de delimitadores ambíguos.

---

## 15. Comentários

Comentários são reconhecidos pelo lexer, mas não são enviados ao parser por padrão.

Formas iniciais:

| Forma | Tratamento |
| --- | --- |
| `// ...` | comentário de linha até quebra de linha ou EOF. |
| `/* ... */` | comentário de bloco. |

Regras:

- comentários devem atualizar posição corretamente;
- comentário de bloco não terminado deve produzir diagnóstico;
- comentários não devem gerar tokens do parser por padrão;
- ferramentas futuras podem solicitar preservação de comentários em modo específico;
- comentário não deve alterar semântica do programa.

Se comentários aninhados forem introduzidos ou rejeitados formalmente, a decisão deve ser registrada em `LEXER-IMPLEMENTATION.md`.

---

## 16. Espaços em Branco

Espaços em branco são consumidos pelo lexer e não são enviados ao parser por padrão.

Incluem inicialmente:

- espaço ASCII;
- tab;
- quebras de linha reconhecidas por `UNICODE-AND-ENCODING.md`;
- outros caracteres de espaço somente quando aprovados pela política léxica.

Espaços em branco devem:

- separar tokens quando necessário;
- preservar cálculo correto de spans;
- alimentar a tabela de linhas por meio do source map;
- não produzir tokens salvo modo especial de debug ou ferramenta.

---

## 17. EOF

A sequência de tokens deve terminar obrigatoriamente com EOF.

Regras:

- EOF possui `TokenKind::Eof`;
- EOF deve ter span vazio;
- o offset do EOF deve ser o fim da fonte lexada;
- EOF deve ser produzido mesmo para arquivo vazio;
- EOF permite ao parser detectar encerramento da entrada de forma inequívoca.

---

## 18. Token de Erro

O lexer deve produzir diagnósticos para entradas léxicas inválidas.

A implementação pode também produzir `TokenKind::Error` para permitir recuperação.

Uso esperado:

- caractere inesperado;
- literal não terminado;
- escape inválido;
- comentário de bloco não terminado;
- sequência ambígua sem token válido.

Regras:

- erro deve possuir span útil;
- erro não deve esconder diagnóstico;
- recuperação deve avançar para ponto seguro;
- parser não deve tratar token de erro como construção válida.

---

## 19. Sequência de Tokens

A saída do lexer deve ser sequência ordenada de tokens.

Contrato conceitual:

```rust
pub struct TokenStream {
    tokens: Vec<Token>,
}
```

ou interface equivalente por iterador.

Regras:

- ordem deve seguir a ordem do texto fonte;
- spans de tokens reais não devem retroceder;
- comentários e espaços em branco descartados não aparecem na sequência;
- EOF aparece exatamente uma vez;
- diagnósticos léxicos podem acompanhar a sequência;
- a mesma entrada deve produzir a mesma sequência.

---

## 20. Relação com Parser

O parser deve consumir somente a interface pública de tokens.

O parser pode consultar:

- `TokenKind`;
- `Span`;
- lexema via `SourceMap`, quando necessário;
- valor literal normalizado, se existir.

O parser não deve:

- acessar estado interno do lexer;
- reler o arquivo para classificar tokens;
- reinterpretar comentários descartados;
- decidir se uma palavra reservada é identificador.

---

## 21. Relação com Diagnósticos

Tokens fornecem spans para diagnósticos sintáticos e posteriores.

Regras:

- diagnósticos léxicos devem apontar span produzido durante reconhecimento;
- diagnósticos sintáticos devem apontar tokens recebidos;
- token EOF pode ser usado para erro de fim inesperado;
- token de erro deve preservar a região problemática;
- dumps de tokens devem incluir kind, span e lexema recuperado.

---

## 22. API Pública Inicial

API conceitual mínima:

```rust
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub enum TokenKind {
    Identifier,
    Keyword(Keyword),
    Literal(LiteralKind),
    Operator(Operator),
    Delimiter(Delimiter),
    Eof,
    Error,
}
```

Helpers esperados:

```rust
impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self;
}

impl TokenKind {
    pub fn is_keyword(&self) -> bool;
    pub fn is_literal(&self) -> bool;
    pub fn is_operator(&self) -> bool;
    pub fn is_delimiter(&self) -> bool;
}
```

Os nomes finais podem variar. O contrato obrigatório é a capacidade de representar e consultar as categorias descritas.

---

## 23. Dump de Tokens

O Stage 1 exige um resultado demonstrável:

```bash
capic --emit tokens arquivo.capi
```

O dump de tokens deve ser determinístico e conter, no mínimo:

- ordem do token;
- kind;
- lexema, quando houver;
- source id ou caminho apresentável;
- linha e coluna inicial;
- linha e coluna final ou span em offsets;
- indicação de EOF.

O formato textual exato deve ser definido em `LEXER-IMPLEMENTATION.md` ou documento de dumps, mas deve ser estável o suficiente para testes.

---

## 24. Limitações do Stage 1

No Stage 1, não é obrigatório implementar:

- tokenização completa de toda sintaxe futura;
- comentários de documentação preservados;
- interning obrigatório de identificadores;
- valor normalizado completo para todos os literais;
- operadores ainda não confirmados pela especificação;
- modo LSP incremental;
- token stream lazy;
- compactação final de memória.

Essas limitações não devem impedir o reconhecimento dos tokens do subconjunto inicial nem a produção de diagnósticos estruturados para entrada inválida.

---

## 25. Testes Obrigatórios

Os testes do modelo de tokens devem cobrir:

- token de identificador;
- distinção case-sensitive de identificadores;
- reconhecimento de cada palavra-chave inicial;
- `true` e `false` conforme decisão de modelagem;
- inteiros;
- floats;
- chars;
- strings;
- operadores aritméticos;
- operadores relacionais;
- operadores lógicos;
- operador de identidade `===`;
- maximal munch para operadores compostos;
- delimitadores;
- anotações com `@`;
- comentários de linha descartados;
- comentários de bloco descartados;
- espaços em branco descartados;
- EOF em arquivo vazio;
- EOF após último token;
- spans corretos em tokens;
- lexema recuperável via span;
- token inválido ou diagnóstico para caractere inesperado;
- literal não terminado;
- dump determinístico de tokens.

Testes de Unicode, linha e coluna devem ser compartilhados com `UNICODE-AND-ENCODING.md` e `SPANS-AND-LOCATIONS.md`.

---

## 26. Critérios de Aprovação

Este documento pode ser considerado aprovado quando:

- a estrutura conceitual de `Token` estiver aceita;
- o conjunto inicial de `TokenKind` estiver definido;
- a lista inicial de keywords estiver alinhada ao Documento 04;
- operadores e delimitadores do subconjunto inicial estiverem definidos;
- a relação com spans, Unicode, lexer, parser e diagnósticos estiver clara;
- as limitações do Stage 1 estiverem explícitas;
- os testes obrigatórios forem rastreáveis às entregas do Documento 28.

Após aprovação, a implementação do lexer deve produzir tokens compatíveis com este contrato antes de avançar para o parser.
