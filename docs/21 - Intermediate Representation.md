# 21 — Intermediate Representation (IR)

## Especificação de Implementação da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a especificação de implementação da **Intermediate Representation (IR)** utilizada pelo compilador oficial da linguagem Capi.

Seu objetivo é estabelecer os contratos arquiteturais da representação intermediária produzida pelo processo de Lowering e utilizada pelas fases do Middle-end do compilador.

A Intermediate Representation constitui a representação oficial do programa durante as etapas de otimização e preparação para a geração de código.

---

## 1.2 Escopo

Este documento especifica:

* o papel da Intermediate Representation na arquitetura do compilador;
* os contratos da representação intermediária;
* as propriedades que toda IR deve preservar;
* a relação entre a IR e as demais etapas do pipeline;
* os princípios arquiteturais que orientam sua utilização.

Não fazem parte deste documento:

* o processo de Lowering;
* as otimizações realizadas sobre a IR;
* a geração de código;
* a estrutura física utilizada para implementar a representação.

Esses assuntos são tratados em seus respectivos documentos da especificação de implementação.

---

## 1.3 Relação com os Documentos Anteriores

A Intermediate Representation é produzida pelo processo de Lowering definido no Documento 20.

Sua construção depende das garantias estabelecidas pelos documentos anteriores, especialmente:

| Documento    | Responsabilidade                          |
| ------------ | ----------------------------------------- |
| Documento 16 | Representação Semântica (HIR)             |
| Documento 17 | Resolução de Nomes e Escopos              |
| Documento 18 | Inferência e Verificação de Tipos         |
| Documento 19 | Verificação Semântica                     |
| Documento 20 | Lowering para Intermediate Representation |

Ao chegar a esta etapa, toda a análise da linguagem já foi concluída.

A Intermediate Representation passa a constituir a representação oficial utilizada pelas etapas posteriores do compilador.

---

## 1.4 Filosofia da Implementação

A Intermediate Representation possui responsabilidade única: representar o programa de forma independente das construções específicas da linguagem Capi, preservando integralmente sua semântica.

Essa representação constitui a base sobre a qual atuam as fases do Middle-end, permitindo transformações, otimizações e preparação para a geração de código sem depender diretamente da HIR.

---

## 1.5 Separação entre Especificação e Implementação

Este documento estabelece os contratos funcionais da Intermediate Representation, mas não impõe qualquer estrutura concreta para sua implementação.

Uma implementação poderá utilizar, por exemplo:

* grafos;
* árvores;
* listas de instruções;
* blocos básicos;
* representações híbridas;
* outras estruturas equivalentes.

Independentemente da estrutura adotada, todas as implementações deverão preservar os contratos arquiteturais definidos nesta especificação e produzir uma representação funcionalmente equivalente para um mesmo programa.

---

# 2. Papel da Intermediate Representation

A Intermediate Representation constitui a representação oficial utilizada pelo Middle-end do compilador.

Ela estabelece uma separação entre os conceitos da linguagem Capi e as transformações internas realizadas durante a compilação, permitindo que as etapas posteriores operem sobre uma representação uniforme e independente da sintaxe da linguagem.

---

## 2.1 Objetivo

A Intermediate Representation possui os seguintes objetivos:

* representar integralmente o programa;
* preservar sua semântica;
* servir como base para as otimizações;
* servir como base para a geração de código;
* desacoplar o Middle-end das construções específicas da linguagem.

---

## 2.2 Papel no Middle-end

Durante o Middle-end, todas as transformações do programa são realizadas sobre a Intermediate Representation.

As fases posteriores não dependem diretamente da HIR nem das estruturas produzidas pelo Frontend.

Essa separação reduz o acoplamento entre os componentes do compilador e facilita sua evolução.

---

## 2.3 Relação com o Frontend

A Intermediate Representation é produzida exclusivamente pelo processo de Lowering.

Ela recebe todas as garantias produzidas pelo Frontend sem realizar novas análises relacionadas à linguagem.

Toda informação semântica necessária já deverá estar incorporada à representação.

---

## 2.4 Relação com o Backend

Após a conclusão das etapas do Middle-end, a Intermediate Representation servirá como entrada para os processos responsáveis pela geração de código.

O Backend deverá depender exclusivamente da IR, sem necessidade de interpretar diretamente os conceitos específicos da linguagem Capi.

---

## 2.5 Responsabilidades

Compete à Intermediate Representation:

* representar o programa de forma intermediária;
* preservar integralmente sua semântica;
* fornecer suporte às transformações do Middle-end;
* servir como base para as otimizações;
* servir como entrada para o Backend.

---

## 2.6 Independência Arquitetural

A Intermediate Representation deve permanecer independente:

* da sintaxe da linguagem;
* da arquitetura de hardware;
* da ABI da plataforma;
* da geração de código;
* das estratégias específicas de otimização.

Essas responsabilidades pertencem às demais etapas do compilador.

---

# 3. Fluxo da Intermediate Representation

A Intermediate Representation é produzida imediatamente após a conclusão do processo de Lowering.

Durante o Middle-end, ela constitui a única representação utilizada pelas fases de transformação e otimização do compilador.

Ao final dessas etapas, a IR é utilizada como entrada pelo processo de geração de código.

---

## 3.1 Visão Geral

Conceitualmente, o fluxo da Intermediate Representation pode ser representado da seguinte forma:

```text
           HIR Validada
                 │
                 ▼
            Lowering
                 │
                 ▼
     Intermediate Representation
                 │
                 ▼
           Otimizações
                 │
                 ▼
      IR Transformada
                 │
                 ▼
      Geração de Código
```

A Intermediate Representation permanece conceitualmente a mesma durante todo o Middle-end, ainda que possa sofrer transformações internas preservando integralmente a semântica do programa.

---

## 3.2 Produção da IR

A Intermediate Representation é produzida exclusivamente pelo processo de Lowering definido no Documento 20 — Lowering para Intermediate Representation.

Toda IR válida deverá possuir correspondência semântica com a HIR que lhe deu origem.

---

## 3.3 Consumo da IR

Após sua construção, a Intermediate Representation passa a ser utilizada por todas as etapas do Middle-end.

Essas etapas poderão transformar sua estrutura interna, desde que preservem os contratos definidos para a representação e mantenham inalterado o comportamento observável do programa.

---

## 3.4 Evolução da IR

Durante sua utilização, a Intermediate Representation poderá evoluir progressivamente em decorrência das transformações realizadas pelas diferentes fases do compilador.

Essas transformações poderão modificar sua organização interna, mas deverão preservar integralmente sua consistência estrutural e equivalência semântica.

---

## 3.5 Encerramento da Representação

A utilização da Intermediate Representation encerra-se quando o Backend inicia o processo de geração de código.

A partir desse ponto, a IR deixa de representar um artefato de transformação e passa a servir como base para a produção da representação final do programa.

---

## 3.6 Responsabilidade Única

A Intermediate Representation possui responsabilidade única: representar o programa de forma intermediária durante todo o Middle-end, preservando integralmente sua semântica e fornecendo uma base estável para as transformações realizadas pelo compilador.

Ela não define como essas transformações devem ser implementadas nem estabelece algoritmos específicos para sua utilização.

---

# 4. Objetivos da Intermediate Representation

A Intermediate Representation constitui a representação central do Middle-end do compilador.

Sua finalidade é fornecer uma forma uniforme de representar programas, preservando integralmente sua semântica e desacoplando as etapas posteriores das construções específicas da linguagem Capi.

---

## 4.1 Objetivo

A Intermediate Representation possui os seguintes objetivos:

* representar integralmente o programa;
* preservar sua semântica;
* reduzir dependências das construções específicas da linguagem;
* fornecer uma base uniforme para as transformações do compilador;
* servir como representação comum para otimizações e geração de código.

---

## 4.2 Representação Canônica

Sempre que possível, construções semanticamente equivalentes deverão possuir representações equivalentes na Intermediate Representation.

Essa uniformização reduz a complexidade das etapas posteriores do compilador e facilita a implementação de transformações e otimizações.

Os mecanismos utilizados para estabelecer essa forma canônica pertencem exclusivamente à implementação.

---

## 4.3 Independência da Linguagem

Embora seja construída a partir da linguagem Capi, a Intermediate Representation não deve depender diretamente de suas construções sintáticas.

Seu objetivo é representar o comportamento do programa de forma suficientemente abstrata para permitir sua manipulação pelas etapas do Middle-end.

---

## 4.4 Base para Otimizações

A Intermediate Representation constitui a representação utilizada pelas fases de otimização do compilador.

Sua organização deverá permitir que essas transformações sejam realizadas preservando integralmente o comportamento observável do programa.

---

## 4.5 Base para Geração de Código

Após concluídas as otimizações, a Intermediate Representation servirá como entrada para o Backend.

A representação deverá fornecer todas as informações necessárias para a geração do código final sem depender novamente da HIR ou de outras representações do Frontend.

---

## 4.6 Independência da Implementação

Este documento estabelece apenas os contratos da Intermediate Representation.

A estrutura concreta utilizada para representar seus elementos pertence exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar os contratos definidos nesta especificação.

---

# 5. Modelo Conceitual da Intermediate Representation

A Intermediate Representation organiza o programa como um conjunto de elementos relacionados entre si, formando uma representação consistente e adequada às transformações realizadas pelo compilador.

Este documento estabelece apenas seu modelo conceitual.

A Intermediate Representation é construída a partir da HIR pelo processo de Lowering definido no Documento 20 — Lowering para Intermediate Representation. Os elementos que compõem a IR correspondem semanticamente às construções da HIR, preservando integralmente o comportamento do programa.

A estrutura concreta utilizada para implementá-lo pertence exclusivamente à implementação.

---

## 5.1 Unidade de Representação

A Intermediate Representation é composta por unidades que representam os diferentes elementos do programa.

Essas unidades correspondem às construções produzidas durante o processo de Lowering e constituem os componentes fundamentais utilizados pelas etapas posteriores do compilador.

---

## 5.2 Elementos da Representação

Entre os elementos que podem compor a Intermediate Representation incluem-se, quando aplicável:

* unidades de código;
* operações;
* valores;
* referências;
* estruturas de controle;
* demais elementos previstos pela especificação da IR.

Este documento não estabelece a forma concreta desses elementos, apenas os contratos que devem ser preservados.

---

## 5.3 Relações entre Elementos

Os elementos da Intermediate Representation poderão manter relações estruturais entre si, representando dependências, fluxo de execução ou outras informações necessárias ao processamento do programa.

A forma como essas relações são representadas pertence exclusivamente à implementação.

---

## 5.4 Organização Hierárquica

A Intermediate Representation poderá organizar seus elementos em diferentes níveis de abstração.

Essa organização deverá facilitar as transformações realizadas pelo Middle-end, preservando integralmente a consistência da representação.

Os mecanismos utilizados para definir essa organização pertencem exclusivamente à implementação.

---

## 5.5 Identidade dos Elementos

Cada elemento da Intermediate Representation deverá possuir identidade suficiente para permitir sua manipulação pelas fases posteriores do compilador.

A forma como essa identidade é representada ou mantida pertence exclusivamente à implementação.

---

## 5.6 Independência da Implementação

A estrutura física da Intermediate Representation não faz parte desta especificação.

Implementações distintas poderão utilizar diferentes organizações internas, desde que preservem os contratos definidos para a representação e produzam comportamento funcionalmente equivalente.

---

# 6. Construção da Intermediate Representation

A Intermediate Representation é produzida exclusivamente pelo processo de Lowering.

Sua construção transforma a HIR semanticamente validada em uma representação apropriada ao Middle-end, preservando integralmente o comportamento do programa.

Este documento define apenas os contratos relacionados a essa representação, não o processo utilizado para construí-la.

---

## 6.1 Origem da Representação

Toda Intermediate Representation válida deverá ser originada a partir de uma HIR semanticamente consistente produzida pelo Frontend.

Nenhuma representação poderá ser considerada válida se não preservar as garantias estabelecidas pelas etapas anteriores da compilação.

A construção da Intermediate Representation não modifica a HIR. A HIR permanece disponível como representação semântica produzida pelo Frontend, enquanto a IR constitui uma nova representação utilizada pelo Middle-end.

---

## 6.2 Correspondência com a HIR

Cada elemento presente na Intermediate Representation deverá possuir correspondência semântica com a HIR que lhe deu origem.

Essa correspondência permite preservar o comportamento do programa independentemente da forma utilizada para representá-lo.

---

## 6.3 Equivalência Semântica

A Intermediate Representation deverá ser semanticamente equivalente à HIR produzida pelo Frontend.

Diferenças estruturais são permitidas, desde que não alterem o comportamento observável do programa.

---

## 6.4 Rastreabilidade

Sempre que necessário, a Intermediate Representation deverá preservar informações suficientes para manter a rastreabilidade com a HIR e, consequentemente, com o código-fonte original.

Os mecanismos utilizados para manter essa rastreabilidade pertencem exclusivamente à implementação.

---

## 6.5 Estado Inicial

Ao término do processo de Lowering, a Intermediate Representation deverá apresentar um estado estruturalmente consistente e apto a servir como entrada para as fases do Middle-end.

Esse estado constitui o ponto de partida para todas as transformações posteriores.

---

## 6.6 Independência da Implementação

Os mecanismos utilizados para construir a Intermediate Representation pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá produzir uma representação compatível com os contratos definidos nesta especificação e semanticamente equivalente à HIR recebida.

---

# 7. Consistência da Intermediate Representation

A Intermediate Representation constitui a representação oficial utilizada durante o Middle-end do compilador.

Ao longo de toda sua existência, a IR deverá permanecer estruturalmente consistente e semanticamente equivalente ao programa representado.

Essa consistência estabelece o contrato fundamental entre as diferentes fases do Middle-end.

---

## 7.1 Objetivo

A consistência da Intermediate Representation possui os seguintes objetivos:

* preservar a integridade da representação;
* manter a equivalência semântica com o programa original;
* permitir transformações sucessivas sobre a IR;
* assegurar previsibilidade durante todo o Middle-end.

---

## 7.2 Integridade Estrutural

Toda Intermediate Representation deverá manter uma estrutura internamente consistente.

As relações existentes entre seus elementos deverão permanecer válidas durante todas as transformações realizadas pelo compilador.

Os mecanismos utilizados para garantir essa propriedade pertencem exclusivamente à implementação.

---

## 7.3 Integridade Semântica

As transformações realizadas sobre a Intermediate Representation não poderão alterar o comportamento observável do programa.

Independentemente das modificações estruturais realizadas pelas etapas posteriores do compilador, a equivalência semântica deverá ser preservada.

---

## 7.4 Contratos da Representação

Toda Intermediate Representation válida deverá satisfazer os contratos estabelecidos por esta especificação.

Esses contratos constituem a base sobre a qual atuam as fases posteriores do Middle-end e garantem interoperabilidade entre diferentes implementações do compilador.

---

## 7.5 Estado Consistente

Ao término de cada etapa do Middle-end, a Intermediate Representation deverá permanecer em estado consistente.

Cada fase assume que a representação recebida satisfaz integralmente os contratos definidos para a IR e deverá entregar uma nova representação igualmente consistente para a etapa seguinte.

---

## 7.6 Independência da Implementação

Os mecanismos utilizados para preservar a consistência da Intermediate Representation pertencem exclusivamente à implementação.

A implementação poderá utilizar validações internas ou outras estratégias equivalentes para assegurar essa propriedade, desde que preserve integralmente os contratos definidos nesta especificação.

---

# 8. Normalização da Representação

A Intermediate Representation poderá adotar formas canônicas para representar construções semanticamente equivalentes.

Essa normalização reduz a diversidade de representações internas, simplifica as transformações realizadas pelo compilador e facilita a implementação das fases de otimização.

---

## 8.1 Objetivo

A normalização da Intermediate Representation possui os seguintes objetivos:

* uniformizar a representação do programa;
* reduzir a complexidade das transformações;
* facilitar a implementação das otimizações;
* preservar integralmente a semântica do programa.

---

## 8.2 Formas Canônicas

Sempre que possível, construções semanticamente equivalentes deverão produzir representações canônicas equivalentes na Intermediate Representation.

A definição dessas formas pertence exclusivamente à implementação.

---

## 8.3 Uniformização

A Intermediate Representation poderá substituir diferentes formas estruturais por uma representação uniforme.

Essa uniformização reduz a quantidade de casos tratados pelas fases posteriores do compilador e melhora a previsibilidade das transformações realizadas sobre a IR.

---

## 8.4 Eliminação de Ambiguidades

A Intermediate Representation não deverá preservar ambiguidades existentes apenas na representação sintática da linguagem.

Toda informação necessária para interpretação do programa deverá estar explicitamente representada na IR.

---

## 8.5 Preparação para Otimizações

A normalização deverá produzir uma representação adequada às fases de otimização do Middle-end.

Seu objetivo não é otimizar o programa, mas fornecer uma base uniforme sobre a qual essas otimizações possam atuar.

---

## 8.6 Independência da Implementação

Os mecanismos utilizados para normalizar a Intermediate Representation pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar os contratos estabelecidos nesta especificação.

---

# 9. Rastreabilidade

Embora a Intermediate Representation seja independente das construções sintáticas da linguagem, poderá ser necessário manter informações que permitam relacionar seus elementos às representações produzidas nas etapas anteriores da compilação.

Essa rastreabilidade auxilia diagnósticos, ferramentas de desenvolvimento e processos de depuração, sem alterar o papel da IR como representação intermediária do programa.

---

## 9.1 Objetivo

A rastreabilidade possui os seguintes objetivos:

* preservar a relação entre a IR e a HIR;
* manter a associação com o código-fonte original;
* apoiar diagnósticos e ferramentas de desenvolvimento;
* facilitar processos de depuração.

---

## 9.2 Relação com a HIR

Sempre que necessário, elementos da Intermediate Representation poderão manter informações suficientes para permitir sua associação com os elementos correspondentes da HIR.

Os mecanismos utilizados para manter essa associação pertencem exclusivamente à implementação.

---

## 9.3 Relação com o Código-Fonte

A Intermediate Representation poderá preservar informações que permitam associar seus elementos ao código-fonte original.

Essas informações destinam-se exclusivamente ao suporte de ferramentas e diagnósticos, não fazendo parte do comportamento funcional da representação.

---

## 9.4 Relação com Diagnósticos

Sempre que necessário, os mecanismos de diagnóstico poderão utilizar as informações de rastreabilidade para localizar, identificar e apresentar elementos do programa ao usuário.

Os mecanismos gerais de emissão, organização e recuperação de diagnósticos pertencem ao Documento 23 — Diagnósticos e Recuperação de Erros. As informações de rastreabilidade da Intermediate Representation fornecem suporte a esses mecanismos, permitindo relacionar os elementos da IR às representações produzidas nas etapas anteriores da compilação.

---

## 9.5 Persistência das Informações

A necessidade de preservar informações de rastreabilidade ao longo das diferentes etapas do Middle-end pertence exclusivamente à implementação.

Sempre que essas informações forem mantidas, deverão permanecer consistentes com a representação correspondente.

---

## 9.6 Independência da Implementação

A forma como as informações de rastreabilidade são armazenadas, atualizadas ou consultadas pertence exclusivamente à implementação.

Independentemente da estratégia adotada, a Intermediate Representation deverá preservar os contratos definidos nesta especificação e manter a equivalência semântica do programa.

---

# 10. Interface da Intermediate Representation

A Intermediate Representation estabelece o contrato de comunicação entre as diferentes fases do compilador após a conclusão do Frontend.

Todas as etapas do Middle-end e do Backend interagem exclusivamente por meio da IR, preservando a independência entre os componentes da arquitetura.

Este documento define apenas os contratos dessa interface, não impondo qualquer estrutura concreta para sua implementação.

---

## 10.1 Objetivo

A interface da Intermediate Representation possui os seguintes objetivos:

* estabelecer um contrato uniforme entre as etapas do compilador;
* permitir a evolução independente das fases do Middle-end;
* preservar a equivalência semântica do programa;
* desacoplar as transformações internas da linguagem Capi.

---

## 10.2 Entrada

A Intermediate Representation é produzida exclusivamente pelo processo de Lowering definido no Documento 20 — Lowering para Intermediate Representation.

Sua construção recebe como entrada uma HIR semanticamente validada e preserva todas as garantias produzidas pelo Frontend.

Nenhuma etapa posterior deverá depender diretamente da HIR.

---

## 10.3 Saída

A Intermediate Representation constitui simultaneamente:

* a saída do processo de Lowering;
* a entrada das fases de otimização do Middle-end;
* a entrada do Backend após o término das otimizações.

Essa representação permanece como o único artefato compartilhado entre essas etapas.

---

## 10.4 Interface com o Middle-end

As fases do Middle-end, incluindo aquelas definidas no Documento 22 — Otimizações sobre a Intermediate Representation, deverão operar exclusivamente sobre a Intermediate Representation.

Cada etapa recebe uma IR consistente, realiza suas transformações e produz uma nova representação igualmente consistente para a etapa seguinte.

---

## 10.5 Interface com o Backend

Ao término das otimizações, o Backend passa a utilizar exclusivamente a Intermediate Representation para realizar a geração de código.

O Backend não deverá depender diretamente da HIR nem das representações produzidas durante o Frontend.

---

## 10.6 Independência da Implementação

Os mecanismos utilizados para implementar essa interface pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar integralmente os contratos definidos nesta especificação.

---

# 11. Evolução da Representação

Durante o Middle-end, a Intermediate Representation poderá sofrer sucessivas transformações.

Essas transformações representam a evolução da representação interna do programa, preservando integralmente sua semântica enquanto preparam a IR para a geração de código.

---

## 11.1 Objetivo

A evolução da Intermediate Representation possui os seguintes objetivos:

* permitir transformações sucessivas da representação;
* preservar a equivalência semântica;
* preparar o programa para a geração de código;
* manter a consistência da IR durante todo o Middle-end.

---

## 11.2 Transformações

Cada fase do Middle-end poderá transformar a Intermediate Representation de acordo com sua responsabilidade específica.

Essas transformações poderão alterar a organização interna da representação, desde que preservem os contratos estabelecidos para a IR.

Entre essas transformações incluem-se as otimizações especificadas no Documento 22 — Otimizações sobre a Intermediate Representation, que atuam exclusivamente sobre a IR preservando integralmente sua equivalência semântica.

---

## 11.3 Preservação da Semântica

Toda transformação realizada sobre a Intermediate Representation deverá preservar integralmente o comportamento observável do programa.

Alterações estruturais são permitidas desde que não modifiquem a semântica estabelecida pelo Frontend.

---

## 11.4 Atualização da Representação

As fases do Middle-end poderão atualizar, substituir ou reorganizar elementos da Intermediate Representation.

Independentemente da estratégia utilizada, a representação produzida deverá permanecer estruturalmente consistente e semanticamente equivalente à representação anterior.

---

## 11.5 Evolução entre Etapas

Ao final de cada fase do Middle-end, a Intermediate Representation deverá permanecer preparada para servir como entrada da etapa seguinte.

Nenhuma fase deverá transferir inconsistências para as fases posteriores do pipeline.

---

## 11.6 Independência da Implementação

Os mecanismos utilizados para transformar e evoluir a Intermediate Representation pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar os contratos definidos nesta especificação.

---

# 12. Diagnósticos

A Intermediate Representation representa uma estrutura interna do compilador.

Consequentemente, a utilização da IR normalmente não produz novos diagnósticos relacionados ao programa analisado.

Entretanto, situações excepcionais poderão impedir sua construção, transformação ou utilização, exigindo a emissão de diagnósticos internos.

Os mecanismos gerais de diagnóstico pertencem ao Documento 23 — Diagnósticos e Recuperação de Erros.

---

## 12.1 Objetivo

Os diagnósticos relacionados à Intermediate Representation possuem os seguintes objetivos:

* identificar inconsistências internas;
* preservar a integridade do pipeline do compilador;
* fornecer informações suficientes para investigação da falha.

---

## 12.2 Situações Excepcionais

Diagnósticos produzidos nesta etapa deverão corresponder exclusivamente a situações excepcionais, como:

* inconsistências estruturais inesperadas;
* violações dos contratos da Intermediate Representation;
* falhas durante transformações internas;
* demais condições que impeçam a continuidade segura do compilador.

Essas situações representam falhas do compilador ou de sua implementação, e não erros do programa analisado.

---

## 12.3 Informações Obrigatórias

Todo diagnóstico deverá conter informações suficientes para permitir a identificação da condição que originou a falha.

Sempre que possível, essas informações deverão preservar a rastreabilidade com a HIR e com o código-fonte original.

---

## 12.4 Relação com o Documento 23

Este documento estabelece apenas os requisitos específicos relacionados aos diagnósticos produzidos durante a utilização da Intermediate Representation.

A apresentação, classificação, recuperação de erros e demais mecanismos gerais pertencem ao Documento 23 — Diagnósticos e Recuperação de Erros.

---

## 12.5 Continuidade

Sempre que puder ser realizado com segurança, as fases do Middle-end deverão prosseguir sua execução após identificar uma situação excepcional.

Quando isso não for possível, a implementação poderá interromper o processamento da Intermediate Representation para preservar a consistência do compilador.

A estratégia utilizada para essa decisão pertence exclusivamente à implementação.

---

## 12.6 Independência da Implementação

Os mecanismos utilizados para identificar, registrar e apresentar diagnósticos pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar os contratos definidos nesta especificação.

---

# 13. Testabilidade

A Intermediate Representation constitui a principal representação manipulada pelo Middle-end do compilador.

Sua natureza estruturada permite a construção de testes independentes que verificam a consistência da representação, a preservação da semântica e o correto funcionamento das transformações realizadas sobre ela.

A implementação deve manter uma suíte abrangente de testes automatizados, incluindo testes de regressão para cada erro identificado e posteriormente corrigido durante o processamento da Intermediate Representation. Esses testes garantem que problemas anteriormente solucionados não sejam reintroduzidos em versões futuras do compilador.

---

## 13.1 Objetivo

Os testes relacionados à Intermediate Representation possuem os seguintes objetivos:

* verificar a consistência estrutural da IR;
* validar a preservação da equivalência semântica;
* detectar regressões;
* garantir comportamento determinístico;
* assegurar a integridade das transformações realizadas pelo Middle-end.

---

## 13.2 Testes Unitários

Cada componente responsável pela manipulação da Intermediate Representation deverá possuir testes específicos.

Entre eles incluem-se, quando aplicável:

* criação de elementos da IR;
* transformação da representação;
* preservação da consistência estrutural;
* preservação da equivalência semântica;
* integração entre elementos relacionados.

---

## 13.3 Golden Tests

A implementação poderá utilizar *Golden Tests* para validar a Intermediate Representation produzida em diferentes etapas do compilador.

Cada execução compara a representação obtida com uma versão previamente validada, permitindo identificar alterações inesperadas na estrutura da IR.

---

## 13.4 Testes de Integração

Além dos testes unitários, deverão existir testes destinados a verificar a integração da Intermediate Representation com:

* o processo de Lowering;
* as fases de otimização;
* o Backend;
* os mecanismos de diagnóstico.

Esses testes asseguram a continuidade arquitetural entre os diferentes componentes do compilador.

---

## 13.5 Benchmarks

A implementação poderá incluir benchmarks destinados a acompanhar o desempenho das operações realizadas sobre a Intermediate Representation.

Entre os aspectos monitorados encontram-se:

* tempo de transformação;
* consumo de memória;
* desempenho em projetos de grande porte;
* impacto de novas estratégias de representação.

Esses benchmarks auxiliam a evolução da implementação sem alterar os contratos definidos nesta especificação.

---

## 13.6 Determinismo

Todos os testes relacionados à Intermediate Representation devem pressupor comportamento determinístico.

Para um mesmo programa, utilizando a mesma versão do compilador e as mesmas opções de compilação, a representação produzida deverá ser funcionalmente equivalente.

---

# 14. Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência da Intermediate Representation utilizada pelo compilador oficial da linguagem Capi.

Seu papel consiste em fornecer uma representação uniforme do programa durante todo o Middle-end, preservando integralmente sua semântica e servindo de base para as transformações posteriores.

---

## 14.1 Papel no Middle-end

A Intermediate Representation constitui a representação oficial utilizada durante todo o Middle-end.

Todas as transformações internas do compilador são realizadas sobre essa representação.

---

## 14.2 Relação com o Lowering

O processo de Lowering produz a primeira versão da Intermediate Representation.

Essa representação constitui o ponto de partida para todas as transformações posteriores realizadas pelo compilador.

---

## 14.3 Relação com as Otimizações

As fases de otimização operam exclusivamente sobre a Intermediate Representation.

Seu objetivo consiste em transformar a representação preservando integralmente a semântica do programa e preparando-a para a geração de código.

---

## 14.4 Independência entre Implementações

Implementações distintas poderão utilizar diferentes estruturas internas para representar a Intermediate Representation.

Independentemente da estratégia adotada, todas deverão preservar:

* os contratos definidos nesta especificação;
* a equivalência semântica do programa;
* a consistência estrutural da representação;
* o comportamento determinístico.

---

## 14.5 Evolução Futura

A arquitetura definida neste documento foi concebida para permitir evolução futura da Intermediate Representation sem alterar os contratos estabelecidos nesta especificação.

Novas formas de representação, melhorias de desempenho e aperfeiçoamentos internos poderão ser incorporados desde que preservem integralmente as garantias aqui definidas.

---

## 14.6 Síntese Arquitetural

A Intermediate Representation representa o núcleo arquitetural do Middle-end do compilador.

Ela estabelece uma representação uniforme, semanticamente consistente e independente da linguagem, permitindo que todas as fases posteriores do compilador operem sobre um modelo comum.

---

# 15. Limites da Intermediate Representation

A arquitetura do compilador da Capi estabelece uma separação clara entre representação e processamento.

A Intermediate Representation constitui exclusivamente a representação utilizada pelo Middle-end, não sendo responsável pelas transformações realizadas sobre ela nem pela geração do código final.

Essa separação reduz o acoplamento entre os componentes do compilador e facilita sua evolução.

---

## 15.1 Objetivo

A definição dos limites da Intermediate Representation possui os seguintes objetivos:

* preservar a responsabilidade única da representação;
* evitar sobreposição entre representação e processamento;
* garantir previsibilidade arquitetural;
* facilitar a evolução da implementação.

---

## 15.2 O que Pertence à Intermediate Representation

Compete exclusivamente à Intermediate Representation:

* representar o programa durante o Middle-end;
* preservar a equivalência semântica do programa;
* servir como base para transformações posteriores;
* estabelecer um contrato comum entre as fases do compilador.

---

## 15.3 O que Não Pertence à Intermediate Representation

Não compete à Intermediate Representation:

* realizar análises da linguagem;
* executar transformações do programa;
* implementar algoritmos de otimização;
* gerar código para plataformas específicas;
* definir estratégias internas de processamento.

Essas responsabilidades pertencem às respectivas etapas do compilador.

---

## 15.4 Relação com o Documento 20

A Intermediate Representation é produzida exclusivamente pelo processo de Lowering definido no Documento 20.

Este documento assume que a representação recebida satisfaz integralmente os contratos estabelecidos naquela etapa e passa a definir apenas sua estrutura conceitual e suas propriedades arquiteturais.

---

## 15.5 Relação com os Documentos Seguintes

O documento seguinte, 22 — Otimizações sobre a Intermediate Representation, especifica as transformações responsáveis por melhorar a qualidade da IR, preservando integralmente sua semântica e preparando-a para as etapas posteriores do compilador.

A Intermediate Representation permanece como a representação oficial do programa durante todo o Middle-end e continua sendo utilizada pelas etapas subsequentes até a geração do código final.

---

## 15.6 Independência da Implementação

A forma como a Intermediate Representation é implementada pertence exclusivamente à implementação.

Independentemente da estrutura adotada, todas as implementações deverão preservar integralmente os contratos definidos nesta especificação.

---

# 16. Princípios Arquiteturais

A Intermediate Representation constitui o núcleo arquitetural do Middle-end do compilador da linguagem Capi.

Sua arquitetura baseia-se em princípios que garantem estabilidade, previsibilidade e independência entre as diferentes fases do pipeline de compilação.

Esses princípios orientam todas as implementações compatíveis com esta especificação.

---

## 16.1 Responsabilidade Única

A Intermediate Representation possui responsabilidade única: representar o programa durante todo o Middle-end do compilador.

Ela não define algoritmos de transformação, otimização ou geração de código, servindo exclusivamente como a representação comum utilizada por essas etapas.

---

## 16.2 Preservação da Semântica

Toda Intermediate Representation deverá preservar integralmente o comportamento observável do programa.

As transformações realizadas sobre a IR poderão modificar sua estrutura interna, mas nunca sua semântica.

Essa propriedade constitui o principal contrato da representação intermediária.

---

## 16.3 Determinismo

Para um mesmo programa, utilizando a mesma versão do compilador e as mesmas opções de compilação, a Intermediate Representation produzida deverá ser funcionalmente equivalente.

Esse princípio assegura previsibilidade durante todas as fases do Middle-end.

---

## 16.4 Evolução Incremental

A Intermediate Representation foi concebida para permitir evolução incremental ao longo do pipeline do compilador.

Cada fase poderá transformar a representação recebida, produzindo uma nova versão igualmente consistente e semanticamente equivalente.

---

## 16.5 Separação de Responsabilidades

A arquitetura do compilador estabelece responsabilidades claramente definidas para cada etapa do pipeline.

A Intermediate Representation representa exclusivamente o programa, enquanto as transformações realizadas sobre ela pertencem às fases do Middle-end e a geração do código pertence ao Backend.

Essa separação constitui um dos princípios fundamentais da arquitetura do compilador da Capi.

---

## 16.6 Independência da Implementação

Os mecanismos utilizados para representar a Intermediate Representation pertencem exclusivamente à implementação.

Independentemente da estrutura adotada, todas as implementações deverão preservar os princípios arquiteturais definidos neste documento.

---

# 17. Considerações sobre a Arquitetura

A Intermediate Representation constitui a principal representação utilizada pelo compilador após a conclusão do Frontend.

Sua existência estabelece uma separação clara entre a análise da linguagem e os processos internos de transformação, permitindo que as fases posteriores operem sobre uma representação uniforme e independente da sintaxe da linguagem.

---

## 17.1 Papel no Pipeline

Dentro do pipeline de compilação, a Intermediate Representation representa o ponto de integração entre o Frontend, o Middle-end e o Backend.

Todas as etapas posteriores ao Lowering passam a utilizar exclusivamente essa representação.

---

## 17.2 Relação com o Middle-end

O Middle-end opera exclusivamente sobre a Intermediate Representation.

Cada uma de suas fases recebe uma representação consistente, realiza as transformações correspondentes à sua responsabilidade e produz uma nova representação igualmente consistente para a etapa seguinte.

---

## 17.3 Relação com o Backend

Ao término do Middle-end, o Backend recebe a Intermediate Representation como única entrada para o processo de geração de código.

Nenhuma dependência direta da HIR ou das estruturas produzidas pelo Frontend deverá existir a partir desse ponto.

---

## 17.4 Estabilidade da Representação

Durante toda sua utilização, a Intermediate Representation deverá preservar seus contratos arquiteturais.

Mesmo quando modificada por diferentes fases do compilador, deverá permanecer estruturalmente consistente e semanticamente equivalente ao programa original.

---

## 17.5 Evolução Futura

A arquitetura descrita neste documento foi concebida para permitir evolução futura da Intermediate Representation sem alterar os contratos definidos nesta especificação.

Novas estratégias de representação, otimizações internas e melhorias de desempenho poderão ser incorporadas desde que preservem integralmente esses contratos.

---

## 17.6 Síntese Arquitetural

A Intermediate Representation constitui o elemento central do Middle-end do compilador.

Ela fornece uma representação uniforme, semanticamente consistente e independente das construções específicas da linguagem, permitindo que todas as etapas posteriores atuem sobre um modelo comum.

---

# 18. Considerações Finais

A Intermediate Representation representa o principal contrato arquitetural entre o Frontend, o Middle-end e o Backend do compilador da linguagem Capi.

Sua função consiste em representar o programa de forma uniforme, preservando integralmente sua semântica e fornecendo uma base estável para todas as transformações realizadas após a análise da linguagem.

---

## 18.1 Síntese da Arquitetura

A Intermediate Representation caracteriza-se pelos seguintes princípios:

* representação uniforme do programa;
* preservação integral da semântica;
* independência das construções da linguagem;
* suporte às transformações do Middle-end;
* independência da implementação.

Esses princípios garantem previsibilidade, interoperabilidade e facilidade de evolução da arquitetura do compilador.

---

## 18.2 Princípios Preservados

Toda implementação compatível com esta especificação deverá preservar:

* a equivalência semântica do programa;
* a consistência estrutural da representação;
* o comportamento determinístico;
* a separação entre representação e processamento;
* a independência entre Frontend, Middle-end e Backend.

Esses princípios asseguram que diferentes implementações possam compartilhar os mesmos contratos arquiteturais da Intermediate Representation.

---

## 18.3 Relação com o Documento Seguinte

Este documento define a Intermediate Representation como a representação oficial utilizada durante o Middle-end do compilador, estabelecendo seus contratos arquiteturais, suas propriedades fundamentais e seu papel no pipeline de compilação.

O documento seguinte, **22 — Otimizações sobre a Intermediate Representation**, especifica as transformações responsáveis por melhorar a qualidade da IR, preservando integralmente sua semântica e preparando-a para a geração de código.

A partir dessa etapa, o foco da especificação deixa de ser a estrutura da representação e passa a concentrar-se nas transformações realizadas sobre ela.

---

## 18.4 Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência da Intermediate Representation utilizada pelo compilador oficial da linguagem Capi.

Outras implementações poderão adotar estruturas internas diferentes para representar a IR, desde que preservem integralmente:

* os contratos definidos nesta especificação;
* a equivalência semântica do programa;
* a consistência estrutural da representação;
* o comportamento determinístico;
* a compatibilidade com as etapas posteriores do compilador.

Dessa forma, este documento estabelece a especificação arquitetural da Intermediate Representation e fornece a base sobre a qual se apoiam todas as transformações realizadas durante o Middle-end do compilador.
