# ADRs — Architecture Decision Records

Este diretório contém os registros de decisões arquiteturais da Linguagem Capi.

ADRs registram decisões relevantes para a especificação, implementação oficial, arquitetura do compilador, runtime, toolchain, build, testes, dependências e evolução do projeto.

---

## Status usados

```text
Proposto
Aprovado
Substituído
Rejeitado
Obsoleto
```

Uma ADR aprovada passa a ser referência decisória para implementação e documentação.

Quando uma decisão mudar, a ADR original não deve ser apagada. Deve ser atualizada com relação explícita para a nova ADR ou marcada como substituída.

---

## ADRs aprovadas para o Stage 0

As ADRs abaixo são obrigatórias para o Stage 0 conforme o Documento 28 e estão aprovadas.

| ADR | Decisão | Status |
| --- | --- | --- |
| [`ADR-0001 — Rust como Linguagem da Implementação Oficial`](ADR-0001%20%E2%80%94%20Rust%20como%20Linguagem%20da%20Implementa%C3%A7%C3%A3o%20Oficial.md) | Usar Rust como linguagem hospedeira inicial da implementação oficial. | Aprovado |
| [`ADR-0002 — Organização da Implementação em Workspace Cargo`](ADR-0002%20%E2%80%94%20Organiza%C3%A7%C3%A3o%20da%20Implementa%C3%A7%C3%A3o%20em%20Workspace%20Cargo.md) | Organizar a implementação inicial como workspace Cargo em `capi-lang/`. | Aprovado |
| [`ADR-0003 — Separação entre Frontend, Middle-end e Backend`](ADR-0003%20%E2%80%94%20Separa%C3%A7%C3%A3o%20entre%20Frontend%2C%20Middle-end%20e%20Backend.md) | Separar o compilador em camadas, com MIR independente de backend. | Aprovado |
| [`ADR-0013 — Política de Dependências Externas`](ADR-0013%20%E2%80%94%20Pol%C3%ADtica%20de%20Depend%C3%AAncias%20Externas.md) | Adotar política restritiva e explícita para dependências externas. | Aprovado |
| [`ADR-0015 — Estratégia Inicial de Testes`](ADR-0015%20%E2%80%94%20Estrat%C3%A9gia%20Inicial%20de%20Testes.md) | Adotar testes automatizados desde o Stage 0. | Aprovado |
| [`ADR-0016 — Organização Física do Repositório`](ADR-0016%20%E2%80%94%20Organiza%C3%A7%C3%A3o%20F%C3%ADsica%20do%20Reposit%C3%B3rio.md) | Organizar o repositório raiz em `capi-docs/`, `capi-lang/` e infraestrutura de repositório. | Aprovado |

---

## ADRs reservadas para stages posteriores

Os arquivos abaixo existem como slots planejados, mas ainda não possuem conteúdo decisório aprovado.

| ADR | Tema | Status |
| --- | --- | --- |
| [`ADR-0004 — AST e HIR como Representações Distintas`](ADR-0004%20%E2%80%94%20AST%20e%20HIR%20como%20Representa%C3%A7%C3%B5es%20Distintas.md) | Fronteira entre AST e HIR. | Reservado |
| [`ADR-0005 — MIR Independente de Backend`](ADR-0005%20%E2%80%94%20MIR%20Independente%20de%20Backend.md) | Independência da MIR em relação a Cranelift, LLVM e plataforma. | Reservado |
| [`ADR-0006 — Cranelift como Backend Inicial`](ADR-0006%20%E2%80%94%20Cranelift%20como%20Backend%20Inicial.md) | Backend inicial da implementação oficial. | Reservado |
| [`ADR-0007 — LLVM como Backend de Otimização`](ADR-0007%20%E2%80%94%20LLVM%20como%20Backend%20de%20Otimiza%C3%A7%C3%A3o.md) | Backend LLVM posterior. | Reservado |
| [`ADR-0008 — Desenvolvimento por Fatias Verticais`](ADR-0008%20%E2%80%94%20Desenvolvimento%20por%20Fatias%20Verticais.md) | Estratégia de evolução incremental por fatias demonstráveis. | Reservado |
| [`ADR-0009 — Runtime Mínimo e Ausência de Máquina Virtual`](ADR-0009%20%E2%80%94%20Runtime%20M%C3%ADnimo%20e%20Aus%C3%AAncia%20de%20M%C3%A1quina%20Virtual.md) | Runtime mínimo e ausência de VM obrigatória. | Reservado |
| [`ADR-0010 — IDs Tipados para Entidades Internas`](ADR-0010%20%E2%80%94%20IDs%20Tipados%20para%20Entidades%20Internas.md) | Identificadores internos tipados. | Reservado |
| [`ADR-0011 — Interning de Símbolos e Tipos`](ADR-0011%20%E2%80%94%20Interning%20de%20S%C3%ADmbolos%20e%20Tipos.md) | Estratégia de interning. | Reservado |
| [`ADR-0012 — Diagnósticos Estruturados`](ADR-0012%20%E2%80%94%20Diagn%C3%B3sticos%20Estruturados.md) | Modelo estruturado de diagnósticos. | Reservado |
| [`ADR-0014 — Política para Código Unsafe`](ADR-0014%20%E2%80%94%20Pol%C3%ADtica%20para%20C%C3%B3digo%20Unsafe.md) | Política detalhada para uso de `unsafe`. | Reservado |

---

## Template

Novas ADRs devem seguir:

```text
ADR-TEMPLATE.md
```

Arquivo:

- [`ADR-TEMPLATE.md`](ADR-TEMPLATE.md)

---

## Relação com o Stage 0

O Stage 0 está registrado como concluído em:

- [`FEATURE-STATUS.md`](../engineering/planning/FEATURE-STATUS.md)

As ADRs aprovadas do Stage 0 registram as decisões mínimas necessárias para:

- usar Rust como linguagem hospedeira inicial;
- criar o workspace Cargo em `capi-lang/`;
- preservar separação entre camadas do compilador;
- controlar dependências externas;
- validar a implementação com testes desde o início;
- manter separação física entre documentação, implementação e infraestrutura do repositório.
