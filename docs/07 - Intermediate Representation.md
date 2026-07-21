# 07 — Intermediate Representation (IR)

## Especificação da Linguagem Capi

Versão: 1.0 (Draft)

---

# 1. Introdução

A **Intermediate Representation (IR)** constitui a representação intermediária oficial utilizada internamente pelo compilador da Capi.

Sua finalidade é servir como ponte entre a análise semântica realizada pelo Frontend e as etapas de otimização e geração de código executadas pelo Middle-end e pelo Backend.

A IR não faz parte da linguagem visível ao programador. Trata-se de uma estrutura interna do compilador, projetada para representar programas de forma precisa, consistente e independente da sintaxe original.

---

## 1.1 Objetivo

Este documento especifica os princípios, responsabilidades e propriedades da Representação Intermediária da Capi.

São objetivos deste documento:

* definir o papel da IR na arquitetura do compilador;
* estabelecer as garantias que a IR deve preservar;
* descrever os conceitos semânticos representados internamente;
* fornecer uma base comum para otimizações e geração de código.

A organização física da IR pertence à implementação do compilador e não faz parte da especificação da linguagem.

---

## 1.2 Escopo

Este documento descreve exclusivamente a representação intermediária produzida após a conclusão da Análise Semântica.

Não fazem parte deste documento:

* a sintaxe da linguagem (Documento 04);
* o Sistema de Tipos (Documento 02);
* o Modelo de Memória (Documento 03);
* a Semântica Operacional (Documento 05);
* a arquitetura geral do compilador (Documento 06);
* formato físico da IR utilizado por uma implementação específica;

Nota: representação física de tipos, convenções de chamada e interoperabilidade com código externo, definidos no Documento 10 — ABI e Interoperabilidade (FFI).

---

## 1.3 Relação com a Arquitetura do Compilador

Conforme definido no Documento 06 — Arquitetura do Compilador, a geração da IR representa a transição entre o **Frontend** e o **Middle-end** do compilador.

Após essa etapa:

* o código-fonte deixa de ser utilizado;
* a AST torna-se somente leitura;
* todas as otimizações passam a operar sobre a IR;
* o Backend recebe exclusivamente a IR como entrada.

A IR constitui, portanto, a representação canônica do programa durante todo o restante do processo de compilação.

---

# 2. Papel da Intermediate Representation

A IR representa o programa em um nível de abstração intermediário entre a linguagem fonte e o código executável.

Ela preserva os conceitos fundamentais da Capi, eliminando apenas os elementos puramente sintáticos presentes no código-fonte.

---

## 2.1 Representação Canônica

Todo programa válido da Capi possui exatamente uma representação semântica na IR.

Embora implementações diferentes possam utilizar estruturas internas distintas, todas devem representar exatamente o mesmo comportamento observável do programa.

A IR constitui a forma canônica utilizada pelo compilador após a Análise Semântica.

A Biblioteca Padrão da Capi utiliza exatamente o mesmo pipeline de compilação definido para qualquer programa escrito na linguagem. Dessa forma, suas implementações também são representadas pela IR descrita neste documento, permitindo que o mesmo conjunto de otimizações seja aplicado de maneira uniforme a todo o ecossistema da linguagem.

---

## 2.2 Fronteira entre Frontend e Backend

A IR estabelece uma separação clara entre:

* análise da linguagem;
* otimizações;
* geração de código.

Essa separação permite que diferentes Frontends compartilhem o mesmo Backend e que diferentes Backends utilizem a mesma representação intermediária.

---

## 2.3 Base para Otimizações

Todas as otimizações realizadas pelo compilador operam sobre a IR.

Essas otimizações poderão:

* reorganizar instruções;
* eliminar redundâncias;
* especializar chamadas;
* simplificar expressões;
* melhorar desempenho.

Nenhuma otimização poderá alterar a semântica do programa.

---

## 2.4 Independência da Plataforma

A IR não representa características específicas de nenhuma arquitetura de hardware ou sistema operacional.

Aspectos como:

* registradores;
* convenções de chamada;
* instruções de máquina;
* organização física da memória;
* ABI;

pertencem exclusivamente ao Backend.

---

# 3. Princípios da Intermediate Representation

A IR da Capi foi projetada para preservar explicitamente os conceitos fundamentais da linguagem.

Ela não é uma representação de baixo nível equivalente a código de máquina, mas uma representação semântica do programa.

---

## 3.1 Preservação da Semântica

Toda transformação realizada sobre a IR deverá preservar integralmente:

* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* todas as garantias estabelecidas pela especificação da linguagem.

---

## 3.2 Representação Semântica

A IR preserva explicitamente conceitos próprios da Capi, incluindo:

* Domains;
* ObjectId;
* Classes;
* Interfaces;
* Traits;
* Lifetime;
* Capacidade de Mutação.

Esses conceitos permanecem representados durante as etapas de otimização, sendo convertidos para representações específicas da plataforma apenas durante a geração de código pelo Backend.

---

## 3.3 Independência da Sintaxe

A IR não preserva elementos pertencentes exclusivamente ao código-fonte.

Entre eles:

* espaços em branco;
* comentários;
* formatação;
* convenções de estilo;
* organização textual.

Apenas a estrutura semântica do programa é representada.

---

## 3.4 Independência da Implementação

A estrutura física utilizada para representar a IR não faz parte desta especificação.

Cada implementação poderá utilizar:

* SSA (*Static Single Assignment*);
* grafos;
* árvores intermediárias;
* bytecode interno;
* representações híbridas;
* outras técnicas equivalentes.

Todas deverão preservar exatamente a mesma semântica observável do programa.

---

## 3.5 Base para Evolução

A IR foi projetada para permitir a evolução futura da linguagem.

Novos recursos sintáticos, novos mecanismos do Sistema de Tipos e novos conceitos semânticos poderão ser incorporados à linguagem sem exigir alterações fundamentais na arquitetura da IR.

Essa estabilidade favorece a evolução independente do compilador, do Backend e da própria linguagem, preservando o Princípio da Separação entre Garantias e Implementação estabelecido nos documentos anteriores.

---

# 4. Organização Geral da IR

A Intermediate Representation da Capi é organizada em níveis hierárquicos, refletindo a estrutura lógica do programa após a Análise Semântica.

Essa organização não corresponde necessariamente à forma como o código-fonte foi escrito, mas sim à forma como o compilador compreende o programa.

---

## 4.1 Organização Hierárquica

Conceitualmente, a IR pode ser representada pela seguinte estrutura:

```text
Module
 ├── Types
 ├── Globals
 ├── Functions
 │     ├── Parameters
 │     ├── Basic Blocks
 │     │      ├── Instructions
 │     │      └── Terminators
 │     └── Metadata
 └── Module Metadata
```

Cada elemento possui responsabilidade específica e bem definida.

---

## 4.2 Módulos

O módulo constitui a unidade lógica superior da IR.

Ele agrupa todos os elementos necessários para representar um conjunto coerente de código compilado, incluindo:

* tipos;
* classes;
* interfaces;
* traits;
* funções;
* constantes globais;
* metadados.

A representação física dos módulos pertence à implementação.

---

## 4.3 Funções

Toda rotina executável da linguagem é representada na IR como uma função.

Isso inclui:

* métodos;
* funções livres;
* construtores;
* destrutores;
* inicializadores;
* funções geradas automaticamente pelo compilador.

Independentemente de sua origem, todas compartilham uma representação uniforme na IR.

---

## 4.4 Blocos Básicos

Cada função é composta por um conjunto de **Blocos Básicos (Basic Blocks)**.

Um bloco básico representa uma sequência contínua de instruções que:

* possui um único ponto de entrada;
* possui um único ponto de saída;
* não contém desvios internos.

A organização em blocos básicos facilita análises de fluxo de controle e otimizações.

---

## 4.5 Instruções

As instruções representam operações elementares da IR.

Exemplos incluem:

* criação de objetos;
* leitura de campos;
* escrita de campos;
* chamadas de métodos;
* operações aritméticas;
* comparações;
* saltos condicionais;
* retornos.

A lista completa de instruções será definida ao longo deste documento.

---

## 4.6 Metadados

A IR poderá conter metadados auxiliares utilizados pelo compilador.

Entre eles:

* informações de depuração;
* localização no código-fonte;
* atributos de otimização;
* informações para análise estática;
* anotações internas do compilador.

Esses metadados não alteram a semântica do programa.

---

# 5. Sistema de Tipos da IR

Embora preserve o Sistema de Tipos definido pelo Documento 02, a IR utiliza uma representação própria para os tipos manipulados internamente pelo compilador.

Essa representação é voltada à análise e à geração de código, e não à interação direta com o programador.

---

## 5.1 Objetivos

O Sistema de Tipos da IR possui como objetivos:

* representar todos os tipos da linguagem;
* permitir verificações internas do compilador;
* facilitar otimizações;
* simplificar a geração de código;
* preservar integralmente a semântica do Sistema de Tipos da Capi.

---

## 5.2 Tipos Primitivos

A IR representa explicitamente os tipos primitivos definidos pela linguagem.

Exemplos:

* `Int8`
* `Int16`
* `Int32`
* `Int64`
* `UInt8`
* `UInt16`
* `UInt32`
* `UInt64`
* `Float32`
* `Float64`
* `Bool`
* `Char`
* `Unit`

A representação física desses tipos pode variar conforme a arquitetura de destino.

---

## 5.3 Tipos Compostos

Além dos tipos primitivos, a IR representa tipos compostos, incluindo:

* estruturas;
* tuplas;
* arrays;
* funções;
* tipos genéricos especializados;
* tipos opcionais (`Optional<T>`);
* resultados (`Result<T,E>`).

Esses tipos preservam integralmente as propriedades definidas pelo Sistema de Tipos da linguagem.

---

## 5.4 Tipos Semânticos

Além dos tipos convencionais, a IR preserva tipos semânticos próprios da arquitetura da Capi.

Entre eles:

* referências de identidade (`ObjectId`);
* Domains;
* capacidades de mutação;
* informações de lifetime.

Esses elementos não constituem apenas detalhes de implementação; representam conceitos centrais da linguagem que permanecem explícitos durante toda a fase intermediária de compilação.

---

## 5.5 Compatibilidade

Todas as operações realizadas sobre a IR deverão respeitar as regras de compatibilidade estabelecidas pelo Sistema de Tipos.

Nenhuma otimização poderá produzir uma IR que viole:

* regras de herança;
* variância;
* conversões válidas;
* restrições de generics;
* regras de identidade;
* garantias do Modelo de Memória.

---

## 5.6 Representação de Generics

A IR preserva a natureza parametrizada dos tipos genéricos definidos pela linguagem.

Dependendo da estratégia adotada pela implementação, o compilador poderá:

* manter representações genéricas durante toda a compilação;
* produzir versões especializadas para tipos concretos;
* combinar ambas as estratégias.

A escolha pertence exclusivamente à implementação, desde que todas as regras do Sistema de Tipos permaneçam preservadas.

---

# 6. Representação de Objetos

A IR preserva explicitamente a existência dos objetos definidos pela linguagem.

Ao contrário de representações de baixo nível baseadas exclusivamente em endereços de memória, a IR continua distinguindo claramente identidade, comportamento e armazenamento físico.

---

## 6.1 Objetos na IR

Cada objeto é representado por uma entidade semântica que mantém sua identidade lógica durante todo o processo de compilação.

Essa representação permanece independente da forma física que será utilizada pelo Backend.

---

## 6.2 Identidade

A identidade de um objeto é preservada através do conceito de `ObjectId`.

A IR nunca substitui identidades por ponteiros físicos ou offsets de memória.

A tradução para representações específicas da arquitetura ocorre apenas durante a geração de código.

---

## 6.3 Estado

O estado de um objeto continua pertencendo ao **Domain** correspondente.

A IR representa explicitamente essa relação, preservando a separação entre:

* identidade;
* comportamento;
* armazenamento físico.

Essa distinção constitui um dos princípios fundamentais da arquitetura da Capi.

---

## 6.4 Operações sobre Objetos

As operações representadas pela IR incluem, entre outras:

* criação;
* leitura;
* escrita;
* chamadas de métodos;
* comparação de identidade;
* destruição lógica.

Essas operações permanecem independentes da forma física utilizada pela implementação.

---

## 6.5 Independência da Representação Física

A IR não especifica:

* layout em memória;
* posição de campos;
* tabelas virtuais;
* offsets;
* ponteiros.

Esses aspectos pertencem exclusivamente ao Backend e ao Runtime Mínimo.

A responsabilidade da IR é preservar os conceitos semânticos necessários para que essas estruturas possam ser geradas posteriormente sem perda de informação.

---

# 7. Representação de Domains

O **Domain** permanece como uma entidade semântica explícita durante toda a existência da IR.

Ao contrário de representações intermediárias tradicionais, que reduzem toda a memória a ponteiros e regiões genéricas, a IR da Capi preserva o conceito de **Domain** como unidade lógica de gerenciamento de memória, identidade e lifetime.

Essa preservação permite que otimizações específicas da linguagem sejam realizadas sem perda das garantias estabelecidas pelo Modelo de Memória.

---

## 7.1 Domain como Entidade Semântica

Na IR, um Domain representa um contexto lógico de armazenamento.

Ele continua sendo responsável por:

* agrupar objetos relacionados;
* definir o lifetime coletivo dos objetos;
* preservar a integridade das identidades (`ObjectId`);
* delimitar o escopo da Capacidade de Mutação.

A IR não representa Domains como simples blocos de memória.

Nota: a criação lógica de Domains é representada explicitamente pela IR através das operações definidas no Capítulo 12.

---

## 7.2 Representação Interna

A forma física utilizada para representar um Domain pertence exclusivamente à implementação do compilador.

Uma implementação poderá utilizar:

* identificadores internos;
* handles;
* índices;
* estruturas próprias;
* outras representações equivalentes.

Independentemente da estratégia adotada, todas deverão preservar exatamente a mesma semântica definida pelo Modelo de Memória.

---

## 7.3 Associação entre Objetos e Domains

Toda instância representada na IR permanece associada exatamente a um Domain.

Essa associação é preservada durante todas as etapas de otimização.

Nenhuma transformação poderá alterar implicitamente o Domain ao qual um objeto pertence.

---

## 7.4 Lifetime dos Domains

A IR preserva explicitamente o lifetime de cada Domain.

As etapas posteriores da compilação poderão reorganizar instruções e otimizar o programa, mas nunca poderão modificar:

* o instante lógico de criação;
* o instante lógico de destruição;
* as regras de validade das referências.

Essas propriedades fazem parte da semântica observável da linguagem.

---

## 7.5 Operações sobre Domains

A IR representa explicitamente operações relacionadas aos Domains, incluindo:

* criação;
* destruição;
* obtenção de referências válidas;
* validação de acesso;
* verificação de lifetime.

A forma como essas operações serão traduzidas para código executável pertence ao Backend.

---

# 8. Representação do Fluxo de Controle

A IR representa explicitamente o fluxo de execução do programa.

Essa representação elimina construções puramente sintáticas, preservando apenas o comportamento operacional definido pela Semântica Operacional.

---

## 8.1 Fluxo Explícito

Todo desvio de execução passa a ser representado explicitamente.

Isso inclui:

* chamadas de função;
* decisões condicionais;
* laços;
* retornos;
* tratamento de erros;
* pattern matching.

Não existem desvios implícitos na IR.

---

## 8.2 Blocos Básicos

O fluxo de controle é organizado através de Blocos Básicos.

Cada bloco:

* inicia em um único ponto;
* termina em uma instrução de controle;
* não contém desvios internos.

Essa organização facilita análises de dependência e otimizações.

---

## 8.3 Terminadores

Todo bloco básico termina obrigatoriamente com uma instrução terminadora.

Exemplos:

* retorno;
* salto incondicional;
* salto condicional;
* término da função.

Essa propriedade garante que o fluxo de execução seja completamente explícito.

---

## 8.4 Estruturas de Alto Nível

Construções sintáticas como:

* `if`;
* `when`;
* `while`;
* `for`;
* `match`;

não são preservadas como estruturas sintáticas.

Na IR elas são representadas por grafos equivalentes de fluxo de controle.

---

## 8.5 Fluxo Determinístico

Independentemente das otimizações realizadas, o fluxo de controle representado pela IR deverá preservar exatamente o comportamento definido pela Semântica Operacional da linguagem.

---

# 9. Representação de Expressões

Expressões representam operações que produzem valores durante a execução do programa.

Na IR, todas as expressões são convertidas para operações elementares claramente identificáveis pelo compilador.

---

## 9.1 Eliminação da Sintaxe

Expressões deixam de existir como construções textuais.

Elementos como:

* precedência;
* associatividade;
* parênteses;
* formatação;

são resolvidos durante o parsing e deixam de existir na IR.

---

## 9.2 Operações Elementares

Cada expressão é decomposta em operações elementares.

Exemplos incluem:

* soma;
* subtração;
* multiplicação;
* divisão;
* comparação;
* negação;
* conversões;
* chamadas de método;
* acesso a campos.

Essa decomposição facilita otimizações posteriores.

---

## 9.3 Chamadas

Toda chamada é representada explicitamente.

A IR distingue, entre outras:

* chamadas estáticas;
* chamadas virtuais;
* chamadas através de interfaces;
* chamadas através de traits;
* chamadas geradas automaticamente pelo compilador.

Cada modalidade preserva sua semântica específica.

---

## 9.4 Avaliação

A ordem de avaliação das expressões é preservada pela IR conforme estabelecido pela Semântica Operacional.

Otimizações poderão reorganizar instruções apenas quando essa reorganização não alterar o comportamento observável do programa.

---

## 9.5 Representação Canônica

Expressões semanticamente equivalentes deverão produzir representações equivalentes na IR, independentemente da forma sintática utilizada no código-fonte.

Essa propriedade reduz ambiguidades, simplifica otimizações e favorece a consistência entre diferentes implementações do compilador.

---

# 10. Representação de Métodos

Na IR, métodos deixam de ser representações sintáticas associadas a classes e passam a constituir unidades semânticas de execução.

Cada método é representado por uma função intermediária cuja estrutura preserva integralmente as regras definidas pela Semântica Operacional.

---

## 10.1 Representação Uniforme

Independentemente de sua origem, todos os métodos são representados de forma uniforme.

Isso inclui:

* métodos de instância;
* métodos estáticos;
* construtores;
* destrutores;
* implementações de interfaces;
* implementações de traits;
* métodos gerados automaticamente pelo compilador.

Essa uniformidade simplifica as etapas posteriores de otimização e geração de código.

---

## 10.2 Assinaturas

Cada método preserva sua assinatura semântica, incluindo:

* nome;
* parâmetros;
* tipo de retorno;
* genericidade;
* restrições de tipos;
* requisitos relacionados à Capacidade de Mutação.

A representação física desses elementos pertence exclusivamente à implementação.

---

## 10.3 Despacho de Chamadas

A IR distingue explicitamente diferentes mecanismos de despacho.

Entre eles:

* chamadas diretas;
* despacho virtual;
* chamadas através de interfaces;
* chamadas através de traits;
* chamadas estáticas.

Essa distinção permite que otimizações especializadas sejam realizadas sem perda da semântica original.

---

### 10.3.1 Interfaces e Traits

A IR preserva explicitamente a distinção entre métodos declarados em interfaces, implementações fornecidas por traits e implementações concretas das classes.

Durante a geração da IR, cada chamada permanece associada ao mecanismo de despacho correspondente, permitindo que otimizações posteriores eliminem indireções quando sua correção puder ser comprovada estaticamente.

---

## 10.4 Especialização

Durante a geração da IR, o compilador poderá produzir versões especializadas de métodos quando permitido pela Semântica Operacional.

Exemplos incluem:

* especialização de generics;
* especialização de classes `sealed`;
* eliminação de chamadas indiretas comprovadamente desnecessárias.

Essas transformações não alteram o comportamento observável do programa.

---

## 10.5 Métodos Sintéticos

O compilador poderá introduzir métodos inexistentes no código-fonte para facilitar otimizações ou simplificar a geração de código.

Esses métodos permanecem invisíveis ao programador e não fazem parte da linguagem.

Sua existência pertence exclusivamente à implementação.

---

# 11. Representação do Modelo de Memória

A IR preserva explicitamente o Modelo de Memória definido pelo Documento 03.

Ao contrário de representações intermediárias tradicionais, a memória continua sendo descrita em termos dos conceitos próprios da Capi, e não apenas por estruturas físicas da arquitetura de destino.

---

## 11.1 Preservação do Modelo

Durante toda a existência da IR permanecem preservados:

* Domains;
* ObjectId;
* lifetime;
* identidade dos objetos;
* separação entre identidade e estado;
* regras de acesso ao armazenamento.

Esses conceitos somente deixam de existir na etapa final da geração de código.

---

## 11.2 Representação Lógica

A IR representa a memória de forma lógica.

Ela não contém:

* endereços físicos;
* offsets definitivos;
* registradores;
* posições reais na memória.

Esses elementos são introduzidos apenas pelo Backend.

---

## 11.3 Layout dos Objetos

Embora o Modelo de Memória defina que cada classe possui um layout lógico, a IR não determina sua representação física.

A IR preserva apenas as informações necessárias para que o Backend possa gerar corretamente:

* organização dos campos;
* alinhamento;
* representação dos tipos;
* integração com a ABI da plataforma.

---

## 11.4 Identidade e Estado

A IR preserva explicitamente a distinção entre:

* identidade (`ObjectId`);
* estado armazenado no Domain.

Essa separação constitui uma das garantias fundamentais da linguagem e não poderá ser eliminada por otimizações.

---

## 11.5 Lifetime

A validade temporal dos objetos permanece representada durante toda a existência da IR.

Otimizações poderão reorganizar instruções, mas nunca poderão modificar:

* criação lógica;
* destruição lógica;
* escopo de validade;
* relação entre objetos e seus Domains.

---

# 12. Operações sobre Domains

A IR representa explicitamente todas as operações semânticas relacionadas aos Domains.

Essas operações permanecem independentes da estratégia física utilizada para implementá-las.

---

## 12.1 Criação

A criação de um Domain é representada como uma operação semântica explícita.

A forma física como essa criação será implementada pertence ao Backend e ao Runtime Mínimo.

---

## 12.2 Alocação de Objetos

A criação de um objeto sempre ocorre associada a um Domain previamente válido.

A IR preserva essa relação de forma explícita.

Nenhum objeto pode existir fora de um Domain.

---

## 12.3 Leitura

Operações de leitura são representadas explicitamente.

A IR preserva as regras estabelecidas pelo Sistema de Tipos, pela Semântica Operacional e pelo Modelo de Memória.

Leituras nunca modificam o estado do Domain.

---

## 12.4 Escrita

Operações de escrita permanecem explicitamente associadas à Capacidade de Mutação.

A IR representa essa dependência de forma semântica, independentemente do mecanismo utilizado pela implementação para verificá-la.

Nenhuma transformação poderá produzir uma operação de escrita sem respeitar essa garantia.

---

## 12.5 Destruição

A destruição de um Domain representa o encerramento de seu lifetime.

Na IR essa operação permanece explícita, preservando todas as garantias relacionadas à:

* destruição coletiva dos objetos;
* invalidação dos `ObjectId`;
* encerramento das referências válidas;
* execução das operações de finalização previstas pela Semântica Operacional.

A tradução dessa operação para código executável pertence exclusivamente ao Backend e ao Runtime Mínimo.

---

# 13. Operações sobre ObjectId

A IR preserva explicitamente o conceito de **ObjectId**, conforme definido pelo Sistema de Tipos e pelo Modelo de Memória.

Durante toda a existência da IR, a identidade dos objetos permanece independente de sua representação física em memória.

Nenhuma transformação poderá substituir um `ObjectId` por um endereço físico, ponteiro ou outra estrutura específica da plataforma.

---

## 13.1 Preservação da Identidade

Todo objeto representado na IR possui exatamente um `ObjectId`.

Esse identificador permanece associado ao objeto durante todo o seu lifetime.

A identidade do objeto não poderá ser modificada por otimizações ou transformações realizadas pelo compilador.

---

## 13.2 Comparação de Identidade

A IR distingue explicitamente comparações por identidade das comparações por valor.

Comparações de identidade operam exclusivamente sobre `ObjectId`.

Comparações de valor operam sobre o estado armazenado no Domain.

Essa distinção deverá permanecer preservada até a geração de código.

---

## 13.3 Resolução de Referências

Toda referência a um objeto é representada através de seu `ObjectId`.

Quando necessário acessar o estado do objeto, a IR representa explicitamente a operação de resolução da identidade para o respectivo Domain.

Essa operação continua sendo um conceito semântico da linguagem, independentemente da estratégia física adotada pelo Backend.

---

## 13.4 Conversões

Conversões válidas entre tipos preservam integralmente o `ObjectId`.

Operações como:

* upcast;
* chamadas através de interfaces;
* chamadas através de traits;

não alteram a identidade do objeto.

Caso um downcast seja permitido pela Semântica Operacional, ele também deverá preservar exatamente o mesmo `ObjectId`.

---

## 13.5 Invalidação

Quando o lifetime do Domain termina, todos os `ObjectId` associados tornam-se logicamente inválidos.

A IR preserva essa relação durante toda a compilação.

Nenhuma otimização poderá prolongar ou reduzir artificialmente a validade de uma identidade.

---

# 14. Otimizações Permitidas

A IR foi projetada para permitir otimizações agressivas, desde que preservem integralmente a semântica definida pela linguagem.

As otimizações atuam exclusivamente sobre a representação intermediária e nunca sobre o significado do programa.

---

## 14.1 Objetivos

As otimizações possuem como objetivos:

* reduzir o número de instruções;
* eliminar redundâncias;
* melhorar desempenho;
* reduzir consumo de memória;
* facilitar a geração de código;
* produzir executáveis mais eficientes.

Esses objetivos nunca prevalecem sobre a preservação da semântica.

---

## 14.2 Simplificações

São permitidas simplificações como:

* propagação de constantes;
* eliminação de código morto;
* simplificação de expressões;
* propagação de cópias;
* reorganização de blocos básicos.

Essas transformações deverão produzir programas semanticamente equivalentes.

---

## 14.3 Especializações

O compilador poderá especializar partes da IR quando essa transformação puder ser comprovada como semanticamente equivalente.

Exemplos:

* especialização de generics;
* especialização de chamadas virtuais;
* otimizações envolvendo classes `sealed`;
* eliminação de verificações redundantes.

Também é permitido eliminar verificações cuja necessidade tenha sido completamente descartada por análise estática, desde que essa eliminação preserve integralmente todas as garantias estabelecidas pela linguagem.

---

## 14.4 Reorganização

Instruções poderão ser reorganizadas para melhorar desempenho.

Entretanto, essa reorganização não poderá alterar:

* ordem observável de efeitos;
* lifetime dos objetos;
* regras da Capacidade de Mutação;
* dependências entre Domains;
* comportamento definido pela Semântica Operacional.

---

## 14.5 Independência da Implementação

A especificação não determina quais otimizações devem existir.

Cada implementação poderá adotar estratégias próprias, desde que todas preservem integralmente as garantias estabelecidas pela linguagem.

---

# 15. Otimizações Proibidas

Embora a implementação possua ampla liberdade para otimizar programas, determinadas transformações são incompatíveis com a semântica da Capi e são expressamente proibidas.

---

## 15.1 Alteração de Identidade

Nenhuma otimização poderá:

* alterar um `ObjectId`;
* substituir uma identidade por outra;
* fundir identidades distintas;
* reutilizar identidades logicamente inválidas.

A identidade dos objetos constitui uma garantia permanente da linguagem.

---

## 15.2 Alteração de Domains

Nenhuma otimização poderá:

* mover objetos entre Domains;
* fundir Domains;
* dividir Domains;
* alterar implicitamente a associação entre objeto e Domain.

Qualquer reorganização física deverá preservar exatamente a organização lógica definida pela linguagem.

---

## 15.3 Alteração do Lifetime

O compilador não poderá modificar:

* instante lógico de criação;
* instante lógico de destruição;
* validade das referências;
* escopo de existência dos objetos.

Essas propriedades fazem parte da Semântica Operacional.

---

## 15.4 Violação da Capacidade de Mutação

Nenhuma transformação poderá:

* criar operações de escrita não autorizadas;
* eliminar verificações sem comprovação estática;
* permitir múltiplas capacidades incompatíveis para um mesmo Domain.

As garantias relacionadas à Capacidade de Mutação permanecem válidas durante toda a compilação.

---

## 15.5 Alteração da Semântica

É expressamente proibida qualquer otimização que altere:

* o comportamento observável do programa;
* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* qualquer garantia estabelecida pela especificação da linguagem.

A corretude do programa possui prioridade absoluta sobre qualquer ganho de desempenho.

---

# 16. Relação com o Backend

A Intermediate Representation (IR) constitui a interface formal entre o **Middle-end** e o **Backend** do compilador.

Após a conclusão das etapas de análise e otimização, a IR torna-se a única entrada utilizada pelo Backend para geração do código executável.

A tradução da IR para a arquitetura de destino deve preservar integralmente todas as garantias estabelecidas pela especificação da linguagem.

---

## 16.1 Interface de Transição

A passagem da IR para o Backend representa a transição entre uma representação semântica da linguagem e uma representação específica da plataforma.

Até esse momento, conceitos como:

* Domains;
* ObjectId;
* Classes;
* Interfaces;
* Traits;
* Lifetime;
* Capacidade de Mutação;

permanecem representados explicitamente na IR.

Cabe ao Backend traduzir esses conceitos para mecanismos compatíveis com a arquitetura de destino.

---

## 16.2 Preservação da Semântica

A geração de código nunca poderá alterar:

* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* as regras de identidade;
* as garantias relacionadas aos Domains.

Toda transformação realizada pelo Backend deverá produzir um programa semanticamente equivalente ao representado pela IR.

---

## 16.3 Adaptação à Plataforma

O Backend é responsável por adaptar a IR às características da plataforma de destino.

Entre elas:

* conjunto de instruções;
* registradores;
* convenção de chamadas;
* ABI;
* alinhamento de memória;
* formato de executáveis;
* modelo de ligação.

Esses aspectos pertencem exclusivamente à implementação.

---

## 16.4 Materialização dos Conceitos da Linguagem

Durante a geração de código, o Backend converte os conceitos semânticos preservados pela IR em estruturas executáveis específicas da plataforma.

Essa conversão inclui, entre outros:

* representação física dos Domains;
* organização dos objetos em memória;
* preservação das identidades (`ObjectId`);
* integração com o Runtime Mínimo;
* geração das estruturas necessárias ao despacho de métodos.

A forma como essa materialização ocorre pertence exclusivamente à implementação, desde que todas as garantias da linguagem sejam preservadas.

---

## 16.5 Independência do Backend

A especificação da Capi não exige um Backend específico.

Implementações poderão utilizar:

* LLVM;
* Cranelift;
* GCC;
* backends próprios;
* outras infraestruturas equivalentes.

Independentemente da tecnologia utilizada, o programa produzido deverá possuir comportamento semanticamente equivalente.

---

## 16.6 Metadados

A IR poderá preservar metadados auxiliares destinados a ferramentas de depuração, análise estática e diagnóstico.

Esses metadados podem incluir:

* localização no código-fonte;
* nomes simbólicos;
* informações para geração de debug;
* anotações internas do compilador.

A presença, formato e persistência desses metadados pertencem exclusivamente à implementação.

---

# 17. Garantias da Intermediate Representation

A IR constitui uma representação normativa da linguagem durante a compilação.

Todas as implementações deverão preservar as garantias estabelecidas neste capítulo.

---

## 17.1 Preservação do Sistema de Tipos

Nenhuma transformação sobre a IR poderá produzir uma representação que viole as regras estabelecidas pelo Sistema de Tipos.

Isso inclui:

* compatibilidade entre tipos;
* herança;
* interfaces;
* traits;
* generics;
* variância;
* restrições de conversão.

---

## 17.2 Preservação do Modelo de Memória

Toda representação intermediária deverá preservar:

* Domains;
* ObjectId;
* lifetime;
* organização lógica dos objetos;
* separação entre identidade e estado.

Essas propriedades permanecem válidas independentemente da plataforma de destino.

---

## 17.3 Preservação da Semântica Operacional

Todas as operações representadas pela IR deverão manter exatamente o comportamento definido pela Semântica Operacional.

Isso inclui:

* ordem observável de execução;
* criação e destruição de objetos;
* chamadas de métodos;
* tratamento de erros;
* concorrência baseada em Domains.

---

## 17.4 Determinismo

Programas semanticamente equivalentes deverão produzir representações intermediárias equivalentes.

Diferenças de implementação poderão existir internamente, mas não poderão alterar o comportamento observável do programa nem as garantias estabelecidas pela especificação.

---

## 17.5 Evolução Compatível

Novas versões da IR poderão introduzir novos elementos internos, desde que permaneçam compatíveis com as garantias definidas pela linguagem.

A evolução da IR nunca poderá modificar a semântica dos programas escritos em Capi.

---

# 18. Princípio da Separação entre Garantias e Implementação

Este documento segue o mesmo princípio arquitetural estabelecido nos documentos anteriores da especificação.

A IR define **o que** deve ser preservado durante a compilação.

A implementação define **como** essa preservação será realizada.

---

## 18.1 Garantias

Constituem garantias obrigatórias da IR:

* preservação do Sistema de Tipos;
* preservação do Modelo de Memória;
* preservação da Semântica Operacional;
* preservação dos Domains;
* preservação dos `ObjectId`;
* preservação do lifetime;
* preservação da Capacidade de Mutação.

Essas garantias fazem parte da linguagem e são independentes da implementação do compilador.

---

## 18.2 Implementação

A implementação poderá utilizar qualquer estrutura interna que preserve integralmente essas garantias.

Entre elas:

* SSA;
* grafos de fluxo;
* bytecode intermediário;
* árvores otimizadas;
* representações híbridas;
* outras técnicas equivalentes.

A escolha dessas estruturas pertence exclusivamente ao compilador.

---

## 18.3 Evolução

A organização interna da IR poderá evoluir continuamente.

Novos tipos de instruções, novos metadados, novas estratégias de representação ou novos elementos necessários para suportar recursos futuros da linguagem poderão ser incorporados sem modificar a semântica dos programas nem as garantias estabelecidas pela especificação.

Essa evolução permite que a tecnologia empregada pelo compilador acompanhe o crescimento da linguagem sem comprometer sua estabilidade. A forma como diferentes implementações representam ou versionam a IR pertence exclusivamente à implementação.

---

# 19. Não Objetivos

Este documento define exclusivamente os princípios, responsabilidades e garantias da **Intermediate Representation (IR)** da Capi.

Diversos aspectos relacionados à implementação do compilador e às tecnologias utilizadas permanecem deliberadamente fora de seu escopo.

---

## 19.1 Formato Físico

A especificação não define:

* formato binário da IR;
* serialização da IR;
* persistência em disco;
* representação textual;
* protocolos de intercâmbio entre ferramentas.

Cada implementação poderá utilizar o formato mais adequado aos seus objetivos.

---

## 19.2 Estruturas Internas

Este documento não especifica:

* classes internas do compilador;
* estruturas de dados;
* algoritmos utilizados para manipulação da IR;
* gerenciamento interno de memória;
* organização dos módulos do compilador.

Esses aspectos pertencem exclusivamente à implementação.

---

## 19.3 Estratégias de Otimização

A especificação não determina:

* quais otimizações devem existir;
* ordem de execução das otimizações;
* quantidade de fases;
* algoritmos empregados;
* heurísticas de otimização.

A única exigência é a preservação integral da semântica da linguagem.

---

## 19.4 Backend

A IR não define:

* geração de código de máquina;
* instruções específicas de processadores;
* convenções de chamada;
* organização de registradores;
* formatos de executáveis.

Esses aspectos pertencem ao Backend e à ABI da plataforma de destino.

---

## 19.5 Runtime

A IR também não especifica:

* gerenciamento físico dos Domains;
* implementação da Capacidade de Mutação;
* estruturas internas do Runtime Mínimo;
* gerenciamento de recursos da plataforma.

Esses aspectos serão definidos no **Documento 09 — Runtime Mínimo**.

---

# 20. Considerações Finais

A **Intermediate Representation (IR)** constitui o núcleo semântico do compilador da Capi.

Ela estabelece uma representação única, consistente e independente da sintaxe da linguagem, servindo como base para todas as etapas posteriores da compilação.

Diferentemente de representações intermediárias tradicionalmente orientadas apenas à geração de código, a IR da Capi preserva explicitamente os principais conceitos da linguagem, incluindo:

* Domains;
* ObjectId;
* Sistema de Tipos;
* Modelo de Memória;
* Lifetime;
* Capacidade de Mutação;
* Semântica Operacional.

Essa característica permite que otimizações sejam realizadas em um nível de abstração mais elevado, preservando integralmente as garantias definidas pela especificação e reduzindo a necessidade de reconstrução de informações semânticas durante a geração de código.

Ao separar claramente a representação semântica da representação física da plataforma, a IR reforça um dos princípios centrais da arquitetura da Capi: a distinção entre **garantias da linguagem** e **mecanismos de implementação**. Diferentes compiladores poderão adotar estruturas internas, formatos de IR e estratégias de otimização distintos, desde que produzam programas semanticamente equivalentes.

Com a conclusão deste documento, encontra-se definida a representação intermediária oficial da linguagem. Os documentos subsequentes detalharão os componentes que utilizam ou complementam essa representação, iniciando pela **Biblioteca Padrão**, seguida pelo **Runtime Mínimo**, pela **ABI e Interoperabilidade (FFI)**, pela **Toolchain** e pelo **Ecossistema de Pacotes**.

A IR representa, portanto, o ponto de convergência entre a linguagem definida pelos documentos anteriores e a implementação prática do compilador, fornecendo uma base sólida para otimizações, geração de código e evolução futura da Capi sem comprometer a estabilidade de sua especificação.
