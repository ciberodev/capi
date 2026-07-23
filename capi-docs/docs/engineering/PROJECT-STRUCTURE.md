# Project Structure

**Projeto:** Linguagem Capi
**Documento:** PROJECT-STRUCTURE
**Status:** Proposto
**Stage:** Stage 0 — Fundação do projeto
**Natureza:** Documento de engenharia bloqueante
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define a organização física inicial do projeto Capi.

Seu objetivo é estabelecer onde cada tipo de artefato deve viver, quais responsabilidades pertencem a cada diretório e quais fronteiras devem ser preservadas durante a criação do workspace Rust da implementação oficial.

Este documento orienta a estrutura do repositório.

Ele não define a arquitetura interna dos crates, que pertence a `WORKSPACE-ARCHITECTURE.md`, `COMPILER-ARCHITECTURE.md` e `COMPONENT-RESPONSIBILITIES.md`.

---

## 2. Escopo

Este documento cobre:

* diretórios principais do repositório;
* separação entre documentação e implementação;
* estrutura esperada de `capi-docs`;
* estrutura inicial esperada de `capi-lang`;
* responsabilidades de diretórios;
* regras para inclusão de novos diretórios;
* relação com testes, exemplos, scripts e bootstrap.

Este documento não cobre:

* nomes finais de todos os crates;
* APIs internas;
* layout detalhado de módulos Rust;
* estrutura de pacotes Capi de usuário;
* formato do manifesto `capi.toml`;
* pipeline de compilação.

---

## 3. Princípios de organização

A estrutura do projeto deve seguir os princípios definidos em `ENGINEERING-PRINCIPLES.md`.

Em particular:

* separar garantias da linguagem de mecanismos da implementação;
* manter a especificação próxima da implementação;
* preservar rastreabilidade entre docs, ADRs, código e testes;
* permitir evolução incremental;
* manter fronteiras arquiteturais claras;
* evitar acoplamento entre frontend, runtime, toolchain e backends;
* facilitar bootstrap e futura auto-hospedagem.

A organização física deve tornar explícita a responsabilidade de cada área.

Arquivos não devem ser posicionados por conveniência local quando isso enfraquecer fronteiras arquiteturais.

---

## 4. Estrutura geral do repositório

O projeto é organizado inicialmente em dois diretórios principais:

```text
capi/
├── capi-docs/
│   └── documentação do projeto
│
└── capi-lang/
    └── implementação oficial
```

Essa separação existe para distinguir:

```text
capi-docs
Fonte documental, normativa, decisória e operacional.

capi-lang
Fonte executável da implementação oficial.
```

O desenvolvimento deve preservar essa separação.

Documentos normativos não devem ser movidos para `capi-lang`.

Código da implementação oficial não deve ser mantido em `capi-docs`, exceto exemplos documentais, snippets ou templates explicitamente classificados como documentação.

---

## 5. Diretório `capi-docs`

`capi-docs` contém a documentação oficial do projeto.

Sua responsabilidade é registrar:

* especificação da linguagem;
* especificação de implementação;
* documentação de engenharia;
* ADRs;
* RFCs;
* governança;
* templates;
* políticas do projeto;
* planejamento;
* critérios de validação.

Estrutura esperada:

```text
capi-docs/
├── docs/
│   ├── adr/
│   ├── engineering/
│   ├── governance/
│   ├── rfc/
│   ├── specification/
│   └── templates/
├── CHANGELOG.md
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── LICENSE
├── SECURITY.md
└── README.md
```

O conteúdo de `capi-docs` deve permanecer legível sem depender da existência de build da implementação.

Documentos de engenharia podem fazer referência a caminhos futuros em `capi-lang`, mas devem deixar claro quando a estrutura ainda está em criação.

---

## 6. Diretório `capi-docs/docs/specification`

`capi-docs/docs/specification` contém os documentos normativos e arquiteturais da linguagem e da implementação.

Estrutura:

```text
specification/
├── language/
└── implementation/
```

Responsabilidades:

* `language/` define a linguagem, sua semântica, biblioteca, runtime, ABI, toolchain e ecossistema.
* `implementation/` define os contratos de implementação, arquitetura do compilador, bootstrap e plano de desenvolvimento.

Regras:

* alterações normativas devem seguir o processo de RFC ou alteração de especificação;
* documentos de engenharia não podem contradizer a especificação;
* comportamento implementado deve ser rastreável até documentos desta árvore.

---

## 7. Diretório `capi-docs/docs/engineering`

`capi-docs/docs/engineering` contém a documentação operacional e técnica da implementação oficial.

Ele organiza decisões derivadas da especificação em documentos utilizáveis durante implementação, revisão, teste e manutenção.

Estrutura principal:

```text
engineering/
├── abi/
├── ai-assisted-development/
├── architecture/
├── build-and-ci/
├── compiler/
├── development/
├── observability/
├── performance/
├── planning/
├── release/
├── runtime/
├── security/
├── standard-library/
├── testing/
├── toolchain/
├── ENGINEERING-GLOSSARY.md
├── ENGINEERING-PRINCIPLES.md
├── PROJECT-STRUCTURE.md
└── README.md
```

Documentos diretamente ligados ao Stage 0 podem ficar na raiz de `engineering/` quando forem transversais ao projeto inteiro.

Documentos específicos devem ficar no subdiretório do componente correspondente.

---

## 8. Diretório `capi-docs/docs/adr`

`capi-docs/docs/adr` contém registros de decisões arquiteturais.

ADRs devem ser usados para decisões que afetem:

* organização física;
* workspace Cargo;
* dependências externas;
* separação entre componentes;
* backends;
* runtime;
* testes;
* uso significativo de `unsafe`;
* bootstrap;
* auto-hospedagem.

Um ADR aprovado complementa os documentos de engenharia, mas não pode alterar silenciosamente contratos normativos da especificação.

---

## 9. Diretório `capi-lang`

`capi-lang` contém a implementação oficial da Capi.

No início do Stage 0, esse diretório pode estar vazio.

Ao final do Stage 0, deve conter um workspace Rust funcional e a infraestrutura mínima para evoluir a implementação.

Estrutura inicial esperada:

```text
capi-lang/
├── Cargo.toml
├── Cargo.lock
├── compiler/
├── runtime/
├── std/
├── toolchain/
├── tests/
├── examples/
├── bootstrap/
├── scripts/
├── tools/
├── docs/
└── target/
```

`target/` é diretório gerado e não deve ser versionado.

A existência física de alguns diretórios pode ser adiada até o stage em que forem necessários, desde que a responsabilidade de cada área permaneça preservada.

---

## 10. Diretório `capi-lang/compiler`

`compiler` contém a implementação do compilador oficial.

Responsabilidades:

* driver do compilador;
* sessão de compilação;
* infraestrutura de fontes;
* diagnósticos;
* lexer;
* parser;
* AST;
* HIR;
* análises semânticas;
* MIR;
* interface abstrata de backend;
* backends oficialmente suportados.

O compilador não deve conter:

* implementação principal do runtime;
* implementação principal da biblioteca padrão;
* programas de exemplo;
* documentação normativa;
* scripts operacionais sem relação direta com compilação.

Backends devem permanecer isolados atrás da interface de backend.

Cranelift e LLVM não devem vazar para fases anteriores do pipeline.

---

## 11. Diretório `capi-lang/runtime`

`runtime` contém a implementação do runtime oficial.

Responsabilidades:

* inicialização e encerramento;
* suporte a Domains;
* suporte a ObjectId;
* infraestrutura mínima de execução;
* integração com código gerado;
* abstrações dependentes de plataforma quando aplicável.

O runtime deve permanecer separado do compilador.

O compilador pode depender dos contratos documentados do runtime, mas não deve acessar detalhes privados da sua implementação.

---

## 12. Diretório `capi-lang/std`

`std` contém a biblioteca padrão oficial.

Durante o bootstrap, pode existir coexistência entre:

* implementação temporária em Rust;
* implementação progressiva em Capi.

Essa coexistência deve ser documentada e rastreável.

Ela não representa duas bibliotecas padrão oficiais independentes.

O objetivo é permitir migração gradual até que os componentes previstos sejam implementados em Capi conforme o plano de bootstrap.

---

## 13. Diretório `capi-lang/toolchain`

`toolchain` contém as ferramentas oficiais voltadas ao fluxo de desenvolvimento Capi.

Responsabilidades:

* comando `capi`;
* integração com `capic`;
* criação de projetos;
* build;
* run;
* test;
* formatter;
* documentação;
* gerenciamento inicial de projetos e pacotes.

A toolchain deve orquestrar componentes.

Ela não deve absorver responsabilidades internas do compilador, runtime ou biblioteca padrão.

---

## 14. Diretório `capi-lang/tests`

`tests` contém a suíte oficial de testes da implementação.

Estrutura esperada:

```text
tests/
├── unit/
├── integration/
├── ui/
├── compile-pass/
├── compile-fail/
├── run-pass/
├── codegen/
├── conformance/
├── differential/
├── performance/
└── fixtures/
```

A estrutura física pode evoluir conforme `TEST-STRATEGY.md`.

Regras:

* testes devem ser versionados quando forem fonte de validação;
* artefatos gerados por testes não devem ser versionados;
* casos de regressão devem ser rastreáveis ao erro corrigido;
* testes de conformidade devem permanecer independentes de detalhes internos sempre que possível.

---

## 15. Diretório `capi-lang/examples`

`examples` contém programas Capi usados como documentação executável.

Exemplos devem:

* demonstrar uso real da linguagem;
* acompanhar o subconjunto implementado;
* evitar depender de comportamento experimental sem marcação;
* poder ser usados em validações simples quando aplicável.

Exemplos não substituem testes.

---

## 16. Diretório `capi-lang/bootstrap`

`bootstrap` contém materiais específicos do processo de bootstrap e auto-hospedagem.

Estrutura esperada:

```text
bootstrap/
├── stage0/
├── stage1/
├── stage2/
├── scripts/
├── validation/
└── provenance/
```

Responsabilidades:

* scripts de bootstrap;
* configuração de builds por estágio;
* comparação entre compiladores;
* registro de proveniência de artefatos;
* validação de reprodutibilidade;
* suporte à migração Rust para Capi.

Esse diretório não deve ser usado como depósito genérico de experimentos.

Protótipos devem ter finalidade explícita e localização própria.

---

## 17. Diretório `capi-lang/scripts`

`scripts` contém automações operacionais do repositório.

Exemplos:

* setup local;
* execução padronizada de testes;
* validação de formatação;
* geração de artefatos auxiliares;
* tarefas de CI reutilizáveis localmente.

Scripts devem ser pequenos, documentados e reproduzíveis.

Fluxos essenciais não devem depender de conhecimento implícito fora do repositório.

---

## 18. Diretório `capi-lang/tools`

`tools` contém ferramentas auxiliares de desenvolvimento que não fazem parte diretamente da toolchain pública.

Exemplos:

* geradores internos;
* validadores;
* analisadores de dumps;
* ferramentas de benchmark;
* inspeção de artefatos.

Ferramentas promovidas para uso público devem migrar para `toolchain` ou ser documentadas como parte da toolchain.

---

## 19. Diretório `capi-lang/docs`

`docs` contém documentação técnica local da implementação quando ela precisar estar próxima do código.

Esse diretório pode conter:

* notas de implementação;
* documentação gerada de APIs internas;
* diagramas auxiliares;
* documentação específica do workspace.

Ele não substitui `capi-docs`.

Documentação normativa, ADRs, RFCs e documentos oficiais de engenharia devem permanecer em `capi-docs`.

---

## 20. Arquivos de configuração na raiz de `capi-lang`

A raiz de `capi-lang` deve conter arquivos necessários ao workspace e à automação.

Arquivos esperados no Stage 0:

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml ou política equivalente de toolchain
rustfmt.toml
clippy.toml, quando necessário
README.md
```

Regras:

* `Cargo.lock` deve ser versionado para executáveis e reprodutibilidade do bootstrap;
* versões mínimas de ferramenta devem ser documentadas;
* configurações devem favorecer execução local e CI com os mesmos comandos;
* arquivos gerados devem ser excluídos por `.gitignore`.

---

## 21. Diretórios gerados

Diretórios gerados não devem ser tratados como fonte.

Exemplos:

```text
capi-lang/target/
capi-lang/.cache/
capi-lang/.tmp/
capi-lang/coverage/
```

Esses diretórios devem ser ignorados pelo controle de versão, exceto quando algum artefato gerado for deliberadamente usado como fixture, snapshot ou golden file.

Nesse caso, o motivo deve estar documentado.

---

## 22. Regra para novos diretórios

Um novo diretório de primeiro nível em `capi-lang` somente deve ser criado quando:

* representar uma responsabilidade arquitetural distinta;
* reduzir acoplamento;
* melhorar rastreabilidade;
* evitar mistura de código, testes, documentação e artefatos;
* possuir documentação mínima de finalidade.

Não devem ser criados diretórios genéricos como:

```text
misc/
old/
new/
temp/
stuff/
```

Protótipos devem ser identificados como protótipos e possuir critério de descarte ou promoção.

---

## 23. Relação com o Stage 0

Para concluir o Stage 0, a estrutura do projeto deve permitir:

```bash
cargo test
capic --help
capic --version
```

Isso implica:

* workspace Cargo criado em `capi-lang`;
* crates fundamentais localizados em diretórios coerentes;
* testes iniciais localizáveis;
* scripts ou comandos de desenvolvimento documentados;
* separação inicial entre compilador, runtime, biblioteca padrão e toolchain preservada;
* arquivos gerados fora do controle de versão.

O Stage 0 não exige que todos os diretórios futuros estejam completamente preenchidos.

Ele exige que a estrutura inicial não bloqueie a evolução prevista pelos stages seguintes.

---

## 24. Critérios de aceite deste documento

Este documento estará pronto para orientar o Stage 0 quando:

* refletir a separação entre `capi-docs` e `capi-lang`;
* preservar a estratégia inicial de monorepo;
* definir responsabilidades claras para os diretórios principais;
* não contradizer os Documentos 00 a 28;
* não substituir `WORKSPACE-ARCHITECTURE.md`;
* permitir a criação do workspace Rust inicial;
* orientar a localização de código, testes, scripts, docs e artefatos gerados.

---

## 25. Síntese

A estrutura do projeto deve tornar explícita a arquitetura da implementação oficial.

`capi-docs` preserva a fonte documental do projeto.

`capi-lang` materializa a implementação oficial.

A organização física deve favorecer evolução incremental, testes, bootstrap, rastreabilidade e futura auto-hospedagem sem transformar decisões temporárias em contratos permanentes.
