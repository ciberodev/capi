# 14 — Lexer e Tokens

## Especificação de Implementação da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a especificação de implementação da análise léxica (*Lexical Analysis*) do compilador oficial da linguagem Capi.

Seu objetivo é estabelecer como o código-fonte é transformado em uma sequência ordenada de tokens, definindo os princípios arquiteturais, os artefatos produzidos e as responsabilidades da primeira fase do processo de compilação.

Este documento não define a sintaxe da linguagem, mas sim o mecanismo responsável por reconhecer seus elementos léxicos de forma determinística e independente das etapas posteriores do compilador.

---

## 1.2 Escopo

Este documento especifica:

* o papel do Lexer no pipeline de compilação;
* a estrutura dos tokens produzidos;
* o reconhecimento das categorias léxicas da linguagem;
* o tratamento de comentários e espaços em branco;
* o gerenciamento da localização dos tokens no código-fonte;
* a interface conceitual entre o Lexer e o Parser;
* os princípios arquiteturais da implementação da análise léxica.

Não fazem parte deste documento:

* a gramática da linguagem;
* a construção da AST;
* a resolução de ambiguidades sintáticas;
* a resolução de nomes;
* a inferência de tipos;
* a verificação semântica.

Esses assuntos são definidos pelos documentos subsequentes desta especificação de implementação.

---

## 1.3 Relação com os Documentos Anteriores

Este documento complementa diretamente a especificação da linguagem e a arquitetura do compilador.

Em especial, sua implementação deve preservar integralmente as garantias estabelecidas por:

| Documento    | Responsabilidade          |
| ------------ | ------------------------- |
| Documento 04 | Sintaxe da Linguagem      |
| Documento 06 | Arquitetura do Compilador |
| Documento 13 | Estrutura do Compilador   |

As regras sintáticas definidas pelo Documento 04 determinam quais elementos léxicos podem existir na linguagem, enquanto este documento especifica como esses elementos são reconhecidos durante a compilação.

---

## 1.4 Filosofia da Implementação

O Lexer possui responsabilidade única: transformar uma sequência de caracteres em uma sequência de tokens.

Ele não interpreta o significado dos elementos reconhecidos, não valida regras sintáticas e não realiza qualquer análise semântica do programa.

Sua implementação deve ser:

* determinística;
* previsível;
* independente das fases posteriores do compilador;
* eficiente para processamento de grandes arquivos;
* capaz de produzir diagnósticos precisos para erros léxicos.

Essa separação de responsabilidades reduz o acoplamento entre as fases do compilador e facilita tanto a implementação quanto os testes da análise léxica.

---

## 1.5 Separação entre Especificação e Implementação

Este documento define o comportamento esperado do Lexer, mas não impõe uma estratégia específica de implementação.

Uma implementação poderá utilizar, por exemplo:

* máquinas de estados finitos;
* analisadores gerados automaticamente;
* reconhecimento manual por código;
* tabelas de transição;
* outras técnicas equivalentes.

Independentemente da estratégia adotada, todas as implementações deverão produzir exatamente a mesma sequência de tokens para um mesmo código-fonte válido e emitir diagnósticos compatíveis para entradas inválidas.

---

# 2. Papel do Lexer

O Lexer constitui a primeira fase efetiva do pipeline de compilação.

Sua função é analisar o fluxo de caracteres do código-fonte e identificar as unidades léxicas que servirão como entrada para o Parser.

Ao término dessa etapa, todo o código-fonte passa a ser representado como uma sequência estruturada de tokens.

---

## 2.1 Entrada

A entrada do Lexer consiste em um ou mais arquivos-fonte contendo o programa escrito em Capi.

Esses arquivos são disponibilizados pelo componente responsável pelo gerenciamento de código-fonte, conforme definido no Documento 13 — Estrutura do Compilador.

O Lexer opera exclusivamente sobre caracteres, sem qualquer conhecimento prévio sobre a estrutura sintática do programa.

---

## 2.2 Saída

A saída do Lexer é uma sequência ordenada de tokens.

Cada token representa uma unidade léxica reconhecida no código-fonte, contendo informações suficientes para permitir que o Parser realize a análise sintática.

Além da sequência de tokens, o Lexer poderá produzir diagnósticos referentes a erros léxicos identificados durante o processamento.

---

## 2.3 Responsabilidades

Compete ao Lexer:

* reconhecer os elementos léxicos da linguagem;
* produzir tokens válidos para o Parser;
* identificar palavras reservadas;
* reconhecer literais;
* reconhecer operadores e delimitadores;
* registrar a localização de cada token no código-fonte;
* detectar erros léxicos.

Cada uma dessas responsabilidades deve ser executada sem depender da análise sintática ou semântica do programa.

---

## 2.4 Limitações

Não compete ao Lexer:

* validar a gramática da linguagem;
* construir a AST;
* resolver ambiguidades sintáticas;
* identificar símbolos;
* inferir tipos;
* validar regras semânticas.

Sempre que uma decisão depender do contexto sintático ou semântico, ela pertence às fases posteriores do compilador.

---

## 2.5 Determinismo

O Lexer deve produzir resultados determinísticos.

Dado um mesmo código-fonte, utilizando a mesma versão do compilador e as mesmas opções de compilação, a sequência de tokens produzida deverá ser semanticamente idêntica.

Esse princípio garante previsibilidade, reprodutibilidade e facilita a validação da implementação.

---

## 2.6 Independência Arquitetural

O Lexer deve permanecer completamente independente das demais fases do compilador.

Ele não deve depender:

* do Parser;
* da AST;
* da HIR;
* da IR;
* do sistema de tipos;
* da verificação semântica.

Sua única responsabilidade consiste em transformar caracteres em tokens.

---

# 3. Fluxo da Análise Léxica

A análise léxica representa a primeira transformação realizada pelo compilador sobre o código-fonte.

Seu funcionamento consiste em percorrer continuamente a sequência de caracteres, identificar padrões válidos da linguagem e produzir os tokens correspondentes para consumo pelo Parser.

---

## 3.1 Visão Geral

Conceitualmente, o fluxo da análise léxica pode ser representado da seguinte forma:

```text
Código Fonte
      │
      ▼
Leitura de Caracteres
      │
      ▼
Reconhecimento Léxico
      │
      ▼
Sequência de Tokens
      │
      ▼
Parser
```

Cada caractere do código-fonte é processado exatamente conforme as regras léxicas definidas para a linguagem.

---

## 3.2 Leitura do Código-Fonte

O Lexer percorre o código-fonte de forma sequencial.

Durante esse processo, ele identifica padrões que correspondem às categorias léxicas definidas pela linguagem, consumindo apenas os caracteres necessários para formar cada token.

O mecanismo utilizado para essa leitura pertence exclusivamente à implementação.

---

## 3.3 Reconhecimento Léxico

O reconhecimento léxico consiste na identificação da maior sequência válida de caracteres correspondente a um token.

Esse processo deve seguir as regras definidas pela linguagem para cada categoria léxica, garantindo que um mesmo trecho do código-fonte seja sempre reconhecido da mesma forma.

As regras específicas para identificadores, literais, operadores e demais categorias são apresentadas nos capítulos seguintes.

---

## 3.4 Produção dos Tokens

Sempre que um elemento léxico válido for reconhecido, o Lexer deverá produzir um token contendo todas as informações necessárias para o Parser.

Cada token representa uma unidade indivisível da análise sintática e constitui a interface formal entre o Lexer e o Parser.

---

## 3.5 Comunicação com o Parser

A comunicação entre o Lexer e o Parser ocorre exclusivamente por meio da sequência de tokens produzida durante a análise léxica.

O Parser não deve acessar diretamente o código-fonte nem depender dos mecanismos internos utilizados pelo Lexer para reconhecer os tokens.

Essa separação preserva o baixo acoplamento entre as fases do compilador.

---

## 3.6 Princípio da Responsabilidade Única

A análise léxica encerra-se no momento em que todos os elementos do código-fonte foram convertidos em tokens.

Qualquer interpretação estrutural, sintática ou semântica desses tokens pertence exclusivamente às fases posteriores do compilador.

Esse princípio preserva a modularidade do pipeline e garante que o Lexer permaneça um componente simples, previsível e facilmente testável.

---

# 4. Estrutura do Token

Os tokens constituem a interface formal entre o Lexer e o Parser.

Cada token representa uma unidade léxica reconhecida no código-fonte e contém todas as informações necessárias para que as fases posteriores possam interpretar corretamente a estrutura do programa.

A estrutura exata utilizada para representar um token pertence à implementação, desde que todas as informações definidas neste documento estejam disponíveis.

---

## 4.1 Objetivo

Um token deve representar completamente um elemento léxico da linguagem.

Além de identificar sua categoria, o token deve preservar informações suficientes para:

* análise sintática;
* emissão de diagnósticos;
* rastreabilidade até o código-fonte;
* depuração do compilador.

---

## 4.2 Componentes Conceituais

Conceitualmente, um token é composto pelos seguintes elementos:

| Campo     | Finalidade                              |
| --------- | --------------------------------------- |
| Kind      | Categoria léxica do token               |
| Lexema    | Texto correspondente no código-fonte    |
| Span      | Localização do token no arquivo         |
| Valor     | Valor normalizado, quando aplicável     |
| Metadados | Informações auxiliares da implementação |

Essa divisão é conceitual e não impõe uma estrutura física específica.

---

## 4.3 Kind

O **Kind** identifica a categoria léxica do token.

Exemplos incluem:

* identificador;
* palavra reservada;
* literal inteiro;
* literal decimal;
* operador;
* delimitador;
* fim de arquivo.

O conjunto completo de categorias é definido na Seção 5.

---

## 4.4 Lexema

O **Lexema** corresponde exatamente à sequência de caracteres reconhecida no código-fonte.

Exemplos:

```text
let

contador

12345

+

{
```

O Lexer deve preservar o lexema exatamente como aparece no código-fonte.

---

## 4.5 Span

O **Span** identifica a localização completa do token no arquivo de origem.

Ele permite:

* emissão de diagnósticos;
* localização precisa de erros;
* integração com IDEs;
* geração de informações de depuração.

A composição detalhada do Span é definida na Seção 6.

---

## 4.6 Valor

Algumas categorias de tokens possuem um valor lógico além do texto originalmente escrito.

Por exemplo:

```text
123
```

possui:

* Lexema: `"123"`
* Valor: inteiro de valor 123

Da mesma forma:

```text
3.14
```

possui:

* Lexema: `"3.14"`
* Valor: número decimal correspondente.

A forma utilizada para representar esse valor pertence à implementação.

---

# 5. Categorias de Tokens

Todo token reconhecido pelo Lexer pertence exatamente a uma categoria léxica.

Essas categorias constituem a linguagem utilizada pelo Parser para realizar a análise sintática.

---

## 5.1 Classificação Geral

As categorias de tokens são agrupadas conforme sua função na linguagem.

Conceitualmente, incluem:

* palavras reservadas;
* identificadores;
* literais;
* operadores;
* delimitadores;
* comentários;
* fim de arquivo.

Novas categorias poderão ser incorporadas em versões futuras da linguagem.

---

## 5.2 Palavras Reservadas

Palavras reservadas representam elementos definidos pela própria linguagem.

Seu reconhecimento deve ocorrer antes da classificação como identificador.

Exemplos incluem palavras utilizadas para:

* declaração;
* controle de fluxo;
* definição de tipos;
* modificadores;
* operadores textuais.

A lista completa das palavras reservadas é definida pelo Documento 04 — Sintaxe da Linguagem.

---

## 5.3 Identificadores

Identificadores representam nomes definidos pelo usuário.

Exemplos:

* variáveis;
* funções;
* tipos;
* módulos;
* parâmetros;
* constantes.

As regras para reconhecimento dos identificadores são apresentadas na Seção 7.

---

## 5.4 Literais

Literais representam valores escritos diretamente no código-fonte.

Entre eles incluem-se:

* inteiros;
* números em ponto flutuante;
* caracteres;
* cadeias de caracteres;
* valores booleanos;
* outros literais definidos pela linguagem.

O reconhecimento de cada categoria é tratado na Seção 8.

---

## 5.5 Operadores e Delimitadores

Operadores representam operações da linguagem.

Delimitadores representam elementos estruturais utilizados pela sintaxe.

Exemplos incluem:

* operadores aritméticos;
* operadores lógicos;
* operadores relacionais;
* parênteses;
* colchetes;
* chaves;
* separadores.

Seu reconhecimento é detalhado na Seção 9.

---

## 5.6 Fim de Arquivo

Ao término da leitura do código-fonte, o Lexer deve produzir um token especial representando o fim da entrada (*End Of File* — EOF).

Esse token permite que o Parser determine de forma inequívoca o encerramento da sequência de tokens.

---

# 6. Localização no Código-Fonte

Todo token produzido pelo Lexer deve manter informações suficientes para localizar precisamente sua origem no código-fonte.

Essas informações são fundamentais para emissão de diagnósticos, geração de informações de depuração e integração com ferramentas de desenvolvimento.

---

## 6.1 Objetivo

A localização dos tokens permite estabelecer a correspondência entre os artefatos internos do compilador e o código escrito pelo desenvolvedor.

Essa rastreabilidade deve ser preservada durante todas as fases do Frontend.

---

## 6.2 Arquivo de Origem

Cada token deve estar associado ao arquivo-fonte do qual foi extraído.

Essa associação torna possível identificar corretamente a origem dos elementos léxicos em projetos compostos por múltiplos módulos.

---

## 6.3 Linha e Coluna

O Lexer deve registrar, no mínimo:

* linha inicial;
* coluna inicial.

A implementação poderá registrar também:

* linha final;
* coluna final.

Essas informações destinam-se principalmente à emissão de diagnósticos.

---

## 6.4 Offset

Além da posição em linha e coluna, a implementação poderá registrar o deslocamento absoluto (*offset*) do token dentro do arquivo.

Esse valor facilita operações como:

* navegação rápida;
* indexação;
* integração com editores;
* recuperação eficiente do texto original.

---

## 6.5 Span

O **Span** representa o intervalo completo ocupado pelo token no código-fonte.

Conceitualmente, ele associa:

* arquivo;
* posição inicial;
* posição final.

Todas as fases posteriores do compilador deverão utilizar o Span como referência oficial para localização dos elementos do programa.

---

## 6.6 Preservação da Localização

As informações de localização produzidas pelo Lexer devem permanecer disponíveis durante todo o Frontend.

Sempre que possível, as representações posteriores (AST, HIR e diagnósticos) devem manter referências ao Span original dos tokens que lhes deram origem.

Essa preservação garante diagnósticos precisos, facilita a depuração do compilador e mantém a rastreabilidade entre o código-fonte e as representações internas do programa.

---

# 7. Identificadores

Os identificadores representam os nomes definidos pelo desenvolvedor para declarar e referenciar elementos do programa.

O papel do Lexer consiste exclusivamente em reconhecer identificadores válidos segundo as regras léxicas da linguagem, sem interpretar seu significado ou verificar sua existência.

A associação entre um identificador e o elemento que ele representa é responsabilidade da fase de Resolução de Nomes, definida no Documento 17 — Resolução de Nomes e Escopos.

---

## 7.1 Objetivo

O reconhecimento de identificadores possui os seguintes objetivos:

* distinguir identificadores de palavras reservadas;
* preservar o texto original do identificador;
* produzir tokens apropriados para o Parser;
* registrar corretamente sua localização no código-fonte.

---

## 7.2 Regras de Formação

Um identificador deve obedecer às regras léxicas definidas pela linguagem.

De forma geral, poderá ser composto por:

* letras;
* dígitos;
* caractere de sublinhado (`_`);
* demais caracteres permitidos pela especificação da linguagem.

As regras completas de formação pertencem ao Documento 04 — Sintaxe da Linguagem.

---

## 7.3 Sensibilidade a Maiúsculas e Minúsculas

O reconhecimento de identificadores deve preservar exatamente a sequência de caracteres presente no código-fonte.

Quando a linguagem distinguir letras maiúsculas e minúsculas (*case-sensitive*), essa distinção deverá ser preservada pelo Lexer.

Qualquer comparação entre identificadores pertence às fases posteriores do compilador.

---

## 7.4 Palavras Reservadas

Após reconhecer uma sequência de caracteres compatível com um identificador, o Lexer deve verificar se ela corresponde a uma palavra reservada da linguagem.

Caso corresponda, deverá produzir o token específico da palavra reservada.

Caso contrário, deverá produzir um token de identificador.

Essa decisão é puramente léxica e independe do contexto do programa.

---

## 7.5 Internamento

A implementação poderá utilizar internamento (*string interning*) para armazenar identificadores.

Essa estratégia reduz:

* duplicação de memória;
* custo de comparação entre identificadores;
* consumo de memória em projetos de grande porte.

O mecanismo utilizado pertence exclusivamente à implementação.

---

## 7.6 Independência Semântica

O Lexer não deve interpretar o significado de um identificador.

Por exemplo, ele não verifica:

* se o identificador foi declarado;
* qual símbolo ele representa;
* seu tipo;
* seu escopo;
* sua visibilidade.

Essas verificações pertencem às fases posteriores do Frontend.

---

# 8. Literais

Os literais representam valores escritos diretamente no código-fonte.

O Lexer é responsável apenas por reconhecer essas construções e produzir os respectivos tokens.

A interpretação semântica desses valores pertence às fases posteriores do compilador.

---

## 8.1 Objetivo

O reconhecimento de literais deve:

* identificar corretamente cada categoria de literal;
* preservar o texto original;
* produzir o token correspondente;
* registrar sua localização no código-fonte.

---

## 8.2 Categorias

Entre as categorias de literais reconhecidas pela linguagem incluem-se:

* inteiros;
* números em ponto flutuante;
* caracteres;
* cadeias de caracteres;
* valores booleanos;
* outras categorias previstas pela especificação da linguagem.

A linguagem não possui literais nulos (null ou nil). A ausência de valor é representada exclusivamente por meio do tipo Optional<T>, conforme definido no Documento 02 — Sistema de Tipos.

---

## 8.3 Valor Léxico

Além do lexema original, a implementação poderá associar ao token uma representação normalizada do valor reconhecido.

Por exemplo:

* conversão de sequências de escape;
* normalização de bases numéricas;
* representação interna de caracteres Unicode.

Essas representações auxiliam as fases posteriores, mas não alteram o texto original do código-fonte.

---

## 8.4 Sequências de Escape

Quando a linguagem suportar sequências de escape em caracteres ou cadeias de caracteres, o Lexer deverá reconhecê-las conforme a especificação da linguagem.

A validação detalhada dessas sequências poderá ocorrer durante a análise léxica ou em fases posteriores, conforme a estratégia adotada pela implementação.

---

## 8.5 Literais Inválidos

Sempre que um literal não puder ser reconhecido conforme as regras da linguagem, o Lexer deverá emitir um diagnóstico apropriado.

Exemplos incluem:

* caracteres inválidos;
* cadeias de caracteres não encerradas;
* sequências de escape inválidas;
* formatos numéricos incorretos.

Sempre que possível, o Lexer deverá recuperar-se do erro para continuar produzindo tokens.

---

## 8.6 Independência da Representação Interna

A forma utilizada para representar internamente os valores reconhecidos pertence exclusivamente à implementação.

Independentemente da representação adotada, o comportamento observado pelas fases posteriores deverá ser equivalente ao definido pela especificação da linguagem.

---

# 9. Operadores e Delimitadores

Operadores e delimitadores representam elementos estruturais fundamentais da linguagem.

Seu reconhecimento é responsabilidade exclusiva do Lexer, que deve identificar corretamente cada símbolo e produzir o token correspondente.

---

## 9.1 Objetivo

O reconhecimento desses elementos possui os seguintes objetivos:

* identificar operadores;
* identificar delimitadores;
* distinguir operadores compostos;
* produzir tokens inequívocos para o Parser.

---

## 9.2 Operadores

Os operadores representam operações definidas pela linguagem.

Entre eles podem existir:

* operadores aritméticos;
* operadores relacionais;
* operadores lógicos;
* operadores de atribuição;
* operadores bit a bit;
* operadores específicos da linguagem.

A semântica desses operadores pertence ao Documento 05 — Semântica Operacional.

---

## 9.3 Delimitadores

Os delimitadores organizam estruturalmente o programa.

Entre eles encontram-se elementos como:

* parênteses;
* colchetes;
* chaves;
* vírgulas;
* ponto e vírgula;
* dois-pontos;
* separadores definidos pela linguagem.

Seu reconhecimento depende exclusivamente da análise léxica.

---

## 9.4 Maior Correspondência

Sempre que múltiplos operadores compartilharem um prefixo comum, o Lexer deverá reconhecer a maior sequência válida de caracteres.

Por exemplo, se a linguagem definir simultaneamente:

```text id="s5xj2m"
>

>=

>>

>>=
```

o Lexer deverá produzir sempre o token correspondente à maior sequência válida iniciada na posição atual.

Esse princípio elimina ambiguidades durante a análise léxica.

Esse comportamento corresponde ao princípio conhecido como Maximal Munch, amplamente utilizado em analisadores léxicos para resolver ambiguidades entre operadores que compartilham prefixos comuns.

---

## 9.5 Operadores Compostos

Operadores formados por múltiplos caracteres devem ser reconhecidos como um único token.

A divisão desses operadores em múltiplos tokens somente poderá ocorrer quando não existir um operador composto válido correspondente.

---

## 9.6 Independência Sintática

O Lexer reconhece operadores e delimitadores exclusivamente com base em sua forma textual.

Ele não interpreta:

* precedência;
* associatividade;
* contexto sintático;
* contexto semântico.

Essas responsabilidades pertencem ao Parser e às fases posteriores do compilador.

---

# 10. Comentários

Os comentários permitem a inclusão de informações destinadas exclusivamente aos desenvolvedores, sem produzir qualquer efeito sobre o comportamento do programa.

O Lexer é responsável por reconhecer comentários conforme as regras da linguagem e decidir como eles serão tratados durante a análise léxica.

---

## 10.1 Objetivo

O reconhecimento de comentários possui os seguintes objetivos:

* identificar corretamente cada comentário;
* impedir que seu conteúdo interfira na análise léxica;
* preservar a correta localização dos elementos subsequentes;
* permitir a emissão de diagnósticos quando necessário.

---

## 10.2 Tipos de Comentários

A linguagem poderá definir diferentes categorias de comentários, tais como:

* comentários de linha;
* comentários de bloco;
* comentários destinados à documentação.

As formas sintáticas correspondentes são definidas pelo Documento 04 — Sintaxe da Linguagem.

---

## 10.3 Tratamento

Por padrão, comentários não produzem tokens destinados ao Parser.

Após seu reconhecimento, o Lexer poderá simplesmente descartá-los, preservando apenas as informações necessárias para manter a localização correta dos demais tokens.

Implementações que suportem ferramentas de documentação poderão preservar informações adicionais associadas aos comentários.

---

## 10.4 Comentários Aninhados

Quando a linguagem permitir comentários de bloco aninhados, o Lexer deverá reconhecer corretamente seus níveis de aninhamento.

Caso a linguagem não suporte esse comportamento, comentários aninhados deverão produzir os diagnósticos apropriados.

O comportamento esperado é determinado pela especificação sintática da linguagem.

---

## 10.5 Comentários Inválidos

Sempre que um comentário não puder ser reconhecido corretamente, o Lexer deverá emitir um diagnóstico apropriado.

Exemplos incluem:

* comentário de bloco não encerrado;
* sequência de encerramento inesperada;
* construção inválida segundo a especificação da linguagem.

Sempre que possível, a análise deverá prosseguir após a recuperação do erro.

---

## 10.6 Independência Semântica

O conteúdo de um comentário não participa da compilação do programa.

Consequentemente, o Lexer não interpreta seu conteúdo nem realiza qualquer validação semântica sobre ele, exceto quando necessário para reconhecer corretamente seu término.

---

# 11. Espaços em Branco

Os espaços em branco organizam visualmente o código-fonte, mas, salvo quando especificado pela linguagem, não possuem significado sintático próprio.

O Lexer é responsável por reconhecê-los e tratá-los de forma consistente durante a análise léxica.

---

## 11.1 Objetivo

O tratamento de espaços em branco possui os seguintes objetivos:

* separar elementos léxicos adjacentes;
* preservar a correta localização dos tokens;
* facilitar a emissão de diagnósticos;
* garantir comportamento determinístico da análise léxica.

---

## 11.2 Categorias

Entre os caracteres considerados espaços em branco incluem-se, quando aplicável:

* espaço;
* tabulação;
* quebra de linha;
* retorno de carro;
* demais caracteres classificados como espaço pela especificação da linguagem.

A classificação exata pertence ao Documento 04 — Sintaxe da Linguagem.

---

## 11.3 Separação de Tokens

Os espaços em branco podem ser utilizados para separar elementos léxicos consecutivos.

Quando sua presença não alterar a interpretação do programa, eles não produzem tokens destinados ao Parser.

O Lexer deve simplesmente consumi-los e continuar a análise.

---

## 11.4 Quebras de Linha

As quebras de linha devem ser registradas para atualização das informações de localização dos tokens.

Quando a linguagem atribuir significado sintático às quebras de linha, esse comportamento será definido pela especificação sintática, cabendo ao Lexer apenas seu reconhecimento.

---

## 11.5 Normalização

A implementação poderá normalizar diferentes representações de quebra de linha para uma forma interna única.

Essa normalização não deverá alterar:

* a localização dos tokens;
* os diagnósticos emitidos;
* o comportamento observável da compilação.

---

## 11.6 Independência Sintática

O tratamento de espaços em branco limita-se ao reconhecimento dos caracteres correspondentes.

A decisão sobre seu eventual significado sintático pertence exclusivamente às fases posteriores do compilador.

---

# 12. Codificação

O Lexer opera sobre arquivos contendo texto codificado.

A correta interpretação dessa codificação é essencial para garantir que o código-fonte seja analisado de forma consistente em diferentes plataformas.

---

## 12.1 Objetivo

O tratamento da codificação possui os seguintes objetivos:

* interpretar corretamente os caracteres do código-fonte;
* preservar a portabilidade entre plataformas;
* detectar arquivos inválidos;
* garantir consistência durante toda a compilação.

---

## 12.2 Codificação Suportada

A codificação oficial da linguagem Capi é **UTF-8**, conforme estabelecido pelo Documento 04 — Sintaxe da Linguagem.

Todo arquivo-fonte deverá ser interpretado utilizando essa codificação.

Implementações poderão oferecer suporte adicional a outras codificações apenas como mecanismo de conversão prévia para UTF-8.

---

## 12.3 Byte Order Mark (BOM)

Quando presente, o marcador de ordem de bytes (*Byte Order Mark* — BOM) deverá ser tratado de acordo com a política definida pela implementação.

Independentemente da estratégia adotada, sua presença não poderá alterar o comportamento léxico do programa.

---

## 12.4 Caracteres Inválidos

Sempre que a sequência de bytes não puder ser interpretada como UTF-8 válido, o Lexer deverá emitir um diagnóstico apropriado.

Sempre que possível, a análise deverá continuar após a recuperação do erro.

---

## 12.5 Caracteres Unicode

O Lexer deverá reconhecer corretamente todos os caracteres Unicode permitidos pela especificação da linguagem.

A classificação desses caracteres como letras, dígitos ou outros elementos léxicos deverá seguir as regras estabelecidas pelo Documento 04.

---

## 12.6 Independência da Implementação

Os mecanismos utilizados para leitura e decodificação dos arquivos pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todos os compiladores compatíveis deverão interpretar arquivos UTF-8 válidos de forma equivalente e produzir exatamente a mesma sequência de tokens para um mesmo código-fonte.

---

# 13. Interface do Lexer

A interface do Lexer define o contrato de comunicação entre a análise léxica e as demais fases do compilador.

Seu objetivo é fornecer uma sequência ordenada de tokens ao Parser, ocultando completamente os detalhes internos utilizados para reconhecer os elementos léxicos do código-fonte.

A forma concreta dessa interface pertence à implementação, desde que preserve os princípios estabelecidos neste documento.

---

## 13.1 Objetivo

A interface do Lexer deve:

* disponibilizar os tokens produzidos durante a análise léxica;
* permitir consumo determinístico pelo Parser;
* ocultar os mecanismos internos de reconhecimento;
* preservar a independência entre as fases do compilador.

---

## 13.2 Entrada

O Lexer recebe como entrada:

* o conteúdo de um arquivo-fonte;
* informações de localização fornecidas pelo Source Manager;
* o contexto necessário para emissão de diagnósticos.

O Source Manager, definido no Documento 13 — Estrutura do Compilador, é responsável por fornecer ao Lexer o conteúdo dos arquivos-fonte, bem como as informações necessárias para construção dos respectivos Spans.

O fornecimento dessas informações é responsabilidade da infraestrutura do compilador definida no Documento 13.

---

## 13.3 Saída

A saída do Lexer consiste em uma sequência ordenada de tokens.

Cada token deverá conter todas as informações necessárias para permitir a análise sintática, incluindo:

* categoria léxica;
* localização no código-fonte;
* informações auxiliares definidas pela implementação.

A sequência termina obrigatoriamente com o token de fim de arquivo (EOF).

---

## 13.4 Modelo de Consumo

A implementação poderá disponibilizar os tokens utilizando diferentes modelos de consumo, tais como:

* sequência previamente construída;
* iterador;
* gerador (*generator*);
* fluxo (*stream*);
* outras estratégias equivalentes.

A escolha do modelo pertence exclusivamente à implementação.

---

## 13.5 Independência do Parser

O Parser não deve depender da implementação interna do Lexer.

Sua interação limita-se ao consumo da sequência de tokens disponibilizada pela interface pública da análise léxica.

Essa separação preserva o baixo acoplamento entre os componentes do Frontend.

---

## 13.6 Contrato da Interface

Independentemente da implementação adotada, toda interface do Lexer deve garantir:

* ordem determinística dos tokens;
* preservação da localização de cada token;
* emissão consistente de diagnósticos;
* produção obrigatória do token EOF;
* comportamento equivalente para uma mesma entrada.

---

# 14. Estratégia de Implementação

Este documento define apenas o comportamento esperado da análise léxica.

A estratégia utilizada para implementar esse comportamento pertence exclusivamente ao compilador.

Diferentes implementações podem empregar técnicas distintas, desde que produzam exatamente os mesmos resultados observáveis.

---

## 14.1 Objetivo

A implementação do Lexer deve buscar:

* simplicidade;
* eficiência;
* previsibilidade;
* facilidade de manutenção;
* facilidade de testes.

A escolha da técnica utilizada deve considerar esses objetivos.

---

## 14.2 Técnicas de Implementação

Entre as estratégias normalmente utilizadas para implementar analisadores léxicos encontram-se:

* máquinas de estados finitos (*Finite State Machines*);
* autômatos determinísticos (*Deterministic Finite Automata*);
* tabelas de transição;
* reconhecimento manual por código;
* geradores automáticos de Lexer (como ferramentas semelhantes a Lex/Flex);
* outras técnicas equivalentes.

Todas essas abordagens são compatíveis com esta especificação.

---

## 14.3 Critérios de Escolha

A escolha da estratégia de implementação poderá considerar fatores como:

* desempenho;
* consumo de memória;
* simplicidade do código;
* facilidade de evolução;
* integração com o restante do compilador.

Esses critérios pertencem exclusivamente ao projeto de implementação.

---

## 14.4 Independência Arquitetural

Independentemente da estratégia utilizada, o comportamento do Lexer deve permanecer completamente independente:

* do Parser;
* da AST;
* da HIR;
* da IR;
* da arquitetura de destino.

Essa independência constitui um dos princípios fundamentais da arquitetura do compilador.

---

## 14.5 Equivalência Funcional

Implementações distintas devem produzir exatamente a mesma sequência de tokens para um mesmo código-fonte válido.

Da mesma forma, entradas inválidas deverão resultar em diagnósticos funcionalmente equivalentes.

Esse princípio garante interoperabilidade entre diferentes implementações da linguagem.

---

## 14.6 Evolução

A estratégia de implementação poderá evoluir ao longo do tempo.

Mudanças internas são permitidas desde que preservem:

* a interface pública do Lexer;
* os contratos definidos neste documento;
* o comportamento observável da análise léxica.

---

# 15. Integração com o Parser

O Parser constitui o consumidor direto da sequência de tokens produzida pelo Lexer.

A comunicação entre ambos representa a fronteira entre a análise léxica e a análise sintática do compilador.

Essa interface deve permanecer simples, determinística e independente das implementações específicas de cada componente.

---

## 15.1 Objetivo

A integração entre Lexer e Parser deve permitir que a análise sintática seja realizada exclusivamente a partir da sequência de tokens produzida durante a análise léxica.

Nenhuma outra informação proveniente do código-fonte deve ser necessária ao Parser.

---

## 15.2 Fluxo de Comunicação

Conceitualmente, a comunicação entre ambos pode ser representada da seguinte forma:

```text id="a1q7wm"
Código Fonte
      │
      ▼
Lexer
      │
      ▼
Sequência de Tokens
      │
      ▼
Parser
```

O Parser recebe apenas os tokens produzidos pelo Lexer e constrói a representação sintática correspondente.

---

## 15.3 Lookahead

A implementação poderá permitir que o Parser consulte antecipadamente (*lookahead*) um ou mais tokens antes de consumi-los.

Esse mecanismo auxilia a resolução de determinadas construções sintáticas, sem alterar a responsabilidade do Lexer.

A quantidade de tokens observados antecipadamente pertence exclusivamente à implementação do Parser.

---

## 15.4 Consumo dos Tokens

Cada token deverá ser consumido pelo Parser de maneira ordenada.

A implementação poderá oferecer mecanismos para:

* consultar o próximo token;
* avançar na sequência;
* verificar o fim da entrada;
* recuperar a posição corrente durante estratégias de recuperação de erros.

Esses mecanismos fazem parte da interface entre as duas fases.

---

## 15.5 Token de Fim de Arquivo

O token EOF marca formalmente o encerramento da sequência produzida pelo Lexer.

O Parser deve utilizá-lo como única indicação oficial de que não existem mais tokens disponíveis.

A ausência do token EOF constitui uma violação da interface definida neste documento.

---

## 15.6 Separação de Responsabilidades

A integração entre Lexer e Parser não altera as responsabilidades individuais de cada componente.

O Lexer permanece responsável exclusivamente pelo reconhecimento dos elementos léxicos.

O Parser permanece responsável exclusivamente pela interpretação sintática da sequência de tokens.

Essa separação preserva a modularidade do Frontend e reduz o acoplamento entre suas fases.

---

# 16. Diagnósticos

O Lexer é responsável por identificar e reportar todos os erros relacionados à análise léxica do código-fonte.

Sempre que possível, esses erros devem ser apresentados de forma clara e precisa, permitindo que o desenvolvedor identifique rapidamente sua causa e localização.

Os mecanismos gerais de emissão de diagnósticos são definidos pelo Documento 23 — Diagnósticos e Recuperação de Erros. Este capítulo especifica apenas os aspectos particulares da análise léxica.

---

## 16.1 Objetivo

O sistema de diagnósticos da análise léxica possui os seguintes objetivos:

* identificar erros durante o reconhecimento dos elementos léxicos;
* localizar precisamente a origem de cada erro;
* permitir a continuidade da análise sempre que possível;
* fornecer informações suficientes para facilitar a correção do código.

---

## 16.2 Categorias de Erros

Entre os erros que podem ser detectados pelo Lexer incluem-se:

* caracteres inválidos;
* sequências UTF-8 inválidas;
* identificadores malformados;
* literais inválidos;
* cadeias de caracteres não encerradas;
* caracteres não encerrados;
* comentários não encerrados;
* sequências de escape inválidas.

Novas categorias poderão ser incorporadas em versões futuras da linguagem.

---

## 16.3 Localização

Todo diagnóstico emitido pelo Lexer deverá estar associado ao Span correspondente no código-fonte.

Sempre que possível, o diagnóstico deverá identificar:

* arquivo;
* linha;
* coluna;
* intervalo de caracteres relacionado ao erro.

Essa informação permite que ferramentas de desenvolvimento apresentem diagnósticos precisos ao usuário.

---

## 16.4 Recuperação

Sempre que um erro léxico puder ser recuperado com segurança, o Lexer deverá continuar sua execução.

O objetivo da recuperação é permitir que múltiplos erros sejam reportados em uma única compilação, reduzindo o número de ciclos de correção necessários.

A recuperação de erros léxicos pode envolver a identificação de um ponto seguro no código-fonte (por exemplo, o próximo caractere que inicia um token válido) para retomar a análise.

A estratégia exata utilizada para essa recuperação pertence exclusivamente à implementação.

---

## 16.5 Mecanismos Internos de Recuperação

Durante a recuperação de erros léxicos, a implementação poderá utilizar mecanismos internos para preservar a consistência da análise léxica e do pipeline de compilação.

Esses mecanismos pertencem exclusivamente à implementação e não fazem parte da interface formal definida por este documento.

---

## 16.6 Consistência

A ocorrência de um erro léxico não deve comprometer a consistência dos tokens produzidos posteriormente.

Após recuperar-se de um erro, o Lexer deverá retomar a análise a partir de uma posição válida do código-fonte.

Essa propriedade reduz a propagação de erros para o Parser.

---

## 16.7 Independência

O Lexer deve emitir apenas diagnósticos relacionados à análise léxica.

Erros de natureza sintática, semântica ou relacionados ao sistema de tipos pertencem exclusivamente às fases posteriores do compilador.

---

# 17. Testabilidade

A análise léxica constitui um dos componentes mais facilmente isoláveis do compilador.

Sua natureza determinística permite a construção de testes independentes, garantindo elevado nível de confiabilidade e facilitando a evolução da implementação.

A implementação deve manter uma suíte abrangente de testes automatizados, incluindo testes de regressão para cada erro léxico identificado e posteriormente corrigido. Esses testes garantem que defeitos anteriormente solucionados não sejam reintroduzidos em versões futuras do compilador.

---

## 17.1 Objetivo

Os testes do Lexer possuem os seguintes objetivos:

* validar o reconhecimento correto dos elementos léxicos;
* detectar regressões;
* verificar a estabilidade da interface com o Parser;
* garantir comportamento determinístico.

---

## 17.2 Testes Unitários

Cada categoria de token deverá possuir testes específicos.

Exemplos incluem:

* identificadores;
* palavras reservadas;
* literais;
* operadores;
* delimitadores;
* comentários;
* espaços em branco.

Esses testes verificam exclusivamente o comportamento da análise léxica.

---

## 17.3 Golden Tests

A implementação poderá utilizar *Golden Tests* para validar a sequência completa de tokens produzida a partir de arquivos de entrada previamente conhecidos.

Cada execução compara os tokens produzidos com uma saída considerada correta, permitindo detectar alterações inesperadas no comportamento do Lexer.

---

## 17.4 Testes de Diagnósticos

Além dos casos válidos, deverão existir testes destinados à validação dos diagnósticos emitidos pelo Lexer.

Esses testes verificam aspectos como:

* categoria do erro;
* localização;
* recuperação da análise;
* continuidade da produção de tokens.

---

## 17.5 Benchmarks

A implementação poderá incluir benchmarks destinados a acompanhar o desempenho da análise léxica.

Entre os aspectos monitorados encontram-se:

* tempo de processamento;
* quantidade de tokens por segundo;
* consumo de memória;
* desempenho em arquivos de grande porte.

Esses benchmarks auxiliam a evolução da implementação sem alterar seus contratos funcionais.

---

## 17.6 Determinismo

Todos os testes da análise léxica devem pressupor comportamento determinístico.

Para uma mesma entrada, utilizando a mesma versão do compilador e as mesmas opções de compilação, a sequência de tokens e os diagnósticos produzidos deverão ser funcionalmente equivalentes.

---

# 18. Considerações Finais

A análise léxica constitui a primeira etapa efetiva do processo de compilação da linguagem Capi.

Sua responsabilidade consiste em transformar o código-fonte em uma sequência ordenada de tokens, estabelecendo a base sobre a qual todas as fases posteriores do Frontend são construídas.

---

## 18.1 Síntese da Arquitetura

O Lexer é caracterizado pelos seguintes princípios:

* responsabilidade única;
* comportamento determinístico;
* independência das demais fases;
* produção de tokens como interface formal;
* suporte à emissão de diagnósticos precisos.

Esses princípios garantem simplicidade, previsibilidade e facilidade de manutenção.

---

## 18.2 Princípios Preservados

Toda implementação compatível com esta especificação deverá preservar:

* equivalência funcional entre implementações;
* separação entre análise léxica e análise sintática;
* rastreabilidade dos tokens ao código-fonte;
* independência em relação ao Parser;
* conformidade com a especificação sintática da linguagem.

Esses princípios asseguram a interoperabilidade entre diferentes implementações do compilador.

---

## 18.3 Relação com o Documento Seguinte

Este documento define apenas a etapa responsável pelo reconhecimento dos elementos léxicos da linguagem.

O documento seguinte, **15 — Parser e AST**, especifica como a sequência de tokens produzida pelo Lexer é utilizada para construir a Árvore Sintática Abstrata (AST), iniciando a análise sintática do programa.

Dessa forma, o Parser estabelece a transição entre o reconhecimento léxico e a representação estrutural do código-fonte.

---

## 18.4 Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência da análise léxica do compilador oficial da Capi.

Outras implementações poderão adotar estratégias diferentes para reconhecimento dos elementos léxicos, desde que preservem integralmente:

* os contratos definidos neste documento;
* a interface formal baseada em tokens;
* os diagnósticos esperados;
* o comportamento determinístico da análise léxica;
* a compatibilidade com o Parser e as fases posteriores do compilador.

Dessa forma, este documento conclui a especificação da análise léxica e estabelece a base sobre a qual o Parser inicia a construção da representação sintática do programa.

