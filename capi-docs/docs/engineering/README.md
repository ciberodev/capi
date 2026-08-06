# Engineering

Esta pasta reúne a documentação de engenharia da implementação oficial da
Linguagem Capi.

Ela traduz a especificação da linguagem em decisões operacionais de projeto:
arquitetura, workspace, build, desenvolvimento, testes, planejamento,
dependências, estilo de código, fases do compilador e critérios de conclusão.

Documentos nesta área não redefinem a semântica da linguagem. Quando houver
conflito, prevalecem a especificação normativa, as ADRs aprovadas e os
documentos bloqueantes do stage atual.

---

## Documentos raiz aprovados no Stage 0

| Documento | Status | Natureza | Finalidade |
| --- | --- | --- | --- |
| `ENGINEERING-PRINCIPLES.md` | Aprovado | Documento de engenharia bloqueante | Define princípios gerais para conduzir a implementação oficial: rastreabilidade, simplicidade, modularidade, validação e controle de decisões. |
| `PROJECT-STRUCTURE.md` | Aprovado | Documento de engenharia bloqueante | Define a organização física do repositório, separando documentação, implementação, scripts, artefatos e áreas futuras. |
| `ENGINEERING-GLOSSARY.md` | Aprovado | Documento de consolidação | Padroniza termos usados nos documentos de engenharia e na implementação oficial. |

---

## Subáreas ativas

| Pasta | Estado | Finalidade |
| --- | --- | --- |
| `architecture/` | Ativa | Arquitetura do compilador, workspace, componentes, dependências e pipeline de compilação. |
| `build-and-ci/` | Ativa | Sistema de build, comandos canônicos, scripts, CI, artefatos e validação automatizada. |
| `compiler/` | Ativa | Documentação específica das fases do compilador: fontes, diagnósticos, lexer, parser, AST, HIR, resolução de nomes e sistema de tipos inicial. |
| `development/` | Ativa | Ambiente local, build a partir do código-fonte, padrões de código e guia de estilo Rust. |
| `planning/` | Ativa | Definition of Done, status de features, ordem de implementação, milestones, roadmap, riscos e dívida técnica. |
| `testing/` | Ativa | Estratégia oficial de testes, suíte mínima do Stage 0, testes léxicos, sintáticos e semânticos dos Stages 1 a 4. |

Essas áreas possuem documentação preenchida e fazem parte da implementação
ativa. Os documentos obrigatórios dos Stages 1, 2, 3 e 4 estão aprovados e
servem como contrato operacional para o frontend inicial, resolução de nomes e
sistema de tipos inicial.

---

## Subáreas reservadas

| Pasta | Finalidade esperada |
| --- | --- |
| `abi/` | Documentar ABI, layout de dados, convenções de chamada, FFI, mangling e visibilidade de símbolos. |
| `ai-assisted-development/` | Definir regras para contribuição assistida por IA, revisão, proveniência e rastreabilidade. |
| `observability/` | Definir logging, tracing, dumps, crash reporting e política de telemetria. |
| `performance/` | Definir metas, benchmarks, ambiente de medição e política de regressão de performance. |
| `release/` | Definir versionamento, canais, checklist, changelog, compatibilidade e assinatura de artefatos. |
| `runtime/` | Definir arquitetura e responsabilidades do runtime quando ele passar a existir. |
| `security/` | Definir threat model, hardening, auditoria de dependências e resposta a vulnerabilidades. |
| `standard-library/` | Definir arquitetura, organização e política da futura biblioteca padrão. |
| `toolchain/` | Definir arquitetura da toolchain além do compilador: CLI, manifesto, lockfile, LSP, formatador, gerador de documentação, package manager e test runner. |

Enquanto uma subárea estiver vazia, ela é apenas reserva estrutural. Ela não cria
obrigações normativas próprias.

---

## Estado dos stages

### Stage 0

O Stage 0 estabeleceu a fundação da implementação oficial.

Resultados registrados:

* documentos bloqueantes de engenharia aprovados;
* documentos operacionais e de consolidação aprovados;
* ADRs obrigatórias aprovadas;
* workspace Cargo criado em `capi-lang`;
* crates fundamentais criados;
* executável `capic` criado;
* build, fmt, lint, testes, documentação Rust e CI configurados;
* `capic --help` e `capic --version` implementados;
* registro formal de progresso criado.

O registro formal fica em:

```text
planning/FEATURE-STATUS.md
```

### Stage 1

O Stage 1 iniciou a infraestrutura real do compilador.

Resultados registrados:

* documentos de fontes preenchidos em `compiler/source/`;
* documentos de frontend léxico preenchidos em `compiler/frontend/`;
* documentos de diagnósticos preenchidos em `compiler/diagnostics/`;
* documento de testes léxicos preenchido em `testing/LEXER-TESTS.md`;
* crates `capi-source`, `capi-diagnostics` e `capi-lexer` consolidados;
* `SourceId`, `SourceFile`, `SourceMap`, `Span`, linha e coluna implementados;
* modelo de tokens implementado;
* lexer do subconjunto inicial implementado;
* identificadores, keywords, literais, operadores, delimitadores e comentários
  reconhecidos;
* erros léxicos estruturados implementados;
* dump de tokens disponível via `capic --emit tokens arquivo.capi`;
* fixtures e snapshots léxicos criados;
* critérios de conclusão do Stage 1 validados por testes.

O resultado demonstrável é:

```bash
capic --emit tokens arquivo.capi
```

### Stage 2

O Stage 2 implementou o frontend sintático inicial.

Resultados registrados:

* documentos de AST, parser, recuperação e lowering preenchidos em
  `compiler/frontend/`;
* documento de testes sintáticos preenchido em `testing/PARSER-TESTS.md`;
* crate `capi-ast` criado;
* crate `capi-parser` criado;
* AST com spans implementada;
* parser de módulos, imports, declarações, classes, funções, tipos, comandos e
  expressões implementado;
* precedência de operadores implementada;
* diagnósticos sintáticos estruturados com códigos `PARSE` implementados;
* recuperação de erros recuperáveis implementada;
* AST parcial com nós de erro implementada;
* dump determinístico da AST implementado;
* snapshots golden de dump de AST criados;
* `capic --emit ast arquivo.capi` implementado;
* critérios de conclusão do Stage 2 validados por testes.

O resultado demonstrável é:

```bash
capic --emit ast arquivo.capi
```

### Stage 3

O Stage 3 implementou HIR e resolução inicial de nomes.

Resultados registrados:

* documentos de HIR, símbolos, escopos e resolução preenchidos em
  `compiler/semantic/`;
* documento de lowering atualizado em `compiler/frontend/AST-LOWERING.md`;
* documento de testes semânticos preenchido em `testing/SEMANTIC-TESTS.md`;
* crate `capi-hir` criado como modelo HIR puro;
* crate `capi-lowering` criado como fronteira AST -> HIR;
* crate `capi-sema` criado para escopos, símbolos e resolução de nomes;
* IDs HIR, `ScopeId` e `SymbolId` internos e determinísticos implementados;
* tabelas de símbolos e escopos implementadas para o subconjunto inicial;
* resolução de nomes implementada para valores, tipos, módulos/imports e
  patterns do subconjunto inicial;
* diagnósticos semânticos estruturados para duplicidade, inexistência e
  ambiguidade implementados;
* dump determinístico de HIR resolvida implementado;
* `capic --emit hir arquivo.capi` implementado;
* critérios de conclusão do Stage 3 validados por testes.

O resultado demonstrável é:

```bash
capic --emit hir arquivo.capi
```

### Stage 4

O Stage 4 implementou o sistema de tipos inicial.

Resultados registrados:

* documentos de modelo de tipos, interning, inferência, pipeline de checagem,
  subtipagem, coerções e generics preenchidos em `compiler/semantic/`;
* documento de testes semânticos ampliado em `testing/SEMANTIC-TESTS.md`;
* tipos internos, interning de tipos, inferência e verificação de tipos
  implementados em `capi-sema`;
* subtipagem, coerções explícitas do subconjunto inicial, resolução de chamadas,
  overload aplicável e generics iniciais implementados;
* diagnósticos de tipo estruturados com códigos `TYPE` implementados;
* auditoria dos testes possíveis de `LEXER-TESTS.md`, `PARSER-TESTS.md` e
  `SEMANTIC-TESTS.md` registrada;
* comportamento de `capic check arquivo.capi` definido para sucesso silencioso e
  falha com diagnóstico em stderr;
* critérios de conclusão do Stage 4 validados por testes.

O resultado demonstrável é:

```bash
capic check arquivo.capi
```

### Próximo stage

O próximo stage planejado é:

```text
Stage 5 — Modelo de objetos
```

O início do Stage 5 deve partir do modelo de objetos, layouts, campos,
métodos, inicialização e integração com o sistema de tipos já entregue.

---

## Ordem de leitura recomendada

Para entender a engenharia do projeto até o Stage 4, leia nesta ordem:

1. `ENGINEERING-PRINCIPLES.md`
2. `PROJECT-STRUCTURE.md`
3. `ENGINEERING-GLOSSARY.md`
4. `architecture/README.md`
5. `development/README.md`
6. `build-and-ci/README.md`
7. `planning/README.md`
8. `compiler/README.md`
9. `testing/README.md`
10. `../adr/README.md`
11. `../specification/README.md`

Essa ordem começa pelos princípios, passa pela estrutura e depois conecta
arquitetura, operação, planejamento, compilador, testes, decisões e
especificação.

---

## Documentos bloqueantes do Stage 0

O Stage 0 dependeu dos seguintes documentos de engenharia:

```text
ENGINEERING-PRINCIPLES.md
PROJECT-STRUCTURE.md
architecture/COMPILER-ARCHITECTURE.md
architecture/WORKSPACE-ARCHITECTURE.md
architecture/COMPONENT-RESPONSIBILITIES.md
architecture/DEPENDENCY-RULES.md
development/DEVELOPMENT-SETUP.md
build-and-ci/BUILD-SYSTEM.md
testing/TEST-STRATEGY.md
planning/DEFINITION-OF-DONE.md
```

Documentos operacionais e de consolidação aprovados no mesmo stage:

```text
ENGINEERING-GLOSSARY.md
architecture/COMPILATION-PIPELINE.md
development/BUILDING-FROM-SOURCE.md
development/CODING-STANDARDS.md
development/RUST-STYLE-GUIDE.md
planning/FEATURE-STATUS.md
```

---

## Documentos ativos dos Stages 1, 2, 3 e 4

O Stage 1 usa os seguintes documentos de engenharia:

```text
compiler/source/SOURCE-MODEL.md
compiler/source/SOURCE-MAP.md
compiler/source/SPANS-AND-LOCATIONS.md
compiler/source/UNICODE-AND-ENCODING.md
compiler/frontend/TOKEN-MODEL.md
compiler/frontend/LEXER-IMPLEMENTATION.md
compiler/diagnostics/DIAGNOSTIC-DATA-MODEL.md
compiler/diagnostics/DIAGNOSTIC-ARCHITECTURE.md
compiler/diagnostics/DIAGNOSTIC-STYLE-GUIDE.md
testing/LEXER-TESTS.md
```

O Stage 2 usa os seguintes documentos de engenharia:

```text
compiler/frontend/AST-MODEL.md
compiler/frontend/PARSER-IMPLEMENTATION.md
compiler/frontend/PARSER-RECOVERY.md
compiler/frontend/AST-LOWERING.md
testing/PARSER-TESTS.md
```

O Stage 3 usa os seguintes documentos de engenharia:

```text
compiler/frontend/AST-LOWERING.md
compiler/semantic/HIR-MODEL.md
compiler/semantic/SYMBOL-MODEL.md
compiler/semantic/SCOPE-MODEL.md
compiler/semantic/NAME-RESOLUTION.md
testing/SEMANTIC-TESTS.md
```

O Stage 4 usa os seguintes documentos de engenharia:

```text
compiler/semantic/TYPE-MODEL.md
compiler/semantic/TYPE-INTERNING.md
compiler/semantic/TYPE-INFERENCE.md
compiler/semantic/TYPE-CHECKING-PIPELINE.md
compiler/semantic/SUBTYPING-AND-COERCIONS.md
compiler/semantic/GENERICS-IMPLEMENTATION.md
testing/SEMANTIC-TESTS.md
```

Esses documentos estão em status `Aprovado` e descrevem a implementação e os
testes entregues para o frontend inicial, a resolução de nomes e o sistema de
tipos inicial.

---

## Documentos ativos de planejamento

O planejamento da implementação oficial é acompanhado por:

```text
planning/DEFINITION-OF-DONE.md
planning/FEATURE-STATUS.md
planning/IMPLEMENTATION-ORDER.md
planning/MILESTONES.md
planning/RISK-REGISTER.md
planning/ROADMAP.md
planning/TECHNICAL-DEBT.md
```

Esses documentos registram critérios de aceite, progresso, ordem operacional,
milestones, roadmap, riscos e dívidas técnicas monitoradas.

---

## Comandos canônicos atuais

Execute a partir de `../../capi-lang/`:

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets
cargo run -p capi-cli -- --emit tokens tests/lexer/pass/basic.cap
cargo run -p capi-cli --bin capic -- --emit ast crates/capi-parser/tests/fixtures/ast_dump/basic.cap
cargo run -p capi-cli -- --emit hir tests/semantic/pass/basic.cap
cargo run -p capi-cli --bin capic -- check tests/semantic/pass/basic.cap
```

Esses comandos validam a implementação atual do workspace, incluindo os critérios
obrigatórios dos Stages 1, 2, 3 e 4.

A validação consolidada do workspace é:

```bash
./scripts/check.sh
```

---

## Relação com a implementação

A implementação oficial vive em:

```text
../../capi-lang/
```

A documentação de engenharia deve permanecer sincronizada com:

```text
../../capi-lang/Cargo.toml
../../capi-lang/Cargo.lock
../../capi-lang/DEPENDENCIES.md
../../capi-lang/TOOLCHAIN.md
../../capi-lang/README.md
../../capi-lang/crates/capi-source/
../../capi-lang/crates/capi-diagnostics/
../../capi-lang/crates/capi-lexer/
../../capi-lang/crates/capi-ast/
../../capi-lang/crates/capi-parser/
../../capi-lang/crates/capi-hir/
../../capi-lang/crates/capi-lowering/
../../capi-lang/crates/capi-sema/
../../capi-lang/crates/capi-driver/
../../capi-lang/crates/capi-cli/
../../capi-lang/tests/lexer/
../../capi-lang/tests/semantic/
../../capi-lang/crates/capi-parser/tests/
../../.github/workflows/capi-lang-ci.yml
```

Mudanças estruturais no workspace, dependências, toolchain, CI, crates
fundamentais, frontend, AST, parser, HIR, semântica inicial, sistema de tipos ou
layout de testes devem ser refletidas nos documentos de engenharia e, quando
forem decisões arquiteturais, nas ADRs correspondentes.

---

## Critério de atualização

Atualize este README quando:

* uma nova subárea ganhar documentação aprovada;
* um documento raiz mudar de status;
* um stage for aberto ou concluído;
* a estrutura de `capi-lang` mudar de forma relevante;
* novos documentos passarem a ser bloqueantes para um stage;
* a relação entre especificação, ADRs e engenharia mudar.
