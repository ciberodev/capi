# Capi Documentation

Este diretório reúne a documentação do projeto Capi.

A documentação está organizada para separar:

* especificação da linguagem;
* decisões arquiteturais;
* engenharia da implementação oficial;
* governança futura;
* RFCs futuras;
* templates operacionais.

---

## Áreas principais

| Pasta | Estado | Finalidade |
| --- | --- | --- |
| `specification/` | Ativa | Contém a especificação da linguagem Capi e a especificação de referência da implementação oficial. |
| `adr/` | Ativa | Contém Architecture Decision Records, incluindo as ADRs aprovadas para a fundação da implementação oficial. |
| `engineering/` | Ativa | Contém documentos de engenharia para arquitetura, workspace, build, desenvolvimento, testes, planejamento e fases do compilador da implementação oficial. |
| `templates/` | Suporte | Contém modelos para ADRs, RFCs, tarefas, issues, pull requests, testes, releases e avisos de segurança. |
| `governance/` | Reservada | Área para políticas de governança do projeto quando forem formalizadas. |
| `rfc/` | Reservada | Área para propostas formais de evolução quando o processo de RFC for adotado. |

---

## Estado atual

Os Stages 0, 1 e 2 da implementação oficial estão concluídos e registrados em:

```text
engineering/planning/FEATURE-STATUS.md
```

O Stage 0 estabeleceu:

* documentação bloqueante de engenharia aprovada;
* ADRs obrigatórias aprovadas;
* workspace Cargo em `../capi-lang`;
* crates fundamentais;
* executável `capic`;
* build, formatação, lint, testes, documentação Rust e CI;
* validação de `capic --help` e `capic --version`.

O Stage 1 iniciou a infraestrutura real do compilador e entregou:

* documentação de fontes, spans, Unicode e source map;
* documentação de diagnósticos estruturados;
* documentação do modelo de tokens e implementação do lexer;
* documentação dos testes léxicos obrigatórios;
* crates `capi-source`, `capi-diagnostics` e `capi-lexer`;
* `SourceId`, `SourceFile`, `SourceMap`, `Span`, linha e coluna;
* lexer do subconjunto inicial;
* diagnósticos léxicos estruturados;
* fixtures e snapshots de testes léxicos;
* dump de tokens via `capic --emit tokens arquivo.capi`;
* critérios de conclusão do Stage 1 validados por testes.

O Stage 2 implementou o frontend sintático inicial e entregou:

* documentação de AST, parser, recuperação sintática e lowering;
* documentação dos testes sintáticos obrigatórios;
* crates `capi-ast` e `capi-parser`;
* AST com spans;
* parser de módulos, imports, declarações, classes, funções, tipos, comandos e
  expressões;
* precedência de operadores;
* diagnósticos sintáticos estruturados;
* recuperação de erros recuperáveis;
* AST parcial com nós de erro;
* dump determinístico da AST;
* snapshots golden de dump da AST;
* dump de AST via `capic --emit ast arquivo.capi`;
* critérios de conclusão do Stage 2 validados por testes.

O próximo stage planejado é:

```text
Stage 3 — HIR e resolução de nomes
```

---

## Ordem de leitura recomendada

Para entender o projeto a partir da documentação, leia nesta ordem:

1. `specification/README.md`
2. `specification/implementation/28 — Plano de Desenvolvimento da Implementação Oficial.md`
3. `adr/README.md`
4. `engineering/README.md`
5. `engineering/planning/README.md`
6. `engineering/compiler/README.md`
7. `engineering/testing/README.md`

Para contribuir com implementação, depois leia:

1. `engineering/architecture/README.md`
2. `engineering/development/README.md`
3. `engineering/build-and-ci/README.md`
4. `engineering/planning/README.md`
5. `engineering/compiler/README.md`
6. `engineering/testing/README.md`

Para entender o frontend inicial entregue nos Stages 1 e 2, leia:

1. `engineering/compiler/source/SOURCE-MODEL.md`
2. `engineering/compiler/source/SOURCE-MAP.md`
3. `engineering/compiler/source/SPANS-AND-LOCATIONS.md`
4. `engineering/compiler/source/UNICODE-AND-ENCODING.md`
5. `engineering/compiler/diagnostics/DIAGNOSTIC-DATA-MODEL.md`
6. `engineering/compiler/diagnostics/DIAGNOSTIC-ARCHITECTURE.md`
7. `engineering/compiler/diagnostics/DIAGNOSTIC-STYLE-GUIDE.md`
8. `engineering/compiler/frontend/TOKEN-MODEL.md`
9. `engineering/compiler/frontend/LEXER-IMPLEMENTATION.md`
10. `engineering/testing/LEXER-TESTS.md`
11. `engineering/compiler/frontend/AST-MODEL.md`
12. `engineering/compiler/frontend/PARSER-IMPLEMENTATION.md`
13. `engineering/compiler/frontend/PARSER-RECOVERY.md`
14. `engineering/compiler/frontend/AST-LOWERING.md`
15. `engineering/testing/PARSER-TESTS.md`

Para iniciar o próximo stage planejado, comece por:

1. `engineering/planning/IMPLEMENTATION-ORDER.md`
2. `engineering/planning/MILESTONES.md`
3. `engineering/planning/ROADMAP.md`
4. `engineering/compiler/semantic/HIR-MODEL.md`
5. `engineering/compiler/semantic/SYMBOL-MODEL.md`
6. `engineering/compiler/semantic/SCOPE-MODEL.md`
7. `engineering/compiler/semantic/NAME-RESOLUTION.md`
8. `engineering/testing/SEMANTIC-TESTS.md`

---

## Fontes de verdade

A documentação segue uma hierarquia prática:

1. `specification/` define garantias da linguagem e arquitetura de referência;
2. `adr/` registra decisões arquiteturais concretas;
3. `engineering/` transforma essas decisões em regras operacionais;
4. `../capi-lang/` contém a implementação oficial;
5. `engineering/planning/FEATURE-STATUS.md` registra progresso formal por stage;
6. `engineering/planning/IMPLEMENTATION-ORDER.md`, `MILESTONES.md`,
   `ROADMAP.md`, `RISK-REGISTER.md` e `TECHNICAL-DEBT.md` registram ordem,
   marcos, próximos passos, riscos e limitações aceitas.

Documentos de engenharia não devem redefinir a semântica da linguagem. ADRs não
devem contradizer a especificação sem registrar explicitamente a necessidade de
ajuste normativo.

---

## Documentos ativos do Stage 0

Documentos de especificação relevantes:

```text
specification/README.md
specification/implementation/27 — Bootstrap Plan e Arquitetura da Implementação Oficial.md
specification/implementation/28 — Plano de Desenvolvimento da Implementação Oficial.md
```

ADRs aprovadas:

```text
adr/ADR-0001 — Rust como Linguagem da Implementação Oficial.md
adr/ADR-0002 — Organização da Implementação em Workspace Cargo.md
adr/ADR-0003 — Separação entre Frontend, Middle-end e Backend.md
adr/ADR-0013 — Política de Dependências Externas.md
adr/ADR-0015 — Estratégia Inicial de Testes.md
adr/ADR-0016 — Organização Física do Repositório.md
```

Documentos de engenharia aprovados:

```text
engineering/ENGINEERING-PRINCIPLES.md
engineering/PROJECT-STRUCTURE.md
engineering/ENGINEERING-GLOSSARY.md
engineering/architecture/COMPILER-ARCHITECTURE.md
engineering/architecture/WORKSPACE-ARCHITECTURE.md
engineering/architecture/COMPONENT-RESPONSIBILITIES.md
engineering/architecture/DEPENDENCY-RULES.md
engineering/architecture/COMPILATION-PIPELINE.md
engineering/development/DEVELOPMENT-SETUP.md
engineering/development/BUILDING-FROM-SOURCE.md
engineering/development/CODING-STANDARDS.md
engineering/development/RUST-STYLE-GUIDE.md
engineering/build-and-ci/BUILD-SYSTEM.md
engineering/testing/TEST-STRATEGY.md
engineering/planning/DEFINITION-OF-DONE.md
engineering/planning/FEATURE-STATUS.md
```

---

## Documentos ativos do Stage 1

Documentos de engenharia do compilador:

```text
engineering/compiler/README.md
engineering/compiler/source/SOURCE-MODEL.md
engineering/compiler/source/SOURCE-MAP.md
engineering/compiler/source/SPANS-AND-LOCATIONS.md
engineering/compiler/source/UNICODE-AND-ENCODING.md
engineering/compiler/diagnostics/DIAGNOSTIC-DATA-MODEL.md
engineering/compiler/diagnostics/DIAGNOSTIC-ARCHITECTURE.md
engineering/compiler/diagnostics/DIAGNOSTIC-STYLE-GUIDE.md
engineering/compiler/frontend/TOKEN-MODEL.md
engineering/compiler/frontend/LEXER-IMPLEMENTATION.md
```

Documentos de testes:

```text
engineering/testing/README.md
engineering/testing/LEXER-TESTS.md
```

Resultado demonstrável:

```bash
capic --emit tokens arquivo.capi
```

---

## Documentos ativos do Stage 2

Documentos de engenharia do compilador:

```text
engineering/compiler/frontend/AST-MODEL.md
engineering/compiler/frontend/PARSER-IMPLEMENTATION.md
engineering/compiler/frontend/PARSER-RECOVERY.md
engineering/compiler/frontend/AST-LOWERING.md
```

Documentos de testes:

```text
engineering/testing/PARSER-TESTS.md
```

Documentos de planejamento atualizados:

```text
engineering/planning/FEATURE-STATUS.md
engineering/planning/IMPLEMENTATION-ORDER.md
engineering/planning/MILESTONES.md
engineering/planning/RISK-REGISTER.md
engineering/planning/ROADMAP.md
engineering/planning/TECHNICAL-DEBT.md
```

Resultado demonstrável:

```bash
capic --emit ast arquivo.capi
```

---

## Relação com o repositório

Este diretório documenta o projeto.

A implementação oficial vive em:

```text
../capi-lang/
```

O README raiz do repositório apresenta a visão geral para quem entra no projeto:

```text
../README.md
../README PT-BR.md
```

---

## Critério de atualização

Atualize este README quando:

* uma nova área de documentação for criada;
* uma área reservada se tornar ativa;
* um stage for concluído;
* novas ADRs forem aprovadas;
* documentos de engenharia passarem a ser bloqueantes;
* a ordem de leitura recomendada mudar.
