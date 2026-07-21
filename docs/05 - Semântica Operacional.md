# Capi — A Próxima Geração da Orientação a Objetos

# Documento 05 — Semântica Operacional

Versão: 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a **Semântica Operacional** da linguagem Capi.

Enquanto os documentos anteriores estabelecem os princípios arquiteturais, o Sistema de Tipos, o Modelo de Memória e a Sintaxe da linguagem, este documento descreve o significado operacional de cada construção da linguagem.

Seu objetivo é responder à pergunta:

> **Como um programa escrito em Capi deve se comportar?**

Este documento não especifica como um compilador deve implementar esse comportamento, mas define quais resultados observáveis devem ser preservados.

---

## 1.2 Escopo

Este documento especifica:

* criação de objetos;
* criação e destruição de Domains;
* resolução de chamadas;
* despacho dinâmico;
* regras de mutação;
* avaliação de expressões;
* controle de fluxo;
* concorrência;
* garantias semânticas da linguagem.

Não fazem parte deste documento:

* detalhes da representação binária;
* formato do código objeto;
* geração de código;
* otimizações específicas;
* arquitetura interna do compilador.

---

## 1.3 Relação com os Documentos Anteriores

Este documento complementa diretamente:

| Documento    | Responsabilidade        |
| ------------ | ----------------------- |
| Documento 01 | Fundamentos conceituais |
| Documento 02 | Sistema de Tipos        |
| Documento 03 | Modelo de Memória       |
| Documento 04 | Sintaxe da Linguagem    |

Todos os conceitos utilizados neste documento assumem que os requisitos estabelecidos nesses documentos já foram satisfeitos.

---

## 1.4 Garantias da Semântica

A Semântica Operacional não cria novas garantias.

Ela apenas formaliza o comportamento necessário para preservar:

* Sistema de Tipos;
* Modelo de Memória;
* Segurança de Memória;
* Determinismo da desalocação;
* Integridade dos Domains;
* Integridade das identidades (`ObjectId`).

---

# 2. Princípios Gerais

A Semântica Operacional da Capi é construída sobre um conjunto reduzido de princípios fundamentais.

Esses princípios orientam todas as regras descritas neste documento.

---

## 2.1 Objetos Representam Comportamento

Objetos representam entidades lógicas compostas por:

* identidade;
* comportamento;
* estado lógico.

O armazenamento físico desse estado pertence exclusivamente ao Domain ao qual o objeto está associado.

Consequentemente, nenhuma operação realizada sobre um objeto pode violar as garantias estabelecidas pelo Modelo de Memória.

---

## 2.2 Domains Controlam a Memória

Toda memória utilizada pela aplicação pertence exatamente a um Domain.

O Domain é responsável por:

* alocação;
* organização física;
* lifetime;
* destruição;
* autorização de escrita.

Objetos jamais controlam diretamente sua própria memória.

---

## 2.3 Escrita Pertence ao Domain

Toda operação de escrita ocorre conceitualmente sobre um Domain.

Chamadas de métodos mutantes representam apenas a interface utilizada pelo programador.

Antes de qualquer modificação de estado, a linguagem deve garantir que exista uma **Capacidade de Mutação** válida para o Domain correspondente.

Essa garantia é obrigatória independentemente do mecanismo utilizado para implementá-la.

---

## 2.4 Leitura Não Altera Estado

Operações de leitura não modificam o estado persistente dos objetos.

Múltiplas leituras podem coexistir simultaneamente, desde que nenhuma regra do Modelo de Memória seja violada.

---

## 2.5 Garantias Independem da Implementação

A especificação distingue explicitamente:

* as garantias oferecidas pela linguagem;
* os mecanismos utilizados para implementá-las.

Por exemplo, a exclusividade da escrita é uma garantia permanente da linguagem.

O mecanismo utilizado para implementá-la — análise estática, capacidades lineares, regiões, ou outro modelo equivalente — pertence exclusivamente à implementação do compilador.

---

# 3. Modelo de Execução

Este capítulo descreve o modelo abstrato de execução adotado pela Capi.

Esse modelo não representa uma arquitetura obrigatória para o compilador, mas uma forma de descrever formalmente o comportamento esperado da linguagem.

---

## 3.1 Estrutura Geral

Durante a execução de um programa, os elementos fundamentais relacionam-se da seguinte forma:

```text
Programa
    │
    ▼
Módulos
    │
    ▼
Domains
    │
    ▼
Objetos
    │
    ▼
Estado
```

Essa representação é conceitual e não impõe qualquer organização física específica.

---

## 3.2 Estado Global

O estado observável de um programa é definido pelo conjunto formado por:

* Domains ativos;
* objetos existentes;
* identidades (`ObjectId`);
* valores armazenados;
* fluxo de execução.

Qualquer transformação produzida pelo programa deve preservar as garantias estabelecidas pelos documentos anteriores.

---

## 3.3 Estado dos Domains

Cada Domain possui, conceitualmente:

* um conjunto de objetos;
* uma organização física de memória;
* um lifetime;
* uma autorização de escrita;
* um estado interno consistente.

Essas informações pertencem ao Modelo de Memória e não precisam possuir representação explícita em tempo de execução.

---

## 3.4 Estado dos Objetos

Cada objeto possui:

* uma identidade (`ObjectId`);
* um tipo;
* um conjunto de comportamentos;
* um estado lógico armazenado no Domain.

O objeto nunca é responsável pela alocação ou desalocação da memória correspondente ao seu estado.

---

## 3.5 Evolução do Estado

Toda instrução executada pela linguagem pode ser interpretada como uma transformação do estado atual do programa em um novo estado.

Essa transformação deve respeitar simultaneamente:

* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* todas as garantias de segurança estabelecidas pela linguagem.

Nenhuma transição de estado pode produzir comportamento indefinido quando o programa estiver em conformidade com a especificação.

---

# 4. Ciclo de Vida do Programa

Este capítulo descreve as etapas conceituais da execução de um programa Capi.

O ciclo de vida definido aqui representa o comportamento observável da linguagem e não impõe uma arquitetura específica ao compilador.

---

## 4.1 Inicialização

A execução de um programa inicia após a conclusão das etapas de compilação e carregamento.

Durante a inicialização, a infraestrutura mínima da linguagem deve:

* preparar os módulos necessários;
* inicializar estruturas internas de suporte;
* criar os Domains iniciais exigidos pelo programa;
* inicializar constantes globais;
* preparar o ambiente de execução.

A forma como essas etapas são implementadas é responsabilidade do compilador e da infraestrutura de execução.

---

## 4.2 Execução da Função Principal

Após a inicialização, a execução é transferida para a função de entrada do programa.

Exemplo conceitual:

```capi
function main() : Unit {

    println("Olá Mundo");

}
```

A assinatura exata da função principal poderá variar conforme o ambiente de execução, permanecendo definida pela especificação da plataforma.

---

## 4.3 Execução

Durante a execução, o programa pode:

* criar objetos;
* criar ou utilizar Domains;
* executar chamadas de métodos;
* avaliar expressões;
* modificar estados;
* produzir efeitos externos.

Cada operação corresponde a uma transição válida entre estados do programa.

---

## 4.4 Finalização

Ao término da execução:

* a função principal retorna;
* inicia-se o processo de encerramento do programa;
* os Domains restantes são destruídos;
* recursos externos são liberados.

Toda desalocação ocorre de maneira determinística.

---

## 4.5 Encerramento

Após a destruição do último Domain, considera-se encerrada a execução do programa.

Nenhum `ObjectId` permanece válido após esse momento.

---

# 5. Criação de Objetos

Este capítulo formaliza o comportamento associado à criação de novas instâncias.

A sintaxe para criação foi definida no Documento 04.

Aqui define-se seu significado operacional.

---

## 5.1 Construção

A criação de um objeto corresponde à execução das seguintes etapas conceituais:

1. seleção do Domain de destino;
2. reserva da memória necessária;
3. inicialização do estado;
4. criação do `ObjectId`;
5. associação entre identidade e estado;
6. execução do construtor;
7. disponibilização do objeto ao programa.

O compilador pode reorganizar internamente essas etapas desde que preserve o comportamento observável.

---

## 5.2 Seleção do Domain

Todo objeto pertence exatamente a um Domain.

A escolha do Domain pode ocorrer:

* por inferência do compilador;
* por diretiva explícita da linguagem;
* por regras definidas pela plataforma.

Independentemente da estratégia utilizada, essa associação torna-se parte permanente do objeto durante todo o seu lifetime.

---

## 5.3 Inicialização

Antes que qualquer referência ao objeto seja disponibilizada, todos os seus campos devem estar inicializados.

A linguagem não permite que objetos parcialmente inicializados sejam observáveis pelo programa.

Essa garantia elimina uma classe importante de erros relacionados à leitura de memória não inicializada.

---

## 5.4 Criação da Identidade

Cada objeto recebe exatamente um `ObjectId`.

Esse identificador:

* é único;
* permanece imutável durante toda a existência do objeto;
* identifica exclusivamente aquela instância.

A identidade não pode ser alterada por código da aplicação.

---

## 5.5 Execução do Construtor

Após a inicialização física da memória, o construtor é executado.

Durante essa execução:

* o objeto já possui identidade válida;
* seus campos podem ser inicializados;
* métodos auxiliares podem ser chamados, respeitando as restrições definidas pela Semântica Operacional.

Ao término do construtor, considera-se que o objeto está completamente inicializado.

---

# 6. Lifetime dos Objetos

O lifetime representa o intervalo durante o qual um objeto pode ser observado pelo programa.

Na Capi, o lifetime dos objetos é consequência direta do lifetime do Domain ao qual pertencem.

---

## 6.1 Nascimento

O lifetime inicia imediatamente após a conclusão bem-sucedida da criação do objeto.

A partir desse momento:

* o `ObjectId` torna-se válido;
* o objeto pode participar do grafo de objetos;
* chamadas de métodos tornam-se permitidas.

---

## 6.2 Validade

Um objeto permanece válido enquanto o Domain correspondente permanecer ativo.

Durante esse período:

* seu estado pode ser consultado;
* sua identidade permanece constante;
* operações de escrita obedecem às regras da Capacidade de Mutação.

---

## 6.3 Destruição

Quando o Domain é destruído:

* todos os objetos pertencentes a ele deixam simultaneamente de existir;
* todos os `ObjectId` associados tornam-se inválidos;
* toda a memória do Domain é liberada de forma determinística.

Não existe destruição individual de objetos.

Essa característica distingue a Capi das linguagens baseadas em ownership individual ou coleta de lixo.

---

## 6.4 Objetos Inacessíveis

Objetos inacessíveis por referências da aplicação permanecem existindo enquanto o Domain existir.

A inacessibilidade lógica não implica destruição.

A remoção da memória depende exclusivamente do lifetime do Domain.

---

## 6.5 Destrutores

Classes podem declarar um destrutor associado ao encerramento de seu lifetime.

O destrutor é executado durante o processo de destruição do Domain ao qual o objeto pertence, imediatamente antes da liberação definitiva da memória correspondente.

Durante a execução do destrutor:

o objeto ainda é considerado válido;
sua identidade (ObjectId) permanece válida;
os objetos pertencentes ao mesmo Domain continuam acessíveis, respeitadas as regras desta especificação.

Todos os destrutores de um Domain devem ser executados antes da liberação de sua memória.

A ordem de execução dos destrutores não faz parte da especificação da linguagem e pertence à implementação do compilador e da infraestrutura mínima da linguagem, desde que todas as garantias estabelecidas pelo Modelo de Memória sejam preservadas.

Destrutores não podem ser invocados explicitamente pelo código da aplicação.

---

## 6.6 Garantias

O modelo de lifetime da Capi garante que:

* não existem objetos parcialmente destruídos;
* não existem referências para memória já liberada;
* não existe dupla destruição (*double free*);
* não existem ciclos de ownership;
* a validade das identidades acompanha rigorosamente a validade do Domain correspondente.

---

# 7. Chamada de Métodos

Este capítulo define a semântica da invocação de métodos na linguagem Capi.

Embora a sintaxe seja semelhante à das linguagens orientadas a objetos tradicionais, a execução deve preservar todas as garantias estabelecidas pelo Sistema de Tipos e pelo Modelo de Memória.

---

## 7.1 Invocação

Uma chamada de método representa a transferência temporária do fluxo de execução para um comportamento associado ao tipo do objeto.

Exemplo:

```capi
conta.depositar(100);
```

Conceitualmente, uma chamada envolve as seguintes etapas:

1. avaliação da expressão que identifica o objeto;
2. validação da identidade (`ObjectId`);
3. resolução do método aplicável;
4. validação das regras do Sistema de Tipos;
5. validação das regras de mutação, quando aplicável;
6. execução do método;
7. retorno do controle ao chamador.

---

## 7.2 Resolução Estática

Quando o método pode ser determinado integralmente em tempo de compilação, sua resolução é considerada estática.

Nesses casos, o compilador pode realizar otimizações como:

* eliminação de indireções;
* inline;
* especialização de chamadas.

Essas otimizações não alteram o comportamento observável do programa.

---

## 7.3 Resolução Dinâmica

Quando o tipo concreto do objeto somente pode ser determinado durante a execução, a chamada utiliza despacho dinâmico.

Exemplo:

```capi
Animal animal = obterAnimal();

animal.emitirSom();
```

A implementação poderá utilizar tabelas virtuais (*vtables*) ou mecanismos equivalentes.

O mecanismo empregado pertence exclusivamente à implementação do compilador.

---

## 7.4 Método Receptor

Toda chamada de método possui um objeto receptor.

Conceitualmente:

```capi
conta.depositar(100);
```

equivale a:

```text
depositar(conta, 100)
```

A forma como o objeto receptor é representado internamente não faz parte da especificação.

---

## 7.5 Validação da Mutação

Caso o método realize escrita no estado persistente do objeto, o compilador deverá garantir que exista uma Capacidade de Mutação válida para o Domain correspondente.

Na ausência dessa garantia, o programa é considerado inválido e deve ser rejeitado durante a compilação.

---

# 8. Resolução de Tipos

Este capítulo define como o compilador resolve os tipos utilizados pelo programa.

Toda resolução deve preservar as garantias estabelecidas pelo Sistema de Tipos.

---

## 8.1 Inferência

Sempre que possível, o compilador poderá inferir automaticamente tipos omitidos pelo programador.

Exemplo:

```capi
let cliente = new Cliente();
```

é semanticamente equivalente a:

```capi
Cliente cliente = new Cliente();
```

A inferência jamais poderá alterar o significado do programa.

---

## 8.2 Conversões

Conversões entre tipos somente são permitidas quando preservam as garantias da linguagem.

Podem existir:

* conversões implícitas;
* conversões explícitas;
* conversões proibidas.

As regras específicas são definidas pelo Sistema de Tipos.

---

## 8.3 Upcast

A conversão de um subtipo para um supertipo é sempre segura.

Exemplo:

```capi
ContaEspecial especial = new ContaEspecial();

Conta conta = especial;
```

Essa operação preserva a identidade do objeto.

Nenhuma nova instância é criada.

Operacionalmente, um upcast não modifica o objeto nem sua identidade. Apenas altera a forma como o compilador interpreta estaticamente a referência. Nenhuma nova instância é criada e nenhuma alteração ocorre no estado do objeto.

---

## 8.4 Downcast

Conversões da superclasse para subclasses somente são permitidas quando sua segurança puder ser comprovada.

Essa comprovação poderá ocorrer através de:

* hierarquias seladas (*sealed*);
* verificações explícitas;
* mecanismos definidos pela Semântica Operacional.

Quando não houver comprovação suficiente, o compilador deverá rejeitar a conversão ou exigir mecanismos específicos previstos pela linguagem.

Em classes sealed, o compilador conhece previamente todos os subtipos possíveis, podendo validar determinadas conversões de forma estática. Para hierarquias abertas, essa garantia pode não ser possível.

---

## 8.5 Tipos Opcionais

Operações envolvendo `Optional<T>` devem considerar explicitamente as possibilidades de presença ou ausência de valor.

O compilador poderá exigir tratamento explícito dessas situações.

Essa característica elimina a necessidade de referências nulas.

---

# 9. Resolução de Sobrecarga

A linguagem permite que múltiplos métodos compartilhem o mesmo nome, desde que possam ser distinguidos por suas assinaturas.

Este capítulo define como o compilador determina qual implementação deve ser utilizada.

---

## 9.1 Assinatura

A assinatura de um método é composta por:

* nome;
* quantidade de parâmetros;
* tipos dos parâmetros.

O tipo de retorno não participa da diferenciação de assinaturas.

---

## 9.2 Escolha da Sobrecarga

Durante a compilação, o compilador seleciona a implementação mais específica compatível com os argumentos fornecidos.

Caso exista ambiguidade, o programa deve ser rejeitado.

Exemplo:

```capi
imprimir(10);
```

A resolução depende da existência de uma assinatura compatível.

---

## 9.3 Conversões Implícitas

Conversões implícitas somente poderão ser consideradas durante a resolução quando não introduzirem ambiguidades.

Na existência de múltiplas alternativas igualmente válidas, o compilador deverá emitir erro de compilação.

---

## 9.4 Métodos Genéricos

Métodos genéricos participam normalmente da resolução de sobrecarga.

A inferência dos parâmetros genéricos ocorre antes da seleção definitiva da implementação.

---

## 9.5 Determinismo

A resolução de sobrecarga deve ser completamente determinística.

Um mesmo programa deve produzir exatamente a mesma escolha de métodos em qualquer compilação compatível com a especificação da linguagem.

A implementação do compilador não pode introduzir comportamentos dependentes da plataforma ou da ordem interna de análise.

---

# 10. Despacho Dinâmico

O despacho dinâmico (*dynamic dispatch*) é o mecanismo que permite que chamadas realizadas através de um supertipo sejam resolvidas para a implementação correspondente ao tipo concreto do objeto.

A Capi suporta despacho dinâmico preservando as garantias do Sistema de Tipos e do Modelo de Memória.

A especificação define apenas o comportamento observável, não impondo um mecanismo específico de implementação.

---

## 10.1 Objetivo

Considere o exemplo:

```capi
Animal animal = new Cachorro();

animal.emitirSom();
```

Embora a variável possua o tipo estático `Animal`, a implementação executada deverá ser aquela definida por `Cachorro`.

Esse comportamento constitui o despacho dinâmico.

---

## 10.2 Tipo Estático e Tipo Dinâmico

Durante a execução, todo objeto possui simultaneamente:

* um **tipo estático**, conhecido pelo compilador;
* um **tipo dinâmico**, correspondente à classe concreta da instância.

A resolução dinâmica utiliza o tipo dinâmico para determinar a implementação correta do método.

---

## 10.3 Métodos Elegíveis

Participam do despacho dinâmico apenas métodos passíveis de sobrescrita.

Métodos declarados como:

* `final`;
* `static`;
* construtores;

não participam do despacho dinâmico.

Esses métodos podem ser resolvidos integralmente em tempo de compilação.

---

## 10.4 Implementação

A linguagem não exige nenhuma estratégia específica para implementar o despacho dinâmico.

Entre as estratégias possíveis encontram-se:

* Virtual Tables (vtables);
* tabelas de despacho;
* caches de chamadas;
* especialização em tempo de compilação;
* qualquer mecanismo equivalente.

Todas devem produzir exatamente o mesmo comportamento observável.

---

## 10.5 Garantias

Independentemente da implementação utilizada, o despacho dinâmico deve preservar:

* identidade do objeto;
* regras do Sistema de Tipos;
* Modelo de Memória;
* Capacidade de Mutação;
* comportamento polimórfico definido pela especificação.

---

# 11. Domains em Execução

Este capítulo descreve o comportamento operacional dos Domains durante a execução do programa.

Enquanto o Documento 03 define o Modelo de Memória, este capítulo estabelece como os Domains participam da execução.

---

## 11.1 Criação

Todo objeto pertence exatamente a um Domain.

A criação de um Domain pode ocorrer:

* automaticamente, por inferência do compilador;
* por diretivas explícitas da linguagem;
* por mecanismos definidos pela biblioteca padrão.

A estratégia utilizada pertence à implementação.

---

## 11.2 Associação

Após criado, um objeto permanece associado ao mesmo Domain durante todo o seu lifetime.

Essa associação é imutável.

O compilador deve garantir que nenhuma operação viole essa propriedade.

---

## 11.3 Estado do Domain

Conceitualmente, cada Domain mantém:

* seus objetos;
* sua organização física de memória;
* seu lifetime;
* seu estado de mutação.

A forma como essas informações são representadas internamente não faz parte da especificação.

---

## 11.4 Destruição

Quando um Domain atinge o final de seu lifetime:

* toda sua memória é liberada;
* todos os objetos deixam de existir simultaneamente;
* todos os `ObjectId` tornam-se inválidos.

Essa operação é indivisível do ponto de vista da Semântica Operacional.

---

## 11.5 Migração

Objetos não podem migrar entre Domains. A identidade de um objeto está permanentemente vinculada ao Domain onde foi criado. Qualquer operação que necessite de dados em outro Domain deve criar uma nova instância no Domain de destino, com nova identidade. A transferência de dados é uma cópia, não uma movimentação.

Caso a linguagem ofereça mecanismos explícitos de migração, estes deverão:

* preservar a segurança de memória;
* preservar o Sistema de Tipos;
* preservar a coerência das identidades;
* respeitar as garantias definidas pelo Modelo de Memória.

A sintaxe e os mecanismos dessa funcionalidade serão definidos em documento específico.

---

# 12. Associação entre Objetos e Domains

Este capítulo formaliza a relação entre identidades, objetos e Domains durante a execução.

---

## 12.1 Associação Permanente

Cada objeto pertence exatamente a um único Domain.

Essa associação é estabelecida durante a criação da instância e permanece válida até sua destruição.

Não existe estado intermediário no qual um objeto pertença simultaneamente a dois Domains.

---

## 12.2 ObjectId

Todo `ObjectId` referencia exatamente um objeto.

Enquanto o objeto permanecer válido:

* seu `ObjectId` permanece estável;
* sua identidade não pode ser alterada;
* todas as referências observam a mesma identidade.

A invalidação do `ObjectId` ocorre exclusivamente com a destruição do Domain correspondente.

---

## 12.3 Compartilhamento

Múltiplos `ObjectId` podem referenciar o mesmo objeto.

Esse compartilhamento não implica compartilhamento da autoridade de escrita.

A escrita permanece subordinada às regras da Capacidade de Mutação.

---

## 12.4 Grafos de Objetos

Objetos podem formar grafos arbitrários.

São permitidos:

* ciclos;
* referências cruzadas;
* referências múltiplas;
* estruturas recursivas.

Essas estruturas não afetam o gerenciamento de memória, uma vez que a propriedade da memória pertence ao Domain, e não aos objetos.

---

## 12.5 Integridade

A Semântica Operacional garante que:

* nenhum `ObjectId` referencia memória inexistente;
* nenhuma identidade muda durante o lifetime do objeto;
* nenhuma operação pode produzir objetos "órfãos";
* nenhum objeto pode existir fora de um Domain válido.

Essas propriedades constituem uma das principais garantias da arquitetura da Capi.

---

# 13. Capacidade de Mutação

A Capacidade de Mutação é um conceito abstrato do Sistema de Tipos que representa a autorização exclusiva para modificar o estado de um Domain. Ela não constitui um objeto da aplicação nem um tipo manipulável pelo programador. Sua existência é inferida e verificada pelo compilador durante a análise semântica.

A Capacidade de Mutação é o mecanismo conceitual responsável por garantir que operações de escrita ocorram de forma segura e determinística.

Este capítulo formaliza seu comportamento operacional.

A forma como essa capacidade é implementada pertence exclusivamente ao compilador e à infraestrutura mínima da linguagem.

---

## 13.1 Objetivo

A Capi estabelece que:

* múltiplas leituras podem ocorrer simultaneamente;
* múltiplas identidades podem referenciar o mesmo objeto;
* grafos arbitrários de objetos são permitidos.

Entretanto, operações de escrita devem permanecer exclusivas para preservar a integridade do estado do Domain.

A Capacidade de Mutação é a garantia utilizada para alcançar esse objetivo.

---

## 13.2 Exclusividade

Em qualquer instante lógico da execução, um Domain pode possuir no máximo uma Capacidade de Mutação ativa.

Consequentemente:

* duas operações incompatíveis de escrita não podem ocorrer simultaneamente sobre o mesmo Domain;
* uma operação de escrita não pode coexistir com operações de leitura que violem as garantias do Modelo de Memória.

O compilador deve rejeitar programas que não permitam comprovar essa propriedade.

---

## 13.3 Escopo

A Capacidade de Mutação possui escopo limitado.

Ela permanece válida apenas durante a execução da operação que a adquiriu ou durante o bloco de código autorizado a utilizá-la.

Ao término desse escopo, considera-se automaticamente liberada.

Esse comportamento impede retenções indefinidas da autorização de escrita.

---

## 13.4 Inferência

Na maioria dos programas, a Capacidade de Mutação é inferida automaticamente.

Exemplo:

```capi id="4mta8d"
conta.depositar(100);
```

Embora nenhuma capacidade seja visível ao programador, o compilador deverá verificar que todas as condições necessárias para a escrita estão satisfeitas antes da execução da operação.

---

## 13.5 Garantias

A Capacidade de Mutação garante que:

* operações incompatíveis de escrita não coexistam;
* a integridade do Domain seja preservada;
* o estado dos objetos permaneça consistente;
* não ocorram *data races* sobre o mesmo Domain.

Essas garantias são independentes da estratégia utilizada pelo compilador para implementá-las.

---

# 14. Operações de Escrita

Este capítulo define quando uma operação que modifica o estado de um objeto é considerada válida.

---

## 14.1 Escrita Válida

Uma operação de escrita é considerada válida quando:

* o objeto pertence a um Domain válido;
* o `ObjectId` permanece válido;
* existe uma Capacidade de Mutação ativa para esse Domain;
* todas as regras do Sistema de Tipos são satisfeitas.

Na ausência de qualquer uma dessas condições, a operação é inválida.

---

## 14.2 Escrita em Campos

A modificação de qualquer campo persistente de um objeto constitui uma operação de escrita.

Exemplo:

```capi id="h79rbl"
saldo = saldo + valor;
```

Essa operação está sujeita às mesmas regras aplicáveis à execução de métodos mutantes.

---

## 14.3 Escrita por Métodos

Métodos capazes de alterar o estado persistente do objeto também representam operações de escrita.

Exemplo:

```capi id="afbl6l"
conta.depositar(100);
```

Conceitualmente, a modificação ocorre sobre o Domain correspondente ao objeto.

---

## 14.4 Operações Atômicas

Operações atômicas são operações observadas pelos demais fluxos de execução como indivisíveis. A biblioteca padrão poderá fornecer primitivas atômicas específicas, cuja semântica será definida em documento próprio. Essas operações complementam, mas não substituem, as garantias fornecidas pela Capacidade de Mutação.

A especificação permite que determinadas operações sejam executadas de forma atômica.

A definição das primitivas atômicas pertence à biblioteca padrão ou a documentos específicos.

Independentemente da implementação adotada, todas as garantias da Capacidade de Mutação devem ser preservadas.

---

## 14.5 Rejeição de Programas Inválidos

Quando o compilador não puder comprovar que uma operação de escrita preserva todas as garantias da linguagem, o programa deverá ser rejeitado.

A linguagem não admite comportamento indefinido decorrente de conflitos de mutação em código seguro.

---

# 15. Avaliação de Expressões

Este capítulo define como expressões são avaliadas durante a execução.

A ordem de avaliação deve produzir resultados determinísticos e compatíveis com o Sistema de Tipos.

---

## 15.1 Ordem de Avaliação

Expressões são avaliadas da esquerda para a direita, respeitando as regras de precedência definidas pelo Documento 04.

Exemplo:

```capi id="u8m88n"
a() + b() * c()
```

A ordem de invocação das funções permanece observável e não pode ser alterada quando isso modificar o comportamento do programa.

---

## 15.2 Curto-Circuito

Operadores lógicos de curto-circuito interrompem a avaliação quando o resultado já puder ser determinado.

Exemplo:

```capi id="53sj9o"
if (a && b) {

}
```

Caso `a` seja falso, `b` não será avaliado.

Essa propriedade faz parte da semântica da linguagem.

---

## 15.3 Temporários

Durante a avaliação de uma expressão, o compilador poderá criar valores temporários.

Esses temporários:

* não possuem identidade própria;
* obedecem às regras do Sistema de Tipos;
* permanecem válidos apenas durante a avaliação da expressão correspondente.

A existência desses temporários não altera o comportamento observável do programa.

---

## 15.4 Chamadas Aninhadas

Chamadas de métodos ou funções podem ocorrer como parte da avaliação de expressões.

Exemplo:

```capi id="8fx8du"
calcular(totalizar(cliente), desconto());
```

Cada chamada é avaliada segundo as mesmas regras estabelecidas para invocação de métodos e funções.

---

## 15.5 Determinismo

A avaliação de expressões deve ser determinística.

Um mesmo programa, executado sob as mesmas condições, deve produzir os mesmos resultados observáveis, independentemente das otimizações realizadas pelo compilador.

Qualquer otimização deve preservar integralmente a semântica definida por este documento.

---

# 16. Controle de Fluxo

Este capítulo define o comportamento operacional das estruturas de controle da linguagem.

Todas as construções descritas preservam o fluxo determinístico de execução e obedecem às garantias estabelecidas pelo Sistema de Tipos e pelo Modelo de Memória.

---

## 16.1 Blocos

Um bloco representa uma sequência ordenada de instruções executadas no mesmo contexto léxico.

Exemplo:

```capi
{

    instrucao1();

    instrucao2();

    instrucao3();

}
```

As instruções são executadas sequencialmente, salvo quando alteradas explicitamente por estruturas de controle.

---

## 16.2 Estruturas Condicionais

A instrução `if` avalia sua condição exatamente uma vez.

Dependendo do resultado, apenas um dos blocos será executado.

Exemplo:

```capi
if (saldo > limite) {

    bloquear();

}
else {

    liberar();

}
```

Nenhum dos blocos pode ser executado parcialmente.

---

## 16.3 Estruturas Iterativas

As estruturas `while`, `for` e `foreach` executam repetidamente um bloco de instruções enquanto suas condições permanecerem válidas.

Cada iteração constitui uma nova execução lógica do bloco correspondente.

A criação de variáveis temporárias, quando necessária, pertence à implementação do compilador.

---

## 16.4 Interrupção

A instrução `break` encerra imediatamente a execução da estrutura iterativa ou da instrução `switch` mais interna.

Após sua execução, o fluxo continua na primeira instrução subsequente.

---

## 16.5 Continuação

A instrução `continue` interrompe apenas a iteração corrente.

A próxima iteração inicia imediatamente, respeitando as regras da estrutura de repetição correspondente.

---

# 17. Pattern Matching

A Capi oferece um mecanismo de *Pattern Matching* para seleção de comportamento baseada na estrutura lógica dos valores.

Este mecanismo complementa as estruturas tradicionais de controle e permite que determinadas verificações sejam realizadas pelo compilador.

---

## 17.1 Avaliação

Uma expressão `match` avalia inicialmente seu valor de entrada.

Em seguida, cada padrão é analisado na ordem em que foi declarado.

A primeira correspondência válida determina o bloco executado.

Nenhum outro padrão será considerado após uma correspondência bem-sucedida.

---

## 17.2 Exaustividade

Sempre que possível, o compilador deverá verificar que todas as possibilidades foram consideradas.

Essa verificação é obrigatória para estruturas cujo conjunto de possibilidades possa ser determinado estaticamente.

Quando a exaustividade não puder ser comprovada, o compilador poderá exigir um caso padrão (`default`) ou equivalente.

---

## 17.3 Classes Seladas

Quando aplicado a hierarquias de classes `sealed`, o compilador conhece previamente todos os subtipos possíveis.

Consequentemente, pode verificar que todos os casos foram tratados.

Exemplo conceitual:

```capi
match (forma) {

    case Circulo(c):

        ...

    case Retangulo(r):

        ...

}
```

Caso uma nova subclasse seja adicionada à hierarquia, todas as expressões `match` afetadas deverão ser reavaliadas durante a compilação.

---

## 17.4 Optional e Result

Os tipos `Optional<T>` e `Result<T,E>` integram naturalmente o mecanismo de *Pattern Matching*.

Exemplo:

```capi
match (resultado) {

    case Success(valor):

        ...

    case Failure(erro):

        ...

}
```

O compilador poderá exigir que todas as possibilidades sejam tratadas explicitamente.

Essa exigência elimina classes inteiras de erros relacionados à ausência de tratamento para situações previstas.

---

## 17.5 Segurança

O *Pattern Matching* jamais poderá produzir conversões de tipo incompatíveis.

Toda variável introduzida por um padrão possui tipo conhecido e validado durante a compilação.

---

# 18. Modelo de Concorrência

Este capítulo estabelece a semântica geral da concorrência na linguagem Capi.

A sintaxe utilizada para criação de fluxos concorrentes não faz parte desta versão da especificação e será definida em documento próprio ou em versão futura.

As regras descritas neste capítulo definem apenas as garantias que qualquer mecanismo de concorrência deverá preservar.

A biblioteca padrão poderá oferecer primitivas para criação de fluxos concorrentes, como spawn, pools de execução e mecanismos de comunicação entre Domains. A sintaxe definitiva dessas construções será definida em documento próprio.

---

## 18.1 Concorrência Baseada em Domains

A unidade fundamental de sincronização da linguagem é o **Domain**.

As garantias de concorrência são estabelecidas sobre Domains e não sobre objetos individuais.

Essa decisão decorre diretamente do Modelo de Memória definido no Documento 03.

---

## 18.2 Execução Paralela

Domains independentes podem ser executados concorrentemente.

Como cada Domain controla sua própria memória e sua própria Capacidade de Mutação, operações realizadas em Domains distintos não interferem entre si.

A implementação poderá explorar esse paralelismo automaticamente.

---

## 18.3 Exclusividade da Escrita

Em qualquer instante lógico da execução, um Domain possui no máximo uma Capacidade de Mutação ativa.

Consequentemente, duas operações incompatíveis de escrita sobre o mesmo Domain nunca poderão ocorrer simultaneamente.

Essa propriedade elimina *data races* em código seguro.

---

## 18.4 Sincronização

Quando múltiplos fluxos de execução necessitarem acessar o mesmo Domain, a sincronização deverá preservar todas as garantias estabelecidas pelo Sistema de Tipos e pelo Modelo de Memória.

Os mecanismos específicos de sincronização — tais como primitivas da biblioteca padrão, abstrações de concorrência ou recursos da plataforma — pertencem à implementação da linguagem e não fazem parte desta especificação.

---

## 18.5 Garantias

Independentemente da estratégia de implementação adotada, a concorrência na Capi deve preservar obrigatoriamente:

* ausência de *data races* em código seguro;
* integridade dos Domains;
* validade das identidades (`ObjectId`);
* determinismo do gerenciamento de memória;
* exclusividade da Capacidade de Mutação;
* todas as garantias estabelecidas pelos Documentos 01, 02 e 03.

---

# 19. Código `unsafe`

Embora a Capi tenha sido projetada para oferecer segurança de memória estática, determinados cenários exigem operações que não podem ser verificadas integralmente pelo compilador.

Para esses casos, a linguagem oferece o bloco `unsafe`.

Este capítulo define sua semântica operacional.

---

## 19.1 Objetivo

O bloco `unsafe` permite executar operações fora das garantias verificadas pelo Sistema de Tipos e pela Semântica Operacional.

Seu uso é destinado principalmente a:

* interoperabilidade com código nativo;
* manipulação direta de memória;
* acesso a recursos específicos da plataforma;
* implementação de bibliotecas de baixo nível.

---

## 19.2 Escopo

As garantias da linguagem permanecem válidas para todo o programa, exceto dentro do escopo explicitamente marcado como `unsafe`.

Exemplo:

```capi
unsafe {

    nativeCall();

}
```

Todo código potencialmente inseguro permanece claramente identificado.

---

## 19.3 Responsabilidade

Dentro de um bloco `unsafe`, a responsabilidade pela preservação das garantias da linguagem passa a ser do programador.

A implementação deixa de verificar determinadas propriedades, incluindo, quando aplicável:

* validade de ponteiros externos;
* correção de conversões de baixo nível;
* integridade de estruturas manipuladas externamente.

---

## 19.4 Limites

Mesmo dentro de um bloco `unsafe`, permanecem obrigatórias todas as regras da linguagem que não dependam da verificação de segurança desabilitada.

O uso de `unsafe` não altera:

* a sintaxe da linguagem;
* o Modelo de Memória;
* a estrutura dos Domains;
* o comportamento de código seguro externo ao bloco.

---

## 19.5 Interoperabilidade

A interoperabilidade com linguagens nativas normalmente ocorrerá através de código `unsafe`.

A sintaxe específica para FFI será definida em documento próprio.

Independentemente da implementação adotada, o código externo não poderá comprometer as garantias observadas pelo restante do programa escrito em Capi.

---

## 19.6 Foreign Function Interface

A Capi poderá oferecer um mecanismo de **Foreign Function Interface** para interoperabilidade com código externo, especialmente bibliotecas nativas escritas em C.

Chamadas FFI pertencem ao domínio do código `unsafe`, pois envolvem elementos que não podem ser integralmente verificados pela Semântica Operacional da Capi.

A sintaxe específica para declaração e chamada de funções externas será definida em documento próprio.

Toda chamada FFI deve preservar os seguintes princípios:

* o programador é responsável por garantir que a assinatura declarada em Capi seja compatível com a função externa;
* o programador é responsável por garantir a validade dos ponteiros, buffers e estruturas recebidas ou enviadas ao código externo;
* a Capi não oferece garantias sobre o comportamento interno do código externo;
* o código externo não deve comprometer as garantias observáveis do código seguro escrito em Capi.

O uso de FFI não altera o Modelo de Memória da Capi.

Qualquer memória compartilhada com código externo deverá ser tratada como recurso fora das garantias estáticas da linguagem, salvo quando encapsulada por abstrações seguras da biblioteca padrão.

---

# 20. Garantias da Linguagem

Este capítulo consolida as garantias fundamentais oferecidas pela Capi.

Essas garantias representam compromissos permanentes da especificação e independem da estratégia utilizada pelo compilador.

---

## 20.1 Segurança de Memória

Programas escritos integralmente em código seguro não podem produzir:

* uso de memória após liberação (*use-after-free*);
* dupla liberação (*double free*);
* referências pendentes (*dangling references*);
* acesso a memória não inicializada;
* referências nulas.

---

## 20.2 Integridade das Identidades

Todo objeto possui exatamente um `ObjectId`.

Durante seu lifetime:

* a identidade permanece imutável;
* diferentes objetos nunca compartilham a mesma identidade;
* todas as referências observam a mesma identidade.

---

## 20.3 Integridade dos Domains

Todo objeto pertence exatamente a um Domain.

Nenhuma operação válida pode:

* remover um objeto de seu Domain sem obedecer às regras da linguagem;
* associar simultaneamente um objeto a múltiplos Domains;
* permitir a existência de objetos fora de um Domain válido.

---

## 20.4 Segurança de Concorrência

Programas escritos em código seguro não podem produzir *data races*.

A exclusividade da Capacidade de Mutação garante que operações incompatíveis sobre o mesmo Domain não possam ocorrer simultaneamente.

---

## 20.5 Determinismo

A destruição da memória é determinística.

Todo Domain possui início e término claramente definidos.

A destruição dos objetos acompanha necessariamente a destruição do Domain ao qual pertencem.

---

# 21. Princípio da Separação entre Garantias e Implementação

A Capi distingue explicitamente entre:

* as garantias oferecidas pela linguagem;
* os mecanismos utilizados para implementá-las.

Esse princípio aplica-se a toda a especificação.

---

## 21.1 Garantias

Garantias representam propriedades obrigatórias observáveis pelo programador.

Exemplos:

* ausência de referências nulas;
* segurança de memória;
* gerenciamento determinístico;
* integridade das identidades;
* exclusividade da escrita.

Essas propriedades nunca podem ser alteradas por decisões de implementação.

---

## 21.2 Implementação

A implementação compreende os mecanismos utilizados pelo compilador e pela infraestrutura mínima da linguagem para satisfazer essas garantias.

Podem ser utilizados, entre outros:

* análise estática;
* capacidades lineares;
* análise de regiões;
* especialização de código;
* otimizações;
* qualquer mecanismo equivalente.

A especificação não exige nenhum desses mecanismos em particular.

---

## 21.3 Evolução

Novas versões do compilador podem introduzir técnicas mais sofisticadas de implementação.

Desde que preservem todas as garantias estabelecidas por esta especificação, essas alterações não modificam a semântica da linguagem.

Esse princípio assegura a estabilidade da Capi e permite sua evolução sem comprometer a compatibilidade conceitual.

---

# 22. Considerações Finais

A Semântica Operacional conclui a definição do comportamento fundamental da linguagem Capi.

Enquanto os documentos anteriores estabeleceram os princípios arquiteturais, o Sistema de Tipos, o Modelo de Memória e a Sintaxe, este documento formalizou o significado operacional dessas construções.

A especificação estabelece uma separação clara entre:

* o comportamento observável da linguagem;
* os mecanismos utilizados para implementá-lo.

Essa distinção permite que compiladores evoluam continuamente, adotando novas técnicas de análise, otimização e geração de código, sem alterar as garantias oferecidas aos programadores.

Com a conclusão deste documento, encontram-se formalmente definidos:

* os fundamentos conceituais da linguagem;
* o Sistema de Tipos;
* o Modelo de Memória;
* a Sintaxe;
* a Semântica Operacional.

Os documentos subsequentes poderão aprofundar aspectos específicos da implementação, como o formato da representação intermediária (IR), arquitetura do compilador, biblioteca padrão, interoperabilidade, concorrência avançada e ecossistema da linguagem, preservando integralmente as garantias estabelecidas pela especificação principal.

A Capi propõe uma nova interpretação da Orientação a Objetos, na qual identidade, comportamento, memória e mutabilidade deixam de competir entre si e passam a atuar como responsabilidades independentes. Essa separação permite combinar herança, subtipagem e identidade compartilhada com segurança de memória estática e gerenciamento determinístico, estabelecendo as bases para uma linguagem orientada a objetos moderna, segura e preparada para evolução futura.
