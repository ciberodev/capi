# Capi — A Próxima Geração da Orientação a Objetos

# Documento 01 — Visão Geral e Fundamentos

Versão: 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define os fundamentos conceituais da linguagem **Capi**.

Seu objetivo é estabelecer os princípios arquiteturais que orientam toda a especificação da linguagem.

Todos os documentos posteriores devem ser compatíveis com os conceitos definidos aqui.

---

## 1.2 Relação com o Documento 00

Este documento complementa o **00 - Linguagem Capi - Apresentação**.

Enquanto o Documento 00 apresenta a visão geral da linguagem, este documento estabelece seu modelo conceitual e suas definições fundamentais.

Recomenda-se a leitura do Documento 00 antes desta especificação.

---

# 2. Objetivos Fundamentais

A Capi é construída sobre quatro objetivos fundamentais.

## 2.1 Segurança

Toda operação envolvendo memória deve ser verificável em tempo de compilação.

A linguagem deve impedir:

* Use After Free;
* Double Free;
* Dangling References;
* Data Races;
* Null Pointer Dereference;
* Corrupção de memória.

A linguagem não possui referências nulas. A ausência de valor é representada explicitamente por tipos opcionais definidos pelo Sistema de Tipos.

Sempre que possível, essas garantias devem ser obtidas sem verificações dinâmicas.

---

## 2.2 Orientação a Objetos

A linguagem preserva os conceitos clássicos da Orientação a Objetos:

* classes;
* encapsulamento;
* herança de implementação;
* subtipagem;
* despacho dinâmico;
* identidade dos objetos;
* estado mutável.

A inovação da Capi não está no paradigma orientado a objetos, mas no modelo de gerenciamento de memória.

---

## 2.3 Determinismo

Toda alocação e desalocação devem ocorrer de maneira previsível.

A linguagem não utiliza:

* Garbage Collector;
* Reference Counting;
* mecanismos automáticos de destruição baseados em ciclos.

O tempo de vida da memória deve ser determinístico.

---

## 2.4 Performance

A linguagem segue o princípio de **Zero-Cost Abstractions**.

Toda abstração incorporada à linguagem deve possuir custo previsível e, sempre que possível, ser completamente eliminada pelo compilador.

---

# 3. O Problema

Nas linguagens orientadas a objetos tradicionais, uma referência representa simultaneamente:

* identidade;
* localização física;
* autoridade para modificação;
* tempo de vida.

Essa unificação dificulta a obtenção simultânea de:

* segurança de memória;
* gerenciamento determinístico;
* identidade compartilhada;
* concorrência segura.

A Capi resolve esse problema separando essas responsabilidades.

---

# 4. Modelo Fundamental

A unidade fundamental de gerenciamento de memória da Capi é o **Domain**.

Objetos deixam de ser responsáveis pela memória que ocupam.

A memória passa a pertencer exclusivamente aos Domains.

Essa decisão produz naturalmente:

* grafos arbitrários de objetos;
* identidade compartilhada;
* destruição determinística;
* ausência de ciclos de ownership;
* isolamento de escrita.

Todos os demais conceitos da linguagem derivam desse princípio.

---

# 5. Modelo Conceitual

## 5.1 Classe

Uma classe define:

* estrutura lógica;
* comportamento;
* contratos públicos;
* relações de herança.

Classes não administram memória.

Interfaces definem contratos públicos. Traits permitem reutilização de comportamento. Nenhum dos dois participa do layout físico dos objetos.

---

## 5.2 Objeto

Um objeto representa uma instância lógica de uma classe.

Todo objeto possui:

* identidade;
* estado lógico;
* comportamento.

O estado pertence conceitualmente ao objeto.

Entretanto, seu armazenamento físico pertence ao Domain onde o objeto reside.

Objetos nunca administram diretamente sua própria memória.

---

## 5.3 Domain

Um Domain representa a unidade física de gerenciamento de memória.

Todo objeto pertence exatamente a um Domain.

O Domain é responsável por:

* alocação;
* desalocação;
* organização física da memória;
* lifetime;
* isolamento de escrita;
* sincronização.

Domains podem conter qualquer quantidade de objetos.

Quando um Domain é destruído, toda a memória sob sua responsabilidade é liberada deterministicamente.

A linguagem utiliza apenas uma infraestrutura mínima de execução para suportar funcionalidades fundamentais que não podem ser resolvidas exclusivamente em tempo de compilação.

Essa infraestrutura faz parte do binário gerado e não constitui uma máquina virtual nem um ambiente de execução instalável.

Programas Capi são compilados diretamente para código nativo e executam sem dependência de ambientes equivalentes à JVM, CLR ou JRE.

---

## 5.4 ObjectId

Todo objeto possui um identificador lógico denominado **ObjectId**.

O ObjectId:

* identifica unicamente um objeto;
* pode ser copiado livremente;
* pode participar de grafos arbitrários;
* não representa ownership;
* não concede autoridade de escrita.

Todo ObjectId está associado exatamente a um Domain.

Essa associação é validada estaticamente pelo compilador.

Referências entre Domains diferentes somente podem ocorrer através dos mecanismos definidos pelo sistema de tipos.

---

# 6. Separação de Responsabilidades

A Capi separa responsabilidades tradicionalmente concentradas em uma única referência.

| Conceito       | Responsável           |
| -------------- | --------------------- |
| Identidade     | ObjectId              |
| Estado lógico  | Objeto                |
| Comportamento  | Classe                |
| Memória física | Domain                |
| Lifetime       | Domain                |
| Escrita        | Domain                |
| Polimorfismo   | Hierarquia de Classes |

Cada responsabilidade possui exatamente um proprietário conceitual.

---

# 7. Princípios Fundamentais

Toda implementação da Capi deve respeitar os seguintes princípios.

---

## Princípio 1 — Objetos não possuem memória

Objetos representam apenas entidades lógicas.

A memória pertence exclusivamente aos Domains.

---

## Princípio 2 — Objetos não possuem outros objetos

Objetos armazenam identidades de outros objetos.

Consequentemente:

* grafos arbitrários são suportados;
* ciclos são naturais;
* ownership deixa de depender da estrutura do grafo.

---

## Princípio 3 — Escrita pertence ao Domain

Modificar um objeto significa modificar o Domain ao qual ele pertence.

A linguagem deve possuir um mecanismo estático — doravante denominado Capacidade de Mutação — que garanta a exclusividade das operações de escrita sobre um Domain.

A forma como esse mecanismo é implementado faz parte do Sistema de Tipos.

---

## Princípio 4 — Leitura é compartilhável

Qualquer quantidade de ObjectIds pode coexistir simultaneamente.

Leituras simultâneas são permitidas desde que nenhuma operação de escrita esteja ativa sobre o mesmo Domain.

A linguagem deve impedir que leituras e escritas concorrentes produzam data races.

---

## Princípio 5 — O compilador realiza a inferência

Sempre que possível, o compilador deve inferir automaticamente:

* Domains;
* relações entre objetos;
* verificações de segurança;
* otimizações.

A ergonomia da linguagem não deve comprometer suas garantias.

---

# 8. Modelo de Execução

Programas Capi são organizados em Domains independentes.

Cada Domain possui:

* memória própria;
* lifetime próprio;
* sincronização própria;
* destruição própria.

A unidade de gerenciamento de memória da linguagem é o Domain.

A unidade de identidade da linguagem é o ObjectId.

A unidade de comportamento da linguagem é o Objeto.

---

# 9. Concorrência

A concorrência é definida em termos de Domains.

Domains independentes podem ser modificados simultaneamente.

Um mesmo Domain não pode sofrer mutações concorrentes sem mecanismos explícitos definidos pela linguagem.

O compilador deve garantir, sempre que possível, que operações concorrentes sobre o mesmo Domain sejam detectadas em tempo de compilação.

Assim, o isolamento concorrente pertence ao Domain e não aos objetos individuais.

---

# 10. Filosofia da Linguagem

A Capi não pretende adaptar Rust à Orientação a Objetos.

Também não pretende reproduzir Java, C++, C# ou qualquer outra linguagem existente.

A linguagem adota um novo modelo conceitual no qual:

* objetos representam entidades lógicas;
* Domains representam entidades físicas;
* memória pertence aos Domains;
* identidade pertence aos objetos;
* mutação pertence aos Domains.

Toda a arquitetura da linguagem deriva dessa separação.

---

# 11. Princípio da Separação entre Garantias e Implementação

Todos os documentos da especificação da Capi devem distinguir explicitamente:

* as garantias oferecidas pela linguagem;
* os mecanismos utilizados para implementá-las.

As garantias constituem parte permanente da especificação.

Os mecanismos de implementação pertencem ao compilador, ao Modelo de Memória ou à Infraestrutura de Execução, e podem evoluir ao longo do tempo, desde que preservem integralmente essas garantias.

Este princípio aplica-se a todos os documentos da especificação da Capi.

Cada documento deverá deixar claro quando está descrevendo uma propriedade garantida pela linguagem e quando está descrevendo uma possível estratégia de implementação.

---

# 12. Não Objetivos

A Capi não pretende:

* substituir linguagens de script;
* competir com linguagens interpretadas para prototipação rápida;
* utilizar máquinas virtuais como ambiente principal de execução;
* depender de ambientes de execução instaláveis para executar programas;
* Os mecanismos que escapam das garantias estáticas da linguagem serão definidos em documentos específicos e não fazem parte deste documento;
* reproduzir integralmente os modelos de C++, Java ou Rust.

---

# 13. Organização da Especificação

Os detalhes da linguagem são definidos em documentos específicos.

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

Todos os documentos posteriores devem respeitar os princípios definidos nesta especificação.
