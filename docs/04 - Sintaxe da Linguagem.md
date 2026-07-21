# Capi — A Próxima Geração da Orientação a Objetos

# Documento 04 — Sintaxe da Linguagem

Versão: 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a sintaxe da linguagem **Capi**.

Seu objetivo é estabelecer a forma como os conceitos apresentados nos documentos anteriores são expressos pelo programador, definindo a estrutura do código-fonte da linguagem.

Enquanto:

* o **Documento 01** define a arquitetura conceitual;
* o **Documento 02** define o Sistema de Tipos;
* o **Documento 03** define o Modelo de Memória;

este documento define exclusivamente a representação sintática desses conceitos.

---

## 1.2 Escopo

Este documento especifica:

* estrutura dos programas;
* organização dos arquivos;
* regras léxicas;
* declaração de variáveis;
* declaração de tipos;
* classes;
* interfaces;
* traits;
* Domains;
* expressões;
* estruturas de controle;
* funções;
* generics;
* organização modular.

Não fazem parte deste documento:

* semântica operacional;
* regras completas de inferência;
* arquitetura do compilador;
* representação intermediária;
* geração de código.

Esses assuntos serão tratados em documentos específicos.

---

## 1.3 Filosofia da Sintaxe

A sintaxe da Capi foi concebida com quatro objetivos principais:

* baixo custo cognitivo;
* alta legibilidade;
* forte consistência;
* proximidade com a Orientação a Objetos clássica.

A linguagem adota uma sintaxe inspirada principalmente em **Java**, **C#** e **Kotlin**, preservando convenções amplamente conhecidas pela comunidade.

Ao mesmo tempo, busca eliminar construções consideradas desnecessariamente verbosas ou ambíguas.

---

## 1.4 Princípios

A definição sintática da linguagem segue os seguintes princípios.

### Clareza

Cada construção deve possuir significado único e facilmente identificável.

---

### Consistência

Construções semelhantes devem utilizar sintaxe semelhante.

A linguagem evita exceções desnecessárias.

---

### Segurança

Nenhuma construção sintática pode contornar as garantias definidas pelo Sistema de Tipos ou pelo Modelo de Memória.

---

### Expressividade

Construções frequentes devem exigir pouca quantidade de código.

Construções complexas devem permanecer explícitas.

---

## 1.5 Relação com os Documentos Anteriores

Este documento descreve apenas **como** o programador escreve código.

As garantias oferecidas pela linguagem continuam sendo definidas pelos documentos anteriores.

Em caso de conflito entre sintaxe e semântica, prevalecem:

1. Documento 02 — Sistema de Tipos;
2. Documento 03 — Modelo de Memória;
3. Documento 05 — Semântica Operacional.

---

# 2. Estrutura de um Programa

Um programa em Capi é composto por um ou mais módulos organizados em arquivos fonte.

Cada arquivo contém declarações pertencentes a um único módulo.

A organização modular tem como objetivos:

* facilitar reutilização;
* reduzir acoplamento;
* permitir compilação incremental;
* favorecer encapsulamento.

---

## 2.1 Arquivos Fonte

A extensão oficial dos arquivos fonte é:

```text
.cap
```

Exemplo:

```text
Conta.cap
Cliente.cap
Banco.cap
```

---

## 2.2 Módulos

Todo arquivo pertence exatamente a um módulo.

Exemplo:

```capi
module banco.contas;
```

O módulo define o espaço lógico ao qual pertencem as declarações contidas no arquivo.

---

## 2.3 Imports

Declarações pertencentes a outros módulos tornam-se visíveis através da diretiva `import`.

Exemplo:

```capi
import banco.pessoas.Cliente;
import banco.financeiro.*;
```

O formato exato das regras de resolução de módulos será definido na Semântica Operacional.

---

## 2.4 Organização Recomendada

A especificação recomenda que a estrutura de diretórios reflita a estrutura lógica dos módulos.

Exemplo:

```text
src/

    banco/

        contas/

            Conta.cap

            Agencia.cap

        pessoas/

            Cliente.cap

            Endereco.cap
```

Essa organização é recomendada, mas não obrigatória.

---

## 2.5 Ponto de Entrada

Todo programa executável possui exatamente um ponto de entrada.

Inicialmente a linguagem adota a função:

```capi
main()
```

Exemplo:

```capi
function main() {

}
```

A assinatura definitiva será especificada na Semântica Operacional.

---

# 3. Léxico

Este capítulo define os elementos básicos da linguagem.

---

## 3.1 Identificadores

Identificadores representam nomes definidos pelo programador.

Podem conter:

* letras;
* dígitos;
* caractere `_`.

Não podem:

* iniciar por número;
* utilizar palavras reservadas.

Exemplos válidos:

```text
Cliente

ContaCorrente

saldo

valor_total

calcularJuros
```

---

## 3.2 Sensibilidade a Maiúsculas

A linguagem diferencia letras maiúsculas de minúsculas.

Exemplo:

```text
Conta

conta

CONTA
```

representam identificadores distintos.

---

## 3.3 Comentários

Comentários de linha:

```capi
// comentário
```

Comentários em bloco:

```capi
/*
comentário
*/
```

Comentários não influenciam a semântica do programa.

---

## 3.4 Literais

A linguagem suporta literais para:

* inteiros;
* ponto flutuante;
* caracteres;
* strings;
* booleanos.

Exemplos:

```capi
10

3.14

'A'

"Cliente"

true

false
```

Outros formatos poderão ser adicionados futuramente.

---

## 3.5 Palavras Reservadas

Entre as palavras reservadas da linguagem encontram-se:

```text
abstract
break
case
class
const
constructor
continue
default
else
extends
false
final
for
function
if
implements
import
interface
let
match
module
new
override
private
protected
public
return
sealed
static
switch
trait
true
unsafe
uses
while
```

A lista completa será mantida como parte da especificação oficial da linguagem.

---

# 4. Declarações Básicas

Este capítulo apresenta as construções fundamentais utilizadas na escrita de programas.

---

## 4.1 Variáveis

Variáveis armazenam referências, valores ou identidades durante a execução do programa.

Sua declaração utiliza a palavra-chave `let`.

Exemplo:

```capi
let idade = 25;

let nome = "Gabriel";
```

Sempre que possível, o compilador realizará inferência automática do tipo.

---

## 4.2 Declaração Explícita de Tipo

Quando desejado, o tipo pode ser declarado explicitamente.

Exemplo:

```capi
let idade : Int32 = 25;

let nome : String = "Gabriel";
```

---

## 4.3 Constantes

Valores imutáveis podem ser declarados utilizando `const`.

Exemplo:

```capi
const PI = 3.1415926535;
```

Constantes não podem receber novo valor após sua inicialização.

const impede a reatribuição do identificador após sua inicialização. Os efeitos dessa restrição sobre o modelo de execução são definidos na Semântica Operacional.

---

## 4.4 Escopo

O escopo de uma declaração é delimitado por blocos.

Exemplo:

```capi
function exemplo() {

    let x = 10;

    if (x > 5) {

        let y = 20;

    }

}
```

Nesse exemplo:

* `x` existe em toda a função;
* `y` existe apenas dentro do bloco `if`.

---

## 4.5 Inferência de Tipos

A inferência de tipos constitui o comportamento padrão da linguagem.

Sempre que o compilador puder determinar o tipo de maneira inequívoca, sua declaração poderá ser omitida.

Exemplo:

```capi
let saldo = 100.0;

let cliente = new Cliente();
```

A inferência nunca reduz as garantias do Sistema de Tipos.

Ela representa apenas uma simplificação sintática.

---

## 4.6 Inicialização Obrigatória

Toda variável deve possuir um valor válido antes de sua primeira utilização.

A linguagem não permite variáveis contendo valores indefinidos.

Essa regra elimina uma classe comum de erros presentes em linguagens tradicionais.

---

# 5. Tipos Primitivos

Os tipos primitivos representam os blocos fundamentais do Sistema de Tipos da Capi.

São tipos definidos pela própria linguagem e possuem representação direta na memória.

A semântica de cada tipo é definida pelo Documento 02 — Sistema de Tipos. Este capítulo descreve apenas sua representação sintática.

---

## 5.1 Tipos Integrais

A Capi fornece tipos inteiros assinados e não assinados.

Exemplos:

```capi
let idade : Int32 = 35;

let total : UInt64 = 1000;
```

Os tamanhos suportados são:

| Tipo   | Bits |
| ------ | ---: |
| Int8   |    8 |
| Int16  |   16 |
| Int32  |   32 |
| Int64  |   64 |
| UInt8  |    8 |
| UInt16 |   16 |
| UInt32 |   32 |
| UInt64 |   64 |

---

## 5.2 Tipos de Ponto Flutuante

A linguagem fornece:

```text
Float32

Float64
```

Exemplo:

```capi
let altura : Float32 = 1.82;

let pi : Float64 = 3.1415926535;
```

---

## 5.3 Booleano

Representa valores lógicos.

```capi
let ativo = true;

let finalizado = false;
```

Tipo:

```text
Bool
```

---

## 5.4 Caracteres

Representam um único caractere Unicode.

```capi
let letra = 'A';
```

Tipo:

```text
Char
```

---

## 5.5 Strings

Strings representam sequências imutáveis de caracteres.

```capi
let nome = "Gabriel";
```

Tipo:

```text
String
```

A representação física da String pertence ao Modelo de Memória.

---

## 5.6 Unit

A linguagem define o tipo especial:

```text
Unit
```

`Unit` representa ausência de valor significativo.

Toda função que não declara retorno produz implicitamente um valor do tipo `Unit`.

Exemplo:

```capi
function imprimir() {

    println("Olá");

}
```

---

# 6. Tipos Compostos

Além dos tipos primitivos, a linguagem oferece construções para composição de dados.

---

## 6.1 Arrays

Arrays pertencem ao Domain no qual são criados. Arrays de tipos por identidade armazenam ObjectIds; arrays de tipos por valor armazenam diretamente seus valores.

Arrays possuem tamanho fixo definido durante sua criação.

Exemplo:

```capi
let valores : Int32[10];
```

ou

```capi
let valores = [1,2,3,4];
```

Todos os elementos devem possuir o mesmo tipo.

---

## 6.2 Tuplas

Tuplas agrupam valores heterogêneos.

Exemplo:

```capi
let pessoa = ("Gabriel", 40, true);
```

Os elementos são acessados por posição.

---

## 6.3 Collections

A biblioteca padrão fornecerá coleções como:

* List<T>
* Set<T>
* Map<K,V>
* Queue<T>

A sintaxe de utilização segue o modelo de Generics da linguagem.

Exemplo:

```capi
let clientes : List<Cliente>;

let contas : Map<String, Conta>;
```

---

## 6.4 Optional

A Capi não possui referências nulas.

A ausência de um valor é representada por:

```text
Optional<T>
```

Exemplo:

```capi
let cliente : Optional<Cliente>;
```

Um valor opcional deverá ser tratado explicitamente antes de sua utilização.

---

## 6.5 Result

Operações que podem produzir sucesso ou falha utilizam:

```text
Result<T,E>
```

Exemplo:

```capi
let conta = abrirConta();
```

onde:

```text
abrirConta()

↓

Result<Conta,ErroConta>
```

O compilador poderá exigir que ambos os casos sejam tratados antes da utilização do resultado.

O tratamento de Result<T,E> normalmente é realizado através de Pattern Matching (Capítulo 16) ou pelas abstrações fornecidas pela biblioteca padrão.

---

# 7. Classes

A Classe representa a principal unidade de abstração da Orientação a Objetos na Capi.

Ela define:

* estado lógico;
* comportamento;
* contratos de construção;
* relacionamento de herança.

A representação física dos objetos pertencentes à classe é definida pelo Modelo de Memória.

---

## 7.1 Declaração

Classes são declaradas utilizando a palavra-chave:

```text
class
```

Exemplo:

```capi
public class Cliente {

}
```

---

## 7.2 Campos

Campos representam o estado lógico do objeto.

Exemplo:

```capi
public class Cliente {

    private nome : String;

    private idade : Int32;

}
```

A disposição física desses campos é responsabilidade do compilador.

---

## 7.3 Métodos

Métodos representam comportamento.

Exemplo:

```capi
public class Cliente {

    function nomeCompleto() : String {

        return nome;

    }

}
```

A sintaxe completa será detalhada no capítulo de Funções.

---

## 7.4 Construtores

Objetos são inicializados por construtores.

Exemplo:

```capi
public class Cliente {

    constructor(nome : String) {

        this.nome = nome;

    }

}
```

O processo físico de criação do objeto é definido pelo Modelo de Memória.

---

## 7.5 Visibilidade

A linguagem fornece inicialmente quatro modificadores:

```text
public

protected

private

internal
```

O comportamento exato de cada modificador será definido pela Semântica Operacional.

Nota: Os modificadores de visibilidade (public, protected, internal, private) controlam o acesso a membros entre diferentes partes do programa. Suas regras formais são definidas no Documento 05 — Semântica Operacional.

---

## 7.6 Classes Abstratas

Classes podem ser declaradas como abstratas.

Exemplo:

```capi
public abstract class Pessoa {

}
```

Classes abstratas não podem ser instanciadas diretamente.

Nota: Classes abstratas não podem ser instanciadas diretamente. Servem como base para outras classes na hierarquia de herança. As regras de abstração são definidas no Documento 05 — Semântica Operacional.

---

## 7.7 Classes Seladas

Uma classe pode ser declarada como:

```text
sealed
```

Exemplo:

```capi
public sealed class Forma {

}
```

Classes seladas restringem sua extensibilidade conforme definido pelo Sistema de Tipos e pela Semântica Operacional.

Nota: Classes seladas restringem o conjunto de subclasses conhecidas em tempo de compilação. Essa característica permite verificações exaustivas em operações de pattern matching e possibilita validações estáticas adicionais, conforme definido no Documento 02 e detalhado na Semântica Operacional.

---

# 8. Herança

A Capi preserva a herança clássica de implementação.

Cada classe pode possuir exatamente uma superclasse.

---

## 8.1 Extensão

Uma classe deriva de outra utilizando:

```text
extends
```

Exemplo:

```capi
public class Cliente extends Pessoa {

}
```

A representação física dessa relação é definida pelo Modelo de Memória.

---

## 8.2 Sobrescrita

Métodos podem ser sobrescritos utilizando:

```text
override
```

Exemplo:

```capi
override function nomeCompleto() : String {

    return nome;

}
```

Nota: A sobrescrita de métodos (override) permite que uma classe derivada substitua a implementação de um método da superclasse. As regras de validade, incluindo compatibilidade de assinaturas e variância, são definidas no Documento 05 — Semântica Operacional.

---

## 8.3 Classes Finais

Classes que não podem ser herdadas utilizam:

```text
final
```

Exemplo:

```capi
public final class CPF {

}
```

---

## 8.4 Métodos Finais

Métodos também podem impedir sobrescrita.

Exemplo:

```capi
public final function calcular() : Float64 {

}
```

---

## 8.5 Hierarquia

A Capi suporta apenas herança simples.

Exemplo:

```text
Pessoa

↓

Cliente

↓

ClientePremium
```

Herança múltipla de implementação não é suportada.

A reutilização horizontal de comportamento é realizada através de **Traits**, enquanto contratos são definidos por **Interfaces**.

Essa separação preserva a simplicidade do layout de memória e reduz a complexidade do Sistema de Tipos.

---

# 9. Interfaces

Interfaces definem contratos públicos de comportamento.

Uma Interface estabelece quais operações uma classe deve fornecer, sem impor qualquer representação física ou estado persistente.

Interfaces não participam do layout de memória dos objetos.

---

## 9.1 Declaração

Interfaces são declaradas utilizando a palavra-chave:

```text
interface
```

Exemplo:

```capi
public interface Autenticavel {

    function autenticar(senha : String) : Bool;

}
```

---

## 9.2 Implementação

Uma classe implementa uma ou mais Interfaces utilizando a palavra-chave:

```text
implements
```

Exemplo:

```capi
public class Usuario implements Autenticavel {

    override function autenticar(senha : String) : Bool {

        ...

    }

}
```

---

## 9.3 Múltiplas Interfaces

Uma classe pode implementar diversas Interfaces simultaneamente.

Exemplo:

```capi
public class Conta
    implements Persistente,
               Auditavel,
               Exportavel {

}
```

Essa capacidade não altera a organização física do objeto.

---

## 9.4 Estado

Interfaces não podem declarar:

* campos;
* atributos;
* estado persistente.

Seu propósito é exclusivamente definir contratos públicos.

---

## 9.5 Compatibilidade

Uma referência para uma Interface pode apontar para qualquer objeto cuja classe implemente esse contrato.

Exemplo:

```capi
let usuario : Usuario = new Usuario();

let autenticavel : Autenticavel = usuario;
```

Essa conversão constitui um upcast e possui custo constante.

As regras formais são definidas pelo Sistema de Tipos.

---

# 10. Traits

Traits representam mecanismos de reutilização de comportamento.

Enquanto Interfaces definem **o que** uma classe deve oferecer, Traits permitem compartilhar **como** parte desse comportamento é implementado.

Traits não alteram a hierarquia de herança.

---

## 10.1 Declaração

Traits são declaradas utilizando:

```text
trait
```

Exemplo:

```capi
public trait Logavel {

    function log(texto : String) {

        println(texto);

    }

}
```

---

## 10.2 Implementação

Uma classe pode utilizar uma ou mais Traits.

Exemplo:

```capi
public class Usuario
    implements Autenticavel
    uses Logavel {

}
```

A sintaxe definitiva para composição de Traits será estabelecida na Semântica Operacional.

---

## 10.3 Implementação Padrão

Traits podem fornecer implementação padrão para seus métodos.

Exemplo:

```capi
public trait Nomeavel {

    function nomeCompleto() : String {

        return nome();

    }

}
```

Classes podem reutilizar essa implementação ou sobrescrevê-la.

---

## 10.4 Estado

Traits não podem declarar estado persistente.

Não podem possuir:

* campos;
* atributos de instância;
* armazenamento próprio.

Qualquer estado necessário deve ser obtido através:

* de métodos da classe;
* de parâmetros;
* de outros contratos definidos pela própria Trait.

Essa restrição preserva a simplicidade do Modelo de Memória.

---

## 10.5 Diferença entre Interface e Trait

| Interface                       | Trait                               |
| ------------------------------- | ----------------------------------- |
| Define contratos                | Reutiliza comportamento             |
| Não possui implementação padrão | Pode possuir implementação padrão   |
| Não possui estado               | Não possui estado persistente       |
| Não altera layout               | Não altera layout                   |
| Suporta polimorfismo            | Suporta composição de comportamento |

---

# 11. Objetos

Objetos representam instâncias de Classes.

Cada objeto possui:

* identidade;
* estado lógico;
* comportamento definido por sua Classe.

Sua organização física é definida pelo Modelo de Memória.

---

## 11.1 Criação

Objetos são criados utilizando a palavra-chave:

```text
new
```

Exemplo:

```capi
let cliente = new Cliente("Gabriel");
```

A criação de um objeto implica sua associação a um Domain.

---

## 11.2 Identidade

Cada objeto possui uma identidade única representada por um `ObjectId`.

Essa identidade:

* permanece constante durante todo seu lifetime;
* não representa ownership;
* não representa endereço físico.

Essas propriedades são garantidas pelo Sistema de Tipos e pelo Modelo de Memória.

---

## 11.3 Compartilhamento

Objetos podem ser compartilhados livremente.

Exemplo:

```capi
let conta = new Conta();

let a = conta;

let b = conta;
```

As variáveis `a` e `b` referenciam o mesmo objeto.

O gerenciamento de memória permanece responsabilidade do Domain.

---

## 11.4 Comparação

A linguagem distingue comparação por identidade de comparação por valor.

Exemplo conceitual:

```capi
a === b    // identidade

a == b     // igualdade lógica
```

A semântica completa desses operadores será definida no Documento 05.

---

## 11.5 Destruição

Objetos não são destruídos individualmente por mecanismos automáticos de gerenciamento de memória.

Seu lifetime é determinado exclusivamente pelo Domain ao qual pertencem.

---

# 12. Organização de Domains

Os Domains representam unidades de organização do Modelo de Memória.

Na maioria dos programas, sua existência é completamente inferida pelo compilador.

O programador normalmente não cria Domains explicitamente.

Quando necessário, poderá fornecer **declarações de organização de memória**, permitindo influenciar a construção dos Domains sem expor sua implementação.

Essas declarações representam **diretivas de compilação**, e não objetos da aplicação.

---

## 12.1 Diretivas de Organização

A linguagem poderá oferecer diretivas para indicar que determinados objetos devem compartilhar o mesmo Domain.

Uma diretiva @domain(Nome) indica ao compilador que os objetos declarados dentro desse contexto (instâncias de classes anotadas, objetos criados em funções anotadas) devem ser alocados no mesmo Domain. O compilador deve respeitar essa diretiva sempre que possível, mas pode realizar otimizações desde que preservem as garantias da linguagem.

Exemplo conceitual:

```capi
@domain(Banco)

class Cliente {

}
```

```capi
@domain(Banco)

class Conta {

}
```

ou

```capi
@domain(Banco)

function processar() {

}
```

A forma sintática definitiva será definida em versões futuras da especificação.

---

## 12.2 Inferência

Na ausência de diretivas explícitas, o compilador é responsável por inferir automaticamente a organização dos Domains.

Essa inferência deve preservar todas as garantias definidas pelo Sistema de Tipos e pelo Modelo de Memória.

---

## 12.3 Associação

Todo objeto pertence exatamente a um Domain.

Essa associação pode ser explícita ou inferida pelo compilador, mas permanece imutável durante o lifetime do objeto.

---

## 12.4 Lifetime

O lifetime dos objetos acompanha integralmente o lifetime do Domain ao qual pertencem.

---

## 12.5 Migração

Objetos não podem migrar implicitamente entre Domains.

Caso a linguagem ofereça mecanismos explícitos de transferência de dados entre Domains, esses mecanismos criarão novas instâncias no Domain de destino, preservando todas as garantias do Sistema de Tipos e do Modelo de Memória.

---

# 13. Mutação

A mutação representa qualquer operação que altere o estado persistente de um objeto.

Na Capi, a sintaxe de mutação procura preservar a simplicidade da Orientação a Objetos tradicional, enquanto as garantias de segurança permanecem responsabilidade do Sistema de Tipos, do Modelo de Memória e da Semântica Operacional.

---

## 13.1 Métodos Mutantes

Métodos capazes de alterar o estado do objeto são declarados normalmente.

Exemplo:

```capi
public class Conta {

    private saldo : Float64;

    function depositar(valor : Float64) {

        saldo = saldo + valor;

    }

}
```

A sintaxe não diferencia visualmente métodos mutantes de métodos somente leitura.

Essa distinção pertence à Semântica Operacional.

---

## 13.2 Métodos de Leitura

Métodos que apenas consultam o estado do objeto seguem exatamente a mesma sintaxe.

Exemplo:

```capi
function saldo() : Float64 {

    return saldo;

}
```

O compilador determinará, durante a análise semântica, se o método realiza ou não operações de escrita.

---

## 13.3 Capacidade de Mutação

Toda operação de escrita pressupõe a existência de uma **Capacidade de Mutação** válida para o Domain ao qual pertence o objeto.

Essa capacidade não é manipulada explicitamente pelo programador.

Exemplo:

```capi
conta.depositar(100);
```

Embora a chamada pareça uma invocação tradicional de método, o compilador verificará que o Domain associado ao objeto possui uma Capacidade de Mutação válida antes de permitir a operação.

O mecanismo utilizado para essa verificação pertence à Semântica Operacional e não altera a sintaxe da linguagem.

---

## 13.4 Propriedade da Escrita

Do ponto de vista sintático, toda escrita continua sendo expressa como uma chamada de método.

Entretanto, conceitualmente:

* o objeto define o comportamento;
* o Domain autoriza a escrita;
* a Capacidade de Mutação garante a exclusividade dessa autorização.

Essa separação permanece transparente ao programador.

---

## 13.5 Transparência

Um dos objetivos da Capi é preservar a experiência tradicional de desenvolvimento orientado a objetos.

Assim, operações como:

```capi
conta.depositar(100);

cliente.alterarEndereco(novoEndereco);

pedido.cancelar();
```

continuam sendo escritas exatamente como em linguagens OO tradicionais.

A segurança adicional é introduzida pelo compilador, sem aumentar a complexidade sintática.

---

# 14. Expressões

Expressões representam construções que produzem valores.

A Capi adota uma sintaxe semelhante às linguagens modernas da família C.

---

## 14.1 Operadores Aritméticos

A linguagem suporta os operadores tradicionais.

```text
+
-
*
/
%
```

Exemplo:

```capi
let total = quantidade * preco;
```

---

## 14.2 Operadores Relacionais

```text
==
!=
<
<=
>
>=
```

Exemplo:

```capi
if (saldo >= limite) {

}
```

---

## 14.3 Operadores Lógicos

```text
&&

||

!
```

Exemplo:

```capi
if (ativo && autenticado) {

}
```

---

## 14.4 Operadores de Identidade

A linguagem distingue igualdade lógica de identidade.

```text
==

===
```

Onde:

```text
==

↓

Igualdade lógica
```

```text
===

↓

Mesma identidade (ObjectId)
```

O operador === compara a identidade dos objetos, isto é, seus ObjectId. Dois objetos são idênticos somente quando possuem exatamente o mesmo ObjectId.

Essa distinção elimina ambiguidades comuns em linguagens orientadas a objetos.

---

## 14.5 Precedência

A linguagem segue a precedência tradicional das linguagens da família C.

Quando necessário, parênteses podem ser utilizados para explicitar a ordem de avaliação.

Exemplo:

```capi
let valor = (a + b) * c;
```

---

## 14.6 Conversões

Conversões implícitas são limitadas.

Conversões potencialmente perigosas exigem sintaxe explícita.

Exemplo ilustrativo:

```capi
let inteiro = valor.as<Int32>();
```

A lista completa de conversões permitidas será definida na Semântica Operacional.

---

# 15. Controle de Fluxo

A linguagem fornece as estruturas tradicionais de controle.

Todas seguem sintaxe semelhante às linguagens OO modernas.

---

## 15.1 If

```capi
if (saldo > limite) {

    bloquear();

}
else {

    liberar();

}
```

---

## 15.2 Switch

```capi
switch (status) {

    case Aberto:

        ...

    case Fechado:

        ...

    default:

        ...

}
```

---

## 15.3 While

```capi
while (saldo > 0) {

    saldo--;

}
```

---

## 15.4 For

```capi
for (let i = 0; i < 10; i++) {

    println(i);

}
```

---

## 15.5 Foreach

Coleções podem ser percorridas utilizando:

```capi
foreach (cliente in clientes) {

    println(cliente.nome());

}
```

---

## 15.6 Break

```capi
while (true) {

    if (erro) {

        break;

    }

}
```

---

## 15.7 Continue

```capi
foreach (valor in valores) {

    if (valor < 0) {

        continue;

    }

    processar(valor);

}
```

---

## 15.8 Retorno

Funções retornam valores utilizando:

```capi
return valor;
```

ou simplesmente:

```capi
return;
```

para funções cujo retorno é `Unit`.

---

# 16. Pattern Matching

A Capi oferece um mecanismo de *Pattern Matching* para simplificar o tratamento de hierarquias de tipos e estruturas algébricas.

Esse mecanismo complementa, mas não substitui, as estruturas tradicionais de controle.

---

## 16.1 Match

Exemplo:

```capi
match (resultado) {

    case Success(valor):

        println(valor);

    case Error(erro):

        println(erro);

}
```

---

## 16.2 Optional

```capi
match (cliente) {

    case Some(valor):

        println(valor.nome());

    case None:

        println("Cliente inexistente");

}
```

---

## 16.3 Result

```capi
match (resultado) {

    case Success(conta):

        utilizar(conta);

    case Failure(erro):

        tratar(erro);

}
```

---

## 16.4 Classes Seladas

Quando aplicado a hierarquias de classes `sealed`, o compilador pode verificar que todos os casos possíveis foram tratados.

Exemplo:

```capi
match (forma) {

    case Circulo(c):

        ...

    case Retangulo(r):

        ...

}
```

Caso uma nova subclasse seja adicionada à hierarquia, o compilador poderá exigir a atualização do `match`.

---

## 16.5 Exaustividade

Sempre que possível, o compilador verificará que todas as possibilidades foram consideradas.

Essa verificação reduz a ocorrência de erros decorrentes de casos não tratados e constitui uma das principais vantagens do Pattern Matching sobre cadeias extensas de `if` e `switch`.

---

# 17. Funções

Funções representam unidades reutilizáveis de comportamento.

Na Capi, funções podem existir de forma independente ou como membros de Classes, Interfaces e Traits.

A semântica de chamadas, despacho e resolução de sobrecarga é definida pelo Documento 05 — Semântica Operacional.

---

## 17.1 Declaração

Funções são declaradas utilizando a palavra-chave:

```text
function
```

Exemplo:

```capi
function somar(a : Int32, b : Int32) : Int32 {

    return a + b;

}
```

---

## 17.2 Parâmetros

Os parâmetros são declarados entre parênteses.

Exemplo:

```capi
function transferir(
    origem : Conta,
    destino : Conta,
    valor : Float64
) {

}
```

A passagem de parâmetros respeita as regras do Sistema de Tipos.

---

## 17.3 Valor de Retorno

O tipo de retorno é declarado após os parâmetros.

Exemplo:

```capi
function nome() : String {

    return "Gabriel";

}
```

Quando omitido, o retorno implícito é `Unit`.

---

## 17.4 Sobrecarga

A linguagem permite sobrecarga de funções.

Exemplo:

```capi
function imprimir(valor : Int32) {

}

function imprimir(valor : String) {

}
```

A escolha da função correta é realizada em tempo de compilação.

---

## 17.5 Recursão

Funções podem chamar a si próprias.

Exemplo:

```capi
function fatorial(n : Int32) : Int32 {

    if (n <= 1) {

        return 1;

    }

    return n * fatorial(n - 1);

}
```

A implementação poderá realizar otimizações como *Tail Call Optimization* quando possível.

---

## 17.6 Funções Estáticas

Classes podem declarar funções estáticas.

Exemplo:

```capi
public class Matematica {

    static function max(a : Int32, b : Int32) : Int32 {

        return a > b ? a : b;

    }

}
```

Funções estáticas não pertencem a uma instância específica.

---

# 18. Generics

Generics permitem escrever algoritmos independentes de tipos concretos.

A Capi adota um modelo de generics fortemente tipado e verificado em tempo de compilação.

---

## 18.1 Declaração

Exemplo:

```capi
public class Lista<T> {

}
```

---

## 18.2 Múltiplos Parâmetros

Uma declaração pode possuir diversos parâmetros de tipo.

Exemplo:

```capi
public class Mapa<K, V> {

}
```

---

## 18.3 Restrições

Tipos genéricos podem impor restrições.

Exemplo ilustrativo:

```capi
public class Repositorio<T>
    where T implements Entidade {

}
```

A sintaxe definitiva das restrições será definida pela Semântica Operacional.

---

## 18.4 Inferência

Sempre que possível, o compilador realizará inferência automática.

Exemplo:

```capi
let clientes = new Lista<Cliente>();

let nomes = new Lista();
```

No segundo exemplo, o tipo poderá ser inferido pelo compilador.

---

## 18.5 Variância

A variância segue as regras estabelecidas pelo Documento 02 — Sistema de Tipos.

Em resumo:

* tipos genéricos são invariantes por padrão;
* co-variância e contra-variância poderão ser permitidas em situações específicas.

A sintaxe correspondente será definida em documento próprio.

---

## 18.6 Compatibilidade

Tipos genéricos preservam integralmente:

* segurança de memória;
* regras de herança;
* polimorfismo;
* garantias do Sistema de Tipos.

---

# 19. Organização do Código

A Capi incentiva a organização modular do código-fonte.

O objetivo é facilitar manutenção, reutilização e evolução de sistemas de grande porte.

---

## 19.1 Packages

Os módulos da linguagem são agrupados em Packages.

Exemplo:

```capi
module financeiro.contas;
```

Essa organização representa o espaço lógico da aplicação.

---

## 19.2 Namespaces

Packages funcionam como namespaces naturais.

Exemplo:

```capi
financeiro.contas.Conta
```

Evita conflitos entre nomes de Classes pertencentes a módulos distintos.

---

## 19.3 Arquivos

Recomenda-se que cada Classe pública seja declarada em um único arquivo.

Exemplo:

```text
Conta.cap

Cliente.cap

Banco.cap
```

Essa convenção melhora a navegabilidade do projeto.

---

## 19.4 Visibilidade

A organização modular trabalha em conjunto com os modificadores de visibilidade:

```text
public

protected

internal

private
```

A Semântica Operacional definirá exatamente o alcance de cada modificador.

---

## 19.5 Convenções

A especificação recomenda as seguintes convenções:

| Elemento  | Convenção  |
| --------- | ---------- |
| Classe    | PascalCase |
| Interface | PascalCase |
| Trait     | PascalCase |
| Método    | camelCase  |
| Variável  | camelCase  |
| Constante | UPPER_CASE |
| Package   | minúsculas |

Essas recomendações favorecem consistência entre projetos.

---

# 20. Atributos e Anotações

A linguagem fornece um mecanismo de metadados para associar informações adicionais a declarações.

Esses metadados não alteram a semântica da linguagem, salvo quando explicitamente definido por alguma especificação.

---

## 20.1 Declaração

Anotações utilizam o caractere `@`.

Exemplo:

```capi
@Entity
public class Cliente {

}
```

---

## 20.2 Aplicação

Anotações podem ser aplicadas a:

* Classes;
* Interfaces;
* Traits;
* Funções;
* Métodos;
* Campos;
* Parâmetros.

---

## 20.3 Parâmetros

Anotações podem receber parâmetros.

Exemplo:

```capi
@Table("clientes")
public class Cliente {

}
```

---

## 20.4 Metadados

As anotações representam apenas metadados.

Seu significado depende da biblioteca, framework ou ferramenta que as interpreta.

A linguagem não atribui comportamento implícito a anotações, exceto quando definido explicitamente pela própria especificação.

---

## 20.5 Extensibilidade

Bibliotecas e frameworks podem definir novas anotações sem necessidade de alteração da linguagem.

Essa abordagem permite ampliar o ecossistema da Capi mantendo o núcleo da linguagem simples, estável e independente de tecnologias específicas.

---

# 21. Código `unsafe`

Embora a Capi tenha sido projetada para oferecer segurança de memória estática, interoperabilidade com bibliotecas nativas, acesso a hardware e determinadas otimizações exigem operações que não podem ser verificadas integralmente pelo compilador.

Para esses casos, a linguagem oferece o bloco `unsafe`.

Seu uso deve ser excepcional.

---

## 21.1 Objetivo

O bloco `unsafe` permite executar operações fora das garantias verificadas pelo Sistema de Tipos.

Exemplos de utilização incluem:

* interoperabilidade com código C;
* manipulação de memória bruta;
* acesso a registradores de hardware;
* implementação de bibliotecas de baixo nível;
* otimizações altamente especializadas.

---

## 21.2 Sintaxe

Exemplo:

```capi
unsafe {

    // operações de baixo nível

}
```

Todo código executado dentro do bloco `unsafe` é explicitamente identificado.

Essa distinção facilita auditoria, revisão de código e análise estática.

---

## 21.3 Responsabilidade

Dentro de um bloco `unsafe`, a responsabilidade pela preservação das garantias da linguagem passa a ser do programador.

O compilador deixa de verificar determinadas propriedades, como:

* segurança de memória;
* validade de conversões de baixo nível;
* acesso direto à memória;
* integridade de ponteiros nativos.

---

## 21.4 Interoperabilidade

O mecanismo `unsafe` constitui o principal meio de interoperabilidade da Capi com código externo.

Exemplo conceitual:

```capi
unsafe {

    native_call();

}
```

A sintaxe definitiva para FFI será especificada em documento próprio.

---

## 21.5 Escopo

O uso de `unsafe` deve ser limitado ao menor escopo possível.

Exemplo recomendado:

```capi
function lerRegistrador() : UInt32 {

    unsafe {

        return hardware.read();

    }

}
```

Evita-se envolver funções inteiras em blocos `unsafe` quando apenas poucas instruções necessitam desse tratamento.

---

## 21.6 Garantias

O uso de `unsafe` não altera as garantias oferecidas ao restante do programa.

Todo código fora de blocos `unsafe` continua sujeito às verificações normais do compilador.

---

# 22. Convenções de Código

Este capítulo apresenta recomendações para promover consistência entre projetos escritos em Capi.

Essas convenções não fazem parte da sintaxe da linguagem, mas representam boas práticas recomendadas pela especificação.

---

## 22.1 Nomenclatura

Recomenda-se:

| Elemento  | Convenção  |
| --------- | ---------- |
| Classe    | PascalCase |
| Interface | PascalCase |
| Trait     | PascalCase |
| Enum      | PascalCase |
| Método    | camelCase  |
| Variável  | camelCase  |
| Campo     | camelCase  |
| Constante | UPPER_CASE |
| Package   | minúsculas |

---

## 22.2 Organização

Sugere-se que cada arquivo contenha uma única declaração pública principal.

Exemplo:

```text
Cliente.cap

Conta.cap

Banco.cap
```

Essa convenção facilita navegação e manutenção.

---

## 22.3 Legibilidade

A especificação recomenda:

* identação consistente;
* linhas curtas;
* nomes descritivos;
* baixo acoplamento;
* alta coesão.

O estilo de formatação oficial poderá ser definido futuramente por uma ferramenta automática.

---

## 22.4 Documentação

Comentários públicos devem utilizar sintaxe própria para documentação.

Exemplo ilustrativo:

```capi
///
/// Calcula o saldo disponível.
///
function saldo() : Float64 {

}
```

Ferramentas poderão utilizar essas informações para geração automática de documentação.

---

## 22.5 Estilo

A Capi procura privilegiar:

* simplicidade;
* previsibilidade;
* clareza;
* uniformidade.

Construções excessivamente implícitas ou dependentes de convenções ocultas devem ser evitadas.

---

# 23. Relação com a Semântica Operacional

Este documento define apenas a representação sintática da linguagem.

O comportamento de cada construção será formalizado no Documento 05 — Semântica Operacional.

---

## 23.1 Responsabilidades

A divisão de responsabilidades entre os documentos da especificação é a seguinte:

| Documento    | Responsabilidade        |
| ------------ | ----------------------- |
| Documento 01 | Fundamentos conceituais |
| Documento 02 | Sistema de Tipos        |
| Documento 03 | Modelo de Memória       |
| Documento 04 | Sintaxe da Linguagem    |
| Documento 05 | Semântica Operacional   |

Essa organização evita duplicação de conceitos e facilita a evolução da especificação.

---

## 23.2 Garantias

A sintaxe não cria garantias.

As garantias da linguagem são definidas pelo:

* Sistema de Tipos;
* Modelo de Memória;
* Semântica Operacional.

O Documento 04 apenas estabelece como essas construções são escritas pelo programador.

---

## 23.3 Evolução

A sintaxe poderá evoluir ao longo do desenvolvimento da linguagem.

Entretanto, qualquer evolução deverá preservar:

* compatibilidade com os princípios fundamentais;
* coerência com o Sistema de Tipos;
* coerência com o Modelo de Memória;
* estabilidade da experiência de desenvolvimento.

---

## 23.4 Separação entre Garantias e Implementação

Conforme estabelecido nos Documentos 01, 02 e 03, a Capi distingue explicitamente:

* **o que** a linguagem garante;
* **como** essas garantias são implementadas.

A sintaxe pertence exclusivamente ao primeiro aspecto: ela descreve as construções visíveis ao programador, sem impor decisões de implementação ao compilador.

---

# 24. Considerações Finais

A sintaxe da Capi foi projetada para preservar a familiaridade da Orientação a Objetos clássica, ao mesmo tempo em que incorpora um modelo moderno de segurança de memória.

O objetivo da linguagem não é reinventar a forma de escrever programas orientados a objetos, mas redefinir sua arquitetura interna sem impor um novo paradigma ao desenvolvedor.

Ao manter uma sintaxe próxima de linguagens como Java e C#, a curva de aprendizado permanece reduzida para a maioria dos desenvolvedores.

As principais inovações da Capi — Domains, `ObjectId`, Capacidade de Mutação e gerenciamento determinístico de memória — permanecem, na maior parte do tempo, transparentes ao programador.

Essa decisão permite que aplicações utilizem recursos avançados de segurança e desempenho sem sacrificar legibilidade ou produtividade.

Os documentos seguintes da especificação detalharão o comportamento operacional de cada construção apresentada aqui, transformando a sintaxe definida neste documento em um modelo formal de execução.

Este documento conclui a definição da superfície da linguagem. A partir do Documento 05 — Semântica Operacional, a especificação passa a descrever formalmente como cada instrução é analisada, validada e executada pelo compilador e pela infraestrutura da linguagem.
