# Risk Register

**Projeto:** Linguagem Capi  
**Documento:** RISK-REGISTER  
**Status:** Aprovado  
**Stage:** Stages 0-19 — Registro de riscos da implementação oficial  
**Natureza:** Registro operacional de riscos  
**Base normativa:** Documento 28 — Plano de Desenvolvimento da Implementação Oficial

---

## 1. Finalidade

Este documento registra riscos técnicos, operacionais e de escopo da
implementação oficial da Linguagem Capi.

Riscos não são dívidas técnicas por si só. Um risco descreve uma possibilidade
de impacto futuro. Dívidas aceitas ficam em `TECHNICAL-DEBT.md`.

---

## 2. Escala

### Probabilidade

- Baixa: improvável no stage atual.
- Média: plausível durante os próximos stages.
- Alta: provável sem ação explícita.

### Impacto

- Baixo: incômodo local ou retrabalho pequeno.
- Médio: afeta planejamento, testes ou várias áreas.
- Alto: pode bloquear stage, arquitetura ou compatibilidade.

### Status

- Aberto: risco ativo.
- Monitorado: risco conhecido com mitigação em andamento.
- Mitigado: risco reduzido por decisão, teste ou implementação.
- Fechado: risco não aplicável no estado atual.

---

## 3. Riscos Ativos

| ID | Risco | Probabilidade | Impacto | Status | Mitigação |
| --- | --- | --- | --- | --- | --- |
| R-001 | Divergência entre especificação, documentos de engenharia e implementação. | Média | Alto | Monitorado | Atualizar documentos bloqueantes e `FEATURE-STATUS.md` ao concluir cada entrega. |
| R-003 | Formato de dump de AST mudar sem intenção e quebrar tooling futuro. | Média | Médio | Monitorado | Proteger com snapshots golden e tratar mudanças como alteração contratual. |
| R-004 | Recuperação sintática atual ser confundida com recuperação sofisticada de IDE. | Média | Médio | Monitorado | Documentar limite do Stage 2 e reservar IDE/LSP para stage futuro. |
| R-005 | Política Unicode conservadora de identificadores exigir migração posterior. | Média | Médio | Aberto | Revisitar quando nomes Unicode forem normativamente definidos. |
| R-006 | Novas dependências externas serem introduzidas sem ADR ou revisão. | Baixa | Alto | Monitorado | Aplicar `DEPENDENCY-RULES.md`, `DEPENDENCIES.md` e `ADR-0013`. |
| R-007 | MSRV ser elevada acidentalmente por ferramenta, dependência ou código novo. | Média | Médio | Monitorado | Preservar MSRV `1.88.0` até decisão formal. |
| R-008 | Stages futuros de ownership e Domains exigirem ajustes no modelo de HIR. | Média | Alto | Aberto | Evoluir HIR preservando rastreabilidade, IDs e fronteiras entre `capi-hir`, `capi-lowering` e fases semânticas. |
| R-009 | Modelo de objetos exigir ajustes no sistema de tipos inicial. | Média | Alto | Monitorado | Registrar dívidas de `ObjectId<T>`, overload e generics em `TECHNICAL-DEBT.md`; evoluir Stage 5 preservando testes de Stage 4. |

---

## 4. Riscos Mitigados

| ID | Risco | Mitigação aplicada |
| --- | --- |
| R-101 | Stage 0 não possuir validação reproduzível. | CI local, scripts e comandos canônicos definidos. |
| R-102 | Lexer aceitar entradas malformadas com panic. | Testes de entradas inválidas e recuperação léxica básica. |
| R-103 | AST não preservar spans suficientes para diagnósticos futuros. | Spans por nó relevante e testes granulares adicionados. |
| R-104 | Dump de AST não ser determinístico. | Implementação determinística e snapshots golden byte a byte. |
| R-002 | Crescimento do parser antes de HIR/semântica criar contratos instáveis. | Stage 3 introduziu HIR, lowering, símbolos, escopos e resolução sem acoplar semântica à AST. |
| R-105 | HIR ficar acoplada diretamente à estrutura da AST. | `capi-hir` separado como modelo puro e lowering movido para `capi-lowering`. |
| R-106 | Resolução de nomes depender de ordem instável. | IDs internos, mapas ordenados e testes de determinismo adicionados. |
| R-107 | Erros de resolução sem diagnósticos estruturados. | Diagnósticos `SEM0001`, `SEM0002` e `SEM0003` testados com spans e labels. |
| R-108 | Inferência e type checking dependerem de ordem acidental ou emitirem diagnósticos instáveis. | `TypeInterner`, tabelas ordenadas, testes de determinismo, `capic check` e diagnósticos de tipo estruturados implementados no Stage 4. |
| R-109 | Coerções implícitas serem aceitas antes de regras normativas suficientes. | Stage 4 restringiu coerções ao upcast nominal do subconjunto e registrou lacunas de `ObjectId<T>`/overload em `TECHNICAL-DEBT.md`. |

---

## 5. Riscos Por Próximo Stage

### Stage 5 — Modelo de objetos

Riscos prioritários:

- modelo de objetos exigir mudanças incompatíveis no sistema de tipos inicial;
- `ObjectId<T>` público alterar regras de subtipagem e coerção;
- validação de overrides exigir resolução de membros mais completa;
- layout, vtables e despacho dinâmico anteciparem decisões de MIR/backend;
- hierarquias de classes e interfaces criarem ambiguidades não cobertas pelo
  overload simples atual.

Mitigações esperadas:

- preservar os testes de Stage 4 como regressão;
- remover ou reduzir dívidas `TD-S4-001`, `TD-S4-002` e `TD-S4-003` ao
  introduzir `ObjectId<T>` público;
- manter layout/despacho separados de decisões de backend até os stages de MIR
  e runtime;
- adicionar testes pequenos para overrides, herança, interfaces e identidade;
- atualizar `TECHNICAL-DEBT.md` quando uma limitação de objetos for aceita.

---

## 6. Critério de Atualização

Atualize este documento quando:

- um risco novo for identificado;
- uma mitigação for implementada;
- um risco mudar de probabilidade, impacto ou status;
- um stage for concluído;
- uma dívida técnica for aceita em `TECHNICAL-DEBT.md`;
- uma ADR reduzir ou aumentar risco de planejamento.
