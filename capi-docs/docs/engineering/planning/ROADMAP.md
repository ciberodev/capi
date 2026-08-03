# Roadmap

**Projeto:** Linguagem Capi  
**Documento:** ROADMAP  
**Status:** Aprovado  
**Stage:** Stages 0-19 — Roadmap operacional da implementação oficial  
**Natureza:** Documento de planejamento derivado  
**Base normativa:** Documento 28 — Plano de Desenvolvimento da Implementação Oficial

---

## 1. Finalidade

Este documento consolida a visão de evolução da implementação oficial da
Linguagem Capi.

O roadmap organiza os stages em horizontes de entrega. Ele não altera a ordem
normativa do Documento 28 e não redefine critérios de conclusão.

---

## 2. Estado Atual

```text
Horizonte atual: Frontend inicial concluído
Último stage concluído: Stage 2 — Parser e AST
Próximo foco: Stage 3 — HIR e resolução de nomes
```

---

## 3. Horizontes do Roadmap

| Horizonte | Stages | Status | Resultado |
| --- | --- | --- | --- |
| H0 — Fundação | Stage 0 | Concluído | Workspace, CI, CLI mínima e critérios de aceite. |
| H1 — Frontend inicial | Stages 1-2 | Concluído | Fontes, diagnósticos, lexer, parser e AST. |
| H2 — Semântica inicial | Stages 3-5 | Planejado | HIR, nomes, tipos e modelo de objetos. |
| H3 — Segurança de memória | Stages 6-7 | Planejado | Ownership, borrowing, regiões e Domains. |
| H4 — IR e execução mínima | Stages 8-11 | Planejado | MIR, runtime, ABI, biblioteca mínima e Cranelift. |
| H5 — Toolchain e ecossistema | Stages 12-14 | Planejado | Projetos, pacotes, documentação e ferramentas. |
| H6 — Qualidade, bootstrap e release | Stages 15-19 | Planejado | Conformidade, bootstrap, LLVM, auto-hospedagem e 1.0. |

---

## 4. Horizonte H0 — Fundação

Status: Concluído.

Entregas:

- workspace Cargo;
- crates iniciais;
- CLI `capic`;
- CI;
- validação local;
- ADRs iniciais;
- definição de pronto.

Resultado:

```bash
capic --help
capic --version
cargo test
```

---

## 5. Horizonte H1 — Frontend Inicial

Status: Concluído.

Entregas:

- fontes e spans;
- diagnósticos estruturados;
- lexer;
- tokens;
- parser;
- AST;
- recuperação sintática;
- dumps determinísticos de tokens e AST.

Resultados:

```bash
capic --emit tokens arquivo.capi
capic --emit ast arquivo.capi
```

---

## 6. Horizonte H2 — Semântica Inicial

Status: Planejado.

Stages:

- Stage 3 — HIR e resolução de nomes;
- Stage 4 — Sistema de tipos;
- Stage 5 — Modelo de objetos.

Resultados esperados:

- HIR determinística;
- símbolos e escopos;
- resolução de nomes;
- inferência e checagem de tipos;
- generics iniciais;
- classes e hierarquias validadas.

Comandos esperados:

```bash
capic --emit hir arquivo.capi
capic check arquivo.capi
```

---

## 7. Horizonte H3 — Segurança de Memória

Status: Planejado.

Stages:

- Stage 6 — Ownership, borrowing e regiões;
- Stage 7 — Domains.

Resultados esperados:

- moves e borrows validados;
- regiões implementadas;
- escapes inválidos rejeitados;
- Domains criados e descartados deterministicamente;
- integração entre Domains e ownership.

---

## 8. Horizonte H4 — IR e Execução Mínima

Status: Planejado.

Stages:

- Stage 8 — MIR;
- Stage 9 — ABI, runtime mínimo e codegen básico;
- Stage 10 — Biblioteca padrão mínima;
- Stage 11 — Backend Cranelift.

Resultados esperados:

- MIR validada;
- runtime mínimo;
- ABI interna definida;
- biblioteca padrão mínima;
- geração de executáveis nativos.

Comandos esperados:

```bash
capic --emit mir arquivo.capi
capic build arquivo.capi
```

---

## 9. Horizonte H5 — Toolchain e Ecossistema

Status: Planejado.

Stages:

- Stage 12 — Toolchain mínima;
- Stage 13 — Pacotes e dependências;
- Stage 14 — Ferramentas de desenvolvimento.

Resultados esperados:

- criação de projetos;
- build, run e test;
- manifesto e lockfile;
- resolução determinística de dependências;
- documentação de APIs;
- LSP inicial;
- saída estruturada.

Comandos esperados:

```bash
capi new
capi build
capi run
capi test
capi doc
```

---

## 10. Horizonte H6 — Qualidade, Bootstrap e Release

Status: Planejado.

Stages:

- Stage 15 — Conformidade, robustez e performance;
- Stage 16 — Bootstrap;
- Stage 17 — Backend LLVM;
- Stage 18 — Auto-hospedagem;
- Stage 19 — Preparação da versão 1.0.

Resultados esperados:

- suíte de conformidade;
- fuzzing e benchmarks;
- bootstrap reproduzível;
- backend LLVM;
- auto-hospedagem;
- artefatos assinados;
- versão 1.0 distribuível.

---

## 11. Próximo Foco

O próximo foco operacional é:

```text
Stage 3 — HIR e resolução de nomes
```

O trabalho deve começar pela documentação obrigatória:

- `compiler/semantic/HIR-MODEL.md`;
- `compiler/semantic/SYMBOL-MODEL.md`;
- `compiler/semantic/SCOPE-MODEL.md`;
- `compiler/semantic/NAME-RESOLUTION.md`;
- `testing/SEMANTIC-TESTS.md`.

---

## 12. Critério de Atualização

Atualize este documento quando:

- um stage mudar de status;
- um horizonte for concluído;
- um comando demonstrável novo for implementado;
- o Documento 28 for alterado;
- `IMPLEMENTATION-ORDER.md` mudar a ordem operacional;
- `FEATURE-STATUS.md` registrar novo progresso.
