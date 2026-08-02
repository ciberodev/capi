# Lexer Tests

**Projeto:** Linguagem Capi  
**Documento:** LEXER-TESTS  
**Status:** Aprovado  
**Stage:** Stage 1 — Fontes, diagnósticos e lexer  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define a estratégia de testes para o lexer da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- quais comportamentos léxicos devem ser testados;
- onde os testes devem viver;
- quais camadas de teste são obrigatórias;
- como validar tokens, spans, Unicode e diagnósticos;
- como testar o resultado demonstrável `capic --emit tokens`;
- quais critérios precisam ser atendidos para concluir o Stage 1.

---

## 2. Escopo

Este documento cobre:

- testes unitários do lexer;
- testes de integração entre source, lexer e diagnostics;
- testes de dump de tokens;
- testes negativos de erros léxicos;
- testes de spans e localizações em tokens;
- testes de Unicode relevantes ao lexer;
- testes de comentários e whitespace;
- testes de recuperação;
- snapshots ou UI tests para saída textual.

Este documento não cobre:

- testes completos do parser;
- AST;
- resolução de nomes;
- checagem de tipos;
- MIR;
- codegen;
- runtime;
- testes de performance completos;
- conformidade final da linguagem.

Esses temas pertencem aos documentos específicos de cada fase.

---

## 3. Princípios

Os testes do lexer devem seguir:

- determinismo;
- fixtures pequenas e legíveis;
- uma causa principal por teste;
- cobertura de casos válidos e inválidos;
- validação explícita de spans;
- validação de diagnósticos estruturados;
- ausência de dependência de caminhos absolutos;
- preservação de regressões corrigidas.

Um teste de lexer deve falhar de forma local: ao quebrar, deve ficar claro se o problema é tokenização, span, Unicode, diagnóstico ou dump.

---

## 4. Camadas de Teste

O Stage 1 deve usar quatro camadas principais.

| Camada | Objetivo |
| --- | --- |
| Unitários | Validar regras locais de tokenização. |
| Integração | Validar `SourceMap -> Lexer -> Diagnostics`. |
| Snapshot/UI | Validar saída textual de dumps e diagnósticos. |
| CLI | Validar `capic --emit tokens arquivo.capi`. |

Testes unitários devem cobrir a maior parte dos casos. Snapshots devem ser usados quando a saída textual for o contrato observado.

---

## 5. Organização Esperada

Organização sugerida no workspace:

```text
capi-lang/
├── crates/
│   ├── capi-lexer/
│   │   ├── src/
│   │   └── tests/
│   ├── capi-source/
│   └── capi-diagnostics/
└── tests/
    └── lexer/
        ├── pass/
        ├── fail/
        └── snapshots/
```

Enquanto a estrutura final não existir, os testes podem viver junto ao crate que expõe a API pública do lexer.

Casos compartilhados ou fixtures devem ter nomes descritivos e estáveis.

---

## 6. Convenções de Nome

Nomes de testes devem indicar comportamento.

Preferir:

```text
lexes_identifier
lexes_each_keyword
rejects_unterminated_string
preserves_span_for_multibyte_string
emits_eof_for_empty_file
```

Evitar:

```text
test1
lexer_works
bad_input
misc_tokens
```

Fixtures devem usar nomes como:

```text
valid_keywords.cap
invalid_unterminated_string.cap
unicode_string.cap
comments_block_unterminated.cap
```

---

## 7. Testes Unitários Obrigatórios

Testes unitários do lexer devem cobrir:

- arquivo vazio produz EOF;
- identificador simples;
- identificador com `_`;
- identificador com dígitos após o primeiro caractere;
- identificador não pode iniciar por dígito;
- distinção case-sensitive;
- keyword isolada;
- keyword como prefixo de identificador;
- todas as keywords iniciais;
- `true` e `false` conforme decisão do token model;
- inteiro decimal;
- float decimal;
- sinal `-` separado de número;
- string válida;
- char válido;
- operadores simples;
- operadores compostos;
- maximal munch;
- delimitadores;
- anotação com `@`;
- whitespace descartado;
- comentário de linha descartado;
- comentário de bloco descartado;
- EOF no offset final.

Cada teste deve validar `TokenKind` e, quando aplicável, lexema recuperado por span.

---

## 8. Testes de Keywords

A lista inicial de keywords deve ter cobertura explícita:

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

Requisitos:

- cada palavra deve produzir a categoria esperada;
- versões com maiúscula inicial devem ser identificadores, salvo regra futura contrária;
- prefixos e sufixos devem ser identificadores.

Exemplos:

```text
let      -> Keyword(Let)
letter   -> Identifier
Class    -> Identifier
class    -> Keyword(Class)
```

---

## 9. Testes de Operadores

Devem cobrir operadores aritméticos:

```text
+ - * / %
```

Operadores relacionais:

```text
== != < <= > >=
```

Operadores lógicos:

```text
&& || !
```

Operador de identidade:

```text
===
```

Casos de maximal munch:

```text
===  -> EqualEqualEqual
==   -> EqualEqual
>=   -> GreaterEqual
<=   -> LessEqual
&&   -> AmpAmp
||   -> PipePipe
```

Também deve haver teste garantindo que `//` e `/*` iniciam comentários, não sequência de tokens de operador.

---

## 10. Testes de Delimitadores

Devem cobrir:

```text
( ) { } [ ] , . ; : ? @
```

Requisitos:

- cada delimitador produz token correto;
- spans cobrem exatamente um caractere ASCII;
- `@` funciona para anotações;
- `.` funciona para acesso e imports;
- `<` e `>` permanecem operadores, não delimitadores especiais de generic no lexer.

---

## 11. Testes de Literais

### 11.1 Inteiros

Casos obrigatórios:

- `0`;
- inteiro com múltiplos dígitos;
- inteiro seguido de delimitador;
- inteiro seguido de whitespace;
- `-1` como operador `Minus` mais inteiro.

### 11.2 Floats

Casos obrigatórios:

- `0.0`;
- `3.14`;
- float seguido de delimitador;
- `1.` conforme decisão documentada em `LEXER-IMPLEMENTATION.md`.

### 11.3 Strings

Casos obrigatórios:

- string vazia;
- string ASCII;
- string com Unicode;
- string com escapes mínimos;
- string não terminada;
- escape inválido.

### 11.4 Chars

Casos obrigatórios:

- char ASCII;
- char Unicode;
- char com escape;
- char vazio;
- char com múltiplos caracteres;
- char não terminado.

---

## 12. Testes de Comentários

Devem cobrir:

- comentário de linha até `\n`;
- comentário de linha até EOF;
- comentário de bloco em uma linha;
- comentário de bloco com múltiplas linhas;
- comentário de bloco contendo Unicode;
- comentário de bloco não terminado;
- `/*` dentro de comentário de bloco tratado como conteúdo no Stage 1;
- tokens antes e depois de comentários.

Comentários descartados não devem aparecer no token stream normal.

---

## 13. Testes de Whitespace e Quebras de Linha

Devem cobrir:

- espaço;
- tab;
- múltiplos espaços;
- `\n`;
- `\r\n`;
- `\r`;
- whitespace antes do primeiro token;
- whitespace após o último token;
- EOF após quebra de linha final.

Requisitos:

- whitespace não aparece no stream normal;
- spans dos tokens seguintes permanecem corretos;
- linha e coluna em diagnósticos permanecem determinísticas.

---

## 14. Testes de Unicode

Devem cobrir:

- Unicode em string;
- Unicode em char;
- Unicode em comentário;
- caractere multibyte antes de token seguinte;
- offset em bytes distinto de coluna textual;
- rejeição de slice em fronteira UTF-8 inválida, quando aplicável;
- BOM inicial sem token real;
- `U+FEFF` fora do início conforme regra léxica normal.

Testes de identificação Unicode em nomes devem seguir a política final aprovada em `UNICODE-AND-ENCODING.md` e `LEXER-IMPLEMENTATION.md`.

---

## 15. Testes de Spans

Todo grupo de tokens deve ter ao menos um teste validando spans.

Casos obrigatórios:

- primeiro token começa em offset `0`;
- token após whitespace tem offset correto;
- token após comentário tem offset correto;
- token multibyte tem `end - start` em bytes correto;
- EOF tem span vazio no fim da fonte;
- erro léxico tem span primário correto;
- token em segunda linha resolve linha e coluna corretamente.

Validações devem usar `SourceMap` e APIs de localização, não cálculo manual duplicado no teste quando houver helper oficial.

---

## 16. Testes de Diagnósticos Léxicos

Devem cobrir:

- caractere inválido;
- literal numérico inválido;
- string não terminada;
- char vazio;
- char com múltiplos caracteres;
- char não terminado;
- escape inválido;
- comentário de bloco não terminado;
- erro após recuperação com tokens posteriores;
- múltiplos erros no mesmo arquivo quando recuperável.

Cada teste deve validar:

- severidade;
- código ou categoria, quando implementado;
- mensagem principal;
- span primário;
- linha e coluna;
- ausência de panic.

---

## 17. Testes de Recuperação

Recuperação deve ser testada separadamente.

Casos obrigatórios:

```capi
let x = @;
let y = 1;
```

Deve produzir diagnóstico para `@` inválido se `@` não for válido naquele contexto léxico, ou token de anotação se o contexto for léxico válido. A decisão do lexer deve ser consistente com `TOKEN-MODEL.md`.

Casos mais diretos:

```capi
let x = "abc
let y = 1;
```

Requisitos:

- erro é emitido;
- lexer avança para ponto seguro;
- tokens posteriores têm spans corretos quando a política permitir recuperação;
- EOF ainda é emitido.

---

## 18. Testes de Dump de Tokens

O comando demonstrável do Stage 1 é:

```bash
capic --emit tokens arquivo.capi
```

Testes devem cobrir:

- dump de arquivo vazio;
- dump de declaração simples;
- dump com keywords;
- dump com literais;
- dump com operadores compostos;
- dump com Unicode;
- dump com comentários descartados;
- dump após erro recuperável, se o modo permitir;
- determinismo do formato.

O dump deve ser validado por snapshot ou comparação textual estável.

Snapshots devem normalizar:

- caminhos;
- separadores de diretório;
- cor ANSI;
- mensagens dependentes do sistema operacional.

---

## 19. Testes de CLI

Quando `capic --emit tokens` existir, testes de CLI devem validar:

- comando com arquivo válido;
- comando com arquivo inexistente;
- comando com arquivo UTF-8 inválido;
- código de saída em sucesso;
- código de saída em erro léxico;
- stderr para diagnósticos;
- stdout ou arquivo de saída para dump, conforme decisão da CLI.

Testes de CLI devem ficar separados dos testes unitários do lexer.

---

## 20. Testes Compile-Fail Léxicos

Casos `compile-fail` léxicos devem existir para entradas rejeitadas antes do parser.

Categorias:

- UTF-8 inválido;
- caractere inválido;
- string não terminada;
- char inválido;
- escape inválido;
- comentário de bloco não terminado.

Cada fixture deve declarar o erro principal esperado.

Quando a infraestrutura de annotations em arquivos de teste existir, usar comentários próprios para indicar o erro esperado. Até lá, snapshots podem cumprir esse papel.

---

## 21. Fixtures

Fixtures devem ser pequenas.

Preferir um arquivo por comportamento quando a saída for snapshot.

Exemplo:

```text
tests/lexer/pass/keywords.cap
tests/lexer/pass/operators.cap
tests/lexer/fail/unterminated-string.cap
tests/lexer/fail/invalid-character.cap
```

Fixtures não devem depender de imports, módulos externos ou biblioteca padrão, salvo quando o objetivo do teste exigir.

---

## 22. Snapshots

Snapshots devem ser usados para:

- dump de tokens;
- diagnósticos renderizados;
- saída de CLI.

Regras:

- sem cor;
- caminhos normalizados;
- ordem determinística;
- atualização de snapshot deve ser revisão consciente;
- snapshot não substitui teste unitário quando um assert simples for mais claro.

---

## 23. Testes de Regressão

Todo bug corrigido no lexer deve ganhar teste de regressão.

O nome do teste ou fixture deve indicar o comportamento corrigido.

Quando houver issue ou código de regressão, pode ser usado no nome:

```text
regression_lexer_does_not_split_identity_operator
regression_unterminated_block_comment_keeps_eof_span
```

Regressões não devem ser removidas enquanto a regra permanecer válida.

---

## 24. Comandos de Validação

Comandos mínimos:

```bash
cargo test --workspace
cargo test -p capi-lexer
```

Quando a CLI suportar o comando:

```bash
cargo run -p capi-cli -- --emit tokens path/to/file.cap
```

Se houver script local de CI:

```bash
./scripts/ci-local.sh
```

O comando final oficial deve seguir `TEST-STRATEGY.md` e a organização real do workspace.

---

## 25. Critérios de Aceite do Stage 1

Os testes de lexer são aceitos quando:

- todos os tokens do subconjunto inicial possuem cobertura;
- todos os diagnósticos léxicos obrigatórios possuem cobertura;
- spans são validados;
- Unicode básico é validado;
- comentários e whitespace são validados;
- EOF é validado;
- recuperação é validada quando implementada;
- dump de tokens é determinístico;
- CLI `--emit tokens` possui teste quando existir;
- nenhum teste depende de caminho absoluto local;
- `cargo test --workspace` passa.

---

## 26. Limitações do Stage 1

No Stage 1, não é obrigatório testar:

- parser completo;
- AST;
- semântica de operadores;
- nomes e escopos;
- tipos;
- ownership;
- MIR;
- backend;
- performance em arquivos grandes;
- LSP incremental.

Essas limitações não reduzem a exigência de precisão para tokens, spans e diagnósticos léxicos do subconjunto implementado.

---

## 27. Critérios de Aprovação

Este documento pode ser considerado aprovado quando:

- a organização dos testes do lexer estiver definida;
- categorias válidas e inválidas estiverem cobertas;
- spans, Unicode e diagnósticos estiverem incluídos;
- dump de tokens e CLI estiverem contemplados;
- regras de snapshots estiverem explícitas;
- limitações do Stage 1 estiverem documentadas;
- os testes obrigatórios forem rastreáveis às entregas do Documento 28.

Após aprovação, a implementação do lexer deve criar testes compatíveis com este plano antes da conclusão formal do Stage 1.
