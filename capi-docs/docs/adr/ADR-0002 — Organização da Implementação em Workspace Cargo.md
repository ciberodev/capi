# ADR-0002 — Organização da Implementação em Workspace Cargo

**Status:** Aprovado  
**Data:** 2026-07-30  
**Stage:** Stage 0 — Fundação do projeto  
**Decisão:** Organizar a implementação oficial inicial como um workspace Cargo em `capi-lang/`.

---

## Contexto

A implementação oficial da Capi será inicialmente escrita em Rust.

O Stage 0 exige:

- criação do workspace Cargo em `capi-lang`;
- criação dos crates fundamentais;
- build completo do workspace;
- testes;
- formatação;
- Clippy;
- documentação Rust;
- CI;
- executável mínimo `capic`.

A arquitetura do compilador exige separação clara entre CLI, driver, sessão, fontes, diagnósticos, frontend, middle-end, backend, runtime, biblioteca padrão e toolchain.

Cargo fornece o mecanismo inicial para agrupar crates, resolver dependências, executar testes e produzir binários.

---

## Decisão

A implementação oficial inicial será organizada como um workspace Cargo localizado em:

```text
capi-lang/
```

Estrutura inicial esperada:

```text
capi-lang/
├── Cargo.toml
├── Cargo.lock
├── DEPENDENCIES.md
├── TOOLCHAIN.md
├── clippy.toml
├── rustfmt.toml
├── rust-toolchain.toml
├── crates/
├── tests/
├── scripts/
└── README.md
```

O manifesto raiz deve declarar um workspace Cargo com resolver moderno:

```toml
[workspace]
members = [
  "crates/*",
]
resolver = "2"
```

Os crates fundamentais do Stage 0 são:

```text
capi-cli
capi-driver
capi-session
capi-diagnostics
capi-source
capi-common
```

`Cargo.lock` deve ser versionado.

`target/` não deve ser versionado.

---

## Justificativa

Um workspace Cargo permite:

- build coordenado de todos os crates;
- separação de responsabilidades por crate;
- testes por crate e de integração;
- controle de dependências compartilhadas;
- lockfile reprodutível;
- integração simples com CI;
- geração do binário `capic`;
- evolução incremental por stages.

Essa organização é compatível com a decisão de usar Rust no bootstrap e com a necessidade de preservar fronteiras arquiteturais.

---

## Alternativas Consideradas

### Um único crate Rust

Rejeitado porque concentraria responsabilidades demais e dificultaria preservar fronteiras entre CLI, driver, infraestrutura, frontend, middle-end e backend.

### Múltiplos repositórios

Rejeitado para o Stage 0 porque aumentaria custo operacional, sincronização e complexidade de bootstrap antes de existir uma implementação funcional.

### Sistema de build próprio desde o início

Rejeitado porque desviaria esforço do objetivo principal do Stage 0: criar uma base compilável, testável e evolutiva.

---

## Consequências Positivas

- O projeto passa a ter uma unidade clara de build.
- Crates podem refletir fronteiras arquiteturais.
- Testes, lint, formatação e documentação usam ferramentas padrão.
- Dependências transitivas ficam visíveis em `Cargo.lock`.
- O CI pode executar comandos simples e equivalentes ao ambiente local.

---

## Consequências Negativas

- O Stage 0 fica temporariamente acoplado aos mecanismos de Cargo.
- Features Cargo e dependências externas precisam ser governadas com cuidado.
- A organização futura da toolchain Capi não deve ser confundida com a organização Cargo inicial.

---

## Restrições

- Cargo é mecanismo da implementação inicial, não parte da linguagem Capi.
- O workspace Cargo não define o futuro formato de pacotes Capi.
- Dependências entre crates devem formar grafo acíclico.
- Crates binários não devem ser dependência de crates de biblioteca.
- Backends concretos devem permanecer isolados.
- `Cargo.lock` deve ser revisável e versionado.

---

## Critérios de Validação

Esta decisão será considerada operacional quando, em `capi-lang/`, os comandos abaixo funcionarem:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets
cargo test --workspace
cargo build --workspace
```

Quando `capic` existir, também devem funcionar:

```bash
cargo run -p capi-cli -- --help
cargo run -p capi-cli -- --version
```

## Implementação no Stage 0

A estrutura inicial foi criada em `capi-lang`.

Crates presentes:

```text
capi-cli
capi-driver
capi-session
capi-diagnostics
capi-source
capi-common
```

Arquivos operacionais adicionados:

```text
capi-lang/DEPENDENCIES.md
capi-lang/TOOLCHAIN.md
capi-lang/clippy.toml
capi-lang/rustfmt.toml
capi-lang/rust-toolchain.toml
capi-lang/scripts/
```

Scripts locais:

```bash
scripts/tools.sh
scripts/deps.sh
scripts/fmt-check.sh
scripts/lint.sh
scripts/test.sh
scripts/build.sh
scripts/doc.sh
scripts/check.sh
scripts/ci-local.sh
```

A CI do repositório fica em `.github/workflows/capi-lang-ci.yml` e valida o workspace `capi-lang`.

Documentação Rust também foi incorporada à validação operacional:

```bash
cargo doc --workspace --no-deps --locked
```

---

## Referências

- Documento 27 — Bootstrap Plan e Arquitetura da Implementação Oficial
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial
- `PROJECT-STRUCTURE.md`
- `WORKSPACE-ARCHITECTURE.md`
- `DEPENDENCY-RULES.md`
- `BUILD-SYSTEM.md`
