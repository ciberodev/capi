# Implementation Order

**Projeto:** Linguagem Capi  
**Documento:** IMPLEMENTATION-ORDER  
**Status:** Aprovado  
**Stage:** Stages 0-19 — Ordem operacional da implementação oficial  
**Natureza:** Documento de planejamento derivado  
**Base normativa:** Documento 28 — Plano de Desenvolvimento da Implementação Oficial

---

## 1. Finalidade

Este documento consolida a ordem operacional de implementação da implementação
oficial da Linguagem Capi.

Ele existe para responder, de forma direta, qual é a sequência esperada de
stages, quais entregas cada stage destrava e quais resultados demonstráveis
devem existir ao final de cada etapa.

Este documento não substitui o Documento 28. Quando houver conflito, prevalecem:

1. especificação normativa;
2. Documento 28;
3. ADRs aprovadas;
4. documentos de engenharia bloqueantes do stage atual;
5. este registro operacional.

---

## 2. Estado Atual

```text
Último stage concluído: Stage 2 — Parser e AST
Próximo stage planejado: Stage 3 — HIR e resolução de nomes
Registro de progresso: FEATURE-STATUS.md
```

Stages concluídos:

- Stage 0 — Fundação do projeto;
- Stage 1 — Fontes, diagnósticos e lexer;
- Stage 2 — Parser e AST.

Stages planejados:

- Stage 3 a Stage 19.

---

## 3. Regras de Execução

A ordem dos stages deve respeitar as dependências naturais entre fases do
compilador:

- frontend léxico antes de frontend sintático;
- AST antes de HIR;
- resolução de nomes antes de checagem de tipos;
- tipos antes de ownership e MIR executável;
- MIR antes de codegen;
- runtime e ABI antes de biblioteca padrão executável;
- backend antes de toolchain de execução;
- conformidade antes de bootstrap e versão 1.0.

Entregas antecipadas são permitidas apenas quando:

- não quebram o contrato do stage atual;
- não tornam obrigatória uma fase futura;
- não introduzem dependência incompatível;
- permanecem documentadas como parciais ou preparatórias.

Um stage só deve ser marcado como concluído quando os critérios aplicáveis em
`DEFINITION-OF-DONE.md` e `FEATURE-STATUS.md` estiverem satisfeitos.

---

## 4. Ordem Completa dos Stages

| Ordem | Stage | Status | Objetivo principal | Resultado demonstrável |
| --- | --- | --- | --- | --- |
| 0 | Fundação do projeto | Concluído | Criar workspace, crates iniciais, CI, padrões e CLI mínima. | `capic --help`, `capic --version`, `cargo test` |
| 1 | Fontes, diagnósticos e lexer | Concluído | Ler fontes, mapear spans, emitir diagnósticos e tokens. | `capic --emit tokens arquivo.capi` |
| 2 | Parser e AST | Concluído | Transformar tokens em AST com spans, diagnósticos e recuperação. | `capic --emit ast arquivo.capi` |
| 3 | HIR e resolução de nomes | Planejado | Baixar AST para HIR e resolver símbolos, módulos, imports e escopos. | `capic --emit hir arquivo.capi` |
| 4 | Sistema de tipos | Planejado | Implementar tipos internos, inferência, verificação, coerções e generics. | `capic check arquivo.capi` |
| 5 | Modelo de objetos | Planejado | Implementar classes, identidade, herança, override, layout e despacho. | Checagem válida de hierarquias e objetos |
| 6 | Ownership, borrowing e regiões | Planejado | Implementar regras fundamentais de segurança de memória. | Rejeição de moves, borrows e escapes inválidos |
| 7 | Domains | Planejado | Implementar semântica de Domains, associação e descarte determinístico. | Diagnósticos e validação de Domains |
| 8 | MIR | Planejado | Implementar IR intermediária, lowering, validação, passes e dump. | `capic --emit mir arquivo.capi` |
| 9 | ABI, runtime mínimo e codegen básico | Planejado | Definir contratos executáveis entre MIR, backend e runtime. | Runtime mínimo e interface de backend operacional |
| 10 | Biblioteca padrão mínima | Planejado | Implementar núcleo mínimo da biblioteca padrão. | Programas básicos usando biblioteca padrão |
| 11 | Backend Cranelift | Planejado | Gerar executáveis nativos via Cranelift. | `capic build arquivo.capi` e execução do binário |
| 12 | Toolchain mínima | Planejado | Criar, compilar, executar, testar e formatar projetos Capi. | `capi new`, `capi build`, `capi run`, `capi test` |
| 13 | Pacotes e dependências | Planejado | Implementar manifesto completo, lockfile e resolução de dependências. | Resolução determinística de pacotes |
| 14 | Ferramentas de desenvolvimento | Planejado | Implementar documentação, LSP, JSON e flags estruturadas de dump. | `capi doc` e diagnósticos consumíveis por ferramentas |
| 15 | Conformidade, robustez e performance | Planejado | Consolidar suíte de conformidade, fuzzing, benchmarks e hardening. | Suíte de conformidade e métricas de performance |
| 16 | Bootstrap | Planejado | Tornar o processo de bootstrap executável, reproduzível e auditável. | Build por estágios e artefatos verificáveis |
| 17 | Backend LLVM | Planejado | Implementar LLVM como backend de otimização e comparar com Cranelift. | Seleção de backend e compatibilidade validada |
| 18 | Auto-hospedagem | Planejado | Permitir que a implementação em Capi compile uma versão equivalente. | Cadeia de bootstrap auto-hospedada validada |
| 19 | Preparação da versão 1.0 | Planejado | Estabilizar implementação, documentação, artefatos e release. | Release candidate e versão 1.0 distribuível |

---

## 5. Stages em Detalhe

### Stage 0 — Fundação do projeto

Status: Concluído.

Objetivo:

- preparar infraestrutura mínima da implementação oficial;
- criar workspace Cargo;
- criar crates iniciais;
- configurar build, testes, lint, CI e CLI mínima.

Entregas principais:

- `capic`;
- `capi-driver`;
- `capi-session`;
- `capi-diagnostics`;
- `capi-source`;
- `capic --help`;
- `capic --version`.

Resultado demonstrável:

```bash
capic --help
capic --version
cargo test
```

### Stage 1 — Fontes, diagnósticos e lexer

Status: Concluído.

Objetivo:

- implementar infraestrutura de fonte;
- implementar spans e localizações;
- implementar diagnósticos estruturados;
- implementar tokens e lexer.

Entregas principais:

- `SourceId`, `SourceFile`, `SourceMap`, `Span`;
- linha e coluna;
- diagnósticos estruturados;
- modelo de tokens;
- lexer;
- dump de tokens.

Resultado demonstrável:

```bash
capic --emit tokens arquivo.capi
```

### Stage 2 — Parser e AST

Status: Concluído.

Objetivo:

- implementar parser;
- implementar AST;
- preservar spans;
- emitir diagnósticos sintáticos;
- recuperar erros recuperáveis;
- produzir dump determinístico da AST.

Entregas principais:

- `capi-ast`;
- `capi-parser`;
- AST de unidade, módulos, imports, declarações, tipos, comandos e expressões;
- parser do subconjunto sintático inicial;
- precedência de operadores;
- recuperação sintática;
- `capic --emit ast`.

Resultado demonstrável:

```bash
capic --emit ast arquivo.capi
```

### Stage 3 — HIR e resolução de nomes

Status: Planejado.

Objetivo:

- implementar HIR;
- baixar AST para HIR;
- implementar IDs internos, símbolos, escopos, módulos e imports;
- resolver nomes e detectar ambiguidades.

Entregas principais:

- lowering efetivo de AST para HIR;
- tabelas de símbolos;
- escopos;
- resolução de nomes;
- diagnósticos de símbolos duplicados, inexistentes e ambíguos;
- dump de HIR.

Resultado esperado:

```bash
capic --emit hir arquivo.capi
```

### Stage 4 — Sistema de tipos

Status: Planejado.

Objetivo:

- implementar representação interna de tipos;
- implementar interning;
- implementar inferência e verificação;
- implementar subtipagem, coerções, chamadas e generics iniciais.

Entregas principais:

- tipos internos;
- inferência;
- checagem de tipos;
- coerções;
- resolução de chamadas;
- diagnósticos de tipo.

Resultado esperado:

```bash
capic check arquivo.capi
```

### Stage 5 — Modelo de objetos

Status: Planejado.

Objetivo:

- implementar classes, identidade, herança e subtipagem nominal;
- preparar layout, vtables e despacho dinâmico.

Entregas principais:

- representação de classes;
- campos, métodos e construtores;
- validação de hierarquias;
- override;
- identidade de objetos;
- layout e vtables iniciais.

### Stage 6 — Ownership, borrowing e regiões

Status: Planejado.

Objetivo:

- implementar garantias fundamentais de segurança de memória próprias da Capi.

Entregas principais:

- places;
- moves e cópias;
- empréstimos compartilhados e mutáveis;
- regiões;
- descarte;
- análise de escape;
- diagnósticos de memória.

### Stage 7 — Domains

Status: Planejado.

Objetivo:

- implementar semântica e infraestrutura de Domains.

Entregas principais:

- tipos de Domain;
- criação e descarte de Domain;
- associação de objetos;
- restrições de escape;
- integração com ownership;
- modelo inicial de alocação.

### Stage 8 — MIR

Status: Planejado.

Objetivo:

- implementar representação intermediária independente de backend.

Entregas principais:

- funções MIR;
- blocos básicos;
- instruções e terminadores;
- fluxo de controle;
- lowering da HIR;
- validação estrutural;
- dump textual;
- passes iniciais.

Resultado esperado:

```bash
capic --emit mir arquivo.capi
```

### Stage 9 — ABI, runtime mínimo e codegen básico

Status: Planejado.

Objetivo:

- definir contratos executáveis entre MIR, backend e runtime;
- criar runtime mínimo e interface abstrata de backend.

Entregas principais:

- runtime mínimo;
- inicialização do runtime;
- intrínsecos iniciais;
- layout de dados;
- convenções de chamada;
- name mangling;
- interface de backend.

### Stage 10 — Biblioteca padrão mínima

Status: Planejado.

Objetivo:

- implementar o núcleo mínimo da biblioteca padrão necessário para programas
  básicos e bootstrap.

Entregas principais:

- tipos fundamentais;
- `Option`;
- `Result`;
- `String`;
- coleções mínimas;
- entrada e saída;
- intrínsecos necessários.

### Stage 11 — Backend Cranelift

Status: Planejado.

Objetivo:

- gerar executáveis nativos usando Cranelift.

Entregas principais:

- tradução de MIR para Cranelift IR;
- geração de funções, chamadas e controle de fluxo;
- geração de tipos primitivos e operações básicas;
- geração de objetos;
- arquivos objeto;
- linking;
- executáveis.

Resultado esperado:

```bash
capic build arquivo.capi
./arquivo
```

### Stage 12 — Toolchain mínima

Status: Planejado.

Objetivo:

- permitir criação, compilação, execução e teste de projetos Capi.

Entregas principais:

- `capi new`;
- `capi build`;
- `capi run`;
- `capi test`;
- manifesto inicial;
- integração entre `capi` e `capic`;
- formatter inicial;
- test runner.

### Stage 13 — Pacotes e dependências

Status: Planejado.

Objetivo:

- implementar resolução de dependências e gerenciamento de pacotes.

Entregas principais:

- manifesto completo;
- lockfile;
- resolução de versões;
- cache de pacotes;
- dependências locais;
- registro inicial;
- validação de integridade.

### Stage 14 — Ferramentas de desenvolvimento

Status: Planejado.

Objetivo:

- implementar ferramentas de produtividade e integração com editores.

Entregas principais:

- `capi doc`;
- geração de documentação de APIs;
- LSP inicial;
- saída estruturada;
- saída JSON;
- flags de dump;
- diagnósticos integrados a editores.

### Stage 15 — Conformidade, robustez e performance

Status: Planejado.

Objetivo:

- validar sistematicamente a implementação contra a especificação;
- fortalecer robustez e mensurar performance.

Entregas principais:

- suíte de conformidade;
- fuzzing;
- testes diferenciais;
- benchmarks;
- métricas de compilação;
- métricas de código gerado;
- limites de entrada;
- hardening;
- política de regressões.

### Stage 16 — Bootstrap

Status: Planejado.

Objetivo:

- executar o plano de bootstrap de forma reproduzível, auditável e integrada à
  suíte oficial.

Entregas principais:

- subconjunto de bootstrap;
- compilador de estágio inicial;
- migração de componentes selecionados para Capi;
- builds por estágio;
- verificação de reprodutibilidade;
- comparação de artefatos;
- cadeia de confiança documentada.

### Stage 17 — Backend LLVM

Status: Planejado.

Objetivo:

- implementar LLVM como backend de otimização.

Entregas principais:

- tradução de MIR para LLVM IR;
- geração de código LLVM;
- otimizações;
- debug information;
- comparação com Cranelift;
- seleção de backend;
- validação de compatibilidade.

### Stage 18 — Auto-hospedagem

Status: Planejado.

Objetivo:

- tornar a implementação oficial capaz de compilar sua própria implementação em
  Capi, conforme os limites do Documento 27.

Entregas principais:

- implementação parcial ou integral do compilador em Capi;
- build por múltiplos estágios;
- verificação de equivalência;
- testes completos;
- cadeia de confiança documentada;
- release inicial auto-hospedada.

### Stage 19 — Preparação da versão 1.0

Status: Planejado.

Objetivo:

- estabilizar a implementação oficial e preparar a primeira versão estável.

Entregas principais:

- APIs públicas estabilizadas;
- contratos de compatibilidade estabilizados;
- ABI aplicável consolidada;
- documentação pública concluída;
- instaladores;
- artefatos assinados;
- release candidate;
- versão 1.0 publicada.

---

## 6. Próxima Ação Operacional

Com o Stage 2 concluído, a próxima ação operacional é iniciar o Stage 3:

```text
Stage 3 — HIR e resolução de nomes
```

Antes da implementação, devem ser preenchidos ou revisados os documentos
obrigatórios do Stage 3:

- `compiler/semantic/HIR-MODEL.md`;
- `compiler/semantic/SYMBOL-MODEL.md`;
- `compiler/semantic/SCOPE-MODEL.md`;
- `compiler/semantic/NAME-RESOLUTION.md`;
- `compiler/frontend/AST-LOWERING.md`;
- `testing/SEMANTIC-TESTS.md`.

---

## 7. Relação com Outros Registros

Este documento define a ordem operacional.

O progresso real fica em:

```text
FEATURE-STATUS.md
```

Os critérios de aceite ficam em:

```text
DEFINITION-OF-DONE.md
```

O plano normativo completo fica em:

```text
../../specification/implementation/28 — Plano de Desenvolvimento da Implementação Oficial.md
```

---

## 8. Critério de Atualização

Atualize este documento quando:

- um stage mudar de status;
- o Documento 28 for alterado;
- uma entrega antecipada mudar a ordem operacional;
- um novo resultado demonstrável for estabelecido;
- `FEATURE-STATUS.md` registrar conclusão ou bloqueio de stage;
- algum documento obrigatório de stage for renomeado, dividido ou consolidado.
