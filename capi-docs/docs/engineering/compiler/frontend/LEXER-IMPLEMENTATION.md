# Lexer Implementation

**Projeto:** Linguagem Capi  
**Documento:** LEXER-IMPLEMENTATION  
**Status:** Aprovado  
**Stage:** Stage 1 — Fontes, diagnósticos e lexer  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o contrato de engenharia para a implementação do lexer da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- entradas e saídas do lexer;
- organização do scanner;
- ordem de reconhecimento léxico;
- integração com `SourceMap`, spans, Unicode e diagnósticos;
- regras operacionais para identificadores, palavras-chave, literais, operadores, delimitadores, comentários e espaços em branco;
- estratégia inicial de recuperação;
- dump de tokens exigido pelo Stage 1;
- testes mínimos para validar a implementação.

---

## 2. Escopo

Este documento cobre:

- implementação inicial do lexer;
- contrato do crate ou módulo de lexer;
- cursor de leitura;
- produção de tokens;
- emissão de diagnósticos léxicos;
- maximal munch;
- EOF;
- comentários e whitespace;
- integração com o driver para `--emit tokens`;
- limitações do Stage 1.

Este documento não cobre:

- modelo conceitual completo de tokens;
- estrutura final do parser;
- gramática sintática;
- AST;
- resolução de nomes;
- checagem de tipos;
- semântica de operadores;
- renderização final de diagnósticos;
- infraestrutura completa de UI tests.

Esses temas pertencem a:

- `TOKEN-MODEL.md`;
- `PARSER-IMPLEMENTATION.md`;
- `AST-MODEL.md`;
- `DIAGNOSTIC-DATA-MODEL.md`;
- `DIAGNOSTIC-ARCHITECTURE.md`;
- `DIAGNOSTIC-STYLE-GUIDE.md`;
- `LEXER-TESTS.md`.

---

## 3. Princípios

A implementação do lexer deve seguir estes princípios:

- responsabilidade única: transformar texto em tokens;
- independência do parser, AST, HIR, MIR e backends;
- determinismo para a mesma entrada;
- ausência de panic para entrada de usuário malformada;
- diagnósticos estruturados para erros léxicos;
- spans precisos em todos os tokens;
- preservação do lexema original via `SourceMap`;
- tratamento de Unicode conforme `UNICODE-AND-ENCODING.md`;
- simplicidade de implementação no Stage 1.

---

## 4. Papel no Pipeline

Fluxo conceitual:

```text
SourceMap + SourceFile
    ↓
Lexer
    ↓
TokenStream + diagnósticos léxicos
    ↓
Parser ou dump de tokens
```

O lexer deve receber fonte já carregada e validada como UTF-8. Ele não deve abrir arquivos, resolver módulos ou criar `SourceId`.

---

## 5. Entradas

Entrada conceitual do lexer:

```rust
pub struct LexInput<'a> {
    pub source_id: SourceId,
    pub text: &'a str,
}
```

O lexer pode receber `&SourceFile`, `SourceId` + `&str` ou uma view equivalente, desde que preserve:

- identidade da fonte;
- acesso ao texto;
- capacidade de criar spans;
- integração com coletor de diagnósticos.

O texto recebido deve obedecer à política de UTF-8 definida em `UNICODE-AND-ENCODING.md`.

---

## 6. Saídas

Saída conceitual:

```rust
pub struct LexOutput {
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}
```

Ou interface equivalente, por exemplo:

- iterador de tokens com coletor de diagnósticos externo;
- `Result<TokenStream, Diagnostics>` quando erro bloqueador impedir sequência útil;
- stream incremental, se mantiver o contrato observável.

No Stage 1, a forma preferida é produzir uma sequência completa de tokens e diagnósticos associados, por simplicidade de teste e dump.

---

## 7. Organização do Crate

O Stage 1 deve criar um componente dedicado ao lexer, preferencialmente um crate como:

```text
capi-lexer
```

Responsabilidades do componente:

- expor API pública de lexing;
- definir ou reutilizar `Token` e `TokenKind`;
- implementar cursor de leitura;
- produzir spans;
- emitir diagnósticos léxicos;
- fornecer dump de tokens ou dados suficientes para o driver emitir dump.

Dependências permitidas:

- `capi-source`;
- `capi-diagnostics`;
- `capi-common`, se necessário;
- biblioteca padrão Rust.

O lexer não deve depender de parser, AST, HIR, MIR, backend ou driver.

---

## 8. Cursor de Leitura

O lexer deve manter um cursor sobre o texto.

Estado conceitual:

```rust
struct Lexer<'a> {
    source: SourceId,
    text: &'a str,
    offset: ByteOffset,
    tokens: Vec<Token>,
    diagnostics: Vec<Diagnostic>,
}
```

O cursor deve:

- avançar em fronteiras válidas de UTF-8;
- preservar offset inicial de cada token;
- permitir lookahead de caracteres;
- detectar fim de arquivo;
- nunca indexar string em byte arbitrário sem validação.

Linha e coluna não precisam ser atualizadas diretamente pelo lexer se o `SourceMap` resolver offsets posteriormente.

---

## 9. Ordem de Reconhecimento

Para cada posição da entrada, o lexer deve aplicar uma ordem determinística de reconhecimento.

Ordem inicial:

```text
1. BOM inicial, apenas no offset 0
2. whitespace
3. comentários
4. identificadores e palavras-chave
5. literais numéricos
6. literais de string
7. literais de char
8. operadores compostos
9. operadores simples
10. delimitadores
11. caractere inválido
12. EOF
```

Essa ordem pode ser reorganizada internamente se o resultado observável permanecer idêntico.

---

## 10. Maximal Munch

O lexer deve reconhecer a maior sequência válida quando tokens compartilham prefixo.

Exemplos:

| Entrada | Token esperado |
| --- | --- |
| `===` | `EqualEqualEqual` |
| `==` | `EqualEqual` |
| `>=` | `GreaterEqual` |
| `&&` | `AmpAmp` |
| `//` | início de comentário de linha |
| `/*` | início de comentário de bloco |

Maximal munch não autoriza o lexer a interpretar sintaxe. Ele apenas decide a maior forma léxica válida.

---

## 11. Whitespace

Whitespace deve ser consumido e descartado da sequência enviada ao parser.

Inclui inicialmente:

- espaço ASCII;
- tab;
- quebras de linha reconhecidas por `UNICODE-AND-ENCODING.md`;
- outros espaços somente quando aprovados pela política léxica.

Regras:

- whitespace não gera token por padrão;
- offsets devem avançar corretamente;
- quebras de linha devem permanecer resolvíveis pelo `SourceMap`;
- modo de debug futuro pode preservar whitespace se houver necessidade de ferramenta.

---

## 12. Comentários

Comentários devem ser reconhecidos e descartados por padrão.

Formas iniciais:

```text
// comentário de linha
/* comentário de bloco */
```

Regras:

- comentário de linha termina antes da quebra de linha ou no EOF;
- comentário de bloco termina em `*/`;
- comentário de bloco não terminado produz diagnóstico;
- comentários podem conter Unicode válido;
- comentários devem preservar posições corretas dos tokens seguintes;
- comentários não são enviados ao parser no modo normal.

No Stage 1, comentários de bloco aninhados não devem ser aceitos como aninhamento especial, salvo decisão posterior em especificação. Uma ocorrência de `/*` dentro de comentário de bloco é conteúdo do comentário.

---

## 13. Identificadores

O lexer deve reconhecer identificadores conforme o subconjunto aprovado da sintaxe.

Regra inicial:

```text
identifier_start = letter | "_"
identifier_continue = letter | digit | "_"
```

Detalhes de Unicode para letras e dígitos devem seguir `UNICODE-AND-ENCODING.md` e o contrato final do lexer.

No Stage 1, é aceitável iniciar com identificadores ASCII se essa limitação estiver explícita nos testes e diagnósticos. Antes da conclusão formal do Stage 1, a política de Unicode permitida em identificadores deve estar documentada e testada.

Depois de reconhecer um identificador, o lexer deve consultar a tabela de palavras-chave. Se houver correspondência exata, deve emitir keyword.

Identificadores são case-sensitive.

---

## 14. Palavras-Chave

O lexer deve reconhecer a lista definida em `TOKEN-MODEL.md`.

Regras:

- comparação deve ser exata;
- comparação não deve normalizar Unicode;
- keyword só é reconhecida quando o lexema completo corresponde;
- prefixos de keywords continuam identificadores.

Exemplos:

| Entrada | Token |
| --- | --- |
| `let` | keyword `Let` |
| `letter` | identifier |
| `class` | keyword `Class` |
| `Class` | identifier |

---

## 15. Literais Numéricos

O lexer deve reconhecer inteiros e floats do subconjunto inicial.

Regras iniciais:

- sequência de dígitos decimais produz literal inteiro;
- sequência de dígitos seguida de `.` e pelo menos um dígito produz literal float;
- sinal `+` ou `-` não faz parte do literal; é operador separado;
- formato inválido deve produzir diagnóstico;
- o lexema original deve ser preservado.

Exemplos:

| Entrada | Tokens |
| --- | --- |
| `123` | integer literal |
| `3.14` | float literal |
| `-1` | `Minus`, integer literal |
| `1.` | diagnóstico ou `Integer`, `Dot`, conforme decisão do lexer/parser documentada |

No Stage 1, bases numéricas alternativas, separadores como `_` e expoentes podem ficar fora do subconjunto se ainda não estiverem definidos.

---

## 16. Literais de String

Strings começam e terminam com aspas duplas.

Regras:

- span cobre delimitadores;
- lexema preserva delimitadores e escapes;
- conteúdo deve ser UTF-8 válido;
- quebra de linha não escapada dentro de string deve seguir política definida pelo lexer;
- string não terminada produz diagnóstico;
- EOF dentro de string produz diagnóstico e recuperação.

Escapes mínimos a definir no Stage 1:

```text
\\
\"
\n
\r
\t
```

Escapes Unicode ou adicionais devem ser documentados antes de aceitos.

---

## 17. Literais de Char

Chars começam e terminam com aspas simples.

Regras:

- span cobre delimitadores;
- lexema preserva delimitadores e escapes;
- conteúdo deve representar exatamente um valor Unicode escalar após escape, quando escapes forem processados;
- char vazio é erro;
- char com múltiplos caracteres é erro;
- char não terminado produz diagnóstico.

Validação completa do valor pode ocorrer no lexer ou em fase posterior, mas erro lexical evidente deve ser diagnosticado no Stage 1.

---

## 18. Operadores

O lexer deve reconhecer os operadores definidos em `TOKEN-MODEL.md`.

Regras:

- operadores compostos são reconhecidos antes dos simples;
- `//` e `/*` iniciam comentários, não operador `/` seguido de delimitador;
- `<` e `>` são tokens únicos; o parser decide se participam de generics ou comparação;
- `===` deve ser reconhecido como operador único;
- `==` não deve ser produzido como parte inicial de `===`.

O lexer não decide precedência, associatividade ou validade semântica.

---

## 19. Delimitadores

O lexer deve reconhecer delimitadores definidos em `TOKEN-MODEL.md`.

Regras:

- cada delimitador simples produz um token;
- `@` produz delimitador ou token específico de anotação conforme o modelo final;
- `.` deve ser tokenizado de forma que `import banco.*;` e acesso a membro sejam possíveis;
- o parser decide significado sintático.

---

## 20. Caracteres Inválidos

Quando nenhum token válido puder iniciar na posição atual, o lexer deve emitir diagnóstico léxico.

Regras:

- o span deve cobrir o caractere inválido completo;
- a recuperação deve avançar pelo menos um caractere Unicode;
- um `TokenKind::Error` pode ser produzido;
- a análise deve continuar quando houver ponto seguro.

UTF-8 inválido deve ser detectado antes do lexer receber o texto. O lexer trata apenas Unicode válido mas lexicalmente inválido.

---

## 21. EOF

Ao fim da entrada, o lexer deve emitir exatamente um token EOF.

Regras:

- EOF tem span vazio;
- offset do EOF é o tamanho em bytes da fonte;
- EOF é produzido para arquivo vazio;
- EOF é produzido mesmo após erros recuperáveis;
- EOF não possui lexema real.

---

## 22. Diagnósticos Léxicos

O lexer deve emitir diagnósticos estruturados para:

- caractere inválido;
- identificador malformado, se aplicável;
- literal numérico inválido;
- string não terminada;
- char não terminado;
- char vazio;
- char com múltiplos caracteres;
- escape inválido;
- comentário de bloco não terminado.

Cada diagnóstico deve possuir:

- severidade;
- código ou categoria;
- mensagem;
- span primário;
- nota ou sugestão quando aplicável.

A forma final de código, severidade e renderização pertence aos documentos de diagnósticos.

---

## 23. Recuperação

O lexer deve recuperar de erros quando isso preservar consistência.

Estratégias iniciais:

- caractere inválido: emitir diagnóstico e avançar um caractere;
- string não terminada: consumir até quebra de linha ou EOF, conforme política definida;
- char inválido: consumir até próxima aspa simples, quebra de linha ou EOF;
- comentário de bloco não terminado: consumir até EOF e emitir diagnóstico;
- número inválido: consumir a sequência problemática e emitir diagnóstico.

Após recuperação, os tokens seguintes devem possuir spans corretos.

---

## 24. Token Stream

O lexer deve produzir sequência ordenada.

Invariantes:

- tokens reais aparecem em ordem crescente de offset;
- spans não devem se sobrepor de forma incoerente;
- whitespace e comentários descartados não aparecem no stream normal;
- EOF aparece no final;
- token de erro, se produzido, aparece na posição do erro;
- diagnostics podem coexistir com tokens recuperados.

---

## 25. Interface Pública Inicial

API conceitual:

```rust
pub fn lex(source_id: SourceId, text: &str) -> LexOutput;
```

Tipos conceituais:

```rust
pub struct LexOutput {
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}
```

Alternativa aceitável:

```rust
pub struct Lexer<'a> { ... }

impl<'a> Lexer<'a> {
    pub fn new(source_id: SourceId, text: &'a str) -> Self;
    pub fn lex(self) -> LexOutput;
}
```

Os nomes finais podem variar. O contrato obrigatório é que o driver consiga executar o lexer e obter tokens e diagnósticos estruturados.

---

## 26. Integração com `SourceMap`

O lexer deve receber uma fonte registrada no `SourceMap`.

Integração esperada:

- `SourceMap` carrega arquivo;
- driver obtém `SourceFile`;
- lexer recebe `SourceId` e `&str`;
- lexer produz tokens com spans;
- diagnósticos usam spans;
- dump recupera lexemas e localizações via `SourceMap`.

O lexer não deve possuir seu próprio mapa de arquivos.

---

## 27. Integração com o Driver

Para o Stage 1, o driver deve permitir o resultado demonstrável:

```bash
capic --emit tokens arquivo.capi
```

Fluxo esperado:

```text
1. Ler argumentos.
2. Carregar arquivo no SourceMap.
3. Executar lexer.
4. Emitir diagnósticos, se houver.
5. Emitir dump de tokens, se solicitado.
6. Retornar código de saída apropriado.
```

Se houver erro de carregamento ou UTF-8 inválido, o lexer não precisa executar.

---

## 28. Dump de Tokens

O dump de tokens deve ser determinístico.

Formato mínimo conceitual:

```text
0  Keyword(Let)      main.cap:1:1..1:4   "let"
1  Identifier        main.cap:1:5..1:10  "valor"
2  Operator(Equal)   main.cap:1:11..1:12 "="
3  Literal(Integer)  main.cap:1:13..1:15 "10"
4  Delimiter(Semicolon) main.cap:1:15..1:16 ";"
5  Eof              main.cap:1:16..1:16
```

O formato final pode mudar, mas deve conter:

- índice;
- kind;
- localização;
- lexema quando aplicável;
- EOF.

Snapshots ou UI tests devem estabilizar o formato aprovado.

---

## 29. Erros Internos

Erro interno do lexer deve ser distinto de erro de usuário.

Exemplos de erro interno:

- span construído com `start > end` por bug;
- cursor avança para offset inválido;
- token emitido sem span;
- EOF ausente por violação de invariante.

Esses casos podem usar mecanismos de internal compiler error definidos pelos documentos de diagnósticos e desenvolvimento.

Entrada malformada do usuário não deve ser tratada como erro interno.

---

## 30. Performance

O lexer do Stage 1 deve priorizar simplicidade, mas manter complexidade linear.

Diretrizes:

- percorrer o texto uma vez, com lookahead limitado;
- evitar alocação de lexema para cada token;
- recuperar lexemas por span;
- usar tabela estática para keywords;
- reconhecer operadores compostos sem backtracking excessivo;
- preservar O(n) para tamanho da fonte.

O Stage 1 não exige lexer gerado, DFA formal ou otimizações avançadas.

---

## 31. Limitações do Stage 1

No Stage 1, não é obrigatório implementar:

- token stream lazy;
- modo incremental para LSP;
- preservação de comentários de documentação;
- todas as extensões futuras de literais;
- normalização Unicode;
- suporte nativo a encodings não UTF-8;
- lexer gerado automaticamente;
- recuperação sofisticada para todos os erros.

Essas limitações não devem impedir reconhecimento do subconjunto inicial, spans precisos, diagnósticos estruturados e dump de tokens.

---

## 32. Testes Obrigatórios

Os testes de lexer devem cobrir:

- arquivo vazio produz EOF;
- identificador simples;
- distinção case-sensitive;
- cada keyword inicial;
- keywords como prefixo de identificador;
- inteiros;
- floats;
- strings válidas;
- string não terminada;
- char válido;
- char vazio;
- char não terminado;
- operadores simples;
- operadores compostos;
- maximal munch com `===`, `==`, `>=`, `&&`, `||`;
- delimitadores;
- anotação com `@`;
- comentário de linha;
- comentário de bloco;
- comentário de bloco não terminado;
- whitespace descartado;
- quebras de linha preservando localização;
- Unicode válido em string e comentário;
- caractere lexicalmente inválido;
- spans de todos os tokens;
- EOF no offset final;
- recuperação após erro;
- dump determinístico de tokens.

Testes de posição dos diagnósticos devem validar arquivo, linha, coluna e span.

---

## 33. Critérios de Aprovação

Este documento pode ser considerado aprovado quando:

- a entrada e saída do lexer estiverem definidas;
- a ordem de reconhecimento estiver aceita;
- a estratégia de maximal munch estiver clara;
- comentários, whitespace, EOF e erros estiverem definidos;
- a integração com `SourceMap`, `Token`, diagnósticos e driver estiver definida;
- o dump `capic --emit tokens` estiver especificado em nível suficiente;
- as limitações do Stage 1 estiverem explícitas;
- os testes obrigatórios forem rastreáveis às entregas do Documento 28.

Após aprovação, a implementação pode criar o componente de lexer e conectá-lo ao driver para produzir o resultado demonstrável do Stage 1.
