# Development

Esta pasta reúne a documentação operacional e de estilo para desenvolver a
implementação oficial da Linguagem Capi.

Ela cobre o ambiente local, a construção a partir do código-fonte, padrões gerais
de código e convenções Rust. As regras aqui complementam a especificação, as
ADRs aprovadas e os documentos de arquitetura.

---

## Documentos aprovados no Stage 0

| Documento | Status | Natureza | Finalidade |
| --- | --- | --- | --- |
| `DEVELOPMENT-SETUP.md` | Aprovado | Documento de engenharia bloqueante | Define o ambiente mínimo de desenvolvimento, ferramentas obrigatórias, estrutura esperada do repositório, scripts locais e validação do Stage 0. |
| `BUILDING-FROM-SOURCE.md` | Aprovado | Documento operacional | Descreve como construir, testar, validar e executar a implementação oficial a partir do workspace `capi-lang`. |
| `CODING-STANDARDS.md` | Aprovado | Documento de consolidação | Define padrões gerais de código, organização, erros, diagnósticos, testes, dependências e revisão. |
| `RUST-STYLE-GUIDE.md` | Aprovado | Documento de consolidação | Define as convenções Rust do projeto: formatação, Clippy, módulos, visibilidade, tipos, `Result`, testes, documentação e uso de `unsafe`. |

---

## Documentos reservados

| Documento | Finalidade esperada |
| --- | --- |
| `API-DESIGN-GUIDELINES.md` | Consolidar critérios para desenho de APIs internas e públicas quando as superfícies do compilador amadurecerem. |
| `COMMIT-GUIDELINES.md` | Definir convenções de commits, granularidade e mensagens quando o fluxo de contribuição for formalizado. |
| `DEBUGGING-GUIDE.md` | Registrar procedimentos de depuração para compilador, testes, diagnósticos e pipeline. |
| `DEPENDENCY-POLICY.md` | Espelhar ou detalhar a política operacional de dependências a partir de `DEPENDENCY-RULES.md` e `capi-lang/DEPENDENCIES.md`. |
| `DOCUMENTATION-GUIDELINES.md` | Definir padrão de escrita para documentação técnica, documentação Rust e exemplos. |
| `ERROR-HANDLING-GUIDELINES.md` | Detalhar a política de erros internos, diagnósticos recuperáveis e falhas controladas. |
| `ISSUE-TRIAGE.md` | Definir classificação, priorização e tratamento de issues. |
| `REVIEW-GUIDELINES.md` | Definir critérios formais de revisão de código e documentação. |
| `TROUBLESHOOTING.md` | Consolidar problemas comuns de setup, build, testes, CI e execução local. |
| `UNSAFE-POLICY.md` | Formalizar a política de uso de `unsafe` quando houver necessidade concreta. |

Enquanto esses documentos estiverem vazios, eles não introduzem regras próprias.
As decisões aplicáveis vêm dos documentos aprovados, da especificação, das ADRs
e da implementação já existente em `capi-lang`.

---

## Estado implementado

O Stage 0 criou a base de desenvolvimento em:

```text
capi-lang/
```

Essa base inclui:

* workspace Cargo;
* crates fundamentais;
* executável `capic`;
* `rust-toolchain.toml`;
* `rustfmt.toml`;
* `clippy.toml`;
* `Cargo.lock` versionado;
* scripts locais em `capi-lang/scripts/`;
* validação local equivalente à CI inicial.

---

## Fluxo local recomendado

Entre no workspace:

```bash
cd capi-lang
```

Verifique as ferramentas:

```bash
scripts/tools.sh
```

Execute a validação de rotina:

```bash
scripts/check.sh
```

Reproduza localmente a validação esperada pela CI:

```bash
scripts/ci-local.sh
```

Para validar manualmente os comandos principais:

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked
cargo test --workspace --locked
cargo run -p capi-cli --locked -- --help
cargo run -p capi-cli --locked -- --version
```

---

## Responsabilidades desta área

Esta área deve responder:

* como preparar o ambiente local;
* como construir a implementação oficial;
* como executar validação local;
* quais padrões seguir ao escrever código;
* quais convenções Rust são obrigatórias;
* quais práticas ainda precisam de documentos próprios;
* como evitar divergência entre desenvolvimento local e CI.

Ela não deve redefinir:

* a semântica da linguagem Capi;
* a arquitetura do compilador;
* a organização normativa do workspace;
* a política de dependências quando já definida em arquitetura ou ADR;
* os critérios de conclusão dos stages quando já definidos no plano de implementação.

---

## Relação com build e CI

Os documentos desta pasta descrevem como a pessoa desenvolvedora trabalha
localmente.

A definição canônica do sistema de build e da validação automatizada fica em:

```text
../build-and-ci/BUILD-SYSTEM.md
../build-and-ci/README.md
```

Na prática, desenvolvimento local e CI devem usar comandos equivalentes. Scripts
podem reduzir repetição, mas não devem esconder os comandos Cargo que formam a
base de validação.

---

## Leitura recomendada

Para começar a desenvolver, leia nesta ordem:

1. `DEVELOPMENT-SETUP.md`
2. `BUILDING-FROM-SOURCE.md`
3. `CODING-STANDARDS.md`
4. `RUST-STYLE-GUIDE.md`
5. `../build-and-ci/BUILD-SYSTEM.md`
6. `../architecture/WORKSPACE-ARCHITECTURE.md`
7. `../architecture/DEPENDENCY-RULES.md`

Essa ordem parte do setup prático, passa pelo build local e depois conecta o
trabalho diário às regras de arquitetura e dependência.

---

## Critério de atualização

Atualize este README quando:

* um documento desta pasta passar de reservado para aprovado;
* um novo procedimento operacional for adotado;
* scripts de desenvolvimento forem adicionados, removidos ou renomeados;
* a validação local do Stage atual mudar;
* houver alteração relevante na relação entre desenvolvimento local e CI.
