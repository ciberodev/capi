# 23 — Diagnósticos e Recuperação de Erros

## Especificação de Implementação da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a especificação de implementação da arquitetura de **Diagnósticos e Recuperação de Erros** utilizada pelo compilador oficial da linguagem Capi.

Seu objetivo é estabelecer os contratos arquiteturais responsáveis pela produção, organização, recuperação e apresentação de diagnósticos durante todo o processo de compilação.

Este documento não especifica mensagens de erro individuais, formatos de apresentação nem algoritmos específicos de recuperação. Seu foco está exclusivamente nos contratos que toda implementação compatível deverá respeitar.

---

## 1.2 Escopo

Este documento especifica:

* a arquitetura de diagnósticos do compilador;
* os contratos para produção de diagnósticos;
* os princípios de recuperação de erros;
* a organização dos diagnósticos ao longo do pipeline;
* a relação entre diagnósticos e as diferentes representações do programa.

Não fazem parte deste documento:

* regras específicas de análise léxica;
* regras específicas de análise sintática;
* regras específicas da análise semântica;
* formatos de interface gráfica;
* algoritmos de recuperação;
* mecanismos internos de armazenamento.

Esses assuntos pertencem aos respectivos documentos da especificação de implementação.

---

## 1.3 Relação com os Documentos Anteriores

A arquitetura de diagnósticos é utilizada por praticamente todas as etapas do compilador.

Cada fase possui responsabilidade pela detecção das situações relacionadas à sua própria atividade, enquanto este documento estabelece a infraestrutura comum utilizada para registrar, organizar, recuperar e apresentar essas informações.

Relaciona-se diretamente com:

| Documento    | Responsabilidade                                |
| ------------ | ----------------------------------------------- |
| Documento 14 | Análise Léxica                                  |
| Documento 15 | Parser e AST                                    |
| Documento 17 | Resolução de Nomes e Escopos                    |
| Documento 18 | Inferência e Verificação de Tipos               |
| Documento 19 | Verificação Semântica                           |
| Documento 20 | Lowering para Intermediate Representation       |
| Documento 21 | Intermediate Representation                     |
| Documento 22 | Otimizações sobre a Intermediate Representation |

As responsabilidades específicas de cada fase permanecem definidas em seus respectivos documentos.

---

## 1.4 Filosofia da Implementação

A arquitetura de diagnósticos possui responsabilidade única: fornecer uma infraestrutura comum para comunicação de problemas identificados durante o processo de compilação.

Essa arquitetura separa claramente as responsabilidades de:

* detectar problemas;
* produzir diagnósticos;
* recuperar a compilação, quando possível;
* organizar as informações produzidas;
* apresentar os diagnósticos ao usuário ou às ferramentas de desenvolvimento.

A forma como essas atividades são implementadas pertence exclusivamente à implementação.

---

## 1.5 Separação entre Especificação e Implementação

Este documento estabelece apenas os contratos arquiteturais da infraestrutura de diagnósticos.

Uma implementação poderá utilizar, por exemplo:

* diferentes estruturas para armazenamento;
* diferentes mecanismos de recuperação;
* diferentes formatos de apresentação;
* diferentes estratégias de organização;
* diferentes políticas de continuidade da compilação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 2. Papel dos Diagnósticos

Os diagnósticos constituem o mecanismo oficial de comunicação entre o compilador, as ferramentas de desenvolvimento e o usuário sempre que uma situação relevante for identificada durante o processo de compilação.

Seu papel consiste em registrar essas situações de maneira consistente, preservando informações suficientes para permitir sua compreensão, localização e eventual correção.

---

## 2.1 Objetivo

Os diagnósticos possuem os seguintes objetivos:

* comunicar problemas encontrados durante a compilação;
* identificar sua localização no programa;
* fornecer informações suficientes para compreensão da situação;
* permitir continuidade da compilação quando possível;
* oferecer suporte às ferramentas de desenvolvimento.

---

## 2.2 Papel no Pipeline

Cada etapa do compilador poderá produzir diagnósticos relacionados exclusivamente à sua responsabilidade.

A infraestrutura definida neste documento estabelece uma arquitetura comum para que esses diagnósticos possam ser tratados de maneira uniforme durante todo o pipeline.

---

## 2.3 Continuidade da Compilação

Sempre que puder ser realizado com segurança, a compilação deverá prosseguir após a identificação de um problema.

A recuperação de erros possui como objetivo reduzir o efeito cascata de diagnósticos e permitir que múltiplos problemas sejam identificados em uma única execução do compilador.

Os mecanismos utilizados para essa recuperação pertencem exclusivamente à implementação.

---

## 2.4 Relação com as Etapas do Compilador

Cada etapa do compilador é responsável exclusivamente pelos diagnósticos relacionados à sua própria atividade.

Entre eles incluem-se, quando aplicável:

* diagnósticos léxicos;
* diagnósticos sintáticos;
* diagnósticos de resolução de nomes;
* diagnósticos de tipos;
* diagnósticos semânticos;
* diagnósticos internos das etapas posteriores do compilador.

Este documento não altera as responsabilidades definidas para cada fase da compilação.

---

## 2.5 Responsabilidades

Compete à arquitetura de diagnósticos:

* registrar as situações identificadas pelo compilador;
* preservar as informações relevantes para cada diagnóstico;
* permitir a continuidade da compilação quando apropriado;
* manter consistência na organização dos diagnósticos;
* fornecer uma interface comum às demais etapas do compilador.

---

## 2.6 Independência Arquitetural

A infraestrutura de diagnósticos permanece independente:

* da sintaxe da linguagem;
* da HIR;
* da Intermediate Representation;
* das otimizações;
* da arquitetura de hardware;
* da geração de código.

Ela constitui um serviço compartilhado por todas as etapas do compilador, sem pertencer exclusivamente a nenhuma delas.

---

# 3. Fluxo dos Diagnósticos

Os diagnósticos acompanham todo o processo de compilação.

Sempre que uma etapa identifica uma situação relevante, um diagnóstico poderá ser produzido, registrado e disponibilizado para as etapas posteriores do compilador e para as ferramentas de desenvolvimento.

Este documento estabelece apenas os contratos desse fluxo, não impondo qualquer arquitetura específica para sua implementação.

---

## 3.1 Visão Geral

Conceitualmente, o fluxo dos diagnósticos pode ser representado da seguinte forma:

```text
        Situação Detectada
                 │
                 ▼
      Produção do Diagnóstico
                 │
                 ▼
     Registro e Organização
                 │
                 ▼
 Recuperação (quando aplicável)
                 │
                 ▼
 Continuação da Compilação
                 │
                 ▼
 Apresentação ao Usuário
```

Cada etapa participa desse fluxo apenas naquilo que pertence à sua responsabilidade.

---

## 3.2 Produção

Sempre que uma etapa identificar uma situação que deva ser comunicada, poderá produzir um diagnóstico correspondente.

A forma como essa identificação é realizada pertence exclusivamente à implementação.

---

## 3.3 Registro

Após sua produção, o diagnóstico deverá ser incorporado à infraestrutura utilizada pelo compilador.

Esse registro deverá preservar todas as informações necessárias para sua utilização pelas etapas posteriores do processo de compilação.

---

## 3.4 Recuperação

Sempre que possível, a etapa responsável poderá realizar mecanismos de recuperação que permitam a continuidade da compilação.

A recuperação não elimina o diagnóstico produzido nem altera sua validade.

Os mecanismos utilizados para essa recuperação pertencem exclusivamente à implementação.

---

## 3.5 Encerramento

Ao término da compilação, todos os diagnósticos produzidos deverão permanecer disponíveis para apresentação ao usuário e para utilização pelas ferramentas compatíveis com o compilador.

A forma como essas informações são disponibilizadas pertence exclusivamente à implementação.

---

## 3.6 Responsabilidade Única

A infraestrutura definida neste documento possui responsabilidade única: estabelecer uma arquitetura comum para produção, organização, recuperação e apresentação de diagnósticos durante todo o processo de compilação.

Este documento não define regras específicas de cada etapa do compilador nem estabelece políticas obrigatórias de recuperação de erros, deixando essas decisões exclusivamente para a implementação.

---

# 4. Estrutura dos Diagnósticos

Os diagnósticos constituem a unidade fundamental de comunicação de problemas durante o processo de compilação.

Cada diagnóstico representa uma situação identificada por uma etapa do compilador e reúne as informações necessárias para sua compreensão, localização e eventual correção.

Este documento estabelece apenas os contratos arquiteturais dessa estrutura, sem definir sua representação concreta.

---

## 4.1 Objetivo

A estrutura dos diagnósticos possui os seguintes objetivos:

* representar de forma uniforme as situações identificadas pelo compilador;
* preservar informações relevantes sobre cada ocorrência;
* permitir tratamento consistente por todas as etapas do pipeline;
* oferecer suporte às ferramentas de desenvolvimento.

---

## 4.2 Severidade

Cada diagnóstico deverá possuir uma classificação de severidade compatível com sua finalidade.

Entre as categorias que poderão existir incluem-se, quando aplicável:

* erro;
* aviso (*warning*);
* informação (*info*);
* sugestão (*hint*);
* outras categorias definidas pela implementação.

A classificação adotada pertence exclusivamente à implementação.

---

## 4.3 Localização

Sempre que aplicável, um diagnóstico deverá preservar informações suficientes para localizar a situação correspondente no programa analisado.

Entre essas informações incluem-se, quando aplicável:

* arquivo de origem;
* linha;
* coluna;
* intervalo correspondente (*span*);
* demais informações de localização disponíveis.

A forma como essas informações são representadas pertence exclusivamente à implementação.

A representação da localização dos diagnósticos baseia-se conceitualmente no conceito de *Span* definido pelo Documento 14 — Análise Léxica, que estabelece a estrutura utilizada para identificar intervalos no código-fonte.

---

## 4.4 Código do Diagnóstico

Uma implementação poderá associar identificadores únicos aos diagnósticos produzidos.

Esses códigos facilitam:

* documentação;
* pesquisa de problemas;
* integração com ferramentas;
* automação de testes;
* referência em materiais técnicos.

A definição desses códigos pertence exclusivamente à implementação.

---

## 4.5 Dados Adicionais

Além das informações obrigatórias, um diagnóstico poderá conter dados complementares relevantes para sua compreensão.

Entre eles incluem-se, quando aplicável:

* contexto da ocorrência;
* elementos relacionados;
* informações de rastreabilidade;
* referências internas;
* demais informações úteis ao processo de compilação.

A seleção dessas informações pertence exclusivamente à implementação.

---

## 4.6 Independência da Implementação

A estrutura concreta utilizada para representar diagnósticos pertence exclusivamente à implementação.

Independentemente da representação adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 5. Produção dos Diagnósticos

Os diagnósticos poderão ser produzidos por qualquer etapa do compilador cuja responsabilidade inclua a identificação de situações relevantes durante o processamento do programa.

Cada fase permanece responsável exclusivamente pelos diagnósticos relacionados à sua própria atividade.

Este documento estabelece apenas a infraestrutura comum utilizada para representá-los.

Independentemente da etapa responsável por sua produção, todos os diagnósticos deverão utilizar a infraestrutura comum definida neste documento, garantindo uniformidade na representação, organização e recuperação das informações.

---

## 5.1 Origem

Cada diagnóstico deverá possuir uma origem claramente identificável.

Essa origem corresponde à etapa do compilador responsável por detectar a situação que motivou sua produção.

Uma mesma situação não deverá produzir diagnósticos redundantes em diferentes fases do pipeline.

---

## 5.2 Diagnósticos Léxicos

A análise léxica poderá produzir diagnósticos relacionados às regras definidas para o processo de tokenização.

Esses diagnósticos pertencem exclusivamente ao **Documento 14 — Análise Léxica**.

---

## 5.3 Diagnósticos Sintáticos

O Parser poderá produzir diagnósticos relacionados às regras sintáticas da linguagem.

Esses diagnósticos pertencem exclusivamente ao **Documento 15 — Parser e AST**.

---

## 5.4 Diagnósticos Semânticos

As etapas da análise semântica poderão produzir diagnósticos relacionados às respectivas responsabilidades.

Entre elas incluem-se:

* Resolução de Nomes e Escopos;
* Inferência e Verificação de Tipos;
* Verificação Semântica.

Cada uma permanece responsável exclusivamente pelos diagnósticos definidos em sua própria especificação.

---

## 5.5 Diagnósticos do Middle-end

As etapas posteriores ao Frontend normalmente operam sobre representações previamente validadas.

Consequentemente, diagnósticos produzidos durante o Middle-end correspondem, em regra, a situações excepcionais relacionadas ao funcionamento interno do compilador e não a erros presentes no programa analisado.

---

## 5.6 Independência da Implementação

Os mecanismos utilizados para detectar situações, produzir diagnósticos e associá-los às diferentes etapas do compilador pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar os contratos definidos nesta especificação.

---

# 6. Organização dos Diagnósticos

Os diagnósticos produzidos durante a compilação deverão ser organizados de forma consistente, permitindo sua utilização pelas etapas posteriores do compilador e pelas ferramentas de desenvolvimento.

Este documento não estabelece uma estrutura obrigatória para esse armazenamento, definindo apenas os contratos arquiteturais que devem ser preservados.

---

## 6.1 Objetivo

A organização dos diagnósticos possui os seguintes objetivos:

* manter todos os diagnósticos produzidos durante a compilação;
* permitir sua recuperação pelas etapas posteriores;
* preservar informações de rastreabilidade;
* facilitar sua apresentação ao usuário.

---

## 6.2 Coleções

Uma implementação poderá manter os diagnósticos em uma ou mais coleções organizadas conforme sua arquitetura.

Essas coleções poderão ser estruturadas por diferentes critérios, como:

* fase da compilação;
* severidade;
* arquivo de origem;
* ordem de produção;
* outros critérios definidos pela implementação.

---

## 6.3 Ordenação

A ordem utilizada para apresentar os diagnósticos pertence exclusivamente à implementação.

Sempre que possível, essa ordenação deverá facilitar sua compreensão e localização pelo usuário.

---

## 6.4 Agrupamento

Uma implementação poderá agrupar diagnósticos relacionados sempre que isso contribuir para melhorar sua apresentação.

Os critérios utilizados para esse agrupamento pertencem exclusivamente à implementação.

---

## 6.5 Persistência

Os diagnósticos deverão permanecer disponíveis durante todo o processo de compilação enquanto forem necessários às etapas posteriores ou às ferramentas que utilizam suas informações.

A política utilizada para preservar ou descartar esses registros pertence exclusivamente à implementação.

---

## 6.6 Independência da Implementação

Os mecanismos utilizados para registrar, armazenar, organizar e recuperar diagnósticos pertencem exclusivamente à implementação.

Independentemente da arquitetura adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 7. Recuperação de Erros

A recuperação de erros permite que o compilador continue seu processamento após a identificação de determinadas situações, sempre que isso puder ser realizado com segurança.

Seu objetivo consiste em reduzir a propagação de erros decorrentes de uma mesma causa, permitindo que múltiplos diagnósticos relevantes sejam produzidos em uma única execução do compilador.

Este documento estabelece apenas os contratos arquiteturais da recuperação de erros, não especificando algoritmos nem estratégias concretas de implementação.

---

## 7.1 Objetivo

A recuperação de erros possui os seguintes objetivos:

* permitir a continuidade da compilação quando apropriado;
* reduzir o efeito cascata de diagnósticos;
* preservar a consistência das representações internas do compilador;
* aumentar a quantidade de informações úteis fornecidas ao usuário;
* apoiar as ferramentas de desenvolvimento.

---

## 7.2 Continuação da Compilação

Sempre que puder ser realizada sem comprometer a consistência da etapa atual, a compilação deverá prosseguir após a produção de um diagnóstico.

A continuidade do processamento não altera a validade do diagnóstico produzido nem elimina a necessidade de sua apresentação ao usuário.

A decisão de prosseguir ou interromper a compilação pertence exclusivamente à implementação.

---

## 7.3 Recuperação Léxica

A análise léxica poderá utilizar mecanismos de recuperação compatíveis com sua responsabilidade, permitindo a continuidade da tokenização após determinadas situações.

As estratégias utilizadas para essa recuperação pertencem exclusivamente ao **Documento 14 — Análise Léxica**.

A infraestrutura definida neste documento fornece os mecanismos comuns utilizados para registrar, organizar e apresentar os diagnósticos produzidos durante esse processo.

---

## 7.4 Recuperação Sintática

O Parser poderá utilizar mecanismos de recuperação destinados a restabelecer a análise sintática após determinadas inconsistências.

As estratégias utilizadas para essa recuperação pertencem exclusivamente ao **Documento 15 — Parser e AST**.

A infraestrutura definida neste documento fornece os mecanismos comuns utilizados para registrar, organizar e apresentar os diagnósticos produzidos durante esse processo.

---

## 7.5 Recuperação Semântica

As etapas da análise semântica poderão utilizar mecanismos de recuperação compatíveis com suas respectivas responsabilidades.

Entre elas incluem-se:

* Resolução de Nomes e Escopos;
* Inferência e Verificação de Tipos;
* Verificação Semântica.

Cada etapa permanece responsável pelos mecanismos utilizados para preservar a consistência da representação produzida durante sua execução.

---

## 7.6 Independência da Implementação

Os mecanismos utilizados para recuperar o processamento após uma situação diagnosticada pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar a consistência das representações internas do compilador e os contratos definidos nesta especificação.

---

# 8. Qualidade dos Diagnósticos

A utilidade de um diagnóstico depende não apenas da identificação correta da situação correspondente, mas também da qualidade das informações fornecidas ao usuário.

Os diagnósticos produzidos pelo compilador deverão ser claros, consistentes e suficientemente precisos para permitir a compreensão da situação identificada.

Este documento estabelece apenas os princípios arquiteturais relacionados à qualidade dos diagnósticos.

A infraestrutura definida neste documento fornece os mecanismos necessários para que os diagnósticos produzidos por todas as etapas do compilador mantenham um padrão consistente de clareza, precisão e rastreabilidade.

---

## 8.1 Clareza

Os diagnósticos deverão comunicar a situação identificada de forma clara e objetiva.

A redação utilizada para essa comunicação pertence exclusivamente à implementação.

---

## 8.2 Precisão

Sempre que possível, o diagnóstico deverá identificar com precisão a situação correspondente.

Essa precisão poderá abranger, entre outros aspectos:

* localização;
* elemento envolvido;
* contexto da ocorrência;
* demais informações relevantes.

---

## 8.3 Não Duplicação

Uma mesma situação não deverá produzir múltiplos diagnósticos redundantes sempre que isso puder ser evitado.

Caso diferentes etapas detectem consequências da mesma ocorrência, a implementação poderá adotar mecanismos destinados a reduzir diagnósticos repetitivos.

A estratégia utilizada para essa identificação pertence exclusivamente à implementação.

---

## 8.4 Localização

Sempre que aplicável, os diagnósticos deverão indicar a localização correspondente à situação identificada.

As informações utilizadas para essa localização deverão permanecer consistentes com os mecanismos de rastreabilidade adotados pelo compilador.

---

## 8.5 Consistência

Todos os diagnósticos produzidos pelo compilador deverão seguir uma arquitetura uniforme de representação.

Independentemente da etapa responsável por sua produção, deverão ser preservados:

* os contratos definidos neste documento;
* a consistência das informações apresentadas;
* a compatibilidade com as ferramentas de desenvolvimento.

---

## 8.6 Independência da Implementação

Os mecanismos utilizados para produzir diagnósticos claros, precisos e consistentes pertencem exclusivamente à implementação.

Este documento estabelece apenas os princípios arquiteturais que devem orientar sua qualidade.

---

# 9. Rastreabilidade

A rastreabilidade permite relacionar os diagnósticos produzidos durante a compilação às diferentes representações internas do programa.

Essa capacidade é fundamental para preservar a correspondência entre o código-fonte analisado, as representações produzidas pelo compilador e as informações apresentadas ao usuário.

Este documento estabelece apenas os contratos arquiteturais dessa rastreabilidade.

---

## 9.1 Objetivo

A rastreabilidade possui os seguintes objetivos:

* relacionar diagnósticos às representações internas;
* preservar a origem das informações utilizadas pelo compilador;
* apoiar ferramentas de desenvolvimento;
* facilitar a compreensão dos diagnósticos.

---

## 9.2 Localização

Sempre que aplicável, a rastreabilidade deverá preservar a correspondência entre o diagnóstico produzido e sua localização no código-fonte.

Os mecanismos utilizados para manter essa correspondência pertencem exclusivamente à implementação.

---

## 9.3 Histórico

Uma implementação poderá preservar informações que permitam acompanhar a evolução de um elemento ao longo das diferentes etapas do compilador.

A forma como esse histórico é mantido pertence exclusivamente à implementação.

---

## 9.4 Relação com as Representações

Sempre que aplicável, os diagnósticos poderão manter relações com as diferentes representações internas produzidas durante a compilação.

Entre elas incluem-se:

* tokens;
* AST;
* HIR;
* Intermediate Representation.

Essas representações são definidas pelos respectivos documentos da especificação: Documento 14 — Análise Léxica (tokens), Documento 15 — Parser e AST, Documento 16 — High-level Intermediate Representation (HIR) e Documento 21 — Intermediate Representation (IR).

Essa rastreabilidade facilita a localização da origem da situação diagnosticada sem alterar a independência entre as diferentes representações.

---

## 9.5 Diagnósticos Compostos

Uma implementação poderá associar múltiplas informações de rastreabilidade a um mesmo diagnóstico quando isso contribuir para sua compreensão.

Essas associações poderão envolver diferentes elementos relacionados à mesma situação identificada.

A estratégia utilizada para essas associações pertence exclusivamente à implementação.

---

## 9.6 Independência da Implementação

Os mecanismos utilizados para manter a rastreabilidade entre diagnósticos e representações internas pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar os contratos definidos nesta especificação e manter consistência entre os diagnósticos produzidos e as representações utilizadas pelo compilador.

---

# 10. Interface dos Diagnósticos

A infraestrutura de diagnósticos constitui um serviço compartilhado por todas as etapas do compilador.

Seu objetivo consiste em fornecer uma interface uniforme para produção, organização, recuperação e apresentação dos diagnósticos, independentemente da fase responsável por sua emissão.

Este documento estabelece apenas os contratos dessa interface, não impondo qualquer arquitetura específica para sua implementação.

A infraestrutura de diagnósticos definida neste documento constitui o serviço compartilhado utilizado por todas as etapas do compilador, fornecendo uma interface uniforme conforme os contratos arquiteturais aqui especificados.

---

## 10.1 Objetivo

A interface dos diagnósticos possui os seguintes objetivos:

* fornecer uma infraestrutura comum para todas as etapas do compilador;
* permitir tratamento uniforme dos diagnósticos;
* desacoplar a produção dos diagnósticos de sua apresentação;
* preservar consistência durante todo o pipeline de compilação.

---

## 10.2 Interface com o Frontend

As etapas do Frontend poderão produzir diagnósticos relacionados às suas respectivas responsabilidades.

A infraestrutura definida neste documento deverá permitir que esses diagnósticos sejam registrados, organizados e disponibilizados de maneira uniforme, independentemente da etapa que os produziu.

---

## 10.3 Interface com o Middle-end

As etapas do Middle-end normalmente operam sobre representações previamente validadas.

Consequentemente, diagnósticos produzidos durante essa fase correspondem, em regra, a situações excepcionais relacionadas ao funcionamento interno do compilador.

Esses diagnósticos deverão utilizar a mesma infraestrutura comum definida neste documento.

---

## 10.4 Interface com o Backend

O Backend poderá produzir diagnósticos relacionados às etapas de geração de código sempre que situações relevantes forem identificadas durante seu processamento.

Independentemente de sua origem, esses diagnósticos deverão utilizar a infraestrutura comum definida nesta especificação.

---

## 10.5 Interface com Ferramentas

Ferramentas de desenvolvimento poderão utilizar os diagnósticos produzidos pelo compilador para oferecer funcionalidades adicionais aos usuários.

Entre elas incluem-se, quando aplicável:

* ambientes integrados de desenvolvimento (IDEs);
* servidores de linguagem;
* analisadores estáticos;
* ferramentas de integração contínua;
* demais ferramentas compatíveis.

A forma como essas ferramentas consomem os diagnósticos pertence exclusivamente à implementação.

---

## 10.6 Independência da Implementação

Os mecanismos utilizados para disponibilizar diagnósticos às diferentes etapas do compilador e às ferramentas de desenvolvimento pertencem exclusivamente à implementação.

Independentemente da arquitetura adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 11. Apresentação dos Diagnósticos

A apresentação constitui a etapa responsável por disponibilizar os diagnósticos produzidos pelo compilador ao usuário ou às ferramentas de desenvolvimento.

Este documento estabelece apenas os contratos arquiteturais relacionados às informações que deverão estar disponíveis para apresentação, não definindo formatos específicos de saída.

---

## 11.1 Objetivo

A apresentação dos diagnósticos possui os seguintes objetivos:

* comunicar as situações identificadas durante a compilação;
* facilitar sua compreensão pelo usuário;
* preservar a consistência entre diferentes ferramentas;
* permitir diferentes formas de apresentação.

---

## 11.2 Conteúdo

Sempre que aplicável, a apresentação de um diagnóstico deverá disponibilizar informações suficientes para permitir sua compreensão.

Entre elas incluem-se, quando aplicável:

* severidade;
* localização;
* descrição da situação;
* contexto da ocorrência;
* demais informações relevantes.

A forma como essas informações são apresentadas pertence exclusivamente à implementação.

---

## 11.3 Destaques

Uma implementação poderá utilizar mecanismos destinados a destacar visualmente as informações mais relevantes de um diagnóstico.

Entre eles incluem-se, quando aplicável:

* realce da localização;
* marcação do trecho correspondente;
* diferenciação por severidade;
* demais recursos de apresentação.

Esses mecanismos pertencem exclusivamente à implementação.

---

## 11.4 Múltiplos Diagnósticos

Quando múltiplos diagnósticos forem produzidos durante uma mesma compilação, sua apresentação deverá preservar uma organização consistente.

Os critérios utilizados para ordenar ou agrupar essas informações pertencem exclusivamente à implementação.

---

## 11.5 Internacionalização

Uma implementação poderá oferecer diferentes idiomas para apresentação dos diagnósticos.

A infraestrutura definida neste documento permanece independente do idioma utilizado para comunicação com o usuário.

---

## 11.6 Independência da Implementação

Os mecanismos utilizados para apresentar diagnósticos pertencem exclusivamente à implementação.

Este documento estabelece apenas os contratos arquiteturais relacionados às informações que deverão estar disponíveis para apresentação.

---

# 12. Diagnósticos Internos

Além dos diagnósticos relacionados ao programa analisado, o compilador poderá identificar situações excepcionais decorrentes de falhas internas durante sua própria execução.

Esses diagnósticos não representam erros da linguagem nem do programa compilado, mas condições inesperadas encontradas pela implementação.

Seu tratamento deverá preservar a integridade das informações produzidas pelo compilador e auxiliar a identificação da causa da falha.

---

## 12.1 Objetivo

Os diagnósticos internos possuem os seguintes objetivos:

* identificar falhas da implementação;
* preservar informações relevantes para investigação;
* distinguir erros internos de erros do programa;
* apoiar a evolução do compilador.

---

## 12.2 Falhas Internas

Entre as situações que poderão produzir diagnósticos internos incluem-se, quando aplicável:

* inconsistências inesperadas das representações internas;
* violações dos contratos arquiteturais;
* condições impossíveis segundo a especificação;
* falhas internas da implementação;
* demais situações excepcionais.

---

## 12.3 Continuidade

Sempre que puder ser realizado com segurança, a implementação poderá prosseguir após identificar uma situação interna.

Quando isso não for possível, o processo de compilação poderá ser interrompido para preservar a consistência das representações internas.

A estratégia utilizada para essa decisão pertence exclusivamente à implementação.

---

## 12.4 Rastreabilidade

Sempre que possível, diagnósticos internos deverão preservar informações suficientes para permitir sua investigação.

Essas informações poderão incluir elementos das diferentes representações internas produzidas durante a compilação.

Os mecanismos utilizados para essa rastreabilidade pertencem exclusivamente à implementação.

---

## 12.5 Registros

Uma implementação poderá manter registros adicionais relacionados aos diagnósticos internos.

Esses registros poderão conter informações destinadas exclusivamente ao desenvolvimento e manutenção do compilador.

Sua estrutura e seu conteúdo pertencem exclusivamente à implementação.

---

## 12.6 Independência da Implementação

Os mecanismos utilizados para detectar, registrar e apresentar diagnósticos internos pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar os contratos definidos nesta especificação e distinguir claramente diagnósticos internos daqueles produzidos em decorrência de erros presentes no programa analisado.

---

# 13. Testabilidade

A infraestrutura de diagnósticos constitui um componente transversal do compilador e deverá ser validada de forma abrangente durante seu desenvolvimento.

Sua natureza compartilhada exige testes que assegurem a consistência dos diagnósticos produzidos por todas as etapas da compilação, bem como a correta recuperação do processamento sempre que aplicável.

A implementação deve manter uma suíte abrangente de testes automatizados, incluindo testes de regressão para cada erro identificado e posteriormente corrigido durante o desenvolvimento da infraestrutura de diagnósticos. Esses testes garantem que problemas anteriormente solucionados não sejam reintroduzidos em versões futuras do compilador.

---

## 13.1 Objetivo

Os testes relacionados à infraestrutura de diagnósticos possuem os seguintes objetivos:

* verificar a produção consistente de diagnósticos;
* validar os mecanismos de recuperação;
* assegurar a rastreabilidade das informações;
* detectar regressões;
* preservar os contratos definidos nesta especificação.

---

## 13.2 Testes Unitários

Cada componente da infraestrutura de diagnósticos deverá possuir testes específicos que validem seu comportamento isoladamente.

Entre eles incluem-se, quando aplicável:

* produção de diagnósticos;
* organização das informações;
* recuperação de erros;
* rastreabilidade;
* apresentação das informações;
* integração com as demais etapas do compilador.

---

## 13.3 Golden Tests

A implementação poderá utilizar *Golden Tests* para validar os diagnósticos produzidos durante diferentes situações de compilação.

Cada execução compara os diagnósticos produzidos com uma versão previamente validada, permitindo identificar alterações inesperadas decorrentes da evolução da implementação.

---

## 13.4 Testes de Recuperação

Os mecanismos de recuperação deverão ser validados por testes específicos que assegurem a continuidade da compilação sempre que isso puder ser realizado com segurança.

Esses testes deverão verificar, quando aplicável:

* preservação da consistência das representações internas;
* redução do efeito cascata de diagnósticos;
* manutenção da rastreabilidade;
* continuidade correta do processamento.

---

## 13.5 Testes de Regressão

Sempre que uma falha relacionada à produção, organização, recuperação ou apresentação de diagnósticos for corrigida, deverá ser incorporado um teste de regressão correspondente.

Esses testes contribuem para preservar a estabilidade da infraestrutura de diagnósticos ao longo da evolução do compilador.

---

## 13.6 Determinismo

Para um mesmo programa, utilizando a mesma versão do compilador e as mesmas opções de compilação, a infraestrutura de diagnósticos deverá produzir resultados funcionalmente equivalentes.

Esse princípio assegura previsibilidade para usuários, ferramentas de desenvolvimento e ambientes automatizados de integração contínua.

---

# 14. Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência da infraestrutura de diagnósticos utilizada pelo compilador oficial da linguagem Capi.

Seu papel consiste em fornecer um mecanismo uniforme para produção, organização, recuperação e apresentação de diagnósticos durante todas as etapas do processo de compilação.

---

## 14.1 Papel na Arquitetura do Compilador

A infraestrutura de diagnósticos constitui um serviço compartilhado por todo o compilador.

Ela fornece uma arquitetura comum utilizada pelas diferentes etapas do pipeline sem pertencer exclusivamente a nenhuma delas.

---

## 14.2 Relação com o Pipeline

Cada etapa do compilador permanece responsável pela identificação das situações relacionadas à sua própria atividade.

A infraestrutura de diagnósticos fornece os mecanismos comuns utilizados para registrar, organizar, recuperar e apresentar essas informações.

---

## 14.3 Relação com Ferramentas

Ferramentas de desenvolvimento poderão utilizar os diagnósticos produzidos pelo compilador para oferecer funcionalidades adicionais aos usuários.

A arquitetura definida neste documento fornece uma base uniforme para essa integração, independentemente da tecnologia utilizada pelas ferramentas.

---

## 14.4 Independência entre Implementações

Implementações distintas poderão adotar diferentes arquiteturas para representar e organizar seus diagnósticos.

Independentemente da estratégia utilizada, todas deverão preservar:

* os contratos definidos nesta especificação;
* a consistência das informações produzidas;
* a rastreabilidade dos diagnósticos;
* a independência entre as etapas do compilador.

---

## 14.5 Evolução Futura

A arquitetura definida neste documento foi concebida para permitir evolução contínua da infraestrutura de diagnósticos sem alterar seus contratos fundamentais.

Novas estratégias de recuperação, organização ou apresentação poderão ser incorporadas desde que preservem integralmente os princípios estabelecidos nesta especificação.

---

## 14.6 Síntese Arquitetural

A infraestrutura de diagnósticos representa um dos componentes compartilhados mais importantes do compilador.

Sua arquitetura estabelece um conjunto uniforme de contratos que permite integrar todas as etapas do pipeline de compilação, preservando consistência, rastreabilidade e independência entre as diferentes implementações.

---

# 15. Limites da Infraestrutura de Diagnósticos

A infraestrutura de diagnósticos possui responsabilidade exclusiva sobre os mecanismos comuns utilizados para representar, organizar, recuperar e apresentar diagnósticos produzidos durante o processo de compilação.

Ela não participa diretamente da análise da linguagem nem define as regras utilizadas para detectar cada situação específica, atuando apenas como serviço compartilhado pelas diferentes etapas do compilador.

Essa separação reduz o acoplamento entre os componentes do compilador e facilita sua evolução.

---

## 15.1 Objetivo

A definição dos limites da infraestrutura de diagnósticos possui os seguintes objetivos:

* preservar sua responsabilidade única;
* evitar sobreposição entre diferentes etapas do compilador;
* garantir previsibilidade arquitetural;
* facilitar a evolução da implementação.

---

## 15.2 O que Pertence à Infraestrutura de Diagnósticos

Compete exclusivamente à infraestrutura de diagnósticos:

* representar diagnósticos;
* organizar os diagnósticos produzidos;
* fornecer mecanismos de recuperação compatíveis com sua arquitetura;
* disponibilizar informações para apresentação;
* manter a rastreabilidade das informações produzidas.

---

## 15.3 O que Não Pertence à Infraestrutura de Diagnósticos

Não compete à infraestrutura de diagnósticos:

* definir as regras da linguagem;
* detectar erros específicos de cada etapa;
* executar análises léxicas, sintáticas ou semânticas;
* transformar representações internas do compilador;
* gerar código para plataformas específicas.

Essas responsabilidades pertencem às respectivas etapas do processo de compilação.

---

## 15.4 Relação com os Documentos Anteriores

A infraestrutura definida neste documento é utilizada pelas etapas especificadas nos documentos anteriores, preservando sua independência arquitetural.

Cada etapa permanece responsável pela identificação das situações relacionadas à sua própria atividade, utilizando a infraestrutura comum apenas para representar, organizar e apresentar os diagnósticos produzidos.

---

## 15.5 Relação com o Documento Seguinte

O documento seguinte, **24 — Backend e Geração de Código**, especifica a arquitetura responsável pela transformação da Intermediate Representation em código executável para a arquitetura de destino.

O Backend poderá utilizar a infraestrutura de diagnósticos definida neste documento sempre que precisar comunicar situações relevantes identificadas durante a geração de código.

---

## 15.6 Independência da Implementação

Os mecanismos utilizados para implementar a infraestrutura de diagnósticos pertencem exclusivamente à implementação.

Independentemente da arquitetura adotada, toda implementação deverá preservar integralmente os contratos definidos nesta especificação.

---

# 16. Princípios Arquiteturais

A infraestrutura de diagnósticos constitui um componente transversal da arquitetura do compilador, fornecendo um conjunto uniforme de contratos utilizados por todas as etapas do processo de compilação.

Sua arquitetura baseia-se em princípios que asseguram consistência, rastreabilidade, previsibilidade e independência entre as diferentes fases do compilador.

Esses princípios orientam todas as implementações compatíveis com esta especificação.

---

## 16.1 Responsabilidade Única

A infraestrutura de diagnósticos possui responsabilidade única: fornecer mecanismos comuns para produção, organização, recuperação e apresentação dos diagnósticos produzidos pelo compilador.

Não compete a esta infraestrutura detectar situações específicas da linguagem nem definir as regras pertencentes às diferentes etapas do pipeline.

---

## 16.2 Consistência

Todos os diagnósticos produzidos durante a compilação deverão obedecer a uma arquitetura uniforme de representação.

Independentemente da etapa responsável por sua emissão, deverão ser preservados:

* a consistência estrutural das informações;
* a uniformidade dos contratos;
* a compatibilidade com as ferramentas de desenvolvimento.

---

## 16.3 Determinismo

Para um mesmo programa, utilizando a mesma versão do compilador e as mesmas opções de compilação, a infraestrutura de diagnósticos deverá produzir resultados funcionalmente equivalentes.

Esse princípio assegura previsibilidade durante o desenvolvimento, os testes automatizados e a integração com ferramentas externas.

---

## 16.4 Recuperação Segura

Sempre que a continuidade da compilação puder ser realizada sem comprometer a consistência das representações internas, mecanismos de recuperação poderão ser utilizados.

A recuperação constitui um mecanismo destinado a aumentar a quantidade de informações úteis fornecidas ao usuário, sem comprometer a integridade do processo de compilação.

---

## 16.5 Separação de Responsabilidades

A arquitetura do compilador estabelece responsabilidades claramente definidas para cada etapa do pipeline.

Cada fase permanece responsável pela detecção das situações relacionadas à sua própria atividade, enquanto a infraestrutura definida neste documento fornece os mecanismos comuns utilizados para representar, organizar e apresentar os diagnósticos produzidos.

Essa separação reduz o acoplamento entre os componentes do compilador e facilita sua evolução.

---

## 16.6 Independência da Implementação

Os mecanismos utilizados para implementar a infraestrutura de diagnósticos pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, toda implementação deverá preservar integralmente os princípios arquiteturais definidos neste documento.

---

# 17. Considerações sobre a Arquitetura

A infraestrutura de diagnósticos representa um dos componentes compartilhados fundamentais da arquitetura do compilador.

Sua existência permite que todas as etapas do pipeline utilizem um conjunto comum de contratos para comunicação de problemas, preservando consistência e reduzindo o acoplamento entre os diferentes componentes da implementação.

---

## 17.1 Papel no Pipeline

A infraestrutura de diagnósticos acompanha todo o processo de compilação.

Cada etapa utiliza essa infraestrutura para registrar as situações identificadas durante sua execução, mantendo uma arquitetura uniforme para toda a cadeia de processamento.

---

## 17.2 Relação com o Frontend

Durante o Frontend, a infraestrutura de diagnósticos é utilizada pelas etapas de análise léxica, sintática e semântica para comunicar problemas relacionados ao programa analisado.

Cada uma dessas etapas permanece responsável pelos diagnósticos correspondentes à sua própria atividade.

---

## 17.3 Relação com o Middle-end e Backend

As etapas do Middle-end e do Backend também utilizam a infraestrutura definida neste documento sempre que precisarem produzir diagnósticos.

Nessas etapas, entretanto, os diagnósticos normalmente correspondem a situações excepcionais relacionadas ao funcionamento interno do compilador, uma vez que o programa já foi previamente validado pelo Frontend.

---

## 17.4 Estabilidade da Arquitetura

A infraestrutura de diagnósticos deverá permanecer estável ao longo da evolução do compilador.

Novas etapas poderão utilizar seus serviços sem necessidade de alterar os contratos arquiteturais definidos nesta especificação.

---

## 17.5 Evolução Futura

A arquitetura descrita neste documento foi concebida para permitir evolução contínua da infraestrutura de diagnósticos.

Novos mecanismos de recuperação, organização, rastreabilidade ou apresentação poderão ser incorporados desde que preservem integralmente os princípios estabelecidos nesta especificação.

---

## 17.6 Síntese Arquitetural

A infraestrutura de diagnósticos estabelece um conjunto uniforme de contratos utilizados por todas as etapas do compilador.

Sua arquitetura fornece independência entre os componentes da implementação, preservando consistência, rastreabilidade e previsibilidade durante todo o processo de compilação.

---

# 18. Considerações Finais

A infraestrutura de diagnósticos representa o mecanismo oficial de comunicação de problemas durante todo o processo de compilação da linguagem Capi.

Este documento estabelece os contratos arquiteturais responsáveis por sua produção, organização, recuperação e apresentação, permitindo que todas as etapas do compilador utilizem uma arquitetura uniforme para registrar e comunicar situações relevantes.

---

## 18.1 Síntese da Arquitetura

A infraestrutura de diagnósticos caracteriza-se pelos seguintes princípios:

* arquitetura compartilhada por todas as etapas do compilador;
* responsabilidade única para representação e organização dos diagnósticos;
* separação entre detecção e apresentação;
* suporte à recuperação de erros;
* preservação da rastreabilidade das informações.

Esses princípios asseguram que diferentes implementações possam adotar estratégias distintas preservando um conjunto comum de contratos arquiteturais.

---

## 18.2 Princípios Preservados

Toda implementação compatível com esta especificação deverá preservar:

* a consistência dos diagnósticos produzidos;
* a rastreabilidade das informações;
* a previsibilidade da infraestrutura;
* a separação entre responsabilidades;
* a independência entre as etapas do compilador.

Esses princípios garantem que diferentes implementações permaneçam compatíveis com a arquitetura definida para o compilador da linguagem Capi.

---

## 18.3 Relação com o Documento Seguinte

Este documento define a infraestrutura comum responsável pela produção, organização, recuperação e apresentação dos diagnósticos utilizados durante todo o processo de compilação.

O documento seguinte, **24 — Backend e Geração de Código**, especifica a arquitetura responsável por transformar a Intermediate Representation otimizada em código executável para a arquitetura de destino.

Sempre que necessário, o Backend utilizará a infraestrutura de diagnósticos definida neste documento para comunicar situações relevantes identificadas durante essa etapa da compilação.

---

## 18.4 Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência da infraestrutura de diagnósticos utilizada pelo compilador oficial da linguagem Capi.

Outras implementações poderão adotar diferentes estratégias para produção, organização, recuperação e apresentação de diagnósticos, desde que preservem integralmente:

* os contratos definidos nesta especificação;
* a consistência das informações produzidas;
* a rastreabilidade dos diagnósticos;
* a separação entre detecção e apresentação;
* a compatibilidade com as diferentes etapas do compilador.

Dessa forma, este documento estabelece a especificação arquitetural da infraestrutura de diagnósticos e recuperação de erros, fornecendo uma base comum para todo o processo de compilação da linguagem Capi.
