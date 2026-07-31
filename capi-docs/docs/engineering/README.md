# Engineering

Esta pasta reúne a documentação de engenharia da implementação oficial da
Linguagem Capi.

Ela traduz a especificação da linguagem em decisões operacionais de projeto:
arquitetura, workspace, build, desenvolvimento, testes, planejamento,
dependências, estilo de código e critérios de conclusão.

Documentos nesta área não redefinem a semântica da linguagem. Quando houver
conflito, prevalecem a especificação normativa e as ADRs aprovadas.

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
| `development/` | Ativa | Ambiente local, build a partir do código-fonte, padrões de código e guia de estilo Rust. |
| `planning/` | Ativa | Definition of Done, registro de progresso do Stage 0 e documentos futuros de planejamento. |
| `testing/` | Ativa | Estratégia oficial de testes e categorias futuras da suíte. |

Essas áreas possuem pelo menos um documento preenchido e aprovado no Stage 0.

---

## Subáreas reservadas

| Pasta | Finalidade esperada |
| --- | --- |
| `abi/` | Documentar ABI, layout de dados, convenções de chamada, FFI, mangling e visibilidade de símbolos. |
| `ai-assisted-development/` | Definir regras para contribuição assistida por IA, revisão, proveniência e rastreabilidade. |
| `compiler/` | Consolidar documentação específica das fases do compilador quando lexer, parser, IR e verificadores amadurecerem. |
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

## Estado do Stage 0

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

---

## Ordem de leitura recomendada

Para entender a engenharia do projeto a partir do Stage 0, leia nesta ordem:

1. `ENGINEERING-PRINCIPLES.md`
2. `PROJECT-STRUCTURE.md`
3. `ENGINEERING-GLOSSARY.md`
4. `architecture/README.md`
5. `development/README.md`
6. `build-and-ci/README.md`
7. `testing/README.md`
8. `planning/README.md`
9. `../adr/README.md`
10. `../specification/README.md`

Essa ordem começa pelos princípios, passa pela estrutura e depois conecta
arquitetura, operação, testes, planejamento, decisões e especificação.

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

## Relação com a implementação

A implementação oficial vive em:

```text
../../capi-lang/
```

No Stage 0, a documentação de engenharia deve permanecer sincronizada com:

```text
../../capi-lang/Cargo.toml
../../capi-lang/Cargo.lock
../../capi-lang/DEPENDENCIES.md
../../capi-lang/TOOLCHAIN.md
../../capi-lang/README.md
../../.github/workflows/capi-lang-ci.yml
```

Mudanças estruturais no workspace, dependências, toolchain, CI ou crates
fundamentais devem ser refletidas nos documentos de engenharia e, quando forem
decisões arquiteturais, nas ADRs correspondentes.

---

## Critério de atualização

Atualize este README quando:

* uma nova subárea ganhar documentação aprovada;
* um documento raiz mudar de status;
* um stage for aberto ou concluído;
* a estrutura de `capi-lang` mudar de forma relevante;
* novos documentos passarem a ser bloqueantes para um stage;
* a relação entre especificação, ADRs e engenharia mudar.
