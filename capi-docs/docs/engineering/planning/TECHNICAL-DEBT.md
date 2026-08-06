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
Dívidas técnicas aceitas: 7
Itens monitorados: 5
```

Até a conclusão do Stage 4, não há dívida técnica aceita que bloqueie avanço do
projeto no subconjunto implementado.

Os itens abaixo são limitações ou decisões conservadoras monitoradas. Eles não
invalidam os critérios de conclusão dos Stages 0, 1, 2, 3 ou 4, desde que
permaneçam fora do subconjunto declarado de cada stage.

---

## 4. Itens Monitorados

| ID | Item | Motivo | Impacto | Condição de remoção | Status |
| --- | --- | --- | --- | --- | --- |
| TD-W-001 | Política Unicode de identificadores é conservadora. | Evitar aceitar regras não definidas normativamente. | Pode exigir ampliação futura do lexer. | Especificação definir política final de identificadores Unicode. | Monitorado |
| TD-W-002 | Recuperação sintática não é incremental nem sofisticada para IDE. | Stage 2 exige recuperação determinística básica, não LSP. | IDE futura exigirá estratégia própria. | Stage de ferramentas/LSP definir recuperação incremental. | Monitorado |
| TD-W-004 | Dumps de tokens, AST e HIR são formatos iniciais de engenharia. | Necessários para validação e snapshots antes de formatos externos completos. | Mudanças futuras exigem atualização intencional de snapshots. | `OUTPUT-FORMATS.md` e flags de dump estruturadas no stage apropriado. | Monitorado |
| TD-W-005 | Imports e módulos entre arquivos permanecem no subconjunto inicial. | Stage 3 validou resolução em uma unidade, sem graph completo de módulos. | Programas multi-arquivo exigirão evolução do loader, sessão e resolução. | Stage de módulos/pacotes implementar graph de módulos e resolução entre arquivos. | Monitorado |
| TD-W-006 | Resolução completa de membros permanece dependente da tipagem e dispatch futuros. | Stage 3 resolveu nomes do subconjunto inicial; Stage 4 iniciou chamadas e membros simples. | Sobrecarga, dispatch e regras avançadas de membros ainda exigem expansão. | Stage de chamadas/dispatch completo implementar resolução final de membros. | Monitorado |

---

## 5. Dívidas Aceitas

| ID | Item | Motivo do aceite | Impacto | Condição de remoção | Status |
| --- | --- | --- | --- | --- | --- |
| TD-S4-001 | `ObjectId<Sub>` ainda não é aceito como `ObjectId<Super>`. | O Stage 4 implementou `ObjectId` internamente, mas ainda não há sintaxe pública/stdlib completa para exercitar a regra de forma normativa. | Subtipagem de identidade lógica ainda fica limitada fora de testes públicos e programas reais. | Expor/modelar `ObjectId<T>` no frontend/type checker e adicionar teste de upcast `ObjectId<Sub> -> ObjectId<Super>`. | Aceita |
| TD-S4-002 | `ObjectId<Super>` ainda não é rejeitado explicitamente como `ObjectId<Sub>` implícito em programa público. | Mesma limitação de superfície pública de `ObjectId<T>`; o checker não possui caso observável completo para downcast de `ObjectId`. | Downcast implícito de identidade lógica não possui diagnóstico testável no subset atual. | Expor/modelar `ObjectId<T>` e adicionar teste compile-fail para `ObjectId<Super> -> ObjectId<Sub>`. | Aceita |
| TD-S4-003 | Conversão entre `ObjectId<T>` e inteiro ainda não possui teste/diagnóstico público específico. | `ObjectId<T>` não está disponível como tipo escrito pelo usuário no subset atual. | Regras de separação entre identidade lógica e representação numérica ainda não são demonstráveis por `capic check`. | Implementar superfície pública de `ObjectId<T>` e teste rejeitando conversão nos dois sentidos entre inteiro e `ObjectId<T>`. | Aceita |
| TD-S4-004 | Ambiguidade de overload por coerções ainda não é diagnosticada. | O Stage 4 implementa resolução de chamada aplicável simples, sem sistema completo de overload. | Programas que dependerem de múltiplos candidatos aplicáveis por coerção ainda não têm diagnóstico de ambiguidade. | Implementar tabela de overload, ranking de candidatos, detecção de empate e diagnóstico específico. | Aceita |
| TD-S4-005 | Bounds, inferência e substituição genérica de chamadas permanecem parciais. | O subset inicial cobre declaração, aridade, instanciação e invariância; bounds e substituição exigem modelo adicional de constraints. | Chamadas genéricas com inferência de `T`, bounds satisfeitos/não satisfeitos e retorno substituído ainda não são suportados. | Implementar `GenericChecker` completo, constraints, substituição recursiva e testes de inferência/conflito/bounds. | Aceita |
| TD-S4-006 | `Optional<T>` e `Result<T, E>` ainda não estão modelados como tipos padrão reais. | Não há stdlib nem definição canônica desses tipos no subset atual. | Testes normativos de `Optional`/`Result` ficam limitados a classes genéricas artificiais e não validam semântica de ausência/falha esperada. | Introduzir modelos canônicos de `Optional` e `Result`, aridade, invariância e interning específico. | Aceita |
| TD-S4-007 | Dump tipado formal ainda não existe. | Stage 4 expõe tabelas internas em testes, mas não define formato textual estável de typed HIR/types. | Critérios de determinismo de dump tipado ainda não são demonstráveis por snapshot. | Definir formato de dump tipado, flag de CLI quando aplicável e snapshots determinísticos. | Aceita |

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

### Stage 4

Nenhuma dívida técnica bloqueante aceita para o subconjunto implementado.

Dívidas aceitas:

- `ObjectId<T>` possui representação interna, mas ainda não possui superfície
  pública suficiente para validar upcast, downcast e conversões proibidas em
  programas Capi;
- resolução de overload por múltiplos candidatos e coerções ainda não existe;
- generics cobrem o subconjunto inicial, mas não bounds, inferência de chamada
  nem substituição completa;
- `Optional<T>` e `Result<T, E>` ainda dependem de modelo canônico/stdlib;
- dump tipado formal ainda não foi definido.

---

## 8. Critério de Atualização

Atualize este documento quando:

- uma limitação for aceita conscientemente como dívida técnica;
- uma dívida for removida;
- uma dívida mudar de impacto;
- um item monitorado virar dívida aceita;
- um stage for concluído com limitação documentada;
- `RISK-REGISTER.md` identificar risco que exija aceite explícito de dívida.
