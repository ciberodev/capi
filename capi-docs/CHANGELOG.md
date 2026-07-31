# Changelog

Todas as mudanças relevantes em `capi-docs` devem ser registradas neste arquivo.

Este changelog cobre a documentação do projeto Capi. Mudanças da implementação
Rust em `capi-lang` devem ser registradas no contexto próprio da implementação
quando houver política de release ou changelog específico.

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
