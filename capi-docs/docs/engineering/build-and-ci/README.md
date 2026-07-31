# Build and CI

Esta pasta reúne a documentação de engenharia sobre build, validação local,
integração contínua, artefatos e política operacional de construção da
implementação oficial da Linguagem Capi.

No estado atual do projeto, o conteúdo normativo preenchido para o Stage 0 está
concentrado em `BUILD-SYSTEM.md`. Os demais documentos existem como pontos de
consolidação para detalhamentos futuros.

---

## Documento aprovado no Stage 0

| Documento | Status | Finalidade |
| --- | --- | --- |
| `BUILD-SYSTEM.md` | Aprovado | Define o sistema inicial de build em Cargo, os comandos canônicos, a validação local, a geração de documentação Rust, o uso de `Cargo.lock`, a política de artefatos gerados e os critérios mínimos de build do Stage 0. |

---

## Documentos reservados

| Documento | Finalidade esperada |
| --- | --- |
| `CI-CD.md` | Consolidar a política completa de CI/CD quando o projeto avançar para release, publicação, empacotamento ou distribuição. |
| `CI-JOBS.md` | Documentar em detalhe os jobs da CI, seus gatilhos, responsabilidades, ordem lógica e critérios de falha. |
| `BUILD-PROFILES.md` | Definir perfis de build, diferenças entre desenvolvimento, teste, release e possíveis builds especializados. |
| `REPRODUCIBLE-BUILDS.md` | Definir requisitos de reprodutibilidade além da validação funcional inicial. |
| `SUPPORTED-PLATFORMS.md` | Registrar plataformas suportadas, plataformas experimentais e política de suporte. |
| `CROSS-COMPILATION.md` | Definir estratégia de compilação cruzada quando houver necessidade formal. |
| `BOOTSTRAP-BUILD.md` | Descrever o processo de bootstrap da implementação oficial quando a toolchain Capi depender de componentes próprios. |
| `ARTIFACT-PROVENANCE.md` | Definir rastreabilidade, metadados e procedência de artefatos gerados. |

Enquanto esses documentos estiverem vazios, eles não introduzem regras próprias.
As decisões aplicáveis vêm de `BUILD-SYSTEM.md`, das ADRs aprovadas e dos
documentos de engenharia já preenchidos.

---

## Estado implementado

O Stage 0 estabeleceu o workspace Rust em:

```text
capi-lang/
```

A implementação inicial inclui:

* workspace Cargo com `resolver = "2"`;
* crates fundamentais da base do compilador;
* executável `capic` no crate `capi-cli`;
* `Cargo.lock` versionado;
* configuração inicial de `rustfmt`;
* configuração inicial de `clippy`;
* scripts locais de desenvolvimento;
* workflow de CI em `.github/workflows/capi-lang-ci.yml`;
* documentação Rust gerada por `cargo doc`;
* validação de `capic --help`, `capic --version` e erro para arquivo inexistente.

---

## Comandos canônicos

Todos os comandos abaixo devem ser executados a partir de `capi-lang/`.

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked
cargo test --workspace --locked
cargo doc --workspace --no-deps --locked
cargo run -p capi-cli --locked -- --help
cargo run -p capi-cli --locked -- --version
```

Esses comandos formam a validação mínima do Stage 0 para build, estilo, lint,
testes, documentação Rust e smoke tests do executável `capic`.

---

## Scripts locais

O workspace também expõe scripts em `capi-lang/scripts/` para padronizar a
execução local:

```bash
scripts/tools.sh
scripts/deps.sh
scripts/fmt.sh
scripts/fmt-check.sh
scripts/lint.sh
scripts/test.sh
scripts/build.sh
scripts/doc.sh
scripts/check.sh
scripts/ci-local.sh
```

Regra prática:

* use `scripts/check.sh` para validação local de rotina;
* use `scripts/ci-local.sh` para reproduzir localmente o conjunto esperado pela
  CI do Stage 0;
* mantenha os scripts como encapsulamento de comandos explícitos, sem substituir
  a documentação dos comandos Cargo canônicos.

---

## Integração contínua

A CI inicial está definida em:

```text
.github/workflows/capi-lang-ci.yml
```

Os jobs atuais cobrem:

* formatação;
* versões mínimas de ferramentas;
* lint com Clippy;
* política inicial de dependências;
* testes;
* build completo do workspace;
* documentação Rust;
* smoke tests do `capic`.

A CI roda para alterações em:

```text
.github/workflows/capi-lang-ci.yml
capi-lang/**
```

---

## Artefatos

Artefatos gerados pelo build devem permanecer fora do versionamento.

Exemplos:

```text
capi-lang/target/
```

O lockfile do workspace deve ser versionado:

```text
capi-lang/Cargo.lock
```

Essa separação preserva reprodutibilidade sem transformar saídas locais de build
em fonte normativa do projeto.

---

## Relação com o Stage 0

Para o Stage 0, esta área de documentação sustenta os seguintes critérios de
conclusão:

* o workspace compila sem erros;
* `cargo fmt` não encontra divergências;
* `cargo clippy` não encontra erros bloqueadores;
* todos os testes passam;
* `capic --help` funciona;
* `capic --version` funciona;
* a CI está operacional;
* as dependências respeitam as regras definidas;
* existe validação local equivalente à CI.

O registro formal de progresso do Stage 0 fica em:

```text
capi-docs/docs/engineering/planning/FEATURE-STATUS.md
```

---

## Leitura recomendada

Para entender esta área, leia nesta ordem:

1. `BUILD-SYSTEM.md`
2. `../development/DEVELOPMENT-SETUP.md`
3. `../development/BUILDING-FROM-SOURCE.md`
4. `../architecture/WORKSPACE-ARCHITECTURE.md`
5. `../architecture/DEPENDENCY-RULES.md`
6. `../../adr/README.md`

Essa ordem parte da operação prática do Stage 0 e depois conecta o build às
decisões arquiteturais e ADRs aprovadas.
