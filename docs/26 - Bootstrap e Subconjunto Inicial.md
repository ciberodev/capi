# 26 — Bootstrap e Subconjunto Inicial

## Especificação de Implementação da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a especificação de implementação do processo de **Bootstrap e Subconjunto Inicial** utilizado pelo compilador oficial da linguagem Capi.

Seu objetivo é estabelecer os contratos arquiteturais responsáveis pela construção das primeiras versões do compilador, pela evolução gradual da implementação e pela transição para um compilador integralmente desenvolvido na própria linguagem Capi.

Este documento não especifica linguagens de implementação, ferramentas de desenvolvimento, cronogramas, metodologias de engenharia de software ou tecnologias utilizadas durante o processo de bootstrap. Seu foco está exclusivamente nos contratos arquiteturais que toda implementação compatível deverá respeitar.

---

## 1.2 Escopo

Este documento especifica:

* a arquitetura geral do processo de bootstrap;
* o papel do subconjunto inicial da linguagem;
* a evolução incremental da implementação;
* os contratos relacionados ao processo de auto-hospedagem (*bootstrapping*);
* os princípios arquiteturais que orientam a construção do compilador.

Não fazem parte deste documento:

* a linguagem utilizada para implementar o primeiro compilador;
* ferramentas específicas de desenvolvimento;
* cronogramas de implementação;
* organização da equipe de desenvolvimento;
* decisões relacionadas ao gerenciamento do projeto.

Esses assuntos pertencem exclusivamente à implementação.

---

## 1.3 Relação com os Documentos Anteriores

O processo de bootstrap integra todas as etapas da arquitetura de implementação definidas pelos documentos anteriores.

Seu papel consiste em estabelecer uma estratégia de evolução que permita construir progressivamente um compilador plenamente compatível com esta especificação.

Relaciona-se diretamente com:

| Documento    | Responsabilidade                                |
| ------------ | ----------------------------------------------- |
| Documento 13 | Estrutura do Compilador                         |
| Documento 14 | Lexer e Tokens                                  |
| Documento 15 | Parser e AST                                    |
| Documento 16 | Representação Semântica (HIR)                   |
| Documento 17 | Resolução de Nomes e Escopos                    |
| Documento 18 | Inferência e Verificação de Tipos               |
| Documento 19 | Verificação Semântica                           |
| Documento 20 | Lowering para Intermediate Representation       |
| Documento 21 | Intermediate Representation                     |
| Documento 22 | Otimizações sobre a Intermediate Representation |
| Documento 23 | Diagnósticos e Recuperação de Erros             |
| Documento 24 | Backend e Geração de Código                     |
| Documento 25 | Testes do Compilador                            |

Os contratos definidos por esses documentos constituem a base para a evolução do compilador até sua completa auto-hospedagem.

---

## 1.4 Filosofia da Implementação

O processo de bootstrap possui responsabilidade única: permitir que a implementação evolua gradualmente até tornar-se capaz de compilar integralmente a si própria.

Essa arquitetura separa claramente:

* a especificação da linguagem;
* a implementação do compilador;
* o processo de evolução da implementação;
* as tecnologias utilizadas durante sua construção.

A forma como essa evolução é conduzida pertence exclusivamente à implementação.

---

## 1.5 Separação entre Especificação e Implementação

Este documento estabelece apenas os contratos arquiteturais relacionados ao processo de bootstrap.

Uma implementação poderá utilizar, por exemplo:

* diferentes linguagens para implementar o primeiro compilador;
* diferentes compiladores hospedeiros;
* diferentes ferramentas de desenvolvimento;
* diferentes estratégias de migração para Capi;
* diferentes processos de evolução incremental.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 2. Papel do Bootstrap

O bootstrap constitui o processo responsável por viabilizar a construção inicial do compilador da linguagem Capi e sua evolução progressiva até atingir a auto-hospedagem.

Seu papel consiste em estabelecer uma estratégia arquitetural que permita desenvolver o compilador de forma incremental, preservando continuamente os contratos definidos por esta especificação durante toda a evolução da implementação.

---

## 2.1 Objetivo

O processo de bootstrap possui os seguintes objetivos:

* permitir a construção inicial do compilador;
* possibilitar evolução incremental da implementação;
* preservar os contratos definidos pela especificação;
* conduzir a implementação até a auto-hospedagem;
* assegurar continuidade durante a evolução do compilador.

---

## 2.2 Papel na Evolução do Compilador

O bootstrap acompanha as primeiras etapas da implementação do compilador.

Durante esse processo, novas funcionalidades poderão ser incorporadas progressivamente até que o compilador suporte integralmente a linguagem definida por esta especificação.

---

## 2.3 Responsabilidades

Compete ao processo de bootstrap:

* estabelecer uma estratégia de evolução da implementação;
* permitir o desenvolvimento incremental do compilador;
* preservar a compatibilidade entre as diferentes etapas de evolução;
* preparar a implementação para a auto-hospedagem;
* assegurar continuidade durante a transição entre versões.

---

## 2.4 Escopo do Bootstrap

O bootstrap aplica-se exclusivamente ao processo de construção e evolução do compilador.

Não constitui responsabilidade desta etapa definir funcionalidades da linguagem nem alterar os contratos estabelecidos pelos documentos anteriores.

---

## 2.5 Integração com a Arquitetura

O processo de bootstrap integra-se conceitualmente a toda a arquitetura do compilador.

Seu papel consiste em fornecer uma estratégia de evolução para a implementação, sem modificar os contratos arquiteturais estabelecidos pelas demais etapas da compilação.

---

## 2.6 Independência Arquitetural

O bootstrap permanece independente:

* da linguagem utilizada para implementar o primeiro compilador;
* das ferramentas empregadas durante o desenvolvimento;
* do compilador hospedeiro;
* da organização da implementação;
* das estratégias adotadas para evolução incremental.

Sua única responsabilidade consiste em definir os contratos arquiteturais que orientam a construção progressiva do compilador.

---

# 3. Fluxo Geral do Bootstrap

O bootstrap representa um processo evolutivo destinado à construção gradual do compilador da linguagem Capi.

Durante esse processo, diferentes versões da implementação ampliam progressivamente o suporte à linguagem até que o compilador se torne capaz de compilar integralmente seu próprio código-fonte.

Este documento estabelece apenas os contratos arquiteturais desse processo, não impondo qualquer estratégia específica para sua execução.

---

## 3.1 Visão Geral

Conceitualmente, o processo de bootstrap pode ser representado da seguinte forma:

```text
Compilador Inicial
        │
        ▼
Subconjunto Inicial da Linguagem
        │
        ▼
Evolução Incremental
        │
        ▼
Compilador Auto-hospedado
        │
        ▼
Evolução Contínua
```

Cada implementação poderá adotar diferentes estratégias para percorrer esse processo.

---

## 3.2 Compilador Inicial

O processo de bootstrap inicia-se com uma implementação capaz de compilar um subconjunto da linguagem Capi.

A tecnologia utilizada para construir essa implementação pertence exclusivamente à implementação.

---

## 3.3 Evolução Incremental

Novas funcionalidades da linguagem poderão ser incorporadas progressivamente ao compilador.

Cada etapa de evolução deverá preservar os contratos definidos pela especificação, mantendo compatibilidade com as versões anteriormente implementadas sempre que aplicável.

---

## 3.4 Auto-hospedagem

O processo de bootstrap culmina na obtenção de um compilador capaz de compilar integralmente seu próprio código-fonte.

Os critérios utilizados para caracterizar essa condição serão definidos nas seções posteriores deste documento.

---

## 3.5 Evolução Contínua

Após atingir a auto-hospedagem, o compilador continuará evoluindo normalmente.

Novas funcionalidades, otimizações e melhorias poderão ser incorporadas preservando integralmente os contratos definidos por esta especificação.

---

## 3.6 Responsabilidade Única

O processo de bootstrap possui responsabilidade única: estabelecer a estratégia arquitetural para construção e evolução do compilador até sua auto-hospedagem.

Este documento não define tecnologias, linguagens, ferramentas ou metodologias específicas de desenvolvimento, deixando essas decisões exclusivamente a cargo da implementação.

---

# 4. Subconjunto Inicial da Linguagem

O processo de bootstrap inicia-se com um subconjunto da linguagem Capi suficiente para permitir a construção das primeiras versões do compilador.

Esse subconjunto representa uma etapa temporária da evolução da implementação e não constitui uma variante da linguagem. Seu objetivo é viabilizar o desenvolvimento incremental até que todas as funcionalidades definidas pela especificação estejam disponíveis.

Este documento estabelece apenas os contratos arquiteturais relacionados a esse subconjunto.

---

## 4.1 Objetivo

O subconjunto inicial possui os seguintes objetivos:

* viabilizar a construção das primeiras versões do compilador;
* reduzir a complexidade inicial da implementação;
* permitir evolução incremental da linguagem suportada;
* preservar os contratos definidos pela especificação.

---

## 4.2 Finalidade

O subconjunto inicial constitui um recurso temporário utilizado exclusivamente durante o processo de bootstrap.

Sua existência não altera a definição oficial da linguagem nem cria diferentes versões da especificação.

Após a conclusão do bootstrap, a implementação deverá evoluir para suportar integralmente a linguagem definida pelos documentos desta especificação.

---

## 4.3 Escopo

O conjunto de funcionalidades inicialmente implementadas pertence exclusivamente à implementação.

A especificação não impõe quais recursos da linguagem deverão compor o subconjunto inicial, exigindo apenas que sua evolução preserve continuamente os contratos arquiteturais estabelecidos.

---

## 4.4 Evolução

O subconjunto inicial deverá evoluir progressivamente até incorporar todas as construções previstas pela linguagem.

Cada etapa dessa evolução deverá manter compatibilidade com os contratos já implementados e ampliar gradualmente o suporte às funcionalidades restantes.

---

## 4.5 Compatibilidade

Durante todo o processo de bootstrap, o subconjunto implementado deverá permanecer consistente com a especificação da linguagem.

Nenhuma funcionalidade temporária deverá alterar os contratos arquiteturais estabelecidos pelos documentos desta especificação.

A evolução do subconjunto inicial deverá ser continuamente validada pela infraestrutura de testes definida no Documento 25 — Testes do Compilador, garantindo que os contratos arquiteturais permaneçam preservados durante todo o processo de bootstrap.

---

## 4.6 Independência da Implementação

Os mecanismos utilizados para definir, implementar e evoluir o subconjunto inicial pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 5. Implementação Inicial

A implementação inicial corresponde à primeira versão funcional do compilador capaz de processar programas escritos no subconjunto inicial da linguagem Capi.

Sua finalidade consiste em estabelecer a base sobre a qual o processo de bootstrap será conduzido até a obtenção de um compilador completamente auto-hospedado.

Este documento estabelece apenas os contratos arquiteturais relacionados a essa implementação inicial.

---

## 5.1 Objetivo

A implementação inicial possui os seguintes objetivos:

* fornecer a primeira versão funcional do compilador;
* permitir o desenvolvimento incremental da linguagem;
* servir como base para o processo de bootstrap;
* preservar os contratos definidos pela especificação.

---

## 5.2 Linguagem de Implementação

A linguagem utilizada para desenvolver a implementação inicial pertence exclusivamente à implementação.

Este documento não estabelece qualquer restrição quanto à tecnologia empregada para construir o primeiro compilador.

---

## 5.3 Compilador Hospedeiro

Sempre que necessário, a implementação inicial poderá utilizar um compilador hospedeiro para produzir seus primeiros artefatos executáveis.

A escolha desse compilador pertence exclusivamente à implementação.

---

## 5.4 Ferramentas Utilizadas

Ferramentas auxiliares poderão ser empregadas durante o desenvolvimento da implementação inicial.

Entre elas incluem-se, quando aplicável:

* compiladores;
* montadores (*assemblers*);
* ligadores (*linkers*);
* geradores de código;
* ferramentas de automação;
* demais ferramentas consideradas necessárias pela implementação.

A utilização dessas ferramentas não altera os contratos definidos por esta especificação.

---

## 5.5 Independência Tecnológica

A implementação inicial permanece independente das tecnologias utilizadas para sua construção.

Uma vez concluído o processo de bootstrap, essas tecnologias poderão ser substituídas ou eliminadas sem modificar os contratos arquiteturais estabelecidos pela linguagem.

---

## 5.6 Independência da Implementação

Os mecanismos utilizados para construir a implementação inicial pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 6. Compatibilidade entre Versões

Durante o processo de bootstrap, diferentes versões do compilador coexistirão enquanto novas funcionalidades são incorporadas à implementação.

A evolução entre essas versões deverá preservar continuamente os contratos definidos pela especificação, permitindo que o processo de desenvolvimento avance de forma segura e previsível.

Este documento estabelece apenas os contratos arquiteturais relacionados a essa evolução.

---

## 6.1 Objetivo

A compatibilidade entre versões possui os seguintes objetivos:

* preservar a continuidade da evolução do compilador;
* reduzir riscos durante o processo de bootstrap;
* assegurar estabilidade arquitetural;
* facilitar a incorporação de novas funcionalidades.

---

## 6.2 Evolução Controlada

Cada nova versão do compilador deverá representar uma evolução da versão anterior.

Sempre que novas funcionalidades forem incorporadas, os contratos previamente implementados deverão permanecer preservados, salvo quando alterações forem explicitamente previstas pela especificação.

---

## 6.3 Compatibilidade

Sempre que aplicável, versões sucessivas do compilador deverão manter compatibilidade funcional com os programas suportados pela etapa anterior do processo de bootstrap.

A estratégia utilizada para garantir essa compatibilidade pertence exclusivamente à implementação.

---

## 6.4 Estabilidade

A evolução da implementação deverá buscar preservar a estabilidade do compilador durante todo o processo de bootstrap.

Sempre que possível, alterações estruturais deverão ser introduzidas de forma incremental, reduzindo riscos para a continuidade do desenvolvimento.

---

## 6.5 Continuidade

Cada etapa do bootstrap deverá fornecer uma base adequada para a etapa seguinte.

Essa continuidade assegura que o processo de evolução possa prosseguir até atingir a implementação completa da linguagem, sem necessidade de reconstruções incompatíveis com os contratos definidos nesta especificação.

---

## 6.6 Independência da Implementação

Os mecanismos utilizados para controlar a evolução entre diferentes versões do compilador pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 7. Auto-hospedagem

A auto-hospedagem representa o estágio em que o compilador da linguagem Capi torna-se capaz de compilar integralmente seu próprio código-fonte.

Essa condição constitui um marco arquitetural importante na evolução da implementação, demonstrando que a linguagem possui recursos suficientes para sustentar seu próprio desenvolvimento.

Este documento estabelece apenas os contratos arquiteturais relacionados ao processo de auto-hospedagem.

---

## 7.1 Objetivo

A auto-hospedagem possui os seguintes objetivos:

* permitir que o compilador seja desenvolvido na própria linguagem Capi;
* reduzir progressivamente a dependência da implementação inicial;
* consolidar a maturidade da implementação;
* estabelecer uma base estável para a evolução futura do compilador.

---

## 7.2 Definição

Considera-se auto-hospedado o compilador capaz de compilar integralmente seu próprio código-fonte utilizando uma implementação anterior funcionalmente equivalente.

Essa condição caracteriza a conclusão do processo de bootstrap, preservando integralmente os contratos definidos por esta especificação.

---

## 7.3 Transição

A transição para a auto-hospedagem poderá ocorrer de forma incremental.

Durante esse processo, diferentes componentes do compilador poderão ser progressivamente reimplementados em Capi, até que toda a implementação seja escrita na própria linguagem.

A estratégia utilizada para conduzir essa transição pertence exclusivamente à implementação.

---

## 7.4 Compilação da Própria Linguagem

Uma vez atingida a auto-hospedagem, o compilador deverá ser capaz de produzir uma nova versão funcionalmente equivalente de si próprio.

Esse processo deverá preservar integralmente os contratos arquiteturais definidos por esta especificação e manter a conformidade com toda a suíte de testes definida no **Documento 25 — Testes do Compilador**.

---

## 7.5 Critério para Bootstrap Completo

Considera-se concluído o processo de bootstrap quando, simultaneamente:

* o compilador estiver implementado integralmente em Capi;
* for capaz de compilar seu próprio código-fonte;
* o compilador gerado apresentar comportamento funcionalmente equivalente ao compilador utilizado para produzi-lo;
* os contratos definidos por esta especificação permanecerem integralmente preservados;
* a suíte de testes do compilador permanecer integralmente aprovada.

A suíte de testes referida nesta seção corresponde à infraestrutura definida no Documento 25 — Testes do Compilador, responsável pela validação contínua da conformidade da implementação com os contratos estabelecidos por esta especificação.

Esses critérios estabelecem o marco arquitetural que caracteriza a conclusão do processo de bootstrap.

---

## 7.6 Independência da Implementação

Os mecanismos utilizados para atingir a auto-hospedagem pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 8. Evolução Incremental

A evolução incremental constitui o princípio arquitetural que orienta o desenvolvimento contínuo do compilador durante e após o processo de bootstrap.

Seu objetivo consiste em permitir que novas funcionalidades sejam incorporadas progressivamente, preservando continuamente os contratos definidos pela especificação da linguagem.

Este documento estabelece apenas os contratos arquiteturais relacionados a essa evolução.

---

## 8.1 Objetivo

A evolução incremental possui os seguintes objetivos:

* reduzir a complexidade da implementação;
* facilitar a validação contínua do compilador;
* preservar a estabilidade da implementação;
* permitir crescimento controlado da linguagem suportada.

---

## 8.2 Desenvolvimento por Etapas

O desenvolvimento do compilador poderá ser organizado em etapas sucessivas.

Cada etapa amplia as capacidades da implementação, preservando integralmente os contratos já estabelecidos pelas etapas anteriores.

---

## 8.3 Crescimento Funcional

Novas funcionalidades poderão ser incorporadas gradualmente ao compilador.

A ordem em que essas funcionalidades são implementadas pertence exclusivamente à implementação, desde que os contratos arquiteturais permaneçam preservados.

---

## 8.4 Validação Contínua

Cada etapa da evolução deverá ser continuamente validada pela infraestrutura de testes definida no **Documento 25 — Testes do Compilador**.

Essa validação permite identificar regressões, preservar os contratos arquiteturais e assegurar a continuidade segura da implementação.

---

## 8.5 Preservação dos Contratos

Independentemente da ordem adotada para implementação das funcionalidades, cada nova etapa deverá preservar integralmente os contratos definidos pelos documentos desta especificação.

A evolução da implementação não deverá introduzir incompatibilidades que violem esses contratos.

---

## 8.6 Independência da Implementação

Os mecanismos utilizados para conduzir a evolução incremental pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 9. Compatibilidade Arquitetural

A compatibilidade arquitetural assegura que todas as etapas do processo de bootstrap permaneçam alinhadas aos contratos estabelecidos pela especificação da linguagem.

Seu objetivo consiste em permitir a evolução contínua da implementação sem comprometer a arquitetura definida pelos documentos desta série.

Este documento estabelece apenas os contratos arquiteturais relacionados a essa compatibilidade.

---

## 9.1 Objetivo

A compatibilidade arquitetural possui os seguintes objetivos:

* preservar a coerência entre as diferentes versões do compilador;
* assegurar continuidade durante a evolução da implementação;
* reduzir riscos de incompatibilidades arquiteturais;
* manter alinhamento com a especificação da linguagem.

---

## 9.2 Preservação dos Contratos

Toda evolução da implementação deverá preservar os contratos definidos pelos documentos desta especificação.

Nenhuma etapa do processo de bootstrap poderá modificar esses contratos sem que a própria especificação seja atualizada de forma correspondente.

---

## 9.3 Evolução da Implementação

A implementação poderá evoluir continuamente após a conclusão do bootstrap.

Novas versões poderão incorporar melhorias, otimizações, correções e novas funcionalidades, desde que permaneçam compatíveis com os contratos arquiteturais estabelecidos.

---

## 9.4 Compatibilidade entre Compiladores

Durante o processo de bootstrap poderão coexistir diferentes versões do compilador.

Sempre que aplicável, essas versões deverão permanecer funcionalmente compatíveis entre si, permitindo uma transição segura durante a evolução da implementação.

A estratégia utilizada para garantir essa compatibilidade pertence exclusivamente à implementação.

---

## 9.5 Continuidade da Especificação

A evolução do compilador deverá ocorrer em conformidade com a evolução da especificação da linguagem.

Sempre que novos contratos arquiteturais forem incorporados aos documentos desta série, a implementação deverá evoluir de forma correspondente.

---

## 9.6 Independência da Implementação

Os mecanismos utilizados para preservar a compatibilidade arquitetural pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 10. Interface do Processo de Bootstrap

O processo de bootstrap estabelece a transição entre a implementação inicial do compilador e sua evolução até a auto-hospedagem.

Sua interface define os contratos arquiteturais que permitem integrar o processo de construção do compilador com as demais etapas da arquitetura da linguagem, preservando continuamente os contratos estabelecidos pela especificação.

Este documento define apenas esses contratos, não impondo qualquer estratégia específica para sua implementação.

---

## 10.1 Papel

A interface do processo de bootstrap possui os seguintes objetivos:

* integrar as diferentes etapas da evolução do compilador;
* preservar os contratos definidos pela especificação;
* permitir diferentes estratégias de bootstrap;
* fornecer uma arquitetura uniforme para a evolução da implementação.

---

## 10.2 Relação com o Pipeline

O processo de bootstrap abrange todas as etapas da implementação do compilador.

Durante sua evolução, diferentes versões do compilador deverão preservar os contratos definidos pelos documentos da arquitetura, desde a análise léxica até a geração de código, utilizando continuamente a infraestrutura de testes definida no **Documento 25 — Testes do Compilador** para validar sua conformidade.

---

## 10.3 Entradas

O processo de bootstrap poderá utilizar como entrada:

* a implementação existente do compilador;
* o código-fonte desenvolvido em Capi;
* ferramentas auxiliares de construção;
* compiladores hospedeiros;
* demais recursos necessários para a evolução da implementação.

A forma como essas entradas são organizadas pertence exclusivamente à implementação.

---

## 10.4 Saídas

Cada etapa do bootstrap deverá produzir uma nova versão funcional do compilador.

Essa versão deverá servir como base para a etapa seguinte da evolução, preservando integralmente os contratos definidos por esta especificação.

---

## 10.5 Continuidade

O processo de bootstrap deverá estabelecer uma sequência contínua de evolução entre suas diferentes etapas.

Cada versão produzida deverá permitir que o desenvolvimento prossiga sem necessidade de reconstruções incompatíveis com a arquitetura estabelecida.

---

## 10.6 Independência da Implementação

Os mecanismos utilizados para integrar as diferentes etapas do processo de bootstrap pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 11. Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência do processo de bootstrap utilizado pelo compilador oficial da linguagem Capi.

Seu papel consiste em orientar a construção progressiva do compilador até sua completa auto-hospedagem, preservando continuamente os contratos estabelecidos pela especificação da linguagem.

---

## 11.1 Papel na Arquitetura

O processo de bootstrap constitui um componente transversal da arquitetura de implementação.

Ele não representa uma etapa do pipeline de compilação, mas um processo evolutivo que acompanha a construção do compilador desde sua implementação inicial até sua consolidação como compilador auto-hospedado.

---

## 11.2 Relação com o Compilador

A arquitetura de bootstrap fornece a estratégia de evolução da implementação do compilador.

Cada nova versão produzida deverá ampliar gradualmente suas capacidades, mantendo compatibilidade com os contratos definidos por esta especificação.

---

## 11.3 Relação com o Processo de Evolução

A arquitetura de referência organiza o bootstrap como um processo contínuo de evolução incremental.

Cada etapa fornece a base para a etapa seguinte, permitindo que o compilador evolua de forma previsível, estável e compatível com a especificação.

---

## 11.4 Independência entre Implementações

Implementações distintas poderão adotar diferentes estratégias para conduzir o bootstrap.

Entre elas incluem-se, quando aplicável:

* diferentes linguagens para implementação inicial;
* diferentes compiladores hospedeiros;
* diferentes estratégias de migração para Capi;
* diferentes formas de organização do processo de desenvolvimento;
* diferentes mecanismos de validação contínua.

Independentemente da estratégia adotada, todas deverão preservar integralmente os contratos definidos nesta especificação.

---

## 11.5 Evolução Futura

A arquitetura definida neste documento foi concebida para permitir evolução contínua do compilador após a conclusão do bootstrap.

Novas funcionalidades da linguagem, novas arquiteturas de compilação e novas estratégias de implementação poderão ser incorporadas sem alterar os contratos arquiteturais estabelecidos por esta especificação.

---

## 11.6 Síntese Arquitetural

A arquitetura de bootstrap estabelece um conjunto uniforme de contratos destinados a orientar a construção progressiva do compilador.

Sua arquitetura preserva a separação entre especificação e implementação, permitindo que diferentes estratégias de desenvolvimento conduzam à mesma arquitetura final da linguagem Capi.

---

# 12. Critérios para Bootstrap Completo

A conclusão do processo de bootstrap representa o momento em que o compilador atinge maturidade suficiente para sustentar sua própria evolução.

Essa condição não depende da tecnologia utilizada para construir a implementação inicial, mas da preservação dos contratos definidos por esta especificação e da capacidade do compilador de evoluir utilizando exclusivamente a própria linguagem Capi.

---

## 12.1 Objetivo

Os critérios para bootstrap completo possuem os seguintes objetivos:

* estabelecer um marco arquitetural para a conclusão do bootstrap;
* definir quando a implementação pode ser considerada auto-hospedada;
* assegurar a preservação dos contratos da especificação;
* orientar a evolução futura do compilador.

---

## 12.2 Compilador Auto-hospedado

Considera-se plenamente auto-hospedado o compilador que:

* esteja implementado integralmente em Capi;
* seja capaz de compilar integralmente seu próprio código-fonte;
* produza novas versões funcionalmente equivalentes de si próprio;
* preserve integralmente os contratos definidos por esta especificação;
* permaneça aprovado em toda a infraestrutura de testes definida no **Documento 25 — Testes do Compilador**.

A infraestrutura de testes referida nesta seção corresponde à definida no Documento 25 — Testes do Compilador, responsável por verificar continuamente a conformidade da implementação com os contratos estabelecidos por esta especificação

---

## 12.3 Preservação da Especificação

A conclusão do bootstrap não altera a especificação da linguagem.

Toda evolução posterior do compilador deverá continuar preservando integralmente os contratos definidos pelos documentos desta série.

---

## 12.4 Independência do Compilador Inicial

Após a conclusão do bootstrap, a implementação não dependerá mais do compilador utilizado durante sua construção inicial.

A permanência ou remoção dessa implementação inicial pertence exclusivamente à estratégia adotada pela implementação.

---

## 12.5 Evolução Posterior

A conclusão do bootstrap marca apenas o encerramento da etapa inicial de construção do compilador.

A partir desse momento, o compilador continuará evoluindo normalmente, incorporando novas funcionalidades, otimizações e melhorias em conformidade com esta especificação.

---

## 12.6 Independência da Implementação

Os mecanismos utilizados para atingir os critérios estabelecidos nesta seção pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 13. Limites do Bootstrap

O processo de bootstrap possui responsabilidade exclusiva sobre a construção inicial do compilador e sua evolução até a auto-hospedagem.

Ele não altera a especificação da linguagem, não modifica os contratos definidos pelos documentos anteriores e não impõe tecnologias específicas para implementação do compilador.

Essa separação reduz o acoplamento entre a arquitetura da linguagem e o processo utilizado para construir sua implementação.

---

## 13.1 Objetivo

A definição dos limites do bootstrap possui os seguintes objetivos:

* preservar sua responsabilidade única;
* evitar sobreposição com a especificação da linguagem;
* garantir previsibilidade arquitetural;
* permitir diferentes estratégias de implementação.

---

## 13.2 O que Pertence ao Bootstrap

Compete exclusivamente ao processo de bootstrap:

* definir a estratégia de evolução da implementação;
* estabelecer o subconjunto inicial da linguagem;
* orientar a transição para a auto-hospedagem;
* preservar a continuidade entre as diferentes versões do compilador;
* estabelecer os critérios arquiteturais para conclusão do bootstrap.

---

## 13.3 O que Não Pertence ao Bootstrap

Não compete ao processo de bootstrap:

* definir a sintaxe da linguagem;
* estabelecer regras semânticas;
* modificar a arquitetura do compilador;
* impor linguagens de implementação;
* definir ferramentas específicas de desenvolvimento;
* estabelecer metodologias de gerenciamento do projeto.

Essas responsabilidades pertencem aos documentos específicos da especificação ou exclusivamente à implementação.

---

## 13.4 Relação com a Implementação

O bootstrap constitui uma estratégia arquitetural para evolução do compilador.

As decisões relacionadas às tecnologias utilizadas, organização do código-fonte, ferramentas empregadas e cronograma de desenvolvimento pertencem exclusivamente à implementação.

---

## 13.5 Relação com Ferramentas

Compiladores hospedeiros, ferramentas de automação, ambientes de desenvolvimento e demais recursos utilizados durante o bootstrap pertencem exclusivamente à implementação.

Este documento estabelece apenas os contratos arquiteturais do processo de bootstrap, permanecendo independente das tecnologias adotadas.

---

## 13.6 Independência da Implementação

Os mecanismos utilizados para implementar o processo de bootstrap pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 14. Princípios Arquiteturais

O processo de bootstrap constitui o mecanismo responsável por permitir o nascimento e a evolução inicial do compilador da linguagem Capi.

Sua arquitetura baseia-se em princípios que asseguram crescimento incremental, continuidade da implementação, preservação da especificação e independência tecnológica.

Esses princípios orientam todas as implementações compatíveis com esta especificação.

---

## 14.1 Evolução Incremental

A implementação deverá evoluir por etapas sucessivas, incorporando progressivamente novas funcionalidades da linguagem.

Cada etapa deverá servir como base para a etapa seguinte, preservando continuamente os contratos definidos pela especificação.

---

## 14.2 Compatibilidade

A evolução do compilador deverá preservar compatibilidade arquitetural entre versões sucessivas durante o processo de bootstrap.

Sempre que possível, versões intermediárias deverão permanecer capazes de sustentar a continuidade da evolução da implementação.

---

## 14.3 Auto-hospedagem

A auto-hospedagem representa o objetivo arquitetural do processo de bootstrap.

Sua obtenção demonstra que a linguagem possui maturidade suficiente para sustentar a evolução de seu próprio compilador.

---

## 14.4 Continuidade

O bootstrap deverá constituir um processo contínuo de evolução da implementação.

Cada nova versão deverá ampliar as capacidades do compilador sem comprometer os contratos estabelecidos pelas versões anteriores.

---

## 14.5 Separação de Responsabilidades

A arquitetura estabelece responsabilidades claramente definidas entre:

* a especificação da linguagem;
* a implementação do compilador;
* o processo de bootstrap;
* as ferramentas utilizadas durante o desenvolvimento.

Essa separação reduz o acoplamento entre esses componentes e facilita sua evolução independente.

---

## 14.6 Independência da Implementação

Os mecanismos utilizados para conduzir o processo de bootstrap pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os princípios arquiteturais definidos nesta especificação.

---

# 15. Considerações sobre a Arquitetura

O processo de bootstrap representa a etapa responsável por transformar a especificação da linguagem em uma implementação plenamente funcional.

Sua arquitetura fornece uma estratégia de evolução capaz de conduzir o compilador desde suas primeiras versões até a auto-hospedagem, preservando continuamente os contratos definidos pela especificação.

Este documento estabelece apenas os princípios arquiteturais desse processo, permanecendo independente das tecnologias utilizadas para sua implementação.

---

## 15.1 Papel na Evolução da Linguagem

O bootstrap desempenha papel fundamental na consolidação da linguagem Capi.

Embora constitua um processo temporário, ele estabelece a base sobre a qual toda a evolução futura do compilador será construída.

---

## 15.2 Relação com Toda a Arquitetura

O processo de bootstrap integra conceitualmente todos os componentes definidos pelos documentos desta série.

Sua função consiste em conduzir a implementação de forma que cada etapa da arquitetura seja incorporada progressivamente, preservando continuamente seus respectivos contratos.

---

## 15.3 Relação com o Processo de Desenvolvimento

O bootstrap fornece uma estratégia arquitetural para o desenvolvimento inicial do compilador.

Entretanto, os métodos de engenharia de software, a organização da equipe, as ferramentas utilizadas e o planejamento da implementação permanecem fora do escopo desta especificação.

---

## 15.4 Evolução Futura

Após a conclusão do bootstrap, o compilador continuará evoluindo normalmente.

Novas funcionalidades, melhorias de desempenho, novas arquiteturas suportadas e aperfeiçoamentos internos poderão ser incorporados sem modificar os contratos estabelecidos por esta especificação.

---

## 15.5 Escalabilidade

A arquitetura do bootstrap foi concebida para permitir crescimento progressivo da implementação.

Novos recursos poderão ser incorporados ao compilador preservando a continuidade da evolução e reduzindo a necessidade de reestruturações incompatíveis com a arquitetura definida.

---

## 15.6 Síntese Arquitetural

O processo de bootstrap estabelece uma arquitetura de evolução destinada a transformar gradualmente a especificação da linguagem em um compilador plenamente funcional e auto-hospedado.

Sua arquitetura preserva a separação entre especificação e implementação, permitindo que diferentes estratégias de desenvolvimento conduzam ao mesmo resultado arquitetural definido para a linguagem Capi.

---

# 16. Considerações Finais

O processo de bootstrap constitui o mecanismo responsável por transformar a especificação da linguagem Capi em uma implementação plenamente funcional e capaz de evoluir de forma autônoma.

Este documento estabelece os contratos arquiteturais que orientam essa evolução, desde a construção das primeiras versões do compilador até sua completa auto-hospedagem, preservando continuamente os princípios definidos por toda a série de especificações de implementação.

---

## 16.1 Síntese da Arquitetura

A arquitetura do processo de bootstrap caracteriza-se pelos seguintes princípios:

* evolução incremental da implementação;
* utilização de um subconjunto inicial da linguagem;
* preservação contínua dos contratos da especificação;
* transição controlada para a auto-hospedagem;
* independência entre especificação e implementação.

Esses princípios asseguram que diferentes estratégias de desenvolvimento possam conduzir à mesma arquitetura final do compilador da linguagem Capi.

---

## 16.2 Princípios Preservados

Toda implementação compatível com esta especificação deverá preservar:

* a continuidade da evolução do compilador;
* a compatibilidade arquitetural entre versões sucessivas;
* a preservação integral dos contratos definidos pela especificação;
* a separação entre especificação e implementação;
* a independência das tecnologias utilizadas durante o bootstrap.

Esses princípios garantem que o processo de bootstrap permaneça compatível com toda a arquitetura estabelecida para a linguagem Capi.

---

## 16.3 Bootstrap como Processo Contínuo

A conclusão do bootstrap não representa o encerramento da evolução do compilador.

Ela estabelece apenas o momento em que a implementação passa a ser capaz de sustentar seu próprio desenvolvimento utilizando exclusivamente a linguagem Capi.

A partir desse ponto, o compilador continuará evoluindo conforme as necessidades da linguagem, preservando continuamente os contratos arquiteturais definidos por esta especificação.

---

## 16.4 Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência do processo de bootstrap utilizada pelo compilador oficial da linguagem Capi.

Outras implementações poderão adotar diferentes linguagens para implementação inicial, diferentes compiladores hospedeiros, diferentes estratégias de migração, diferentes formas de evolução incremental e diferentes processos de auto-hospedagem, desde que preservem integralmente:

* os contratos definidos nesta especificação;
* a continuidade da evolução do compilador;
* a preservação da arquitetura da linguagem;
* a conformidade com a infraestrutura de testes;
* a separação entre especificação e implementação.

Dessa forma, este documento estabelece a especificação arquitetural do **Bootstrap e Subconjunto Inicial**, concluindo a série de documentos dedicada à implementação do compilador da linguagem Capi e fornecendo uma base comum para a construção, evolução e manutenção de todas as implementações compatíveis.
