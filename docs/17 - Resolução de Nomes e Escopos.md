# 17 — Resolução de Nomes e Escopos

## Especificação de Implementação da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a especificação de implementação da fase de Resolução de Nomes e Escopos (*Name Resolution*) utilizada pelo compilador oficial da linguagem Capi.

Seu objetivo é estabelecer como os identificadores presentes na Representação Semântica de Alto Nível (HIR) são associados às respectivas declarações, transformando referências puramente sintáticas em vínculos semânticos explícitos.

A Resolução de Nomes constitui a primeira fase ativa da análise semântica do compilador e fornece a base necessária para a Inferência e Verificação de Tipos, Verificação Semântica e Lowering para a Intermediate Representation (IR).

---

## 1.2 Escopo

Este documento especifica:

* o papel da Resolução de Nomes na arquitetura do compilador;
* os princípios que orientam a construção e utilização dos escopos;
* as regras gerais para associação entre referências e declarações;
* o enriquecimento da HIR com informações de resolução;
* a interface com as fases posteriores do Frontend.

Não fazem parte deste documento:

* a inferência de tipos;
* a verificação de tipos;
* a verificação semântica;
* o processo de Lowering;
* a definição da Intermediate Representation (IR).

Esses assuntos são tratados em seus respectivos documentos da especificação de implementação.

---

## 1.3 Relação com os Documentos Anteriores

A Resolução de Nomes atua diretamente sobre a HIR definida no Documento 16.

Sua implementação deve preservar integralmente as garantias estabelecidas por:

| Documento    | Responsabilidade              |
| ------------ | ----------------------------- |
| Documento 02 | Sistema de Tipos              |
| Documento 04 | Sintaxe da Linguagem          |
| Documento 05 | Semântica Operacional         |
| Documento 13 | Estrutura do Compilador       |
| Documento 15 | Parser e AST                  |
| Documento 16 | Representação Semântica (HIR) |

O Documento 16 define a representação semântica utilizada pelo Frontend.

Este documento especifica como essa representação é enriquecida por meio da associação entre identificadores e os elementos declarados no programa.

---

## 1.4 Filosofia da Implementação

A Resolução de Nomes possui uma responsabilidade única: identificar a declaração correspondente a cada referência presente na HIR.

Essa fase não interpreta tipos nem valida regras semânticas da linguagem.

Seu papel consiste exclusivamente em estabelecer as relações entre nomes utilizados no programa e os elementos que os definem, preservando a consistência dos escopos e preparando a HIR para as etapas seguintes da análise semântica.

---

## 1.5 Separação entre Especificação e Implementação

Este documento define os contratos funcionais da Resolução de Nomes, mas não impõe algoritmos nem estruturas específicas para sua implementação.

Uma implementação poderá utilizar, por exemplo:

* tabelas de símbolos;
* mapas hierárquicos;
* grafos de escopos;
* estruturas indexadas;
* outras abordagens equivalentes.

Independentemente da estratégia adotada, todas as implementações deverão produzir resultados funcionalmente equivalentes para um mesmo programa válido e preservar os contratos definidos nesta especificação.

---

# 2. Papel da Resolução de Nomes

A Resolução de Nomes constitui a primeira etapa da análise semântica do compilador.

Seu objetivo é transformar referências textuais presentes na HIR em associações explícitas com os elementos declarados no programa, permitindo que as fases posteriores trabalhem diretamente sobre entidades semânticas, e não apenas sobre identificadores.

---

## 2.1 Entrada

A entrada da Resolução de Nomes consiste na HIR produzida conforme definido no Documento 16.

Nessa etapa, a representação já descreve a estrutura semântica do programa, porém referências entre elementos ainda podem estar pendentes de resolução.

---

## 2.2 Saída

Ao término da Resolução de Nomes, a HIR deverá conter associações explícitas entre cada referência válida e sua respectiva declaração.

Essa representação enriquecida constitui a entrada para as fases de Inferência e Verificação de Tipos.

---

## 2.3 Responsabilidades

Compete à Resolução de Nomes:

* construir e organizar os escopos do programa;
* registrar os elementos declarados;
* resolver referências entre identificadores e declarações;
* detectar nomes inexistentes ou ambíguos;
* enriquecer a HIR com informações de resolução.

A verificação da correção dos tipos ou das regras semânticas não pertence a esta fase.

---

## 2.4 Limitações

Não compete à Resolução de Nomes:

* inferir tipos;
* verificar compatibilidade entre tipos;
* validar regras semânticas;
* avaliar expressões;
* gerar código.

Essas responsabilidades pertencem às fases posteriores do Frontend.

---

## 2.5 Enriquecimento da HIR

A Resolução de Nomes não altera a estrutura conceitual da HIR.

Seu papel consiste em associar informações adicionais aos elementos existentes, estabelecendo os vínculos entre referências e declarações preservando a organização previamente construída.

---

## 2.6 Independência Arquitetural

A Resolução de Nomes deve permanecer completamente independente:

* da geração de código;
* da arquitetura de hardware;
* da ABI da plataforma;
* do Backend;
* da Intermediate Representation (IR).

Sua responsabilidade limita-se exclusivamente ao enriquecimento semântico da HIR por meio da resolução de identificadores.

---

# 3. Fluxo da Resolução

A Resolução de Nomes ocorre imediatamente após a construção inicial da HIR.

Durante essa etapa, o compilador identifica os elementos declarados, organiza os escopos existentes e estabelece as associações entre cada referência e sua respectiva declaração.

Ao final do processo, a HIR encontra-se preparada para as etapas de Inferência e Verificação de Tipos.

---

## 3.1 Visão Geral

Conceitualmente, o fluxo da Resolução de Nomes pode ser representado da seguinte forma:

```text
            HIR Inicial
                 │
                 ▼
      Construção dos Escopos
                 │
                 ▼
   Registro das Declarações
                 │
                 ▼
  Resolução das Referências
                 │
                 ▼
     HIR com Nomes Resolvidos
                 │
                 ▼
 Inferência e Verificação de Tipos
```

A HIR permanece como a representação compartilhada entre todas as fases do Frontend.

---

## 3.2 Construção dos Escopos

O primeiro passo da Resolução de Nomes consiste na identificação dos escopos presentes no programa.

Cada escopo estabelece uma região de visibilidade para os elementos declarados e define o contexto no qual referências deverão ser resolvidas.

A estratégia utilizada para construir esses escopos pertence exclusivamente à implementação.

---

## 3.3 Registro das Declarações

Após identificar os escopos, a implementação registra todos os elementos que introduzem nomes no programa.

Esses registros constituem a base utilizada para resolver as referências encontradas durante a análise.

Os mecanismos empregados para armazenar essas informações pertencem exclusivamente à implementação.

---

## 3.4 Resolução das Referências

Uma vez registradas as declarações, cada referência presente na HIR é associada ao elemento correspondente, respeitando as regras de visibilidade e escopo definidas pela linguagem.

Sempre que uma referência não puder ser resolvida ou resultar em ambiguidades, deverão ser produzidos os diagnósticos apropriados.

---

## 3.5 Enriquecimento da HIR

As associações produzidas durante a Resolução de Nomes são incorporadas à HIR como informações semânticas adicionais.

Esse enriquecimento preserva a estrutura existente da representação e prepara o programa para as fases seguintes da análise semântica.

---

## 3.6 Princípio da Responsabilidade Única

A Resolução de Nomes possui responsabilidade única: associar referências aos elementos declarados no programa.

Ela não interpreta tipos, não valida regras semânticas e não modifica a estrutura lógica da HIR.

Seu resultado consiste exclusivamente em uma representação semanticamente enriquecida quanto às relações entre identificadores e declarações.

---

# 4. Escopos

A Resolução de Nomes organiza o programa em escopos que determinam a região de visibilidade das declarações.

Cada escopo define o conjunto de elementos que podem ser referenciados a partir de determinado ponto do programa, estabelecendo os limites para a busca de identificadores durante a análise semântica.

---

## 4.1 Objetivo

Os escopos possuem os seguintes objetivos:

* organizar a visibilidade das declarações;
* definir os limites da resolução de nomes;
* preservar o isolamento entre diferentes contextos;
* permitir o aninhamento de regiões de visibilidade;
* fornecer base para a construção da tabela de símbolos.

---

## 4.2 Escopo Global

O escopo global corresponde ao nível mais externo do programa e constitui a raiz da hierarquia de escopos.

Ele contém todas as declarações pertencentes ao nível superior do programa, conforme definido pela linguagem.

Todos os demais escopos são descendentes diretos ou indiretos desse escopo.

---

## 4.3 Escopos Locais

Construções da linguagem poderão introduzir novos escopos locais.

Entre elas incluem-se, quando aplicável:

* módulos;
* funções;
* blocos;
* estruturas de controle;
* demais construções definidas pela linguagem.

As construções que efetivamente introduzem novos escopos são definidas pelo Documento 04 — Sintaxe da Linguagem.

---

## 4.4 Região de Visibilidade

Cada escopo define uma região na qual suas declarações podem ser referenciadas.

A visibilidade de um elemento é determinada pelas regras da linguagem e pela posição ocupada pelo escopo dentro da hierarquia do programa.

A interpretação dessas regras constitui responsabilidade da Resolução de Nomes.

---

## 4.5 Independência entre Escopos

Cada escopo representa um contexto independente para a resolução de identificadores.

Declarações pertencentes a um escopo não alteram diretamente a organização dos demais, exceto conforme as regras de aninhamento e visibilidade definidas pela linguagem.

---

## 4.6 Independência da Implementação

Os mecanismos utilizados para representar escopos pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar as mesmas regras de visibilidade e produzir resultados funcionalmente equivalentes para um mesmo programa.

---

# 5. Hierarquia de Escopos

Os escopos existentes em um programa organizam-se de forma hierárquica.

Essa organização determina a ordem utilizada pela Resolução de Nomes para localizar declarações e estabelecer a associação entre referências e elementos declarados.

---

## 5.1 Organização Hierárquica

Todo escopo, exceto o escopo global, encontra-se contido em outro escopo.

Essa relação estabelece uma hierarquia na qual cada escopo herda a visibilidade dos escopos externos, respeitando as regras definidas pela linguagem.

---

## 5.2 Relação Pai-Filho

Cada escopo pode possuir um único escopo pai e zero ou mais escopos filhos.

Essa relação permite representar o aninhamento estrutural existente no programa e constitui a base para o processo de busca de identificadores.

A forma utilizada para representar essa hierarquia pertence exclusivamente à implementação.

---

## 5.3 Busca Hierárquica

Quando um identificador não puder ser resolvido no escopo atual, a busca deverá prosseguir nos escopos ancestrais, conforme as regras de visibilidade da linguagem.

Essa busca encerra-se quando:

* a declaração correspondente for encontrada;
* o escopo global for alcançado sem sucesso;
* outra condição definida pela linguagem interromper o processo.

---

## 5.4 Fronteiras de Escopo

A transição entre escopos deve respeitar as fronteiras estabelecidas pela linguagem.

Essas fronteiras determinam quais declarações permanecem visíveis e quais deixam de participar da resolução de nomes.

As regras específicas pertencem à definição da linguagem.

---

## 5.5 Determinismo

Para um mesmo programa válido, utilizando a mesma versão do compilador e as mesmas opções de compilação, a organização hierárquica dos escopos deverá produzir resultados funcionalmente equivalentes.

Esse princípio garante previsibilidade durante toda a Resolução de Nomes.

---

## 5.6 Independência da Implementação

A estratégia utilizada para percorrer a hierarquia de escopos pertence exclusivamente à implementação.

Este documento estabelece apenas o comportamento esperado da resolução, não impondo algoritmos específicos para sua realização.

---

# 6. Tabela de Símbolos

A Resolução de Nomes necessita manter uma representação das declarações disponíveis em cada escopo.

Conceitualmente, essa representação corresponde à Tabela de Símbolos, responsável por registrar os elementos que introduzem nomes no programa e permitir sua localização durante a análise.

Este documento utiliza o termo "Tabela de Símbolos" como um conceito arquitetural. Sua representação concreta pertence exclusivamente à implementação.

---

## 6.1 Objetivo

A Tabela de Símbolos possui os seguintes objetivos:

* registrar declarações;
* permitir a localização de identificadores;
* preservar a identidade dos elementos declarados;
* apoiar a Resolução de Nomes;
* fornecer informações para as fases posteriores da análise semântica.

A Tabela de Símbolos mantém associações entre identificadores e os elementos correspondentes da HIR, permitindo que a Resolução de Nomes estabeleça vínculos semânticos sem duplicar as informações já existentes na representação semântica.

---

## 6.2 Registro de Declarações

Todo elemento que introduz um nome no programa deverá possuir um registro associado durante a Resolução de Nomes.

Entre esses elementos incluem-se, quando aplicável:

* módulos;
* tipos;
* funções;
* parâmetros;
* variáveis;
* constantes;
* demais construções declarativas da linguagem.

---

## 6.3 Consulta de Símbolos

Durante a resolução de uma referência, a implementação consulta os registros disponíveis nos escopos acessíveis.

A consulta deve respeitar integralmente as regras de:

* visibilidade;
* hierarquia de escopos;
* prioridade entre declarações;
* restrições definidas pela linguagem.

---

## 6.4 Identidade dos Símbolos

Cada símbolo representa uma declaração específica do programa.

Essa identidade deverá permanecer estável durante toda a análise semântica, permitindo que diferentes referências sejam associadas ao mesmo elemento declarado.

Os mecanismos utilizados para representar essa identidade pertencem exclusivamente à implementação.

---

## 6.5 Relação com a HIR

A Tabela de Símbolos constitui um mecanismo auxiliar da Resolução de Nomes.

Os símbolos registrados são utilizados para enriquecer a HIR com associações explícitas entre referências e declarações.

A HIR permanece como a representação semântica oficial do Frontend, enquanto a Tabela de Símbolos representa apenas um mecanismo de apoio à implementação.

---

## 6.6 Independência da Implementação

A implementação poderá utilizar tabelas, mapas, grafos, índices ou quaisquer outras estruturas para representar os símbolos do programa.

Independentemente da estratégia adotada, todas as implementações deverão produzir associações semanticamente equivalentes entre referências e declarações, preservando os contratos definidos nesta especificação.

---

# 7. Declarações

A Resolução de Nomes inicia-se pela identificação dos elementos que introduzem nomes no programa.

Esses elementos passam a constituir declarações conhecidas pelo compilador, podendo posteriormente ser referenciados por outras construções da linguagem conforme as regras de visibilidade e escopo.

---

## 7.1 Objetivo

A identificação das declarações possui os seguintes objetivos:

* registrar todos os nomes introduzidos no programa;
* estabelecer a identidade de cada declaração;
* permitir a resolução posterior das referências;
* preservar a organização dos escopos;
* preparar a HIR para o enriquecimento semântico.

---

## 7.2 Elementos Declarativos

Introduzem declarações todos os elementos da linguagem definidos como declarativos.

Entre eles incluem-se, quando aplicável:

* módulos;
* tipos;
* funções;
* parâmetros;
* variáveis;
* constantes;
* demais construções definidas pela linguagem.

As construções que efetivamente introduzem declarações são definidas pelo Documento 04 — Sintaxe da Linguagem.

---

## 7.3 Registro das Declarações

Cada declaração deverá ser registrada no escopo correspondente durante a Resolução de Nomes.

Esse registro estabelece a associação entre o identificador declarado e o elemento semântico representado na HIR.

A forma utilizada para realizar esse registro pertence exclusivamente à implementação.

---

## 7.4 Identidade das Declarações

Cada declaração representa uma entidade semântica distinta.

Mesmo quando diferentes declarações utilizam o mesmo identificador em escopos distintos, cada uma delas deverá possuir identidade própria durante toda a análise semântica.

Essa identidade será utilizada pelas fases posteriores do compilador.

---

## 7.5 Disponibilidade

Uma declaração somente poderá ser utilizada conforme as regras de visibilidade e disponibilidade estabelecidas pela linguagem.

A interpretação dessas regras constitui responsabilidade da Resolução de Nomes.

A verificação de aspectos semânticos adicionais pertence às fases posteriores do Frontend.

---

## 7.6 Independência da Implementação

Os mecanismos utilizados para registrar declarações pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão identificar as mesmas declarações e produzir resultados funcionalmente equivalentes.

---

# 8. Referências

Uma referência corresponde ao uso de um identificador que pretende acessar uma declaração previamente existente no programa.

A responsabilidade da Resolução de Nomes consiste em identificar qual declaração corresponde a cada referência, respeitando as regras de escopo e visibilidade da linguagem.

---

## 8.1 Objetivo

A resolução das referências possui os seguintes objetivos:

* localizar a declaração correspondente;
* estabelecer vínculos semânticos na HIR;
* eliminar ambiguidades entre identificadores;
* preparar a representação para as fases posteriores.

---

## 8.2 Categorias de Referências

Entre as referências que poderão ser resolvidas incluem-se, quando aplicável:

* referências a variáveis;
* referências a constantes;
* referências a parâmetros;
* referências a funções;
* referências a tipos;
* referências a módulos;
* acessos qualificados;
* referências definidas pela linguagem.

As construções sintáticas correspondentes são definidas pelo Documento 04.

---

## 8.3 Referências Qualificadas

A linguagem poderá permitir referências qualificadas compostas por múltiplos identificadores.

Nesses casos, a Resolução de Nomes deverá interpretar cada qualificador conforme as regras estabelecidas pela linguagem, preservando a associação correta entre os elementos envolvidos.

As regras específicas pertencem à definição da linguagem.

---

## 8.4 Referências Inválidas

Quando uma referência não puder ser associada a nenhuma declaração válida, deverá ser produzido o diagnóstico correspondente.

Sempre que possível, a implementação deverá continuar a análise semântica, permitindo a identificação de múltiplos problemas em uma única compilação.

As estratégias de recuperação pertencem exclusivamente à implementação.

---

## 8.5 Referências Ambíguas

Quando múltiplas declarações puderem satisfazer uma mesma referência e a linguagem não definir um critério determinístico de escolha, a Resolução de Nomes deverá produzir um diagnóstico de ambiguidade.

Nenhuma associação arbitrária deverá ser realizada nesses casos.

---

## 8.6 Independência da Implementação

Os algoritmos utilizados para localizar declarações pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão resolver referências equivalentes de maneira funcionalmente idêntica.

---

# 9. Associação entre Referências e Declarações

A principal responsabilidade da Resolução de Nomes consiste em estabelecer uma associação explícita entre cada referência presente na HIR e a declaração correspondente.

Após essa etapa, as fases posteriores deixam de operar sobre identificadores textuais e passam a utilizar diretamente os elementos semânticos previamente resolvidos.

---

## 9.1 Objetivo

A associação entre referências e declarações possui os seguintes objetivos:

* eliminar dependência de buscas repetidas por identificadores;
* preservar a identidade dos elementos declarados;
* enriquecer semanticamente a HIR;
* preparar a representação para a Inferência e Verificação de Tipos.

---

## 9.2 Associação Semântica

Uma referência corretamente resolvida deverá manter uma associação explícita com a declaração correspondente.

Essa associação representa um vínculo semântico permanente durante toda a análise do programa.

A forma utilizada para representar esse vínculo pertence exclusivamente à implementação.

---

## 9.3 Identidade Preservada

Diferentes referências poderão apontar para uma mesma declaração.

Todas essas referências deverão permanecer associadas ao mesmo elemento semântico, preservando sua identidade durante toda a compilação.

Essa propriedade reduz ambiguidades e evita duplicação de informações na HIR.

---

## 9.4 Associação Determinística

Para um mesmo programa válido, utilizando a mesma versão do compilador e as mesmas opções de compilação, toda referência deverá ser associada sempre à mesma declaração.

Esse princípio garante previsibilidade e reprodutibilidade durante a Resolução de Nomes.

---

## 9.5 Preparação para as Fases Posteriores

Após concluída a associação entre referências e declarações, a HIR passa a disponibilizar uma representação semanticamente enriquecida.

As fases de Inferência e Verificação de Tipos, Verificação Semântica e Lowering deixam de depender da resolução textual de identificadores, operando diretamente sobre as associações estabelecidas nesta etapa.

---

## 9.6 Independência da Implementação

Os mecanismos utilizados para representar as associações entre referências e declarações pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar a correspondência entre referências e elementos declarados, garantindo comportamento funcionalmente equivalente durante toda a análise semântica.

---

# 10. Ordem de Resolução

Quando uma referência precisa ser associada a uma declaração, a Resolução de Nomes deve seguir uma ordem de busca bem definida.

Essa ordem garante comportamento determinístico e assegura que diferentes implementações do compilador produzam resultados funcionalmente equivalentes para um mesmo programa.

As regras específicas de visibilidade são definidas pela linguagem. Este documento estabelece apenas os princípios que orientam sua aplicação durante a análise semântica.

---

## 10.1 Objetivo

A ordem de resolução possui os seguintes objetivos:

* garantir previsibilidade na associação entre referências e declarações;
* preservar a hierarquia de escopos;
* eliminar ambiguidades decorrentes da ordem de busca;
* preparar a HIR para as fases posteriores.

---

## 10.2 Escopo Atual

A busca por uma declaração inicia-se no escopo onde a referência ocorre.

Sempre que uma declaração compatível estiver disponível nesse escopo, ela deverá ser considerada antes de qualquer declaração pertencente a escopos externos.

Essa prioridade preserva o princípio da localidade definido pela linguagem.

---

## 10.3 Escopos Ancestrais

Quando nenhuma declaração compatível puder ser encontrada no escopo atual, a busca prossegue pelos escopos ancestrais.

Essa progressão deverá respeitar a hierarquia estabelecida durante a construção dos escopos, até que:

* uma declaração válida seja encontrada;
* o escopo global seja alcançado;
* outra condição definida pela linguagem interrompa a busca.

---

## 10.4 Visibilidade Externa

Após esgotada a busca nos escopos locais, a Resolução de Nomes poderá considerar elementos disponibilizados por mecanismos de visibilidade externa definidos pela linguagem.

Entre esses mecanismos podem incluir-se, quando aplicável:

* módulos importados;
* bibliotecas padrão;
* prelúdios automáticos;
* outras formas de disponibilização de declarações.

As regras específicas pertencem à definição da linguagem.

---

## 10.5 Determinismo

A ordem de resolução deve produzir resultados determinísticos.

Para um mesmo programa válido, utilizando a mesma versão do compilador e as mesmas opções de compilação, toda referência deverá ser associada sempre à mesma declaração.

---

## 10.6 Independência da Implementação

Os algoritmos utilizados para percorrer escopos e localizar declarações pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão respeitar a mesma ordem lógica de resolução definida pela linguagem.

---

# 11. Sombreamento (*Shadowing*)

Em determinados contextos, diferentes declarações podem utilizar o mesmo identificador em escopos distintos.

Esse fenômeno, conhecido como *Shadowing*, modifica a declaração visível em determinado contexto sem alterar a existência das demais declarações.

A Resolução de Nomes deve interpretar essas situações de forma determinística e consistente com as regras da linguagem.

---

## 11.1 Objetivo

O tratamento do *Shadowing* possui os seguintes objetivos:

* preservar a organização hierárquica dos escopos;
* permitir reutilização de identificadores quando autorizada pela linguagem;
* evitar ambiguidades na resolução de referências;
* garantir comportamento determinístico.

As regras que determinam quando o Shadowing é permitido ou proibido são definidas pelo Documento 04 — Sintaxe da Linguagem. A Resolução de Nomes limita-se a aplicar essas regras durante a análise semântica.

---

## 11.2 Declarações em Escopos Diferentes

Quando duas declarações utilizarem o mesmo identificador em escopos distintos e a linguagem permitir essa construção, a declaração pertencente ao escopo mais interno deverá possuir prioridade durante a resolução das referências realizadas nesse contexto.

A declaração existente no escopo externo permanece inalterada, podendo continuar visível nos contextos apropriados.

---

## 11.3 Declarações no Mesmo Escopo

Quando múltiplas declarações incompatíveis utilizarem o mesmo identificador dentro de um mesmo escopo e a linguagem não permitir essa situação, a Resolução de Nomes deverá produzir o diagnóstico correspondente.

Nenhuma associação arbitrária deverá ser realizada.

As regras específicas pertencem à definição da linguagem.

---

## 11.4 Prioridade entre Declarações

Sempre que mais de uma declaração puder ser considerada durante a resolução de uma referência, a escolha deverá obedecer exclusivamente às regras de prioridade estabelecidas pela linguagem.

A implementação não poderá introduzir critérios adicionais para decidir entre declarações concorrentes.

---

## 11.5 Preservação da Identidade

O *Shadowing* não altera a identidade das declarações envolvidas.

Cada declaração continua representando uma entidade semântica distinta, ainda que apenas uma delas permaneça visível em determinado ponto do programa.

Essa propriedade preserva a consistência da HIR e das fases posteriores da análise semântica.

---

## 11.6 Independência da Implementação

Os mecanismos utilizados para representar e resolver situações de *Shadowing* pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir resultados funcionalmente equivalentes para programas que utilizem identificadores repetidos em escopos distintos.

---

# 12. Nomes Qualificados e Espaços de Nomes

A linguagem poderá permitir que diferentes declarações sejam organizadas em espaços de nomes distintos ou acessadas por meio de nomes qualificados.

Esses mecanismos ampliam a capacidade de organização dos programas sem alterar os princípios fundamentais da Resolução de Nomes.

As regras específicas são definidas pela linguagem.

---

## 12.1 Objetivo

O suporte a nomes qualificados possui os seguintes objetivos:

* organizar grandes conjuntos de declarações;
* reduzir conflitos entre identificadores;
* permitir acesso explícito a elementos externos;
* preservar a clareza da resolução de nomes.

---

## 12.2 Nomes Qualificados

Uma referência poderá ser composta por múltiplos qualificadores que identificam progressivamente o elemento desejado.

A Resolução de Nomes deverá interpretar cada qualificador conforme as regras definidas pela linguagem, preservando a associação correta entre os elementos envolvidos.

---

## 12.3 Espaços de Nomes

A linguagem poderá definir diferentes espaços de nomes para categorias distintas de declarações.

Quando existirem, esses espaços deverão ser considerados durante a Resolução de Nomes, evitando conflitos entre elementos pertencentes a categorias independentes.

A definição desses espaços pertence à especificação da linguagem.

---

## 12.4 Resolução Determinística

A resolução de nomes qualificados deve produzir resultados determinísticos.

Para uma mesma referência qualificada, utilizando a mesma versão do compilador e as mesmas opções de compilação, deverá sempre ser obtida a mesma declaração correspondente.

---

## 12.5 Relação com a HIR

Após resolvidos, nomes qualificados passam a constituir associações semânticas explícitas na HIR, da mesma forma que referências simples.

As fases posteriores do Frontend operam diretamente sobre essas associações, sem necessidade de repetir o processo de resolução.

---

## 12.6 Independência da Implementação

Os mecanismos utilizados para representar nomes qualificados e espaços de nomes pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar as regras de resolução definidas pela linguagem e produzir resultados funcionalmente equivalentes.

---

# 13. Ambiguidade

Durante a Resolução de Nomes podem surgir situações nas quais uma referência corresponda, aparentemente, a mais de uma declaração.

Sempre que a linguagem não definir uma regra determinística para resolver essa situação, a referência deverá ser considerada ambígua e o compilador deverá produzir o diagnóstico apropriado.

A Resolução de Nomes não poderá escolher arbitrariamente entre múltiplas declarações candidatas.

---

## 13.1 Objetivo

O tratamento de ambiguidades possui os seguintes objetivos:

* preservar o determinismo da compilação;
* evitar interpretações inconsistentes do programa;
* impedir associações arbitrárias entre referências e declarações;
* fornecer diagnósticos claros ao desenvolvedor.

---

## 13.2 Múltiplas Declarações Candidatas

Uma referência poderá tornar-se ambígua quando mais de uma declaração válida puder satisfazer simultaneamente as regras de resolução definidas pela linguagem.

Nesses casos, a implementação deverá identificar todas as declarações candidatas relevantes antes de produzir o diagnóstico correspondente.

Em determinados contextos, mecanismos definidos pela linguagem poderão eliminar a ambiguidade antes da emissão do diagnóstico. A Resolução de Nomes deverá aplicar esses mecanismos conforme as regras da linguagem antes de concluir que uma referência é ambígua.

---

## 13.3 Ambiguidades de Visibilidade

Ambiguidades podem surgir em decorrência das regras de visibilidade estabelecidas pela linguagem.

Entre elas incluem-se, quando aplicável:

* importações conflitantes;
* múltiplos módulos acessíveis;
* diferentes caminhos de qualificação;
* demais situações previstas pela linguagem.

As regras específicas pertencem à definição da linguagem.

---

## 13.4 Diagnóstico

Sempre que uma ambiguidade for identificada, a Resolução de Nomes deverá produzir um diagnóstico suficientemente preciso para permitir sua correção.

Sempre que possível, o diagnóstico deverá identificar:

* a referência ambígua;
* as declarações candidatas;
* a localização correspondente no código-fonte.

Os mecanismos gerais de emissão de diagnósticos são definidos pelo Documento 23.

---

## 13.5 Continuidade da Análise

Sempre que puder ser realizado de forma segura, o compilador deverá continuar a análise após detectar uma ambiguidade.

O objetivo dessa continuidade é permitir que múltiplos problemas sejam identificados em uma única compilação.

As estratégias de recuperação pertencem exclusivamente à implementação.

---

## 13.6 Independência da Implementação

Os mecanismos utilizados para detectar ambiguidades pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão identificar as mesmas situações ambíguas e produzir resultados funcionalmente equivalentes.

---

# 14. Referências Não Resolvidas

Nem toda referência presente em um programa corresponde necessariamente a uma declaração válida.

Quando uma referência não puder ser associada a nenhum elemento declarado conforme as regras da linguagem, a Resolução de Nomes deverá identificá-la como não resolvida e produzir o diagnóstico correspondente.

---

## 14.1 Objetivo

O tratamento de referências não resolvidas possui os seguintes objetivos:

* identificar identificadores inexistentes;
* impedir associações inválidas;
* preservar a consistência da HIR;
* permitir continuidade controlada da análise.

---

## 14.2 Referências Inexistentes

Uma referência será considerada inexistente quando nenhuma declaração compatível puder ser localizada durante o processo de resolução.

Entre elas incluem-se, quando aplicável:

* variáveis inexistentes;
* funções inexistentes;
* tipos inexistentes;
* módulos inexistentes;
* membros inexistentes;
* demais referências definidas pela linguagem.

---

## 14.3 Diagnóstico

Toda referência não resolvida deverá produzir um diagnóstico correspondente.

Sempre que possível, o diagnóstico deverá indicar:

* o identificador utilizado;
* sua localização no código-fonte;
* o contexto em que ocorreu a tentativa de resolução.

Os mecanismos gerais de emissão de diagnósticos são definidos pelo Documento 23.

Sempre que possível, os diagnósticos poderão incluir sugestões destinadas a auxiliar a correção do problema. A geração dessas sugestões pertence exclusivamente à implementação.

---

## 14.4 Continuidade

Após identificar uma referência não resolvida, a implementação deverá continuar a análise sempre que isso puder ser realizado com segurança.

Essa continuidade permite que outros problemas independentes sejam identificados durante a mesma compilação.

Os mecanismos utilizados para essa recuperação pertencem exclusivamente à implementação.

---

## 14.5 Representação na HIR

Quando apropriado, a implementação poderá utilizar mecanismos internos para representar referências ainda não resolvidas durante a continuidade da análise.

Esses mecanismos pertencem exclusivamente à implementação e não fazem parte da interface formal da HIR definida por esta especificação.

---

## 14.6 Independência da Implementação

Os mecanismos utilizados para representar referências não resolvidas pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar comportamento funcionalmente equivalente diante de programas que contenham referências inválidas.

---

# 15. Consistência da HIR

Ao término da Resolução de Nomes, a HIR deverá permanecer em um estado consistente e preparado para utilização pelas fases posteriores da análise semântica.

Essa consistência constitui um dos contratos fundamentais entre a Resolução de Nomes e os demais componentes do Frontend.

---

## 15.1 Objetivo

A consistência da HIR possui os seguintes objetivos:

* preservar a integridade da representação semântica;
* garantir a validade das associações estabelecidas;
* preparar a HIR para a Inferência e Verificação de Tipos;
* manter a continuidade do pipeline do compilador.

---

## 15.2 Estado Inicial

Antes da Resolução de Nomes, a HIR contém referências ainda não associadas às respectivas declarações.

Essas referências representam apenas identificadores presentes no programa, sem vínculos semânticos estabelecidos.

---

## 15.3 Estado Final

Após concluída a Resolução de Nomes, toda referência válida deverá estar associada à declaração correspondente.

Referências inválidas ou ambíguas deverão possuir os diagnósticos apropriados e permanecer representadas de forma consistente para permitir a continuidade da análise, quando aplicável.

---

## 15.4 Preparação para as Etapas Posteriores

A HIR produzida ao final desta fase constitui a entrada para a Inferência e Verificação de Tipos.

As fases posteriores deverão operar diretamente sobre as associações estabelecidas pela Resolução de Nomes, sem necessidade de repetir o processo de localização de identificadores.

---

## 15.5 Contrato Arquitetural

A Resolução de Nomes estabelece um contrato arquitetural com as fases seguintes do compilador.

Esse contrato garante que:

* todas as declarações conhecidas estejam registradas;
* todas as referências válidas estejam resolvidas;
* todas as inconsistências identificadas possuam diagnóstico correspondente;
* a HIR permaneça consistente para continuidade da análise.

---

## 15.6 Independência da Implementação

Os mecanismos utilizados para preservar a consistência da HIR pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir uma HIR semanticamente consistente e funcionalmente equivalente ao término da Resolução de Nomes.

---

# 16. Interface da Resolução de Nomes

A Resolução de Nomes constitui uma etapa intermediária da análise semântica do Frontend.

Sua interface estabelece o contrato entre a HIR inicial produzida pelo Documento 16 e a HIR enriquecida que será utilizada pelas fases de Inferência e Verificação de Tipos.

Este documento define apenas o comportamento esperado dessa interface, não impondo qualquer estratégia específica para sua implementação.

---

## 16.1 Objetivo

A interface da Resolução de Nomes possui os seguintes objetivos:

* receber a HIR produzida pela etapa anterior;
* enriquecer a representação com associações entre referências e declarações;
* preservar a consistência da HIR;
* disponibilizar uma representação preparada para as fases posteriores.

---

## 16.2 Entrada

A entrada da Resolução de Nomes consiste na HIR construída conforme definido no Documento 16.

Essa representação contém:

* elementos semânticos do programa;
* estrutura de escopos;
* informações de rastreabilidade;
* referências ainda não associadas às respectivas declarações.

A implementação poderá utilizar estruturas auxiliares adicionais, desde que não altere os contratos estabelecidos para a HIR.

---

## 16.3 Saída

Ao término da Resolução de Nomes, a saída consiste em uma HIR enriquecida contendo:

* declarações registradas;
* referências resolvidas;
* associações explícitas entre identificadores e declarações;
* informações necessárias para a Inferência e Verificação de Tipos;
* diagnósticos produzidos durante esta etapa, quando aplicável.

---

## 16.4 Relação com Diagnósticos

A Resolução de Nomes poderá identificar situações como:

* referências inexistentes;
* ambiguidades;
* conflitos de declarações;
* demais inconsistências relacionadas à resolução de identificadores.

Os mecanismos gerais para emissão, organização e recuperação de diagnósticos são definidos pelo Documento 23 — Diagnósticos e Recuperação de Erros.

---

## 16.5 Contrato entre Etapas

A Inferência e Verificação de Tipos deve assumir que toda referência válida presente na HIR já foi devidamente associada à sua declaração correspondente.

Da mesma forma, a Resolução de Nomes não deve antecipar verificações que pertençam às fases posteriores da análise semântica.

Essa separação reduz o acoplamento entre os componentes do compilador e preserva o Princípio da Responsabilidade Única.

---

## 16.6 Independência da Implementação

Os mecanismos utilizados para implementar a interface da Resolução de Nomes pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar os contratos definidos neste documento e produzir comportamento funcionalmente equivalente.

---

# 17. Testabilidade

A Resolução de Nomes constitui uma das fases centrais da análise semântica do compilador.

Seu comportamento determinístico permite a construção de testes independentes, garantindo elevado nível de confiabilidade e facilitando a evolução da implementação.

A implementação deve manter uma suíte abrangente de testes automatizados, incluindo testes de regressão para cada erro relacionado à Resolução de Nomes identificado e posteriormente corrigido. Esses testes garantem que problemas anteriormente solucionados não sejam reintroduzidos em versões futuras do compilador.

---

## 17.1 Objetivo

Os testes da Resolução de Nomes possuem os seguintes objetivos:

* validar a construção dos escopos;
* verificar a resolução correta das referências;
* detectar regressões;
* garantir comportamento determinístico;
* assegurar a consistência da HIR produzida.

---

## 17.2 Testes Unitários

Cada componente responsável pela Resolução de Nomes deverá possuir testes específicos.

Entre eles incluem-se, quando aplicável:

* construção de escopos;
* registro de declarações;
* resolução de referências;
* tratamento de ambiguidades;
* referências não resolvidas.

---

## 17.3 Golden Tests

A implementação poderá utilizar *Golden Tests* para validar a HIR enriquecida produzida após a Resolução de Nomes.

Cada execução compara a representação produzida com uma versão previamente validada, permitindo detectar alterações inesperadas no comportamento da análise.

---

## 17.4 Testes de Integração

Além dos testes unitários, deverão existir testes destinados a verificar a integração da Resolução de Nomes com:

* a construção da HIR;
* a Inferência e Verificação de Tipos;
* a Verificação Semântica.

Esses testes asseguram a continuidade arquitetural do Frontend.

---

## 17.5 Benchmarks

A implementação poderá incluir benchmarks destinados a acompanhar o desempenho da Resolução de Nomes.

Entre os aspectos monitorados encontram-se:

* tempo de construção dos escopos;
* tempo de resolução das referências;
* consumo de memória;
* desempenho em projetos de grande porte.

Esses benchmarks auxiliam a evolução da implementação sem alterar os contratos definidos nesta especificação.

---

## 17.6 Determinismo

Todos os testes relacionados à Resolução de Nomes devem pressupor comportamento determinístico.

Para um mesmo programa válido, utilizando a mesma versão do compilador e as mesmas opções de compilação, as associações estabelecidas entre referências e declarações deverão ser funcionalmente equivalentes.

---

# 18. Considerações Finais

A Resolução de Nomes constitui a primeira etapa ativa da análise semântica do compilador da linguagem Capi.

Sua responsabilidade consiste em transformar referências textuais presentes na HIR em associações explícitas com as respectivas declarações, preservando a organização dos escopos e preparando a representação semântica para as fases posteriores do Frontend.

---

## 18.1 Síntese da Arquitetura

A Resolução de Nomes é caracterizada pelos seguintes princípios:

* responsabilidade única;
* resolução determinística;
* organização hierárquica dos escopos;
* preservação da identidade das declarações;
* enriquecimento progressivo da HIR;
* independência da implementação.

Esses princípios garantem previsibilidade, simplicidade e facilidade de evolução da implementação.

---

## 18.2 Princípios Preservados

Toda implementação compatível com esta especificação deverá preservar:

* equivalência funcional entre implementações;
* resolução determinística das referências;
* consistência da HIR;
* separação entre resolução de nomes, inferência de tipos e verificação semântica;
* rastreabilidade entre referências e declarações.

Esses princípios asseguram a interoperabilidade entre diferentes implementações do compilador.

---

## 18.3 Relação com o Documento Seguinte

Este documento define a primeira etapa da análise semântica responsável por enriquecer a HIR com associações entre referências e declarações.

O documento seguinte, **18 — Inferência e Verificação de Tipos**, especifica a fase responsável por determinar os tipos dos elementos do programa e verificar sua consistência, utilizando as associações produzidas pela Resolução de Nomes.

Ao término dessa etapa, a HIR continuará sendo enriquecida pela **Verificação Semântica (Documento 19)**, antes de servir como entrada para o processo de **Lowering para IR (Documento 20)**.

---

## 18.4 Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência da Resolução de Nomes e Escopos do compilador oficial da linguagem Capi.

Outras implementações poderão adotar estratégias diferentes para organizar escopos, registrar declarações e resolver referências, desde que preservem integralmente:

* os contratos definidos neste documento;
* o comportamento determinístico da resolução;
* a consistência da HIR;
* a separação entre as fases da análise semântica;
* a compatibilidade com as etapas de Inferência e Verificação de Tipos, Verificação Semântica e Lowering.

Dessa forma, este documento conclui a especificação da Resolução de Nomes e Escopos e estabelece a base para a fase de **Inferência e Verificação de Tipos**, responsável por enriquecer a HIR com as informações necessárias para a validação completa do programa.

