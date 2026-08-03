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
| R-002 | Crescimento do parser antes de HIR/semântica criar contratos instáveis. | Média | Médio | Monitorado | Manter AST estritamente sintática e adiar semântica para Stage 3+. |
| R-003 | Formato de dump de AST mudar sem intenção e quebrar tooling futuro. | Média | Médio | Monitorado | Proteger com snapshots golden e tratar mudanças como alteração contratual. |
| R-004 | Recuperação sintática atual ser confundida com recuperação sofisticada de IDE. | Média | Médio | Monitorado | Documentar limite do Stage 2 e reservar IDE/LSP para stage futuro. |
| R-005 | Política Unicode conservadora de identificadores exigir migração posterior. | Média | Médio | Aberto | Revisitar quando nomes Unicode forem normativamente definidos. |
| R-006 | Novas dependências externas serem introduzidas sem ADR ou revisão. | Baixa | Alto | Monitorado | Aplicar `DEPENDENCY-RULES.md`, `DEPENDENCIES.md` e `ADR-0013`. |
| R-007 | MSRV ser elevada acidentalmente por ferramenta, dependência ou código novo. | Média | Médio | Monitorado | Preservar MSRV `1.88.0` até decisão formal. |
| R-008 | Stages futuros de ownership e Domains exigirem ajustes no modelo de AST/HIR. | Média | Alto | Aberto | Definir HIR e lowering com rastreabilidade e semântica extensível. |

---

## 4. Riscos Mitigados

| ID | Risco | Mitigação aplicada |
| --- | --- |
| R-101 | Stage 0 não possuir validação reproduzível. | CI local, scripts e comandos canônicos definidos. |
| R-102 | Lexer aceitar entradas malformadas com panic. | Testes de entradas inválidas e recuperação léxica básica. |
| R-103 | AST não preservar spans suficientes para diagnósticos futuros. | Spans por nó relevante e testes granulares adicionados. |
| R-104 | Dump de AST não ser determinístico. | Implementação determinística e snapshots golden byte a byte. |

---

## 5. Riscos Por Próximo Stage

### Stage 3 — HIR e resolução de nomes

Riscos prioritários:

- definir HIR com acoplamento excessivo à AST;
- perder spans ou origem AST durante lowering;
- resolver nomes de forma dependente da ordem acidental de estruturas internas;
- emitir diagnósticos de resolução sem códigos estruturados;
- introduzir símbolos sem identidade estável.

Mitigações esperadas:

- preencher `HIR-MODEL.md`, `SYMBOL-MODEL.md`, `SCOPE-MODEL.md` e
  `NAME-RESOLUTION.md` antes da implementação;
- testar dumps determinísticos de HIR;
- testar resolução válida, duplicada, inexistente e ambígua;
- preservar rastreabilidade AST-HIR.

---

## 6. Critério de Atualização

Atualize este documento quando:

- um risco novo for identificado;
- uma mitigação for implementada;
- um risco mudar de probabilidade, impacto ou status;
- um stage for concluído;
- uma dívida técnica for aceita em `TECHNICAL-DEBT.md`;
- uma ADR reduzir ou aumentar risco de planejamento.
