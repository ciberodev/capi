# Building From Source

**Projeto:** Linguagem Capi  
**Documento:** BUILDING-FROM-SOURCE  
**Status:** Aprovado  
**Stage:** Stage 0 — Fundação do projeto  
**Natureza:** Documento operacional  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define como construir a implementação oficial da Linguagem Capi a partir do código-fonte.

Seu objetivo é fornecer um procedimento operacional para:

- preparar o ambiente mínimo;
- localizar o workspace de implementação;
- executar build local;
- executar testes;
- validar formatação e lint;
- executar o binário `capic`;
- reproduzir localmente a validação esperada em CI.

Este documento não cria o workspace `capi-lang`.

Ele descreve como o workspace deve ser construído quando existir.

---

## 2. Estado Atual

No início do Stage 0, o diretório `capi-lang/` pode existir sem workspace Cargo.

Enquanto `capi-lang/Cargo.toml` não existir:

- não há compilador para construir;
- comandos Cargo em `capi-lang/` ainda não são aplicáveis;
- este documento serve como guia para a estrutura que será criada;
- a validação do repositório é documental.

Após a criação do workspace, este documento passa a ser o procedimento canônico para build local da implementação.

---

## 3. Pré-requisitos

Plataforma inicial suportada:

```text
Sistema operacional: Linux
Arquitetura: x86-64
Formato de objeto inicial: ELF
ABI inicial: System V AMD64
```

Ferramentas obrigatórias:

```text
git
rustup
rustc
cargo
rustfmt
clippy
```

Ferramentas de sistema esperadas quando alguma dependência exigir compilação nativa:

```text
cc
pkg-config
sh ou bash
```

O Stage 0 deve evitar dependências que exijam setup nativo complexo.

---

## 4. Verificação do Ambiente

Antes de construir, verifique as ferramentas:

```bash
git --version
rustc --version
cargo --version
rustfmt --version
cargo clippy --version
```

Se `rustfmt` ou `clippy` não estiverem instalados:

```bash
rustup component add rustfmt
rustup component add clippy
```

Quando `capi-lang/rust-toolchain.toml` existir, a versão de Rust deve ser resolvida por esse arquivo.

Verificação recomendada:

```bash
cd capi-lang
rustup show
```

---

## 5. Estrutura Esperada

Estrutura geral do repositório:

```text
capi/
├── capi-docs/
└── capi-lang/
```

O build da implementação oficial ocorre em:

```text
capi-lang/
```

Estrutura esperada após a criação do workspace:

```text
capi-lang/
├── Cargo.toml
├── Cargo.lock
├── rustfmt.toml
├── rust-toolchain.toml
├── crates/
├── tests/
├── scripts/
└── README.md
```

`Cargo.lock` deve ser versionado.

`target/` não deve ser versionado.

---

## 6. Entrada no Workspace

Todos os comandos Cargo deste documento devem ser executados a partir de:

```bash
cd capi-lang
```

Se este comando funcionar, mas `Cargo.toml` não existir, o workspace ainda não foi criado.

Nesse caso, a próxima ação é criar o workspace conforme:

- `WORKSPACE-ARCHITECTURE.md`;
- `BUILD-SYSTEM.md`;
- `DEPENDENCY-RULES.md`;
- `DEFINITION-OF-DONE.md`.

---

## 7. Build de Desenvolvimento

Comando canônico:

```bash
cargo build --workspace
```

Esse comando deve:

- compilar todos os crates do workspace;
- falhar se qualquer crate estiver quebrado;
- usar dependências resolvidas pelo `Cargo.lock`;
- funcionar em ambiente local e CI.

Falha nesse comando bloqueia conclusão do Stage 0.

---

## 8. Build de Release

Comando:

```bash
cargo build --workspace --release
```

No Stage 0, build de release é desejável, mas não substitui o build de desenvolvimento nem os demais comandos de validação.

Build de release passa a ser mais importante em stages com geração de artefatos executáveis, runtime e backend.

---

## 9. Testes

Comando canônico:

```bash
cargo test --workspace
```

No Stage 0, a suíte mínima deve cobrir:

- `capic --help`;
- `capic --version`;
- arquivo inexistente;
- inicialização básica do driver;
- inicialização básica da sessão;
- diagnósticos básicos.

Testes de lexer, parser, HIR, MIR, codegen e runtime entram nos stages correspondentes.

---

## 10. Formatação

Verificação obrigatória:

```bash
cargo fmt --all --check
```

Correção local:

```bash
cargo fmt --all
```

Diferenças de formatação bloqueiam CI.

---

## 11. Lint

Verificação obrigatória:

```bash
cargo clippy --workspace --all-targets
```

O Stage 0 deve tratar avisos relevantes de Clippy como problemas de qualidade, especialmente em crates fundamentais.

Políticas mais estritas podem ser definidas em documentos de estilo e CI.

---

## 12. Validação Local Completa

Sequência recomendada antes de considerar uma mudança pronta:

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

Essa sequência deve ser equivalente à validação mínima de CI do Stage 0.

---

## 13. Executando `capic`

Enquanto `capic` não estiver instalado no `PATH`, execute via Cargo:

```bash
cargo run -p capi-cli -- --help
cargo run -p capi-cli -- --version
```

Quando o binário existir em `target`, também poderá ser executado diretamente.

Exemplo conceitual:

```bash
./target/debug/capic --help
./target/debug/capic --version
```

No Stage 0, `capic` não precisa compilar código Capi.

Ele deve, no mínimo:

- exibir ajuda;
- exibir versão;
- tratar argumentos inválidos;
- reportar arquivo inexistente sem panic não controlado.

---

## 14. Documentação Rust

Comando recomendado:

```bash
cargo doc --workspace --no-deps
```

No Stage 0, documentação Rust gerada é útil para revisar APIs internas dos crates fundamentais.

Esse comando pode se tornar obrigatório em CI se o projeto decidir bloquear documentação quebrada.

---

## 15. Artefatos Gerados

Artefatos gerados por build não devem ser versionados.

Exemplos:

```text
capi-lang/target/
```

Arquivos que devem ser versionados:

```text
capi-lang/Cargo.toml
capi-lang/Cargo.lock
capi-lang/rustfmt.toml
capi-lang/rust-toolchain.toml
```

Arquivos de configuração adicionais devem ser versionados quando forem necessários para reproduzir build, lint, testes ou CI.

---

## 16. Limpeza de Build

Para remover artefatos locais:

```bash
cargo clean
```

Esse comando remove `target/`.

Ele não deve ser necessário para builds normais. Se um problema só for resolvido com `cargo clean`, a causa deve ser investigada quando recorrente.

---

## 17. Atualização de Dependências

Dependências devem seguir `DEPENDENCY-RULES.md`.

Atualizações de dependências devem ser intencionais e revisáveis.

Comando típico:

```bash
cargo update
```

Após atualizar dependências, a validação completa deve ser executada:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets
cargo test --workspace
cargo build --workspace
```

Alterações em `Cargo.lock` devem ser revisadas.

---

## 18. Problemas Comuns

### `capi-lang/Cargo.toml` não existe

O workspace ainda não foi criado.

Esse estado é esperado antes da entrega de infraestrutura do Stage 0.

### `cargo fmt` falha por componente ausente

Instale o componente:

```bash
rustup component add rustfmt
```

### `cargo clippy` falha por componente ausente

Instale o componente:

```bash
rustup component add clippy
```

### Build falha em dependência nativa

Verifique se a dependência é permitida e necessária.

Quando aplicável, instale ferramentas de sistema como `cc` e `pkg-config`.

Dependências nativas complexas devem ser evitadas no Stage 0.

### `capic` não é encontrado

Use Cargo:

```bash
cargo run -p capi-cli -- --help
```

Ou verifique se o crate `capi-cli` já foi criado.

---

## 19. Relação com CI

O build local deve espelhar a CI.

No Stage 0, a CI deve executar comandos equivalentes a:

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

Se local e CI divergirem, a diferença deve ser documentada e corrigida.

---

## 20. Critérios de Aceitação do Documento

Este documento é considerado preenchido quando:

- explica o estado antes e depois da criação do workspace;
- define pré-requisitos;
- indica o diretório correto para build;
- documenta comandos de build, testes, fmt e clippy;
- documenta execução mínima de `capic`;
- explica artefatos gerados e `Cargo.lock`;
- registra problemas comuns;
- permanece coerente com `DEVELOPMENT-SETUP.md`, `BUILD-SYSTEM.md` e `DEFINITION-OF-DONE.md`.

---

## 21. Síntese

Construir a Capi a partir do código-fonte deve ser um processo simples, reprodutível e equivalente à CI.

No Stage 0, o objetivo é garantir que o workspace Rust nasça com comandos claros de build, teste, formatação, lint e execução mínima do `capic`.
