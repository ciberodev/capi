# Build System

**Projeto:** Linguagem Capi
**Documento:** BUILD-SYSTEM
**Status:** Proposto
**Stage:** Stage 0 — Fundação do projeto
**Natureza:** Documento de engenharia bloqueante
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o sistema de build inicial da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer como o workspace Rust em `capi-lang` deve ser construído, testado, formatado, analisado e validado localmente e em integração contínua durante o Stage 0.

Este documento não cria o workspace.

Ele define os requisitos que o workspace deve atender quando for criado.

---

## 2. Escopo

Este documento cobre:

* uso inicial do Cargo;
* comandos canônicos de build;
* formatação;
* lint;
* testes;
* documentação Rust;
* perfis de build;
* lockfile;
* artefatos gerados;
* validação local equivalente à CI;
* critérios mínimos do Stage 0.

Este documento não cobre:

* pipeline completo de CI;
* release;
* assinatura de artefatos;
* bootstrap completo;
* build da futura toolchain Capi;
* empacotamento de distribuição;
* instalação de dependências de sistema em cada distribuição Linux.

---

## 3. Princípios

O build deve seguir os documentos:

* `ENGINEERING-PRINCIPLES.md`;
* `PROJECT-STRUCTURE.md`;
* `WORKSPACE-ARCHITECTURE.md`;
* `DEPENDENCY-RULES.md`;
* `DEVELOPMENT-SETUP.md`.

Regras centrais:

* build local e CI devem usar comandos equivalentes;
* comandos devem ser reproduzíveis;
* falhas devem ser visíveis;
* artefatos gerados não devem ser versionados;
* dependências devem ser controladas por `Cargo.lock`;
* o build não deve depender de estado local não documentado;
* scripts podem encapsular comandos, mas não escondê-los.

---

## 4. Sistema de build inicial

O sistema de build inicial é Cargo.

Cargo será usado para:

* organizar o workspace Rust;
* construir crates;
* executar testes;
* executar exemplos quando existirem;
* gerar documentação Rust;
* resolver dependências;
* aplicar perfis de build;
* produzir o executável `capic`.

Cargo é mecanismo da implementação inicial.

Ele não define a linguagem Capi nem a futura toolchain pública.

---

## 5. Localização

O build da implementação oficial ocorre em:

```text
capi-lang/
```

Comando de entrada:

```bash
cd capi-lang
```

Antes da criação do workspace, comandos Cargo não são aplicáveis.

Após o início do Stage 0, `capi-lang/Cargo.toml` deve existir.

---

## 6. Arquivos obrigatórios

Arquivos esperados no Stage 0:

```text
capi-lang/Cargo.toml
capi-lang/Cargo.lock
capi-lang/rustfmt.toml
capi-lang/rust-toolchain.toml ou política equivalente
capi-lang/README.md
```

Arquivos opcionais conforme necessidade:

```text
capi-lang/clippy.toml
capi-lang/.cargo/config.toml
capi-lang/scripts/
```

`Cargo.lock` deve ser versionado.

`target/` não deve ser versionado.

---

## 7. Manifesto do workspace

O manifesto raiz deve declarar o workspace.

Formato conceitual:

```toml
[workspace]
members = [
  "crates/*",
]
resolver = "2"

[workspace.package]
edition = "2021"

[workspace.dependencies]
# Dependências compartilhadas aprovadas.
```

A edição Rust, MSRV, licença e metadados exatos devem seguir os documentos aprovados.

Dependências compartilhadas devem respeitar `DEPENDENCY-RULES.md`.

---

## 8. Comandos canônicos

Os comandos canônicos do Stage 0 são:

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets
```

Esses comandos devem funcionar no ambiente local e em CI.

Quando `capic` existir, também devem funcionar:

```bash
cargo run -p capi-cli -- --help
cargo run -p capi-cli -- --version
```

Esses comandos são a base mínima de validação do Stage 0.

---

## 9. Build

Build de desenvolvimento:

```bash
cargo build --workspace
```

Build de release:

```bash
cargo build --workspace --release
```

No Stage 0, build de release é desejável, mas não substitui validação de desenvolvimento.

Falha de build em qualquer crate do workspace bloqueia conclusão do stage.

---

## 10. Testes

Comando padrão:

```bash
cargo test --workspace
```

O Stage 0 deve incluir testes para:

* `capic --help`;
* `capic --version`;
* erro controlado para arquivo inexistente quando aplicável;
* comportamento básico de diagnósticos;
* inicialização do driver ou sessão quando existir.

Testes de fases futuras devem ser adicionados nos stages correspondentes.

---

## 11. Formatação

Formatação deve usar `rustfmt`.

Comando de verificação:

```bash
cargo fmt --all --check
```

Comando de correção local:

```bash
cargo fmt --all
```

Configuração esperada:

```text
capi-lang/rustfmt.toml
```

O formato deve ser determinístico.

Diferenças de formatação bloqueiam CI.

---

## 12. Lint

Lint deve usar Clippy.

Comando:

```bash
cargo clippy --workspace --all-targets
```

No Stage 0, warnings de Clippy devem ser tratados como problemas a corrigir.

Quando a política for formalizada, CI pode usar:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Exceções devem ser locais, justificadas e revisáveis.

---

## 13. Documentação Rust

Documentação Rust deve poder ser gerada.

Comando:

```bash
cargo doc --workspace --no-deps
```

No Stage 0, documentação Rust não precisa estar completa, mas APIs públicas internas entre crates devem possuir documentação suficiente para uso correto.

Falhas de documentação devem ser corrigidas antes da conclusão formal do stage quando a documentação fizer parte da validação.

---

## 14. Verificação local completa

Comando local recomendado:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets
cargo test --workspace
cargo build --workspace
```

Quando `capic` existir:

```bash
cargo run -p capi-cli -- --help
cargo run -p capi-cli -- --version
```

Scripts podem agrupar esses comandos em:

```text
capi-lang/scripts/check
capi-lang/scripts/ci-local
```

Scripts devem preservar códigos de saída.

---

## 15. Perfis de build

Perfis mínimos:

```text
dev
test
release
```

### 15.1 `dev`

Usado para desenvolvimento cotidiano.

Prioriza:

* build rápido;
* símbolos de depuração;
* assertions internas;
* boa experiência de iteração.

### 15.2 `test`

Usado por `cargo test`.

Prioriza:

* determinismo;
* diagnósticos úteis;
* execução completa da suíte;
* isolamento entre testes.

### 15.3 `release`

Usado para artefatos otimizados.

No Stage 0, release não é critério principal.

Ele se tornará mais importante em stages de backend, performance e distribuição.

---

## 16. Artefatos gerados

Artefatos gerados devem ficar fora do controle de versão.

Diretórios comuns:

```text
capi-lang/target/
capi-lang/coverage/
capi-lang/.cache/
capi-lang/.tmp/
```

Exceções:

* snapshots;
* fixtures;
* golden files;
* artefatos deliberadamente versionados para testes.

Exceções devem ter justificativa documentada.

---

## 17. Lockfile

`Cargo.lock` deve ser preservado.

Regras:

* deve ser gerado no Stage 0;
* deve ser versionado;
* deve ser usado em CI;
* mudanças devem ser revisadas;
* atualizações devem respeitar MSRV e licenças.

O lockfile é parte da estratégia de reprodutibilidade do bootstrap.

---

## 18. MSRV e toolchain

O projeto deve declarar uma versão mínima de Rust ou política equivalente.

Arquivo recomendado:

```text
capi-lang/rust-toolchain.toml
```

Objetivos:

* reduzir divergência entre ambientes;
* permitir CI reproduzível;
* controlar adoção de recursos Rust;
* manter dependências compatíveis.

Alterar MSRV exige atualização documental e validação.

---

## 19. Configuração Cargo

Configuração local opcional:

```text
capi-lang/.cargo/config.toml
```

Uso aceitável:

* aliases de desenvolvimento;
* configuração de target;
* flags reproduzíveis;
* integração controlada com linker.

Uso proibido:

* depender de caminho local específico;
* esconder dependência de sistema não documentada;
* alterar comportamento sem refletir em CI;
* baixar código durante build.

---

## 20. Scripts

Scripts operacionais podem existir em:

```text
capi-lang/scripts/
```

Scripts recomendados:

```text
check
fmt
lint
test
doc
ci-local
```

Regras:

* scripts devem ser pequenos;
* scripts devem chamar comandos documentados;
* scripts devem falhar em erro;
* scripts devem funcionar a partir de caminho documentado;
* scripts não devem substituir documentação.

---

## 21. Integração com CI

CI deve executar comandos equivalentes aos locais.

Jobs mínimos esperados:

```text
build
test
fmt
clippy
doc, quando habilitado
```

CI deve:

* usar ambiente documentado;
* respeitar `Cargo.lock`;
* reportar falhas claramente;
* evitar estado persistente não declarado;
* validar o workspace inteiro.

Detalhes de CI pertencem a `CI-CD.md` e `CI-JOBS.md`.

---

## 22. Build de documentação

Documentação de engenharia em `capi-docs` não depende do build Rust.

Documentação Rust em `capi-lang` depende de:

```bash
cargo doc --workspace --no-deps
```

Documentação pública da linguagem e toolchain será tratada em stages posteriores.

---

## 23. Build de exemplos

Exemplos Capi serão introduzidos progressivamente.

No Stage 0, exemplos não são obrigatórios.

Quando existirem exemplos Rust auxiliares ou exemplos Capi futuros, devem possuir comando de validação documentado.

Exemplos não substituem testes.

---

## 24. Build de bootstrap

Build de bootstrap não pertence ao Stage 0.

Entretanto, o build do Stage 0 deve preservar condições para bootstrap futuro:

* `Cargo.lock` versionado;
* dependências controladas;
* comandos reproduzíveis;
* scripts claros;
* ausência de acoplamento indevido;
* separação entre compilador, runtime e std.

Detalhes ficam em `BOOTSTRAP-BUILD.md`.

---

## 25. Falhas de build

Falhas devem ser tratadas de forma objetiva.

Uma falha de build deve registrar:

* comando executado;
* ambiente relevante;
* erro observado;
* crate afetado;
* alteração recente provável;
* forma de reprodução.

Falhas intermitentes devem ser tratadas como problema de engenharia.

Elas não devem ser ignoradas por simples repetição manual.

---

## 26. Reprodutibilidade

No Stage 0, a exigência mínima é reprodutibilidade funcional.

Isso significa que os mesmos comandos, na mesma versão documentada de ferramentas e dependências, devem produzir os mesmos resultados observáveis:

* build bem-sucedido;
* testes aprovados;
* formatação estável;
* lint estável;
* `capic --help` consistente;
* `capic --version` consistente.

Reprodutibilidade binária exata não é exigência do Stage 0.

---

## 27. Comandos que bloqueiam conclusão do Stage 0

O Stage 0 não pode ser concluído se algum comando obrigatório falhar:

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets
cargo run -p capi-cli -- --help
cargo run -p capi-cli -- --version
```

Se o nome do pacote binário mudar por decisão aprovada, este documento deve ser atualizado.

---

## 28. Critérios de aceite deste documento

Este documento estará pronto para orientar o Stage 0 quando:

* definir Cargo como sistema de build inicial;
* definir comandos obrigatórios;
* definir uso de `Cargo.lock`;
* definir formatação, lint, testes e documentação Rust;
* definir artefatos gerados;
* distinguir build local e CI;
* não contradizer os Documentos 00 a 28;
* complementar `DEVELOPMENT-SETUP.md`, `WORKSPACE-ARCHITECTURE.md` e `DEPENDENCY-RULES.md`.

---

## 29. Síntese

O sistema de build inicial da Capi deve ser simples, reproduzível e baseado em Cargo.

O Stage 0 deve estabelecer uma base em que build, testes, formatação, lint e comandos mínimos de `capic` possam ser executados localmente e em CI com os mesmos resultados observáveis.
