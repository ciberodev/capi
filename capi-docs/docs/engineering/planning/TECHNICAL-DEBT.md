# Technical Debt

**Projeto:** Linguagem Capi  
**Documento:** TECHNICAL-DEBT  
**Status:** Aprovado  
**Stage:** Stages 0-19 — Registro de dívidas técnicas aceitas  
**Natureza:** Registro operacional de dívida técnica  
**Base normativa:** Documento 28 — Plano de Desenvolvimento da Implementação Oficial

---

## 1. Finalidade

Este documento registra dívidas técnicas aceitas na implementação oficial da
Linguagem Capi.

Uma dívida técnica é uma limitação conhecida que foi aceita conscientemente para
preservar progresso, reduzir escopo de um stage ou adiar uma solução completa
para um stage futuro.

Riscos sem decisão de aceite pertencem a `RISK-REGISTER.md`.

---

## 2. Política

Toda dívida técnica registrada deve conter:

- identificador estável;
- descrição objetiva;
- motivo do aceite;
- impacto esperado;
- stage ou condição de remoção;
- status.

Dívida técnica não pode ser usada para:

- esconder bug conhecido sem rastreabilidade;
- encerrar stage com critério bloqueante descumprido;
- contrariar especificação, ADR ou documento bloqueante;
- introduzir dependência externa sem revisão.

---

## 3. Estado Atual

```text
Dívidas técnicas bloqueantes: nenhuma
Dívidas técnicas aceitas: nenhuma
Itens monitorados: 3
```

Até a conclusão do Stage 3, não há dívida técnica aceita que bloqueie avanço do
projeto.

Os itens abaixo são limitações ou decisões conservadoras monitoradas. Eles não
invalidam os critérios de conclusão dos Stages 0, 1, 2 ou 3.

---

## 4. Itens Monitorados

| ID | Item | Motivo | Impacto | Condição de remoção | Status |
| --- | --- | --- | --- | --- | --- |
| TD-W-001 | Política Unicode de identificadores é conservadora. | Evitar aceitar regras não definidas normativamente. | Pode exigir ampliação futura do lexer. | Especificação definir política final de identificadores Unicode. | Monitorado |
| TD-W-002 | Recuperação sintática não é incremental nem sofisticada para IDE. | Stage 2 exige recuperação determinística básica, não LSP. | IDE futura exigirá estratégia própria. | Stage de ferramentas/LSP definir recuperação incremental. | Monitorado |
| TD-W-004 | Dumps de tokens, AST e HIR são formatos iniciais de engenharia. | Necessários para validação e snapshots antes de formatos externos completos. | Mudanças futuras exigem atualização intencional de snapshots. | `OUTPUT-FORMATS.md` e flags de dump estruturadas no stage apropriado. | Monitorado |

---

## 5. Dívidas Aceitas

Nenhuma dívida técnica aceita no estado atual.

Quando uma dívida for aceita, registre usando o formato:

```text
ID:
Descrição:
Motivo do aceite:
Impacto:
Stage/condição de remoção:
Status:
```

---

## 6. Dívidas Removidas

| ID | Item | Remoção |
| --- | --- | --- |
| TD-W-003 | Ausência temporária de lowering AST-HIR antes do Stage 3. | Removido pela implementação de `capi-lowering`, `capi-hir` e testes de lowering no Stage 3. |

---

## 7. Relação com Stages Concluídos

### Stage 0

Nenhuma dívida técnica bloqueante aceita.

### Stage 1

Nenhuma dívida técnica bloqueante aceita.

Itens monitorados:

- política Unicode conservadora;
- formato inicial de dump de tokens.

### Stage 2

Nenhuma dívida técnica bloqueante aceita.

Itens monitorados:

- recuperação sintática não incremental;
- formato inicial de dump de AST.

### Stage 3

Nenhuma dívida técnica bloqueante aceita.

Itens monitorados:

- formatos iniciais de dump de tokens, AST e HIR;
- módulos/imports implementados no subconjunto inicial, sem graph completo de
  módulos entre arquivos;
- resolução de nomes limitada ao subconjunto inicial, antes de tipagem e
  resolução completa de membros.

---

## 8. Critério de Atualização

Atualize este documento quando:

- uma limitação for aceita conscientemente como dívida técnica;
- uma dívida for removida;
- uma dívida mudar de impacto;
- um item monitorado virar dívida aceita;
- um stage for concluído com limitação documentada;
- `RISK-REGISTER.md` identificar risco que exija aceite explícito de dívida.
