# Feature Status

**Projeto:** Linguagem Capi  
**Documento:** FEATURE-STATUS  
**Status:** Aprovado  
**Stage:** Stage 0 — Fundação do projeto  
**Natureza:** Registro de progresso  
**Base normativa:** Documento 28 — Plano de Desenvolvimento da Implementação Oficial

---

## Stage 0 — Fundação do projeto

```text
Stage: Stage 0 — Fundação do projeto
Responsável: Projeto Capi
Data de início: 2026-07-30
Data de conclusão: 2026-07-30
Status: Concluído
```

### Documentos concluídos

Documentos bloqueantes aprovados:

- `ENGINEERING-PRINCIPLES.md`
- `PROJECT-STRUCTURE.md`
- `COMPILER-ARCHITECTURE.md`
- `WORKSPACE-ARCHITECTURE.md`
- `COMPONENT-RESPONSIBILITIES.md`
- `DEPENDENCY-RULES.md`
- `DEVELOPMENT-SETUP.md`
- `BUILD-SYSTEM.md`
- `TEST-STRATEGY.md`
- `DEFINITION-OF-DONE.md`

Documentos operacionais e de consolidação aprovados:

- `ENGINEERING-GLOSSARY.md`
- `COMPILATION-PIPELINE.md`
- `BUILDING-FROM-SOURCE.md`
- `CODING-STANDARDS.md`
- `RUST-STYLE-GUIDE.md`

Documento operacional de referência aprovado:

- `28 — Plano de Desenvolvimento da Implementação Oficial.md`

### ADRs criados e aprovados

- `ADR-0001 — Rust como Linguagem da Implementação Oficial.md`
- `ADR-0002 — Organização da Implementação em Workspace Cargo.md`
- `ADR-0003 — Separação entre Frontend, Middle-end e Backend.md`
- `ADR-0013 — Política de Dependências Externas.md`
- `ADR-0015 — Estratégia Inicial de Testes.md`
- `ADR-0016 — Organização Física do Repositório.md`

### Infraestrutura concluída

- workspace Cargo criado em `capi-lang`;
- estrutura inicial de crates criada;
- `cargo fmt` configurado;
- `cargo clippy` configurado;
- execução de testes configurada;
- integração contínua configurada em `.github/workflows/capi-lang-ci.yml`;
- documentação Rust configurada;
- scripts de desenvolvimento configurados;
- política inicial de dependências definida;
- versões mínimas das ferramentas definidas.

### Implementações concluídas

- executável `capic` criado;
- crate `capi-driver` criado;
- crate `capi-session` criado;
- crate `capi-diagnostics` criado;
- crate `capi-source` criado;
- `capic --help` implementado;
- `capic --version` implementado;
- leitura inicial de argumentos implementada;
- inicialização da sessão implementada;
- tratamento básico de erros internos implementado.

### Testes concluídos

Validação executada em `capi-lang`:

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

Resultado:

- build completo aprovado;
- formatação aprovada;
- Clippy aprovado;
- suíte inicial aprovada;
- `capic --help` aprovado;
- `capic --version` aprovado;
- arquivo inexistente retorna erro controlado;
- validação local equivalente à CI aprovada.

### Pendências

Nenhuma pendência bloqueante do Stage 0.

A execução remota do workflow no provedor de CI depende de `push` ou `pull_request`, mas o workflow está versionado e a validação local equivalente foi aprovada.

### Riscos

- As ADRs e documentos aprovados devem ser mantidos sincronizados com mudanças posteriores no workspace.
- Dependências externas futuras exigirão revisão explícita e possível ADR.
- A MSRV `1.88.0` deve ser preservada até decisão formal de atualização.

### Resultado da validação

Stage 0 concluído conforme os critérios do Documento 28.

O workspace Rust existe, compila, testa, gera documentação Rust, valida dependências, valida toolchain, executa `capic --help`, executa `capic --version`, reporta arquivo inexistente de forma controlada e possui CI configurada.
