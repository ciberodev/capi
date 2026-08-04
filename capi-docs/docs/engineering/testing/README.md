# Testing

Esta pasta reúne a documentação de engenharia sobre testes da implementação
oficial da Linguagem Capi.

Ela define como a suíte deve validar o workspace, a CLI `capic`, os diagnósticos,
as fases do compilador e a conformidade com a especificação. A partir do Stage
1, esta área documenta os testes obrigatórios do lexer e dos componentes de
fonte usados pelo frontend inicial. A partir do Stage 2, também documenta os
testes obrigatórios do parser, da AST, da recuperação sintática e do dump de
AST. A partir do Stage 3, documenta os testes de lowering AST-HIR, HIR, escopos,
símbolos, resolução de nomes, diagnósticos semânticos e `capic --emit hir`.

---

## Documentos ativos

| Documento | Status | Natureza | Finalidade |
| --- | --- | --- | --- |
| `TEST-STRATEGY.md` | Aprovado | Documento de engenharia bloqueante | Define a estratégia oficial de testes, comandos de validação, camadas de teste, suíte mínima do Stage 0, critérios de CI e evolução da suíte nos próximos stages. |
| `LEXER-TESTS.md` | Aprovado | Documento de testes do Stage 1 | Define cobertura obrigatória para `SourceMap`, spans, Unicode, tokenização, diagnósticos léxicos, entradas inválidas, snapshots e `capic --emit tokens`. |
| `PARSER-TESTS.md` | Aprovado | Documento de testes do Stage 2 | Define cobertura obrigatória para parser, AST, precedência, tipos, classes, diagnósticos sintáticos, recuperação, spans, snapshots e `capic --emit ast`. |
| `SEMANTIC-TESTS.md` | Aprovado | Documento de testes do Stage 3 | Define cobertura obrigatória para lowering, HIR, escopos, símbolos, resolução de nomes, diagnósticos semânticos, snapshots e `capic --emit hir`. |

---

## Documentos reservados

| Documento | Finalidade esperada |
| --- | --- |
| `UNIT-TESTS.md` | Detalhar política de testes unitários por crate e módulo. |
| `INTEGRATION-TESTS.md` | Definir testes de integração entre crates e contratos públicos internos. |
| `UI-TESTS.md` | Definir testes de saída textual, diagnósticos e comportamento observável de CLI. |
| `COMPILE-PASS-TESTS.md` | Definir programas Capi que devem ser aceitos pelo compilador. |
| `COMPILE-FAIL-TESTS.md` | Definir programas Capi que devem ser rejeitados com erro esperado. |
| `RUN-PASS-TESTS.md` | Definir programas Capi que devem compilar, executar e produzir resultado esperado. |
| `OWNERSHIP-TESTS.md` | Detalhar testes de ownership, borrowing, lifetime e modelos relacionados quando forem implementados. |
| `MIR-TESTS.md` | Definir validação de MIR, passes e invariantes intermediários. |
| `CODEGEN-TESTS.md` | Definir testes de geração de código e integração futura com backend. |
| `DOMAIN-TESTS.md` | Definir testes ligados aos domínios explícitos da linguagem. |
| `CONFORMANCE-SUITE.md` | Consolidar a futura suíte de conformidade da linguagem Capi. |
| `DIFFERENTIAL-TESTS.md` | Definir testes diferenciais quando houver implementações ou modos comparáveis. |
| `FUZZING.md` | Definir estratégia de fuzzing para parser, diagnósticos, IR e fases críticas. |
| `PERFORMANCE-TESTS.md` | Definir testes de performance, benchmarks e limites aceitáveis. |
| `TEST-DATA-POLICY.md` | Definir organização, versionamento e estabilidade dos dados de teste. |
| `TEST-ORGANIZATION.md` | Consolidar estrutura física da suíte quando ela crescer além dos testes iniciais. |

Enquanto esses documentos estiverem vazios ou reservados, eles não introduzem
regras próprias. As decisões aplicáveis vêm de `TEST-STRATEGY.md`, dos
documentos de engenharia aprovados, das ADRs e da especificação.

---

## Suíte mínima por stage

### Stage 0

No Stage 0, a suíte é intencionalmente pequena. Ela valida que a fundação do
workspace está pronta antes da implementação de funcionalidades reais da
linguagem.

O mínimo demonstrável inclui:

* build completo do workspace;
* formatação;
* lint;
* testes do workspace;
* `capic --help`;
* `capic --version`;
* erro controlado para arquivo inexistente;
* validação local equivalente à CI.

### Stage 1

No Stage 1, a suíte passa a validar o frontend léxico inicial e os crates de
infraestrutura usados por ele.

O mínimo demonstrável inclui:

* testes de `SourceMap`, `SourceFile`, `SourceId`, `Span`, linha e coluna;
* testes de leitura de arquivo UTF-8 válido;
* teste de rejeição de UTF-8 inválido sem panic;
* testes de Unicode e limites de byte/coluna;
* testes de identificadores, keywords, literais, operadores e delimitadores;
* testes de comentários de linha e bloco;
* testes de tokens inválidos;
* testes de posição de diagnósticos;
* testes de diagnósticos estruturados para entradas inválidas;
* testes de não-pânico para entradas malformadas;
* fixtures léxicos em `capi-lang/tests/lexer/`;
* snapshot de dump de tokens;
* teste de CLI para `capic --emit tokens arquivo.capi`.

### Stage 2

No Stage 2, a suíte passa a validar o frontend sintático inicial, a AST e a
recuperação de erros do parser.

O mínimo demonstrável inclui:

* testes de declarações de topo;
* testes de expressões;
* testes de precedência e associatividade de operadores;
* testes de tipos sintáticos;
* testes de classes, membros, construtores e métodos;
* testes de erros sintáticos com diagnósticos estruturados `PARSE`;
* testes de recuperação após erros recuperáveis;
* testes de AST e preservação de spans;
* testes de nós de erro na AST parcial;
* fixtures de dump determinístico da AST;
* snapshots golden para o dump de AST;
* teste de CLI para `capic --emit ast arquivo.capi`.

### Stage 3

No Stage 3, a suíte passa a validar HIR, lowering e a primeira análise
semântica.

O mínimo demonstrável inclui:

* testes de lowering em `capi-lowering`;
* testes de IDs HIR determinísticos;
* testes de preservação de `SourceId`, spans e `AstToHirMap`;
* testes de HIR inicial e dump determinístico;
* testes de `ScopeGraph` para o subconjunto inicial;
* testes de `SymbolTable`, `SymbolId`, namespaces e duplicidade;
* testes de resolução de nomes para valores, tipos, módulos/imports e patterns;
* testes de referências inexistentes;
* testes de ambiguidades;
* testes de diagnósticos semânticos estruturados `SEM`;
* fixtures semânticas em `capi-lang/tests/semantic/`;
* snapshots de HIR inicial e HIR resolvida;
* teste de CLI para `capic --emit hir arquivo.capi`.

---

## Comandos canônicos de validação

Execute a partir de `capi-lang/`:

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets
cargo run -p capi-cli -- --help
cargo run -p capi-cli -- --version
cargo run -p capi-cli -- --emit tokens tests/lexer/pass/basic.cap
cargo run -p capi-cli --bin capic -- --emit ast crates/capi-parser/tests/fixtures/ast_dump/basic.cap
cargo run -p capi-cli -- --emit hir tests/semantic/pass/basic.cap
```

Esses comandos são a base de validação usada para considerar os Stages 1, 2 e 3
concluídos localmente.

A validação consolidada do workspace é:

```bash
./scripts/check.sh
```

Quando a validação precisar reproduzir a CI com lockfile estrito, use a variante
`--locked` dos comandos Cargo e o script local de CI, se disponível:

```bash
cargo build --workspace --locked
cargo clippy --workspace --all-targets --locked
cargo test --workspace --locked
scripts/ci-local.sh
```

---

## Organização dos testes léxicos

Os testes do lexer usam três camadas:

| Camada | Local | Finalidade |
| --- | --- | --- |
| Unitários | `capi-lang/crates/capi-source/src/lib.rs` e `capi-lang/crates/capi-lexer/src/lib.rs` | Validar contratos internos pequenos e invariantes de fonte, span e tokenização. |
| Integração | `capi-lang/crates/capi-lexer/tests/lexer_tests.rs` | Validar fixtures, snapshots, Unicode, diagnósticos, compile-fail léxico e não-pânico. |
| CLI/driver | `capi-lang/crates/capi-cli/tests/` e `capi-lang/crates/capi-driver/src/lib.rs` | Validar comportamento observável de `capic --emit tokens`. |

Os dados de teste ficam em:

```text
capi-lang/tests/lexer/pass/
capi-lang/tests/lexer/fail/
capi-lang/tests/lexer/snapshots/
```

Fixtures `pass/` não devem produzir diagnósticos. Fixtures `fail/` devem
produzir diagnósticos estruturados, com código, severidade e span primário.

---

## Organização dos testes sintáticos

Os testes do parser e da AST usam quatro camadas:

| Camada | Local | Finalidade |
| --- | --- | --- |
| Unitários | `capi-lang/crates/capi-parser/src/lib.rs` | Validar regras locais de parsing, precedência e recuperação. |
| Integração | `capi-lang/crates/capi-parser/tests/parser_tests.rs` | Validar `SourceMap -> Lexer -> Parser -> AST -> Diagnostics`, estrutura da AST, spans e diagnósticos sintáticos. |
| Snapshot/golden | `capi-lang/crates/capi-parser/tests/fixtures/ast_dump/` | Validar dump determinístico da AST byte a byte. |
| CLI/driver | `capi-lang/crates/capi-cli/tests/` e `capi-lang/crates/capi-driver/src/lib.rs` | Validar comportamento observável de `capic --emit ast`. |

Os dados de teste do dump de AST ficam em:

```text
capi-lang/crates/capi-parser/tests/fixtures/ast_dump/
```

Fixtures `.cap` definem a entrada. Fixtures `.ast` definem a saída esperada do
dump determinístico. Mudanças nesses arquivos devem representar mudança
intencional no contrato textual da AST.

---

## Organização dos testes semânticos

Os testes de HIR, lowering e análise semântica inicial usam cinco camadas:

| Camada | Local | Finalidade |
| --- | --- | --- |
| Lowering | `capi-lang/crates/capi-lowering/tests/lowering_tests.rs` | Validar `SourceMap -> Lexer -> Parser -> AST -> HIR`, IDs HIR, spans, `AstToHirMap` e bloqueio por AST inválida. |
| Integração semântica | `capi-lang/crates/capi-sema/tests/semantic_tests.rs` | Validar escopos, símbolos, resolução, diagnósticos e dumps resolvidos. |
| Fixtures pass/fail | `capi-lang/tests/semantic/pass/` e `capi-lang/tests/semantic/fail/` | Validar programas aceitos e rejeitados pelo subconjunto semântico inicial. |
| Snapshots | `capi-lang/tests/semantic/snapshots/` | Validar saída determinística de HIR inicial e HIR resolvida. |
| CLI/driver | `capi-lang/crates/capi-cli/tests/` e `capi-lang/crates/capi-driver/src/lib.rs` | Validar comportamento observável de `capic --emit hir`. |

Os dados de teste ficam em:

```text
capi-lang/tests/semantic/pass/
capi-lang/tests/semantic/fail/
capi-lang/tests/semantic/snapshots/
```

Fixtures `pass/` não devem produzir diagnósticos semânticos. Fixtures `fail/`
devem produzir diagnósticos estruturados, com código, severidade e span
primário.

Testes semânticos devem consumir HIR por `capi-hir` e executar o lowering por
`capi-lowering`. A HIR não deve depender diretamente da estrutura da AST após o
lowering.

---

## Evolução da suíte

A suíte deve crescer junto com as fases do compilador.

Ordem esperada de amadurecimento:

1. testes unitários e de CLI;
2. testes de fontes, sessão e diagnósticos;
3. testes de lexer;
4. testes de parser;
5. testes semânticos;
6. testes de MIR;
7. testes de geração de código;
8. testes `compile-pass`, `compile-fail` e `run-pass`;
9. suíte de conformidade;
10. fuzzing, diferenciais e performance quando houver base suficiente.

Cada nova regra da linguagem implementada deve ter teste correspondente. Quando
a regra ainda não estiver normativamente definida, a especificação deve ser
ajustada antes de o teste ser tratado como fonte de verdade.

---

## Relação com build e CI

A estratégia de testes é executada pelo sistema de build e pela CI.

Documentos relacionados:

```text
../build-and-ci/BUILD-SYSTEM.md
../build-and-ci/README.md
../compiler/README.md
../compiler/frontend/LEXER-IMPLEMENTATION.md
../compiler/frontend/TOKEN-MODEL.md
../compiler/frontend/AST-MODEL.md
../compiler/frontend/PARSER-IMPLEMENTATION.md
../compiler/frontend/PARSER-RECOVERY.md
../compiler/frontend/AST-LOWERING.md
../compiler/semantic/HIR-MODEL.md
../compiler/semantic/SCOPE-MODEL.md
../compiler/semantic/SYMBOL-MODEL.md
../compiler/semantic/NAME-RESOLUTION.md
../compiler/source/SOURCE-MAP.md
../compiler/source/SPANS-AND-LOCATIONS.md
../planning/DEFINITION-OF-DONE.md
../planning/FEATURE-STATUS.md
```

Nenhuma entrega de código deve ser considerada concluída se quebrar build,
formatação, lint, testes ou smoke tests definidos para o stage atual.

---

## Leitura recomendada

Para entender a estratégia de testes, leia nesta ordem:

1. `TEST-STRATEGY.md`
2. `LEXER-TESTS.md`
3. `PARSER-TESTS.md`
4. `../compiler/README.md`
5. `../compiler/source/SOURCE-MAP.md`
6. `../compiler/frontend/TOKEN-MODEL.md`
7. `../compiler/frontend/LEXER-IMPLEMENTATION.md`
8. `../compiler/frontend/AST-MODEL.md`
9. `../compiler/frontend/PARSER-IMPLEMENTATION.md`
10. `../compiler/frontend/PARSER-RECOVERY.md`
11. `../compiler/frontend/AST-LOWERING.md`
12. `SEMANTIC-TESTS.md`
13. `../compiler/semantic/HIR-MODEL.md`
14. `../compiler/semantic/SCOPE-MODEL.md`
15. `../compiler/semantic/SYMBOL-MODEL.md`
16. `../compiler/semantic/NAME-RESOLUTION.md`
17. `../build-and-ci/BUILD-SYSTEM.md`
18. `../planning/DEFINITION-OF-DONE.md`
19. `../architecture/COMPILATION-PIPELINE.md`
20. `../architecture/COMPILER-ARCHITECTURE.md`

Essa ordem parte da política geral, passa pelas suítes léxica e sintática dos
Stages 1 e 2, entra na suíte semântica do Stage 3 e depois conecta a validação
ao compilador, ao build, aos critérios de aceite e à arquitetura.

---

## Critério de atualização

Atualize este README quando:

* um documento reservado desta pasta for preenchido;
* uma nova categoria de teste se tornar obrigatória;
* os comandos canônicos de validação mudarem;
* a CI passar a executar novos grupos de teste;
* um stage mudar a suíte mínima exigida.
