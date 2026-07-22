# 10 — ABI e Interoperabilidade (FFI)

## Especificação da Linguagem Capi

Versão: 1.0 (Draft)

---

# 1. Introdução

Programas escritos em Capi não existem de forma isolada. Em diversos cenários, torna-se necessário interagir com bibliotecas do sistema operacional, componentes escritos em outras linguagens ou aplicações já existentes.

Essa interação exige um conjunto bem definido de regras que permitam a troca segura de dados e a chamada de funções entre diferentes ambientes de execução.

Essas regras constituem a **Application Binary Interface (ABI)** da linguagem.

Além da ABI, a Capi define mecanismos de **Foreign Function Interface (FFI)**, responsáveis por permitir a interoperabilidade entre código Capi e código externo.

---

## 1.1 Objetivo

Este documento especifica a interface binária oficial da linguagem Capi e os princípios gerais que regem sua interoperabilidade com outras linguagens.

São objetivos deste documento:

* definir a ABI oficial da linguagem;
* estabelecer a representação física dos principais elementos utilizados durante a execução;
* especificar as convenções de chamada entre módulos compilados;
* definir os princípios da interoperabilidade com código externo;
* preservar a compatibilidade binária entre implementações da linguagem.

A implementação física de cada plataforma permanece responsabilidade do Compilador e do Runtime.

---

## 1.2 Escopo

Este documento especifica exclusivamente:

* a interface binária da linguagem;
* os contratos entre código compilado e Runtime;
* os contratos entre código compilado e bibliotecas externas;
* os princípios gerais da interoperabilidade.

Não fazem parte deste documento:

* a sintaxe da linguagem;
* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* o Compilador;
* a Intermediate Representation (IR);
* a Biblioteca Padrão;
* o Runtime Mínimo;
* formatos de arquivos executáveis;
* formatos de bibliotecas dinâmicas.

---

## 1.3 Relação com os Documentos Anteriores

Este documento materializa, em nível binário, conceitos definidos pelos documentos anteriores.

Enquanto:

* o Sistema de Tipos define os tipos da linguagem;
* o Modelo de Memória define a organização lógica dos objetos;
* a Semântica Operacional define o comportamento dos programas;
* o Compilador gera código executável;
* o Runtime executa esse código;

a ABI estabelece como essas informações são representadas fisicamente durante a execução e como podem ser compartilhadas entre diferentes componentes do sistema.

---

# 2. Princípios da ABI

A ABI da Capi define os contratos binários utilizados entre componentes compilados.

Seu objetivo é garantir que diferentes módulos produzidos por compiladores compatíveis possam cooperar corretamente, independentemente da plataforma utilizada.

A ABI também estabelece a fronteira entre o ambiente seguro da linguagem e os mecanismos de interoperabilidade com código externo.

---

## 2.1 Papel da ABI

A ABI define como elementos da linguagem são representados durante a execução.

Entre eles:

* chamadas de funções;
* parâmetros;
* valores de retorno;
* representação física dos tipos;
* organização de objetos;
* convenções de chamada.

A ABI não altera a semântica da linguagem.

---

## 2.2 Independência da Linguagem

A especificação da linguagem permanece independente da ABI.

O comportamento dos programas é definido pelo Sistema de Tipos, pelo Modelo de Memória e pela Semântica Operacional.

A ABI apenas estabelece como esses conceitos são materializados durante a execução.

---

## 2.3 Independência da Plataforma

A ABI oficial da Capi define contratos abstratos.

Cada plataforma poderá adaptar esses contratos às convenções locais de chamada, alinhamento e organização binária, desde que preserve integralmente o comportamento definido pela especificação.

As adaptações específicas pertencem à implementação.

---

## 2.4 Compatibilidade Binária

Implementações compatíveis da linguagem deverão produzir módulos capazes de interoperar entre si sempre que utilizarem a mesma ABI.

Essa compatibilidade constitui um dos principais objetivos deste documento.

Alterações incompatíveis deverão resultar em novas versões da ABI.

---

## 2.5 Evolução

A ABI poderá evoluir em versões futuras da linguagem.

Novos recursos poderão ser incorporados desde que:

* preservem compatibilidade quando possível;
* mantenham claramente identificadas mudanças incompatíveis;
* não alterem as garantias fundamentais da linguagem.

---

# 3. Modelo Geral

A interoperabilidade da Capi baseia-se na existência de uma fronteira claramente definida entre o código produzido pelo Compilador e o ambiente externo.

Essa fronteira é estabelecida pela ABI e pelos mecanismos de Foreign Function Interface (FFI).

Toda comunicação entre a linguagem e componentes externos deverá respeitar os contratos definidos neste documento.

---

## 3.1 Código Capi

Código compilado em Capi utiliza integralmente:

* Sistema de Tipos;
* Modelo de Memória;
* Semântica Operacional;
* Runtime Mínimo;
* Biblioteca Padrão.

Enquanto permanecer dentro desse ambiente, todas as garantias da linguagem permanecem válidas.

---

## 3.2 Código Externo

Código escrito em outras linguagens não está sujeito às garantias da Capi.

Ao atravessar a fronteira da interoperabilidade, a linguagem deixa de controlar completamente:

* gerenciamento de memória;
* representação de objetos;
* sincronização;
* validade de referências;
* comportamento das funções chamadas.

Esses aspectos passam a depender do ambiente externo.

---

## 3.3 Fronteira da Interoperabilidade

Toda comunicação entre código Capi e código externo deverá ocorrer através da FFI.

Essa fronteira representa um ponto de transição entre dois ambientes distintos:

* o ambiente seguro da linguagem;
* o ambiente externo.

Cabe à ABI definir os contratos necessários para que essa transição ocorra de maneira consistente.

---

## 3.4 Chamadas entre Linguagens

Chamadas realizadas através da FFI deverão respeitar simultaneamente:

* a ABI da Capi;
* a ABI da plataforma de destino;
* as convenções exigidas pela linguagem externa.

O Compilador será responsável por gerar as adaptações necessárias para que essas chamadas ocorram corretamente.

---

## 3.5 Responsabilidades da ABI

Compete à ABI definir:

* representação física dos tipos;
* convenções de chamada;
* organização binária dos objetos;
* contratos de interoperabilidade;
* integração entre Compilador, Runtime e código externo.

Todos os demais aspectos permanecem responsabilidade da implementação.

---

# 4. Representação Física dos Tipos

A ABI define como os tipos da linguagem são representados durante a execução.

Essa representação constitui um contrato entre o Compilador, o Runtime e os componentes externos que interoperam com a Capi.

A representação física dos tipos não altera o Sistema de Tipos definido pelo Documento 02, limitando-se a materializar seus conceitos em nível binário.

---

## 4.1 Tipos Primitivos

Os tipos primitivos possuem representação binária compatível com a ABI da plataforma de destino.

Entre eles incluem-se:

* inteiros;
* números de ponto flutuante;
* booleanos;
* caracteres;
* ponteiros utilizados internamente pelo Runtime.

O tamanho, alinhamento e representação binária deverão respeitar as convenções da plataforma, preservando o comportamento definido pela linguagem.

---

## 4.2 Tipos por Valor

Tipos por valor são representados diretamente em memória.

Sua representação física contém exclusivamente os dados necessários para representar seu estado.

Quando utilizados como parâmetros ou valores de retorno, poderão ser:

* copiados;
* passados por registradores;
* passados por memória;

conforme definido pela ABI da plataforma.

---

## 4.3 Tipos por Identidade

Tipos por identidade são representados através dos mecanismos definidos pelo Runtime.

Sua representação física deverá preservar:

* identidade do objeto (`ObjectId`);
* lifetime;
* associação ao Domain;
* regras do Modelo de Memória.

A representação física utilizada para materializar o `ObjectId` pertence exclusivamente à implementação, desde que preserve sua unicidade e estabilidade durante todo o lifetime do objeto.

---

## 4.4 Generics

A ABI não define uma estratégia específica para representação de tipos genéricos.

O Compilador poderá utilizar:

* especialização (*monomorphization*);
* compartilhamento de código;
* dicionários de tipos;
* outras estratégias equivalentes.

Independentemente da estratégia adotada, a representação física produzida deverá permanecer compatível com os contratos definidos por esta ABI.

---

## 4.5 Alinhamento e Padding

A organização física dos tipos poderá exigir alinhamento e preenchimento (*padding*) para atender às restrições da arquitetura de destino.

Esses ajustes pertencem exclusivamente à implementação da ABI e não modificam a semântica da linguagem.

A disposição física dos campos deverá permanecer consistente entre módulos compilados utilizando a mesma ABI.

---

# 5. Layout de Objetos

Objetos por identidade possuem uma organização física utilizada pelo Runtime durante sua execução.

Esse layout representa apenas uma materialização dos conceitos definidos pelo Modelo de Memória.

A especificação não determina uma estrutura fixa para a implementação interna dos objetos, mas estabelece os contratos mínimos necessários para garantir interoperabilidade entre componentes compilados.

---

## 5.1 Organização Geral

Todo objeto deverá possuir uma representação física compatível com a ABI.

Essa representação poderá incluir:

* metadados internos;
* informações de tipo;
* informações para despacho dinâmico;
* área destinada aos campos declarados pela classe.

A existência e a organização desses elementos pertencem à implementação.

---

## 5.2 Campos

Os campos declarados por uma classe deverão possuir uma disposição física determinística.

Essa organização deverá permanecer consistente entre módulos compilados utilizando a mesma versão da ABI.

A ordem física dos campos poderá diferir da ordem utilizada no código-fonte, desde que essa transformação permaneça transparente ao programador e preserve a compatibilidade binária.

---

## 5.3 Herança

A representação física de classes derivadas deverá preservar integralmente as regras de herança definidas pelo Sistema de Tipos.

A representação deverá respeitar o princípio de **layout prefixado** estabelecido pelo Modelo de Memória.

A porção correspondente à superclasse ocupa o início da representação física da subclasse, permitindo que operações de *upcast* sejam realizadas sem necessidade de conversões, cópias ou reorganização dos dados.

Implementações poderão adotar diferentes estratégias para organizar a representação física dos objetos, incluindo metadados internos e os campos introduzidos pela própria subclasse, desde que:

* preservem o layout prefixado da superclasse;
* preservem a identidade do objeto;
* garantam compatibilidade entre chamadas polimórficas;
* mantenham comportamento consistente durante toda a execução.

---

## 5.4 Interfaces

Interfaces representam contratos de comportamento e não impõem, por si só, uma organização física específica aos objetos.

A implementação poderá utilizar tabelas de despacho, descritores ou outros mecanismos equivalentes para suportar chamadas através de interfaces.

A estratégia adotada pertence exclusivamente à implementação.

---

## 5.5 Traits

Traits não alteram a representação física fundamental dos objetos.

Sua integração poderá ser realizada através de mecanismos definidos pelo Compilador e pelo Runtime, preservando integralmente os contratos estabelecidos pelo Sistema de Tipos.

A ABI exige apenas que módulos compilados compartilhem uma representação compatível para permitir chamadas corretas entre componentes produzidos por diferentes compilações.

---

# 6. Convenção de Chamadas

A ABI da Capi define os contratos utilizados para chamadas entre funções compiladas e para interoperabilidade com componentes externos.

Seu objetivo é garantir que parâmetros, valores de retorno e contexto de execução sejam interpretados de forma consistente por todas as implementações compatíveis.

A convenção concreta utilizada poderá variar conforme a plataforma, desde que preserve os contratos definidos neste documento.

---

## 6.1 Parâmetros

Parâmetros deverão ser transferidos conforme as convenções da ABI da plataforma de destino.

O Compilador poderá utilizar:

* registradores;
* pilha;
* memória temporária;
* mecanismos equivalentes.

A estratégia utilizada pertence exclusivamente à implementação.

---

## 6.2 Valores de Retorno

O retorno de funções deverá seguir a convenção definida pela ABI da plataforma.

Dependendo do tipo retornado, a implementação poderá utilizar:

* registradores;
* memória previamente alocada pelo chamador;
* estruturas temporárias;
* outros mecanismos compatíveis.

Essa decisão pertence exclusivamente à implementação.

---

## 6.3 Passagem por Valor

Tipos por valor poderão ser transmitidos integralmente durante chamadas de função.

A implementação poderá decidir entre cópia direta, passagem por registradores ou utilização de memória auxiliar, desde que preserve a semântica definida pela linguagem.

O comportamento observado pelo programa deverá permanecer independente da estratégia utilizada.

---

## 6.4 Passagem por Identidade

Objetos por identidade não são copiados durante chamadas de função.

A ABI deverá preservar:

* a identidade do objeto;
* seu lifetime;
* sua associação ao Domain de origem.

A representação física utilizada para essa passagem pertence exclusivamente à implementação.

---

## 6.5 Convenções da Plataforma

A ABI oficial da Capi adapta-se às convenções binárias da plataforma de destino.

Aspectos como:

* utilização de registradores;
* organização da pilha;
* alinhamento;
* convenções de chamada;
* convenções de retorno;

deverão seguir as regras da plataforma, preservando integralmente a semântica da linguagem.

---

# 7. Representação de Domains

Domains constituem a unidade fundamental de organização da memória da Capi.

Embora sejam um conceito definido pelo Modelo de Memória, sua existência durante a execução precisa ser representada de forma compatível com a ABI.

Essa representação não torna Domains parte da API pública da linguagem nem altera sua natureza conceitual.

---

## 7.1 Domains na ABI

Domains não constituem tipos manipuláveis diretamente pelo programador.

Na ABI, representam contextos internos de execução utilizados pelo Runtime para preservar:

* lifetime;
* isolamento;
* gerenciamento de objetos;
* sincronização;
* integridade da memória.

Sua representação física pertence exclusivamente à implementação.

---

## 7.2 Lifetime

Todo objeto permanece associado ao Domain onde foi criado.

A ABI deverá preservar essa associação durante toda a execução.

Nenhuma chamada entre módulos poderá alterar implicitamente o Domain de um objeto.

---

## 7.3 ObjectId

A identidade dos objetos deverá permanecer estável independentemente da quantidade de chamadas realizadas entre módulos.

A ABI deverá preservar integralmente o conceito de `ObjectId` definido pelo Modelo de Memória.

A forma física utilizada para representar essa identidade pertence exclusivamente à implementação.

---

## 7.4 Ownership

A ABI não modifica as regras de ownership definidas pela Semântica Operacional e pelo Modelo de Memória.

Chamadas entre módulos não transferem implicitamente:

* ownership;
* lifetime;
* responsabilidade pela destruição dos objetos.

Qualquer mecanismo de transferência deverá estar explicitamente previsto pela linguagem.

---

## 7.5 Limites da ABI

A ABI representa apenas a fronteira binária entre componentes.

As regras relacionadas a:

* criação de Domains;
* destruição;
* gerenciamento de memória;
* concorrência;
* Capacidade de Mutação;

continuam pertencendo ao Runtime Mínimo, ao Modelo de Memória e à Semântica Operacional.

A ABI apenas garante que essas informações possam ser representadas de maneira consistente durante a execução.

---

# 8. Foreign Function Interface (FFI)

A **Foreign Function Interface (FFI)** permite que programas escritos em Capi interoperem com bibliotecas e componentes implementados em outras linguagens.

A FFI representa a fronteira entre o ambiente seguro da linguagem e ambientes cuja semântica e garantias não são controladas pela Capi.

Toda interoperabilidade deverá respeitar simultaneamente a ABI da Capi e a ABI da plataforma de destino.

As operações de interoperabilidade dependem dos serviços disponibilizados pelo Runtime Mínimo.

Entre eles incluem-se:

* carregamento de bibliotecas;
* resolução de símbolos;
* preparação do ambiente de execução;
* adaptação às convenções da plataforma;
* gerenciamento das chamadas entre código Capi e código externo.

A forma como esses serviços são implementados pertence exclusivamente ao Runtime.

---

## 8.1 Objetivos

A FFI tem como objetivos:

* permitir chamadas para funções externas;
* permitir que código externo invoque funções escritas em Capi;
* possibilitar integração com bibliotecas existentes;
* preservar, sempre que possível, as garantias da linguagem;
* limitar claramente a fronteira entre código seguro e código externo.

---

## 8.2 Declaração de Funções Externas

A linguagem poderá fornecer mecanismos para declarar funções implementadas externamente.

Essas declarações informam ao Compilador:

* nome da função;
* convenção de chamada;
* parâmetros;
* tipo de retorno;
* biblioteca de origem, quando necessário.

A sintaxe utilizada para essas declarações pertence ao Documento 04 — Sintaxe da Linguagem.

---

## 8.3 Chamadas para Código Externo

Chamadas realizadas através da FFI deverão respeitar:

* a ABI da plataforma;
* a assinatura declarada;
* as regras definidas para a interoperabilidade.

O Compilador será responsável por gerar as adaptações necessárias entre a representação utilizada pela Capi e a representação esperada pelo código externo.

---

## 8.4 Chamadas para Código Capi

A ABI da Capi poderá permitir que componentes externos invoquem funções escritas na linguagem.

Nesse caso, o Compilador deverá gerar pontos de entrada compatíveis com a ABI da plataforma e realizar as adaptações necessárias para integrar esses parâmetros ao ambiente da linguagem.

---

## 8.5 Bibliotecas Dinâmicas

A especificação não define formatos específicos para bibliotecas compartilhadas.

Cada implementação poderá utilizar os mecanismos disponibilizados pela plataforma, incluindo:

* bibliotecas dinâmicas;
* bibliotecas estáticas;
* módulos compartilhados;
* outros formatos equivalentes.

A interoperabilidade deverá permanecer compatível com os contratos estabelecidos pela ABI.

---

# 9. Código `unsafe`

A interoperabilidade com código externo frequentemente exige operações que não podem ser verificadas integralmente pelo Sistema de Tipos.

Por esse motivo, operações relacionadas à FFI pertencem ao contexto de código `unsafe`.

O uso de `unsafe` representa uma transferência explícita de responsabilidade do compilador para o programador.

---

## 9.1 Papel do `unsafe`

Blocos `unsafe` delimitam regiões nas quais determinadas garantias da linguagem deixam de poder ser verificadas integralmente.

Isso não altera o comportamento da linguagem, mas reduz o conjunto de garantias que o Compilador pode oferecer.

---

## 9.2 Limites das Garantias

Dentro de blocos `unsafe`, o Compilador deixa de garantir aspectos como:

* validade de ponteiros externos;
* compatibilidade de assinaturas declaradas;
* validade de estruturas provenientes de outras linguagens;
* respeito ao Modelo de Memória por código externo.

As demais garantias da linguagem permanecem válidas sempre que aplicáveis.

A ABI da Capi não oferece garantias sobre o comportamento, integridade ou segurança de componentes externos.

Essas responsabilidades pertencem exclusivamente ao código `unsafe` e à linguagem ou biblioteca utilizada durante a interoperabilidade.

---

## 9.3 Conversões

Conversões entre representações da Capi e representações externas poderão ocorrer durante chamadas FFI.

Essas conversões deverão preservar, sempre que possível:

* integridade dos dados;
* representação dos tipos;
* compatibilidade binária.

Conversões incompatíveis constituem responsabilidade do código `unsafe`.

---

## 9.4 Ponteiros

A manipulação direta de ponteiros externos poderá ser permitida apenas em contexto `unsafe`.

O Runtime e o Compilador não oferecem garantias quanto à validade, lifetime ou integridade desses ponteiros.

Sua utilização permanece sob responsabilidade do programador.

---

## 9.5 Responsabilidade do Programador

Ao utilizar mecanismos de FFI, cabe ao programador garantir:

* compatibilidade das assinaturas;
* validade dos parâmetros;
* respeito às convenções da ABI externa;
* gerenciamento correto de recursos compartilhados;
* conformidade com as regras da linguagem externa.

A Capi não oferece garantias sobre o comportamento de código externo.

---

# 10. Strings e Arrays

Strings e arrays representam estruturas frequentemente utilizadas durante a interoperabilidade com outras linguagens.

Este capítulo estabelece os princípios gerais para sua representação na fronteira da ABI.

As representações concretas poderão variar conforme a plataforma e a implementação.

---

## 10.1 Strings

O tipo `String` é definido pelo Sistema de Tipos da linguagem.

Sua representação interna pertence exclusivamente à implementação da Capi e não precisa ser compatível com qualquer representação utilizada por linguagens externas.

A ABI não exige que essa representação seja compatível com qualquer formato utilizado por linguagens externas.

Quando necessário, o Compilador poderá gerar conversões apropriadas durante chamadas FFI.

---

## 10.2 Arrays

Arrays deverão preservar sua organização lógica conforme definida pelo Sistema de Tipos.

Sua representação física poderá incluir informações adicionais necessárias ao Runtime, como tamanho ou metadados.

A forma exata dessa representação pertence à implementação.

---

## 10.3 Buffers

Durante operações de interoperabilidade, implementações poderão converter arrays ou strings para buffers compatíveis com a linguagem externa.

Essas conversões deverão preservar integralmente o conteúdo dos dados.

Os mecanismos utilizados pertencem exclusivamente à implementação.

---

## 10.4 Unicode

Strings da Capi utilizam Unicode conforme definido pela Biblioteca Padrão.

Durante chamadas FFI, conversões entre codificações poderão ser necessárias para atender aos requisitos da linguagem ou biblioteca externa.

A implementação dessas conversões pertence ao Compilador e ao Runtime.

---

## 10.5 Compartilhamento

O compartilhamento de strings, arrays ou buffers entre Capi e código externo não altera as regras do Modelo de Memória.

Lifetime, ownership e gerenciamento dos recursos compartilhados deverão permanecer claramente definidos pela interface utilizada.

A ABI apenas estabelece os contratos binários necessários para que esse compartilhamento ocorra de forma consistente.

---

# 11. Tratamento de Erros

A interoperabilidade entre Capi e código externo estabelece uma fronteira entre ambientes que podem possuir modelos distintos de tratamento de erros.

A ABI não impõe um mecanismo único para linguagens externas, mas define como essas diferenças devem ser tratadas para preservar a consistência da execução.

Os mecanismos públicos de tratamento de erros da Capi permanecem aqueles definidos pelo Sistema de Tipos e pela Biblioteca Padrão.

---

## 11.1 Erros da ABI

Falhas relacionadas à interoperabilidade poderão ocorrer durante:

* resolução de símbolos externos;
* carregamento de bibliotecas;
* incompatibilidade de assinaturas;
* conversões entre representações incompatíveis;
* chamadas para componentes externos.

A forma como essas falhas são detectadas pertence à implementação.

---

## 11.2 Integração com `Result<T,E>`

Sempre que possível, falhas previsíveis da interoperabilidade deverão ser representadas utilizando `Result<T,E>`.

Essa abordagem mantém consistência com os mecanismos de tratamento de erros adotados pela linguagem.

A definição dos tipos concretos de erro pertence à Biblioteca Padrão.

---

## 11.3 Integração com `Optional<T>`

Quando uma operação puder resultar legitimamente na ausência de um valor, a interoperabilidade poderá utilizar `Optional<T>`.

Essa situação não representa uma condição excepcional, mas apenas a inexistência de um resultado válido.

---

## 11.4 Falhas Externas

Falhas originadas em componentes externos permanecem responsabilidade desses componentes.

O Runtime e o Compilador não podem assumir que bibliotecas externas respeitam:

* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* as garantias de segurança da Capi.

A interoperabilidade deverá tratar essas situações apenas na medida em que isso seja possível sem comprometer as garantias da linguagem.

Erros originados em código externo, como violações de acesso, corrupção de memória ou falhas de segmentação, não podem ser representados por `Result<T,E>` ou `Optional<T>`, pois ocorrem fora do domínio de controle da linguagem.

Seu tratamento depende da plataforma, do Runtime e dos mecanismos de interoperabilidade utilizados.

---

## 11.5 Independência

A ABI não define mecanismos específicos de propagação de erros entre linguagens.

Cada implementação poderá adaptar esses mecanismos às características da plataforma e da linguagem externa, preservando sempre os contratos definidos pela especificação da Capi.

---

# 12. Compatibilidade entre Implementações

Um dos principais objetivos da ABI oficial da Capi é permitir que diferentes implementações da linguagem produzam componentes binários capazes de interoperar corretamente.

Essa compatibilidade favorece a evolução do ecossistema e reduz dependências entre implementações específicas.

---

## 12.1 ABI Oficial

A especificação da Capi define uma ABI oficial.

Implementações compatíveis deverão respeitar integralmente seus contratos públicos.

Aspectos internos poderão variar livremente, desde que permaneçam transparentes para os componentes que utilizam essa ABI.

---

## 12.2 Implementações Alternativas

Compiladores, Runtimes e Bibliotecas Padrão poderão possuir implementações independentes.

Todas deverão preservar:

* representação binária compatível;
* convenções de chamada;
* contratos públicos da ABI;
* comportamento definido pela linguagem.

---

## 12.3 Compatibilidade Binária

Módulos compilados por implementações diferentes poderão interoperar quando utilizarem a mesma versão da ABI oficial.

Essa compatibilidade depende da preservação integral dos contratos estabelecidos por este documento.

Diferenças internas de implementação não deverão afetar essa interoperabilidade.

---

## 12.4 Versionamento

Alterações incompatíveis da ABI deverão resultar em uma nova versão oficialmente identificada.

O mecanismo de identificação da versão pertence à implementação.

Essa identificação poderá ser registrada em metadados do executável, bibliotecas compartilhadas, símbolos especiais ou outros mecanismos equivalentes definidos pela Toolchain.

O objetivo é permitir que incompatibilidades de ABI sejam detectadas durante a ligação ou o carregamento dos componentes.

---

## 12.5 Evolução

A ABI poderá evoluir para suportar novos recursos da linguagem.

Essa evolução deverá buscar preservar compatibilidade binária sempre que tecnicamente possível.

Quando incompatibilidades forem inevitáveis, deverão ser claramente identificadas por meio do versionamento da ABI.

---

# 13. Segurança

A interoperabilidade representa o principal ponto de contato entre o ambiente seguro da Capi e componentes cuja execução não está sujeita às garantias da linguagem.

A ABI estabelece princípios destinados a reduzir riscos, preservando ao máximo a integridade dos programas.

---

## 13.1 Limites da Linguagem

As garantias oferecidas pela Capi aplicam-se exclusivamente ao código executado dentro do ambiente da linguagem.

Código externo permanece sujeito às regras da linguagem ou plataforma onde foi desenvolvido.

A interoperabilidade não amplia automaticamente essas garantias.

---

## 13.2 Integridade dos Dados

A ABI deverá preservar a integridade dos dados transferidos entre componentes sempre que suas representações forem compatíveis.

Quando conversões forem necessárias, o Compilador e o Runtime deverão realizá-las de forma consistente com os contratos definidos por esta especificação.

---

## 13.3 Segurança de Memória

O acesso à memória pertencente ao ambiente externo deverá ocorrer exclusivamente através dos mecanismos definidos pela FFI.

A validade dessa memória não pode ser garantida pela Capi.

Cabe ao código `unsafe` assumir a responsabilidade pelas verificações necessárias.

---

## 13.4 Isolamento

A interoperabilidade não modifica o isolamento entre Domains estabelecido pelo Modelo de Memória.

Objetos pertencentes à Capi permanecem sujeitos às regras de lifetime, identidade e gerenciamento definidas pela linguagem, independentemente das operações realizadas através da FFI.

---

## 13.5 Evolução

Novos mecanismos de interoperabilidade poderão ser incorporados em versões futuras.

Esses mecanismos não poderão reduzir as garantias fundamentais da linguagem nem enfraquecer os contratos estabelecidos por esta ABI.

---

# 14. Não Objetivos

Este documento define exclusivamente a **Application Binary Interface (ABI)** oficial da Capi e os princípios gerais de interoperabilidade entre código Capi e componentes externos.

Diversos aspectos permanecem deliberadamente fora de seu escopo para preservar a separação entre especificação da linguagem, implementação do compilador, Runtime e plataforma de destino.

---

## 14.1 ABI Específica da Plataforma

Este documento não especifica detalhes particulares de nenhuma arquitetura de processador ou sistema operacional.

Aspectos como:

* registradores utilizados;
* organização da pilha;
* convenções específicas de chamada;
* alinhamento físico da memória;
* formato binário dos executáveis;

pertencem às ABIs das plataformas suportadas.

A ABI da Capi define apenas os contratos abstratos que deverão ser preservados por todas as implementações.

---

## 14.2 Formatos de Executáveis

A especificação não define formatos de arquivos executáveis ou bibliotecas.

Entre eles:

* ELF;
* PE;
* Mach-O;
* WebAssembly;
* outros formatos equivalentes.

A escolha do formato pertence exclusivamente ao Compilador e à plataforma de destino.

---

## 14.3 Linkers

A especificação não determina:

* mecanismos de ligação estática;
* ligação dinâmica;
* resolução de símbolos;
* carregamento de bibliotecas;
* organização do processo de linkedição.

Esses mecanismos pertencem ao ambiente de compilação e às ferramentas da plataforma.

---

## 14.4 Ferramentas Externas

Este documento não especifica:

* depuradores;
* analisadores estáticos;
* profiladores;
* geradores de documentação;
* ferramentas de instrumentação.

Esses componentes poderão utilizar informações disponibilizadas pela ABI, mas não fazem parte de sua definição.

---

## 14.5 Otimizações

A ABI não determina estratégias de otimização.

Cada implementação poderá utilizar técnicas como:

* especialização de chamadas;
* eliminação de conversões;
* inline entre módulos;
* otimizações de layout;
* otimizações específicas da arquitetura;

desde que preserve integralmente os contratos públicos definidos por esta especificação.

---

# 15. Princípio da Separação entre Garantias e Implementação

Assim como os demais documentos da especificação da Capi, este documento distingue claramente entre as garantias oferecidas pela ABI e os mecanismos utilizados para implementá-las.

A especificação define **o que** deve ser preservado durante a interoperabilidade.

Cada implementação define **como** esses contratos serão materializados na plataforma de destino.

---

## 15.1 Garantias

Constituem garantias obrigatórias da ABI:

* compatibilidade binária entre componentes;
* representação consistente dos tipos;
* convenções de chamada compatíveis;
* preservação da identidade dos objetos;
* integração correta com o Runtime;
* interoperabilidade consistente com código externo.

Essas garantias fazem parte da especificação oficial da linguagem.

---

## 15.2 Implementação

Pertencem exclusivamente à implementação:

* organização física das estruturas internas;
* adaptação às convenções da plataforma;
* algoritmos de conversão;
* geração de código de interoperabilidade;
* estratégias de otimização;
* mecanismos internos do Runtime utilizados durante chamadas FFI.

Implementações diferentes poderão adotar soluções distintas para esses aspectos.

---

## 15.3 Evolução

A ABI poderá evoluir continuamente para acomodar novos recursos da linguagem e novas plataformas.

Essa evolução deverá preservar, sempre que possível:

* compatibilidade binária;
* estabilidade das interfaces públicas;
* interoperabilidade entre implementações.

Mudanças incompatíveis deverão ser formalmente identificadas por meio do versionamento da ABI.

---

## 15.4 Compatibilidade

A evolução da ABI não deverá impor alterações desnecessárias ao Compilador, ao Runtime ou à Biblioteca Padrão.

Da mesma forma, a evolução desses componentes deverá preservar os contratos públicos definidos pela ABI sempre que permanecerem compatíveis com a mesma versão da especificação.

---

## 15.5 Estabilidade

Os contratos públicos definidos por este documento constituem a interface oficial entre o código compilado, o Runtime e os componentes externos.

Sua evolução deverá ocorrer de forma controlada, preservando a estabilidade do ecossistema e mantendo a separação entre garantias da linguagem e mecanismos de implementação.

---

# 16. Considerações Finais

A ABI e a Foreign Function Interface completam a infraestrutura fundamental da linguagem Capi ao definir a fronteira entre o ambiente seguro da linguagem e o restante do ecossistema computacional.

Enquanto os documentos anteriores estabeleceram os conceitos semânticos da linguagem, este documento define como esses conceitos são representados fisicamente durante a execução e como podem ser compartilhados de maneira consistente entre módulos compilados e componentes externos.

Ao separar claramente a especificação da ABI das decisões particulares de cada plataforma, a Capi preserva simultaneamente dois objetivos fundamentais: a portabilidade da linguagem e a eficiência das implementações.

Essa abordagem permite que diferentes Compiladores e Runtimes produzam componentes binários compatíveis, favorecendo um ecossistema aberto e interoperável, sem restringir a evolução tecnológica das implementações.

Com a conclusão deste documento, encontram-se especificados todos os elementos necessários para a compilação, execução e interoperabilidade de programas escritos em Capi.

O documento seguinte, **11 — Toolchain (capi, capic, capi fmt, capi test, capi doc)**, definirá as ferramentas oficiais disponibilizadas pela linguagem, estabelecendo a organização do ambiente de desenvolvimento, compilação, testes, formatação, documentação e distribuição de aplicações Capi.
