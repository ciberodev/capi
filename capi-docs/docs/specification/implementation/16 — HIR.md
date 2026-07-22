# 16 — Representação Semântica (HIR)

## Especificação de Implementação da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a especificação de implementação da Representação Semântica de Alto Nível (*High-level Intermediate Representation* — HIR) utilizada pelo compilador oficial da linguagem Capi.

Seu objetivo é estabelecer a representação intermediária produzida a partir da Árvore Sintática Abstrata (AST), definindo sua função no pipeline de compilação, seus princípios arquiteturais e os contratos que deverão ser preservados pelas fases responsáveis pela análise semântica.

A HIR constitui a primeira representação do compilador capaz de armazenar informações semânticas sobre o programa, servindo como base para a Resolução de Nomes, Inferência de Tipos, Verificação Semântica e Lowering para a Intermediate Representation (IR).

---

## 1.2 Escopo

Este documento especifica:

* o papel da HIR na arquitetura do compilador;
* sua posição no pipeline de compilação;
* os princípios que orientam sua construção;
* sua organização conceitual;
* sua relação com a AST;
* sua evolução durante as fases semânticas;
* sua interface com as etapas posteriores do Frontend.

Não fazem parte deste documento:

* a implementação da Resolução de Nomes;
* a Inferência de Tipos;
* a Verificação Semântica;
* o processo de Lowering;
* a definição da IR.

Esses assuntos são tratados nos documentos correspondentes da especificação de implementação.

---

## 1.3 Relação com os Documentos Anteriores

A HIR estabelece a transição entre a representação sintática produzida pelo Parser e a representação semântica utilizada pelo restante do Frontend.

Sua construção deve preservar integralmente as garantias estabelecidas por:

| Documento    | Responsabilidade        |
| ------------ | ----------------------- |
| Documento 02 | Sistema de Tipos        |
| Documento 04 | Sintaxe da Linguagem    |
| Documento 05 | Semântica Operacional   |
| Documento 13 | Estrutura do Compilador |
| Documento 15 | Parser e AST            |

O Documento 15 define como o Parser produz a AST.

Este documento especifica como essa árvore sintática é transformada em uma representação semântica adequada para as fases posteriores do compilador.

---

## 1.4 Filosofia da Implementação

A HIR possui uma responsabilidade distinta da AST.

Enquanto a AST representa exclusivamente a estrutura sintática do programa, a HIR representa sua estrutura semântica, preservando apenas as informações relevantes para as etapas posteriores da compilação.

A HIR deve:

* permanecer independente da representação textual original;
* preservar rastreabilidade com a AST;
* permitir enriquecimento progressivo durante a análise semântica;
* servir como base única para todas as fases posteriores do Frontend;
* permanecer independente da arquitetura de destino.

Essa separação reduz o acoplamento entre as fases do compilador e permite que a análise semântica evolua sem modificar a representação sintática.

---

## 1.5 Separação entre Especificação e Implementação

Este documento define os contratos e o comportamento esperado da HIR, mas não impõe uma estrutura específica para sua implementação.

Uma implementação poderá utilizar, por exemplo:

* árvores;
* grafos;
* índices;
* identificadores internos (*handles*);
* arenas de alocação;
* outras estruturas equivalentes.

Independentemente da estratégia adotada, todas as implementações deverão produzir uma HIR funcionalmente equivalente para um mesmo programa válido e preservar os contratos definidos nesta especificação.

---

# 2. Papel da HIR

A HIR constitui a representação semântica oficial do Frontend da linguagem Capi.

Ela estabelece a fronteira entre a análise sintática realizada pelo Parser e as fases responsáveis por interpretar o significado do programa.

A partir da HIR, o compilador passa a trabalhar sobre elementos semânticos em vez de apenas estruturas sintáticas.

---

## 2.1 Entrada

A entrada da HIR consiste na AST produzida pelo Parser.

A construção inicial da HIR utiliza exclusivamente as informações estruturais presentes nessa árvore, sem modificar sua organização nem depender da sequência original de tokens.

---

## 2.2 Saída

A HIR produzida constitui a base para todas as fases posteriores do Frontend.

Ela será utilizada por:

* Resolução de Nomes;
* Inferência de Tipos;
* Verificação Semântica;
* Lowering para IR.

Cada uma dessas etapas acrescenta novas informações sem alterar a identidade conceitual da HIR.

---

## 2.3 Responsabilidades

Compete à HIR:

* representar a estrutura semântica do programa;
* preservar a organização lógica produzida pela AST;
* permitir enriquecimento progressivo;
* servir como artefato comum para todas as fases semânticas;
* preservar rastreabilidade com o código-fonte.

A HIR não executa nenhuma análise por si própria.

Ela constitui apenas a representação sobre a qual essas análises serão realizadas.

---

## 2.4 Limitações

Não compete à HIR:

* reconhecer tokens;
* interpretar a gramática;
* produzir diagnósticos léxicos;
* construir a AST;
* gerar código;
* representar detalhes físicos da arquitetura de destino.

Essas responsabilidades pertencem a outras fases do compilador.

---

## 2.5 Evolução Progressiva

Diferentemente da AST, a HIR pode evoluir ao longo das fases semânticas.

Durante essa evolução poderão ser incorporadas informações como:

* símbolos resolvidos;
* tipos inferidos;
* referências entre elementos;
* atributos semânticos;
* informações auxiliares necessárias ao Lowering.

Essa evolução não altera sua identidade como representação semântica do Frontend.

---

## 2.6 Independência Arquitetural

A HIR deve permanecer completamente independente:

* da IR;
* do Backend;
* da arquitetura de hardware;
* da ABI da plataforma;
* da geração de código.

Sua responsabilidade limita-se exclusivamente à representação semântica de alto nível do programa.

---

# 3. Fluxo da Representação Semântica

A construção da HIR representa a transição entre a análise sintática e a análise semântica do compilador.

A partir desse momento, todas as etapas do Frontend passam a operar sobre uma representação comum, progressivamente enriquecida ao longo do processo de compilação.

---

## 3.1 Visão Geral

Conceitualmente, o fluxo da representação semântica pode ser representado da seguinte forma:

```text
Código-fonte
        │
        ▼
      Lexer
        │
        ▼
      Tokens
        │
        ▼
      Parser
        │
        ▼
        AST
        │
        ▼
   Construção da HIR
        │
        ▼
        HIR
        │
        ▼
 Resolução de Nomes
        │
        ▼
 Inferência de Tipos
        │
        ▼
 Verificação Semântica
        │
        ▼
     Lowering
        │
        ▼
         IR
```

A HIR permanece como representação comum durante toda a análise semântica do Frontend.

---

## 3.2 Construção Inicial

A construção inicial da HIR ocorre imediatamente após a conclusão da análise sintática.

Nessa etapa, a estrutura produzida representa apenas a organização semântica básica derivada da AST, ainda sem as informações que serão incorporadas pelas fases posteriores.

---

## 3.3 Enriquecimento Progressivo

Ao longo da análise semântica, a HIR recebe sucessivamente novas informações produzidas pelas fases especializadas do compilador.

Esse enriquecimento pode incluir:

* resolução de referências;
* associação de símbolos;
* anotação de tipos;
* validação semântica;
* metadados necessários ao Lowering.

Cada fase acrescenta informações próprias sem modificar a finalidade da representação.

---

## 3.4 Comunicação entre as Fases

A HIR constitui o principal mecanismo de comunicação entre as fases semânticas do Frontend.

Cada etapa recebe a HIR produzida pela fase anterior, acrescenta as informações sob sua responsabilidade e disponibiliza uma representação enriquecida para a etapa seguinte.

Essa abordagem reduz o acoplamento entre os componentes do compilador e evita a criação de múltiplas representações intermediárias.

---

## 3.5 Relação com a AST

A AST permanece disponível durante toda a compilação como representação sintática do programa.

A HIR é construída a partir da AST, mas constitui um artefato independente.

As fases semânticas operam sobre a HIR, preservando a AST exclusivamente como referência para rastreabilidade e emissão de diagnósticos.

---

## 3.6 Princípio da Responsabilidade Única

A HIR possui responsabilidade única: representar semanticamente o programa durante o Frontend.

Ela não substitui a AST nem antecipa responsabilidades da IR.

Seu papel consiste em oferecer uma representação suficientemente rica para suportar toda a análise semântica antes do processo de Lowering que produzirá a representação intermediária utilizada pelo Backend.

---

# 4. Estrutura Geral da HIR

A HIR constitui a representação semântica oficial utilizada pelo Frontend da linguagem Capi.

Sua organização deve refletir a estrutura lógica do programa, preservando apenas as informações necessárias para as fases de análise semântica e eliminando detalhes exclusivamente sintáticos que não possuam relevância após a construção da representação.

---

## 4.1 Objetivo

Este documento define os princípios arquiteturais que orientam a construção da HIR. A representação interna de seus elementos, seus campos e sua organização física pertencem exclusivamente à implementação, desde que preservem os contratos estabelecidos nesta especificação.

A HIR possui os seguintes objetivos:

* representar semanticamente o programa;
* servir como base comum para toda a análise semântica;
* preservar rastreabilidade com a AST e o código-fonte;
* permitir enriquecimento progressivo;
* preparar a representação utilizada pelo processo de Lowering.

---

## 4.2 Representação Semântica

A HIR representa a estrutura semântica do programa, e não sua organização textual ou puramente sintática.

Informações cuja finalidade seja exclusivamente permitir o reconhecimento da gramática podem ser descartadas durante sua construção, desde que isso não comprometa a rastreabilidade nem as análises posteriores.

---

## 4.3 Organização Hierárquica

A HIR organiza o programa como uma estrutura composta por elementos semanticamente relacionados.

Esses elementos representam, por exemplo:

* módulos;
* declarações;
* funções;
* tipos;
* comandos;
* expressões.

A forma utilizada para estabelecer essas relações pertence exclusivamente à implementação.

---

## 4.4 Representação Canônica

Construções sintáticas diferentes que representem o mesmo conceito semântico poderão ser convertidas para uma representação comum durante a construção da HIR.

Essa normalização reduz a complexidade das fases posteriores do compilador e simplifica o processo de Lowering.

As regras específicas de normalização pertencem à implementação.

---

## 4.5 Evolução Controlada

A HIR é construída de forma incremental ao longo das fases semânticas do Frontend.

Cada etapa acrescenta exclusivamente as informações sob sua responsabilidade, preservando a consistência da representação produzida pelas fases anteriores.

---

## 4.6 Independência da Implementação

A forma utilizada para representar internamente a HIR pertence exclusivamente à implementação.

Independentemente da estrutura adotada, todas as implementações deverão produzir uma representação semanticamente equivalente para um mesmo programa válido.

---

# 5. Elementos da HIR

A HIR é composta por elementos que representam os componentes semânticos do programa.

Cada elemento corresponde a uma construção da linguagem cuja interpretação será utilizada durante as etapas posteriores do Frontend.

A estrutura concreta desses elementos pertence exclusivamente à implementação.

---

## 5.1 Elementos Semânticos

Cada elemento da HIR representa um conceito semântico identificado no programa.

Exemplos incluem:

* módulos;
* funções;
* tipos;
* variáveis;
* constantes;
* expressões;
* comandos.

Esses elementos constituem a base sobre a qual atuam as fases de análise semântica.

A representação dos tipos na HIR baseia-se nas definições estabelecidas pelo Documento 02 — Sistema de Tipos, preservando as informações necessárias para as fases de Inferência e Verificação de Tipos.

---

## 5.2 Categorias de Elementos

Os elementos da HIR podem ser organizados conceitualmente em categorias como:

* organização do programa;
* declarações;
* tipos;
* expressões;
* comandos;
* atributos semânticos.

Essas categorias não impõem qualquer estrutura específica de implementação.

---

## 5.3 Relações Semânticas

Os elementos da HIR estabelecem relações que representam a organização lógica do programa.

Essas relações poderão representar, por exemplo:

* pertencimento a módulos;
* composição entre elementos;
* dependências;
* referências;
* associações produzidas pela análise semântica.

A representação dessas relações pertence exclusivamente à implementação.

---

## 5.4 Identidade dos Elementos

Cada elemento da HIR deverá possuir identidade própria durante toda a análise semântica.

Essa identidade permite que diferentes fases acrescentem informações ao mesmo elemento sem comprometer a consistência da representação.

O mecanismo utilizado para representar essa identidade pertence exclusivamente à implementação.

---

## 5.5 Compartilhamento

Quando apropriado, diferentes partes da HIR poderão referenciar um mesmo elemento semântico.

Essa capacidade reduz duplicações e facilita a manutenção da consistência durante a evolução da representação.

A estratégia utilizada para esse compartilhamento pertence exclusivamente à implementação.

---

## 5.6 Estabilidade

Uma vez incorporado à HIR, um elemento deve permanecer estável durante toda sua existência.

Novas informações poderão ser associadas ao elemento pelas fases posteriores, mas sua identidade conceitual deverá ser preservada.

---

# 6. Informações Preservadas

Embora a HIR represente uma abstração semântica do programa, determinadas informações provenientes das fases anteriores devem permanecer disponíveis durante toda a análise do Frontend.

Essas informações permitem preservar a rastreabilidade entre o código-fonte original e os elementos semânticos produzidos pelo compilador.

---

## 6.1 Objetivo

A preservação dessas informações possui os seguintes objetivos:

* permitir emissão de diagnósticos precisos;
* manter rastreabilidade com a AST;
* facilitar ferramentas de desenvolvimento;
* apoiar processos de depuração;
* fornecer contexto para as fases posteriores.

---

## 6.2 Localização

Sempre que aplicável, cada elemento da HIR deverá estar associado ao Span correspondente no código-fonte.

Essa associação permite que diagnósticos continuem referenciando corretamente o programa original, mesmo após sucessivas transformações internas.

---

## 6.3 Origem na AST

Cada elemento da HIR deverá preservar sua relação conceitual com a construção correspondente da AST.

Essa rastreabilidade facilita:

* emissão de diagnósticos;
* geração de documentação;
* ferramentas de análise estática;
* navegação em ambientes de desenvolvimento.

A forma utilizada para representar essa relação pertence exclusivamente à implementação.

---

## 6.4 Metadados

A implementação poderá associar metadados adicionais aos elementos da HIR sempre que isso facilitar a análise semântica.

Esses metadados podem incluir informações produzidas pelas fases posteriores do Frontend.

Sua representação pertence exclusivamente à implementação.

---

## 6.5 Continuidade da Rastreabilidade

A relação entre código-fonte, AST e HIR deve permanecer consistente durante toda a compilação.

Essa propriedade garante que qualquer elemento semântico possa ser associado, direta ou indiretamente, à sua origem no programa analisado.

---

## 6.6 Independência da Representação

Os mecanismos utilizados para preservar localização, rastreabilidade e metadados pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar funcionalmente essas informações durante toda a análise semântica do Frontend.

---

# 7. Construção da HIR

A construção da HIR representa a transição entre a representação sintática produzida pelo Parser e a representação semântica utilizada pelas fases posteriores do Frontend.

Durante essa transformação, a estrutura da AST é reinterpretada sob uma perspectiva semântica, preservando apenas as informações necessárias para a continuidade da compilação.

---

## 7.1 Objetivo

A construção da HIR possui os seguintes objetivos:

* transformar a AST em uma representação semântica;
* preservar a estrutura lógica do programa;
* eliminar detalhes exclusivamente sintáticos;
* preparar o programa para as análises semânticas;
* manter rastreabilidade com a AST.

---

## 7.2 Transformação da AST

A construção da HIR ocorre a partir da AST produzida pelo Parser.

Cada construção sintática é convertida para seu correspondente semântico, preservando o significado estrutural do programa.

A estratégia utilizada para realizar essa transformação pertence exclusivamente à implementação.

---

## 7.3 Correspondência entre AST e HIR

Nem todo nó da AST possui correspondência direta com um único elemento da HIR.

Durante a transformação poderão ocorrer:

* fusão de construções sintáticas;
* decomposição em múltiplos elementos semânticos;
* eliminação de estruturas auxiliares da gramática;
* reorganização da representação.

Independentemente da estratégia adotada, o significado do programa deverá permanecer inalterado.

---

## 7.4 Preservação Semântica

A construção da HIR não deve alterar o significado do programa.

Seu objetivo consiste exclusivamente em produzir uma representação mais adequada para as fases posteriores da análise semântica.

Qualquer transformação realizada durante essa etapa deve preservar integralmente a semântica definida pela linguagem.

---

## 7.5 Construção Determinística

Para uma mesma AST, utilizando a mesma versão do compilador e as mesmas opções de compilação, a construção da HIR deverá produzir uma representação funcionalmente equivalente.

Esse princípio garante previsibilidade e reprodutibilidade durante todo o processo de compilação.

---

## 7.6 Independência da Implementação

Os algoritmos utilizados para transformar a AST em HIR pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir representações semanticamente equivalentes.

---

# 8. Normalização Semântica

A HIR pode adotar formas canônicas para representar construções equivalentes da linguagem.

O objetivo dessa normalização é reduzir a diversidade de representações internas, simplificando as fases responsáveis pela análise semântica e pelo processo de Lowering.

---

## 8.1 Objetivo

A normalização possui os seguintes objetivos:

* simplificar a representação semântica;
* reduzir casos especiais;
* facilitar análises posteriores;
* tornar o comportamento do compilador mais previsível;
* preparar a HIR para o Lowering.

---

## 8.2 Açúcar Sintático

Construções classificadas como açúcar sintático (*syntactic sugar*) poderão ser convertidas para representações semânticas equivalentes durante a construção da HIR.

Essa transformação elimina diferenças puramente sintáticas sem alterar o comportamento observado do programa.

A normalização deve preservar integralmente a semântica definida pelo Documento 05 — Semântica Operacional, garantindo que diferentes representações sintáticas produzam comportamento observável equivalente.

As regras específicas de normalização pertencem à implementação.

---

## 8.3 Formas Canônicas

Construções semanticamente equivalentes poderão ser representadas por uma única forma canônica na HIR.

Essa abordagem reduz a complexidade das fases posteriores, permitindo que diferentes formas sintáticas sejam tratadas de maneira uniforme.

---

## 8.4 Simplificação Estrutural

Durante a normalização poderão ser eliminados elementos cuja existência seja necessária apenas para a representação sintática.

Entre esses elementos incluem-se construções intermediárias utilizadas exclusivamente pela gramática.

A remoção desses elementos não deve comprometer a rastreabilidade com o código-fonte.

---

## 8.5 Preservação do Significado

Toda transformação realizada durante a normalização deve preservar integralmente o significado do programa.

Diferenças introduzidas pela implementação não podem alterar o comportamento observável definido pela linguagem.

---

## 8.6 Independência da Implementação

As estratégias de normalização pertencem exclusivamente à implementação.

Este documento estabelece apenas que representações semanticamente equivalentes deverão produzir resultados funcionalmente equivalentes durante as fases posteriores do compilador.

---

# 9. Independência da AST

A construção da HIR não substitui a AST nem modifica sua finalidade.

Ambas as representações coexistem durante o Frontend, cada uma exercendo responsabilidades distintas dentro da arquitetura do compilador.

---

## 9.1 Objetivo

A coexistência entre AST e HIR possui os seguintes objetivos:

* preservar a separação entre sintaxe e semântica;
* manter rastreabilidade com o código-fonte;
* reduzir o acoplamento entre as fases;
* facilitar diagnósticos e ferramentas de desenvolvimento.

---

## 9.2 Preservação da AST

A construção da HIR não deve modificar a AST produzida pelo Parser.

A AST permanece disponível como representação sintática oficial durante toda a compilação.

Essa propriedade preserva a estabilidade do pipeline do Frontend.

---

## 9.3 Independência das Representações

A AST e a HIR representam artefatos distintos.

Alterações realizadas durante o enriquecimento da HIR não devem modificar a estrutura da AST.

Da mesma forma, a existência da AST não deve impor restrições à evolução da HIR.

---

## 9.4 Rastreabilidade

Embora independentes, AST e HIR devem permanecer semanticamente relacionadas.

Sempre que necessário, deve ser possível estabelecer a correspondência entre um elemento da HIR e sua construção sintática de origem.

Os mecanismos utilizados para manter essa relação pertencem exclusivamente à implementação.

---

## 9.5 Comunicação com as Fases Posteriores

Após sua construção, a HIR passa a constituir a representação oficial utilizada pelas fases de:

* Resolução de Nomes;
* Inferência de Tipos;
* Verificação Semântica;
* Lowering.

Essas fases não devem depender diretamente da AST para realizar suas responsabilidades.

---

## 9.6 Princípio da Separação

A coexistência entre AST e HIR constitui um princípio fundamental da arquitetura do compilador da Capi.

A AST permanece responsável pela representação sintática do programa.

A HIR permanece responsável por sua representação semântica.

Essa separação reduz o acoplamento entre as fases do Frontend, facilita a evolução da implementação e preserva a clareza arquitetural do pipeline de compilação.

---

# 10. Informações Semânticas

A principal característica da HIR é sua capacidade de armazenar informações semânticas produzidas pelas diferentes fases do Frontend.

Enquanto a AST representa exclusivamente a estrutura sintática do programa, a HIR permite que informações progressivamente obtidas durante a compilação sejam associadas aos seus elementos, preservando uma representação unificada durante toda a análise semântica.

---

## 10.1 Objetivo

O enriquecimento semântico da HIR possui os seguintes objetivos:

* associar informações produzidas pelas fases semânticas;
* evitar a duplicação de representações intermediárias;
* permitir compartilhamento de informações entre as fases do Frontend;
* preparar a representação utilizada pelo processo de Lowering;
* preservar a consistência da análise semântica.

---

## 10.2 Informações Associadas

Ao longo da análise semântica, diferentes categorias de informações poderão ser incorporadas aos elementos da HIR.

Entre elas incluem-se, quando aplicável:

* símbolos;
* referências resolvidas;
* tipos;
* atributos semânticos;
* qualificadores;
* metadados utilizados pelas fases posteriores.

A forma utilizada para representar essas informações pertence exclusivamente à implementação.

Entre essas informações podem incluir-se propriedades semânticas dos tipos definidas pelo Documento 02 — Sistema de Tipos, incluindo atributos necessários às fases posteriores de inferência, validação e verificação semântica.

---

## 10.3 Associação Progressiva

As informações semânticas não precisam estar disponíveis no momento da construção inicial da HIR.

Cada fase do Frontend acrescenta exclusivamente as informações sob sua responsabilidade, preservando os resultados produzidos pelas fases anteriores.

Essa evolução progressiva reduz o acoplamento entre os componentes do compilador.

---

## 10.4 Consistência

Toda informação associada à HIR deve permanecer consistente com o estado atual da análise semântica.

Sempre que uma fase acrescentar novos dados, deverá preservar a validade das informações previamente produzidas.

A coordenação dessa consistência pertence exclusivamente à implementação.

---

## 10.5 Compartilhamento

As informações semânticas incorporadas à HIR devem permanecer disponíveis para todas as fases posteriores que delas necessitem.

Essa característica evita recomputações desnecessárias e reduz a duplicação de informações durante o Frontend.

---

## 10.6 Independência da Implementação

Os mecanismos utilizados para armazenar e atualizar informações semânticas pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar o comportamento funcional definido por esta especificação.

---

# 11. Evolução da HIR

A HIR representa um artefato que evolui ao longo da análise semântica do programa.

Sua evolução não consiste na substituição da representação existente, mas na incorporação gradual de novas informações produzidas pelas diferentes fases do Frontend.

---

## 11.1 Objetivo

A evolução da HIR possui os seguintes objetivos:

* permitir enriquecimento progressivo;
* reduzir o número de representações intermediárias;
* preservar continuidade entre as fases;
* facilitar a comunicação entre os componentes do compilador.

---

## 11.2 Estado Inicial

Após sua construção a partir da AST, a HIR contém apenas as informações estruturais necessárias para iniciar a análise semântica.

Nesse momento, elementos como símbolos, tipos e referências ainda poderão estar ausentes ou incompletos.

---

## 11.3 Evolução pelas Fases

À medida que o Frontend avança, diferentes fases acrescentam novas informações à HIR.

Entre elas incluem-se:

* Resolução de Nomes;
* Inferência de Tipos;
* Verificação Semântica.

Cada fase atua exclusivamente sobre os aspectos sob sua responsabilidade.

---

## 11.4 Estado Final

Ao término da análise semântica, a HIR deverá conter todas as informações necessárias para iniciar o processo de Lowering.

Nesse estágio, a representação encontra-se semanticamente consistente e suficientemente enriquecida para permitir sua transformação em IR.

---

## 11.5 Continuidade

A evolução da HIR deve ocorrer de forma contínua e consistente.

Ao término de cada fase, a representação deve permanecer consistente e pronta para utilização pela etapa seguinte do Frontend.

Fases posteriores não devem depender da correção de inconsistências produzidas por fases anteriores.

---

## 11.6 Determinismo

Para um mesmo programa válido, utilizando a mesma versão do compilador e as mesmas opções de compilação, a evolução da HIR deverá produzir resultados funcionalmente equivalentes.

Esse princípio garante previsibilidade e reprodutibilidade durante toda a análise semântica.

---

# 12. Imutabilidade e Atualização

Embora a HIR evolua durante o Frontend, essa evolução deve ocorrer de forma controlada e consistente.

O objetivo é permitir o enriquecimento progressivo da representação sem comprometer sua estabilidade nem introduzir inconsistências entre as diferentes fases do compilador.

---

## 12.1 Objetivo

A política de atualização da HIR possui os seguintes objetivos:

* preservar a consistência da representação;
* permitir enriquecimento progressivo;
* reduzir efeitos colaterais entre as fases;
* facilitar futuras estratégias de compilação incremental.

---

## 12.2 Atualização Controlada

Cada fase do Frontend deverá modificar exclusivamente as informações sob sua responsabilidade.

Nenhuma fase deve alterar informações pertencentes conceitualmente a outra etapa da análise semântica.

Essa separação reduz o acoplamento entre os componentes do compilador.

---

## 12.3 Consistência

Após a conclusão de cada fase da análise semântica, a HIR deverá permanecer em um estado consistente.

As informações produzidas deverão estar prontas para utilização pela etapa seguinte do pipeline.

---

## 12.4 Compilação Incremental

A arquitetura da HIR deve permitir futura integração com mecanismos de compilação incremental.

Essa capacidade poderá incluir:

* atualização localizada de elementos;
* invalidação seletiva de informações;
* reconstrução parcial da representação;
* reutilização de resultados previamente calculados.

A implementação inicial não é obrigada a oferecer essas funcionalidades.

---

## 12.5 Imutabilidade Conceitual

Embora determinadas informações possam ser acrescentadas durante a evolução da HIR, sua estrutura conceitual deve permanecer estável.

A identidade dos elementos semânticos não deve ser alterada ao longo da análise do programa.

Os mecanismos utilizados para preservar essa propriedade pertencem exclusivamente à implementação.

---

## 12.6 Independência da Implementação

As estratégias utilizadas para atualizar, enriquecer e manter a consistência da HIR pertencem exclusivamente à implementação.

Independentemente da abordagem adotada, todas as implementações deverão preservar:

* a identidade dos elementos semânticos;
* a consistência da representação;
* os contratos definidos neste documento;
* o comportamento determinístico da análise semântica.

---

# 13. Interface da HIR

A HIR constitui a principal interface de comunicação entre as fases semânticas do Frontend.

Seu objetivo é disponibilizar uma representação comum do programa, permitindo que cada etapa da análise semântica acrescente as informações sob sua responsabilidade sem depender dos mecanismos internos das demais fases.

A forma concreta dessa interface pertence à implementação, desde que preserve os princípios estabelecidos neste documento.

---

## 13.1 Objetivo

A interface da HIR deve:

* disponibilizar a representação semântica do programa;
* permitir o enriquecimento progressivo pelas fases do Frontend;
* preservar a independência entre os componentes do compilador;
* ocultar os mecanismos internos de implementação.

---

## 13.2 Entrada

A construção inicial da HIR recebe como entrada:

* a AST produzida pelo Parser;
* as informações de localização preservadas pela análise sintática;
* o contexto necessário para emissão de diagnósticos.

As fases posteriores recebem a própria HIR enriquecida pela etapa anterior.

---

## 13.3 Saída

A saída da HIR consiste em uma representação semântica progressivamente enriquecida do programa.

Ao término da análise semântica, essa representação deverá conter todas as informações necessárias para permitir o processo de Lowering para a Intermediate Representation (IR).

---

## 13.4 Compartilhamento

Todas as fases semânticas do Frontend deverão operar sobre a HIR.

A implementação poderá utilizar diferentes mecanismos para disponibilizar essa representação, desde que preserve:

* consistência;
* identidade dos elementos;
* rastreabilidade;
* comportamento determinístico.

---

## 13.5 Comunicação com o Lowering

Ao término da análise semântica, a HIR constitui a única interface formal entre o Frontend e a etapa responsável pelo Lowering.

O processo de Lowering não deve depender:

* da AST;
* da sequência de tokens;
* do código-fonte;
* dos mecanismos internos das fases anteriores.

Essa separação preserva o baixo acoplamento entre o Frontend e o Middle-end.

---

## 13.6 Contrato da Interface

Independentemente da implementação adotada, toda interface da HIR deve garantir:

* representação semântica consistente;
* preservação da rastreabilidade;
* compartilhamento entre as fases do Frontend;
* compatibilidade com o processo de Lowering;
* comportamento equivalente para um mesmo programa válido.

---

# 14. Estratégias de Implementação

Este documento define o comportamento esperado da HIR, mas não impõe uma estratégia específica para sua implementação.

Diferentes estruturas de dados podem ser utilizadas para representar os elementos semânticos do programa, desde que preservem os contratos definidos nesta especificação.

---

## 14.1 Objetivo

A implementação da HIR deve buscar:

* simplicidade;
* previsibilidade;
* eficiência;
* facilidade de manutenção;
* facilidade de evolução;
* facilidade de testes.

A escolha da estratégia pertence exclusivamente ao projeto de implementação.

---

## 14.2 Estruturas de Representação

Entre as estratégias normalmente utilizadas para implementar representações semânticas encontram-se:

* árvores;
* grafos;
* arenas de objetos;
* identificadores internos (*handles*);
* índices para tabelas de elementos;
* outras estruturas equivalentes.

Todas essas abordagens são compatíveis com esta especificação.

---

## 14.3 Critérios de Escolha

A escolha da estratégia poderá considerar fatores como:

* desempenho;
* consumo de memória;
* simplicidade da implementação;
* facilidade de atualização;
* integração com as fases posteriores do compilador.

Esses critérios pertencem exclusivamente ao projeto de implementação.

---

## 14.4 Independência Arquitetural

Independentemente da estratégia utilizada, a HIR deve permanecer completamente independente:

* da IR;
* do Backend;
* da arquitetura de hardware;
* da ABI da plataforma;
* da geração de código.

Essa independência constitui um dos princípios fundamentais da arquitetura do compilador.

---

## 14.5 Equivalência Funcional

Implementações distintas devem produzir HIRs funcionalmente equivalentes para um mesmo programa válido.

Da mesma forma, programas semanticamente inválidos deverão resultar em diagnósticos funcionalmente equivalentes e em comportamento compatível com esta especificação.

---

## 14.6 Evolução

A estratégia de implementação poderá evoluir ao longo do tempo.

Mudanças internas são permitidas desde que preservem:

* a interface pública da HIR;
* os contratos definidos neste documento;
* o comportamento observável da análise semântica.

---

# 15. Integração com as Próximas Etapas

A HIR constitui a representação compartilhada por todas as fases semânticas do Frontend.

Cada etapa do compilador atua sobre essa representação, acrescentando exclusivamente as informações sob sua responsabilidade e preparando a HIR para a fase seguinte.

---

## 15.1 Objetivo

A integração entre a HIR e as fases posteriores possui os seguintes objetivos:

* preservar a continuidade da análise semântica;
* reduzir o acoplamento entre os componentes;
* evitar múltiplas representações intermediárias;
* preparar o programa para o processo de Lowering.

---

## 15.2 Resolução de Nomes

A primeira etapa que atua diretamente sobre a HIR é a Resolução de Nomes.

Essa fase estabelece a correspondência entre identificadores e seus respectivos elementos declarados, enriquecendo a representação semântica sem modificar sua estrutura fundamental.

Sua especificação é apresentada no Documento 17 — Resolução de Nomes e Escopos.

---

## 15.3 Inferência e Verificação de Tipos

Após a resolução de nomes, a HIR é utilizada pela fase responsável pela Inferência e Verificação de Tipos.

Essa etapa incorpora informações relacionadas aos tipos dos elementos do programa e verifica sua consistência.

Sua especificação é apresentada no Documento 18 — Inferência e Verificação de Tipos.

---

## 15.4 Verificação Semântica

Com os símbolos resolvidos e os tipos conhecidos, a HIR passa a ser utilizada pela Verificação Semântica.

Essa fase valida as regras semânticas definidas pela linguagem, produzindo diagnósticos quando necessário.

Sua especificação é apresentada no Documento 19 — Verificação Semântica.

---

## 15.5 Lowering

Concluída a análise semântica, a HIR torna-se a entrada do processo de Lowering.

Essa etapa transforma a representação semântica de alto nível na Intermediate Representation (IR), utilizada pelo Middle-end do compilador.

Sua especificação é apresentada no Documento 20 — Lowering para IR.

---

## 15.6 Continuidade Arquitetural

A HIR constitui o elo de integração entre todas as fases do Frontend.

Sua utilização como representação semântica única elimina redundâncias, reduz o acoplamento entre os componentes e estabelece uma arquitetura contínua desde a AST até a geração da IR.

Esse princípio constitui um dos fundamentos da arquitetura do compilador oficial da linguagem Capi.

---

# 16. Diagnósticos

A HIR constitui a principal representação utilizada durante a análise semântica do Frontend.

Embora a emissão efetiva dos diagnósticos pertença às fases responsáveis pela Resolução de Nomes, Inferência de Tipos e Verificação Semântica, a HIR deve preservar todas as informações necessárias para que esses diagnósticos possam ser produzidos de forma precisa e consistente.

Os mecanismos gerais de emissão de diagnósticos são definidos pelo Documento 23 — Diagnósticos e Recuperação de Erros. Este capítulo especifica apenas os requisitos da HIR relacionados ao suporte à produção de diagnósticos.

---

## 16.1 Objetivo

O suporte da HIR aos diagnósticos possui os seguintes objetivos:

* preservar a localização dos elementos do programa;
* manter rastreabilidade com a AST e o código-fonte;
* fornecer contexto às fases semânticas;
* permitir diagnósticos precisos e consistentes.

---

## 16.2 Localização

Sempre que aplicável, os elementos da HIR deverão preservar informações suficientes para localizar sua origem no código-fonte.

Essas informações deverão permitir identificar:

* arquivo;
* linha;
* coluna;
* intervalo correspondente (*Span*).

A forma utilizada para preservar essas informações pertence exclusivamente à implementação.

---

## 16.3 Contexto

Além da localização, a HIR poderá disponibilizar informações de contexto necessárias para produção de diagnósticos mais precisos.

Entre elas incluem-se, quando aplicável:

* elemento semântico relacionado;
* declaração de origem;
* construção sintática correspondente;
* demais informações auxiliares.

A representação dessas informações pertence exclusivamente à implementação.

---

## 16.4 Continuidade

Durante toda a análise semântica, os elementos da HIR deverão preservar a consistência das informações utilizadas para emissão de diagnósticos.

O enriquecimento progressivo da representação não deve comprometer a rastreabilidade estabelecida durante sua construção.

---

## 16.5 Independência

A HIR não é responsável por produzir diagnósticos.

Sua responsabilidade limita-se a disponibilizar as informações necessárias para que as fases posteriores possam emitir mensagens consistentes e corretamente localizadas.

---

## 16.6 Compatibilidade

Independentemente da estratégia de implementação adotada, todas as implementações deverão preservar informações suficientes para que os diagnósticos produzidos durante a análise semântica permaneçam funcionalmente equivalentes.

---

# 17. Testabilidade

A HIR representa o principal artefato compartilhado pelas fases semânticas do Frontend.

Sua natureza determinística permite a construção de testes independentes das etapas posteriores do compilador, garantindo elevado nível de confiabilidade e facilitando a evolução da implementação.

A implementação deve manter uma suíte abrangente de testes automatizados, incluindo testes de regressão para cada defeito identificado e posteriormente corrigido. Esses testes garantem que problemas anteriormente solucionados não sejam reintroduzidos em versões futuras do compilador.

---

## 17.1 Objetivo

Os testes da HIR possuem os seguintes objetivos:

* validar sua construção;
* verificar seu enriquecimento progressivo;
* detectar regressões;
* garantir comportamento determinístico;
* assegurar compatibilidade com as fases posteriores.

---

## 17.2 Testes de Construção

A construção inicial da HIR deverá possuir testes específicos.

Esses testes verificam aspectos como:

* transformação da AST;
* preservação da estrutura lógica;
* rastreabilidade;
* consistência inicial da representação.

---

## 17.3 Golden Tests

A implementação poderá utilizar *Golden Tests* para validar a representação semântica produzida para programas previamente conhecidos.

Cada execução compara a HIR produzida com uma representação previamente validada, permitindo detectar alterações inesperadas em sua construção ou evolução.

---

## 17.4 Testes de Integração

Além dos testes unitários, deverão existir testes destinados a verificar a integração entre a HIR e as fases responsáveis por:

* Resolução de Nomes;
* Inferência de Tipos;
* Verificação Semântica;
* Lowering.

Esses testes verificam a continuidade arquitetural do Frontend.

---

## 17.5 Benchmarks

A implementação poderá incluir benchmarks destinados a acompanhar o desempenho da construção e evolução da HIR.

Entre os aspectos monitorados encontram-se:

* tempo de construção;
* consumo de memória;
* desempenho durante o enriquecimento semântico;
* escalabilidade em projetos de grande porte.

Esses benchmarks auxiliam a evolução da implementação sem alterar seus contratos funcionais.

---

## 17.6 Determinismo

Todos os testes relacionados à HIR devem pressupor comportamento determinístico.

Para um mesmo programa válido, utilizando a mesma versão do compilador e as mesmas opções de compilação, a representação produzida e seu enriquecimento ao longo das fases semânticas deverão ser funcionalmente equivalentes.

---

# 18. Considerações Finais

A HIR constitui a representação semântica central do Frontend do compilador da linguagem Capi.

Sua responsabilidade consiste em servir como base comum para todas as fases da análise semântica, estabelecendo uma arquitetura contínua entre a representação sintática produzida pelo Parser e a Intermediate Representation (IR) utilizada pelo Middle-end.

---

## 18.1 Síntese da Arquitetura

A HIR é caracterizada pelos seguintes princípios:

* representação semântica única;
* enriquecimento progressivo;
* comportamento determinístico;
* preservação da rastreabilidade;
* separação entre sintaxe e semântica;
* independência da arquitetura de destino.

Esses princípios garantem simplicidade, previsibilidade e facilidade de evolução da implementação.

---

## 18.2 Princípios Preservados

Toda implementação compatível com esta especificação deverá preservar:

* equivalência funcional entre implementações;
* representação semântica consistente;
* rastreabilidade com a AST e o código-fonte;
* compartilhamento entre as fases do Frontend;
* independência em relação ao Middle-end e ao Backend.

Esses princípios asseguram interoperabilidade entre diferentes implementações do compilador.

---

## 18.3 Relação com o Documento Seguinte

Este documento define a representação semântica utilizada durante todo o Frontend do compilador.

O documento seguinte, **17 — Resolução de Nomes e Escopos**, especifica a primeira fase responsável por enriquecer a HIR, estabelecendo a associação entre identificadores e seus respectivos elementos declarados.

A Resolução de Nomes representa o primeiro passo da análise semântica propriamente dita. Em seguida, a HIR continuará sendo enriquecida pelas fases de **Inferência e Verificação de Tipos (Documento 18)** e **Verificação Semântica (Documento 19)**, até tornar-se a entrada do processo de **Lowering para IR (Documento 20)**.

---

## 18.4 Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência da Representação Semântica de Alto Nível (HIR) do compilador oficial da linguagem Capi.

Outras implementações poderão adotar estratégias diferentes para representar e enriquecer semanticamente o programa, desde que preservem integralmente:

* os contratos definidos neste documento;
* a representação semântica compartilhada;
* a rastreabilidade com a AST;
* o comportamento determinístico da análise semântica;
* a compatibilidade com as fases posteriores do Frontend e com o processo de Lowering.

Dessa forma, este documento conclui a especificação da representação semântica da Capi e estabelece a base arquitetural sobre a qual se iniciam as fases responsáveis pela Resolução de Nomes, Inferência de Tipos e Verificação Semântica.
