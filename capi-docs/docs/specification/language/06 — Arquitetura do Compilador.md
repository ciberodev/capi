# 06 — Arquitetura do Compilador

## Especificação da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento descreve a arquitetura lógica do compilador da linguagem Capi.

Seu objetivo é estabelecer as responsabilidades de cada etapa do processo de compilação, desde a leitura do código-fonte até a geração do código executável, preservando todas as garantias definidas pelos documentos anteriores da especificação.

Este documento não define uma implementação específica do compilador, mas sim os princípios arquiteturais que qualquer implementação deverá respeitar.

---

## 1.2 Escopo

Este documento especifica:

* a arquitetura geral do compilador;
* as fases do processo de compilação;
* as responsabilidades de cada fase;
* os artefatos produzidos durante a compilação;
* as garantias que devem ser preservadas entre cada etapa.

Não fazem parte deste documento:

* a definição da IR (Documento 07);
* a biblioteca padrão (Documento 08);
* o Runtime Mínimo (Documento 09);
* detalhes da ABI e FFI (Documento 10);
* a Toolchain (Documento 11).

---

## 1.3 Relação com os Documentos Anteriores

O compilador é responsável por transformar programas escritos na sintaxe da Capi em código executável, preservando integralmente as garantias estabelecidas pelos documentos anteriores.

Em especial, deve respeitar:

* os princípios fundamentais (Documento 01);
* o Sistema de Tipos (Documento 02);
* o Modelo de Memória (Documento 03);
* a Sintaxe (Documento 04);
* a Semântica Operacional (Documento 05).

Nenhuma etapa da compilação pode alterar ou enfraquecer essas garantias.

---

## 1.4 Princípio da Separação entre Garantias e Implementação

Assim como estabelecido nos Documentos 01, 02, 03, 04 e 05, este documento distingue explicitamente entre:

* as garantias oferecidas pela linguagem;
* os mecanismos utilizados pelo compilador para implementá-las.

As garantias são normativas e fazem parte da especificação da linguagem.

A arquitetura apresentada neste documento descreve uma organização recomendada para o compilador, permitindo que implementações adotem estratégias diferentes, desde que preservem integralmente as garantias observáveis da linguagem.

---

# 2. Arquitetura Geral

## 2.1 Visão Geral

O compilador da Capi é organizado como uma sequência de fases independentes.

Cada fase recebe um conjunto de artefatos produzidos pela etapa anterior, realiza uma transformação bem definida e produz novos artefatos para a etapa seguinte.

Essa organização favorece:

* modularidade;
* manutenção;
* otimização;
* testes independentes;
* evolução futura do compilador.

---

## 2.2 Pipeline de Compilação

Conceitualmente, o processo completo de compilação pode ser representado da seguinte forma:

```text
Código Fonte
      │
      ▼
Análise Léxica
      │
      ▼
Parsing
      │
      ▼
Árvore Sintática (AST)
      │
      ▼
Resolução de Símbolos
      │
      ▼
Resolução de Tipos
      │
      ▼
Análise Semântica
      │
      ▼
Representação Intermediária (IR)
      │
      ▼
Otimizações
      │
      ▼
Backend
      │
      ▼
Executável
```

Cada etapa possui responsabilidades específicas e não deve assumir funções pertencentes às demais.

---

## 2.3 Organização em Camadas

O compilador é dividido conceitualmente em quatro grandes camadas:

### Frontend

Responsável por compreender o código escrito pelo programador.

Inclui:

* análise léxica;
* parsing;
* construção da AST;
* resolução de símbolos;
* resolução de tipos;
* análise semântica.

---

### Middle-end

Responsável pelas transformações independentes da arquitetura de destino.

Inclui:

* geração da IR;
* validações adicionais;
* otimizações.

---

### Backend

Responsável pela geração do código executável.

Inclui:

* seleção de instruções;
* geração de código;
* organização do layout final;
* integração com a plataforma de destino.

---

### Infraestrutura

Responsável pelos serviços auxiliares utilizados durante toda a compilação.

Inclui, entre outros:

* gerenciamento de módulos;
* cache de compilação;
* sistema de diagnósticos;
* gerenciamento de dependências;
* infraestrutura para compilação incremental.

---

## 2.4 Entradas

O compilador recebe como entrada:

* arquivos-fonte da linguagem Capi;
* módulos importados;
* configurações do projeto;
* opções de compilação;
* biblioteca padrão;
* metadados produzidos por compilações anteriores, quando disponíveis.

A organização física desses artefatos pertence à Toolchain e não faz parte deste documento.

---

## 2.5 Saídas

Ao término da compilação, o compilador poderá produzir:

* executáveis;
* bibliotecas;
* representação intermediária;
* metadados;
* símbolos de depuração;
* diagnósticos.

A forma exata desses artefatos depende do backend utilizado e da plataforma de destino.

---

# 3. Fluxo de Compilação

## 3.1 Modelo Geral

A compilação é composta por uma sequência ordenada de transformações.

Cada fase possui responsabilidades claramente delimitadas e produz um artefato bem definido.

Sempre que possível, fases posteriores devem operar exclusivamente sobre os artefatos produzidos pela etapa imediatamente anterior.

Esse princípio reduz acoplamento e facilita a evolução independente de cada componente do compilador.

---

## 3.2 Artefatos Intermediários

Ao longo da compilação são produzidos diversos artefatos intermediários, tais como:

* sequência de tokens;
* árvore sintática abstrata (AST);
* tabela de símbolos;
* informações de tipos;
* representação intermediária (IR).

Esses artefatos pertencem exclusivamente ao compilador e não fazem parte da semântica observável da linguagem.

Sua estrutura poderá evoluir entre versões do compilador sem afetar a compatibilidade dos programas.

---

## 3.3 Preservação das Garantias

Todas as fases do compilador devem preservar integralmente:

* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* a integridade dos Domains;
* a integridade dos `ObjectId`;
* a Capacidade de Mutação;
* todas as garantias estabelecidas pela especificação da linguagem.

Nenhuma otimização, transformação ou simplificação realizada durante a compilação pode modificar o comportamento observável de um programa válido.

---

## 3.4 Independência entre Fases

Cada fase do compilador deve depender apenas dos artefatos formais produzidos pelas fases anteriores.

Nenhuma fase deve acessar diretamente estruturas internas pertencentes a outra etapa quando existir um artefato intermediário equivalente.

Esse princípio favorece:

* modularidade;
* substituição de implementações;
* testes isolados;
* compilação incremental;
* evolução futura da arquitetura do compilador.

---

## 3.5 Determinismo

O compilador da Capi deve ser determinístico.

Dadas:

* a mesma versão do compilador;
* os mesmos arquivos-fonte;
* as mesmas dependências;
* as mesmas opções de compilação;

o resultado produzido deverá ser semanticamente idêntico.

Diferenças de implementação, plataforma ou estratégias internas de otimização não podem alterar o comportamento observável do programa gerado.

---

# 4. Análise Léxica

A Análise Léxica é a primeira etapa efetiva do processo de compilação.

Sua responsabilidade consiste em transformar o fluxo de caracteres do código-fonte em uma sequência estruturada de **tokens**, preservando todas as informações necessárias para as etapas posteriores.

A Análise Léxica não interpreta o significado do programa; ela apenas identifica seus elementos sintáticos básicos.

---

## 4.1 Objetivos

A Análise Léxica possui os seguintes objetivos:

* reconhecer os elementos léxicos da linguagem;
* identificar palavras reservadas;
* identificar identificadores;
* reconhecer literais;
* remover comentários;
* registrar a posição de cada token para fins de diagnóstico.

Nenhuma validação semântica ocorre nesta fase.

---

## 4.2 Tokens

Cada token representa uma unidade léxica indivisível.

Exemplos de categorias de tokens:

* palavras reservadas;
* identificadores;
* operadores;
* separadores;
* literais numéricos;
* literais textuais;
* literais booleanos;
* pontuação.

Cada token deverá preservar, no mínimo:

* categoria;
* lexema;
* posição no arquivo;
* linha;
* coluna.

A representação interna dos tokens pertence à implementação.

---

## 4.3 Identificadores

Identificadores representam nomes definidos pelo programador.

Devem obedecer às regras sintáticas estabelecidas pela linguagem.

A Análise Léxica não verifica:

* escopo;
* existência;
* duplicidade;
* compatibilidade de tipos.

Essas verificações pertencem às fases posteriores.

---

## 4.4 Palavras Reservadas

Palavras reservadas possuem significado fixo na linguagem.

Não podem ser utilizadas como identificadores.

A lista oficial de palavras reservadas é definida pelo Documento 04 — Sintaxe da Linguagem.

---

## 4.5 Literais

A Análise Léxica identifica os diversos tipos de literais definidos pela linguagem, incluindo:

* inteiros;
* números de ponto flutuante;
* caracteres;
* cadeias de caracteres;
* booleanos.

A interpretação semântica desses valores ocorre posteriormente.

---

## 4.6 Comentários

Comentários fazem parte exclusivamente do código-fonte.

Não produzem efeitos sobre o programa compilado.

Após sua identificação, podem ser descartados pelo compilador.

Ferramentas auxiliares, como geradores de documentação, podem optar por preservá-los.

---

## 4.7 Unicode

A linguagem Capi adota Unicode como conjunto oficial de caracteres.

O compilador deverá interpretar corretamente arquivos codificados em UTF-8.

A utilização de outros formatos pertence à implementação.

---

## 4.8 Diagnósticos Léxicos

Erros identificados durante a Análise Léxica interrompem a geração normal de tokens.

Sempre que possível, o compilador deverá produzir diagnósticos contendo:

* localização exata;
* descrição do erro;
* contexto suficiente para facilitar sua correção.

A estratégia de recuperação de erros pertence à implementação.

---

# 5. Parsing

O Parsing transforma a sequência de tokens produzida pela Análise Léxica em uma representação estrutural do programa.

Essa representação corresponde à Árvore Sintática Abstrata (AST).

---

## 5.1 Objetivos

O Parsing possui como responsabilidades:

* validar a gramática da linguagem;
* construir a AST;
* identificar erros sintáticos;
* produzir informações estruturadas para as fases posteriores.

O Parsing não realiza verificações semânticas.

---

## 5.2 Gramática

A gramática oficial da linguagem é definida pelo Documento 04 — Sintaxe da Linguagem.

A técnica utilizada para implementar o parser não faz parte desta especificação.

Podem ser utilizados, entre outros:

* Recursive Descent;
* LL;
* LR;
* LALR;
* PEG;
* técnicas equivalentes.

Todas devem produzir a mesma interpretação sintática do programa.

---

## 5.3 Construção da AST

Durante o Parsing, o compilador constrói a Árvore Sintática Abstrata.

A AST representa apenas a estrutura lógica do programa.

Detalhes puramente sintáticos, como espaços em branco e comentários, não precisam ser preservados.

---

## 5.4 Recuperação de Erros

Sempre que possível, o compilador deverá recuperar-se de erros sintáticos para continuar a análise do restante do programa.

O objetivo dessa recuperação é permitir que múltiplos diagnósticos sejam apresentados em uma única compilação, reduzindo o número de ciclos de correção necessários durante o desenvolvimento.

A estratégia utilizada para essa recuperação — como sincronização por tokens, produção de nós sintáticos incompletos ou outras técnicas equivalentes — pertence exclusivamente à implementação.

A recuperação de erros não deverá comprometer a estrutura da AST correspondente às partes válidas do programa.

---

## 5.5 Diagnósticos

Os diagnósticos produzidos durante o Parsing deverão indicar, sempre que possível:

* localização precisa;
* símbolo inesperado;
* símbolo esperado;
* contexto sintático.

A apresentação das mensagens pertence à implementação.

---

# 6. Árvore Sintática Abstrata (AST)

A AST representa a estrutura sintática do programa de forma independente da gramática concreta utilizada pelo parser.

Ela constitui o principal artefato produzido pelo Frontend antes das etapas de análise semântica.

---

## 6.1 Objetivos

A AST deve representar apenas a estrutura lógica do programa.

Ela não representa:

* layout do código;
* espaços em branco;
* comentários;
* estilo de formatação.

Sua finalidade é permitir que as fases posteriores analisem o programa de maneira estruturada.

---

## 6.2 Nós da AST

A AST é composta por diferentes categorias de nós, incluindo:

* módulos;
* classes;
* interfaces;
* traits;
* métodos;
* declarações;
* expressões;
* comandos;
* literais;
* operadores.

A representação exata desses nós pertence à implementação.

---

## 6.3 Estrutura Hierárquica

Cada nó da AST representa uma construção sintática da linguagem.

As relações entre nós refletem a organização estrutural do programa.

Exemplos:

* uma classe contém métodos;
* um método contém comandos;
* um comando pode conter expressões;
* expressões podem conter outras expressões.

---

## 6.4 Imutabilidade

Após sua construção, recomenda-se que a AST seja tratada como uma estrutura imutável.

Transformações posteriores poderão produzir novas representações intermediárias, sem modificar diretamente a árvore original.

Essa recomendação favorece:

* previsibilidade;
* paralelização;
* análise incremental;
* reutilização entre fases.

Outras estratégias podem ser adotadas pela implementação, desde que preservem a semântica do programa.

---

## 6.5 Independência da Implementação

A estrutura física utilizada para representar a AST não faz parte desta especificação.

Cada implementação poderá utilizar:

* árvores tradicionais;
* grafos acíclicos;
* estruturas compartilhadas;
* representações compactadas;
* outras técnicas equivalentes.

Independentemente da representação escolhida, todas deverão preservar exatamente a mesma estrutura lógica do programa.

---

## 6.6 Papel na Arquitetura do Compilador

A AST representa a fronteira entre a análise sintática e a análise semântica.

As fases seguintes passam a operar sobre a estrutura do programa, e não mais sobre seu texto original.

A partir da AST serão realizadas:

* resolução de símbolos;
* resolução de tipos;
* validações semânticas;
* geração da Representação Intermediária (IR).

A AST constitui, portanto, o último artefato puramente sintático produzido pelo compilador.

---

# 7. Resolução de Símbolos

A Resolução de Símbolos é responsável por associar cada identificador presente no programa à declaração que o define.

Essa etapa estabelece a estrutura lógica dos escopos e constitui a base para a Resolução de Tipos e para a Análise Semântica.

---

## 7.1 Objetivos

A Resolução de Símbolos possui os seguintes objetivos:

* localizar declarações;
* validar escopos;
* identificar ambiguidades;
* construir a tabela de símbolos;
* associar referências às respectivas declarações.

Nenhuma verificação de compatibilidade de tipos ocorre nesta fase.

---

## 7.2 Tabela de Símbolos

O compilador deverá construir uma representação dos símbolos declarados pelo programa.

Entre eles:

* módulos;
* classes;
* interfaces;
* traits;
* métodos;
* funções;
* parâmetros;
* variáveis locais;
* constantes;
* aliases.

A organização física dessa tabela pertence à implementação.

---

## 7.3 Escopos

A linguagem define escopos hierárquicos.

Exemplos de escopos:

* módulo;
* classe;
* método;
* bloco;
* expressão.

A resolução deve sempre localizar a declaração visível mais próxima do ponto de uso.

---

## 7.4 Importações

Durante esta fase, o compilador resolve todas as instruções de importação.

Devem ser verificadas:

* existência do módulo importado;
* acessibilidade;
* ambiguidades;
* conflitos de nomes.

A forma física como módulos são localizados pertence à Toolchain.

---

## 7.5 Diagnósticos

Entre os erros identificados nesta fase incluem-se:

* símbolo inexistente;
* símbolo inacessível;
* declaração duplicada;
* ambiguidades de resolução;
* importações inválidas.

Os diagnósticos devem indicar claramente a origem do problema e, sempre que possível, sugerir alternativas.

---

# 8. Resolução de Tipos

Após a resolução dos símbolos, o compilador determina os tipos associados a todas as construções do programa.

Essa etapa aplica as regras definidas pelo Documento 02 — Sistema de Tipos.

---

## 8.1 Objetivos

A Resolução de Tipos é responsável por:

* determinar o tipo de todas as expressões;
* verificar compatibilidade de atribuições;
* validar chamadas de métodos;
* resolver generics;
* validar herança;
* validar interfaces e traits;
* aplicar regras de inferência.

---

## 8.2 Inferência de Tipos

Sempre que permitido pela linguagem, o compilador poderá inferir automaticamente o tipo de uma expressão.

A inferência não altera o Sistema de Tipos.

Ela apenas evita que determinadas informações precisem ser escritas explicitamente pelo programador.

O comportamento observável do programa deve ser idêntico ao de uma declaração totalmente explícita.

---

## 8.3 Resolução de Generics

Durante esta fase são resolvidos todos os parâmetros genéricos.

Devem ser verificadas:

* quantidade de parâmetros;
* restrições (*constraints*);
* compatibilidade entre tipos;
* especializações.

O mecanismo interno utilizado para representar tipos genéricos pertence à implementação.

---

## 8.4 Interfaces e Traits

A Resolução de Tipos verifica:

* implementação obrigatória de interfaces;
* composição de traits;
* compatibilidade entre assinaturas;
* conflitos de implementação.

As regras formais para interfaces e traits são definidas pelo Documento 02.

---

## 8.5 Conversões

Nesta fase o compilador identifica todas as conversões de tipos.

Entre elas:

* conversões implícitas;
* conversões explícitas;
* upcasts;
* downcasts.

A validade dessas conversões será posteriormente confirmada pela Análise Semântica.

---

## 8.6 Diagnósticos

Erros típicos desta fase incluem:

* incompatibilidade de tipos;
* generic inválido;
* método inexistente;
* assinatura incompatível;
* implementação incompleta de interface;
* conflito entre traits.

---

# 9. Hierarquia de Tipos

Após a Resolução de Tipos, o compilador possui todas as informações necessárias para construir a hierarquia completa de classes da aplicação.

Essa estrutura será utilizada durante toda a Análise Semântica e na geração da Representação Intermediária.

---

## 9.1 Construção da Hierarquia

O compilador identifica todas as relações entre:

* classes;
* superclasses;
* interfaces;
* traits.

A hierarquia construída deve representar fielmente as relações definidas pelo código-fonte.

---

## 9.2 Herança

A Capi adota herança simples de implementação.

Durante esta fase o compilador verifica:

* existência da superclasse;
* ausência de ciclos;
* compatibilidade da hierarquia;
* regras de herança definidas pelo Documento 02.

---

## 9.3 Sobrescrita de Métodos

O compilador identifica todos os métodos sobrescritos (*override*).

Devem ser verificadas:

* compatibilidade da assinatura;
* compatibilidade dos tipos de retorno;
* regras de visibilidade;
* conformidade com as regras de variância.

As regras formais de sobrescrita pertencem à Semântica Operacional.

---

## 9.4 Classes Seladas

Classes declaradas como `sealed` possuem um conjunto finito de subclasses conhecido durante a compilação.

O compilador deve registrar essa informação para permitir:

* verificações exaustivas;
* validação de determinados downcasts;
* otimizações futuras;
* análise de *pattern matching*.

A representação utilizada para armazenar essas informações pertence à implementação.

---

## 9.5 Polimorfismo

Após a construção da hierarquia, o compilador identifica todos os pontos onde ocorre despacho polimórfico.

Essas informações serão utilizadas nas fases posteriores de otimização e geração de código.

A estratégia utilizada para implementar o despacho dinâmico não faz parte desta especificação.

---

## 9.6 Consistência da Hierarquia

Ao término desta fase, a hierarquia completa do programa deverá estar validada.

Devem estar garantidos, entre outros:

* ausência de ciclos de herança;
* compatibilidade entre classes e interfaces;
* conformidade com o Sistema de Tipos;
* preservação das regras de herança da linguagem.

Somente após essa validação o compilador poderá iniciar a Análise Semântica propriamente dita.

---

# 9.7 Interfaces e Traits na Hierarquia

Interfaces e traits não participam da hierarquia de herança de implementação, mas integram o Sistema de Tipos como mecanismos de abstração e reutilização de comportamento.

Durante esta fase, o compilador verifica:

* implementação obrigatória de interfaces;
* composição correta de traits;
* ausência de conflitos entre implementações;
* conformidade com as regras estabelecidas pelo Documento 02.

As chamadas realizadas através de interfaces e traits utilizam os mesmos princípios de despacho definidos para classes, preservando integralmente o polimorfismo da linguagem.

---

# 10. Análise Semântica

A Análise Semântica verifica se um programa sintaticamente válido também é semanticamente correto.

Nesta etapa, o compilador aplica as regras definidas pela Semântica Operacional (Documento 05), pelo Sistema de Tipos (Documento 02) e pelo Modelo de Memória (Documento 03).

---

## 10.1 Objetivos

A Análise Semântica possui como responsabilidades:

* validar a coerência do programa;
* verificar regras da linguagem;
* validar o Modelo de Memória;
* validar o Sistema de Tipos;
* garantir que todas as propriedades da linguagem sejam respeitadas.

Ao término desta fase, todo programa aceito deverá obedecer integralmente às garantias estabelecidas pela especificação.

---

## 10.2 Validação de Declarações

O compilador verifica:

* inicialização obrigatória;
* uso correto de modificadores;
* consistência entre declarações;
* membros duplicados;
* ciclos ilegais de dependência.

---

## 10.3 Validação de Expressões

São verificadas, entre outras:

* compatibilidade entre operandos;
* validade das atribuições;
* chamadas de métodos;
* acesso a membros;
* resolução definitiva de sobrecargas.

---

## 10.4 Validação de Fluxo

O compilador verifica:

* caminhos obrigatórios de retorno;
* código inalcançável;
* inicialização antes do uso;
* tratamento obrigatório de resultados.

Essas verificações podem utilizar técnicas de análise de fluxo, desde que preservem a semântica da linguagem.

---

## 10.5 Diagnósticos

Todos os erros semânticos devem ser apresentados de forma precisa.

Sempre que possível, o compilador deverá fornecer:

* localização;
* descrição do erro;
* contexto;
* sugestão de correção.

---

# 11. Validação da Capacidade de Mutação

A Capacidade de Mutação constitui uma das principais garantias da Capi.

Embora seja um conceito abstrato do Sistema de Tipos, cabe ao compilador verificar que todas as regras relacionadas à escrita sobre Domains sejam respeitadas.

---

## 11.1 Objetivos

Esta fase verifica que:

* toda operação de escrita ocorre sobre um Domain válido;
* a exclusividade da Capacidade de Mutação é preservada;
* nenhuma escrita concorrente incompatível seja possível;
* todas as garantias da Semântica Operacional permaneçam válidas.

---

### 11.2 Inferência

Na maior parte dos programas, a Capacidade de Mutação é inferida automaticamente pelo compilador, sem necessidade de declarações explícitas por parte do programador.

Essa inferência baseia-se na análise dos escopos, do fluxo de controle e das operações realizadas sobre cada Domain. Durante essa análise, o compilador determina os contextos em que operações de escrita podem ocorrer e verifica se existe autorização suficiente para executá-las, conforme as regras estabelecidas pela linguagem.

A inferência constitui apenas um mecanismo de implementação do compilador e não altera a semântica do programa nem as garantias oferecidas pelo Sistema de Tipos e pela Semântica Operacional.

Quando essa comprovação não puder ser realizada estaticamente, o compilador deverá rejeitar o programa ou exigir o uso de mecanismos explicitamente previstos pela linguagem.

---

## 11.3 Exclusividade

Durante toda a análise, o compilador deve garantir que não existam duas Capacidades de Mutação simultaneamente válidas para o mesmo Domain no mesmo contexto de execução.

Essa propriedade é fundamental para impedir *data races* e preservar a integridade do Modelo de Memória.

---

## 11.4 Escopos

A validade de uma Capacidade de Mutação está limitada ao escopo em que foi obtida.

Ao término desse escopo, a autorização correspondente deixa de existir.

O mecanismo utilizado para representar essa informação pertence exclusivamente à implementação do compilador.

---

## 11.5 Diagnósticos

Exemplos de erros detectados nesta fase:

* escrita sem autorização;
* escrita concorrente incompatível;
* conflito entre capacidades;
* tentativa de mutação em contexto somente leitura.

---

# 12. Validação dos Domains

Após a validação da Capacidade de Mutação, o compilador verifica todas as propriedades relacionadas aos Domains.

Essa etapa garante que o Modelo de Memória definido pelo Documento 03 seja preservado.

---

## 12.1 Associação entre Objetos e Domains

Todo objeto deve estar associado exatamente a um Domain válido.

O compilador verifica que:

* nenhum objeto exista fora de um Domain;
* um objeto nunca pertença simultaneamente a dois Domains;
* toda referência permaneça consistente durante o lifetime do objeto.

---

## 12.2 Integridade dos ObjectId

O compilador verifica que cada `ObjectId` permaneça permanentemente associado ao objeto que representa.

Devem ser preservadas:

* unicidade;
* estabilidade;
* identidade compartilhada;
* coerência entre referências.

---

## 12.3 Lifetime

O compilador valida que o lifetime de todos os objetos permaneça compatível com o lifetime do Domain ao qual pertencem.

Nenhuma referência poderá sobreviver ao Domain correspondente.

Essa verificação constitui uma das principais garantias da segurança de memória da Capi.

---

## 12.4 Compartilhamento

O compartilhamento de identidades é permitido.

O compartilhamento da autoridade de escrita não é.

O compilador verifica que essas propriedades permaneçam válidas durante toda a execução semântica do programa.

---

## 12.5 Migração entre Domains

Objetos não podem ser transferidos implicitamente entre Domains.

Caso versões futuras da linguagem ofereçam mecanismos explícitos para migração ou transferência de estado, esses mecanismos deverão preservar integralmente:

* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* a integridade das identidades.

A existência e a semântica desses mecanismos serão definidas em documento específico.

---

## 12.6 Garantias

Ao término desta etapa, o compilador deverá garantir que:

* todos os objetos pertencem a um único Domain;
* todos os `ObjectId` permanecem válidos durante o lifetime correspondente;
* nenhuma violação do Modelo de Memória seja possível;
* todas as propriedades definidas pelos Documentos 02, 03 e 05 permaneçam preservadas.

Somente após essas validações o programa poderá prosseguir para a geração da Representação Intermediária (IR).

---

# 13. Representação Intermediária (IR)

Após a conclusão da Análise Semântica, o compilador transforma o programa em uma **Representação Intermediária (Intermediate Representation — IR)**.

A IR constitui a representação interna utilizada pelas etapas posteriores de otimização e geração de código.

Sua definição completa será apresentada no **Documento 07 — Representação Intermediária (IR)**.

---

## 13.1 Objetivos

A IR possui os seguintes objetivos:

* representar o programa de forma independente da sintaxe da linguagem;
* facilitar otimizações;
* servir de interface entre o Frontend e o Backend;
* preservar integralmente a semântica definida pela linguagem.

A IR não faz parte da linguagem Capi e não é observável pelo programador.

---

## 13.2 Transformação

A geração da IR ocorre somente após a conclusão bem-sucedida de todas as validações semânticas.

Nenhum programa semanticamente inválido poderá produzir uma IR válida.

A transformação deve preservar:

* Sistema de Tipos;
* Modelo de Memória;
* Semântica Operacional;
* todas as garantias estabelecidas pela especificação.

---

## 13.3 Independência

A estrutura interna da IR não faz parte desta especificação.

Implementações diferentes poderão utilizar:

* SSA;
* grafos de fluxo de controle;
* árvores intermediárias;
* bytecode interno;
* representações híbridas;
* outras estratégias equivalentes.

Independentemente da representação adotada, todas deverão preservar exatamente a mesma semântica do programa.

---

## 13.4 Fronteira Arquitetural

A geração da IR representa a transição entre o **Frontend** e o **Middle-end** do compilador.

Após essa etapa:

* o código-fonte deixa de ser utilizado;
* a AST deixa de ser modificada;
* todas as transformações passam a ocorrer exclusivamente sobre a IR.

---

# 14. Otimizações

Após a geração da IR, o compilador poderá realizar otimizações com o objetivo de melhorar desempenho, reduzir consumo de memória ou diminuir o tamanho do código gerado.

Todas as otimizações são opcionais.

Nenhuma otimização pode alterar a semântica observável do programa.

---

## 14.1 Objetivos

As otimizações possuem como objetivos:

* reduzir instruções desnecessárias;
* eliminar redundâncias;
* melhorar desempenho;
* reduzir consumo de memória;
* facilitar a geração de código.

O comportamento observável do programa deve permanecer inalterado.

---

## 14.2 Preservação da Semântica

Toda otimização deverá preservar integralmente:

* Sistema de Tipos;
* Modelo de Memória;
* Semântica Operacional;
* Garantias da Linguagem.

A corretude possui prioridade absoluta sobre desempenho.

---

## 14.3 Exemplos de Otimizações

Entre as otimizações possíveis incluem-se:

* Constant Folding;
* Constant Propagation;
* Dead Code Elimination;
* Common Subexpression Elimination;
* Inlining;
* Escape Analysis;
* Specialization;
* Loop Optimizations.

A presença ou ausência de qualquer otimização pertence exclusivamente à implementação.

---

## 14.4 Otimizações Específicas da Capi

O modelo arquitetural da Capi permite otimizações adicionais, incluindo:

* eliminação de verificações redundantes de identidade;
* especialização de chamadas sobre classes `sealed`;
* otimizações baseadas em Domains;
* simplificação de verificações relacionadas à Capacidade de Mutação;
* eliminação de verificações cuja validade tenha sido comprovada durante a Análise Semântica.

Essas otimizações não constituem parte obrigatória da linguagem.

---

## 14.5 Independência da Implementação

A estratégia de otimização adotada por cada compilador pertence exclusivamente à implementação.

A especificação não impõe:

* quantidade de otimizações;
* ordem de execução;
* algoritmos específicos;
* infraestrutura utilizada.

A única exigência é a preservação integral da semântica observável do programa.

---

# 15. Geração de Código

Após todas as otimizações, a IR é convertida para código executável através do Backend do compilador.

Esta etapa traduz a representação intermediária para a arquitetura e o sistema operacional de destino.

---

## 15.1 Objetivos

A geração de código possui como responsabilidades:

* produzir código executável;
* preservar todas as garantias da linguagem;
* respeitar a ABI da plataforma de destino;
* integrar-se ao Runtime Mínimo da Capi.

---

## 15.2 Backend

A arquitetura do Backend não faz parte desta especificação.

Uma implementação poderá utilizar, entre outras alternativas:

* LLVM;
* Cranelift;
* GCC Backend;
* backend próprio;
* outras infraestruturas equivalentes.

Todas devem produzir programas semanticamente equivalentes.

Além da geração das instruções executáveis, o Backend é responsável por traduzir o Modelo de Memória da Capi para a representação física da plataforma de destino.

Isso inclui a geração do código responsável pelo gerenciamento dos Domains, pela organização dos objetos em memória e pela integração com o Runtime Mínimo, respeitando integralmente a ABI da plataforma de destino.

---

## 15.3 Artefatos Gerados

Dependendo da configuração da compilação, o Backend poderá produzir:

* executáveis;
* bibliotecas estáticas;
* bibliotecas compartilhadas;
* objetos intermediários;
* símbolos de depuração;
* metadados auxiliares.

A escolha dos artefatos pertence à Toolchain.

---

## 15.4 Portabilidade

A Capi foi projetada para compilação **Ahead-of-Time (AOT)**.

O executável gerado deverá poder ser distribuído e executado diretamente na plataforma de destino, sem a necessidade de máquinas virtuais, interpretadores ou ambientes de execução instaláveis específicos da linguagem.

O Runtime Mínimo necessário ao funcionamento do programa será incorporado ao executável ou distribuído conforme definido no **Documento 09 — Runtime Mínimo**.

---

## 15.5 Garantias

Independentemente do Backend utilizado, o código gerado deverá preservar integralmente:

* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* todas as garantias estabelecidas pela especificação da linguagem.

A geração de código representa a última etapa do compilador.

A partir desse ponto, o comportamento do programa passa a depender exclusivamente da plataforma de execução, da ABI correspondente e do Runtime Mínimo da Capi, todos definidos nos documentos subsequentes da especificação.

---

# 16. Diagnósticos

Um dos objetivos do compilador da Capi é produzir diagnósticos claros, precisos e úteis.

Sempre que possível, uma única compilação deverá identificar múltiplos problemas independentes, reduzindo o número de ciclos de correção necessários ao desenvolvimento.

---

## 16.1 Objetivos

O sistema de diagnósticos possui os seguintes objetivos:

* identificar erros com precisão;
* facilitar a compreensão do problema;
* sugerir possíveis correções;
* evitar mensagens redundantes;
* preservar a continuidade da compilação sempre que possível.

A qualidade dos diagnósticos constitui parte importante da experiência de desenvolvimento da linguagem.

---

## 16.2 Categorias

Os diagnósticos podem ser classificados em:

* Erros;
* Avisos (*Warnings*);
* Informações (*Notes*);
* Sugestões (*Hints*).

A classificação exata pertence à implementação, desde que preserve o significado de cada categoria.

---

## 16.3 Recuperação

Sempre que possível, o compilador deverá recuperar-se de erros para continuar analisando o restante do programa.

Essa recuperação permite identificar problemas adicionais em uma única compilação.

A estratégia utilizada pertence exclusivamente à implementação.

---

## 16.4 Localização

Todo diagnóstico deverá informar, sempre que aplicável:

* arquivo;
* linha;
* coluna;
* trecho do código;
* contexto do erro.

Implementações poderão fornecer informações adicionais, como destaque visual, cores ou referências cruzadas.

---

## 16.5 Sugestões

Sempre que possível, o compilador deverá fornecer sugestões automáticas para correção.

Exemplos:

* identificadores semelhantes;
* importações ausentes;
* incompatibilidades de tipos;
* métodos candidatos;
* conversões sugeridas.

Essas sugestões não fazem parte da semântica da linguagem e pertencem à implementação.

---

# 17. Compilação Incremental

A Capi foi projetada para permitir compilação incremental eficiente.

O objetivo é evitar recompilações desnecessárias e reduzir significativamente o tempo de compilação de projetos de grande porte.

---

## 17.1 Objetivos

A compilação incremental busca:

* recompilar apenas os módulos alterados;
* reutilizar resultados previamente produzidos;
* reduzir o tempo de compilação;
* facilitar o desenvolvimento iterativo.

---

## 17.2 Dependências

O compilador deverá identificar as dependências existentes entre módulos.

Sempre que um módulo sofrer alterações, apenas os artefatos afetados deverão ser invalidados.

O algoritmo utilizado para essa análise pertence à implementação.

Alterações que modifiquem a organização dos Domains, a hierarquia de classes ou outras propriedades que afetem o Modelo de Memória poderão invalidar dependências adicionais. O compilador deverá identificar essas relações e recompilar todos os módulos potencialmente afetados.

---

## 17.3 Cache

Implementações poderão manter cache de:

* AST;
* tabelas de símbolos;
* informações de tipos;
* IR;
* objetos compilados;
* metadados.

A organização física desse cache não faz parte da especificação.

---

## 17.4 Recompilação

A recompilação deverá preservar exatamente o mesmo comportamento que seria obtido por uma compilação completa.

A compilação incremental nunca poderá alterar:

* Sistema de Tipos;
* Modelo de Memória;
* Semântica Operacional;
* garantias da linguagem.

---

## 17.5 Determinismo

Independentemente da utilização de cache ou compilação incremental, o resultado final deverá ser semanticamente equivalente ao obtido por uma compilação completa realizada a partir do código-fonte original.

---

# 18. Extensibilidade

A arquitetura do compilador foi projetada para permitir evolução futura da linguagem sem comprometer sua estabilidade.

Este documento não define mecanismos específicos de extensão, mas estabelece os princípios que deverão orientar sua implementação.

---

## 18.1 Objetivos

A extensibilidade busca permitir:

* evolução da linguagem;
* desenvolvimento de ferramentas;
* integração com IDEs;
* análise estática;
* geração de documentação;
* futuras funcionalidades do compilador.

---

## 18.2 APIs do Compilador

Implementações poderão disponibilizar APIs para acesso às estruturas internas produzidas durante a compilação.

Entre elas:

* AST;
* tabela de símbolos;
* informações de tipos;
* diagnósticos;
* IR.

A existência e a forma dessas APIs pertencem à implementação.

---

## 18.3 Plugins

Compiladores poderão oferecer mecanismos para carregamento de plugins.

Plugins poderão atuar, por exemplo, em:

* análise estática;
* geração de documentação;
* inspeções adicionais;
* integração com ferramentas externas.

Nenhum plugin poderá modificar as garantias estabelecidas pela especificação da linguagem.

---

## 18.4 Macros

A presente versão da Capi não define suporte a macros.

Versões futuras poderão introduzir mecanismos controlados de geração de código, desde que:

* preservem o Sistema de Tipos;
* preservem o Modelo de Memória;
* preservem a Semântica Operacional;
* mantenham diagnósticos compreensíveis para o programador.

---

## 18.5 Ferramentas

A arquitetura do compilador deverá facilitar a integração com ferramentas do ecossistema, incluindo:

* formatadores de código;
* analisadores estáticos;
* geradores de documentação;
* depuradores;
* servidores de linguagem (Language Server Protocol — LSP).

Essas ferramentas serão especificadas no **Documento 11 — Toolchain**.

---

## 18.6 Evolução da Arquitetura

A arquitetura descrita neste documento não limita a evolução do compilador.

Novas fases, otimizações ou componentes poderão ser introduzidos futuramente, desde que preservem integralmente:

* o comportamento observável da linguagem;
* todas as garantias da especificação;
* o Princípio da Separação entre Garantias e Implementação.

Esse princípio assegura que a evolução tecnológica do compilador não implique alterações na semântica dos programas escritos em Capi.

A arquitetura também deverá permitir a evolução da própria linguagem, incluindo a introdução de novas construções sintáticas, novos mecanismos do Sistema de Tipos e novas fases de compilação, sem comprometer a estabilidade das fases existentes.

Esse princípio decorre diretamente da separação entre garantias da linguagem e mecanismos de implementação, permitindo que tanto a especificação quanto o compilador evoluam de forma controlada.

---

# 19. Princípio da Separação entre Garantias e Implementação

Este documento adota o mesmo princípio arquitetural estabelecido pelos Documentos 01, 02, 03, 04 e 05:

> A especificação da linguagem define **as garantias** que devem ser observadas pelos programas, enquanto a implementação do compilador define **os mecanismos** utilizados para produzir essas garantias.

Essa separação permite que diferentes implementações do compilador coexistam sem comprometer a compatibilidade da linguagem.

---

## 19.1 Garantias

As garantias estabelecidas pela especificação incluem, entre outras:

* preservação do Sistema de Tipos;
* preservação do Modelo de Memória;
* preservação da Semântica Operacional;
* integridade dos Domains;
* integridade dos `ObjectId`;
* exclusividade da Capacidade de Mutação;
* determinismo do gerenciamento de memória.

Essas garantias são obrigatórias para qualquer implementação da Capi.

---

## 19.2 Implementação

A implementação do compilador poderá utilizar quaisquer técnicas que preservem integralmente as garantias anteriores.

Entre elas:

* diferentes algoritmos de parsing;
* diferentes representações da AST;
* diferentes formatos de IR;
* diferentes estratégias de otimização;
* diferentes backends;
* diferentes estruturas internas de compilação.

A escolha dessas técnicas pertence exclusivamente à implementação.

---

## 19.3 Evolução

A arquitetura do compilador poderá evoluir continuamente.

Novas fases poderão ser introduzidas.

Fases existentes poderão ser reorganizadas.

Novas técnicas poderão substituir técnicas anteriores.

Nenhuma dessas alterações poderá modificar o comportamento observável da linguagem nem reduzir as garantias estabelecidas pela especificação.

---

# 20. Garantias do Compilador

Embora o compilador possua liberdade para organizar internamente sua implementação, ele deve preservar integralmente todas as propriedades definidas pela linguagem.

Este capítulo consolida essas responsabilidades.

---

## 20.1 Preservação da Linguagem

O compilador deve preservar integralmente:

* a Sintaxe;
* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional.

Nenhuma etapa da compilação poderá alterar essas definições.

---

## 20.2 Preservação da Segurança

Todo programa aceito pelo compilador deverá continuar obedecendo às garantias de segurança estabelecidas pela linguagem.

Em particular, o compilador não poderá introduzir:

* violações de memória;
* perda de identidade dos objetos;
* inconsistências entre Domains;
* violações da Capacidade de Mutação;
* comportamento indefinido em código seguro.

---

## 20.3 Preservação da Semântica

Toda transformação realizada durante a compilação deverá produzir um programa semanticamente equivalente ao programa original.

Essa propriedade aplica-se igualmente a:

* otimizações;
* compilação incremental;
* diferentes backends;
* diferentes plataformas de destino.

---

## 20.4 Portabilidade

Implementações distintas do compilador deverão produzir programas semanticamente equivalentes.

Diferenças de desempenho, organização interna ou geração de código não constituem diferenças da linguagem.

A especificação da Capi permanece única e independente da implementação utilizada.

---

# 21. Não Objetivos

Este documento define apenas a arquitetura lógica do compilador.

Diversos aspectos relacionados à implementação são deliberadamente deixados fora de seu escopo.

---

## 21.1 Implementação Interna

Este documento não especifica:

* linguagem utilizada para implementar o compilador;
* organização dos módulos internos;
* estruturas de dados específicas;
* algoritmos internos;
* arquitetura de software.

---

## 21.2 Representação Intermediária

A estrutura completa da IR será definida no Documento 07.

Este documento apenas estabelece seu papel dentro da arquitetura do compilador.

---

## 21.3 Runtime

O Runtime Mínimo da linguagem será definido no Documento 09.

Este documento apenas estabelece sua existência como parte da infraestrutura de execução.

---

## 21.4 Toolchain

Aspectos relacionados às ferramentas da linguagem serão definidos no Documento 11.

Entre eles:

* compilador (`capic`);
* gerenciador de pacotes;
* formatador;
* gerador de documentação;
* executores de testes.

---

## 21.5 Biblioteca Padrão

A biblioteca padrão não faz parte deste documento.

Sua especificação será apresentada no Documento 08.

---

# 22. Considerações Finais

A Arquitetura do Compilador estabelece a organização conceitual necessária para transformar programas escritos em Capi em código executável, preservando todas as garantias definidas pelos documentos anteriores da especificação.

Ao separar claramente as responsabilidades entre Frontend, Middle-end, Backend e Infraestrutura, a arquitetura proposta favorece implementações modulares, testáveis e evolutivas, sem impor tecnologias específicas ou restringir a inovação dos compiladores.

Este documento também reforça um dos princípios centrais da especificação da Capi: a distinção entre **garantias da linguagem** e **mecanismos de implementação**. Essa separação assegura que diferentes compiladores possam utilizar técnicas distintas — como diferentes parsers, representações intermediárias, algoritmos de otimização ou backends — produzindo programas semanticamente equivalentes.

Com a conclusão deste documento, encontra-se definida a arquitetura conceitual do compilador da Capi. Os documentos subsequentes detalharão os componentes que o integram, iniciando pela **Representação Intermediária (IR)**, responsável por servir como base comum para otimizações e geração de código, seguida pela Biblioteca Padrão, Runtime Mínimo, ABI e Interoperabilidade, Toolchain e Ecossistema da linguagem.

A Arquitetura do Compilador representa, portanto, a ponte entre a especificação formal da linguagem e sua implementação prática, estabelecendo as bases para compiladores robustos, portáveis e capazes de evoluir ao longo do tempo sem comprometer a estabilidade da Capi.
