# Architecture

Esta pasta reúne a documentação de arquitetura de engenharia da implementação oficial da Linguagem Capi.

Os documentos daqui não redefinem a linguagem. Eles traduzem a especificação e as ADRs em fronteiras de implementação, responsabilidades de crates, direção de dependências e estrutura inicial do pipeline.

---

## Documentos aprovados do Stage 0

Estes documentos orientaram a criação do workspace Rust em `capi-lang` e dos crates fundamentais do Stage 0.

| Documento | Papel | Status |
| --- | --- | --- |
| [`COMPILER-ARCHITECTURE.md`](COMPILER-ARCHITECTURE.md) | Define a arquitetura de engenharia do compilador oficial. | Aprovado |
| [`WORKSPACE-ARCHITECTURE.md`](WORKSPACE-ARCHITECTURE.md) | Define a arquitetura inicial do workspace Rust em `capi-lang`. | Aprovado |
| [`COMPONENT-RESPONSIBILITIES.md`](COMPONENT-RESPONSIBILITIES.md) | Define responsabilidades dos componentes e crates. | Aprovado |
| [`DEPENDENCY-RULES.md`](DEPENDENCY-RULES.md) | Define regras de dependência entre crates, camadas e bibliotecas externas. | Aprovado |
| [`COMPILATION-PIPELINE.md`](COMPILATION-PIPELINE.md) | Consolida o fluxo conceitual e operacional do pipeline de compilação. | Aprovado |

---

## Documentos reservados

Os documentos abaixo existem como slots planejados para detalhamento posterior. Eles ainda não fazem parte da base aprovada do Stage 0.

| Documento | Tema | Status |
| --- | --- | --- |
| [`COMPILATION-SESSION.md`](COMPILATION-SESSION.md) | Modelo detalhado da sessão de compilação. | Reservado |
| [`DATA-FLOW.md`](DATA-FLOW.md) | Fluxo de dados entre fases do compilador. | Reservado |
| [`ERROR-HANDLING.md`](ERROR-HANDLING.md) | Política arquitetural detalhada para erros de usuário, ambiente e erros internos. | Reservado |
| [`INCREMENTAL-COMPILATION.md`](INCREMENTAL-COMPILATION.md) | Arquitetura futura de compilação incremental. | Reservado |
| [`PARALLELISM.md`](PARALLELISM.md) | Estratégia futura de paralelismo no compilador. | Reservado |

---

## Ordem de leitura recomendada

Para entender a arquitetura inicial:

1. [`COMPILER-ARCHITECTURE.md`](COMPILER-ARCHITECTURE.md)
2. [`WORKSPACE-ARCHITECTURE.md`](WORKSPACE-ARCHITECTURE.md)
3. [`COMPONENT-RESPONSIBILITIES.md`](COMPONENT-RESPONSIBILITIES.md)
4. [`DEPENDENCY-RULES.md`](DEPENDENCY-RULES.md)
5. [`COMPILATION-PIPELINE.md`](COMPILATION-PIPELINE.md)

Para implementar novos crates ou fases, leia primeiro os documentos acima e depois consulte os documentos específicos da área em `engineering/compiler/`, `engineering/runtime/`, `engineering/toolchain/` ou `engineering/testing/`.

---

## Relação com `capi-lang`

O Stage 0 criou em `capi-lang` os crates fundamentais:

```text
capi-cli
capi-driver
capi-session
capi-diagnostics
capi-source
capi-common
```

Esses crates seguem as fronteiras definidas nesta pasta:

- `capi-cli` fornece o executável `capic`;
- `capi-driver` coordena a ação solicitada e inicializa a sessão;
- `capi-session` representa o contexto de uma invocação do compilador;
- `capi-diagnostics` estrutura diagnósticos e erros internos;
- `capi-source` gerencia fontes carregadas;
- `capi-common` concentra primitivas compartilhadas pequenas.

O grafo inicial é validado por:

```bash
cd capi-lang
scripts/deps.sh
```

---

## Relação com ADRs

As principais ADRs relacionadas são:

- [`ADR-0002 — Organização da Implementação em Workspace Cargo`](../../adr/ADR-0002%20%E2%80%94%20Organiza%C3%A7%C3%A3o%20da%20Implementa%C3%A7%C3%A3o%20em%20Workspace%20Cargo.md)
- [`ADR-0003 — Separação entre Frontend, Middle-end e Backend`](../../adr/ADR-0003%20%E2%80%94%20Separa%C3%A7%C3%A3o%20entre%20Frontend%2C%20Middle-end%20e%20Backend.md)
- [`ADR-0013 — Política de Dependências Externas`](../../adr/ADR-0013%20%E2%80%94%20Pol%C3%ADtica%20de%20Depend%C3%AAncias%20Externas.md)
- [`ADR-0016 — Organização Física do Repositório`](../../adr/ADR-0016%20%E2%80%94%20Organiza%C3%A7%C3%A3o%20F%C3%ADsica%20do%20Reposit%C3%B3rio.md)

---

## Relação com o Stage 0

O Stage 0 está registrado como concluído em:

- [`FEATURE-STATUS.md`](../planning/FEATURE-STATUS.md)

A validação mínima relacionada à arquitetura é:

```bash
cd capi-lang
cargo build --workspace --locked
cargo test --workspace --locked
scripts/deps.sh
```
