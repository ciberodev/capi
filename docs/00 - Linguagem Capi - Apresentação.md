# Capi — A próxima geração da Orientação a Objetos

## Apresentação Geral da Linguagem

Versão: 0.2 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

A **Capi** é uma linguagem de programação orientada a objetos cujo objetivo é unir:

* Orientação a Objetos clássica;
* Segurança de memória estática;
* Gerenciamento determinístico de memória;
* Ausência de Garbage Collector;
* Ausência de Reference Counting;
* Alto desempenho;
* Concorrência segura.

A proposta da linguagem não consiste em adaptar o modelo de ownership do Rust à Orientação a Objetos tradicional.

A Capi redefine o próprio modelo de objetos.

Em vez de considerar que cada objeto é dono direto da memória que ocupa, a linguagem considera que a memória pertence a uma unidade maior chamada **Domain**.

---

# 2. Motivação

As linguagens orientadas a objetos atuais concentram quatro conceitos distintos em uma única referência:

* Identidade;
* Localização física;
* Autoridade para mutação;
* Tempo de vida.

Essa fusão dificulta a obtenção simultânea de:

* segurança de memória;
* gerenciamento determinístico;
* identidade compartilhada;
* herança de implementação;
* concorrência segura.

A Capi separa esses conceitos em entidades independentes.

A linguagem parte da seguinte hipótese:

> O problema não está na Orientação a Objetos em si, mas na forma como as linguagens tradicionais representam referências para objetos.

---

# 3. Princípios Fundamentais

A linguagem é construída sobre os seguintes princípios.

---

## 3.1 Objetos não possuem memória

Objetos representam entidades lógicas da aplicação.

Um objeto possui:

* identidade;
* estado;
* comportamento.

Porém, o estado do objeto não é armazenado como uma posse direta do próprio objeto.

O estado é armazenado fisicamente dentro de um **Domain**.

Assim, um objeto pode ser entendido como a união lógica entre:

* um `ObjectId`;
* uma definição de classe;
* um conjunto de métodos;
* um estado armazenado em um Domain.

O objeto existe como conceito da linguagem.

A memória onde seu estado reside pertence ao Domain.

Consequentemente:

* objetos não possuem outros objetos;
* objetos não controlam desalocação;
* objetos podem formar grafos arbitrários;
* ciclos deixam de representar um problema de gerenciamento de memória.

---

## 3.2 O Domain é a unidade física de memória

Um **Domain** representa uma região de memória administrada pelo compilador e pelo runtime mínimo da linguagem.

Responsabilidades de um Domain:

* alocação;
* desalocação;
* layout físico;
* lifetime;
* sincronização de escrita;
* isolamento concorrente.

Quando um Domain deixa de existir, toda sua memória é destruída deterministicamente.

Essa destruição independe da topologia dos objetos existentes dentro dele.

---

## 3.3 ObjectId representa identidade

Cada objeto possui uma identidade lógica permanente chamada **ObjectId**.

Essa identidade:

* pode ser copiada livremente;
* pode participar de grafos arbitrários;
* pode ser armazenada dentro de outros objetos;
* não representa posse da memória;
* não concede permissão de escrita.

Cada `ObjectId` está associado ao Domain onde o estado daquele objeto reside.

Essa relação entre `ObjectId` e `Domain` é validada pelo compilador.

A forma exata como essa associação será representada internamente é detalhe de implementação.

---

## 3.4 Mutação pertence ao Domain

Mutação nunca ocorre diretamente sobre um objeto isolado.

Toda mutação é uma operação sobre o Domain ao qual o objeto pertence.

Em linguagens tradicionais, uma referência normalmente permite acessar e modificar o objeto.

Na Capi, referência e autoridade de mutação são conceitos separados.

O programador pode possuir várias identidades para o mesmo objeto, mas a escrita sobre seu estado depende da permissão de mutação do Domain.

---

## 3.5 Capacidade de mutação

Para que um Domain seja modificado, deve existir uma capacidade de mutação válida para aquele Domain.

Essa capacidade é:

* exclusiva;
* não copiável;
* controlada estaticamente;
* geralmente inferida pelo compilador.

Em código comum, o programador não deverá manipular explicitamente essa capacidade.

A intenção é que chamadas como:

```capi
conta.depositar(100)
```

continuem parecendo chamadas orientadas a objetos tradicionais.

Internamente, o compilador verifica se existe autoridade válida para modificar o Domain onde `conta` reside.

---

# 4. Modelo Conceitual

A arquitetura da linguagem é composta por quatro elementos fundamentais.

```text
                +----------------+
                |   ObjectId     |
                +----------------+
                        |
                        v
                +----------------+
                |     Object     |
                +----------------+
                        |
                        v
                +----------------+
                |     Domain     |
                +----------------+
                        |
                        v
                +----------------+
                | Memory Layout  |
                +----------------+
```

Cada elemento possui responsabilidade única.

## ObjectId

Representa exclusivamente identidade.

## Object

Representa comportamento, estado lógico e pertencimento a uma classe.

## Domain

Representa propriedade física da memória, lifetime e autorização de escrita.

## Memory Layout

Representa a organização física dos dados dentro de um Domain.

---

# 5. Princípio Fundamental de Segurança

A Capi substitui o conceito clássico:

> "o objeto pode ser mutado"

por:

> "o domínio pode ser mutado".

Assim:

* múltiplas referências para um objeto são permitidas;
* múltiplos grafos são permitidos;
* ciclos são permitidos;
* identidade compartilhada é permitida;

porém a escrita é controlada pelo Domain.

Essa separação permite preservar a expressividade da Orientação a Objetos sem permitir mutabilidade compartilhada irrestrita.

---

# 6. Ciclos e Grafos de Objetos

Em linguagens com Reference Counting, ciclos entre objetos podem impedir a liberação da memória.

Na Capi, esse problema não ocorre da mesma forma.

Objetos armazenam identidades de outros objetos, não ownership direto sobre eles.

Um ciclo como:

```text
A -> B -> C
^         |
|_________|
```

é apenas um ciclo lógico entre `ObjectId`.

A memória de `A`, `B` e `C` pertence ao Domain.

Quando o Domain é destruído, toda a memória é liberada em bloco, independentemente da existência de ciclos.

Não há necessidade de contagem de referências nem de quebra manual de ciclos.

---

# 7. Herança, Subtipagem e Layout

A Capi preserva conceitos centrais da Orientação a Objetos:

* classes;
* herança de implementação;
* subtipagem;
* sobrescrita de métodos;
* dispatch dinâmico;
* interfaces ou traits.

A implementação inicial deve priorizar **herança simples**.

Herança múltipla de implementação não faz parte do modelo inicial da linguagem.

Para composição de comportamento, a linguagem deverá oferecer interfaces ou traits.

O layout físico dos objetos dentro de um Domain deverá ser compatível com coerções seguras entre tipos relacionados por herança.

A estratégia esperada é um layout prefixado, onde a parte correspondente à classe base aparece no início da representação da classe derivada.

Os detalhes completos de layout serão definidos no documento de modelo de memória.

---

# 8. Concorrência

A concorrência segura é baseada em Domains.

Domínios diferentes podem ser modificados em paralelo por fluxos de execução diferentes.

O mesmo Domain não pode sofrer mutações simultâneas sem sincronização explícita.

O compilador deve impedir, sempre que possível, que duas operações concorrentes tentem modificar o mesmo Domain ao mesmo tempo.

Quando a sincronização dinâmica for necessária, ela deverá ser explícita por meio de primitivas da linguagem ou da biblioteca padrão.

Assim, a unidade principal de isolamento concorrente da Capi não é o objeto individual, mas o Domain.

---

# 9. Escapes de Segurança

A Capi deverá oferecer algum mecanismo controlado de escape de segurança para cenários de baixo nível.

Esse mecanismo poderá permitir:

* interoperabilidade com C;
* acesso direto a ponteiros;
* manipulação de memória bruta;
* integração com sistemas operacionais;
* implementação de partes críticas da biblioteca padrão.

Essas operações deverão ser marcadas explicitamente como `unsafe` ou por mecanismo equivalente.

Código `unsafe` não invalida o modelo seguro da linguagem, mas transfere ao programador a responsabilidade por preservar as garantias que o compilador normalmente verificaria.

---

# 10. Consequências

Essa arquitetura produz as seguintes propriedades:

* ausência de Garbage Collector;
* ausência de ARC;
* ausência de ciclos de ownership;
* desalocação determinística;
* identidade compartilhada;
* suporte natural a grafos de objetos;
* herança de implementação;
* subtipagem;
* dispatch dinâmico;
* segurança estática;
* concorrência controlada por Domains.

---

# 11. Objetivos Não Funcionais

A implementação deverá buscar:

* custo zero em tempo de execução sempre que possível;
* inferência automática pelo compilador;
* sintaxe inspirada principalmente em Java e C#;
* uso de blocos com `{ }`;
* palavras-chave familiares como `class`, `extends`, `override`, `interface` e `new`;
* baixo custo cognitivo para desenvolvedores vindos de Java, C# e C++;

A diferença conceitual entre objetos, identidades e Domains deverá ser ocultada do programador na maioria dos casos.

O objetivo é preservar a experiência de uma linguagem orientada a objetos tradicional, mas com segurança de memória estática.

---

# 12. Escopo da Apresentação

Este documento descreve a visão geral da linguagem Capi.

Ele não pretende especificar formalmente:

* sistema de tipos;
* sintaxe completa;
* compilador;
* IR;
* backend;
* runtime;
* biblioteca padrão;
* semântica operacional;
* máquina virtual, caso exista.

Esses temas serão definidos em documentos próprios.

---

# 12. Princípio da Separação entre Garantias e Implementação

Todos os documentos da especificação da Capi devem distinguir explicitamente:

* as garantias oferecidas pela linguagem;
* os mecanismos utilizados para implementá-las.

As garantias constituem parte permanente da especificação.

Os mecanismos de implementação pertencem ao compilador, ao Modelo de Memória ou à Infraestrutura de Execução, e podem evoluir ao longo do tempo, desde que preservem integralmente essas garantias.

Este princípio aplica-se a todos os documentos da especificação da Capi.

Cada documento deverá deixar claro quando está descrevendo uma propriedade garantida pela linguagem e quando está descrevendo uma possível estratégia de implementação.

---

# 14. Documentos Relacionados

A documentação técnica da Capi será organizada em módulos:

* 00 — Linguagem Capi — Apresentação
* 01 — Visão Geral e Fundamentos
* 02 — Sistema de Tipos
* 03 — Modelo de Memória
* 04 — Sintaxe da Linguagem
* 05 — Semântica Operacional
* 06 — Arquitetura do Compilador
* 07 — Intermediate Representation (IR)
* 08 — Biblioteca Padrão
* 09 — Runtime Mínimo
* 10 — ABI e Interoperabilidade (FFI)
* 11 — Toolchain (capi, capic, capi fmt, capi test, capi doc)
* 12 — Ecossistema e Gerenciamento de Pacotes

O Documento 00 deve permanecer como apresentação geral e simplificada.

Os documentos seguintes deverão detalhar formalmente as ideias apresentadas aqui.
