# 15 — Parser e AST

## Especificação de Implementação da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a especificação de implementação da análise sintática (*Parsing*) do compilador oficial da linguagem Capi.

Seu objetivo é estabelecer como a sequência de tokens produzida pelo Lexer é transformada em uma Árvore Sintática Abstrata (*Abstract Syntax Tree* — AST), definindo os princípios arquiteturais, os artefatos produzidos e as responsabilidades da segunda fase do processo de compilação.

Este documento não define a gramática da linguagem, mas sim a implementação responsável por reconhecer sua estrutura sintática e construir uma representação abstrata do programa.

---

## 1.2 Escopo

Este documento especifica:

* o papel do Parser no pipeline de compilação;
* a construção da AST;
* a organização conceitual dos nós da AST;
* a interface entre Parser e Lexer;
* a interface entre Parser e HIR;
* os princípios arquiteturais da análise sintática;
* os mecanismos gerais de recuperação de erros sintáticos.

Não fazem parte deste documento:

* a gramática formal da linguagem;
* a análise léxica;
* a resolução de nomes;
* a inferência de tipos;
* a verificação semântica;
* a construção da HIR.

Esses assuntos são definidos pelos documentos correspondentes da especificação de implementação.

---

## 1.3 Relação com os Documentos Anteriores

Este documento complementa diretamente a especificação da linguagem e a arquitetura do compilador.

Em especial, sua implementação deve preservar integralmente as garantias estabelecidas por:

| Documento    | Responsabilidade          |
| ------------ | ------------------------- |
| Documento 04 | Sintaxe da Linguagem      |
| Documento 06 | Arquitetura do Compilador |
| Documento 13 | Estrutura do Compilador   |
| Documento 14 | Lexer e Tokens            |

O Documento 04 define a gramática da linguagem.

O Documento 14 define como essa gramática é convertida em uma sequência de tokens.

Este documento especifica como essa sequência é organizada em uma representação sintática estruturada.

---

## 1.4 Filosofia da Implementação

O Parser possui responsabilidade única: transformar uma sequência válida de tokens em uma representação sintática abstrata do programa.

Ele não interpreta o significado dos elementos sintáticos, não resolve símbolos e não realiza qualquer validação semântica.

Sua implementação deve ser:

* determinística;
* previsível;
* independente das fases semânticas do compilador;
* capaz de produzir diagnósticos precisos;
* capaz de recuperar-se de erros sintáticos sempre que possível.

Essa separação de responsabilidades reduz o acoplamento entre as fases do Frontend e facilita a evolução independente de cada componente.

---

## 1.5 Separação entre Especificação e Implementação

Este documento define o comportamento esperado da análise sintática, mas não impõe uma estratégia específica de implementação.

Uma implementação poderá utilizar, por exemplo:

* *Recursive Descent*;
* *Pratt Parser*;
* analisadores LL;
* analisadores LR;
* PEG (*Parsing Expression Grammar*);
* geradores automáticos de Parser;
* outras técnicas equivalentes.

Independentemente da estratégia adotada, todas as implementações deverão produzir uma AST funcionalmente equivalente para um mesmo programa válido e emitir diagnósticos compatíveis para entradas inválidas.

---

# 2. Papel do Parser

O Parser constitui a segunda fase do pipeline de compilação.

Sua função é consumir a sequência de tokens produzida pelo Lexer e organizá-la segundo a gramática da linguagem, produzindo uma Árvore Sintática Abstrata (AST).

Ao término dessa etapa, o programa deixa de ser representado como uma sequência linear de tokens e passa a ser representado como uma estrutura hierárquica.

---

## 2.1 Entrada

A entrada do Parser consiste exclusivamente na sequência de tokens produzida pelo Lexer.

O Parser não acessa diretamente o código-fonte nem depende dos mecanismos utilizados pela análise léxica para reconhecer os elementos da linguagem.

Essa separação preserva a independência entre as fases do Frontend.

---

## 2.2 Saída

A saída do Parser é uma Árvore Sintática Abstrata (AST).

A AST representa exclusivamente a estrutura sintática do programa, preservando sua organização lógica sem incorporar informações semânticas.

Além da AST, o Parser poderá produzir diagnósticos referentes a erros sintáticos encontrados durante a análise.

---

## 2.3 Responsabilidades

Compete ao Parser:

* consumir a sequência de tokens;
* reconhecer a estrutura sintática do programa;
* construir a AST;
* preservar a localização dos elementos sintáticos;
* emitir diagnósticos sintáticos;
* recuperar-se de erros sintáticos sempre que possível.

Cada uma dessas responsabilidades deve ser executada sem depender da resolução de nomes ou de qualquer informação semântica.

---

## 2.4 Limitações

Não compete ao Parser:

* resolver identificadores;
* construir tabelas de símbolos;
* inferir tipos;
* verificar regras semânticas;
* validar acessibilidade;
* avaliar expressões constantes.

Essas responsabilidades pertencem às fases posteriores do Frontend.

---

## 2.5 Determinismo

O Parser deve produzir resultados determinísticos.

Dada a mesma sequência de tokens, utilizando a mesma versão do compilador e as mesmas opções de compilação, a AST produzida deverá ser funcionalmente equivalente.

Esse princípio garante previsibilidade, reprodutibilidade e estabilidade da implementação.

---

## 2.6 Independência Arquitetural

O Parser deve permanecer completamente independente das fases posteriores do compilador.

Ele não deve depender:

* da HIR;
* da resolução de nomes;
* do sistema de tipos;
* da verificação semântica;
* da IR;
* do Backend.

Sua única responsabilidade consiste em transformar tokens em uma representação sintática estruturada.

---

# 3. Fluxo da Análise Sintática

A análise sintática representa a segunda transformação realizada pelo compilador.

Seu funcionamento consiste em consumir a sequência de tokens produzida pelo Lexer, reconhecer sua organização segundo a gramática da linguagem e construir a AST correspondente.

---

## 3.1 Visão Geral

Conceitualmente, o fluxo da análise sintática pode ser representado da seguinte forma:

```text
Sequência de Tokens
        │
        ▼
      Parser
        │
        ▼
        AST
        │
        ▼
        HIR
```

A AST constitui a representação sintática oficial produzida pelo Parser e serve como entrada para a construção da HIR.

---

## 3.2 Consumo dos Tokens

O Parser percorre a sequência de tokens produzida pelo Lexer de forma ordenada.

Durante esse processo, cada token é interpretado exclusivamente segundo as regras sintáticas definidas pela gramática da linguagem.

Os mecanismos utilizados para consumir os tokens pertencem exclusivamente à implementação.

---

## 3.3 Reconhecimento Sintático

O reconhecimento sintático consiste na identificação das construções previstas pela gramática da linguagem.

Cada construção reconhecida dá origem a um ou mais nós da AST, preservando a organização estrutural do programa.

As regras específicas para módulos, declarações, comandos, expressões e tipos são apresentadas nos capítulos seguintes.

---

## 3.4 Construção da AST

Sempre que uma construção sintática válida for reconhecida, o Parser deverá produzir os nós correspondentes da AST.

Cada nó representa uma construção da linguagem e mantém as informações necessárias para permitir que as fases posteriores compreendam a estrutura do programa.

A organização detalhada da AST é definida neste documento.

---

## 3.5 Comunicação com a HIR

A comunicação entre o Parser e a fase responsável pela construção da HIR ocorre exclusivamente por meio da AST.

A fase seguinte não deve depender dos mecanismos internos utilizados pelo Parser nem da sequência original de tokens.

Essa separação preserva o baixo acoplamento entre as fases do Frontend.

---

## 3.6 Princípio da Responsabilidade Única

A análise sintática encerra-se no momento em que a AST é completamente construída.

Qualquer interpretação relacionada a símbolos, tipos, escopos ou regras semânticas pertence exclusivamente às fases posteriores do compilador.

Esse princípio preserva a modularidade do Frontend e garante que o Parser permaneça um componente simples, previsível e facilmente testável.

---

# 4. Papel da AST

A Árvore Sintática Abstrata (*Abstract Syntax Tree* — AST) constitui a representação estrutural produzida pela análise sintática.

Seu objetivo é representar a organização lógica do programa de forma independente da representação textual original, preservando apenas as informações necessárias para as fases posteriores do compilador.

A AST representa a fronteira entre a análise sintática e a análise semântica do Frontend.

---

## 4.1 Objetivo

Este documento define os princípios arquiteturais que orientam a construção da AST. A estrutura concreta dos nós, seus campos e sua organização física pertencem exclusivamente à implementação, desde que preservem os contratos estabelecidos nesta especificação.

A AST possui os seguintes objetivos:

* representar a estrutura sintática do programa;
* eliminar detalhes puramente léxicos;
* servir como entrada para a construção da HIR;
* preservar a organização hierárquica do código;
* manter rastreabilidade com o código-fonte.

---

## 4.2 Representação Abstrata

A AST representa a estrutura lógica do programa, e não sua forma textual.

Elementos utilizados apenas para facilitar a escrita do código-fonte, como espaços em branco, comentários e detalhes de formatação, não fazem parte da AST.

Diferentes representações textuais equivalentes devem produzir ASTs funcionalmente equivalentes.

---

## 4.3 Fronteira entre Sintaxe e Semântica

A AST representa o último artefato puramente sintático produzido pelo Frontend.

Ela não contém informações relacionadas a:

* resolução de nomes;
* escopos;
* tipos;
* símbolos;
* verificações semânticas.

Essas informações passam a ser incorporadas apenas durante a construção da HIR.

---

## 4.4 Hierarquia

A AST organiza o programa como uma estrutura hierárquica composta por nós.

Cada nó representa exatamente uma construção sintática reconhecida pela gramática da linguagem.

As relações entre os nós representam a organização estrutural do programa.

---

## 4.5 Imutabilidade Conceitual

Após sua construção, a AST deve ser considerada conceitualmente imutável.

As fases posteriores do compilador não devem alterar sua estrutura, utilizando-a apenas como referência para produzir representações mais elaboradas.

A estratégia utilizada para preservar essa propriedade pertence à implementação.

---

## 4.6 Independência da Implementação

A forma utilizada para representar internamente a AST pertence exclusivamente à implementação.

Independentemente da estrutura adotada, todas as implementações deverão produzir uma representação sintática funcionalmente equivalente para um mesmo programa.

---

# 5. Estrutura Geral da AST

A AST é composta por um conjunto organizado de nós que representam as construções sintáticas previstas pela linguagem.

A organização exata desses nós pertence à implementação, desde que preserve a estrutura lógica definida pela gramática da linguagem.

---

## 5.1 Nós da AST

Cada elemento sintático reconhecido pelo Parser é representado por um nó da AST.

Conceitualmente, um nó representa uma única construção sintática da linguagem.

A estrutura física desses nós pertence exclusivamente à implementação.

---

## 5.2 Categorias de Nós

Os nós da AST podem ser agrupados em categorias conceituais, incluindo:

* módulos;
* declarações;
* comandos;
* expressões;
* tipos;
* atributos sintáticos.

Essas categorias refletem exclusivamente a organização da gramática da linguagem.

---

## 5.3 Relações Hierárquicas

Os nós da AST estabelecem relações de composição entre si.

Por exemplo:

* um módulo contém declarações;
* uma função contém parâmetros e um corpo;
* um bloco contém comandos;
* uma expressão pode conter subexpressões.

Essas relações representam a estrutura sintática do programa.

---

## 5.4 Nós Folha

Alguns nós representam elementos indivisíveis da estrutura sintática.

Exemplos incluem:

* identificadores;
* literais;
* operadores;
* qualificadores.

Esses nós constituem as folhas da árvore sintática.

---

## 5.5 Nós Compostos

Outros nós representam construções compostas por diversos elementos sintáticos.

Exemplos incluem:

* funções;
* classes;
* expressões compostas;
* comandos condicionais;
* estruturas de repetição.

Esses nós organizam a hierarquia da AST.

---

## 5.6 Equivalência Estrutural

Programas sintaticamente equivalentes devem produzir ASTs estruturalmente equivalentes.

Diferenças de formatação, comentários ou organização visual do código-fonte não devem alterar a estrutura produzida pelo Parser.

---

# 6. Localização e Rastreabilidade

Embora a AST represente uma abstração da estrutura sintática, ela deve preservar informações suficientes para permitir a rastreabilidade até o código-fonte original.

Essa propriedade é fundamental para emissão de diagnósticos, geração de informações de depuração e integração com ferramentas de desenvolvimento.

---

## 6.1 Objetivo

A rastreabilidade permite estabelecer a correspondência entre os nós da AST e os elementos originais do código-fonte.

Essa correspondência deve permanecer disponível durante toda a análise semântica do Frontend.

---

## 6.2 Span dos Nós

Sempre que aplicável, cada nó da AST deverá estar associado ao Span correspondente no código-fonte.

Esse Span representa a localização completa da construção sintática representada pelo nó.

A definição formal do Span encontra-se no Documento 14 — Lexer e Tokens.

---

## 6.3 Origem dos Tokens

Os nós da AST são construídos a partir dos tokens produzidos pelo Lexer.

A implementação poderá manter referências aos tokens originais sempre que isso facilitar:

* emissão de diagnósticos;
* depuração do compilador;
* geração de informações auxiliares.

A estratégia utilizada pertence exclusivamente à implementação.

---

## 6.4 Diagnósticos

Os diagnósticos emitidos pelas fases posteriores do compilador deverão utilizar, sempre que possível, as informações de localização preservadas pela AST.

Essa abordagem permite apresentar mensagens de erro precisas e diretamente relacionadas ao código escrito pelo desenvolvedor.

---

## 6.5 Rastreabilidade

A relação entre código-fonte, tokens e AST deve permanecer consistente durante toda a construção da representação sintática.

Essa propriedade facilita:

* depuração do compilador;
* ferramentas de análise estática;
* navegação em IDEs;
* geração de documentação.

---

## 6.6 Continuidade para a HIR

Durante a construção da HIR, as informações de localização preservadas pela AST deverão permanecer disponíveis sempre que necessárias.

Essa continuidade garante que os diagnósticos produzidos nas fases semânticas continuem referenciando corretamente o código-fonte original, preservando a rastreabilidade estabelecida pela análise sintática.

---

# 7. Parsing de Módulos e Imports

O Parser inicia a construção da AST reconhecendo a estrutura superior do arquivo-fonte.

Essa etapa identifica os elementos responsáveis pela organização modular do programa, estabelecendo a base sobre a qual todas as demais construções sintáticas serão representadas.

---

## 7.1 Objetivo

O reconhecimento da estrutura superior do arquivo possui os seguintes objetivos:

* identificar o módulo ao qual o arquivo pertence;
* reconhecer declarações de importação;
* estabelecer a organização inicial da AST;
* preparar a análise das declarações contidas no módulo.

---

## 7.2 Declaração de Módulo

Quando a linguagem definir uma declaração explícita de módulo, o Parser deverá reconhecê-la e produzir o nó correspondente na AST.

Esse nó representa o elemento raiz da unidade de compilação e organiza todas as declarações pertencentes ao módulo.

A sintaxe da declaração de módulo é definida pelo Documento 04 — Sintaxe da Linguagem.

---

## 7.3 Declarações de Importação

O Parser deverá reconhecer as declarações de importação previstas pela linguagem.

Cada importação dá origem a um nó específico da AST, preservando exclusivamente sua estrutura sintática.

A resolução efetiva das dependências entre módulos pertence à fase de Resolução de Nomes, definida no Documento 17.

---

## 7.4 Organização do Arquivo

Após reconhecer módulo e importações, o Parser deverá processar as declarações presentes no arquivo na ordem estabelecida pela gramática.

A AST deve preservar essa organização estrutural, independentemente da forma utilizada pela implementação para representá-la internamente.

---

## 7.5 Unidade de Compilação

Cada arquivo-fonte corresponde a uma unidade independente de compilação.

O Parser deverá produzir exatamente uma AST para cada unidade processada.

A coordenação entre múltiplas unidades pertence à infraestrutura do compilador.

---

## 7.6 Independência Semântica

Durante o reconhecimento de módulos e importações, o Parser não deve verificar:

* existência dos módulos importados;
* acessibilidade;
* visibilidade;
* ciclos de dependência;
* compatibilidade entre módulos.

Essas verificações pertencem exclusivamente às fases semânticas do compilador.

---

# 8. Parsing de Declarações

As declarações representam os elementos responsáveis por introduzir novos componentes na estrutura do programa.

O Parser deve reconhecer essas construções conforme a gramática da linguagem e produzir os respectivos nós da AST.

---

## 8.1 Objetivo

O reconhecimento das declarações possui os seguintes objetivos:

* identificar construções declarativas;
* organizar a estrutura da AST;
* preservar a hierarquia sintática do programa;
* preparar a representação utilizada pelas fases posteriores.

---

## 8.2 Categorias de Declarações

Entre as declarações reconhecidas pela linguagem incluem-se, quando aplicável:

* módulos;
* funções;
* estruturas;
* enumerações;
* interfaces;
* traits;
* constantes;
* variáveis globais;
* aliases de tipos;
* demais construções definidas pela gramática.

A lista exata é determinada pelo Documento 04.

---

## 8.3 Modificadores

Quando a linguagem permitir modificadores aplicados às declarações, o Parser deverá reconhecê-los como parte integrante da estrutura sintática.

Exemplos incluem modificadores de:

* visibilidade;
* mutabilidade;
* abstração;
* outros qualificadores previstos pela linguagem.

O significado desses modificadores pertence às fases posteriores.

---

## 8.4 Organização Hierárquica

As declarações podem conter outras construções sintáticas.

Por exemplo:

* uma função possui parâmetros e corpo;
* uma estrutura possui campos;
* uma interface possui membros;
* um módulo contém diversas declarações.

A AST deve preservar integralmente essa hierarquia.

---

## 8.5 Ordem das Declarações

O Parser deve preservar a ordem em que as declarações aparecem no código-fonte.

Essa informação pode ser relevante para:

* emissão de diagnósticos;
* geração de documentação;
* ferramentas de análise;
* depuração do compilador.

---

## 8.6 Independência da Resolução

O Parser reconhece apenas a estrutura sintática das declarações.

Ele não verifica:

* duplicidade de nomes;
* conflitos entre declarações;
* acessibilidade;
* regras de escopo;
* consistência semântica.

Essas responsabilidades pertencem à fase de Resolução de Nomes e às verificações semânticas.

---

# 9. Parsing de Tipos

A linguagem Capi possui uma sintaxe própria para representação de tipos.

O Parser deve reconhecer essas construções e produzir sua representação sintática na AST, sem interpretar seu significado ou validar sua consistência.

---

## 9.1 Objetivo

O reconhecimento das construções de tipos possui os seguintes objetivos:

* identificar expressões de tipo;
* construir sua representação sintática;
* preservar sua organização estrutural;
* disponibilizar essas informações para as fases semânticas.

Nota: a representação sintática dos tipos reconhecida pelo Parser baseia-se nas construções definidas pelo Documento 02 — Sistema de Tipos. Nesta etapa, o Parser limita-se a reconhecer sua estrutura sintática, preservando-a para as fases semânticas posteriores.

---

## 9.2 Categorias de Tipos

Entre as construções sintáticas relacionadas a tipos incluem-se:

* tipos primitivos;
* tipos nomeados;
* tipos compostos;
* tipos genéricos;
* arrays;
* tuplas;
* tipos funcionais;
* demais construções previstas pela linguagem.

Sua sintaxe é definida pelo Documento 04.

---

## 9.3 Tipos Genéricos

O Parser deverá reconhecer corretamente as construções sintáticas relacionadas à genericidade.

A associação entre parâmetros genéricos, argumentos de tipo e respectivas restrições pertence às fases posteriores do compilador.

---

## 9.4 Construções Compostas

Tipos podem conter outros tipos como componentes.

Exemplos incluem:

* coleções parametrizadas;
* tipos opcionais;
* resultados;
* funções;
* estruturas compostas.

A AST deve preservar integralmente essa composição hierárquica.

---

## 9.5 Tipos Qualificados

Quando a linguagem permitir referências qualificadas a tipos pertencentes a módulos ou namespaces, o Parser deverá reconhecer sua estrutura sintática.

A resolução dessas referências pertence à fase de Resolução de Nomes.

---

## 9.6 Independência Semântica

O Parser não interpreta o significado das construções de tipos.

Ele não verifica:

* existência do tipo;
* compatibilidade;
* equivalência;
* restrições genéricas;
* regras de subtipagem.

Essas verificações pertencem às fases responsáveis pela Resolução de Nomes, Inferência de Tipos e Verificação Semântica.

---

# 10. Parsing de Comandos

Os comandos representam as construções responsáveis por controlar o fluxo de execução do programa.

O Parser deve reconhecer essas construções conforme a gramática da linguagem e produzir os respectivos nós da AST, preservando exclusivamente sua organização sintática.

---

## 10.1 Objetivo

O reconhecimento dos comandos possui os seguintes objetivos:

* identificar estruturas de controle;
* construir a representação sintática dos comandos;
* preservar a hierarquia entre blocos e subcomandos;
* preparar a AST para as fases posteriores do Frontend.

---

## 10.2 Blocos

Blocos representam agrupamentos de comandos executados como uma unidade lógica.

O Parser deverá reconhecer blocos e produzir os respectivos nós na AST, preservando a ordem sintática dos comandos que os compõem.

A interpretação do escopo introduzido por um bloco pertence às fases posteriores.

---

## 10.3 Comandos de Declaração

O Parser deverá reconhecer comandos responsáveis pela introdução de variáveis locais e demais construções declarativas permitidas dentro de blocos.

Entre eles incluem-se, quando aplicável:

* declarações locais;
* inicializações;
* constantes locais;
* demais construções previstas pela gramática.

A validação dessas declarações pertence à análise semântica.

---

## 10.4 Estruturas de Controle

O Parser deverá reconhecer as estruturas de controle definidas pela linguagem.

Entre elas incluem-se, quando aplicável:

* seleção;
* repetição;
* desvio;
* retorno;
* casamento de padrões (*pattern matching*);
* demais construções previstas pela gramática.

Cada estrutura deve ser representada por um ou mais nós específicos da AST.

---

## 10.5 Comandos Compostos

Alguns comandos podem conter outros comandos como parte de sua estrutura.

Exemplos incluem:

* blocos condicionais;
* laços de repetição;
* estruturas de seleção;
* comandos aninhados.

A AST deve preservar integralmente essa organização hierárquica.

---

## 10.6 Independência Semântica

O Parser reconhece apenas a estrutura sintática dos comandos.

Ele não verifica:

* alcançabilidade;
* fluxo de controle;
* inicialização de variáveis;
* retorno obrigatório;
* regras de escopo.

Essas verificações pertencem às fases posteriores do compilador.

---

# 11. Parsing de Expressões

As expressões representam construções sintáticas utilizadas para representar valores, operações e cálculos definidos pela linguagem.

O Parser deve reconhecer essas construções conforme a gramática da linguagem e produzir sua representação na AST.

A sintaxe das expressões e dos operadores é definida pelo Documento 04 — Sintaxe da Linguagem. Seu comportamento e significado são definidos pelo Documento 05 — Semântica Operacional.

---

## 11.1 Objetivo

O reconhecimento das expressões possui os seguintes objetivos:

* identificar construções que representam valores;
* construir sua representação sintática;
* preservar a hierarquia entre subexpressões;
* preparar a AST para as fases semânticas.

---

## 11.2 Categorias de Expressões

Entre as expressões reconhecidas pela linguagem incluem-se:

* literais;
* identificadores;
* chamadas de função;
* acesso a membros;
* indexação;
* criação de objetos;
* agrupamentos;
* expressões compostas.

A sintaxe detalhada dessas construções pertence ao Documento 04.

---

## 11.3 Expressões Simples

Expressões simples representam construções indivisíveis da gramática.

Exemplos incluem:

* literais;
* identificadores;
* referências;
* valores constantes.

Essas expressões normalmente correspondem às folhas da árvore sintática.

---

## 11.4 Expressões Compostas

Expressões compostas são formadas pela combinação de outras expressões.

Entre elas incluem-se:

* operações binárias;
* operações unárias;
* chamadas encadeadas;
* acessos sucessivos;
* agrupamentos;
* demais construções compostas.

A AST deve preservar integralmente essa composição.

---

## 11.5 Encadeamento

O Parser deverá reconhecer corretamente construções sintáticas compostas por múltiplas operações encadeadas.

Exemplos incluem:

* chamadas sucessivas;
* acesso a membros;
* indexações;
* encadeamentos previstos pela linguagem.

A resolução do significado dessas construções pertence às fases posteriores.

---

## 11.6 Independência Semântica

O Parser não interpreta o significado das expressões.

Ele não verifica:

* tipos;
* conversões;
* sobrecarga;
* resolução de chamadas;
* avaliação de constantes.

Essas responsabilidades pertencem às fases semânticas do compilador.

---

# 12. Precedência e Associatividade

A linguagem define regras que determinam como expressões compostas devem ser agrupadas durante a análise sintática.

O Parser é responsável por aplicar essas regras durante a construção da AST, garantindo que sua estrutura reflita corretamente a gramática da linguagem.

---

## 12.1 Objetivo

O tratamento da precedência e da associatividade possui os seguintes objetivos:

* eliminar ambiguidades sintáticas;
* produzir uma AST determinística;
* preservar a estrutura definida pela gramática;
* garantir interpretação sintática consistente.

---

## 12.2 Precedência

Quando diferentes operadores puderem participar da mesma expressão, o Parser deverá respeitar a ordem de precedência definida pela linguagem.

Essa ordem determina como as subexpressões são agrupadas na AST.

A tabela oficial de precedência pertence ao Documento 04 — Sintaxe da Linguagem.

---

## 12.3 Associatividade

Quando operadores de mesma precedência aparecerem em sequência, o Parser deverá aplicar as regras de associatividade definidas pela linguagem.

Essas regras determinam a ordem de agrupamento entre operadores equivalentes.

---

## 12.4 Operadores Prefixados e Pós-fixados

O Parser deverá reconhecer corretamente operadores aplicados antes ou depois de uma expressão.

A forma como esses operadores participam da árvore sintática deve obedecer às regras estabelecidas pela gramática da linguagem.

---

## 12.5 Agrupamentos Explícitos

Construções utilizadas para explicitar a ordem de avaliação, como agrupamentos delimitados por parênteses ou mecanismos equivalentes, deverão ser reconhecidas pelo Parser conforme a gramática.

Esses agrupamentos prevalecem sobre as regras gerais de precedência e associatividade durante a construção da AST.

---

## 12.6 Determinismo

As regras de precedência e associatividade devem produzir uma única AST para uma mesma sequência válida de tokens.

Independentemente da estratégia de implementação adotada, todas as implementações do Parser deverão construir árvores sintáticas funcionalmente equivalentes para um mesmo programa válido.

---

# 13. Estratégia de Implementação

Este documento define o comportamento esperado da análise sintática, mas não impõe uma estratégia específica para sua implementação.

Diferentes técnicas podem ser empregadas para reconhecer a gramática da linguagem e construir a AST, desde que produzam representações sintáticas funcionalmente equivalentes e preservem os contratos estabelecidos nesta especificação.

---

## 13.1 Objetivo

A implementação do Parser deve buscar:

* simplicidade;
* previsibilidade;
* eficiência;
* facilidade de manutenção;
* facilidade de evolução;
* facilidade de testes.

A escolha da estratégia pertence exclusivamente ao projeto de implementação.

---

## 13.2 Técnicas de Implementação

Entre as estratégias normalmente utilizadas para implementar analisadores sintáticos encontram-se:

* *Recursive Descent*;
* *Pratt Parser*;
* analisadores LL;
* analisadores LR;
* PEG (*Parsing Expression Grammar*);
* geradores automáticos de Parser;
* outras técnicas equivalentes.

Todas essas abordagens são compatíveis com esta especificação.

---

## 13.3 Critérios de Escolha

A escolha da estratégia poderá considerar fatores como:

* complexidade da gramática;
* desempenho;
* consumo de memória;
* simplicidade da implementação;
* facilidade de manutenção;
* integração com o restante do compilador.

Esses critérios pertencem exclusivamente ao projeto de implementação.

---

## 13.4 Independência Arquitetural

Independentemente da estratégia utilizada, o Parser deve permanecer completamente independente:

* da HIR;
* da resolução de nomes;
* do sistema de tipos;
* da verificação semântica;
* da IR;
* do Backend.

Essa independência constitui um dos princípios fundamentais da arquitetura do compilador.

---

## 13.5 Equivalência Funcional

Implementações distintas devem produzir ASTs funcionalmente equivalentes para um mesmo programa válido.

Da mesma forma, programas sintaticamente inválidos deverão resultar em diagnósticos funcionalmente equivalentes e em comportamento compatível com esta especificação.

---

## 13.6 Evolução

A estratégia de implementação poderá evoluir ao longo do tempo.

Mudanças internas são permitidas desde que preservem:

* a interface pública do Parser;
* os contratos definidos neste documento;
* o comportamento observável da análise sintática.

---

# 14. Interface do Parser

A interface do Parser define o contrato de comunicação entre a análise sintática e as demais fases do compilador.

Seu objetivo é receber a sequência de tokens produzida pelo Lexer e disponibilizar a AST para a fase responsável pela construção da HIR, ocultando completamente os detalhes internos da análise sintática.

A forma concreta dessa interface pertence à implementação, desde que preserve os princípios estabelecidos neste documento.

---

## 14.1 Objetivo

A interface do Parser deve:

* consumir a sequência de tokens produzida pelo Lexer;
* disponibilizar a AST produzida;
* preservar a independência entre as fases do Frontend;
* ocultar os mecanismos internos da análise sintática.

---

## 14.2 Entrada

O Parser recebe como entrada:

* a sequência de tokens produzida pelo Lexer;
* os diagnósticos eventualmente emitidos durante a análise léxica;
* o contexto necessário para emissão de novos diagnósticos.

O fornecimento dessas informações é responsabilidade da infraestrutura definida no Documento 13.

---

## 14.3 Saída

A saída do Parser consiste na AST correspondente ao programa analisado.

Cada nó da árvore deverá conter todas as informações necessárias para permitir sua utilização pelas fases posteriores do Frontend, incluindo:

* categoria sintática;
* relações hierárquicas;
* informações de localização;
* metadados definidos pela implementação.

---

## 14.4 Consumo dos Tokens

O Parser consome a sequência de tokens de forma ordenada.

A implementação poderá utilizar diferentes mecanismos para acessar essa sequência, como:

* iteradores;
* buffers;
* fluxos (*streams*);
* geradores (*generators*);
* outras estratégias equivalentes.

A estratégia adotada pertence exclusivamente à implementação.

---

## 14.5 Comunicação com a HIR

A AST constitui a única interface formal entre o Parser e a fase responsável pela construção da HIR.

A construção da HIR não deve depender:

* da sequência original de tokens;
* do código-fonte;
* dos mecanismos internos utilizados pelo Parser.

Essa separação preserva o baixo acoplamento entre as fases do Frontend.

---

## 14.6 Contrato da Interface

Independentemente da implementação adotada, toda interface do Parser deve garantir:

* construção determinística da AST;
* preservação das informações de localização;
* emissão consistente de diagnósticos;
* compatibilidade com a construção da HIR;
* comportamento equivalente para uma mesma sequência de tokens.

---

# 15. AST Imutável e Construção Incremental

A AST representa um artefato estável produzido pela análise sintática.

Após sua construção, ela passa a servir como base para todas as fases semânticas do Frontend, devendo permanecer conceitualmente imutável durante o restante da compilação.

Essa característica favorece a previsibilidade do pipeline e prepara a arquitetura para futuras estratégias de compilação incremental.

---

## 15.1 Objetivo

A estabilidade da AST possui os seguintes objetivos:

* preservar a separação entre análise sintática e análise semântica;
* reduzir o acoplamento entre as fases;
* facilitar a reutilização da AST;
* permitir futuras otimizações de compilação.

---

## 15.2 Imutabilidade

Após produzida pelo Parser, a AST deve ser considerada conceitualmente imutável.

As fases posteriores do compilador não devem alterar sua estrutura, utilizando-a apenas como entrada para construir novas representações internas.

A HIR é construída a partir da AST, mas não a modifica. Dessa forma, a AST permanece disponível como referência durante toda a compilação, preservando a rastreabilidade com o código-fonte e servindo de base para diagnósticos e ferramentas de desenvolvimento.

Os mecanismos utilizados para garantir essa propriedade pertencem exclusivamente à implementação.

---

## 15.3 Construção Incremental

A implementação poderá construir a AST de forma incremental durante o consumo da sequência de tokens.

Independentemente da estratégia adotada, o artefato final deverá ser funcionalmente equivalente ao produzido por uma construção integral.

---

## 15.4 Alocação de Memória

A implementação poderá utilizar diferentes estratégias para armazenar os nós da AST.

Entre elas incluem-se:

* alocação tradicional;
* arenas (*Arena Allocation*);
* pools de objetos;
* outras estratégias equivalentes.

A escolha pertence exclusivamente à implementação.

---

## 15.5 Evolução para Compilação Incremental

A arquitetura do Parser deve permitir futura integração com mecanismos de compilação incremental.

Essa capacidade poderá incluir:

* reutilização parcial da AST;
* reconstrução localizada de subárvores;
* invalidação seletiva de nós;
* atualização incremental após modificações do código-fonte.

A implementação inicial não é obrigada a oferecer essas funcionalidades.

---

## 15.6 Independência da Implementação

Os mecanismos utilizados para construir, armazenar e atualizar a AST pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar:

* a estrutura sintática da AST;
* os contratos definidos neste documento;
* a compatibilidade com a construção da HIR;
* o comportamento determinístico da análise sintática.

---

# 16. Diagnósticos e Recuperação Sintática

O Parser é responsável por identificar e reportar todos os erros relacionados à estrutura sintática do programa.

Sempre que possível, esses erros devem ser apresentados de forma clara e precisa, permitindo que o desenvolvedor identifique sua origem e continue obtendo diagnósticos adicionais durante a mesma compilação.

Os mecanismos gerais de emissão de diagnósticos são definidos pelo Documento 23 — Diagnósticos e Recuperação de Erros. Este capítulo especifica apenas os aspectos particulares da análise sintática.

---

## 16.1 Objetivo

O sistema de diagnósticos da análise sintática possui os seguintes objetivos:

* identificar erros durante o reconhecimento da gramática;
* localizar precisamente a origem de cada erro;
* permitir a continuidade da análise sempre que possível;
* fornecer informações suficientes para facilitar a correção do código.

---

## 16.2 Categorias de Erros

Entre os erros que podem ser detectados pelo Parser incluem-se:

* tokens inesperados;
* ausência de tokens obrigatórios;
* construções sintáticas incompletas;
* agrupamentos não encerrados;
* declarações malformadas;
* expressões inválidas;
* demais violações da gramática da linguagem.

Novas categorias poderão ser incorporadas em versões futuras da linguagem.

---

## 16.3 Localização

Todo diagnóstico emitido pelo Parser deverá estar associado ao Span correspondente na AST ou aos tokens relacionados ao erro.

Sempre que possível, o diagnóstico deverá identificar:

* arquivo;
* linha;
* coluna;
* intervalo de caracteres relacionado ao erro.

Essa informação permite que ferramentas de desenvolvimento apresentem diagnósticos precisos ao usuário.

---

## 16.4 Recuperação

Sempre que um erro sintático puder ser recuperado com segurança, o Parser deverá continuar sua execução.

O objetivo da recuperação é permitir que múltiplos erros sejam reportados em uma única compilação, reduzindo o número de ciclos de correção necessários.

A recuperação de erros sintáticos pode envolver a identificação de um ponto seguro na sequência de tokens (por exemplo, o próximo token que permita retomar uma produção válida da gramática).

A estratégia exata utilizada para essa recuperação pertence exclusivamente à implementação.

---

## 16.5 Mecanismos Internos de Recuperação

Durante a recuperação de erros sintáticos, a implementação poderá utilizar mecanismos internos para preservar a consistência da análise sintática e do pipeline de compilação.

Esses mecanismos pertencem exclusivamente à implementação e não fazem parte da interface formal definida por este documento.

---

## 16.6 Independência

O Parser deve emitir apenas diagnósticos relacionados à análise sintática.

Erros de natureza léxica, semântica, relacionados à resolução de nomes ou ao sistema de tipos pertencem exclusivamente às fases correspondentes do compilador.

---

# 17. Testabilidade

A análise sintática constitui um dos componentes fundamentais do Frontend e pode ser validada de forma independente das fases semânticas do compilador.

Sua natureza determinística permite a construção de testes automatizados abrangentes, garantindo elevado nível de confiabilidade e facilitando a evolução da implementação.

A implementação deve manter uma suíte abrangente de testes automatizados, incluindo testes de regressão para cada erro sintático identificado e posteriormente corrigido. Esses testes garantem que defeitos anteriormente solucionados não sejam reintroduzidos em versões futuras do compilador.

---

## 17.1 Objetivo

Os testes do Parser possuem os seguintes objetivos:

* validar o reconhecimento correto das construções sintáticas;
* detectar regressões;
* verificar a estabilidade da interface com o Lexer e a HIR;
* garantir comportamento determinístico.

---

## 17.2 Testes Unitários

Cada construção sintática deverá possuir testes específicos.

Exemplos incluem:

* módulos;
* declarações;
* tipos;
* comandos;
* expressões;
* precedência;
* associatividade.

Esses testes verificam exclusivamente o comportamento da análise sintática.

---

## 17.3 Golden Tests

A implementação poderá utilizar *Golden Tests* para validar a AST produzida a partir de programas previamente conhecidos.

Cada execução compara a árvore produzida com uma representação previamente validada, permitindo detectar alterações inesperadas no comportamento do Parser.

---

## 17.4 Testes de Diagnósticos

Além dos casos válidos, deverão existir testes destinados à validação dos diagnósticos emitidos pelo Parser.

Esses testes verificam aspectos como:

* categoria do erro;
* localização;
* recuperação da análise;
* continuidade da construção da AST.

---

## 17.5 Benchmarks

A implementação poderá incluir benchmarks destinados a acompanhar o desempenho da análise sintática.

Entre os aspectos monitorados encontram-se:

* tempo de parsing;
* consumo de memória;
* desempenho em projetos de grande porte;
* escalabilidade da construção da AST.

Esses benchmarks auxiliam a evolução da implementação sem alterar seus contratos funcionais.

---

## 17.6 Determinismo

Todos os testes da análise sintática devem pressupor comportamento determinístico.

Para uma mesma sequência de tokens, utilizando a mesma versão do compilador e as mesmas opções de compilação, a AST produzida e os diagnósticos emitidos deverão ser funcionalmente equivalentes.

---

# 18. Considerações Finais

A análise sintática constitui a segunda etapa efetiva do processo de compilação da linguagem Capi.

Sua responsabilidade consiste em transformar a sequência de tokens produzida pelo Lexer em uma Árvore Sintática Abstrata (AST), estabelecendo a representação estrutural do programa sobre a qual serão realizadas todas as análises semânticas posteriores.

---

## 18.1 Síntese da Arquitetura

O Parser é caracterizado pelos seguintes princípios:

* responsabilidade única;
* comportamento determinístico;
* independência das fases semânticas;
* construção da AST como interface formal;
* suporte à emissão de diagnósticos precisos;
* preservação da rastreabilidade com o código-fonte.

Esses princípios garantem simplicidade, previsibilidade e facilidade de manutenção.

---

## 18.2 Princípios Preservados

Toda implementação compatível com esta especificação deverá preservar:

* equivalência funcional entre implementações;
* separação entre análise sintática e análise semântica;
* rastreabilidade entre AST e código-fonte;
* independência em relação às fases posteriores;
* conformidade com a gramática definida pela linguagem.

Esses princípios asseguram a interoperabilidade entre diferentes implementações do compilador.

---

## 18.3 Relação com o Documento Seguinte

Este documento define apenas a etapa responsável pela construção da representação sintática do programa.

O documento seguinte, **16 — Representação Semântica (HIR)**, especifica como a AST produzida pelo Parser é transformada em uma representação semântica de alto nível, incorporando informações necessárias para as fases de Resolução de Nomes, Inferência de Tipos e Verificação Semântica.

Dessa forma, a HIR estabelece a transição entre a estrutura sintática do programa e sua representação semântica.

---

## 18.4 Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência da análise sintática do compilador oficial da Capi.

Outras implementações poderão adotar estratégias diferentes para reconhecer a gramática da linguagem e construir a AST, desde que preservem integralmente:

* os contratos definidos neste documento;
* a interface formal baseada na AST;
* os diagnósticos esperados;
* o comportamento determinístico da análise sintática;
* a compatibilidade com a HIR e as fases posteriores do compilador.

Dessa forma, este documento conclui a especificação da análise sintática e estabelece a base sobre a qual a HIR inicia a construção da representação semântica do programa.

