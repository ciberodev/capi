# Capi — A Próxima Geração da Orientação a Objetos

# Documento 03 — Modelo de Memória

Versão: 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define o **Modelo de Memória** da linguagem Capi.

Seu objetivo é estabelecer como os conceitos definidos nos documentos anteriores são representados fisicamente durante a execução de um programa.

Enquanto o Documento 01 apresenta a arquitetura conceitual da linguagem e o Documento 02 define seu Sistema de Tipos, este documento descreve como objetos, Domains e identidades são organizados na memória.

O Modelo de Memória constitui a base para:

* gerenciamento determinístico de memória;
* representação física dos objetos;
* suporte à herança de implementação;
* despacho dinâmico;
* concorrência segura;
* otimizações realizadas pelo compilador.

---

## 1.2 Escopo

Este documento define:

* organização física dos Domains;
* organização física dos objetos;
* layout de memória;
* alocação;
* desalocação;
* representação física da identidade dos objetos;
* organização dos grafos de objetos.

Não fazem parte deste documento:

* sintaxe da linguagem;
* regras de tipagem;
* gramática;
* semântica operacional;
* arquitetura interna do compilador.

Esses assuntos são tratados em documentos específicos da especificação.

---

## 1.3 Relação com os Documentos Anteriores

Este documento complementa:

* **Documento 00 — Apresentação**, que apresenta a visão geral da linguagem;
* **Documento 01 — Visão Geral e Fundamentos**, que estabelece os princípios arquiteturais;
* **Documento 02 — Sistema de Tipos**, que define as entidades e garantias formais da linguagem.

Todas as decisões apresentadas neste documento devem preservar integralmente os princípios estabelecidos pelos documentos anteriores.

---

# 2. Objetivos do Modelo de Memória

O Modelo de Memória da Capi foi concebido para atingir simultaneamente objetivos que, nas linguagens tradicionais, costumam ser conflitantes.

---

## 2.1 Gerenciamento Determinístico

Toda memória deve possuir um ciclo de vida completamente previsível.

A criação e destruição da memória não dependem de:

* Garbage Collector;
* Reference Counting;
* análise de ciclos;
* heurísticas de execução.

Toda memória pertence a um Domain e seu lifetime é determinado exclusivamente pelo lifetime desse Domain.

---

## 2.2 Segurança

O Modelo de Memória deve permitir que o compilador garanta, em conjunto com o Sistema de Tipos:

* ausência de *Use After Free*;
* ausência de *Dangling References*;
* ausência de *Double Free*;
* ausência de corrupção de memória;
* ausência de *Data Races* decorrentes de acessos incompatíveis ao mesmo Domain.

Essas garantias devem ser obtidas preferencialmente em tempo de compilação.

---

## 2.3 Preservação da Orientação a Objetos

O Modelo de Memória não altera os princípios clássicos da Orientação a Objetos.

A linguagem continua oferecendo:

* classes;
* herança de implementação;
* subtipagem;
* despacho dinâmico;
* identidade compartilhada;
* estado mutável.

A inovação da Capi consiste exclusivamente na forma como esses conceitos são representados fisicamente.

---

## 2.4 Separação entre Modelo Lógico e Modelo Físico

A arquitetura da Capi distingue explicitamente duas visões complementares.

### Modelo Lógico

Descreve aquilo que o programador manipula.

Exemplos:

* Classes;
* Objetos;
* ObjectIds;
* Interfaces;
* Traits.

---

### Modelo Físico

Descreve aquilo que realmente existe na memória durante a execução.

Exemplos:

* Domains;
* blocos de memória;
* layouts físicos;
* metadados de execução;
* arenas de alocação.

O programador interage predominantemente com o Modelo Lógico.

O compilador é responsável por traduzir esse modelo para sua representação física.

---

## 2.5 Custo Previsível

Todas as operações relacionadas ao gerenciamento de memória devem possuir custo previsível.

O Modelo de Memória deve evitar mecanismos cuja complexidade dependa da estrutura dos grafos de objetos, como:

* coleta de lixo;
* contagem de referências;
* detecção de ciclos.

Sempre que possível, o custo das operações deve depender apenas do Domain envolvido.

---

# 3. Princípios Fundamentais

O Modelo de Memória da Capi baseia-se nos seguintes princípios.

---

## Princípio 1 — Toda memória pertence a um Domain

Nenhum objeto administra sua própria memória.

Toda região de memória utilizada por objetos pertence exatamente a um Domain.

Consequentemente:

* objetos nunca desalocam outros objetos;
* objetos nunca controlam seu próprio lifetime;
* o gerenciamento físico da memória permanece centralizado.

---

## Princípio 2 — Objetos são entidades lógicas

Objetos representam entidades do domínio da aplicação.

Fisicamente, seu estado é armazenado dentro do Domain ao qual pertencem.

A separação entre objeto e memória permite que identidade, comportamento e armazenamento evoluam de forma independente.

---

## Princípio 3 — ObjectId não representa memória

O `ObjectId` representa exclusivamente a identidade lógica de um objeto.

Ele não representa:

* endereço físico;
* ponteiro;
* ownership;
* autorização para escrita.

Sua representação física será definida posteriormente neste documento, mas sua semântica permanece independente da forma de implementação.

---

## Princípio 4 — O layout físico é responsabilidade do Domain

Classes definem apenas a estrutura lógica dos objetos.

A organização física dessa estrutura pertence ao Domain.

Essa separação permite que o compilador organize a memória da forma mais eficiente possível, preservando as garantias da linguagem.

---

## Princípio 5 — A destruição ocorre por Domain

Objetos não são destruídos individualmente por mecanismos automáticos de gerenciamento de memória.

Quando um Domain deixa de existir:

* todos os objetos nele contidos deixam de existir;
* toda memória é liberada deterministicamente;
* grafos arbitrários deixam de representar um problema de gerenciamento.

Essa característica elimina a necessidade de algoritmos de coleta de ciclos.

---

## Princípio 6 — Grafos são independentes da memória

Os relacionamentos entre objetos pertencem exclusivamente ao Modelo Lógico.

O formato desses grafos não influencia:

* alocação;
* desalocação;
* lifetime;
* gerenciamento físico da memória.

Consequentemente, grafos cíclicos e altamente conectados possuem o mesmo modelo de gerenciamento que grafos simples.

---

## Princípio 7 — O Modelo de Memória preserva a arquitetura da linguagem

O Modelo de Memória existe para materializar fisicamente os princípios definidos nos Documentos 01 e 02.

Ele não modifica:

* o Sistema de Tipos;
* o modelo de objetos;
* a semântica da linguagem.

Sua responsabilidade é exclusivamente fornecer uma representação física eficiente e determinística para essas abstrações.

---

## Princípio 8 — Garantias são independentes da implementação

Assim como estabelecido no Documento 02 — Sistema de Tipos (Seção 19 — Princípio da Separação entre Garantias e Implementação), este documento distingue explicitamente entre:

* as garantias fornecidas pelo Modelo de Memória;
* os mecanismos utilizados para implementá-las.

As garantias fazem parte permanente da especificação.

A implementação poderá evoluir ao longo do tempo, desde que preserve integralmente todas essas garantias.

---

# 4. Organização Física dos Domains

O **Domain** constitui a unidade fundamental de gerenciamento físico de memória da Capi.

Toda memória pertencente aos objetos é administrada por exatamente um Domain.

O Domain é responsável por organizar, proteger e liberar toda a memória sob sua responsabilidade.

---

## 4.1 Responsabilidades

Fisicamente, um Domain é responsável por:

* alocação de memória;
* desalocação determinística;
* organização dos objetos;
* controle do lifetime;
* sincronização das operações de escrita;
* metadados necessários à execução.

O Domain **não** representa um objeto da aplicação.

Ele representa uma estrutura interna da infraestrutura de execução da linguagem.

---

## 4.2 Estrutura Conceitual

Conceitualmente, um Domain pode ser representado da seguinte forma:

```text
+------------------------------------------------------+
| Domain                                               |
|------------------------------------------------------|
| Metadados                                            |
|------------------------------------------------------|
| Controle de Lifetime                                 |
|------------------------------------------------------|
| Estruturas de Gerenciamento                          |
|------------------------------------------------------|
| Área de Objetos                                      |
+------------------------------------------------------+
```

A organização interna apresentada acima é ilustrativa.

A implementação poderá utilizar estruturas diferentes, desde que preserve todas as garantias estabelecidas nesta especificação.

---

## 4.3 Área de Objetos

Todo objeto pertence fisicamente à Área de Objetos de um Domain.

Essa área contém exclusivamente o estado persistente dos objetos.

Ela não contém:

* pilha de execução;
* variáveis locais;
* registradores;
* estruturas temporárias do compilador.

Esses elementos pertencem ao ambiente de execução e não ao Modelo de Memória.

---

## 4.4 Lifetime

O lifetime de todos os objetos pertencentes a um Domain é exatamente o lifetime do próprio Domain.

Consequentemente:

* objetos não possuem lifetime independente;
* não existe destruição automática de objetos individuais por mecanismos de gerenciamento de memória;
* a destruição do Domain encerra simultaneamente o lifetime de todos os objetos nele contidos.

Essa propriedade constitui um dos pilares da previsibilidade da linguagem.

---

## 4.5 Isolamento

Cada Domain representa uma unidade isolada de gerenciamento.

O estado interno de um Domain não pode ser modificado diretamente por outro Domain.

Toda interação entre Domains ocorre através de mecanismos definidos pelo Sistema de Tipos e pela Semântica Operacional.

Essa separação permite que diferentes Domains evoluam independentemente, preservando segurança e concorrência.

---

# 5. Organização Física dos Objetos

Enquanto o Domain representa a unidade física de gerenciamento de memória, os objetos representam as entidades persistentes armazenadas nesse espaço.

A representação física de um objeto deve preservar integralmente sua identidade lógica, sua estrutura e seu comportamento.

---

## 5.1 Modelo Conceitual

Cada objeto pode ser compreendido como a união de três componentes distintos.

```text
                Objeto

        +----------------------+
        |      Identidade      |
        +----------------------+
                   │
                   │
        +----------------------+
        |   Estado Persistente |
        +----------------------+
                   │
                   │
        +----------------------+
        |    Comportamento     |
        +----------------------+
```

Cada componente possui responsabilidade distinta.

* A identidade distingue o objeto dos demais.
* O estado representa seus dados persistentes.
* O comportamento é definido pela Classe.

---

## 5.2 Estado Persistente

O estado persistente corresponde ao conjunto de atributos declarados pela Classe.

Fisicamente, esses atributos são armazenados dentro do Domain.

O objeto não controla:

* onde esse estado está localizado;
* como esse estado é organizado;
* quando esse estado será destruído.

Essas responsabilidades pertencem exclusivamente ao Domain.

---

## 5.3 Comportamento

O comportamento não faz parte do estado persistente.

Métodos representam operações executadas sobre o estado do objeto.

O Modelo de Memória não impõe qualquer representação específica para métodos.

A forma como o comportamento é associado aos objetos é definida pela Arquitetura do Compilador.

---

## 5.4 Organização Física

Conceitualmente, um objeto armazenado em um Domain pode ser representado da seguinte forma.

```text
+-------------------------------------+
| Informações de Execução             |
+-------------------------------------+
| Estado Persistente                  |
+-------------------------------------+
```

As Informações de Execução podem conter elementos necessários para a implementação da linguagem, tais como:

* identificação do tipo concreto;
* informações para despacho dinâmico;
* metadados internos.

A presença, formato e localização dessas informações não constituem parte da especificação da linguagem. Elas são detalhes de implementação do compilador e da infraestrutura de execução, desde que não alterem a semântica observável do programa.

A única exigência da especificação é que sua existência não altere a semântica da linguagem.

---

## 5.5 Organização Física do Estado

O estado persistente é armazenado de forma contígua sempre que possível.

Por exemplo:

```text
Classe Conta

saldo
limite
agência
cliente
```

Pode ser representada fisicamente como:

```text
+------------------+
| saldo            |
| limite           |
| agência          |
| cliente          |
+------------------+
```

A ordem física poderá diferir da ordem declarada na Classe caso isso produza melhor alinhamento ou desempenho.

Entretanto, essa reorganização não pode alterar o comportamento observável do programa.

---

# 6. Layout de Herança

Um dos objetivos fundamentais da Capi é preservar a herança de implementação sem comprometer previsibilidade ou eficiência.

Para isso, o Modelo de Memória adota o princípio de **layout prefixado**.

---

## 6.1 Layout Prefixado

Em uma hierarquia de herança, a representação física da superclasse ocupa sempre o início da representação física da subclasse.

Exemplo:

```text
Classe Animal

idade
peso
```

```text
+------------------+
| idade            |
| peso             |
+------------------+
```

Classe derivada:

```text
Classe Cachorro extends Animal

raça
```

Representação física:

```text
+------------------+
| idade            |
| peso             |
| raça             |
+------------------+
```

A porção correspondente à superclasse permanece inalterada.

A existência de um layout compatível entre superclasses e subclasses constitui uma garantia da linguagem. O layout prefixado é o modelo arquitetural adotado pela especificação para expressar essa compatibilidade.

---

## 6.2 Upcast

Graças ao layout prefixado, um objeto derivado pode ser tratado como sua superclasse sem necessidade de cópia ou reorganização da memória.

Conceitualmente:

```text
ObjectId<Cachorro>

↓

ObjectId<Animal>
```

Essa operação:

* preserva a identidade;
* preserva o Domain;
* preserva o estado;
* possui custo constante.

---

## 6.3 Herança Múltipla

A Capi não suporta herança múltipla de implementação.

Essa decisão reduz significativamente a complexidade do layout físico, do despacho dinâmico e da verificação de tipos.

Composição de comportamento deve ser realizada por Interfaces e Traits, conforme definido no Documento 02.

---

## 6.4 Independência da Implementação

O conceito de layout prefixado constitui uma garantia arquitetural da linguagem.

Entretanto, a forma como o compilador implementa essa garantia permanece um detalhe de implementação.

Compiladores poderão utilizar diferentes estratégias internas, desde que preservem as seguintes propriedades:

* compatibilidade entre superclasses e subclasses;
* custo constante para upcast;
* preservação da identidade;
* preservação do comportamento observável.

Assim, a especificação estabelece **o resultado esperado**, e não a técnica obrigatória utilizada para alcançá-lo.

---

# 7. Representação Física do ObjectId

O `ObjectId` constitui o mecanismo utilizado pela linguagem para representar a identidade lógica de um objeto.

Conforme definido no Documento 02, um `ObjectId` **não representa**:

* um endereço de memória;
* um ponteiro;
* ownership;
* autoridade para escrita.

Seu único propósito é identificar unicamente um objeto durante seu lifetime.

O programador nunca manipula diretamente a representação física de um ObjectId. Toda interação ocorre através do Sistema de Tipos.

---

## 7.1 Modelo Conceitual

A relação entre identidade, objeto e memória pode ser representada da seguinte forma.

```text
          ObjectId
              │
              ▼
      +----------------+
      |   Identidade   |
      +----------------+
              │
              ▼
      +----------------+
      |     Objeto     |
      +----------------+
              │
              ▼
      +----------------+
      |     Domain     |
      +----------------+
              │
              ▼
      +----------------+
      | Estado Físico  |
      +----------------+
```

A identidade pertence ao objeto.

O `ObjectId` apenas representa essa identidade no Sistema de Tipos.

---

## 7.2 Independência da Representação

A especificação da linguagem não impõe uma representação física específica para um `ObjectId`.

Uma implementação poderá representá-lo, por exemplo, como:

* um índice interno;
* um identificador opaco;
* uma estrutura composta;
* outra representação equivalente.

Independentemente da representação adotada, o compilador deverá preservar as garantias estabelecidas pelo Sistema de Tipos.

---

## 7.3 Associação ao Domain

Todo `ObjectId` está associado exatamente a um Domain.

Essa associação constitui uma propriedade arquitetural da linguagem.

Entretanto, a forma como essa associação é armazenada ou recuperada faz parte da implementação do compilador e da infraestrutura de execução.

---

## 7.4 Validade

Um `ObjectId` somente é válido enquanto o Domain ao qual seu objeto pertence permanecer válido.

Após a destruição do Domain:

* todos os `ObjectId` associados tornam-se inválidos;
* nenhuma operação sobre esses identificadores pode produzir comportamento indefinido;
* o compilador e a infraestrutura de execução devem impedir seu uso conforme as garantias da linguagem.

---

## 7.5 Estabilidade da Identidade

Durante todo o lifetime de um objeto:

* seu `ObjectId` permanece inalterado;
* sua identidade lógica permanece constante;
* alterações no estado do objeto não modificam sua identidade.

Essa estabilidade é essencial para permitir grafos arbitrários de objetos sem comprometer a segurança da linguagem.

---

# 8. Alocação

A criação de objetos na Capi ocorre sempre dentro de um Domain.

Não existe criação de objetos fora de um Domain válido.

---

## 8.1 Unidade de Alocação

A unidade fundamental de alocação é o Domain.

Objetos individuais utilizam a infraestrutura de alocação pertencente ao Domain onde serão armazenados.

Consequentemente:

* a criação de um objeto não implica a criação de um novo gerenciador de memória;
* todos os objetos de um mesmo Domain compartilham a mesma infraestrutura física.

---

## 8.2 Processo Conceitual de Alocação

Conceitualmente, a criação de um objeto segue as etapas abaixo.

```text
Solicitação de criação
           │
           ▼
 Localização do Domain
           │
           ▼
 Reserva de espaço
           │
           ▼
 Inicialização do estado
           │
           ▼
 Associação da identidade
           │
           ▼
 Objeto disponível
```

As estruturas internas utilizadas para executar esse processo pertencem à implementação da linguagem.

---

## 8.3 Organização da Memória

O Modelo de Memória não exige uma estratégia específica de alocação.

Uma implementação poderá utilizar, por exemplo:

* arenas;
* regiões;
* pools;
* alocadores especializados.

Independentemente da estratégia adotada, a implementação deverá preservar:

* previsibilidade;
* determinismo;
* estabilidade das identidades;
* garantias do Sistema de Tipos.

---

## 8.4 Reutilização de Espaço

Após a destruição de objetos ou do próprio Domain, regiões de memória poderão ser reutilizadas.

Entretanto, essa reutilização nunca poderá permitir que um `ObjectId` válido passe a representar outro objeto diferente.

A linguagem garante que a identidade lógica de um objeto jamais será reutilizada durante seu período de validade.

---

## 8.5 Alocação e Herança

A criação de objetos pertencentes a classes derivadas deve respeitar o layout físico definido para sua hierarquia.

O compilador deverá garantir que:

* a porção correspondente à superclasse permaneça íntegra;
* os campos da subclasse sejam posicionados conforme o layout estabelecido;
* operações de upcast permaneçam de custo constante.

---

# 9. Destruição

A destruição da memória na Capi é determinada exclusivamente pelo lifetime do Domain.

Objetos individuais não controlam sua própria destruição.

---

## 9.1 Destruição Determinística

Quando um Domain deixa de existir:

* todos os objetos nele contidos deixam de existir;
* toda a memória pertencente ao Domain é liberada;
* todas as identidades associadas tornam-se inválidas.

Essa operação ocorre de forma determinística.

Não depende de:

* Garbage Collector;
* contagem de referências;
* análise de ciclos;
* execução periódica de tarefas de limpeza.

---

## 9.2 Ordem de Destruição

A especificação exige apenas que a destruição preserve a consistência da linguagem.

A ordem utilizada para destruir objetos internos ao Domain é um detalhe de implementação, desde que:

* nenhum objeto seja acessado após sua destruição;
* todas as estruturas pertencentes ao Domain sejam corretamente finalizadas;
* os contratos definidos pela linguagem sejam preservados.

---

## 9.3 Destrutores

Classes poderão definir operações de finalização associadas ao encerramento de seu lifetime.

Essas operações serão executadas durante o processo de destruição do Domain.

As regras completas para destrutores, incluindo ordem de execução, tratamento de falhas e interação com herança, serão definidas na Semântica Operacional da linguagem.

Nota: Destrutores serão definidos formalmente no Documento 05 — Semântica Operacional. O Sistema de Tipos (Documento 02) não impõe restrições à sua existência, mas sua execução deve respeitar todas as garantias de memória estabelecidas neste documento.

---

## 9.4 Independência dos Grafos

O processo de destruição é completamente independente da estrutura do grafo de objetos.

Por exemplo:

```text
A → B → C → A
```

e

```text
A → B → C
```

são destruídos exatamente da mesma forma.

A existência de ciclos não altera:

* o algoritmo de destruição;
* o custo da destruição;
* o lifetime dos objetos.

Essa propriedade elimina uma das principais fontes de complexidade presentes em linguagens baseadas em Garbage Collector ou Reference Counting.

---

## 9.5 Garantias

O Modelo de Memória garante que a destruição de um Domain:

* libera integralmente toda a memória sob sua responsabilidade;
* preserva a consistência do Sistema de Tipos;
* não depende da topologia do grafo de objetos;
* possui comportamento determinístico.

Essas garantias constituem parte permanente da especificação da Capi e independem da estratégia utilizada pela implementação para realizar a destruição física da memória.

---

# 10. Grafos de Objetos

Uma das principais características da arquitetura da Capi é a separação entre **estrutura lógica** e **organização física da memória**.

Enquanto a memória é organizada pelo Domain, os relacionamentos entre objetos formam grafos independentes dessa organização.

Essa separação elimina diversas restrições presentes em modelos baseados em ownership.

---

## 10.1 Modelo Lógico

Objetos relacionam-se exclusivamente através de `ObjectId`.

Exemplo:

```text
Cliente
   │
   ├──────────────┐
   ▼              ▼
Conta A        Conta B
   │              │
   └──────┬───────┘
          ▼
      Agência
```

Nenhum desses relacionamentos representa ownership.

Todos representam apenas referências lógicas entre identidades.

---

## 10.2 Independência Física

A disposição física dos objetos na memória é completamente independente da estrutura do grafo.

Por exemplo, o grafo anterior pode estar organizado fisicamente da seguinte forma:

```text
Domain

+--------------------------------+
| Agência                        |
| Cliente                        |
| Conta B                        |
| Conta A                        |
+--------------------------------+
```

Ou:

```text
Domain

+--------------------------------+
| Conta A                        |
| Agência                        |
| Cliente                        |
| Conta B                        |
+--------------------------------+
```

A organização física da memória não altera a semântica do programa.

---

## 10.3 Compartilhamento

Um mesmo objeto pode participar simultaneamente de múltiplos grafos.

Exemplo:

```text
Pedido A
      │
      ▼
   Produto
      ▲
      │
Pedido B
```

Nesse exemplo:

* existe apenas um objeto `Produto`;
* ambos os pedidos compartilham sua identidade;
* não existe duplicação do estado.

O compartilhamento é uma consequência natural da separação entre identidade e memória.

---

## 10.4 Independência do Lifetime

O lifetime dos objetos não depende do número de referências existentes para eles.

Considere:

```text
A → B
```

ou

```text
A → B
↑   │
└───┘
```

Em ambos os casos:

* o lifetime permanece determinado exclusivamente pelo Domain;
* o número de referências não influencia a destruição;
* não existe contagem de referências.

---

## 10.5 Complexidade

As operações de gerenciamento de memória não dependem da forma do grafo.

Consequentemente:

* grafos lineares;
* árvores;
* DAGs;
* grafos arbitrários;
* grafos altamente conectados;

possuem exatamente o mesmo modelo de gerenciamento de memória.

Essa propriedade representa uma diferença fundamental em relação a modelos baseados em ownership ou coleta de lixo.

---

# 11. Ciclos

Na Capi, ciclos são tratados como uma propriedade natural dos grafos de objetos.

Eles não representam uma condição excepcional nem exigem tratamento especial pelo Modelo de Memória.

---

## 11.1 Definição

Existe um ciclo quando um caminho de referências retorna ao objeto de origem.

Exemplo simples:

```text
A
│
▼
B
│
▼
C
│
▼
A
```

Todos os relacionamentos acima são representados por `ObjectId`.

Nenhum deles representa posse da memória.

---

## 11.2 Ciclos e Lifetime

Como o lifetime pertence exclusivamente ao Domain, ciclos não alteram o tempo de vida dos objetos.

Por exemplo:

```text
Pessoa
   │
   ▼
Endereço
   ▲
   │
Cidade
```

Mesmo existindo referências em ambas as direções, a destruição continua determinada exclusivamente pelo Domain.

---

## 11.3 Ausência de Detecção de Ciclos

O Modelo de Memória não realiza:

* análise de ciclos;
* quebra automática de referências;
* coleta de objetos inacessíveis.

Esses mecanismos tornam-se desnecessários porque a existência dos objetos não depende da topologia do grafo.

---

## 11.4 Custo Constante

A presença de ciclos não altera:

* o custo de alocação;
* o custo de destruição;
* o custo de gerenciamento da memória.

As operações continuam dependendo apenas do Domain envolvido.

---

## 11.5 Consequências Arquiteturais

A eliminação da dependência entre lifetime e estrutura do grafo produz diversos benefícios:

* compartilhamento irrestrito de objetos;
* ausência de ciclos de ownership;
* eliminação de algoritmos de coleta de ciclos;
* gerenciamento determinístico da memória.

Essa propriedade constitui uma das principais diferenças arquiteturais entre a Capi e linguagens baseadas em ownership.

---

# 12. Migração entre Domains

Domains representam unidades independentes de gerenciamento de memória.

Consequentemente, a movimentação de objetos entre Domains requer regras específicas para preservar as garantias da linguagem.

---

## 12.1 Princípio Fundamental

Todo objeto pertence exatamente a um Domain durante sua existência.

Essa associação é parte integrante de sua identidade lógica e de seu lifetime.

Nenhuma operação da linguagem pode alterar implicitamente essa associação.

---

## 12.2 Migração Implícita

A linguagem não realiza migração automática de objetos entre Domains.

Operações aparentemente simples, como atribuições ou passagem de parâmetros, nunca alteram o Domain ao qual um objeto pertence.

Essa restrição elimina ambiguidades relacionadas ao lifetime e à identidade.

---

## 12.3 Migração Explícita

A linguagem poderá oferecer mecanismos explícitos para transferência de dados entre Domains.

Entretanto, tais mecanismos não representam movimentação física do mesmo objeto.

Conceitualmente, envolvem:

* criação de uma nova instância;
* cópia ou reconstrução do estado;
* geração de uma nova identidade;
* associação ao novo Domain.

O objeto original permanece inalterado em seu Domain de origem.

Conforme estabelecido no Documento 02 (Seção 17.5), a migração entre Domains constitui uma operação especial da linguagem e será definida formalmente em documento específico. Este documento estabelece apenas as garantias que qualquer mecanismo de migração deverá preservar.

---

## 12.4 Preservação da Identidade

A identidade de um objeto está permanentemente associada ao Domain onde ele foi criado.

Caso um objeto seja reconstruído em outro Domain:

* um novo `ObjectId` será criado;
* uma nova identidade será estabelecida;
* o objeto original continuará existindo até que seu Domain seja destruído ou outra operação definida pela linguagem determine seu encerramento.

Assim, identidade e Domain permanecem inseparáveis durante todo o lifetime do objeto.

---

## 12.5 Garantias

Independentemente da estratégia utilizada para transferência de dados entre Domains, a linguagem deve preservar as seguintes propriedades:

* nenhum objeto pertence simultaneamente a dois Domains;
* nenhuma identidade pode migrar entre Domains;
* nenhum `ObjectId` muda de Domain durante seu lifetime;
* todas as garantias do Sistema de Tipos permanecem válidas.

Essas propriedades são permanentes e independem da implementação adotada pelo compilador ou pela infraestrutura de execução.

---

# 13. Concorrência

A arquitetura da Capi foi concebida para permitir concorrência segura sem recorrer, como mecanismo principal, a Garbage Collector ou Reference Counting.

O Modelo de Memória estabelece o isolamento entre Domains como fundamento para a execução concorrente.

As regras formais de sincronização e autorização de escrita são definidas pela Semântica Operacional e pelo Sistema de Tipos.

---

## 13.1 Domains como Unidades de Isolamento

Cada Domain representa uma unidade independente de gerenciamento de memória.

Consequentemente, Domains distintos podem evoluir independentemente durante a execução de um programa.

Conceitualmente:

```text
Thread A
    │
    ▼
 Domain A
```

```text
Thread B
    │
    ▼
 Domain B
```

Como não existe compartilhamento físico de memória entre os Domains, ambos podem ser executados simultaneamente sem interferência.

---

## 13.2 Compartilhamento

Objetos pertencentes ao mesmo Domain compartilham o mesmo espaço físico de memória.

Objetos pertencentes a Domains diferentes nunca compartilham memória diretamente.

Toda comunicação entre Domains ocorre através de mecanismos definidos pela linguagem.

Esses mecanismos preservam:

* segurança de memória;
* isolamento;
* determinismo.

---

## 13.3 Escrita Concorrente

O Modelo de Memória estabelece que operações de escrita pertencem exclusivamente ao Domain e somente podem ocorrer quando existir uma Capacidade de Mutação válida para esse Domain, conforme definido no Documento 01 (Princípio 3) e no Documento 02 (Seção 15).

Consequentemente, operações incompatíveis de escrita sobre um mesmo Domain não podem ocorrer simultaneamente.

A linguagem garante essa propriedade.

O mecanismo utilizado para implementá-la pertence à Semântica Operacional.

---

## 13.4 Leitura Concorrente

Operações de leitura não alteram o estado persistente dos objetos.

Sempre que permitido pelo Sistema de Tipos, múltiplas leituras podem ocorrer simultaneamente.

Essa propriedade permite elevado grau de paralelismo para operações puramente consultivas.

---

## 13.5 Data Races

O Modelo de Memória da Capi foi concebido para impedir *data races*.

Uma implementação da linguagem deverá garantir que duas operações incompatíveis sobre a mesma região de memória — especialmente leituras e escritas concorrentes envolvendo um mesmo Domain — não possam produzir comportamento indefinido. Essa situação é tradicionalmente conhecida como data race.

Essa garantia é permanente e independe da estratégia utilizada pelo compilador.

---

## 13.6 Independência entre Concorrência e Modelo de Objetos

A existência de múltiplas threads não altera:

* a identidade dos objetos;
* o lifetime dos Domains;
* a organização dos grafos;
* o layout físico da memória.

A concorrência constitui uma propriedade da execução do programa, não da estrutura dos objetos.

---

# 14. Otimizações

Conforme estabelecido no Documento 02 (Seção 19), este capítulo descreve apenas estratégias possíveis de implementação. As garantias da linguagem permanecem invariáveis, independentemente das otimizações empregadas pelo compilador.

O Modelo de Memória da Capi foi concebido para permitir otimizações agressivas sem comprometer as garantias da linguagem.

As otimizações descritas neste capítulo são exemplos de estratégias possíveis.

A linguagem exige apenas que as propriedades observáveis do programa permaneçam inalteradas.

---

## 14.1 Reorganização Física

O compilador poderá reorganizar fisicamente a memória dos objetos para melhorar:

* alinhamento;
* localidade de referência;
* utilização de cache;
* desempenho geral.

Essa reorganização não pode alterar:

* identidade;
* comportamento;
* semântica observável.

---

## 14.2 Compactação Interna

A implementação poderá utilizar estratégias que reduzam fragmentação interna do Domain.

Essa compactação deve preservar integralmente:

* os `ObjectId`;
* o comportamento do programa;
* as garantias do Sistema de Tipos.

---

## 14.3 Otimizações de Alocação

O compilador poderá utilizar diferentes estratégias de alocação, incluindo:

* arenas;
* pools especializados;
* regiões;
* alocadores dedicados.

A escolha da estratégia pertence exclusivamente à implementação.

---

## 14.4 Eliminação de Estruturas Temporárias

Sempre que possível, o compilador poderá eliminar estruturas intermediárias criadas durante a execução.

Exemplos:

* objetos temporários;
* estruturas auxiliares;
* cópias desnecessárias.

Desde que essa eliminação preserve integralmente a semântica do programa.

---

## 14.5 Independência da Especificação

Nenhuma otimização descrita neste documento constitui parte obrigatória da linguagem.

A especificação define apenas as garantias que devem ser preservadas.

A implementação permanece livre para adotar técnicas adicionais de otimização.

---

# 15. Garantias do Modelo de Memória

O Modelo de Memória da Capi estabelece um conjunto de garantias permanentes.

Essas garantias constituem parte integrante da especificação da linguagem.

Toda implementação deverá preservá-las.

---

## 15.1 Garantias de Organização

O Modelo de Memória garante que:

* todo objeto pertence exatamente a um Domain;
* todo Domain administra exclusivamente sua própria memória;
* objetos nunca administram memória diretamente;
* a organização física permanece transparente ao programador.

---

## 15.2 Garantias de Lifetime

A linguagem garante que:

* o lifetime dos objetos é determinado exclusivamente pelo Domain;
* objetos não sobrevivem ao Domain ao qual pertencem;
* a destruição ocorre de forma determinística;
* o lifetime não depende da estrutura dos grafos.

---

## 15.3 Garantias de Identidade

A linguagem garante que:

* cada objeto possui exatamente uma identidade;
* essa identidade permanece constante durante todo seu lifetime;
* um `ObjectId` representa sempre o mesmo objeto enquanto permanecer válido;
* identidades não migram entre Domains.

---

## 15.4 Garantias de Memória

A linguagem garante:

* ausência de Garbage Collector;
* ausência de Reference Counting como mecanismo principal;
* ausência de detecção de ciclos para gerenciamento da memória;
* ausência de ownership entre objetos.

Essas propriedades decorrem diretamente da arquitetura baseada em Domains.

---

## 15.5 Garantias de Concorrência

O Modelo de Memória garante que:

* Domains constituem unidades independentes de isolamento;
* operações incompatíveis de escrita sobre um mesmo Domain não produzem comportamento indefinido;
* o Modelo de Memória é compatível com concorrência segura.

As regras formais que concretizam essas garantias pertencem à Semântica Operacional da linguagem.

---

## 15.6 Garantias Arquiteturais

O Modelo de Memória preserva integralmente os princípios definidos nos Documentos 00, 01 e 02.

Consequentemente, toda implementação da Capi deverá respeitar permanentemente:

* a separação entre identidade, estado e memória;
* a separação entre garantias da linguagem e mecanismos de implementação;
* a independência entre grafos de objetos e gerenciamento físico da memória;
* a previsibilidade do gerenciamento de memória.

Essas garantias constituem o núcleo arquitetural da Capi e não podem ser modificadas por decisões de implementação.

---

# 16. Relação com os Próximos Documentos

O Modelo de Memória estabelece a representação física dos conceitos fundamentais da linguagem Capi.

Os documentos subsequentes utilizam esse modelo como base para definir o comportamento completo da linguagem.

Cada documento especializa um aspecto específico da arquitetura, preservando integralmente as garantias estabelecidas até este ponto.

---

## 16.1 Documento 04 — Sintaxe da Linguagem

O Documento 04 define como os conceitos apresentados até aqui são expressos pelo programador.

Entre os temas abordados estão:

* declaração de classes;
* declaração de Interfaces e Traits;
* criação de objetos;
* declaração de Domains;
* estruturas de controle;
* funções;
* módulos;
* namespaces;
* generics;
* visibilidade.

O Documento 04 não altera o Modelo de Memória.

Ele apenas fornece uma representação sintática para seus conceitos.

---

## 16.2 Documento 05 — Semântica Operacional

O Documento 05 define o comportamento dinâmico da linguagem.

Entre os assuntos tratados estão:

* criação de objetos;
* chamadas de métodos;
* despacho dinâmico;
* resolução de sobrecarga;
* inferência de tipos;
* Capacidade de Mutação;
* sincronização;
* regras de concorrência;
* destrutores;
* conversões de tipos.

Enquanto este documento define **como os objetos são organizados**, o Documento 05 define **como eles se comportam durante a execução**.

---

## 16.3 Documento 06 — Arquitetura do Compilador

O Documento 06 descreve como uma implementação da linguagem pode transformar código-fonte em código executável.

Entre os assuntos previstos encontram-se:

* análise léxica;
* análise sintática;
* análise semântica;
* geração de representação intermediária;
* otimizações;
* geração de código nativo;
* integração com a infraestrutura de execução.

Esse documento trata exclusivamente da implementação do compilador.

Ele não altera a especificação da linguagem.

---

## 16.4 Infraestrutura de Execução

Embora a Capi seja compilada para código nativo e não dependa de máquina virtual ou ambiente de execução instalável, uma implementação poderá utilizar uma infraestrutura mínima de suporte durante a execução do programa.

Essa infraestrutura é responsável apenas por funções necessárias ao funcionamento da linguagem, tais como:

* criação e destruição de Domains;
* gerenciamento interno da memória dos Domains;
* interoperabilidade com o sistema operacional;
* suporte às funcionalidades básicas da biblioteca padrão.

Essa infraestrutura não constitui uma máquina virtual e não interfere na portabilidade do executável gerado.

---

# 17. Considerações Finais

O Modelo de Memória apresentado neste documento constitui o núcleo arquitetural da linguagem Capi.

Ao separar explicitamente:

* identidade;
* estado;
* comportamento;
* gerenciamento de memória;
* lifetime;
* autoridade para escrita,

a linguagem elimina conflitos tradicionalmente existentes entre Orientação a Objetos clássica e gerenciamento determinístico de memória.

Essa arquitetura permite preservar características historicamente associadas às linguagens orientadas a objetos, como:

* herança de implementação;
* subtipagem;
* despacho dinâmico;
* identidade compartilhada;
* grafos arbitrários de objetos;

sem recorrer a Garbage Collector ou Reference Counting como mecanismo principal de gerenciamento de memória.

A principal inovação da Capi não consiste em introduzir novos mecanismos de gerenciamento de memória, mas em redefinir a relação entre objetos e memória.

Enquanto linguagens tradicionais associam a responsabilidade pela memória aos próprios objetos, a Capi concentra essa responsabilidade nos Domains, transformando-os na única unidade física de gerenciamento.

Essa mudança permite que a organização lógica dos objetos permaneça completamente independente da organização física da memória, tornando ciclos, compartilhamento de identidade e grafos complexos propriedades naturais do modelo, e não problemas a serem resolvidos pelo runtime.

O Modelo de Memória estabelece, portanto, uma base determinística, previsível e compatível com otimizações agressivas, preservando simultaneamente segurança de memória, desempenho e os princípios clássicos da Orientação a Objetos.

Os documentos subsequentes detalharão como essa arquitetura é exposta ao programador, executada pelo compilador e materializada durante a execução dos programas, mantendo como princípio permanente a separação entre **as garantias oferecidas pela linguagem** e **os mecanismos utilizados para implementá-las**.
