# Capi — A Próxima Geração da Orientação a Objetos

# Documento 02 — Sistema de Tipos

Versão: 1.1 (Stable Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define o **Sistema de Tipos** da linguagem Capi.

Seu objetivo é estabelecer os tipos fundamentais da linguagem, suas responsabilidades, suas propriedades e as garantias estáticas fornecidas pelo compilador.

O Sistema de Tipos é um dos pilares da arquitetura da Capi e atua como principal mecanismo para garantir:

* segurança de memória;
* segurança de tipos;
* coerência das hierarquias de classes;
* ausência de referências inválidas;
* isolamento entre Domains.

---

## 1.2 Escopo

Este documento define exclusivamente o modelo conceitual do Sistema de Tipos.

Não fazem parte deste documento:

* sintaxe da linguagem;
* layout de memória;
* representação física dos objetos;
* arquitetura do compilador;
* detalhes da infraestrutura de execução.

Esses assuntos são tratados em documentos específicos.

---

## 1.3 Relação com os Documentos Anteriores

Este documento complementa:

* **Documento 00 — Apresentação da Linguagem**, que apresenta a visão geral da Capi;
* **Documento 01 — Visão Geral e Fundamentos**, que define os princípios arquiteturais da linguagem.

Todas as definições aqui apresentadas devem respeitar os princípios estabelecidos nesses documentos.

---

# 2. Objetivos do Sistema de Tipos

O Sistema de Tipos da Capi possui os seguintes objetivos fundamentais.

## 2.1 Segurança

Detectar inconsistências em tempo de compilação sempre que possível.

O compilador deve impedir operações que possam comprometer a integridade da memória ou da hierarquia de tipos.

---

## 2.2 Expressividade

Permitir o desenvolvimento de aplicações orientadas a objetos preservando:

* herança;
* polimorfismo;
* encapsulamento;
* identidade compartilhada.

Sem exigir mecanismos tradicionais de gerenciamento automático de memória.

---

## 2.3 Previsibilidade

Todo tipo possui comportamento bem definido.

O Sistema de Tipos não deve depender de verificações dinâmicas quando essas puderem ser resolvidas estaticamente.

---

## 2.4 Otimização

As informações produzidas pelo Sistema de Tipos devem permitir que o compilador realize otimizações agressivas durante a geração de código.

---

# 3. Definição de Tipo

Na linguagem Capi, um **tipo** representa o conjunto de propriedades, operações e restrições associadas a um elemento manipulável pelo programa.

Todo elemento da linguagem possui exatamente um tipo estático conhecido em tempo de compilação.

O tipo determina, entre outras propriedades:

* operações permitidas;
* regras de conversão;
* forma de armazenamento;
* participação em hierarquias;
* regras de identidade;
* requisitos de segurança.

---

# 4. Categorias Fundamentais

Todos os tipos da linguagem pertencem exatamente a uma das seguintes categorias.

* Tipos Primitivos;
* Tipos por Valor;
* Tipos por Identidade;
* Tipos de Domínio.

Cada categoria possui responsabilidades específicas.

Um mesmo tipo nunca pertence simultaneamente a mais de uma categoria.

---

# 5. Categorias de Tipos

## 5.1 Tipos Primitivos

Tipos primitivos representam valores elementares fornecidos pela própria linguagem.

Características:

* tamanho conhecido em tempo de compilação;
* armazenamento direto;
* ausência de identidade;
* cópia por valor;
* não pertencem a Domains;
* não participam de hierarquias de classes.

Exemplos previstos:

* `Bool`
* `Char`
* `Int`
* `UInt`
* `Float`
* `Double`

### Unit

A linguagem define ainda o tipo especial `Unit`.

`Unit` representa a ausência de um valor significativo.

Toda função que não produz um resultado retorna implicitamente um valor do tipo `Unit`.

Diferentemente de `void` em outras linguagens, `Unit` é um tipo válido e pode participar normalmente do sistema de tipos.

---

## 5.2 Tipos por Valor

Tipos por valor representam agregações de dados copiáveis.

Exemplos:

* `Struct`
* `Tuple`
* `Enum`

Características:

* não possuem identidade própria;
* são armazenados diretamente;
* pertencem ao objeto ou variável que os contém;
* são copiados por valor;
* não participam do sistema de `ObjectId`.

A cópia de um tipo por valor ocorre membro a membro, respeitando a semântica de cada campo.

Caso um campo seja um `ObjectId`, apenas sua identidade será copiada.

---

### Ausência de Valor

A Capi não possui referências nulas (`null`).

A ausência de um valor deve ser representada através de tipos opcionais.

O tipo padrão previsto pela linguagem é:

```text
Optional<T>
```

ou construção equivalente definida pela biblioteca padrão.

Essa decisão elimina a possibilidade de referências nulas acidentais, permitindo que sua utilização seja sempre explícita no sistema de tipos.

---

## 5.3 Tipos por Identidade

Tipos por identidade representam objetos.

Características:

* possuem identidade permanente;
* podem ser compartilhados por múltiplas referências lógicas;
* pertencem exatamente a um Domain;
* suportam herança;
* suportam subtipagem;
* suportam polimorfismo.

A memória desses objetos pertence exclusivamente ao Domain onde residem.

Objetos nunca possuem ownership da própria memória.

---

## 5.4 Tipos de Domínio

Domains representam a unidade física de gerenciamento de memória da linguagem.

Características:

* administram memória;
* administram lifetime;
* administram sincronização;
* administram operações de escrita;
* podem conter objetos de diferentes classes.

Domains não pertencem à hierarquia de objetos.

Domains não possuem identidade orientada a objetos.

Domains representam exclusivamente uma entidade arquitetural responsável pelo gerenciamento da memória.

A organização física dos objetos dentro de um Domain é definida no Documento 03 — Modelo de Memória.

---

## 5.5 Propriedades dos Tipos

Além de sua categoria, cada tipo possui propriedades semânticas utilizadas pelo compilador.

Entre elas:

| Propriedade | Descrição                                                          |
| ----------- | ------------------------------------------------------------------ |
| Identidade  | O tipo representa uma entidade com identidade própria.             |
| Copiável    | Valores desse tipo podem ser copiados diretamente.                 |
| Herdável    | O tipo pode participar de hierarquias de herança.                  |
| Polimórfico | O tipo pode ser utilizado através de uma superclasse ou interface. |
| Persistente | O estado do tipo reside em um Domain.                              |

Essas propriedades são utilizadas pelo compilador para validar operações permitidas sobre cada tipo.

Os detalhes completos dessas validações são definidos nos documentos posteriores da especificação.

---

# 6. Classes

Classes representam o principal mecanismo de modelagem da Orientação a Objetos na linguagem Capi.

Uma classe define a estrutura lógica e o comportamento de um conjunto de objetos.

Classes descrevem:

* atributos;
* métodos;
* construtores;
* contratos públicos;
* hierarquias de herança;
* visibilidade de seus membros.

Uma classe **não** define:

* alocação de memória;
* lifetime;
* gerenciamento de Domains;
* estratégia de sincronização.

Essas responsabilidades pertencem ao modelo de memória da linguagem.

---

## 6.1 Classe como Tipo

Toda classe define um tipo por identidade.

Considere, por exemplo:

```text
Classe Conta
```

Ela define um tipo denominado:

```text
Conta
```

Toda instância dessa classe pertence ao tipo `Conta`.

---

## 6.2 Estado

Uma classe pode declarar atributos.

Os atributos representam o estado lógico do objeto.

Fisicamente, esse estado é armazenado no Domain ao qual a instância pertence.

A classe descreve a estrutura desse estado, mas não controla sua localização física.

---

## 6.3 Comportamento

Uma classe pode declarar métodos.

Métodos representam operações sobre o estado lógico do objeto.

Métodos podem ser classificados como:

* somente leitura;
* mutantes.

Operações mutantes exigem que o sistema de tipos garanta autorização para escrita sobre o Domain correspondente.

O mecanismo utilizado para essa garantia faz parte da arquitetura da linguagem e não desta especificação.

---

## 6.4 Identidade

Toda instância de uma classe possui exatamente um `ObjectId`.

A identidade do objeto permanece constante durante todo o seu lifetime.

A identidade é independente:

* da posição física da memória;
* da hierarquia de herança;
* da quantidade de referências existentes para o objeto.

---

# 7. ObjectId

`ObjectId<T>` é um tipo fundamental da linguagem.

Ele representa exclusivamente a identidade lógica de um objeto pertencente ao tipo `T`.

Apesar do nome, `ObjectId` não representa:

* um endereço de memória;
* um ponteiro;
* uma chave numérica;
* ownership sobre o objeto.

Seu único propósito é representar identidade.

---

## 7.1 Propriedades

Todo `ObjectId<T>` possui as seguintes propriedades:

* copiável;
* comparável;
* persistente durante o lifetime do objeto;
* associado exatamente a um Domain.

O compilador garante que um `ObjectId` nunca faça referência a um objeto inexistente durante seu período de validade.

---

## 7.2 Relação com o Domain

Todo `ObjectId` está associado exatamente a um Domain.

Essa associação é parte do sistema de tipos e é validada estaticamente pelo compilador.

A forma como essa associação é representada internamente é um detalhe de implementação.

---

## 7.3 Relação com Herança

Considere a seguinte hierarquia:

```text
Animal
   ▲
   │
Cachorro
```

Então:

```text
ObjectId<Cachorro>
```

é implicitamente conversível para

```text
ObjectId<Animal>
```

Essa operação representa um **upcast**.

Características:

* custo zero;
* preserva a identidade do objeto;
* não altera o Domain;
* não modifica a memória.

A operação inversa (downcast) é definida pelas regras de conversão descritas neste documento e pela Semântica Operacional da linguagem.

---

## 7.4 Igualdade e Identidade

A Capi distingue dois conceitos diferentes.

### Igualdade

Dois objetos podem possuir estados equivalentes.

Exemplo:

```text
Conta A
Saldo = 100

Conta B
Saldo = 100
```

Os estados podem ser considerados iguais.

---

### Identidade

Mesmo possuindo estados equivalentes, os dois objetos continuam sendo entidades distintas.

Cada um possui seu próprio `ObjectId`.

A igualdade de estado não implica igualdade de identidade.

---

# 8. Domains

O Domain representa a unidade física de gerenciamento de memória da linguagem.

Todo objeto pertence exatamente a um Domain.

Nenhum objeto pode existir fora de um Domain.

---

## 8.1 Responsabilidades

Um Domain é responsável por:

* alocação de memória;
* desalocação determinística;
* organização física dos objetos;
* lifetime;
* sincronização de escrita;
* isolamento concorrente.

Domains não participam da hierarquia de tipos orientados a objetos.

---

## 8.2 Heterogeneidade

Um mesmo Domain pode conter objetos de diferentes classes.

Por exemplo:

```text
Domain Banco

Conta
Cliente
Agência
Funcionário
Movimento
```

O Domain administra memória.

Ele não representa uma categoria de tipos.

---

## 8.3 Lifetime

Todos os objetos pertencentes a um Domain possuem lifetime limitado ao lifetime do próprio Domain.

Quando um Domain é destruído:

* todos os objetos nele contidos deixam de existir;
* toda memória é liberada deterministicamente;
* nenhuma análise de ciclos é necessária.

---

## 8.4 Escrita

Operações de escrita pertencem exclusivamente ao Domain.

O Sistema de Tipos define garantias que impedem operações incompatíveis de escrita sobre um mesmo Domain.

Esta especificação não define o mecanismo utilizado para fornecer essa garantia.

---

# 9. Hierarquia de Classes

A Capi preserva o modelo clássico de herança simples.

Uma classe pode possuir exatamente uma superclasse.

Essa restrição simplifica:

* layout de memória;
* despacho dinâmico;
* verificação de tipos;
* otimizações do compilador.

---

## 9.1 Herança

A herança permite:

* reutilização de implementação;
* extensão de comportamento;
* especialização de tipos.

A subclasse herda os membros acessíveis da superclasse conforme as regras de visibilidade da linguagem.

---

## 9.2 Subtipagem

Toda subclasse define automaticamente um subtipo de sua superclasse.

Se:

```text
Cachorro extends Animal
```

então:

```text
Cachorro
```

é subtipo de

```text
Animal
```

Essa relação é reconhecida pelo compilador.

---

## 9.3 Polimorfismo

A hierarquia de classes permite que um objeto seja tratado como qualquer um de seus supertipos.

O despacho dinâmico preserva o comportamento da instância concreta.

Os detalhes do mecanismo de despacho são definidos no Documento 03 — Modelo de Memória.

---

## 9.4 Interfaces e Traits

Classes podem implementar qualquer quantidade de Interfaces e Traits compatíveis.

Interfaces representam contratos.

Traits representam reutilização de comportamento.

Nenhum dos dois altera a organização física da memória dos objetos.

---

## 9.5 Extensibilidade

A Capi não suporta herança múltipla de implementação.

Essa restrição é permanente e faz parte da arquitetura da linguagem.

A composição de comportamento deve ocorrer através de Interfaces e Traits.

Essa decisão reduz a complexidade do Sistema de Tipos e do Modelo de Memória, preservando compatibilidade com herança de implementação, subtipagem e despacho dinâmico.

---

# 10. Interfaces

Interfaces definem contratos públicos de comportamento.

Uma Interface especifica quais operações um tipo deve oferecer, sem determinar como essas operações são implementadas.

Interfaces não possuem estado persistente e não influenciam a organização física da memória dos objetos.

---

## 10.1 Finalidade

Interfaces existem para desacoplar comportamento da implementação concreta.

Seu principal objetivo é permitir que diferentes classes possam ser utilizadas através de um contrato comum.

Exemplo conceitual:

```text
Interface Pagável

pagar()
cancelar()
```

Diversas classes podem implementar essa Interface independentemente de sua hierarquia.

---

## 10.2 Características

Interfaces possuem as seguintes propriedades:

* não possuem identidade;
* não possuem estado persistente;
* não administram memória;
* não pertencem a Domains;
* podem participar do polimorfismo.

Interfaces representam exclusivamente contratos públicos.

---

## 10.3 Implementação

Uma classe pode implementar qualquer quantidade de Interfaces.

A implementação de uma Interface não altera a hierarquia de herança da classe.

---

## 10.4 Compatibilidade

Sempre que uma classe implementa uma Interface, seus objetos podem ser utilizados através dessa Interface.

Essa conversão é considerada segura e faz parte do sistema de tipos.

---

# 11. Traits

Traits representam reutilização de comportamento.

Enquanto Interfaces definem apenas contratos, Traits permitem compartilhar implementações entre diferentes classes.

Traits não constituem uma forma de herança múltipla.

Seu objetivo é reutilizar comportamento preservando a simplicidade da hierarquia de classes.

Traits não podem declarar estado persistente.

Caso um comportamento reutilizável necessite de informações auxiliares, essas deverão ser fornecidas pela classe que implementa a Trait, por meio de atributos próprios, parâmetros ou métodos abstratos.

---

## 11.1 Finalidade

Traits permitem evitar duplicação de código sem alterar o modelo de objetos da linguagem.

Uma Trait pode fornecer:

* implementação de métodos;
* métodos auxiliares;
* operações derivadas de outros métodos.

---

## 11.2 Estado

Traits não possuem estado persistente.

Consequentemente:

* não possuem identidade;
* não possuem ObjectId;
* não pertencem a Domains;
* não adicionam layout físico aos objetos.

Todo estado manipulado por uma Trait pertence ao objeto que a utiliza.

---

## 11.3 Composição

Uma classe pode utilizar qualquer quantidade de Traits compatíveis.

Caso duas Traits forneçam implementações conflitantes para o mesmo método, o conflito deverá ser resolvido explicitamente pela classe.

As regras formais de resolução de conflitos serão definidas na Semântica Operacional.

---

## 11.4 Diferenças entre Interface e Trait

| Interface                                   | Trait                              |
| ------------------------------------------- | ---------------------------------- |
| Define contratos                            | Reutiliza comportamento            |
| Não possui implementação padrão obrigatória | Pode fornecer implementação padrão |
| Não possui estado                           | Não possui estado persistente      |
| Não altera layout                           | Não altera layout                  |
| Não reutiliza código                        | Reutiliza código                   |

---

# 12. Polimorfismo

A Capi suporta polimorfismo baseado em subtipagem.

Um objeto pode ser utilizado através:

* de sua própria classe;
* de qualquer superclasse;
* de qualquer Interface implementada;
* de qualquer Trait compatível quando permitido pela linguagem.

O polimorfismo preserva a identidade do objeto.

---

## 12.1 Dispatch Dinâmico

Chamadas polimórficas utilizam despacho dinâmico.

O mecanismo utilizado para implementar esse despacho faz parte do Modelo de Memória e da Arquitetura do Compilador.

O Sistema de Tipos garante apenas sua correção semântica.

---

## 12.2 Preservação da Identidade

Operações polimórficas nunca modificam:

* o ObjectId;
* o Domain;
* o lifetime do objeto.

O polimorfismo altera apenas a forma como o objeto é observado pelo sistema de tipos.

---

# 13. Tipos Genéricos

A Capi suporta tipos parametrizados.

Tipos genéricos permitem construir abstrações reutilizáveis preservando segurança de tipos.

Exemplos conceituais:

```text
List<T>

Map<K,V>

Optional<T>

Result<T,E>
```

---

## 13.1 Objetivos

Os tipos genéricos devem:

* preservar segurança estática;
* evitar conversões desnecessárias;
* eliminar verificações de tipo em tempo de execução sempre que possível.

---

## 13.2 Restrições

Parâmetros genéricos podem declarar restrições.

Exemplo conceitual:

```text
T implementa Serializable
```

As regras sintáticas serão definidas no Documento 04.

---

## 13.3 Variância

A Capi define regras formais de variância para preservar a segurança do sistema de tipos.

Como regra geral:

* tipos genéricos são invariantes por padrão;
* exceções poderão existir quando comprovadamente seguras.

As regras completas de covariância, contravariância e invariância são definidas na Semântica Operacional da linguagem.

---

## 13.4 Optional

A ausência de valores é representada através de:

```text
Optional<T>
```

Esse tipo substitui o conceito tradicional de referências nulas.

Todo acesso ao valor contido em um `Optional<T>` exige que sua presença tenha sido previamente comprovada pelo sistema de tipos ou explicitamente verificada pelo programador.

---

## 13.5 Result

Operações que podem produzir sucesso ou falha podem utilizar:

```text
Result<T,E>
```
Result<T,E> representa o resultado de uma operação que pode produzir um valor de sucesso do tipo T ou uma condição de falha do tipo E.

Esse tipo permite representar falhas esperadas sem recorrer a valores nulos ou exceções como mecanismo principal de controle de fluxo.

Ambos os estados fazem parte do sistema de tipos e devem ser tratados explicitamente pelo programador.

O compilador pode exigir que os casos de sucesso e falha sejam considerados antes que o valor seja utilizado.

---

# 14. Conversões

Conversões representam transformações entre tipos compatíveis.

O Sistema de Tipos distingue quatro categorias fundamentais.

---

## 14.1 Conversão Implícita

Conversões implícitas são realizadas automaticamente pelo compilador.

Somente são permitidas quando preservam integralmente a segurança de tipos.

---

## 14.2 Conversão Explícita

Conversões explícitas exigem manifestação do programador.

Seu uso indica que existe mudança semântica entre os tipos envolvidos.

---

## 14.3 Upcast

Upcast representa a conversão de um subtipo para um supertipo.

Exemplo conceitual:

```text
ObjectId<Cachorro>

↓

ObjectId<Animal>
```

Características:

* operação segura;
* custo zero;
* preserva identidade;
* preserva Domain;
* preserva lifetime.

---

## 14.4 Downcast

Downcast representa a conversão inversa.

O Sistema de Tipos somente permite downcast quando sua segurança puder ser comprovada.

Esta especificação não determina quais mecanismos serão utilizados para essa comprovação.

As regras formais pertencem à Semântica Operacional da linguagem.

---

## 14.5 Conversões Proibidas

O Sistema de Tipos não permite conversões que comprometam a segurança da linguagem.

Entre elas:

* conversões entre hierarquias incompatíveis;
* conversões que alterem o Domain de um objeto;
* conversões que modifiquem a identidade de um objeto;
* conversões que produzam referências inválidas.

Essas restrições constituem garantias permanentes da linguagem e não podem ser violadas por mecanismos automáticos do compilador.

---

# 15. Operações de Escrita

Na Capi, operações de escrita pertencem exclusivamente ao **Domain**.

Objetos representam identidade e comportamento, mas não possuem autoridade para modificar diretamente a memória onde seu estado está armazenado.

Essa separação constitui um dos princípios fundamentais da linguagem.

---

## 15.1 Exclusividade de Escrita

O Sistema de Tipos deve garantir que operações incompatíveis de escrita sobre um mesmo Domain não possam ocorrer simultaneamente.

Essa garantia é obrigatória e faz parte das propriedades fundamentais da linguagem.

A forma como essa exclusividade é implementada pertence à arquitetura do compilador e ao Modelo de Memória.

---

## 15.2 Separação entre Garantia e Implementação

Este documento define apenas a garantia semântica oferecida pela linguagem.

Ele não estabelece qual mecanismo será utilizado para implementá-la.

Entre os mecanismos possíveis encontram-se, por exemplo:

* análise estática;
* capacidades lineares;
* regiões;
* análise de fluxo;
* outros mecanismos equivalentes.

A escolha do mecanismo faz parte da implementação da linguagem e poderá evoluir sem alterar sua especificação.

---

## 15.3 Leituras

Leituras não modificam o estado do Domain.

Consequentemente, múltiplas operações de leitura podem coexistir, desde que respeitadas as regras de concorrência definidas pela linguagem.

O Sistema de Tipos deve impedir situações em que operações de leitura e escrita possam produzir comportamento indefinido.

---

# 16. Garantias do Sistema de Tipos

O Sistema de Tipos da Capi fornece garantias estáticas obrigatórias.

Essas garantias constituem parte da definição da linguagem.

---

## 16.1 Garantias Estruturais

O compilador deve garantir:

* coerência das hierarquias de classes;
* validade das implementações de Interfaces e Traits;
* consistência das relações de subtipagem;
* validade das conversões permitidas;
* associação correta entre `ObjectId` e Domain.

---

## 16.2 Garantias de Memória

O compilador deve impedir:

* referências pendentes (*Dangling References*);
* uso após destruição (*Use After Free*);
* dupla liberação (*Double Free*);
* corrupção de memória;
* acesso a objetos fora do lifetime de seu Domain.

---

## 16.3 Garantias de Concorrência

O Sistema de Tipos deve impedir, sempre que possível:

* data races;
* escritas concorrentes incompatíveis;
* acessos inconsistentes ao mesmo Domain.

Essas garantias devem ser obtidas preferencialmente em tempo de compilação.

---

## 16.4 Garantias de Nulabilidade

A linguagem não admite referências nulas.

Toda ausência de valor deve ser representada explicitamente através de tipos opcionais ou mecanismos equivalentes.

O compilador deve impedir acessos inseguros a valores opcionais.

---

# 17. Restrições

Para preservar as garantias da linguagem, o Sistema de Tipos estabelece as seguintes restrições permanentes.

---

## 17.1 Ownership entre Objetos

Objetos não possuem ownership de outros objetos.

As relações entre objetos ocorrem exclusivamente através de identidades (`ObjectId`).

---

## 17.2 Objetos fora de Domains

Nenhum objeto pode existir fora de um Domain.

Todo objeto pertence exatamente a um Domain durante todo o seu lifetime.

---

## 17.3 Referências Inválidas

O Sistema de Tipos não permite a construção de referências para:

* objetos inexistentes;
* objetos destruídos;
* objetos cujo Domain não seja mais válido.

---

## 17.4 Alteração de Identidade

A identidade de um objeto é imutável.

Nenhuma operação da linguagem pode alterar o `ObjectId` associado a um objeto existente.

---

## 17.5 Alteração de Domain

Objetos não podem ser movidos livremente entre Domains.

Caso a linguagem ofereça mecanismos para migração entre Domains, eles deverão preservar integralmente as garantias do Sistema de Tipos e serão definidos em documento específico.

---

## 17.6 Operações Fora das Garantias Estáticas

A linguagem poderá oferecer mecanismos destinados à interoperabilidade com código externo, acesso direto à memória ou outras operações que não possam ser integralmente verificadas pelo Sistema de Tipos.

Tais mecanismos constituem exceções explícitas às garantias estáticas da linguagem e transferem ao programador a responsabilidade por preservar sua correção.

A sintaxe e a semântica desses mecanismos serão definidas em documento específico.

---

# 18. Tabela normativa


| Categoria                         | Possui Identidade | Copiável | Herdável | Polimórfico | Possui Estado Persistente | Requer Domain | Gerencia Memória |
| --------------------------------- | :---------------: | :------: | :------: | :---------: | :-----------------------: | :-----------: | :--------------: |
| Primitive                         |        Não        |    Sim   |    Não   |     Não     |            Não            |      Não      |        Não       |
| Value (`Struct`, `Tuple`, `Enum`) |        Não        |    Sim   |    Não   |     Não     |            Não            |      Não      |        Não       |
| `ObjectId<T>`                     |  Sim (representa) |    Sim   |    Não   |     Sim     |            Não            |      Sim      |        Não       |
| `Class`                           |        Sim        |    Não   |    Sim   |     Sim     |            Sim            |      Sim      |        Não       |
| `Interface`                       |        Não        |    Não   |    Não   |     Sim     |            Não            |      Não      |        Não       |
| `Trait`                           |        Não        |    Não   |    Não   |     Sim     |            Não            |      Não      |        Não       |
| `Domain`                          |        Não        |    Não   |    Não   |     Não     |            Sim            |       —       |        Sim       |

## Definição das Colunas

| Coluna                        | Significado                                                                                         |
| ----------------------------- | --------------------------------------------------------------------------------------------------- |
| **Possui Identidade**         | O tipo representa ou define uma entidade cuja identidade pode ser distinguida de outras instâncias. |
| **Copiável**                  | Valores desse tipo podem ser copiados livremente sem alterar sua semântica.                         |
| **Herdável**                  | O tipo pode participar de uma hierarquia de herança de implementação.                               |
| **Polimórfico**               | Pode ser utilizado através de uma superclasse ou Interface.                                         |
| **Possui Estado Persistente** | O tipo mantém estado armazenado de forma persistente durante seu lifetime.                          |
| **Requer Domain**             | Instâncias desse tipo somente podem existir associadas a um Domain.                                 |
| **Gerencia Memória**          | O tipo é responsável pelo gerenciamento do lifetime e da memória de outros elementos.               |


---

# 19. Princípio da Separação entre Garantias e Implementação

Todos os documentos da especificação da Capi devem distinguir explicitamente:

* as garantias oferecidas pela linguagem;
* os mecanismos utilizados para implementá-las.

As garantias constituem parte permanente da especificação.

Os mecanismos de implementação pertencem ao compilador, ao Modelo de Memória ou à Infraestrutura de Execução, e podem evoluir ao longo do tempo, desde que preservem integralmente essas garantias.

Este princípio aplica-se a todos os documentos da especificação da Capi.

Cada documento deverá deixar claro quando está descrevendo uma propriedade garantida pela linguagem e quando está descrevendo uma possível estratégia de implementação.

---

# 20. Relação com os Próximos Documentos

Este documento define exclusivamente o modelo conceitual do Sistema de Tipos.

Os documentos seguintes detalham aspectos específicos da linguagem.

---

## Documento 03 — Modelo de Memória

Define:

* organização física dos Domains;
* layout dos objetos;
* representação de herança;
* estratégia de alocação;
* destruição determinística;
* representação física de `ObjectId`;
* infraestrutura de gerenciamento de memória.

---

## Documento 04 — Sintaxe da Linguagem

Define:

* gramática;
* declarações;
* expressões;
* instruções;
* palavras-chave;
* regras de escrita do código.

---

## Documento 05 — Semântica Operacional

Define:

* execução das instruções;
* resolução de chamadas de método;
* despacho dinâmico;
* regras formais de conversão;
* variância;
* inferência de tipos;
* validações semânticas realizadas pelo compilador.

---

## Documento 06 — Arquitetura do Compilador

Define:

* etapas de compilação;
* representação intermediária;
* análise semântica;
* geração de código;
* otimizações;
* integração entre compilador e infraestrutura de execução.

---

# 21. Considerações Finais

O Sistema de Tipos da Capi foi projetado para separar responsabilidades tradicionalmente concentradas em uma única referência.

Enquanto linguagens orientadas a objetos convencionais associam identidade, localização física, lifetime e autoridade de escrita ao próprio objeto, a Capi distribui essas responsabilidades entre entidades distintas.

Essa separação permite preservar os princípios clássicos da Orientação a Objetos, ao mesmo tempo em que estabelece garantias estáticas de segurança de memória e gerenciamento determinístico.

O Sistema de Tipos constitui, portanto, a base formal sobre a qual serão construídos o Modelo de Memória, a Semântica Operacional e a Arquitetura do Compilador, mantendo como princípio permanente a distinção entre **garantias oferecidas pela linguagem** e **mecanismos utilizados para implementá-las**.

