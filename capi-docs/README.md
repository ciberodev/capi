# Capi Docs

`capi-docs` contém a documentação do projeto Capi.

Esta árvore documenta a linguagem, as decisões arquiteturais e a engenharia da
implementação oficial. A implementação Rust vive fora deste pacote, em
`../capi-lang`.

---

## Conteúdo principal

| Caminho | Finalidade |
| --- | --- |
| `docs/README.md` | Entrada geral da documentação do projeto. |
| `docs/specification/` | Especificação da linguagem Capi e da implementação oficial. |
| `docs/adr/` | Architecture Decision Records aprovadas e reservadas. |
| `docs/engineering/` | Documentação de engenharia da implementação oficial, incluindo arquitetura, build, testes, planejamento e fases do compilador. |
| `docs/templates/` | Templates para ADRs, RFCs, issues, pull requests, testes, releases e tarefas. |
| `docs/governance/` | Área reservada para governança formal. |
| `docs/rfc/` | Área reservada para propostas formais de evolução. |

---

## Estado atual

O Stage 0 da implementação oficial está concluído e registrado em:

```text
docs/engineering/planning/FEATURE-STATUS.md
```

O Stage 0 consolidou:

* plano operacional da implementação oficial;
* documentos bloqueantes de engenharia;
* documentos operacionais e de consolidação;
* ADRs obrigatórias;
* estrutura inicial de `capi-lang`;
* validação mínima de build, testes, CI e `capic`.

O Stage 1 iniciou a infraestrutura do compilador e consolidou:

* documentação de fontes, spans, Unicode e source map;
* documentação de diagnósticos estruturados;
* documentação do modelo de tokens e do lexer;
* documentação dos testes léxicos obrigatórios;
* crates `capi-source`, `capi-diagnostics` e `capi-lexer`;
* dump de tokens via `capic --emit tokens arquivo.capi`;
* critérios de conclusão do frontend léxico validados por testes.

O histórico documental está registrado em:

```text
CHANGELOG.md
```

---

## Ordem de leitura recomendada

Para começar pela documentação:

1. `docs/README.md`
2. `docs/specification/README.md`
3. `docs/specification/implementation/28 — Plano de Desenvolvimento da Implementação Oficial.md`
4. `docs/adr/README.md`
5. `docs/engineering/README.md`
6. `docs/engineering/compiler/README.md`
7. `docs/engineering/testing/README.md`
8. `docs/engineering/planning/FEATURE-STATUS.md`

Para começar pela engenharia da implementação:

1. `docs/engineering/architecture/README.md`
2. `docs/engineering/development/README.md`
3. `docs/engineering/build-and-ci/README.md`
4. `docs/engineering/compiler/README.md`
5. `docs/engineering/testing/README.md`
6. `docs/engineering/planning/README.md`

Para trabalhar no Stage 1 do compilador:

1. `docs/engineering/compiler/source/SOURCE-MODEL.md`
2. `docs/engineering/compiler/source/SOURCE-MAP.md`
3. `docs/engineering/compiler/source/SPANS-AND-LOCATIONS.md`
4. `docs/engineering/compiler/source/UNICODE-AND-ENCODING.md`
5. `docs/engineering/compiler/diagnostics/DIAGNOSTIC-DATA-MODEL.md`
6. `docs/engineering/compiler/diagnostics/DIAGNOSTIC-ARCHITECTURE.md`
7. `docs/engineering/compiler/diagnostics/DIAGNOSTIC-STYLE-GUIDE.md`
8. `docs/engineering/compiler/frontend/TOKEN-MODEL.md`
9. `docs/engineering/compiler/frontend/LEXER-IMPLEMENTATION.md`
10. `docs/engineering/testing/LEXER-TESTS.md`

---

## Arquivos de projeto

| Arquivo | Finalidade |
| --- | --- |
| `CHANGELOG.md` | Registro de mudanças relevantes em `capi-docs`. |
| `CONTRIBUTING.md` | Orientações futuras de contribuição. |
| `CODE_OF_CONDUCT.md` | Código de conduta do projeto. |
| `SECURITY.md` | Política de segurança. |
| `LICENSE` | Licença principal. |
| `LICENSES.md` | Registro de licenças aplicáveis. |
| `THIRD-PARTY-LICENSES.md` | Registro de licenças de terceiros. |
| `COPYRIGHT.md` | Informações de copyright. |
| `TRADEMARK-POLICY.md` | Política de marcas. |

Alguns desses arquivos podem estar reservados para preenchimento posterior. Eles
não substituem os documentos normativos em `docs/`.

---

## Relação com o repositório

Estrutura relevante:

```text
capi/
├── capi-docs/
└── capi-lang/
```

`capi-docs` documenta o projeto.

`capi-lang` contém a implementação oficial em Rust, incluindo os crates iniciais
do compilador e o executável `capic`.

Mudanças em especificação, ADRs, engenharia ou progresso de stages devem ser
refletidas nos índices correspondentes e registradas no `CHANGELOG.md` quando
forem relevantes.
