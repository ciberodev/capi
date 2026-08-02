# Testing

Esta pasta reúne a documentação de engenharia sobre testes da implementação
oficial da Linguagem Capi.

Ela define como a suíte deve validar o workspace, a CLI `capic`, os diagnósticos,
as fases do compilador e a conformidade com a especificação. A partir do Stage
1, esta área também documenta os testes obrigatórios do lexer e dos componentes
de fonte usados pelo frontend inicial.

---

## Documentos ativos

| Documento | Status | Natureza | Finalidade |
| --- | --- | --- | --- |
| `TEST-STRATEGY.md` | Aprovado | Documento de engenharia bloqueante | Define a estratégia oficial de testes, comandos de validação, camadas de teste, suíte mínima do Stage 0, critérios de CI e evolução da suíte nos próximos stages. |
| `LEXER-TESTS.md` | Aprovado | Documento de testes do Stage 1 | Define cobertura obrigatória para `SourceMap`, spans, Unicode, tokenização, diagnósticos léxicos, entradas inválidas, snapshots e `capic --emit tokens`. |

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
| `PARSER-TESTS.md` | Detalhar cobertura sintática quando o parser for implementado. |
| `SEMANTIC-TESTS.md` | Detalhar cobertura de resolução, tipos e regras semânticas. |
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
```

Esses comandos são a base de validação usada para considerar o Stage 1
concluído localmente.

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
3. `../compiler/README.md`
4. `../compiler/source/SOURCE-MAP.md`
5. `../compiler/frontend/TOKEN-MODEL.md`
6. `../compiler/frontend/LEXER-IMPLEMENTATION.md`
7. `../build-and-ci/BUILD-SYSTEM.md`
8. `../planning/DEFINITION-OF-DONE.md`
9. `../architecture/COMPILATION-PIPELINE.md`
10. `../architecture/COMPILER-ARCHITECTURE.md`

Essa ordem parte da política geral, passa pela suíte léxica do Stage 1 e depois
conecta a validação ao compilador, ao build, aos critérios de aceite e à
arquitetura.

---

## Critério de atualização

Atualize este README quando:

* um documento reservado desta pasta for preenchido;
* uma nova categoria de teste se tornar obrigatória;
* os comandos canônicos de validação mudarem;
* a CI passar a executar novos grupos de teste;
* um stage mudar a suíte mínima exigida.
