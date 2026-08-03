# Changelog

Todas as mudanças relevantes em `capi-docs` devem ser registradas neste arquivo.

Este changelog cobre a documentação do projeto Capi. Mudanças da implementação
Rust em `capi-lang` devem ser registradas no contexto próprio da implementação
quando houver política de release ou changelog específico.

---

## 2026-08-02 — Consolidação documental do Stage 2

### Adicionado

* Documentos de frontend sintático:
  * `docs/engineering/compiler/frontend/AST-MODEL.md`;
  * `docs/engineering/compiler/frontend/PARSER-IMPLEMENTATION.md`;
  * `docs/engineering/compiler/frontend/PARSER-RECOVERY.md`;
  * `docs/engineering/compiler/frontend/AST-LOWERING.md`.
* Documento de testes sintáticos:
  * `docs/engineering/testing/PARSER-TESTS.md`.
* Documentos de planejamento operacional:
  * `docs/engineering/planning/IMPLEMENTATION-ORDER.md`;
  * `docs/engineering/planning/MILESTONES.md`;
  * `docs/engineering/planning/RISK-REGISTER.md`;
  * `docs/engineering/planning/ROADMAP.md`;
  * `docs/engineering/planning/TECHNICAL-DEBT.md`.

### Atualizado

* `docs/README.md` para registrar os Stages 0, 1 e 2 como concluídos, incluir a rota de leitura do Stage 2 e apontar o Stage 3 como próximo stage planejado.
* `docs/engineering/README.md` para refletir o frontend sintático inicial, os documentos ativos do Stage 2 e os documentos de planejamento aprovados.
* `docs/engineering/compiler/README.md` para promover AST, parser, recovery e lowering a documentos ativos do frontend.
* `docs/engineering/testing/README.md` para promover `PARSER-TESTS.md` a documento ativo do Stage 2.
* `docs/engineering/planning/README.md` para listar todos os documentos de planejamento como aprovados.
* `docs/engineering/planning/FEATURE-STATUS.md` para registrar a conclusão formal do Stage 2.
* `README.md` da raiz de `capi-docs` para refletir o Stage 2, `capic --emit ast arquivo.capi`, `capi-ast`, `capi-parser` e o próximo Stage 3.

### Registrado

* Critérios de conclusão do Stage 2:
  * o subconjunto sintático inicial é aceito;
  * entradas inválidas produzem diagnósticos sintáticos adequados;
  * o parser continua após erros recuperáveis;
  * a AST preserva spans;
  * o dump da AST é determinístico;
  * o resultado esperado pode ser obtido por `capic --emit ast arquivo.capi`;
  * todos os testes obrigatórios passam.
* Resultado demonstrável do frontend sintático:

```bash
capic --emit ast arquivo.capi
```

* Próximo stage planejado:

```text
Stage 3 — HIR e resolução de nomes
```

---

## 2026-08-02 — Consolidação documental do Stage 1

### Adicionado

* Documentos de fontes do compilador:
  * `docs/engineering/compiler/source/SOURCE-MODEL.md`;
  * `docs/engineering/compiler/source/SOURCE-MAP.md`;
  * `docs/engineering/compiler/source/SPANS-AND-LOCATIONS.md`;
  * `docs/engineering/compiler/source/UNICODE-AND-ENCODING.md`.
* Documentos de frontend léxico:
  * `docs/engineering/compiler/frontend/TOKEN-MODEL.md`;
  * `docs/engineering/compiler/frontend/LEXER-IMPLEMENTATION.md`.
* Documentos de diagnósticos:
  * `docs/engineering/compiler/diagnostics/DIAGNOSTIC-DATA-MODEL.md`;
  * `docs/engineering/compiler/diagnostics/DIAGNOSTIC-ARCHITECTURE.md`;
  * `docs/engineering/compiler/diagnostics/DIAGNOSTIC-STYLE-GUIDE.md`.
* Documento de testes léxicos:
  * `docs/engineering/testing/LEXER-TESTS.md`.

### Atualizado

* `docs/README.md` para registrar o Stage 1, a rota de leitura do compilador e o resultado demonstrável `capic --emit tokens arquivo.capi`.
* `docs/engineering/README.md` para marcar `compiler/` como subárea ativa e registrar os documentos ativos do Stage 1.
* `docs/engineering/compiler/README.md` como índice operacional do compilador.
* `docs/engineering/testing/README.md` para promover `LEXER-TESTS.md` a documento ativo do Stage 1.
* `README.md` da raiz de `capi-docs` para refletir a nova documentação e a relação com os crates do compilador em `capi-lang`.
* Status dos documentos obrigatórios do Stage 1 de `Proposto` para `Aprovado`.

### Registrado

* Critérios de conclusão do Stage 1:
  * arquivos válidos são lidos corretamente;
  * posições de erro são precisas;
  * todos os tokens do subconjunto inicial são reconhecidos;
  * entradas inválidas produzem diagnósticos estruturados;
  * não há pânico em entradas malformadas;
  * todos os testes obrigatórios passam.
* Resultado demonstrável do frontend léxico:

```bash
capic --emit tokens arquivo.capi
```

---

## 2026-07-30 — Consolidação documental do Stage 0

### Adicionado

* Especificação do Documento 28, `Plano de Desenvolvimento da Implementação Oficial`, como plano operacional dos stages da implementação oficial.
* Documentos de engenharia bloqueantes do Stage 0:
  * `ENGINEERING-PRINCIPLES.md`;
  * `PROJECT-STRUCTURE.md`;
  * `architecture/COMPILER-ARCHITECTURE.md`;
  * `architecture/WORKSPACE-ARCHITECTURE.md`;
  * `architecture/COMPONENT-RESPONSIBILITIES.md`;
  * `architecture/DEPENDENCY-RULES.md`;
  * `development/DEVELOPMENT-SETUP.md`;
  * `build-and-ci/BUILD-SYSTEM.md`;
  * `testing/TEST-STRATEGY.md`;
  * `planning/DEFINITION-OF-DONE.md`.
* Documentos operacionais e de consolidação do Stage 0:
  * `ENGINEERING-GLOSSARY.md`;
  * `architecture/COMPILATION-PIPELINE.md`;
  * `development/BUILDING-FROM-SOURCE.md`;
  * `development/CODING-STANDARDS.md`;
  * `development/RUST-STYLE-GUIDE.md`;
  * `planning/FEATURE-STATUS.md`.
* ADRs obrigatórias do Stage 0:
  * `ADR-0001 — Rust como Linguagem da Implementação Oficial.md`;
  * `ADR-0002 — Organização da Implementação em Workspace Cargo.md`;
  * `ADR-0003 — Separação entre Frontend, Middle-end e Backend.md`;
  * `ADR-0013 — Política de Dependências Externas.md`;
  * `ADR-0015 — Estratégia Inicial de Testes.md`;
  * `ADR-0016 — Organização Física do Repositório.md`.

### Atualizado

* `docs/specification/README.md` para incluir o Documento 28 na lista da especificação.
* `docs/adr/README.md` como índice das ADRs aprovadas e reservadas.
* `docs/engineering/README.md` como entrada principal da documentação de engenharia.
* `docs/engineering/architecture/README.md` como índice da arquitetura.
* `docs/engineering/build-and-ci/README.md` como índice de build e CI.
* `docs/engineering/development/README.md` como índice de desenvolvimento.
* `docs/engineering/planning/README.md` como índice de planejamento.
* `docs/engineering/testing/README.md` como índice de testes.
* `docs/README.md` como entrada geral da documentação.

### Registrado

* Conclusão formal do Stage 0 em `docs/engineering/planning/FEATURE-STATUS.md`.
* Critérios de conclusão, validação e rastreabilidade em `docs/engineering/planning/DEFINITION-OF-DONE.md`.
* Relação entre especificação, ADRs, engenharia e implementação oficial em `docs/README.md` e `docs/engineering/README.md`.
