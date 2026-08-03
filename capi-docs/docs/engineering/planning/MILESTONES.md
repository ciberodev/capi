# Milestones

**Projeto:** Linguagem Capi  
**Documento:** MILESTONES  
**Status:** Aprovado  
**Stage:** Stages 0-19 — Marcos demonstráveis da implementação oficial  
**Natureza:** Registro de planejamento derivado  
**Base normativa:** Documento 28 — Plano de Desenvolvimento da Implementação Oficial

---

## 1. Finalidade

Este documento registra os marcos demonstráveis da implementação oficial da
Linguagem Capi.

Milestones não substituem stages. Um stage define uma etapa de implementação; um
milestone define uma evidência observável de progresso, revisável e
reproduzível.

---

## 2. Estado Atual

```text
Último milestone concluído: M2 — AST sintática inicial
Stage correspondente: Stage 2 — Parser e AST
Próximo milestone planejado: M3 — HIR e resolução de nomes
```

---

## 3. Milestones Principais

| Milestone | Stage | Status | Evidência demonstrável |
| --- | --- | --- | --- |
| M0 — Fundação executável | Stage 0 | Concluído | `capic --help`, `capic --version`, `cargo test` |
| M1 — Tokens e diagnósticos léxicos | Stage 1 | Concluído | `capic --emit tokens arquivo.capi` |
| M2 — AST sintática inicial | Stage 2 | Concluído | `capic --emit ast arquivo.capi` |
| M3 — HIR e nomes resolvidos | Stage 3 | Planejado | `capic --emit hir arquivo.capi` |
| M4 — Checagem semântica inicial | Stage 4 | Planejado | `capic check arquivo.capi` |
| M5 — Objetos semanticamente validados | Stage 5 | Planejado | Hierarquias, overrides e identidade validados |
| M6 — Segurança de memória inicial | Stage 6 | Planejado | Moves, borrows, regiões e escapes validados |
| M7 — Domains operacionais | Stage 7 | Planejado | Domains criados, associados e descartados deterministicamente |
| M8 — MIR validada | Stage 8 | Planejado | `capic --emit mir arquivo.capi` |
| M9 — Runtime e ABI mínimos | Stage 9 | Planejado | Contratos executáveis mínimos entre MIR, runtime e backend |
| M10 — Biblioteca padrão mínima | Stage 10 | Planejado | Programas básicos usando APIs padrão mínimas |
| M11 — Primeiro executável nativo | Stage 11 | Planejado | `capic build arquivo.capi` e execução do binário |
| M12 — Toolchain de projeto | Stage 12 | Planejado | `capi new`, `capi build`, `capi run`, `capi test` |
| M13 — Dependências reproduzíveis | Stage 13 | Planejado | Manifesto, lockfile e resolução determinística |
| M14 — Ferramentas de desenvolvimento | Stage 14 | Planejado | `capi doc`, LSP inicial e saída estruturada |
| M15 — Conformidade mensurável | Stage 15 | Planejado | Suíte de conformidade, fuzzing e benchmarks |
| M16 — Bootstrap reproduzível | Stage 16 | Planejado | Build por estágios e artefatos comparáveis |
| M17 — Backend LLVM compatível | Stage 17 | Planejado | Seleção de backend e comparação com Cranelift |
| M18 — Auto-hospedagem | Stage 18 | Planejado | Compilador em Capi compila versão equivalente |
| M19 — Versão 1.0 | Stage 19 | Planejado | Release candidate e artefatos distribuíveis |

---

## 4. Milestones Concluídos

### M0 — Fundação executável

Status: Concluído.

Evidências:

- workspace Cargo criado;
- CLI `capic` criada;
- crates fundamentais criados;
- CI e validação local configuradas;
- `capic --help` validado;
- `capic --version` validado;
- erro de arquivo inexistente validado.

### M1 — Tokens e diagnósticos léxicos

Status: Concluído.

Evidências:

- `SourceMap`, `Span`, linha e coluna implementados;
- diagnósticos estruturados implementados;
- lexer implementado;
- tokens do subconjunto inicial reconhecidos;
- entradas inválidas produzem diagnósticos léxicos;
- `capic --emit tokens arquivo.capi` validado.

### M2 — AST sintática inicial

Status: Concluído.

Evidências:

- `capi-ast` criado;
- `capi-parser` criado;
- parser do subconjunto sintático inicial implementado;
- AST preserva spans;
- recuperação sintática produz AST parcial;
- dump da AST é determinístico;
- `capic --emit ast arquivo.capi` validado.

---

## 5. Próximo Milestone

### M3 — HIR e nomes resolvidos

Status: Planejado.

Objetivo:

- baixar AST para HIR;
- introduzir IDs internos;
- criar símbolos e escopos;
- resolver módulos, imports e nomes;
- emitir diagnósticos de resolução;
- produzir dump determinístico da HIR.

Evidência esperada:

```bash
capic --emit hir arquivo.capi
```

Documentos que devem estar aprovados antes do encerramento:

- `compiler/semantic/HIR-MODEL.md`;
- `compiler/semantic/SYMBOL-MODEL.md`;
- `compiler/semantic/SCOPE-MODEL.md`;
- `compiler/semantic/NAME-RESOLUTION.md`;
- `compiler/frontend/AST-LOWERING.md`;
- `testing/SEMANTIC-TESTS.md`.

---

## 6. Critério de Atualização

Atualize este documento quando:

- um stage mudar de status;
- um resultado demonstrável for implementado;
- uma evidência de milestone mudar;
- um milestone for dividido, consolidado ou reordenado;
- `FEATURE-STATUS.md` registrar conclusão, bloqueio ou retomada de stage.
