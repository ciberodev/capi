# 24 — Backend e Geração de Código

## Especificação de Implementação da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a especificação de implementação da arquitetura de **Backend e Geração de Código** utilizada pelo compilador oficial da linguagem Capi.

Seu objetivo é estabelecer os contratos arquiteturais responsáveis por transformar a **Intermediate Representation (IR)** otimizada em código executável para a plataforma de destino.

Este documento não especifica arquiteturas de processador, conjuntos de instruções, formatos de executáveis, convenções de chamada (*calling conventions*) nem tecnologias específicas de geração de código. Seu foco está exclusivamente nos contratos que toda implementação compatível deverá respeitar.

---

## 1.2 Escopo

Este documento especifica:

* o papel do Backend na arquitetura do compilador;
* a transformação da Intermediate Representation em código executável;
* os contratos preservados durante a geração de código;
* a relação entre Backend e plataforma de destino;
* os princípios arquiteturais da geração de código.

Não fazem parte deste documento:

* a definição da Intermediate Representation;
* as otimizações realizadas sobre a IR;
* a definição da ABI da linguagem;
* formatos específicos de executáveis;
* arquiteturas de processador;
* algoritmos de seleção de instruções.

Esses assuntos pertencem aos respectivos documentos da especificação de implementação ou às decisões da implementação.

---

## 1.3 Relação com os Documentos Anteriores

O Backend representa a etapa responsável por concluir o processo de compilação, transformando a Intermediate Representation otimizada em código executável.

Relaciona-se diretamente com:

| Documento    | Responsabilidade                                |
| ------------ | ----------------------------------------------- |
| Documento 20 | Lowering para Intermediate Representation       |
| Documento 21 | Intermediate Representation                     |
| Documento 22 | Otimizações sobre a Intermediate Representation |
| Documento 23 | Diagnósticos e Recuperação de Erros             |

Os contratos definidos nesses documentos permanecem integralmente preservados durante a geração de código.

---

## 1.4 Filosofia da Implementação

A arquitetura do Backend possui responsabilidade única: transformar a Intermediate Representation em uma representação executável para a plataforma de destino.

Essa arquitetura separa claramente as responsabilidades de:

* representar o programa;
* otimizar sua representação;
* gerar código para uma arquitetura específica;
* produzir os artefatos finais da compilação.

A forma como essas atividades são implementadas pertence exclusivamente à implementação.

---

## 1.5 Separação entre Especificação e Implementação

Este documento estabelece apenas os contratos arquiteturais da geração de código.

Uma implementação poderá utilizar, por exemplo:

* diferentes arquiteturas de Backend;
* diferentes algoritmos de seleção de instruções;
* diferentes estratégias de alocação de registradores;
* diferentes formatos intermediários;
* diferentes bibliotecas de geração de código.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 2. Papel do Backend

O Backend constitui a etapa responsável por transformar a Intermediate Representation otimizada em código executável compatível com a plataforma de destino.

Seu papel consiste em preservar integralmente a semântica do programa durante essa transformação, produzindo artefatos adequados para execução ou posterior processo de ligação (*linking*).

---

## 2.1 Objetivo

O Backend possui os seguintes objetivos:

* transformar a Intermediate Representation em código executável;
* preservar integralmente a semântica do programa;
* adaptar a representação à plataforma de destino;
* produzir os artefatos finais da compilação;
* fornecer suporte às diferentes arquiteturas suportadas pelo compilador.

---

## 2.2 Papel no Pipeline

O Backend representa a etapa final do pipeline principal de compilação.

Ele recebe como entrada uma Intermediate Representation previamente otimizada e produz como saída os artefatos destinados à plataforma de execução.

---

## 2.3 Responsabilidades

Compete ao Backend:

* consumir a Intermediate Representation;
* selecionar representações compatíveis com a arquitetura de destino;
* produzir código executável;
* preservar os contratos estabelecidos pela especificação da linguagem;
* fornecer suporte às plataformas implementadas.

---

## 2.4 Relação com o Middle-end

O Backend recebe exclusivamente a Intermediate Representation produzida ao término do Middle-end.

Nenhuma responsabilidade pertencente ao Frontend ou ao Middle-end deverá ser repetida durante esta etapa da compilação.

---

## 2.5 Relação com a Plataforma de Destino

O Backend constitui o único componente da arquitetura cuja responsabilidade depende diretamente da plataforma de destino.

Entretanto, essa dependência não altera os contratos definidos pela linguagem nem modifica a semântica do programa representado pela Intermediate Representation.

---

## 2.6 Independência Arquitetural

A arquitetura do Backend permanece independente:

* da sintaxe da linguagem;
* da análise semântica;
* da estrutura da Intermediate Representation;
* das otimizações realizadas pelo Middle-end;
* dos mecanismos internos utilizados pelo compilador.

Sua única responsabilidade consiste em transformar uma Intermediate Representation válida em código executável compatível com a plataforma de destino.

---

# 3. Fluxo da Geração de Código

A geração de código representa a transformação final realizada pelo compilador.

Durante essa etapa, a Intermediate Representation otimizada é convertida em uma representação executável adequada à arquitetura de destino.

Este documento estabelece apenas os contratos arquiteturais desse processo, não impondo qualquer algoritmo específico para sua implementação.

---

## 3.1 Visão Geral

Conceitualmente, o fluxo da geração de código pode ser representado da seguinte forma:

```text id="vqsm66"
 Intermediate Representation
         Otimizada
               │
               ▼
           Backend
               │
               ▼
   Geração de Código
               │
               ▼
     Artefato Executável
```

Cada implementação poderá utilizar diferentes etapas internas para realizar essa transformação.

---

## 3.2 Entrada

O Backend recebe como entrada uma **Intermediate Representation** consistente, produzida conforme os contratos definidos pelos **Documentos 20, 21 e 22**.

Nenhuma etapa anterior da compilação deverá ser repetida durante a geração de código.

---

## 3.3 Transformação

Durante a geração de código, a Intermediate Representation é convertida para uma representação executável compatível com a plataforma de destino.

Essa transformação deverá preservar integralmente os contratos semânticos estabelecidos pelas etapas anteriores da compilação.

---

## 3.4 Saída

Ao término da geração de código, o Backend produz um ou mais artefatos destinados às etapas finais da construção do programa.

A natureza desses artefatos depende da implementação e da plataforma de destino.

---

## 3.5 Relação com Diagnósticos

Situações excepcionais identificadas durante a geração de código poderão produzir diagnósticos utilizando a infraestrutura definida pelo **Documento 23 — Diagnósticos e Recuperação de Erros**.

Esses diagnósticos normalmente correspondem a condições internas da implementação e não a erros presentes no programa analisado.

---

## 3.6 Responsabilidade Única

O Backend possui responsabilidade única: transformar a Intermediate Representation otimizada em código executável para a plataforma de destino.

Este documento não define arquiteturas específicas, algoritmos de geração de código nem estratégias de implementação, deixando essas decisões exclusivamente a cargo da implementação.

---

# 4. Intermediate Representation como Entrada

A Intermediate Representation constitui a única entrada arquitetural do Backend.

Toda geração de código parte exclusivamente da IR produzida ao término do Middle-end, assumindo que ela já satisfaz integralmente os contratos estabelecidos pelas etapas anteriores da compilação.

Este documento estabelece apenas os contratos que governam essa relação, não definindo sua representação concreta.

---

## 4.1 Objetivo

A utilização da Intermediate Representation como entrada possui os seguintes objetivos:

* desacoplar o Backend das etapas anteriores da compilação;
* preservar a separação entre Middle-end e Backend;
* fornecer uma representação uniforme para geração de código;
* permitir diferentes implementações de Backend.

---

## 4.2 Contratos Esperados

Toda Intermediate Representation recebida pelo Backend deverá:

* satisfazer os contratos definidos pelo **Documento 21 — Intermediate Representation**;
* representar um programa semanticamente válido;
* preservar integralmente as garantias produzidas pelo Frontend;
* estar preparada para geração de código.

O Backend poderá assumir que esses contratos já foram integralmente verificados pelas etapas anteriores da compilação.

---

## 4.3 Consistência

A Intermediate Representation recebida deverá apresentar consistência estrutural suficiente para permitir sua transformação em código executável.

O Backend não deverá executar verificações pertencentes às etapas anteriores da compilação.

Situações excepcionais decorrentes de inconsistências internas poderão produzir diagnósticos conforme definido pelo **Documento 23 — Diagnósticos e Recuperação de Erros**.

---

## 4.4 Imutabilidade

A Intermediate Representation deverá permanecer conceitualmente inalterada durante todo o processo de geração de código.

O código executável constitui uma nova representação do programa e não uma modificação da Intermediate Representation existente.

Os mecanismos utilizados para preservar essa propriedade pertencem exclusivamente à implementação.

---

## 4.5 Rastreabilidade

Sempre que aplicável, a implementação poderá preservar mecanismos que permitam relacionar o código gerado aos elementos correspondentes da Intermediate Representation.

Essa rastreabilidade poderá ser utilizada por ferramentas de desenvolvimento, depuração e emissão de diagnósticos.

Os mecanismos utilizados para manter essa correspondência pertencem exclusivamente à implementação.

---

## 4.6 Independência da Implementação

Os mecanismos utilizados para representar, armazenar e consumir a Intermediate Representation pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 5. Geração de Código

A geração de código constitui a responsabilidade central do Backend.

Seu objetivo consiste em transformar a Intermediate Representation em uma representação executável compatível com a plataforma de destino, preservando integralmente a semântica do programa.

Este documento estabelece apenas os contratos arquiteturais dessa transformação.

---

## 5.1 Objetivo

A geração de código possui os seguintes objetivos:

* transformar a Intermediate Representation em código executável;
* preservar integralmente a semântica do programa;
* produzir artefatos compatíveis com a plataforma de destino;
* fornecer suporte às diferentes arquiteturas implementadas.

---

## 5.2 Transformação

A transformação realizada pelo Backend converte a Intermediate Representation em uma representação compatível com a arquitetura de destino.

Essa transformação poderá envolver múltiplas etapas internas, desde que preserve integralmente os contratos definidos nesta especificação.

---

## 5.3 Seleção de Instruções

A seleção das instruções utilizadas para representar cada operação pertence exclusivamente à implementação.

Diferentes implementações poderão adotar estratégias distintas para realizar essa seleção, desde que produzam código semanticamente equivalente ao programa representado pela Intermediate Representation.

---

## 5.4 Emissão de Código

Ao término do processo de transformação, o Backend produz os artefatos correspondentes à plataforma de destino.

Esses artefatos poderão assumir diferentes formas, conforme a implementação adotada.

A forma como essa emissão é realizada pertence exclusivamente à implementação.

---

## 5.5 Preservação da Semântica

Toda transformação realizada pelo Backend deverá preservar integralmente a semântica do programa representado pela Intermediate Representation.

A definição do comportamento esperado do programa pertence principalmente ao **Documento 05 — Semântica Operacional**.

Nenhuma estratégia de geração de código poderá alterar esse comportamento.

---

## 5.6 Independência da Implementação

Os mecanismos utilizados para gerar código pertencem exclusivamente à implementação.

Independentemente da arquitetura adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 6. Arquiteturas de Destino

O Backend permite que uma mesma Intermediate Representation seja convertida para diferentes plataformas de execução.

Este documento não estabelece arquiteturas obrigatórias nem restringe os ambientes suportados, definindo apenas os contratos que garantem a portabilidade da especificação.

---

## 6.1 Objetivo

O suporte a diferentes arquiteturas possui os seguintes objetivos:

* permitir múltiplas plataformas de execução;
* desacoplar a linguagem da arquitetura de hardware;
* facilitar a evolução do compilador;
* preservar a portabilidade da especificação.

---

## 6.2 Independência da Plataforma

A especificação da linguagem permanece independente da plataforma de destino.

As diferenças existentes entre arquiteturas de hardware ou sistemas operacionais pertencem exclusivamente ao Backend e não alteram os contratos definidos pela linguagem.

---

## 6.3 Múltiplos Backends

Uma implementação poderá oferecer suporte a diferentes Backends destinados a arquiteturas distintas.

Cada Backend permanece responsável exclusivamente pela geração de código correspondente à plataforma para a qual foi desenvolvido.

Todos, entretanto, deverão preservar integralmente os contratos definidos nesta especificação.

---

## 6.4 Portabilidade

Sempre que suportado pela implementação, um mesmo programa poderá ser compilado para diferentes plataformas utilizando a mesma Intermediate Representation.

A estratégia utilizada para proporcionar essa portabilidade pertence exclusivamente à implementação.

---

## 6.5 Relação com a ABI

A interação entre o código gerado e o ambiente de execução deverá respeitar a **Application Binary Interface (ABI)** correspondente à plataforma de destino.

A definição da ABI utilizada pertence ao documento específico de interoperabilidade da linguagem e às características da plataforma suportada.

---

## 6.6 Independência da Implementação

Os mecanismos utilizados para suportar diferentes arquiteturas, plataformas e ABIs pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 7. Preservação da Semântica

A preservação da semântica constitui o principal contrato arquitetural do Backend.

Independentemente da plataforma de destino ou da estratégia de geração de código adotada, o comportamento observável do programa deverá permanecer integralmente equivalente ao representado pela Intermediate Representation recebida como entrada.

Este documento estabelece apenas os contratos relacionados a essa preservação, não definindo mecanismos específicos para garanti-la.

---

## 7.1 Objetivo

A preservação da semântica possui os seguintes objetivos:

* garantir equivalência entre a Intermediate Representation e o código gerado;
* assegurar conformidade com a especificação da linguagem;
* preservar a previsibilidade do comportamento do programa;
* permitir diferentes implementações do Backend.

---

## 7.2 Equivalência Semântica

Toda transformação realizada pelo Backend deverá produzir código cujo comportamento seja semanticamente equivalente ao programa representado pela Intermediate Representation.

Essa equivalência constitui um contrato obrigatório desta especificação e deverá ser preservada independentemente da arquitetura de destino.

---

## 7.3 Comportamento Observável

As transformações realizadas pelo Backend não poderão alterar o comportamento observável definido pela linguagem.

Entre os aspectos preservados incluem-se, quando aplicável:

* fluxo de execução;
* resultados produzidos;
* efeitos colaterais;
* interação com o ambiente de execução;
* demais comportamentos definidos pela linguagem.

A definição desses comportamentos pertence principalmente ao **Documento 05 — Semântica Operacional**.

---

## 7.4 Não Alteração da Intermediate Representation

A geração de código não modifica a Intermediate Representation utilizada como entrada.

A IR permanece conceitualmente disponível durante todo o processo de geração de código, enquanto o Backend produz uma nova representação destinada à plataforma de execução.

Os mecanismos utilizados para preservar essa propriedade pertencem exclusivamente à implementação.

---

## 7.5 Limites da Preservação

A preservação da semântica aplica-se exclusivamente ao comportamento definido pela especificação da linguagem.

Aspectos relacionados ao desempenho, organização interna do código gerado ou utilização de recursos da arquitetura pertencem exclusivamente à implementação, desde que não alterem o comportamento definido pela linguagem.

---

## 7.6 Independência da Implementação

Os mecanismos utilizados para preservar a semântica durante a geração de código pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 8. Artefatos Produzidos

Ao término da geração de código, o Backend produz os artefatos correspondentes à plataforma de destino.

Este documento não estabelece formatos obrigatórios para esses artefatos, definindo apenas os contratos arquiteturais relacionados à sua produção.

---

## 8.1 Objetivo

A produção dos artefatos finais possui os seguintes objetivos:

* disponibilizar o programa para execução;
* permitir integração com ferramentas externas;
* suportar diferentes plataformas de destino;
* preservar a independência entre especificação e implementação.

---

## 8.2 Código Objeto

Uma implementação poderá produzir código objeto destinado às etapas posteriores do processo de construção do programa.

A estrutura desse código pertence exclusivamente à implementação e à plataforma de destino.

---

## 8.3 Código Assembly

Uma implementação poderá disponibilizar representações em linguagem Assembly para fins de depuração, análise ou integração com outras ferramentas.

A existência dessa representação é opcional e pertence exclusivamente à implementação.

---

## 8.4 Executáveis

Sempre que suportado pela implementação, o Backend poderá produzir programas executáveis diretamente.

Os formatos utilizados para esses executáveis pertencem exclusivamente à implementação e às plataformas suportadas.

---

## 8.5 Outros Artefatos

Uma implementação poderá produzir outros tipos de artefatos compatíveis com sua arquitetura.

Entre eles incluem-se, quando aplicável:

* bibliotecas;
* módulos compartilhados;
* representações intermediárias adicionais;
* informações de depuração;
* demais formatos suportados.

A definição desses artefatos pertence exclusivamente à implementação.

---

## 8.6 Independência da Implementação

Os mecanismos utilizados para produzir os artefatos finais pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 9. Rastreabilidade

A rastreabilidade permite relacionar o código produzido pelo Backend aos elementos correspondentes da Intermediate Representation.

Essa capacidade facilita atividades de depuração, análise de desempenho, geração de informações auxiliares e integração com ferramentas de desenvolvimento.

Este documento estabelece apenas os contratos arquiteturais dessa rastreabilidade.

---

## 9.1 Objetivo

A rastreabilidade possui os seguintes objetivos:

* relacionar o código gerado à Intermediate Representation;
* preservar a origem das informações utilizadas durante a geração de código;
* apoiar ferramentas de desenvolvimento;
* facilitar processos de depuração.

---

## 9.2 Relação com a Intermediate Representation

Sempre que aplicável, a implementação poderá preservar a correspondência entre elementos da Intermediate Representation e os artefatos produzidos pelo Backend.

Essa correspondência facilita a identificação da origem de cada elemento presente no código gerado.

---

## 9.3 Relação com o Código Gerado

A implementação poderá manter informações que permitam localizar, no código produzido, os elementos correspondentes às construções representadas na Intermediate Representation.

Os mecanismos utilizados para manter essa relação pertencem exclusivamente à implementação.

---

## 9.4 Relação com Diagnósticos

Situações excepcionais identificadas durante a geração de código poderão utilizar as informações de rastreabilidade para produzir diagnósticos mais precisos.

Esses diagnósticos deverão utilizar a infraestrutura definida pelo **Documento 23 — Diagnósticos e Recuperação de Erros**.

---

## 9.5 Relação com Ferramentas

Ferramentas de desenvolvimento poderão utilizar as informações de rastreabilidade produzidas pelo Backend para oferecer funcionalidades adicionais.

Entre elas incluem-se, quando aplicável:

* depuradores;
* analisadores de desempenho;
* ferramentas de inspeção de código;
* ambientes integrados de desenvolvimento;
* demais ferramentas compatíveis.

A forma como essas ferramentas utilizam essas informações pertence exclusivamente à implementação.

---

## 9.6 Independência da Implementação

Os mecanismos utilizados para preservar a rastreabilidade entre a Intermediate Representation e o código gerado pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar os contratos definidos nesta especificação.

---

# 10. Interface do Backend

O Backend constitui a etapa responsável por consumir a Intermediate Representation produzida pelo Middle-end e disponibilizar os artefatos finais da compilação.

Sua interface estabelece os contratos arquiteturais que permitem a integração entre o Middle-end, os componentes responsáveis pela geração de código e as ferramentas utilizadas durante o processo de construção do programa.

Este documento define apenas esses contratos, não impondo qualquer arquitetura específica para sua implementação.

---

## 10.1 Objetivo

A interface do Backend possui os seguintes objetivos:

* integrar o Middle-end ao processo de geração de código;
* preservar os contratos definidos para a Intermediate Representation;
* permitir diferentes implementações do Backend;
* fornecer uma arquitetura uniforme para geração dos artefatos finais.

---

## 10.2 Interface com o Middle-end

O Backend recebe exclusivamente a Intermediate Representation produzida pelo Middle-end.

Essa representação é originada pelo processo de Lowering definido no **Documento 20 — Lowering para Intermediate Representation**, estruturada conforme o **Documento 21 — Intermediate Representation** e transformada pelas otimizações especificadas no **Documento 22 — Otimizações sobre a Intermediate Representation**.

Toda implementação deverá preservar integralmente os contratos produzidos por essas etapas.

---

## 10.3 Interface com a ABI

Durante a geração de código, o Backend deverá produzir artefatos compatíveis com a **Application Binary Interface (ABI)** correspondente à plataforma de destino.

As regras específicas da ABI pertencem à especificação correspondente e às características da arquitetura suportada.

---

## 10.4 Interface com o Processo de Ligação

Os artefatos produzidos pelo Backend poderão servir como entrada para processos posteriores de construção do programa, como ligação (*linking*) ou empacotamento.

A forma como essa integração é realizada pertence exclusivamente à implementação.

---

## 10.5 Interface com Ferramentas

Ferramentas de desenvolvimento poderão utilizar informações produzidas pelo Backend para oferecer funcionalidades adicionais.

Entre elas incluem-se, quando aplicável:

* depuradores;
* analisadores de desempenho;
* ferramentas de inspeção;
* ambientes integrados de desenvolvimento;
* sistemas automatizados de construção.

Os mecanismos utilizados para essa integração pertencem exclusivamente à implementação.

---

## 10.6 Independência da Implementação

Os mecanismos utilizados para integrar o Backend às demais etapas do compilador e às ferramentas externas pertencem exclusivamente à implementação.

Independentemente da arquitetura adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 11. Diagnósticos

A geração de código normalmente opera sobre uma Intermediate Representation previamente validada pelo Frontend e pelo Middle-end.

Consequentemente, diagnósticos produzidos durante o Backend correspondem, em regra, a situações excepcionais relacionadas ao funcionamento interno do compilador ou às limitações da plataforma de destino.

Os mecanismos gerais de diagnóstico pertencem ao **Documento 23 — Diagnósticos e Recuperação de Erros**.

---

## 11.1 Objetivo

Os diagnósticos relacionados ao Backend possuem os seguintes objetivos:

* identificar situações excepcionais durante a geração de código;
* preservar a integridade da geração dos artefatos;
* fornecer informações suficientes para investigação da falha.

---

## 11.2 Situações Excepcionais

Diagnósticos produzidos durante o Backend deverão corresponder exclusivamente a situações excepcionais, como:

* inconsistências inesperadas da Intermediate Representation;
* violações dos contratos arquiteturais;
* limitações não atendidas da plataforma de destino;
* falhas internas da implementação;
* demais condições que impeçam a continuidade segura da geração de código.

Essas situações representam falhas do compilador ou de sua implementação, e não erros do programa analisado.

---

## 11.3 Relação com o Documento 23

Este documento estabelece apenas os requisitos específicos relacionados aos diagnósticos produzidos durante o Backend.

A emissão, organização, apresentação e recuperação de diagnósticos pertencem ao **Documento 23 — Diagnósticos e Recuperação de Erros**.

---

## 11.4 Continuidade

Sempre que puder ser realizado com segurança, o processo de geração de código deverá prosseguir após identificar uma situação excepcional.

Quando isso não for possível, a implementação poderá interromper o processo para preservar a consistência dos artefatos produzidos.

A estratégia utilizada para essa decisão pertence exclusivamente à implementação.

---

## 11.5 Informações Obrigatórias

Todo diagnóstico produzido pelo Backend deverá conter informações suficientes para permitir a identificação da situação correspondente.

Sempre que possível, essas informações deverão preservar a rastreabilidade com a Intermediate Representation e, quando aplicável, com o código gerado.

---

## 11.6 Independência da Implementação

Os mecanismos utilizados para detectar, registrar e apresentar diagnósticos pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 12. Testabilidade

O Backend representa a etapa responsável por produzir os artefatos finais da compilação e, consequentemente, exige mecanismos abrangentes de validação.

Sua natureza dependente da plataforma torna indispensável a existência de testes capazes de verificar tanto a preservação da semântica quanto a compatibilidade dos artefatos produzidos com os ambientes suportados.

A implementação deve manter uma suíte abrangente de testes automatizados, incluindo testes de regressão para cada erro identificado e posteriormente corrigido durante o processo de geração de código. Esses testes garantem que problemas anteriormente solucionados não sejam reintroduzidos em versões futuras do compilador.

---

## 12.1 Objetivo

Os testes relacionados ao Backend possuem os seguintes objetivos:

* verificar a preservação da semântica;
* validar os artefatos produzidos;
* detectar regressões;
* assegurar compatibilidade com as plataformas suportadas;
* validar a integração com as etapas anteriores da compilação.

---

## 12.2 Testes Unitários

Cada componente do Backend deverá possuir testes específicos que validem seu comportamento isoladamente.

Entre eles incluem-se, quando aplicável:

* geração de código;
* emissão dos artefatos;
* preservação da semântica;
* rastreabilidade;
* integração com a Intermediate Representation.

---

## 12.3 Golden Tests

A implementação poderá utilizar *Golden Tests* para validar os artefatos produzidos pelo Backend.

Cada execução compara os resultados obtidos com versões previamente validadas, permitindo identificar alterações inesperadas decorrentes da evolução da implementação.

---

## 12.4 Benchmarks

A implementação poderá manter *benchmarks* destinados a acompanhar o desempenho da geração de código.

Entre os aspectos monitorados incluem-se, quando aplicável:

* tempo de geração;
* consumo de memória;
* qualidade do código produzido;
* desempenho em projetos de grande porte;
* custo computacional das diferentes estratégias de geração.

Esses *benchmarks* auxiliam a evolução da implementação sem alterar os contratos definidos nesta especificação.

---

## 12.5 Testes de Regressão

Sempre que uma falha relacionada à geração de código for corrigida, deverá ser incorporado um teste de regressão correspondente.

Esses testes contribuem para preservar a estabilidade do Backend ao longo da evolução do compilador.

---

## 12.6 Determinismo

Para um mesmo programa, utilizando a mesma versão do compilador, as mesmas opções de compilação e a mesma plataforma de destino, o Backend deverá produzir resultados funcionalmente equivalentes.

Esse princípio assegura previsibilidade para usuários, ferramentas de desenvolvimento e ambientes automatizados de construção.

---

# 13. Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência do Backend utilizado pelo compilador oficial da linguagem Capi.

Seu papel consiste em transformar a Intermediate Representation otimizada em artefatos compatíveis com a plataforma de destino, preservando integralmente a semântica do programa e os contratos estabelecidos pelas etapas anteriores da compilação.

---

## 13.1 Papel na Arquitetura do Compilador

O Backend constitui a etapa final do pipeline principal de compilação.

Ele recebe exclusivamente a Intermediate Representation produzida pelo Middle-end e produz os artefatos finais destinados à execução ou às etapas posteriores de construção do programa.

---

## 13.2 Relação com o Pipeline

A arquitetura do Backend integra-se ao pipeline do compilador da seguinte forma:

* recebe a Intermediate Representation otimizada;
* transforma essa representação em código compatível com a plataforma de destino;
* produz os artefatos finais da compilação;
* preserva integralmente os contratos arquiteturais estabelecidos pelas etapas anteriores.

Nenhuma responsabilidade pertencente ao Frontend ou ao Middle-end deverá ser repetida durante essa etapa.

---

## 13.3 Relação com as Plataformas

A arquitetura definida neste documento permite que diferentes plataformas sejam suportadas por implementações distintas do Backend.

Cada Backend permanece responsável exclusivamente pela plataforma para a qual foi desenvolvido, preservando integralmente a semântica da linguagem independentemente da arquitetura utilizada.

---

## 13.4 Independência entre Implementações

Implementações distintas poderão adotar diferentes arquiteturas internas para geração de código.

Entre elas incluem-se, quando aplicável:

* diferentes algoritmos de seleção de instruções;
* diferentes estratégias de alocação de registradores;
* diferentes modelos de emissão de código;
* diferentes mecanismos de integração com ferramentas externas.

Independentemente da estratégia adotada, todas deverão preservar integralmente os contratos definidos nesta especificação.

---

## 13.5 Evolução Futura

A arquitetura definida neste documento foi concebida para permitir evolução contínua do Backend sem alterar seus contratos fundamentais.

Novas arquiteturas de hardware, novos formatos de artefatos e novas estratégias de geração de código poderão ser incorporados desde que preservem integralmente os princípios estabelecidos nesta especificação.

---

## 13.6 Síntese Arquitetural

O Backend representa o componente responsável por converter a Intermediate Representation em artefatos executáveis compatíveis com a plataforma de destino.

Sua arquitetura estabelece um conjunto uniforme de contratos que permite diferentes implementações preservarem integralmente a semântica da linguagem, a rastreabilidade das informações e a independência entre especificação e implementação.

---

# 14. Limites do Backend

O Backend possui responsabilidade exclusiva sobre a transformação da Intermediate Representation otimizada em artefatos destinados à plataforma de execução.

Ele não participa da análise da linguagem, da otimização da Intermediate Representation nem da definição da ABI, atuando apenas sobre uma representação previamente validada pelo restante do compilador.

Essa separação reduz o acoplamento entre os componentes do compilador e facilita sua evolução.

---

## 14.1 Objetivo

A definição dos limites do Backend possui os seguintes objetivos:

* preservar sua responsabilidade única;
* evitar sobreposição com outras etapas do compilador;
* garantir previsibilidade arquitetural;
* facilitar a evolução da implementação.

---

## 14.2 O que Pertence ao Backend

Compete exclusivamente ao Backend:

* consumir a Intermediate Representation otimizada;
* gerar código compatível com a plataforma de destino;
* produzir os artefatos finais da compilação;
* preservar integralmente a semântica do programa;
* fornecer suporte às arquiteturas implementadas.

---

## 14.3 O que Não Pertence ao Backend

Não compete ao Backend:

* realizar análise léxica, sintática ou semântica;
* produzir ou otimizar a Intermediate Representation;
* definir a Application Binary Interface;
* executar processos de ligação (*linking*);
* definir formatos específicos de executáveis;
* estabelecer regras da linguagem.

Essas responsabilidades pertencem às respectivas etapas do compilador ou aos documentos específicos da implementação.

---

## 14.4 Relação com a ABI

O Backend deverá produzir artefatos compatíveis com a ABI correspondente à plataforma de destino.

Entretanto, a definição da ABI pertence à especificação correspondente e não faz parte deste documento.

---

## 14.5 Relação com o Processo de Ligação

Os artefatos produzidos pelo Backend poderão servir como entrada para processos posteriores de ligação (*linking*) ou empacotamento.

Esses processos não pertencem à responsabilidade do Backend e poderão ser implementados por ferramentas externas ou por componentes adicionais do compilador.

---

## 14.6 Independência da Implementação

Os mecanismos utilizados para implementar o Backend pertencem exclusivamente à implementação.

Independentemente da arquitetura adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 15. Princípios Arquiteturais

O Backend constitui um componente fundamental da arquitetura do compilador, responsável por materializar em código executável a semântica previamente validada pelas etapas anteriores.

Sua arquitetura baseia-se em princípios que asseguram preservação da semântica, portabilidade, previsibilidade e independência entre especificação e implementação.

---

## 15.1 Responsabilidade Única

O Backend possui responsabilidade única: transformar a Intermediate Representation otimizada em artefatos compatíveis com a plataforma de destino.

Não compete a esta etapa realizar análises da linguagem nem modificar os contratos produzidos pelas fases anteriores do compilador.

---

## 15.2 Preservação da Semântica

Toda transformação realizada pelo Backend deverá preservar integralmente a semântica definida pela especificação da linguagem.

Esse princípio constitui o contrato arquitetural mais importante desta etapa da compilação.

---

## 15.3 Portabilidade

A arquitetura do Backend deverá permitir suporte a diferentes plataformas de destino sem alterar os contratos estabelecidos pela linguagem.

As diferenças existentes entre arquiteturas pertencem exclusivamente à implementação.

---

## 15.4 Determinismo

Para um mesmo programa, utilizando a mesma versão do compilador, as mesmas opções de compilação e a mesma plataforma de destino, o Backend deverá produzir resultados funcionalmente equivalentes.

Esse princípio assegura previsibilidade para usuários, ferramentas e ambientes automatizados.

---

## 15.5 Separação de Responsabilidades

A arquitetura do compilador estabelece responsabilidades claramente definidas para cada etapa do pipeline.

O Backend recebe uma Intermediate Representation previamente validada e limita sua atuação à geração dos artefatos destinados à plataforma de execução.

Essa separação reduz o acoplamento entre os componentes do compilador e facilita sua evolução.

---

## 15.6 Independência da Implementação

Os mecanismos utilizados para implementar o Backend pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os princípios arquiteturais definidos nesta especificação.

---

# 16. Considerações sobre a Arquitetura

O Backend representa a etapa responsável por concluir o processo principal de compilação, transformando a Intermediate Representation otimizada em artefatos compatíveis com a plataforma de destino.

Sua arquitetura preserva os contratos estabelecidos pelas etapas anteriores da compilação e fornece a base necessária para que o programa possa ser posteriormente ligado, empacotado e executado.

Este documento estabelece apenas os princípios arquiteturais dessa etapa, mantendo independência em relação às tecnologias utilizadas por cada implementação.

---

## 16.1 Papel no Pipeline

O Backend ocupa a etapa final do pipeline principal de compilação.

Sua entrada consiste exclusivamente na Intermediate Representation produzida pelo Middle-end, enquanto sua saída corresponde aos artefatos destinados à plataforma de execução.

Essa posição permite que toda a análise da linguagem permaneça isolada das particularidades da arquitetura de destino.

---

## 16.2 Relação com o Middle-end

O Backend assume que todas as transformações realizadas pelo Middle-end já foram concluídas.

A Intermediate Representation recebida deverá satisfazer integralmente os contratos definidos pelos **Documentos 20, 21 e 22**, não sendo responsabilidade desta etapa repetir verificações pertencentes ao processo de otimização ou à análise semântica.

---

## 16.3 Relação com a ABI

Durante a geração de código, o Backend deverá respeitar a Application Binary Interface correspondente à plataforma de destino.

Entretanto, a definição da ABI permanece fora do escopo deste documento e pertence à especificação correspondente e às características da arquitetura suportada.

---

## 16.4 Relação com o Runtime

O código produzido pelo Backend poderá interagir com componentes do Runtime da linguagem sempre que necessário para executar programas compatíveis com a especificação.

Os mecanismos utilizados para essa integração pertencem exclusivamente à implementação, desde que preservem integralmente os contratos definidos pela linguagem.

---

## 16.5 Evolução Futura

A arquitetura definida neste documento foi concebida para permitir evolução contínua do Backend.

Novas arquiteturas de hardware, novos formatos de artefatos, novos mecanismos de geração de código e novas estratégias de integração poderão ser incorporados sem alterar os contratos arquiteturais estabelecidos nesta especificação.

---

## 16.6 Síntese Arquitetural

O Backend estabelece a transição entre a representação intermediária do programa e sua representação executável.

Sua arquitetura preserva a separação entre a semântica da linguagem e as particularidades da plataforma de destino, permitindo que diferentes implementações produzam código equivalente mantendo integralmente os contratos definidos pela especificação.

---

# 17. Considerações Finais

O Backend constitui o componente responsável por materializar, em código executável, a semântica preservada durante todo o processo de compilação.

Este documento estabelece os contratos arquiteturais responsáveis pela transformação da Intermediate Representation otimizada em artefatos compatíveis com a plataforma de destino, preservando a independência entre a especificação da linguagem e as estratégias adotadas por cada implementação.

---

## 17.1 Síntese da Arquitetura

A arquitetura do Backend caracteriza-se pelos seguintes princípios:

* transformação da Intermediate Representation em artefatos executáveis;
* preservação integral da semântica da linguagem;
* independência em relação à arquitetura de destino;
* separação entre especificação e implementação;
* suporte a diferentes plataformas e ABIs.

Esses princípios asseguram que diferentes implementações possam utilizar estratégias distintas de geração de código preservando um conjunto comum de contratos arquiteturais.

---

## 17.2 Princípios Preservados

Toda implementação compatível com esta especificação deverá preservar:

* a equivalência semântica entre a Intermediate Representation e o código gerado;
* a separação entre Middle-end e Backend;
* a previsibilidade do processo de geração de código;
* a independência entre especificação e implementação;
* a compatibilidade com a plataforma de destino.

Esses princípios garantem que diferentes implementações permaneçam compatíveis com a arquitetura definida para o compilador da linguagem Capi.

---

## 17.3 Relação com o Documento Seguinte

Este documento define a arquitetura responsável pela geração de código executável a partir da Intermediate Representation otimizada.

O documento seguinte, **25 — Testes do Compilador**, especifica a estratégia arquitetural utilizada para validar todas as etapas do compilador, estabelecendo os contratos relacionados aos testes automatizados, testes de regressão, validação funcional e garantia da conformidade da implementação com a especificação da linguagem.

---

## 17.4 Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência do Backend utilizada pelo compilador oficial da linguagem Capi.

Outras implementações poderão adotar diferentes estratégias para geração de código, seleção de instruções, emissão de artefatos e suporte às plataformas de destino, desde que preservem integralmente:

* os contratos definidos nesta especificação;
* a semântica da linguagem;
* a compatibilidade com as plataformas suportadas;
* a separação entre especificação e implementação;
* a integração com as demais etapas do compilador.

Dessa forma, este documento estabelece a especificação arquitetural do **Backend e Geração de Código**, fornecendo a base comum para todas as implementações compatíveis do compilador da linguagem Capi.
