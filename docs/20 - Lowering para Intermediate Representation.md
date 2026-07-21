# 20 — Lowering para Intermediate Representation (IR)

## Especificação de Implementação da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a especificação de implementação da fase de **Lowering** utilizada pelo compilador oficial da linguagem Capi.

Seu objetivo é estabelecer como a HIR semanticamente validada é transformada em uma **Intermediate Representation (IR)**, preservando integralmente o comportamento do programa e preparando a representação para o Middle-end do compilador.

O Lowering constitui a etapa de transição entre o Frontend e o Middle-end.

---

## 1.2 Escopo

Este documento especifica:

* o papel do Lowering na arquitetura do compilador;
* a transformação da HIR em IR;
* os contratos estabelecidos entre o Frontend e o Middle-end;
* a preservação da semântica durante a transformação;
* a preparação da representação para as fases de otimização.

Não fazem parte deste documento:

* a definição da HIR;
* a análise semântica;
* a estrutura detalhada da IR;
* as otimizações do Middle-end;
* a geração de código.

Esses assuntos são tratados em seus respectivos documentos da especificação de implementação.

---

## 1.3 Relação com os Documentos Anteriores

O Lowering atua diretamente sobre a HIR produzida pela Verificação Semântica definida no Documento 19.

Sua implementação deve preservar integralmente as garantias estabelecidas por:

| Documento    | Responsabilidade                  |
| ------------ | --------------------------------- |
| Documento 02 | Sistema de Tipos                  |
| Documento 04 | Sintaxe da Linguagem              |
| Documento 05 | Semântica Operacional             |
| Documento 13 | Estrutura do Compilador           |
| Documento 16 | Representação Semântica (HIR)     |
| Documento 17 | Resolução de Nomes e Escopos      |
| Documento 18 | Inferência e Verificação de Tipos |
| Documento 19 | Verificação Semântica             |

Ao iniciar esta etapa, toda a análise do programa já se encontra concluída.

Este documento especifica apenas a transformação da representação semântica para uma representação intermediária apropriada ao Middle-end.

---

## 1.4 Filosofia da Implementação

O Lowering possui responsabilidade única: transformar a HIR semanticamente validada em uma Intermediate Representation (IR) equivalente.

Essa etapa não realiza novas verificações da linguagem nem produz código de máquina.

Seu papel consiste exclusivamente em transformar a representação do programa, preservando integralmente sua semântica.

---

## 1.5 Separação entre Especificação e Implementação

Este documento define os contratos funcionais do processo de Lowering, mas não impõe algoritmos específicos para sua implementação.

Uma implementação poderá utilizar, por exemplo:

* visitantes da HIR;
* transformações sucessivas;
* reescrita baseada em regras;
* construtores especializados da IR;
* outras estratégias equivalentes.

Independentemente da técnica adotada, todas as implementações deverão produzir uma Intermediate Representation funcionalmente equivalente para um mesmo programa válido e preservar os contratos definidos nesta especificação.

---

# 2. Papel do Lowering

O Lowering constitui a etapa responsável por converter a HIR produzida pelo Frontend em uma Intermediate Representation (IR) adequada às fases posteriores do compilador.

Essa transformação estabelece a transição entre a análise da linguagem e o processamento interno realizado pelo Middle-end.

Ao término desta etapa, a representação do programa deixa de ser organizada segundo os conceitos da linguagem Capi e passa a ser organizada segundo os conceitos da IR.

---

## 2.1 Entrada

A entrada do Lowering consiste na HIR produzida pela Verificação Semântica.

Nessa etapa:

* todas as referências já foram resolvidas;
* todas as informações de tipos já foram determinadas;
* todas as regras semânticas da linguagem já foram verificadas;
* a HIR encontra-se semanticamente consistente.

---

## 2.2 Saída

Ao término do Lowering, a saída consiste em uma Intermediate Representation (IR) semanticamente equivalente à HIR recebida.

Essa representação deverá preservar integralmente o comportamento observável do programa e constituirá a entrada das fases de otimização do Middle-end.

---

## 2.3 Responsabilidades

Compete ao Lowering:

* transformar a HIR em IR;
* preservar a semântica do programa;
* converter construções de alto nível para representações intermediárias;
* preparar a representação para as fases de otimização;
* estabelecer o contrato entre o Frontend e o Middle-end.

---

## 2.4 Limitações

Não compete ao Lowering:

* resolver identificadores;
* determinar tipos;
* verificar regras semânticas;
* realizar otimizações;
* gerar código de máquina.

Essas responsabilidades pertencem às etapas anteriores ou posteriores do pipeline do compilador.

---

## 2.5 Continuidade do Pipeline

O Lowering representa a conclusão do Frontend e o início efetivo do Middle-end.

A partir dessa etapa, o compilador deixa de interpretar diretamente os conceitos da linguagem Capi e passa a operar sobre uma representação intermediária própria.

Essa separação reduz o acoplamento entre a linguagem e as fases posteriores da compilação.

---

## 2.6 Independência Arquitetural

O Lowering deve permanecer completamente independente:

* da arquitetura de hardware;
* da ABI da plataforma;
* do Backend;
* da geração de código;
* das otimizações específicas da IR.

Sua responsabilidade limita-se exclusivamente à transformação da HIR em uma Intermediate Representation semanticamente equivalente.

---

# 3. Fluxo do Lowering

O Lowering ocorre imediatamente após a conclusão da Verificação Semântica.

Durante essa etapa, o compilador percorre a HIR validada, transforma suas construções em elementos equivalentes da Intermediate Representation (IR) e consolida a representação utilizada pelo Middle-end.

Ao final do processo, a IR encontra-se pronta para as fases de otimização.

---

## 3.1 Visão Geral

Conceitualmente, o fluxo do Lowering pode ser representado da seguinte forma:

```text
          HIR Validada
                │
                ▼
     Recepção da Representação
                │
                ▼
 Transformação para Estruturas da IR
                │
                ▼
 Consolidação da Intermediate Representation
                │
                ▼
         IR Produzida
                │
                ▼
        Middle-end (Otimizações)
```

A HIR permanece conceitualmente inalterada durante todo o processo de Lowering. A Intermediate Representation constitui uma nova representação construída a partir da HIR e não uma modificação da representação existente.

---

## 3.2 Recepção da HIR

O primeiro passo do Lowering consiste em receber a HIR produzida pela Verificação Semântica.

Essa representação já contém todas as informações necessárias para a construção da IR, não sendo permitidas novas verificações relacionadas à linguagem.

---

## 3.3 Transformação Semântica

Após receber a HIR, a implementação transforma cada elemento da representação em construções semanticamente equivalentes da IR.

Essa transformação deverá preservar integralmente o comportamento observável do programa.

Os mecanismos utilizados para realizar essa transformação pertencem exclusivamente à implementação.

---

## 3.4 Construção da IR

À medida que os elementos da HIR são processados, a Intermediate Representation é construída de forma consistente.

A estrutura concreta da IR pertence ao documento específico que a define e não faz parte desta especificação.

---

## 3.5 Consolidação da IR

Ao término do Lowering, toda a representação do programa deverá estar disponível na Intermediate Representation.

Essa representação constitui o contrato entre o processo de Lowering e as fases posteriores do Middle-end.

---

## 3.6 Princípio da Responsabilidade Única

O Lowering possui responsabilidade única: transformar uma HIR semanticamente válida em uma Intermediate Representation semanticamente equivalente.

Ele não realiza verificações da linguagem, não executa otimizações e não produz código executável.

Seu resultado consiste exclusivamente em uma IR consistente e preparada para as etapas seguintes do compilador.

---

# 4. Objetivos do Lowering

O Lowering representa a etapa responsável por transformar a HIR em uma Intermediate Representation (IR) adequada ao Middle-end do compilador.

Essa transformação preserva integralmente o comportamento semântico do programa, eliminando dependências da representação utilizada pelo Frontend e preparando a IR para as etapas posteriores de otimização.

---

## 4.1 Objetivo

O Lowering possui os seguintes objetivos:

* transformar a HIR em uma Intermediate Representation;
* preservar integralmente a semântica do programa;
* eliminar dependências da representação de alto nível;
* produzir uma representação adequada às otimizações do Middle-end;
* estabelecer a transição entre Frontend e Middle-end.

---

## 4.2 Natureza da Transformação

O Lowering consiste em uma transformação entre representações internas do compilador.

Essa transformação não modifica o significado do programa nem introduz novas interpretações sobre suas construções.

Seu objetivo é apenas converter a representação semântica da linguagem para uma forma mais adequada ao processamento interno do compilador.

---

## 4.3 Preservação da Semântica

Toda transformação realizada durante o Lowering deverá preservar integralmente o comportamento observável do programa.

A representação produzida deverá ser semanticamente equivalente à HIR recebida, independentemente da estratégia utilizada pela implementação.

---

## 4.4 Eliminação de Construções de Alto Nível

Durante o Lowering, construções próprias da HIR poderão ser substituídas por representações mais adequadas à Intermediate Representation.

Essa substituição não altera a semântica do programa, apenas modifica sua forma de representação interna.

Os mecanismos utilizados para realizar essa transformação pertencem exclusivamente à implementação.

---

## 4.5 Consistência da Transformação

Cada elemento produzido na IR deverá possuir correspondência semântica com a representação existente na HIR.

Nenhuma informação necessária às fases posteriores poderá ser perdida durante essa transformação.

---

## 4.6 Independência da Implementação

Os mecanismos utilizados para realizar o Lowering pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir uma Intermediate Representation funcionalmente equivalente para um mesmo programa.

---

# 5. HIR como Entrada

A HIR constitui a única entrada do processo de Lowering.

Ela representa o resultado final do Frontend e contém todas as informações necessárias para a construção da Intermediate Representation.

O Lowering utiliza essa representação como fonte de informação, sem realizar novas análises relacionadas à linguagem.

---

## 5.1 Objetivo

A utilização da HIR como entrada possui os seguintes objetivos:

* preservar a separação entre Frontend e Middle-end;
* evitar a repetição das etapas de análise;
* fornecer uma representação semanticamente consistente;
* servir como base para a construção da IR.

---

## 5.2 Estado Esperado

Ao iniciar o Lowering, a HIR deverá apresentar:

* identificadores resolvidos;
* informações completas de tipos;
* regras semânticas validadas;
* consistência global da representação;
* informações necessárias para a construção da IR.

Essas propriedades são garantidas pelas etapas anteriores do Frontend.

---

## 5.3 Garantias Recebidas

O Lowering assume como garantidas todas as propriedades estabelecidas pelos documentos anteriores.

Entre elas incluem-se:

* validade sintática;
* resolução completa de nomes;
* consistência do Sistema de Tipos;
* validade semântica do programa.

Nenhuma dessas propriedades deverá ser novamente verificada durante esta etapa.

---

## 5.4 Informações Utilizadas

Durante o processo de transformação, o Lowering poderá utilizar todas as informações presentes na HIR necessárias para construir a Intermediate Representation.

A forma como essas informações são acessadas, organizadas ou armazenadas pertence exclusivamente à implementação.

---

## 5.5 Imutabilidade da HIR

A HIR deverá permanecer conceitualmente inalterada durante todo o processo de Lowering.

A Intermediate Representation constitui uma nova representação do programa e não uma modificação da HIR existente.

A preservação da HIR durante o Lowering permite manter a rastreabilidade com o código-fonte original e disponibilizar a representação semântica produzida pelo Frontend para diagnósticos, depuração e ferramentas de desenvolvimento.

Os mecanismos utilizados para preservar essa propriedade pertencem exclusivamente à implementação.

---

## 5.6 Independência da Implementação

A forma como a HIR é percorrida, consultada ou utilizada durante o Lowering pertence exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar integralmente os contratos estabelecidos para a HIR.

---

# 6. Construção da Intermediate Representation

A Intermediate Representation (IR) constitui o resultado produzido pelo processo de Lowering.

Ela representa o programa em uma forma apropriada para as etapas do Middle-end, mantendo equivalência funcional com a HIR e eliminando dependências das construções específicas da linguagem.

Este documento estabelece apenas os contratos relacionados à construção da IR. Sua estrutura detalhada será definida no documento específico da Intermediate Representation.

---

## 6.1 Objetivo

A construção da IR possui os seguintes objetivos:

* representar o programa em uma forma intermediária;
* preservar integralmente sua semântica;
* fornecer uma base uniforme para as otimizações;
* estabelecer a representação utilizada pelo Middle-end.

---

## 6.2 Estrutura Geral

A organização interna da Intermediate Representation pertence exclusivamente à sua especificação.

O processo de Lowering deverá apenas produzir uma representação compatível com os contratos definidos para essa estrutura.

---

## 6.3 Representação Canônica

Sempre que possível, construções semanticamente equivalentes deverão produzir representações equivalentes na IR.

Essa uniformização facilita as etapas posteriores de otimização e reduz a complexidade do Middle-end.

Os critérios utilizados para essa canonicalização pertencem exclusivamente à implementação.

---

## 6.4 Preservação da Semântica

A construção da IR não poderá alterar o comportamento observável do programa.

Toda transformação realizada durante o Lowering deverá preservar integralmente as garantias estabelecidas pelo Frontend.

---

## 6.5 Contrato da IR

Ao término desta etapa, a Intermediate Representation deverá:

* representar integralmente o programa;
* preservar sua semântica;
* estar estruturalmente consistente;
* estar preparada para as fases de otimização do Middle-end.

Esse conjunto de propriedades constitui o contrato arquitetural estabelecido pelo processo de Lowering.

---

## 6.6 Independência da Implementação

Os mecanismos utilizados para construir a Intermediate Representation pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir uma IR funcionalmente equivalente e compatível com os contratos definidos nesta especificação.

---

# 7. Preservação do Comportamento

O princípio fundamental do Lowering consiste em preservar integralmente o comportamento observável do programa durante a transformação da HIR para a Intermediate Representation (IR).

Embora a forma de representação seja modificada, o significado do programa deverá permanecer inalterado.

Toda implementação compatível com esta especificação deverá garantir essa equivalência semântica.

---

## 7.1 Objetivo

A preservação do comportamento possui os seguintes objetivos:

* manter o significado do programa;
* garantir equivalência entre HIR e IR;
* permitir otimizações posteriores sem alterar a semântica;
* preservar o contrato entre Frontend e Middle-end.

---

## 7.2 Equivalência Semântica

A IR produzida deverá representar exatamente o mesmo comportamento definido pela HIR.

Diferenças estruturais entre as representações são permitidas, desde que não alterem o comportamento observável do programa.

---

## 7.3 Ordem das Operações

Sempre que a ordem de avaliação influenciar o comportamento do programa, essa ordem deverá ser preservada durante o processo de Lowering.

A forma utilizada para representar essa ordem pertence exclusivamente à implementação.

---

## 7.4 Controle de Fluxo

As estruturas de controle existentes na HIR deverão ser convertidas para construções equivalentes da Intermediate Representation.

Essa transformação poderá modificar a organização interna do fluxo de controle, desde que preserve integralmente seu comportamento semântico.

---

## 7.5 Dados e Estado

As transformações realizadas durante o Lowering deverão preservar o estado observável do programa.

Valores, efeitos colaterais e demais propriedades relevantes não poderão ser alterados durante a construção da Intermediate Representation.

---

## 7.6 Independência da Implementação

Os mecanismos utilizados para preservar a equivalência semântica pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir uma IR funcionalmente equivalente para um mesmo programa.

---

# 8. Simplificação da Representação

Durante o processo de Lowering, a implementação poderá substituir construções de alto nível por representações intermediárias mais uniformes.

Essa simplificação possui caráter exclusivamente estrutural e não modifica o comportamento observável do programa.

Seu objetivo é produzir uma representação mais adequada às fases posteriores do Middle-end.

---

## 8.1 Objetivo

A simplificação da representação possui os seguintes objetivos:

* reduzir a diversidade de construções presentes na representação;
* uniformizar elementos semanticamente equivalentes;
* facilitar as fases de otimização;
* preparar a IR para as transformações posteriores.

---

## 8.2 Eliminação de Açúcar Sintático

Construções introduzidas apenas por conveniência da linguagem poderão ser convertidas para formas intermediárias equivalentes.

Da mesma forma, construções de alto nível que não possuam correspondência direta na Intermediate Representation poderão ser decompostas em sequências equivalentes de operações mais elementares.

Essas transformações deverão preservar integralmente a semântica do programa.

Os mecanismos utilizados para realizar essas transformações pertencem exclusivamente à implementação.

---

## 8.3 Uniformização das Construções

Sempre que possível, construções semanticamente equivalentes deverão produzir estruturas equivalentes na Intermediate Representation.

Essa uniformização reduz a complexidade das fases posteriores do compilador.

---

## 8.4 Canonicalização

A implementação poderá adotar uma forma canônica para representar determinadas construções da linguagem.

Os critérios utilizados para essa canonicalização pertencem exclusivamente à implementação, desde que preservem integralmente a semântica do programa.

---

## 8.5 Preparação para Otimizações

A IR produzida deverá estar preparada para servir como entrada das fases de otimização do Middle-end.

O Lowering não realiza otimizações, mas poderá produzir uma representação que facilite sua execução posterior.

---

## 8.6 Independência da Implementação

Os mecanismos utilizados para simplificar ou canonicalizar a representação pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir representações funcionalmente equivalentes.

---

# 9. Intermediate Representation Produzida

A Intermediate Representation produzida pelo Lowering constitui a representação oficial utilizada pelo Middle-end.

Ela representa o programa de forma independente das construções específicas da linguagem Capi, preservando todas as propriedades semânticas estabelecidas pelo Frontend.

Essa representação servirá de base para as fases de otimização e, posteriormente, para a geração de código.

---

## 9.1 Objetivo

A Intermediate Representation possui os seguintes objetivos:

* representar integralmente o programa;
* preservar sua semântica;
* fornecer uma base uniforme para as otimizações;
* servir como entrada para as fases posteriores do compilador.

A estrutura completa da Intermediate Representation será especificada no Documento 21 — Intermediate Representation (IR). O Lowering produz uma representação compatível com os contratos definidos naquele documento.

---

## 9.2 Propriedades da IR

Ao término do Lowering, a Intermediate Representation deverá apresentar, entre outras, as seguintes propriedades:

* consistência estrutural;
* equivalência semântica com a HIR;
* independência das construções de alto nível da linguagem;
* adequação às etapas do Middle-end.

---

## 9.3 Consistência

Toda IR produzida deverá satisfazer os contratos estabelecidos por sua própria especificação.

Nenhuma inconsistência estrutural poderá ser introduzida durante o processo de Lowering.

---

## 9.4 Rastreabilidade

Sempre que necessário, a Intermediate Representation deverá preservar informações suficientes para manter a rastreabilidade com a HIR e, consequentemente, com o código-fonte original.

Os mecanismos utilizados para manter essa rastreabilidade pertencem exclusivamente à implementação.

---

## 9.5 Entrada para o Middle-end

A IR produzida constitui a entrada das fases posteriores do Middle-end.

Essas fases poderão assumir que a representação já preserva integralmente a semântica do programa e atende aos contratos definidos para a Intermediate Representation.

---

## 9.6 Independência da Implementação

Os mecanismos utilizados para produzir, organizar e armazenar a Intermediate Representation pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir uma IR funcionalmente equivalente e compatível com os contratos definidos nesta especificação.

---

# 10. Interface do Lowering

O Lowering estabelece a interface entre o Frontend e o Middle-end do compilador.

Seu contrato consiste em receber uma HIR semanticamente validada e produzir uma Intermediate Representation (IR) consistente, preservando integralmente o comportamento observável do programa.

Este documento define apenas o comportamento esperado dessa interface, não impondo qualquer estratégia específica para sua implementação.

---

## 10.1 Objetivo

A interface do Lowering possui os seguintes objetivos:

* receber a HIR produzida pelo Frontend;
* transformar a representação semântica em IR;
* preservar integralmente a semântica do programa;
* disponibilizar uma representação adequada às fases de otimização.

---

## 10.2 Entrada

A entrada do Lowering consiste exclusivamente na HIR produzida pela Verificação Semântica.

Essa representação contém:

* identificadores resolvidos;
* informações completas de tipos;
* regras semânticas validadas;
* informações de rastreabilidade;
* demais propriedades produzidas pelo Frontend.

A implementação poderá utilizar estruturas auxiliares adicionais, desde que preserve integralmente os contratos estabelecidos para a HIR.

---

## 10.3 Saída

Ao término do Lowering, a saída consiste em uma Intermediate Representation semanticamente equivalente à HIR recebida.

Essa representação deverá:

* preservar integralmente o comportamento do programa;
* satisfazer os contratos definidos para a IR;
* servir como entrada das fases de otimização do Middle-end.

---

## 10.4 Contrato entre Frontend e Middle-end

O Lowering representa o ponto de transição entre o Frontend e o Middle-end.

O Middle-end poderá assumir que:

* toda a análise da linguagem foi concluída;
* a HIR encontra-se semanticamente válida;
* a IR produzida preserva integralmente a semântica do programa.

Da mesma forma, o Lowering não deverá antecipar responsabilidades pertencentes às fases de otimização.

Essa separação preserva o Princípio da Responsabilidade Única e reduz o acoplamento entre os componentes do compilador.

---

## 10.5 Relação com a Intermediate Representation

A Intermediate Representation constitui exclusivamente o resultado do processo de Lowering.

Sua definição estrutural pertence ao documento específico da IR.

Este documento estabelece apenas os contratos relacionados à sua construção.

---

## 10.6 Independência da Implementação

Os mecanismos utilizados para implementar a interface do Lowering pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar os contratos definidos neste documento e produzir comportamento funcionalmente equivalente.

---

# 11. Diagnósticos

O processo de Lowering pressupõe que toda a análise da linguagem já foi concluída.

Consequentemente, esta etapa normalmente não produz novos diagnósticos relacionados à sintaxe, aos nomes, aos tipos ou à semântica do programa.

Entretanto, situações excepcionais poderão impedir a construção da Intermediate Representation, exigindo a produção de diagnósticos internos.

Os mecanismos gerais de emissão de diagnósticos pertencem ao Documento 23 — Diagnósticos e Recuperação de Erros.

---

## 11.1 Objetivo

Os diagnósticos desta etapa possuem os seguintes objetivos:

* identificar situações que impeçam a construção da IR;
* preservar a consistência do pipeline do compilador;
* fornecer informações suficientes para auxiliar a investigação da falha.

---

## 11.2 Situações Excepcionais

Diagnósticos produzidos durante o Lowering deverão corresponder exclusivamente a situações excepcionais, como:

* inconsistências internas inesperadas;
* violações dos contratos entre etapas;
* representações inválidas recebidas do Frontend;
* demais condições que impeçam a continuidade segura do processo.

Essas situações representam falhas do compilador ou de sua implementação, e não erros do programa analisado.

---

## 11.3 Informações Obrigatórias

Todo diagnóstico produzido durante o Lowering deverá conter informações suficientes para permitir a identificação da condição que originou a falha.

Sempre que possível, essas informações deverão preservar a rastreabilidade com a HIR e com o código-fonte original.

---

## 11.4 Relação com o Documento 23

Este documento especifica apenas os requisitos relacionados às situações excepcionais identificadas durante o Lowering.

A apresentação dos diagnósticos, sua organização, recuperação de erros e demais mecanismos gerais pertencem ao Documento 23 — Diagnósticos e Recuperação de Erros.

---

## 11.5 Continuidade

Sempre que puder ser realizado com segurança, o Lowering deverá prosseguir sua execução após identificar uma situação excepcional.

Quando isso não for possível, a implementação poderá interromper a construção da Intermediate Representation para preservar a consistência do pipeline.

A estratégia utilizada para essa decisão pertence exclusivamente à implementação.

---

## 11.6 Independência da Implementação

Os mecanismos utilizados para identificar e produzir diagnósticos pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar os contratos definidos nesta especificação.

---

# 12. Consistência da Intermediate Representation

Ao término do Lowering, a Intermediate Representation deverá apresentar um estado estruturalmente consistente e semanticamente equivalente à HIR que lhe deu origem.

Essa consistência estabelece o contrato entre o processo de Lowering e as fases posteriores do Middle-end.

---

## 12.1 Objetivo

A consistência da Intermediate Representation possui os seguintes objetivos:

* preservar a equivalência semântica com a HIR;
* garantir a integridade estrutural da IR;
* preparar a representação para as otimizações;
* assegurar a continuidade do pipeline do compilador.

A implementação poderá utilizar mecanismos internos para verificar a consistência estrutural da Intermediate Representation produzida.

Esses mecanismos pertencem exclusivamente à implementação.

---

## 12.2 Estado Inicial

Antes do início do Lowering, o compilador dispõe de uma HIR semanticamente validada.

Essa representação constitui a única fonte de informação necessária para a construção da Intermediate Representation.

---

## 12.3 Estado Final

Após concluído o Lowering, toda a representação do programa deverá estar disponível na Intermediate Representation.

A IR deverá satisfazer integralmente os contratos definidos para sua estrutura e preservar o comportamento observável do programa.

---

## 12.4 Contrato Arquitetural

O Lowering estabelece o contrato arquitetural entre o Frontend e o Middle-end.

Esse contrato garante que:

* toda a análise da linguagem foi concluída;
* a HIR foi integralmente transformada para a IR;
* a representação preserva a semântica do programa;
* a IR encontra-se consistente e preparada para as fases de otimização.

---

## 12.5 Preparação para Otimizações

A Intermediate Representation produzida nesta etapa constitui a entrada das fases de otimização do Middle-end.

Essas fases poderão concentrar-se exclusivamente na transformação e melhoria da IR, sem necessidade de interpretar novamente as construções específicas da linguagem Capi.

---

## 12.6 Independência da Implementação

Os mecanismos utilizados para preservar a consistência da Intermediate Representation pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir uma IR estruturalmente consistente, semanticamente equivalente à HIR e compatível com os contratos definidos nesta especificação.

---

# 13. Testabilidade

O processo de Lowering constitui uma etapa determinística de transformação entre representações internas do compilador.

Sua natureza permite a construção de testes independentes, verificando que programas semanticamente equivalentes produzem Intermediate Representations compatíveis com os contratos definidos nesta especificação.

A implementação deve manter uma suíte abrangente de testes automatizados, incluindo testes de regressão para cada erro identificado e posteriormente corrigido durante o processo de Lowering. Esses testes garantem que problemas anteriormente solucionados não sejam reintroduzidos em versões futuras do compilador.

---

## 13.1 Objetivo

Os testes do Lowering possuem os seguintes objetivos:

* verificar a correta transformação da HIR em IR;
* validar a preservação da semântica;
* detectar regressões;
* garantir comportamento determinístico;
* assegurar a consistência da Intermediate Representation.

---

## 13.2 Testes Unitários

Cada componente responsável pelo processo de Lowering deverá possuir testes específicos.

Entre eles incluem-se, quando aplicável:

* transformação de construções da HIR;
* construção da Intermediate Representation;
* preservação da equivalência semântica;
* consistência estrutural da IR;
* integração entre transformações relacionadas.

---

## 13.3 Golden Tests

A implementação poderá utilizar *Golden Tests* para validar a Intermediate Representation produzida.

Cada execução compara a IR obtida com uma versão previamente validada, permitindo identificar alterações inesperadas no processo de transformação.

---

## 13.4 Testes de Integração

Além dos testes unitários, deverão existir testes destinados a verificar a integração do Lowering com:

* a Verificação Semântica;
* a Intermediate Representation;
* as fases de otimização;
* os mecanismos de diagnósticos.

Esses testes asseguram a continuidade arquitetural entre o Frontend e o Middle-end.

---

## 13.5 Benchmarks

A implementação poderá incluir benchmarks destinados a acompanhar o desempenho do processo de Lowering.

Entre os aspectos monitorados encontram-se:

* tempo de transformação;
* consumo de memória;
* desempenho em projetos de grande porte;
* impacto de novas estratégias de transformação.

Esses benchmarks auxiliam a evolução da implementação sem alterar os contratos definidos nesta especificação.

---

## 13.6 Determinismo

Todos os testes relacionados ao Lowering devem pressupor comportamento determinístico.

Para um mesmo programa, utilizando a mesma versão do compilador e as mesmas opções de compilação, a Intermediate Representation produzida deverá ser funcionalmente equivalente.

---

# 14. Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência do processo de Lowering do compilador oficial da linguagem Capi.

Seu papel consiste em transformar a HIR semanticamente validada em uma Intermediate Representation adequada ao Middle-end.

---

## 14.1 Papel no Middle-end

O Lowering representa a primeira etapa do Middle-end.

Sua responsabilidade consiste em converter a representação produzida pelo Frontend para a forma intermediária utilizada pelas fases posteriores do compilador.

---

## 14.2 Relação com a HIR

A HIR constitui a única fonte de informação utilizada durante o processo de Lowering.

Ela permanece conceitualmente inalterada durante toda a transformação, servindo apenas como entrada para a construção da Intermediate Representation.

---

## 14.3 Relação com a Intermediate Representation

A Intermediate Representation constitui o resultado produzido pelo Lowering.

As fases posteriores do Middle-end passam a operar exclusivamente sobre essa representação, sem necessidade de consultar diretamente a HIR.

---

## 14.4 Independência entre Implementações

Implementações distintas poderão utilizar estratégias diferentes para realizar o processo de Lowering.

Independentemente da técnica adotada, todas deverão preservar:

* os contratos definidos neste documento;
* a equivalência semântica entre HIR e IR;
* o comportamento determinístico;
* a consistência estrutural da Intermediate Representation.

---

## 14.5 Evolução Futura

A arquitetura definida neste documento foi concebida para permitir evolução futura da implementação sem alterar os contratos estabelecidos para o Lowering.

Novas estratégias de transformação, melhorias de desempenho e otimizações internas poderão ser incorporadas desde que preservem integralmente as garantias desta especificação.

---

## 14.6 Síntese Arquitetural

O Lowering representa a transição entre a análise da linguagem e o processamento interno realizado pelo Middle-end.

Ao término desta etapa, toda a representação do programa encontra-se disponível na Intermediate Representation, preservando integralmente sua semântica e preparando-a para as fases de otimização.

---

# 15. Limites do Lowering

A arquitetura do compilador da Capi adota uma divisão clara de responsabilidades entre as diferentes etapas do pipeline.

O Lowering constitui exclusivamente o mecanismo de transformação entre a HIR e a Intermediate Representation, não assumindo responsabilidades pertencentes ao Frontend nem às fases posteriores do Middle-end.

Essa separação reduz o acoplamento entre os componentes do compilador e facilita sua evolução.

---

## 15.1 Objetivo

A definição dos limites do Lowering possui os seguintes objetivos:

* preservar a responsabilidade única da etapa;
* evitar sobreposição entre fases;
* garantir a previsibilidade do pipeline;
* facilitar a evolução da implementação.

---

## 15.2 O que Pertence a esta Etapa

Compete exclusivamente ao Lowering:

* transformar a HIR em Intermediate Representation;
* preservar a equivalência semântica entre as representações;
* consolidar a IR;
* estabelecer a transição entre Frontend e Middle-end.

---

## 15.3 O que Não Pertence a esta Etapa

Não compete ao Lowering:

* realizar análises da linguagem;
* verificar regras semânticas;
* executar otimizações;
* gerar código de máquina;
* produzir representações específicas de plataforma.

Essas responsabilidades pertencem a outras etapas do compilador.

---

## 15.4 Relação com o Documento 19

O Lowering assume que a HIR recebida já passou pelas fases de Inferência e Verificação de Tipos (Documento 18) e de Verificação Semântica (Documento 19).

Nenhuma responsabilidade pertencente a essas etapas deverá ser repetida durante o processo de Lowering.

---

## 15.5 Relação com os Documentos Seguintes

Após a conclusão do Lowering, a Intermediate Representation torna-se a representação oficial utilizada pelo Middle-end.

Os documentos seguintes especificam sua estrutura, as transformações realizadas sobre ela e os mecanismos utilizados para produzir o código final do programa.

---

## 15.6 Independência da Implementação

Os mecanismos utilizados para realizar o processo de Lowering pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar integralmente a separação de responsabilidades estabelecida nesta especificação.

---

# 16. Princípios Arquiteturais

O processo de Lowering representa a transição entre o Frontend e o Middle-end do compilador da linguagem Capi.

Sua arquitetura baseia-se em princípios que garantem previsibilidade, preservação da semântica e independência entre as diferentes fases do pipeline.

Esses princípios orientam todas as implementações compatíveis com esta especificação.

---

## 16.1 Responsabilidade Única

O Lowering possui responsabilidade única: transformar uma HIR semanticamente validada em uma Intermediate Representation semanticamente equivalente.

Essa delimitação reduz o acoplamento entre componentes e simplifica a evolução do compilador.

---

## 16.2 Preservação da Semântica

Toda implementação deverá assegurar que a Intermediate Representation preserve integralmente o comportamento observável do programa.

Nenhuma transformação realizada durante o Lowering poderá modificar a semântica estabelecida pelas etapas anteriores do Frontend.

---

## 16.3 Determinismo

O processo de Lowering deverá produzir resultados determinísticos.

Para um mesmo programa, utilizando a mesma versão do compilador e as mesmas opções de compilação, a Intermediate Representation produzida deverá ser funcionalmente equivalente.

---

## 16.4 Evolução Incremental

A arquitetura foi concebida para permitir evolução incremental da implementação.

Novas estratégias de transformação poderão ser incorporadas desde que preservem integralmente os contratos definidos nesta especificação.

---

## 16.5 Separação de Responsabilidades

Cada etapa do pipeline possui responsabilidades claramente definidas.

O Lowering não deve assumir responsabilidades pertencentes ao Frontend, às fases de otimização do Middle-end ou à geração de código.

Essa separação constitui um dos princípios fundamentais da arquitetura do compilador da Capi.

---

## 16.6 Independência da Implementação

Os mecanismos utilizados para implementar o processo de Lowering pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar os princípios arquiteturais definidos neste documento.

---

# 17. Considerações sobre a Arquitetura

A arquitetura do Lowering representa o ponto de transição entre a análise da linguagem e o processamento interno realizado pelo Middle-end.

Ao término desta etapa, o programa deixa de ser representado pela HIR e passa a ser processado exclusivamente por meio da Intermediate Representation.

---

## 17.1 Papel no Pipeline

Dentro do pipeline de compilação, o Lowering representa a etapa responsável por concluir o Frontend e iniciar o Middle-end.

Seu resultado estabelece a fronteira entre a análise da linguagem e as transformações internas realizadas pelo compilador.

---

## 17.2 Relação com o Frontend

O Lowering depende integralmente das garantias produzidas pelo Frontend.

Ele assume que:

* a análise sintática foi concluída;
* todos os identificadores foram resolvidos;
* todas as informações de tipos foram determinadas;
* todas as regras semânticas foram verificadas.

Dessa forma, pode concentrar-se exclusivamente na transformação da representação.

---

## 17.3 Relação com o Middle-end

As fases posteriores do Middle-end recebem como entrada uma Intermediate Representation consistente e semanticamente equivalente à HIR.

Essas fases poderão concentrar-se exclusivamente na transformação e otimização da IR, sem necessidade de interpretar novamente os conceitos específicos da linguagem Capi.

---

## 17.4 Estabilidade da Intermediate Representation

Ao término do Lowering, a Intermediate Representation representa a forma oficial do programa utilizada pelo Middle-end.

As fases posteriores poderão transformá-la livremente, desde que preservem a semântica estabelecida durante o processo de Lowering.

---

## 17.5 Evolução Futura

A arquitetura descrita neste documento foi projetada para permitir evolução futura da implementação sem alterar os contratos estabelecidos para o processo de Lowering.

Melhorias internas, novas estratégias de transformação e aperfeiçoamentos de desempenho poderão ser incorporados desde que preservem integralmente as garantias desta especificação.

---

## 17.6 Síntese Arquitetural

O Lowering conclui a transição entre o Frontend e o Middle-end.

Seu resultado é uma Intermediate Representation consistente, semanticamente equivalente à HIR e preparada para servir de base às fases de otimização do compilador.

---

# 18. Considerações Finais

O Lowering constitui a etapa responsável por transformar a representação semântica produzida pelo Frontend em uma Intermediate Representation adequada ao Middle-end.

Sua responsabilidade consiste exclusivamente em preservar a semântica do programa durante essa transformação, estabelecendo o contrato arquitetural entre a análise da linguagem e as etapas posteriores do compilador.

---

## 18.1 Síntese da Arquitetura

O processo de Lowering é caracterizado pelos seguintes princípios:

* responsabilidade única;
* preservação integral da semântica;
* transformação determinística;
* separação entre análise e otimização;
* independência da implementação.

Esses princípios garantem previsibilidade, estabilidade e facilidade de evolução da arquitetura do compilador.

---

## 18.2 Princípios Preservados

Toda implementação compatível com esta especificação deverá preservar:

* equivalência funcional entre HIR e IR;
* comportamento determinístico;
* consistência estrutural da Intermediate Representation;
* separação entre transformação e otimização;
* independência entre Frontend e Middle-end.

Esses princípios asseguram interoperabilidade entre diferentes implementações do compilador.

---

## 18.3 Relação com o Documento Seguinte

Este documento define a transformação da HIR semanticamente validada para uma Intermediate Representation (IR), preservando integralmente o comportamento do programa e concluindo a transição entre o Frontend e o Middle-end.

O documento seguinte, **21 — Intermediate Representation (IR)**, especifica a estrutura, os contratos e as propriedades da representação intermediária produzida pelo Lowering e utilizada pelas fases de otimização do compilador.

A partir dessa etapa, o pipeline passa a operar exclusivamente sobre a Intermediate Representation, sem depender diretamente das representações produzidas pelo Frontend.

---

## 18.4 Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência do processo de Lowering do compilador oficial da linguagem Capi.

Outras implementações poderão adotar estratégias diferentes para transformar a HIR em Intermediate Representation, desde que preservem integralmente:

* os contratos definidos neste documento;
* a equivalência semântica entre HIR e IR;
* o comportamento determinístico;
* a consistência estrutural da Intermediate Representation;
* a compatibilidade com as fases posteriores do Middle-end.

Dessa forma, este documento conclui a especificação da transição entre o Frontend e o Middle-end e estabelece a base para a definição da Intermediate Representation utilizada pelo compilador da linguagem Capi.
