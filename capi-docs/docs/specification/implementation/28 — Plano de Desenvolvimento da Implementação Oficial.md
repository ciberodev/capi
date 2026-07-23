# 28 — Plano de Desenvolvimento da Implementação Oficial

**Projeto:** Linguagem Capi
**Documento:** 28 — Plano de Desenvolvimento da Implementação Oficial
**Versão:** 1.0
**Status:** Proposta inicial
**Natureza:** Planejamento normativo-operacional da implementação oficial
**Documentos base:** 00 a 27

---

## 1. Finalidade

Este documento estabelece o plano de desenvolvimento da implementação oficial da Linguagem Capi.

Seu objetivo é transformar os contratos definidos pelos Documentos 00 a 27 em uma sequência ordenada, verificável e executável de trabalho.

O plano organiza:

* a preparação da infraestrutura do projeto;
* a elaboração progressiva da documentação de engenharia;
* a implementação dos componentes do compilador;
* a implementação do runtime, da biblioteca padrão e da toolchain;
* a criação e execução dos testes;
* a validação de cada etapa;
* o processo de bootstrap;
* a introdução dos backends Cranelift e LLVM;
* a evolução até a auto-hospedagem;
* a preparação da versão 1.0.

Este documento responde à seguinte pergunta:

> Em que ordem a implementação oficial da Capi deve ser construída, quais entregas são exigidas em cada etapa e quais condições devem ser atendidas antes de avançar para a etapa seguinte?

---

## 2. Escopo

Este documento abrange o desenvolvimento da implementação oficial da Capi, incluindo:

* organização do repositório;
* workspace Rust;
* compilador;
* frontend;
* middle-end;
* backends;
* runtime;
* biblioteca padrão;
* toolchain;
* infraestrutura de testes;
* integração contínua;
* segurança;
* performance;
* bootstrap;
* auto-hospedagem;
* release da versão 1.0.

Este documento não redefine:

* a sintaxe da linguagem;
* a semântica da linguagem;
* o sistema de tipos;
* o modelo de memória;
* os contratos do runtime;
* a ABI;
* a arquitetura conceitual já estabelecida;
* os requisitos normativos definidos nos Documentos 00 a 27.

Sempre que houver conflito, prevalecem os documentos normativos anteriores aplicáveis ao tema.

---

## 3. Relação com os Documentos 00 a 27

Os Documentos 00 a 12 definem a linguagem e seu ecossistema.

Os Documentos 13 a 26 definem a arquitetura e os componentes da implementação.

O Documento 27 define o plano de bootstrap e a arquitetura da implementação oficial.

O presente Documento 28 organiza a execução prática dessas definições.

A relação entre os grupos é:

```text
Documentos 00–12
Definem o que é a linguagem.

Documentos 13–26
Definem como seus componentes devem ser implementados.

Documento 27
Define a arquitetura oficial e o caminho de bootstrap.

Documento 28
Define em que ordem a implementação será realizada.
```

Este documento não substitui o Documento 27.

O Documento 27 continua sendo a referência para:

* arquitetura da implementação oficial;
* estratégia de bootstrap;
* uso de Rust;
* uso inicial de Cranelift;
* introdução posterior de LLVM;
* evolução em direção à auto-hospedagem.

---

## 4. Natureza do documento

Este documento possui natureza normativo-operacional.

Ele é normativo quanto a:

* ordem geral dos stages;
* dependências entre stages;
* entregas obrigatórias;
* critérios de entrada;
* critérios de conclusão;
* rastreabilidade entre documentação, implementação e testes.

Ele não é normativo quanto a detalhes internos que pertençam aos documentos de engenharia de cada componente.

Por exemplo, este documento pode exigir a criação do documento `SOURCE-MODEL.md`, mas não deve repetir nele a definição completa de `SourceId`, `SourceFile` ou `SourceMap`.

---

## 5. Organização do repositório

A implementação oficial é organizada em dois diretórios principais:

```text
CAPI/
├── capi-docs/
│   └── documentação do projeto
│
└── capi-lang/
    └── fontes da implementação oficial
```

O diretório `capi-docs` contém:

* especificações;
* documentação de engenharia;
* ADRs;
* RFCs;
* documentos de governança;
* documentos de planejamento;
* modelos documentais.

O diretório `capi-lang` contém:

* workspace Rust;
* código do compilador;
* runtime;
* biblioteca padrão;
* toolchain;
* testes;
* exemplos;
* scripts;
* arquivos de build;
* configuração de integração contínua.

---

## 6. Conceito de stage

O desenvolvimento é dividido em stages.

Um stage representa uma unidade de evolução do projeto com:

* objetivo definido;
* dependências explícitas;
* entregas documentais;
* entregas de infraestrutura;
* entregas de implementação;
* entregas de testes;
* critérios objetivos de conclusão.

Um stage somente pode ser formalmente iniciado quando seus critérios de entrada forem atendidos.

Um stage somente pode ser formalmente concluído quando todas as suas entregas obrigatórias e critérios de conclusão forem satisfeitos.

### 6.1 Relação entre stages e marcos demonstráveis

Os stages deste documento são unidades de planejamento e validação.

Além deles, a implementação deve preservar marcos demonstráveis, isto é, pontos em que o compilador ou a toolchain produzem um resultado observável, reproduzível e verificável.

Um marco demonstrável pode corresponder a:

* um comando funcional;
* um dump determinístico de uma representação;
* um diagnóstico estruturado;
* um teste negativo aprovado;
* um primeiro executável nativo;
* uma comparação entre backends;
* uma etapa de bootstrap reproduzível.

Esses marcos não substituem os stages, mas impedem que o desenvolvimento avance apenas por acumulação documental ou por implementação interna não demonstrável.

Cada stage deve declarar, quando aplicável, pelo menos um resultado demonstrável.

### 6.2 Relação com os milestones do Documento 27

O Documento 27 define marcos técnicos da implementação oficial.

Os stages deste documento detalham esses marcos em uma ordem operacional mais granular.

A relação geral é:

```text
Stage 0
Corresponde à fundação do projeto e ao M0 do Documento 27.

Stages 1 e 2
Correspondem à análise léxica, sintática e AST.

Stages 3 e 4
Correspondem à HIR, resolução de nomes e tipagem.

Stages 5, 6 e 7
Consolidam objetos, ownership, regiões e Domains antes da MIR executável.

Stages 8 e 9
Conduzem à MIR, runtime mínimo e ABI aplicável ao subconjunto.

Stages 10 e 11
Consolidam biblioteca padrão mínima e backend Cranelift.

Stages 12, 13 e 14
Consolidam toolchain, pacotes e ferramentas.

Stages 15, 16, 17, 18 e 19
Conduzem conformidade, bootstrap, LLVM, auto-hospedagem e versão 1.0.
```

Quando houver diferença de granularidade, os milestones do Documento 27 continuam sendo a referência arquitetural e os stages deste documento definem a execução prática.

---

## 7. Tipos de entrega

Cada stage pode conter cinco categorias de entrega.

### 7.1 Infraestrutura

Inclui:

* diretórios;
* configuração de build;
* configuração de CI;
* ferramentas;
* scripts;
* ambientes;
* organização de crates;
* arquivos de configuração.

### 7.2 Documentação

Inclui:

* documentos de engenharia;
* ADRs;
* planos;
* políticas;
* guias;
* especificações internas;
* critérios de teste.

### 7.3 Implementação

Inclui:

* módulos;
* crates;
* APIs;
* estruturas de dados;
* algoritmos;
* integração entre componentes;
* executáveis;
* bibliotecas.

### 7.4 Testes e validação

Inclui:

* testes unitários;
* testes de integração;
* testes de conformidade;
* testes positivos;
* testes negativos;
* testes de regressão;
* testes de performance;
* validações manuais ou automatizadas.

### 7.5 Critérios de conclusão

Definem as condições objetivas necessárias para considerar o stage encerrado.

---

## 8. Regra de precedência entre documentação e implementação

Documentos que estabelecem contratos arquiteturais ou de implementação devem ser concluídos antes do código correspondente.

A ordem padrão, respeitada a classificação de maturidade documental definida a seguir, é:

```text
1. Definir o objetivo do stage.
2. Preencher a documentação exigida.
3. Revisar e aprovar a documentação.
4. Preparar a infraestrutura.
5. Implementar os componentes.
6. Executar os testes.
7. Registrar decisões emergentes.
8. Validar os critérios de conclusão.
9. Encerrar o stage.
```

Documentos puramente operacionais ou de consolidação podem ser refinados durante o desenvolvimento.

Entretanto, decisões fundamentais não devem ser deixadas exclusivamente para o código.

### 8.1 Maturidade documental mínima

Nem todo documento obrigatório possui o mesmo papel no início de um stage.

Para conduzir o projeto sem paralisar a evolução incremental, os documentos obrigatórios devem ser classificados em três grupos:

```text
Bloqueantes
Definem contratos sem os quais a implementação correspondente não deve começar.

Operacionais
Definem procedimentos, comandos, organização, execução local, CI ou rotinas de validação.

Consolidação
Registram sínteses, glossários, políticas ou guias que podem amadurecer durante o stage.
```

Um documento bloqueante deve estar `Aprovado` antes da implementação que dele depende.

Um documento operacional deve estar suficientemente definido antes de ser usado como base para automação ou validação.

Um documento de consolidação pode evoluir durante o stage, mas deve estar `Aprovado` antes da conclusão formal do stage.

Essa regra não autoriza implementação sem contrato. Ela apenas distingue documentação que bloqueia código de documentação que consolida decisões já tomadas.

---

## 9. Estado dos documentos de engenharia

Os documentos de engenharia podem assumir os seguintes estados:

```text
Vazio
Arquivo criado, sem conteúdo.

Em elaboração
Documento em processo de escrita.

Proposto
Documento completo, ainda não aprovado.

Aprovado
Documento validado para orientar a implementação.

Implementado
As definições do documento já possuem implementação correspondente.

Revisão necessária
O documento precisa ser atualizado em razão de mudanças aprovadas.
```

Um documento obrigatório classificado como bloqueante para determinado componente deve estar no estado `Aprovado` antes da implementação correspondente.

Documentos operacionais ou de consolidação seguem a regra de maturidade documental definida neste documento.

---

## 10. Registro de decisões

Decisões arquiteturais relevantes devem ser registradas por meio de ADRs.

Um ADR deve ser criado quando houver:

* escolha entre alternativas arquiteturais relevantes;
* mudança de direção já estabelecida;
* adoção de dependência estrutural;
* decisão sobre organização de crates;
* decisão sobre APIs internas estáveis;
* uso significativo de código `unsafe`;
* alteração no modelo de backend;
* alteração no processo de bootstrap.

Mudanças na linguagem ou nos contratos normativos devem utilizar o processo de RFC e de alteração de especificação.

---

## 11. Dependências entre stages

A ordem geral dos stages é sequencial.

Um stage pode iniciar atividades preparatórias antes da conclusão total do anterior, desde que:

* não dependa de contratos ainda instáveis;
* não antecipe implementação incompatível com o stage anterior;
* não comprometa os critérios de conclusão;
* a sobreposição seja documentada.

A conclusão formal deve seguir a ordem definida neste documento.

### 11.1 Cortes verticais controlados

Atividades preparatórias podem assumir a forma de cortes verticais controlados.

Um corte vertical é uma implementação limitada que atravessa mais de uma camada do compilador para validar uma hipótese crítica, como:

* formato de diagnóstico;
* passagem de spans entre fases;
* lowering de uma construção mínima;
* representação de layout;
* interface abstrata de backend;
* geração de um executável mínimo;
* integração entre runtime e backend.

Um corte vertical não conclui automaticamente stages posteriores.

Ele deve permanecer limitado, rastreável e compatível com os contratos já aprovados.

Quando revelar uma decisão arquitetural nova, essa decisão deve ser registrada antes de integrar o resultado à implementação oficial.

---

## 12. Critérios gerais de entrada

Antes do início de qualquer stage, devem estar disponíveis:

* os documentos normativos aplicáveis;
* as entregas obrigatórias dos stages anteriores;
* os documentos de engenharia exigidos;
* o ambiente de desenvolvimento funcional;
* as dependências técnicas necessárias;
* os critérios de aceite do stage.

---

## 13. Critérios gerais de conclusão

Um stage somente pode ser concluído quando:

* todos os documentos obrigatórios estiverem aprovados;
* todas as entregas de infraestrutura estiverem operacionais;
* todas as entregas de implementação estiverem concluídas;
* todos os testes obrigatórios estiverem aprovados;
* não houver erro bloqueador conhecido;
* a CI estiver verde;
* os ADRs necessários estiverem registrados;
* os documentos estiverem coerentes com o código;
* o resultado esperado do stage puder ser demonstrado;
* as limitações conhecidas do subconjunto implementado estiverem documentadas.

---

# Parte II — Stages de Desenvolvimento

---

## 14. Stage 0 — Fundação do projeto

### 14.1 Objetivo

Preparar a infraestrutura mínima necessária para o desenvolvimento da implementação oficial.

Ao final deste stage, o projeto deve possuir um workspace Rust funcional, organização inicial de crates, integração contínua, padrões de desenvolvimento e um executável mínimo do compilador.

O compilador ainda não precisa processar código Capi.

### 14.2 Dependências

* Documentos 00 a 27 concluídos;
* Documento 28 aprovado;
* repositórios `capi-docs` e `capi-lang` disponíveis;
* ambiente Rust definido.

### 14.3 Entregas de infraestrutura

```text
1. Criar o workspace Cargo em capi-lang.
2. Criar a estrutura inicial de crates.
3. Configurar cargo fmt.
4. Configurar cargo clippy.
5. Configurar execução de testes.
6. Configurar integração contínua.
7. Configurar documentação Rust.
8. Configurar scripts de desenvolvimento.
9. Definir política inicial de dependências.
10. Definir versões mínimas das ferramentas.
```

### 14.4 Documentos obrigatórios

Os documentos obrigatórios do Stage 0 são divididos por função.

Documentos bloqueantes para a criação do workspace e dos crates fundamentais:

```text
ENGINEERING-PRINCIPLES.md
PROJECT-STRUCTURE.md
COMPILER-ARCHITECTURE.md
WORKSPACE-ARCHITECTURE.md
COMPONENT-RESPONSIBILITIES.md
DEPENDENCY-RULES.md
DEVELOPMENT-SETUP.md
BUILD-SYSTEM.md
TEST-STRATEGY.md
DEFINITION-OF-DONE.md
```

Documentos operacionais ou de consolidação exigidos para a conclusão formal do Stage 0:

```text
ENGINEERING-GLOSSARY.md
COMPILATION-PIPELINE.md
BUILDING-FROM-SOURCE.md
CODING-STANDARDS.md
RUST-STYLE-GUIDE.md
```

Os documentos de consolidação podem amadurecer junto com a criação do workspace, mas devem estar aprovados antes do encerramento do stage.

### 14.5 ADRs obrigatórios

```text
ADR — Rust como Linguagem da Implementação Oficial
ADR — Organização Física do Repositório
ADR — Organização da Implementação em Workspace Cargo
ADR — Separação entre Frontend, Middle-end e Backend
ADR — Política de Dependências Externas
ADR — Estratégia Inicial de Testes
```

### 14.6 Entregas de implementação

```text
1. Criar o executável capic.
2. Criar o crate do driver.
3. Criar o crate de sessão de compilação.
4. Criar o crate de diagnósticos.
5. Criar o crate de infraestrutura de fontes.
6. Implementar capic --help.
7. Implementar capic --version.
8. Implementar leitura inicial de argumentos.
9. Implementar inicialização da sessão.
10. Implementar tratamento básico de erros internos.
```

### 14.7 Testes e validação

```text
1. Compilação completa do workspace.
2. Execução de cargo fmt.
3. Execução de cargo clippy.
4. Execução da suíte inicial.
5. Teste de capic --help.
6. Teste de capic --version.
7. Teste de arquivo inexistente.
8. Validação da CI.
```

### 14.8 Critérios de conclusão

* o workspace compila sem erros;
* a CI está operacional;
* `cargo fmt` não encontra divergências;
* `cargo clippy` não encontra erros bloqueadores;
* todos os testes passam;
* `capic --help` funciona;
* `capic --version` funciona;
* a arquitetura inicial de crates está documentada;
* as dependências respeitam as regras definidas;
* os documentos bloqueantes e de consolidação do stage estão aprovados;
* existe registro de progresso do Stage 0.

### 14.9 Resultado esperado

```text
O projeto de implementação existe, compila e pode evoluir com segurança.
```

Resultado demonstrável mínimo:

```bash
capic --help
capic --version
cargo test
```

---

## 15. Stage 1 — Fontes, diagnósticos e lexer

### 15.1 Objetivo

Implementar a infraestrutura de código-fonte, o sistema inicial de diagnósticos e a análise léxica.

Ao final deste stage, o compilador deve ler um arquivo Capi e produzir tokens ou erros léxicos estruturados.

### 15.2 Documentos obrigatórios

```text
SOURCE-MODEL.md
SOURCE-MAP.md
SPANS-AND-LOCATIONS.md
UNICODE-AND-ENCODING.md
TOKEN-MODEL.md
LEXER-IMPLEMENTATION.md
DIAGNOSTIC-DATA-MODEL.md
DIAGNOSTIC-ARCHITECTURE.md
DIAGNOSTIC-STYLE-GUIDE.md
LEXER-TESTS.md
```

### 15.3 Entregas de infraestrutura

```text
1. Consolidar o crate de fontes.
2. Consolidar o crate de diagnósticos.
3. Criar o crate do lexer.
4. Criar diretório de casos de teste léxico.
5. Criar infraestrutura de snapshots ou UI tests.
```

### 15.4 Entregas de implementação

```text
1. Implementar SourceId.
2. Implementar SourceFile.
3. Implementar SourceMap.
4. Implementar Span.
5. Implementar linha e coluna.
6. Implementar o modelo de tokens.
7. Implementar o lexer.
8. Implementar identificadores.
9. Implementar palavras-chave.
10. Implementar literais.
11. Implementar operadores.
12. Implementar delimitadores.
13. Implementar comentários.
14. Implementar erros léxicos.
15. Implementar dump de tokens.
```

### 15.5 Testes e validação

```text
1. Testes de SourceMap.
2. Testes de spans.
3. Testes de Unicode.
4. Testes de identificadores.
5. Testes de palavras-chave.
6. Testes de literais.
7. Testes de comentários.
8. Testes de tokens inválidos.
9. Testes de posição dos diagnósticos.
10. Testes compile-fail léxicos.
```

### 15.6 Critérios de conclusão

* arquivos válidos são lidos corretamente;
* posições de erro são precisas;
* todos os tokens do subconjunto inicial são reconhecidos;
* entradas inválidas produzem diagnósticos estruturados;
* não há pânico em entradas malformadas;
* todos os testes obrigatórios passam.

### 15.7 Resultado esperado

```bash
capic --emit tokens arquivo.capi
```

---

## 16. Stage 2 — Parser e AST

### 16.1 Objetivo

Implementar o parser e a representação sintática abstrata.

Ao final deste stage, código Capi sintaticamente válido deve ser transformado em AST.

### 16.2 Documentos obrigatórios

```text
AST-MODEL.md
PARSER-IMPLEMENTATION.md
PARSER-RECOVERY.md
AST-LOWERING.md
PARSER-TESTS.md
```

### 16.3 Entregas de implementação

```text
1. Implementar os nós da AST.
2. Implementar parser de módulos.
3. Implementar parser de declarações.
4. Implementar parser de classes.
5. Implementar parser de funções.
6. Implementar parser de tipos.
7. Implementar parser de expressões.
8. Implementar parser de comandos.
9. Implementar precedência de operadores.
10. Implementar recuperação de erros.
11. Implementar dump da AST.
```

### 16.4 Testes e validação

```text
1. Testes de declarações.
2. Testes de expressões.
3. Testes de precedência.
4. Testes de tipos.
5. Testes de classes.
6. Testes de erros sintáticos.
7. Testes de recuperação.
8. Testes de AST.
```

### 16.5 Critérios de conclusão

* o subconjunto sintático inicial é aceito;
* entradas inválidas produzem diagnósticos adequados;
* o parser consegue continuar após erros recuperáveis;
* a AST preserva spans;
* o dump da AST é determinístico;
* todos os testes passam.

### 16.6 Resultado esperado

```bash
capic --emit ast arquivo.capi
```

---

## 17. Stage 3 — HIR e resolução de nomes

### 17.1 Objetivo

Implementar a representação semântica inicial e a resolução de símbolos.

### 17.2 Documentos obrigatórios

```text
HIR-MODEL.md
SYMBOL-MODEL.md
SCOPE-MODEL.md
NAME-RESOLUTION.md
AST-LOWERING.md
SEMANTIC-TESTS.md
```

### 17.3 Entregas de implementação

```text
1. Implementar lowering de AST para HIR.
2. Implementar IDs internos.
3. Implementar tabelas de símbolos.
4. Implementar escopos.
5. Implementar módulos.
6. Implementar imports.
7. Implementar resolução de nomes.
8. Detectar símbolos duplicados.
9. Detectar referências inexistentes.
10. Detectar ambiguidades.
11. Implementar dump da HIR.
```

### 17.4 Critérios de conclusão

* todos os nomes do subconjunto inicial são resolvidos;
* erros de resolução são diagnosticados;
* a HIR não depende diretamente da estrutura da AST;
* símbolos possuem identidade interna estável;
* todos os testes passam.

### 17.5 Resultado esperado

```bash
capic --emit hir arquivo.capi
```

---

## 18. Stage 4 — Sistema de tipos

### 18.1 Objetivo

Implementar a representação, inferência e verificação de tipos.

### 18.2 Documentos obrigatórios

```text
TYPE-MODEL.md
TYPE-INTERNING.md
TYPE-INFERENCE.md
TYPE-CHECKING-PIPELINE.md
SUBTYPING-AND-COERCIONS.md
GENERICS-IMPLEMENTATION.md
```

### 18.3 Entregas de implementação

```text
1. Implementar tipos internos.
2. Implementar interning de tipos.
3. Implementar inferência.
4. Implementar verificação de tipos.
5. Implementar subtipagem.
6. Implementar coerções.
7. Implementar resolução de chamadas.
8. Implementar sobrecarga aplicável.
9. Implementar generics do subconjunto inicial.
10. Implementar diagnósticos de tipo.
```

### 18.4 Critérios de conclusão

* programas válidos são tipados corretamente;
* programas inválidos são rejeitados;
* resultados de inferência são determinísticos;
* subtipagem respeita a especificação;
* todos os testes semânticos passam.

### 18.5 Resultado esperado

```bash
capic check arquivo.capi
```

---

## 19. Stage 5 — Modelo de objetos

### 19.1 Objetivo

Implementar classes, identidade, herança, subtipagem nominal e despacho dinâmico.

### 19.2 Documentos obrigatórios

```text
OBJECT-MODEL.md
OBJECT-LAYOUT.md
OBJECT-IDENTITY.md
VTABLES.md
DYNAMIC-DISPATCH.md
INHERITANCE-IMPLEMENTATION.md
```

### 19.3 Entregas de implementação

```text
1. Implementar representação de classes.
2. Implementar campos.
3. Implementar métodos.
4. Implementar construtores aplicáveis.
5. Implementar herança.
6. Implementar override.
7. Validar hierarquias.
8. Implementar identidade de objetos.
9. Definir layout de objetos.
10. Definir modelo de vtables.
11. Preparar despacho dinâmico.
```

### 19.4 Critérios de conclusão

* hierarquias válidas são aceitas;
* hierarquias inválidas são rejeitadas;
* override é validado;
* identidade de objetos é preservada;
* subtipagem nominal é coerente;
* todos os testes passam.

---

## 20. Stage 6 — Ownership, borrowing e regiões

### 20.1 Objetivo

Implementar as garantias fundamentais de segurança de memória.

### 20.2 Documentos obrigatórios

```text
PLACE-AND-ACCESS-PATHS.md
OWNERSHIP-MODEL.md
REGION-ANALYSIS.md
BORROW-CHECKER.md
ESCAPE-ANALYSIS.md
DROP-SEMANTICS.md
OWNERSHIP-TESTS.md
```

### 20.3 Entregas de implementação

```text
1. Implementar places.
2. Implementar movimentos.
3. Implementar cópias.
4. Implementar empréstimos.
5. Implementar empréstimos mutáveis.
6. Detectar conflitos de acesso.
7. Implementar regiões.
8. Implementar descarte.
9. Implementar análise de escape.
10. Implementar diagnósticos de memória.
```

### 20.4 Critérios de conclusão

* usos após move são rejeitados;
* empréstimos conflitantes são rejeitados;
* acessos válidos são aceitos;
* descarte ocorre conforme o modelo;
* escapes inválidos são rejeitados;
* todos os testes passam.

As regras aceitas ou rejeitadas neste stage devem ser próprias da Capi.

Nenhuma aceitação ou rejeição deve depender apenas de uma propriedade acidental da implementação em Rust.

---

## 21. Stage 7 — Domains

### 21.1 Objetivo

Implementar a semântica e a infraestrutura dos Domains.

### 21.2 Documentos obrigatórios

```text
DOMAIN-IMPLEMENTATION.md
DOMAIN-RUNTIME.md
DOMAIN-TESTS.md
MEMORY-ALLOCATION.md
```

### 21.3 Entregas de implementação

```text
1. Implementar tipos de Domain.
2. Implementar criação de Domain.
3. Implementar associação de objetos.
4. Implementar descarte de Domain.
5. Implementar restrições de escape.
6. Integrar Domains ao ownership.
7. Implementar modelo inicial de alocação.
8. Implementar diagnósticos específicos.
```

### 21.4 Critérios de conclusão

* objetos são associados corretamente a Domains;
* escapes inválidos são rejeitados;
* descarte de Domain é determinístico;
* integração com ownership é consistente;
* todos os testes passam.

---

## 22. Stage 8 — MIR

### 22.1 Objetivo

Implementar a representação intermediária independente de backend.

### 22.2 Documentos obrigatórios

```text
MIR-MODEL.md
MIR-INVARIANTS.md
MIR-LOWERING.md
MIR-VALIDATION.md
MIR-PASSES.md
MIR-DUMP-FORMAT.md
MIR-TESTS.md
```

### 22.3 Entregas de implementação

```text
1. Implementar funções MIR.
2. Implementar blocos básicos.
3. Implementar instruções.
4. Implementar terminadores.
5. Implementar fluxo de controle.
6. Implementar lowering da HIR.
7. Implementar validação estrutural.
8. Implementar dump textual.
9. Implementar passes iniciais.
10. Implementar verificações de invariantes.
```

### 22.4 Critérios de conclusão

* programas válidos geram MIR;
* a MIR respeita seus invariantes;
* a validação detecta estruturas inválidas;
* o dump é determinístico;
* a MIR não depende de Cranelift ou LLVM;
* todos os testes passam.

### 22.5 Resultado esperado

```bash
capic --emit mir arquivo.capi
```

---

## 23. Stage 9 — ABI, runtime mínimo e codegen básico

### 23.1 Objetivo

Definir e implementar os contratos executáveis entre a MIR, o backend e o runtime.

Neste stage, a estabilidade dos contratos é exigida para o subconjunto executável em implementação.

Essa estabilidade não transforma layouts, ABI interna ou APIs do runtime em contratos públicos permanentes além do escopo definido pelos documentos normativos aplicáveis.

### 23.2 Documentos obrigatórios

```text
RUNTIME-RESPONSIBILITIES.md
RUNTIME-ARCHITECTURE.md
RUNTIME-API.md
RUNTIME-INITIALIZATION.md
ABI-IMPLEMENTATION.md
DATA-LAYOUT.md
CALLING-CONVENTIONS.md
NAME-MANGLING.md
SYMBOL-VISIBILITY.md
CODEGEN-ARCHITECTURE.md
BACKEND-INTERFACE.md
```

### 23.3 Entregas de implementação

```text
1. Criar runtime mínimo.
2. Implementar inicialização do runtime.
3. Implementar funções intrínsecas iniciais.
4. Definir layout de dados.
5. Implementar convenções de chamadas.
6. Implementar name mangling.
7. Implementar interface abstrata de backend.
8. Preparar a MIR para codegen.
```

### 23.4 Critérios de conclusão

* runtime e compilador compartilham contratos estáveis para o subconjunto implementado;
* layouts são definidos;
* chamadas possuem convenção explícita;
* símbolos são gerados de forma determinística;
* a interface de backend está operacional;
* limitações temporárias da ABI interna estão documentadas;
* todos os testes passam.

---

## 24. Stage 10 — Biblioteca padrão mínima

### 24.1 Objetivo

Implementar o núcleo mínimo da biblioteca padrão necessário para programas básicos e para o bootstrap.

### 24.2 Documentos obrigatórios

```text
STANDARD-LIBRARY-ARCHITECTURE.md
MODULE-ORGANIZATION.md
CORE-TYPES.md
INTRINSICS.md
API-DESIGN-GUIDELINES.md
PLATFORM-DEPENDENT-APIS.md
STABILITY-POLICY.md
```

### 24.3 Entregas de implementação

```text
1. Implementar tipos fundamentais.
2. Implementar Option.
3. Implementar Result.
4. Implementar String.
5. Implementar coleções mínimas.
6. Implementar entrada e saída.
7. Integrar biblioteca e runtime.
8. Implementar intrínsecos necessários.
```

### 24.4 Critérios de conclusão

* programas básicos utilizam a biblioteca padrão;
* APIs mínimas estão disponíveis;
* integração com runtime é funcional;
* o subconjunto necessário ao bootstrap está documentado;
* todos os testes passam.

---

## 25. Stage 11 — Backend Cranelift

### 25.1 Objetivo

Gerar executáveis nativos usando Cranelift.

### 25.2 Documentos obrigatórios

```text
CRANELIFT-BACKEND.md
TARGETS.md
OBJECT-FILES.md
LINKING.md
CODEGEN-TESTS.md
RUN-PASS-TESTS.md
```

### 25.3 Entregas de implementação

```text
1. Traduzir MIR para Cranelift IR.
2. Gerar funções.
3. Gerar chamadas.
4. Gerar controle de fluxo.
5. Gerar tipos primitivos.
6. Gerar operações básicas.
7. Gerar objetos do subconjunto inicial.
8. Gerar arquivos objeto.
9. Implementar linking.
10. Produzir executáveis.
```

### 25.4 Critérios de conclusão

* programas do subconjunto inicial geram código nativo;
* executáveis podem ser produzidos;
* resultados são corretos;
* falhas de codegen são diagnosticadas;
* o primeiro pipeline completo é validado por testes run-pass;
* todos os testes de execução passam.

### 25.5 Resultado esperado

```bash
capic build arquivo.capi
./arquivo
```

---

## 26. Stage 12 — Toolchain mínima

### 26.1 Objetivo

Permitir a criação, compilação, execução e teste de projetos Capi.

### 26.2 Documentos obrigatórios

```text
TOOLCHAIN-ARCHITECTURE.md
CLI-DESIGN.md
COMMAND-LINE-CONVENTIONS.md
CONFIGURATION.md
PROJECT-MANIFEST.md
TEST-RUNNER-ARCHITECTURE.md
FORMATTER-ARCHITECTURE.md
```

### 26.3 Entregas de implementação

```text
1. Implementar capi new.
2. Implementar capi build.
3. Implementar capi run.
4. Implementar capi test.
5. Implementar manifesto inicial.
6. Integrar capi e capic.
7. Implementar formatter inicial.
8. Implementar test runner.
```

### 26.4 Critérios de conclusão

* um projeto pode ser criado;
* um projeto pode ser compilado;
* um projeto pode ser executado;
* testes podem ser executados;
* o manifesto é reconhecido;
* todos os testes passam.

---

## 27. Stage 13 — Pacotes e dependências

### 27.1 Objetivo

Implementar resolução de dependências e gerenciamento de pacotes.

### 27.2 Documentos obrigatórios

```text
LOCKFILE-FORMAT.md
PACKAGE-MANAGER-ARCHITECTURE.md
DEPENDENCY-RESOLUTION.md
PACKAGE-REGISTRY.md
```

### 27.3 Entregas de implementação

```text
1. Implementar manifesto completo.
2. Implementar lockfile.
3. Implementar resolução de versões.
4. Implementar cache de pacotes.
5. Implementar dependências locais.
6. Implementar registro inicial.
7. Implementar validação de integridade.
```

### 27.4 Critérios de conclusão

* dependências são resolvidas deterministicamente;
* o lockfile é reproduzível;
* dependências locais funcionam;
* pacotes podem ser obtidos do registro;
* todos os testes passam.

---

## 28. Stage 14 — Ferramentas de desenvolvimento

### 28.1 Objetivo

Implementar ferramentas de produtividade e integração com editores.

### 28.2 Documentos obrigatórios

```text
DOCUMENTATION-GENERATOR.md
LSP-ARCHITECTURE.md
OUTPUT-FORMATS.md
COMPILER-DUMP-FLAGS.md
```

### 28.3 Entregas de implementação

```text
1. Implementar capi doc.
2. Gerar documentação de APIs.
3. Implementar suporte inicial a LSP.
4. Implementar saída estruturada.
5. Implementar saída JSON.
6. Implementar flags de dump.
7. Integrar diagnósticos com editores.
```

### 28.4 Critérios de conclusão

* documentação de APIs pode ser gerada;
* diagnósticos podem ser consumidos por ferramentas;
* o LSP atende ao subconjunto inicial;
* todos os testes passam.

---

## 29. Stage 15 — Conformidade, robustez e performance

### 29.1 Objetivo

Validar sistematicamente a implementação contra a especificação e fortalecer sua robustez.

Este stage consolida a conformidade como disciplina formal.

Ele não adia a criação de testes de conformidade: cada stage anterior deve introduzir os testes positivos, negativos e de regressão necessários ao componente implementado.

### 29.2 Documentos obrigatórios

```text
CONFORMANCE-SUITE.md
FUZZING.md
DIFFERENTIAL-TESTS.md
PERFORMANCE-GOALS.md
BENCHMARKS.md
COMPILATION-PERFORMANCE.md
GENERATED-CODE-PERFORMANCE.md
THREAT-MODEL.md
COMPILER-HARDENING.md
INPUT-LIMITS.md
```

### 29.3 Entregas de implementação

```text
1. Criar suíte de conformidade.
2. Implementar fuzzing.
3. Criar testes diferenciais.
4. Criar benchmarks.
5. Medir tempo de compilação.
6. Medir desempenho do código gerado.
7. Implementar limites de entrada.
8. Implementar hardening.
9. Criar política de regressões.
```

### 29.4 Critérios de conclusão

* a implementação é validada contra a especificação;
* entradas adversariais são testadas;
* métricas de performance são registradas;
* regressões podem ser detectadas;
* todos os testes passam.

---

## 30. Stage 16 — Bootstrap

### 30.1 Objetivo

Executar o plano de bootstrap definido no Documento 27.

Este stage não redefine o subconjunto inicial nem altera os critérios arquiteturais de bootstrap.

Sua função é tornar o processo executável, reproduzível, auditável e integrado à suíte oficial de testes.

### 30.2 Documentos obrigatórios

```text
BOOTSTRAP-BUILD.md
REPRODUCIBLE-BUILDS.md
ARTIFACT-PROVENANCE.md
```

### 30.3 Entregas de implementação

```text
1. Implementar o subconjunto de bootstrap.
2. Construir o compilador de estágio inicial.
3. Migrar componentes selecionados para Capi.
4. Implementar builds por estágio.
5. Verificar reprodutibilidade.
6. Comparar artefatos.
7. Documentar a cadeia de confiança.
```

### 30.4 Critérios de conclusão

* o processo de bootstrap é executável;
* cada estágio pode ser reproduzido;
* a origem dos artefatos é rastreável;
* os resultados são verificáveis;
* a cadeia de confiança está documentada;
* todos os testes passam.

---

## 31. Stage 17 — Backend LLVM

### 31.1 Objetivo

Implementar LLVM como backend de otimização.

### 31.2 Documentos obrigatórios

```text
LLVM-BACKEND.md
BACKEND-COMPATIBILITY.md
PERFORMANCE-REGRESSION-POLICY.md
```

### 31.3 Entregas de implementação

```text
1. Traduzir MIR para LLVM IR.
2. Implementar geração de código LLVM.
3. Implementar otimizações.
4. Implementar debug information.
5. Comparar resultados com Cranelift.
6. Implementar seleção de backend.
7. Validar compatibilidade.
```

### 31.4 Critérios de conclusão

* programas podem ser compilados pelos dois backends;
* resultados observáveis são compatíveis;
* divergências são documentadas;
* ganhos e custos de performance são medidos;
* todos os testes passam.

---

## 32. Stage 18 — Auto-hospedagem

### 32.1 Objetivo

Tornar a implementação oficial capaz de compilar sua própria implementação em Capi, conforme os limites do Documento 27.

Auto-hospedagem é um marco de maturidade da implementação oficial.

Ela não altera a especificação da linguagem, não elimina automaticamente o compilador de bootstrap em Rust e não autoriza divergência funcional entre componentes equivalentes.

### 32.2 Documentos obrigatórios

```text
SELF-HOSTING-PLAN.md
STAGE-COMPARISON.md
BOOTSTRAP-CHAIN.md
RUST-TO-CAPI-MIGRATION.md
SELF-HOSTING-VALIDATION.md
```

### 32.3 Entregas obrigatórias

```text
1. Implementação parcial ou integral do compilador em Capi.
2. Build por múltiplos estágios.
3. Verificação de equivalência.
4. Testes completos.
5. Cadeia de confiança documentada.
6. Release inicial auto-hospedada.
```

### 32.4 Critérios de conclusão

* o compilador em Capi pode ser compilado;
* o compilador gerado pode compilar uma versão equivalente;
* artefatos podem ser comparados;
* a cadeia de bootstrap é reproduzível;
* testes diferenciais demonstram equivalência funcional quando aplicáveis;
* a dependência operacional da implementação em Rust está removida ou formalmente limitada;
* todos os testes passam.

---

## 33. Stage 19 — Preparação da versão 1.0

### 33.1 Objetivo

Estabilizar a implementação oficial e preparar a primeira versão estável.

### 33.2 Documentos obrigatórios

```text
VERSIONING.md
COMPATIBILITY-POLICY.md
DEPRECATION-POLICY.md
RELEASE-CHANNELS.md
RELEASE-PROCESS.md
RELEASE-CHECKLIST.md
CHANGELOG-POLICY.md
ARTIFACT-SIGNING.md
```

### 33.3 Entregas obrigatórias

```text
1. Estabilizar APIs públicas.
2. Estabilizar contratos de compatibilidade.
3. Consolidar ABI aplicável.
4. Concluir documentação pública.
5. Criar instaladores.
6. Assinar artefatos.
7. Criar release candidate.
8. Corrigir problemas bloqueadores.
9. Publicar a versão 1.0.
```

### 33.4 Critérios de conclusão

* não existem erros bloqueadores conhecidos;
* a suíte de conformidade está aprovada;
* os backends suportados estão validados;
* a documentação pública está concluída;
* artefatos são reproduzíveis e assinados;
* a política de compatibilidade está publicada;
* a versão 1.0 pode ser distribuída.

---

# Parte III — Controle e rastreabilidade

---

## 34. Controle de progresso

Cada stage deve possuir um registro de progresso contendo:

```text
Stage:
Responsável:
Data de início:
Data de conclusão:
Status:
Documentos concluídos:
Infraestrutura concluída:
Implementações concluídas:
Testes concluídos:
ADRs criados:
Pendências:
Riscos:
Resultado da validação:
```

Os estados possíveis de um stage são:

```text
Não iniciado
Preparação
Em desenvolvimento
Em validação
Bloqueado
Concluído
Reaberto
```

---

## 35. Rastreabilidade

Cada entrega de implementação deve ser rastreável até:

* documento normativo aplicável;
* documento de engenharia aplicável;
* issue ou tarefa de implementação;
* pull request;
* testes correspondentes;
* ADR, quando houver decisão arquitetural.

A rastreabilidade mínima esperada é:

```text
Especificação
    ↓
Documento de engenharia
    ↓
Tarefa
    ↓
Código
    ↓
Teste
    ↓
Validação
```

---

## 36. Alterações no plano

Este documento pode ser alterado quando:

* uma dependência técnica mudar;
* um stage precisar ser dividido;
* dois stages precisarem ser consolidados;
* surgir uma entrega não prevista;
* uma decisão arquitetural aprovada alterar a ordem de execução;
* o processo de bootstrap exigir ajuste.

Alterações devem:

* preservar a rastreabilidade;
* registrar a justificativa;
* identificar impactos;
* atualizar stages dependentes;
* não alterar silenciosamente contratos normativos.

Mudanças significativas devem ser acompanhadas de ADR ou RFC, conforme sua natureza.

---

## 37. Regra para documentos não previstos

Caso um novo documento de engenharia seja necessário, ele deve ser:

* associado ao stage correspondente;
* concluído antes da implementação que dele dependa;
* incluído na lista de entregas do stage;
* relacionado aos documentos normativos aplicáveis.

A ausência do documento na versão inicial deste plano não impede sua criação.

---

## 38. Regra para entregas antecipadas

Uma entrega pertencente a stage posterior pode ser antecipada quando:

* reduzir riscos;
* validar uma hipótese crítica;
* permitir um protótipo controlado;
* não criar dependência incompatível;
* não comprometer o planejamento vigente.

Entregas antecipadas não alteram automaticamente a conclusão formal dos stages.

---

## 39. Regra para protótipos

Protótipos podem ser criados para:

* validar APIs;
* avaliar Cranelift;
* avaliar LLVM;
* experimentar modelos de IR;
* medir performance;
* testar estratégias de alocação;
* avaliar algoritmos de ownership.

Protótipos:

* não constituem implementação oficial;
* não substituem documentação aprovada;
* não devem ser integrados diretamente sem revisão;
* podem ser descartados;
* devem ter finalidade explícita.

---

## 40. Regra para uso de inteligência artificial

Ferramentas de inteligência artificial podem colaborar em:

* elaboração de documentos;
* revisão de documentos;
* geração de código;
* criação de testes;
* análise de erros;
* exploração de alternativas;
* revisão de arquitetura.

Toda entrega gerada ou assistida por IA deve:

* ser revisada;
* respeitar os documentos normativos;
* respeitar as políticas de dependências e licenciamento;
* possuir testes;
* ser rastreável;
* não ser considerada correta apenas por ter sido gerada automaticamente.

---

## 41. Condição para início do Stage 0

O Stage 0 pode ser iniciado quando:

```text
1. O Documento 28 estiver aprovado.
2. O repositório capi-lang estiver disponível.
3. O ambiente Rust puder ser instalado ou utilizado.
4. A estrutura documental estiver disponível.
5. Os documentos bloqueantes do Stage 0 puderem ser elaborados e aprovados.
6. As decisões fundamentais de workspace puderem ser registradas.
```

Não é necessário concluir todos os documentos de engenharia do projeto antes de iniciar o Stage 0.

Somente os documentos exigidos pelo Stage 0 devem ser preenchidos inicialmente.

Os documentos de consolidação do Stage 0 podem ser concluídos durante o próprio stage, desde que estejam aprovados antes de seu encerramento formal.

---

## 42. Próxima ação operacional

Após a aprovação deste documento, a próxima ação do Projeto Capi será:

```text
Iniciar o Stage 0 — Fundação do projeto.
```

A primeira sequência de trabalho será:

```text
1. Preencher ENGINEERING-PRINCIPLES.md.
2. Preencher PROJECT-STRUCTURE.md.
3. Preencher COMPILER-ARCHITECTURE.md.
4. Preencher WORKSPACE-ARCHITECTURE.md.
5. Preencher COMPONENT-RESPONSIBILITIES.md.
6. Preencher DEPENDENCY-RULES.md.
7. Preencher DEVELOPMENT-SETUP.md, BUILD-SYSTEM.md, TEST-STRATEGY.md e DEFINITION-OF-DONE.md.
8. Aprovar os ADRs iniciais.
9. Criar o workspace Cargo.
10. Criar os crates fundamentais.
11. Configurar build, testes, fmt, clippy e CI.
12. Implementar capic --help e capic --version.
13. Consolidar glossário, pipeline, guias de estilo e instruções de build.
14. Validar os critérios de conclusão do Stage 0.
```

---

## 43. Resultado final esperado

A execução completa deste plano deve resultar em:

* compilador oficial funcional;
* runtime mínimo;
* biblioteca padrão;
* toolchain;
* backend Cranelift;
* backend LLVM;
* suíte de conformidade;
* processo de bootstrap;
* compilador auto-hospedado;
* artefatos reproduzíveis;
* versão 1.0 estável da Linguagem Capi.

---

## 44. Disposição final

Este documento passa a ser a referência operacional para o desenvolvimento da implementação oficial da Capi.

Os stages definidos neste plano devem orientar:

* a criação dos documentos;
* a preparação da infraestrutura;
* a abertura de tarefas;
* a implementação dos módulos;
* a criação dos testes;
* a validação das entregas;
* a evolução até a versão 1.0.

Nenhum stage deve ser considerado concluído apenas pela existência de código.

A conclusão exige coerência entre:

```text
especificação,
documentação,
infraestrutura,
implementação,
testes
e validação.
```
