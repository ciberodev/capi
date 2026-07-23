# Development Setup

**Projeto:** Linguagem Capi
**Documento:** DEVELOPMENT-SETUP
**Status:** Proposto
**Stage:** Stage 0 — Fundação do projeto
**Natureza:** Documento de engenharia bloqueante
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o ambiente mínimo de desenvolvimento para a implementação oficial da Linguagem Capi.

Seu objetivo é permitir que uma pessoa desenvolvedora configure a máquina, valide a estrutura do repositório, execute os comandos básicos do Stage 0 e reproduza localmente os mesmos fluxos esperados pela integração contínua.

Este documento é operacional.

Ele não define arquitetura do compilador, organização de crates ou política de dependências, que pertencem aos documentos específicos.

---

## 2. Escopo

Este documento cobre:

* plataforma inicial suportada;
* ferramentas obrigatórias;
* ferramentas recomendadas;
* configuração inicial do repositório;
* comandos básicos de desenvolvimento;
* validação do Stage 0;
* solução inicial de problemas.

Este documento não cobre:

* instalação detalhada de cada distribuição Linux;
* configuração completa de CI;
* build de releases;
* bootstrap completo;
* setup de LLVM;
* configuração da futura toolchain `capi`.

---

## 3. Plataforma inicial

A plataforma inicial de desenvolvimento e validação é:

```text
Sistema operacional: Linux
Arquitetura: x86-64
Formato de objeto inicial: ELF
ABI de plataforma inicial: System V AMD64
```

Outras plataformas poderão funcionar em parte, mas não são alvo obrigatório do Stage 0.

Problemas específicos de plataformas não suportadas inicialmente não devem bloquear o Stage 0.

---

## 4. Estrutura esperada do repositório

O repositório deve possuir:

```text
capi/
├── capi-docs/
└── capi-lang/
```

`capi-docs` contém documentação.

`capi-lang` contém ou conterá o workspace Rust da implementação oficial.

No início do Stage 0, `capi-lang` pode estar vazio.

Após a criação do workspace, `capi-lang` deve conter `Cargo.toml`, `Cargo.lock`, crates fundamentais e comandos de validação.

---

## 5. Ferramentas obrigatórias

As ferramentas obrigatórias para o Stage 0 são:

```text
git
rustup
rustc
cargo
rustfmt
clippy
```

Também são esperadas ferramentas comuns de ambiente Linux:

```text
sh ou bash
pkg-config, quando necessário
cc, quando alguma dependência exigir compilação nativa
```

A necessidade exata de `cc` e `pkg-config` depende das dependências adotadas.

O Stage 0 deve evitar dependências externas que exijam toolchain nativa complexa.

---

## 6. Ferramentas recomendadas

Ferramentas recomendadas:

```text
rust-analyzer
cargo-audit
cargo-deny
cargo-nextest
just ou make
```

Essas ferramentas não são obrigatórias para concluir o Stage 0, salvo decisão posterior em `BUILD-SYSTEM.md` ou CI.

Quando uma ferramenta recomendada se tornar obrigatória, este documento deve ser atualizado.

---

## 7. Instalação do Rust

A instalação recomendada do Rust deve ser feita por `rustup`.

Comandos típicos:

```bash
rustup toolchain install stable
rustup component add rustfmt clippy
```

Quando `rust-toolchain.toml` existir em `capi-lang`, o comando deve respeitar a versão declarada pelo projeto:

```bash
cd capi-lang
rustup show
```

A MSRV exata será definida em documento próprio.

Até lá, o ambiente deve usar a versão estável documentada pelo Stage 0.

---

## 8. Validação das ferramentas

Verificações básicas:

```bash
git --version
rustc --version
cargo --version
rustfmt --version
cargo clippy --version
```

Esses comandos devem executar sem erro.

Caso `cargo clippy` falhe por componente ausente:

```bash
rustup component add clippy
```

Caso `cargo fmt` falhe por componente ausente:

```bash
rustup component add rustfmt
```

---

## 9. Primeiro acesso ao repositório

Depois de obter o repositório, a pessoa desenvolvedora deve validar a estrutura:

```bash
ls capi-docs
ls capi-lang
```

Enquanto o workspace ainda não existir, a ausência de `capi-lang/Cargo.toml` é esperada.

Depois da inicialização do Stage 0, a ausência de `capi-lang/Cargo.toml` passa a indicar setup incompleto.

---

## 10. Setup de `capi-docs`

`capi-docs` não exige build para leitura e edição básica.

Validações úteis:

```bash
find capi-docs/docs -type f -name '*.md'
```

Edições em documentos devem preservar:

* coerência com a especificação;
* rastreabilidade;
* status documental;
* links relativos quando aplicáveis;
* separação entre documentação e implementação.

---

## 11. Setup de `capi-lang`

Quando o workspace for criado, a entrada padrão será:

```bash
cd capi-lang
```

Comandos obrigatórios:

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets
```

Se o workspace ainda não existir, esses comandos não são aplicáveis.

Nesse caso, a próxima ação operacional é criar o workspace conforme `WORKSPACE-ARCHITECTURE.md` e `BUILD-SYSTEM.md`.

---

## 12. Executável `capic`

No Stage 0, o executável mínimo esperado é:

```text
capic
```

Comandos obrigatórios:

```bash
capic --help
capic --version
```

Enquanto o binário ainda não estiver instalado no `PATH`, ele pode ser executado via Cargo:

```bash
cargo run -p capi-cli -- --help
cargo run -p capi-cli -- --version
```

O nome do pacote responsável pelo binário deve seguir `WORKSPACE-ARCHITECTURE.md`.

---

## 13. Variáveis de ambiente

O Stage 0 deve exigir o mínimo possível de variáveis de ambiente.

Variáveis comuns:

```text
RUST_BACKTRACE
RUST_LOG
```

Uso recomendado para investigação:

```bash
RUST_BACKTRACE=1 cargo test --workspace
```

`RUST_LOG` somente deve afetar logs internos quando a infraestrutura de logging existir.

Comportamento funcional do compilador não deve depender de variável de ambiente não documentada.

---

## 14. Scripts de desenvolvimento

Scripts podem existir em:

```text
capi-lang/scripts/
```

Eles podem encapsular comandos comuns, como:

```text
format
lint
test
check
ci-local
```

Regras:

* scripts devem ser reproduzíveis;
* scripts devem falhar com código de saída não zero em erro;
* scripts não devem esconder comandos essenciais;
* scripts não devem depender de estado local não documentado;
* comandos Cargo equivalentes devem continuar claros.

---

## 15. Fluxo local recomendado

Fluxo recomendado antes de abrir uma alteração:

```bash
cd capi-lang
cargo fmt --all --check
cargo clippy --workspace --all-targets
cargo test --workspace
```

Quando `capic` existir:

```bash
cargo run -p capi-cli -- --help
cargo run -p capi-cli -- --version
```

Se scripts oficiais forem criados, eles podem agrupar esse fluxo.

---

## 16. Edição de documentação

Para editar documentação:

* manter títulos e metadados coerentes;
* não alterar contratos normativos fora do processo adequado;
* atualizar referências quando criar novo documento;
* preservar o escopo do documento editado;
* não mover conteúdo normativo para `capi-lang`.

Documentos de engenharia devem continuar coerentes com:

* `ENGINEERING-PRINCIPLES.md`;
* `PROJECT-STRUCTURE.md`;
* `COMPILER-ARCHITECTURE.md`;
* `WORKSPACE-ARCHITECTURE.md`;
* `DEPENDENCY-RULES.md`;
* Documento 28.

---

## 17. Edição de código

Ao editar código em `capi-lang`:

* respeitar responsabilidade do crate;
* evitar dependência circular;
* adicionar testes quando houver comportamento;
* usar diagnósticos estruturados;
* evitar `unsafe` no Stage 0;
* manter `cargo fmt` e `cargo clippy` limpos;
* não introduzir dependência externa sem seguir `DEPENDENCY-RULES.md`.

Código que muda fronteira arquitetural deve motivar atualização documental ou ADR.

---

## 18. Dependências locais

Antes de adicionar dependência externa:

* verificar se a biblioteca padrão Rust resolve;
* verificar impacto em MSRV;
* verificar licença;
* verificar dependências transitivas;
* verificar necessidade de ADR;
* registrar motivo.

No Stage 0, dependências externas devem ser mínimas.

Cranelift e LLVM não são necessários para o Stage 0.

---

## 19. Problemas comuns

### 19.1 `cargo` não encontrado

Instalar Rust via `rustup` e reiniciar o shell.

### 19.2 `cargo fmt` falha por componente ausente

Executar:

```bash
rustup component add rustfmt
```

### 19.3 `cargo clippy` falha por componente ausente

Executar:

```bash
rustup component add clippy
```

### 19.4 `capi-lang/Cargo.toml` não existe

Antes da criação do workspace, isso é esperado.

Depois do início da implementação do Stage 0, isso indica que o workspace ainda não foi inicializado.

### 19.5 `capic` não está no `PATH`

Usar execução via Cargo:

```bash
cargo run -p capi-cli -- --help
```

Ou ajustar o `PATH` somente quando houver artefato instalado oficialmente.

---

## 20. Validação do Stage 0

O ambiente local está suficiente para o Stage 0 quando:

* Rust está instalado;
* `rustfmt` está instalado;
* Clippy está instalado;
* `capi-docs` pode ser lido e editado;
* `capi-lang` existe;
* o workspace Cargo compila, quando criado;
* testes executam, quando criados;
* `capic --help` e `capic --version` funcionam, quando implementados.

Resultado esperado após criação do workspace:

```bash
cd capi-lang
cargo build --workspace
cargo test --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets
cargo run -p capi-cli -- --help
cargo run -p capi-cli -- --version
```

---

## 21. Critérios de aceite deste documento

Este documento estará pronto para orientar o Stage 0 quando:

* definir plataforma inicial;
* listar ferramentas obrigatórias;
* explicar validação do ambiente;
* distinguir estado antes e depois da criação do workspace;
* documentar comandos Cargo obrigatórios;
* documentar execução inicial de `capic`;
* não contradizer os Documentos 00 a 28;
* complementar `PROJECT-STRUCTURE.md`, `WORKSPACE-ARCHITECTURE.md`, `DEPENDENCY-RULES.md` e `BUILD-SYSTEM.md`.

---

## 22. Síntese

O ambiente de desenvolvimento da implementação oficial deve ser simples, reproduzível e compatível com o Stage 0.

A pessoa desenvolvedora deve conseguir validar ferramentas, entrar em `capi-lang`, executar build, testes, formatação, lint e os comandos mínimos de `capic` assim que o workspace for criado.
