# Testing

Esta pasta reúne a documentação de engenharia sobre testes da implementação
oficial da Linguagem Capi.

Ela define como a suíte deve validar o workspace, a CLI `capic`, os diagnósticos,
as fases futuras do compilador e a conformidade com a especificação. No Stage 0,
o conteúdo normativo preenchido está concentrado em `TEST-STRATEGY.md`.

---

## Documento aprovado no Stage 0

| Documento | Status | Natureza | Finalidade |
| --- | --- | --- | --- |
| `TEST-STRATEGY.md` | Aprovado | Documento de engenharia bloqueante | Define a estratégia oficial de testes, comandos de validação, camadas de teste, suíte mínima do Stage 0, critérios de CI e evolução da suíte nos próximos stages. |

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
| `LEXER-TESTS.md` | Detalhar cobertura de tokenização quando o lexer for implementado. |
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

Enquanto esses documentos estiverem vazios, eles não introduzem regras próprias.
As decisões aplicáveis vêm de `TEST-STRATEGY.md`, dos documentos de engenharia
aprovados, das ADRs e da especificação.

---

## Suíte mínima do Stage 0

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

---

## Comandos canônicos

Execute a partir de `capi-lang/`:

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked
cargo test --workspace --locked
cargo run -p capi-cli --locked -- --help
cargo run -p capi-cli --locked -- --version
cargo run -p capi-cli --locked -- does-not-exist.capi
scripts/ci-local.sh
```

Esses comandos são a base de validação usada para considerar o Stage 0 concluído.

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
../planning/DEFINITION-OF-DONE.md
../planning/FEATURE-STATUS.md
```

Nenhuma entrega de código deve ser considerada concluída se quebrar build,
formatação, lint, testes ou smoke tests definidos para o stage atual.

---

## Leitura recomendada

Para entender a estratégia de testes, leia nesta ordem:

1. `TEST-STRATEGY.md`
2. `../build-and-ci/BUILD-SYSTEM.md`
3. `../planning/DEFINITION-OF-DONE.md`
4. `../architecture/COMPILATION-PIPELINE.md`
5. `../architecture/COMPILER-ARCHITECTURE.md`

Essa ordem parte da política de testes e conecta a validação ao build, aos
critérios de aceite e à arquitetura do compilador.

---

## Critério de atualização

Atualize este README quando:

* um documento reservado desta pasta for preenchido;
* uma nova categoria de teste se tornar obrigatória;
* os comandos canônicos de validação mudarem;
* a CI passar a executar novos grupos de teste;
* um stage mudar a suíte mínima exigida.
