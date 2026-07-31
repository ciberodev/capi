# ADR-0015 — Estratégia Inicial de Testes

**Status:** Aprovado  
**Data:** 2026-07-30  
**Stage:** Stage 0 — Fundação do projeto  
**Decisão:** Adotar testes automatizados desde o Stage 0, com suíte mínima para workspace, CLI, driver, sessão, fontes e diagnósticos.

---

## Contexto

A Capi possui uma especificação extensa e uma implementação oficial planejada por stages.

O Documento 27 estabelece que toda funcionalidade implementada deve possuir testes compatíveis com seu escopo e que regressões devem gerar testes permanentes.

O Documento 28 exige, no Stage 0:

- compilação completa do workspace;
- `cargo fmt`;
- `cargo clippy`;
- suíte inicial;
- teste de `capic --help`;
- teste de `capic --version`;
- teste de arquivo inexistente;
- validação da CI.

Como o compilador ainda não precisa processar código Capi no Stage 0, a estratégia inicial deve validar a fundação do projeto sem fingir cobertura de linguagem inexistente.

---

## Decisão

A implementação oficial terá testes automatizados desde o Stage 0.

A suíte inicial deve cobrir:

- build do workspace;
- formatação;
- lint;
- testes unitários iniciais;
- testes de integração básicos;
- comportamento de `capic --help`;
- comportamento de `capic --version`;
- erro controlado para arquivo inexistente;
- inicialização básica do driver;
- inicialização básica da sessão;
- emissão básica de diagnóstico;
- execução em CI.

Comandos canônicos:

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

No Stage 0, a validação local consolidada é executada por:

```bash
capi-lang/scripts/check.sh
```

---

## Justificativa

Testes desde o início reduzem risco de:

- criar workspace frágil;
- acoplar crates incorretamente;
- aceitar CLI instável;
- confundir erro de usuário com erro interno;
- avançar stages sem evidência objetiva;
- reintroduzir regressões;
- divergir da specification.

A suíte inicial deve ser pequena, rápida e confiável.

Ela prepara a base para suites futuras de lexer, parser, HIR, MIR, codegen, runtime, biblioteca padrão, conformidade, fuzzing, performance e testes diferenciais.

---

## Alternativas Consideradas

### Adiar testes até o lexer ou parser

Rejeitada porque permitiria a criação de infraestrutura sem validação objetiva.

### Criar uma suíte de conformidade completa no Stage 0

Rejeitada porque o compilador ainda não processa código Capi.

A suíte de conformidade deve crescer progressivamente com os stages.

### Testar apenas manualmente

Rejeitada porque o Stage 0 exige CI, reprodutibilidade e critérios objetivos de conclusão.

---

## Consequências Positivas

- O workspace nasce com ciclo de feedback confiável.
- O CI valida a fundação desde o início.
- Regressões passam a ter lugar permanente na suíte.
- Diagnósticos e CLI são tratados como comportamento observável.
- A evolução por stages ganha critério objetivo de avanço.

---

## Consequências Negativas

- O Stage 0 precisa criar infraestrutura de testes antes de funcionalidades da linguagem.
- Mudanças simples terão custo mínimo de validação.
- A suíte precisará evoluir continuamente para não ficar obsoleta.

---

## Restrições

- Testes devem ser determinísticos.
- Testes não devem depender de paths absolutos sem normalização.
- Testes devem minimizar dependências externas.
- Falhas esperadas de usuário não devem ser validadas por panic.
- Snapshots e golden files devem ser revisados como código.
- Benchmarks não substituem testes de corretude.

---

## Evolução Esperada

Após o Stage 0, a estratégia deve crescer por camadas:

- testes de fonte, spans e lexer;
- testes de parser e AST;
- testes de HIR e resolução de nomes;
- testes de tipos;
- testes de ownership, regiões e domains;
- testes de MIR;
- testes de runtime;
- testes de biblioteca padrão;
- testes de backend Cranelift;
- testes de conformidade;
- testes diferenciais com LLVM;
- testes de bootstrap e auto-hospedagem.

---

## Critérios de Validação

Esta decisão será considerada operacional quando:

- `TEST-STRATEGY.md` estiver aprovado;
- `cargo test --workspace` funcionar;
- a CLI mínima tiver testes;
- erro de arquivo inexistente for testado;
- CI executar a suíte inicial;
- regressões passarem a originar testes permanentes.

## Implementação no Stage 0

A suíte inicial foi implementada no workspace `capi-lang`.

Cobertura existente:

- testes de CLI em `capi-lang/crates/capi-cli/tests/capic_cli.rs`;
- testes unitários em `capi-common`;
- testes unitários em `capi-diagnostics`;
- testes unitários em `capi-driver`;
- testes unitários em `capi-session`;
- testes unitários em `capi-source`.

Os testes de CLI cobrem:

- `capic --help`;
- `capic -h`;
- `capic --version`;
- `capic -V`;
- arquivo inexistente;
- opção desconhecida;
- argumento extra.

A validação local equivalente à CI é executada por:

```bash
capi-lang/scripts/ci-local.sh
```

A CI está definida em:

```text
.github/workflows/capi-lang-ci.yml
```

---

## Referências

- Documento 25 — Testes, Validação e Conformidade
- Documento 27 — Bootstrap Plan e Arquitetura da Implementação Oficial
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial
- `TEST-STRATEGY.md`
- `BUILD-SYSTEM.md`
- `DEFINITION-OF-DONE.md`
