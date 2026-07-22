# 19 — Verificação Semântica

## Especificação de Implementação da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a especificação de implementação da fase de Verificação Semântica utilizada pelo compilador oficial da linguagem Capi.

Seu objetivo é estabelecer como a HIR enriquecida pela Resolução de Nomes e pela Inferência e Verificação de Tipos é validada em relação às regras semânticas da linguagem, garantindo que o programa esteja semanticamente correto antes do processo de Lowering.

A Verificação Semântica constitui a terceira e última etapa ativa da análise semântica do Frontend do compilador.

---

## 1.2 Escopo

Este documento especifica:

* o papel da Verificação Semântica na arquitetura do compilador;
* a validação das regras semânticas da linguagem;
* a verificação da consistência global da HIR;
* a produção de diagnósticos semânticos;
* a preparação da HIR para o processo de Lowering.

Não fazem parte deste documento:

* a Resolução de Nomes;
* a Inferência e Verificação de Tipos;
* o processo de Lowering;
* a Intermediate Representation (IR).

Esses assuntos são tratados em seus respectivos documentos da especificação de implementação.

---

## 1.3 Relação com os Documentos Anteriores

A Verificação Semântica atua diretamente sobre a HIR produzida pela Inferência e Verificação de Tipos definida no Documento 18.

Sua implementação deve preservar integralmente as garantias estabelecidas por:

| Documento    | Responsabilidade                  |
| ------------ | --------------------------------- |
| Documento 02 | Sistema de Tipos                  |
| Documento 04 | Sintaxe da Linguagem              |
| Documento 05 | Semântica Operacional             |
| Documento 13 | Estrutura do Compilador           |
| Documento 16 | Representação Semântica (HIR)     |
| Documento 17 | Resolução de Nomes e Escopos      |
| Documento 18 | Inferência e Verificação de Tipos |

Ao iniciar esta etapa, todas as referências do programa já se encontram resolvidas e todas as informações relacionadas ao Sistema de Tipos já foram determinadas.

Este documento especifica a validação das demais regras semânticas da linguagem antes da geração da Intermediate Representation (IR).

---

## 1.4 Filosofia da Implementação

A Verificação Semântica possui uma responsabilidade única: verificar que o programa respeita todas as regras semânticas da linguagem que não pertencem à Resolução de Nomes nem ao Sistema de Tipos.

Essa etapa não determina tipos, não resolve identificadores e não produz código intermediário.

Seu papel consiste exclusivamente em validar a consistência semântica da HIR antes da etapa de Lowering.

---

## 1.5 Separação entre Especificação e Implementação

Este documento define os contratos funcionais da Verificação Semântica, mas não impõe algoritmos específicos para sua implementação.

Uma implementação poderá utilizar, por exemplo:

* visitantes da HIR;
* mecanismos baseados em regras;
* validadores especializados;
* múltiplas passagens sobre a representação;
* outras estratégias equivalentes.

Independentemente da técnica adotada, todas as implementações deverão produzir resultados funcionalmente equivalentes para um mesmo programa válido e preservar os contratos definidos nesta especificação.

---

# 2. Papel da Verificação Semântica

A Verificação Semântica constitui a etapa responsável por concluir a análise semântica do programa.

Seu objetivo é validar todas as regras da linguagem que não pertencem à Resolução de Nomes nem ao Sistema de Tipos, assegurando que a HIR esteja completamente consistente antes do processo de Lowering.

Ao término desta etapa, toda a validação semântica do programa deverá estar concluída.

---

## 2.1 Entrada

A entrada da Verificação Semântica consiste na HIR produzida pela Inferência e Verificação de Tipos.

Nessa etapa:

* todos os identificadores já foram resolvidos;
* todas as informações de tipos já foram determinadas;
* todas as verificações relacionadas ao Sistema de Tipos já foram realizadas.

---

## 2.2 Saída

Ao término da Verificação Semântica, a HIR deverá representar um programa semanticamente consistente.

Quando inconsistências forem identificadas, deverão ser produzidos os diagnósticos correspondentes, preservando a continuidade da análise sempre que isso puder ser realizado com segurança.

Essa representação constitui a entrada para o processo de Lowering.

---

## 2.3 Responsabilidades

Compete à Verificação Semântica:

* validar as regras semânticas da linguagem;
* verificar a consistência global da HIR;
* identificar violações semânticas;
* produzir diagnósticos correspondentes;
* preparar a HIR para o processo de Lowering.

---

## 2.4 Limitações

Não compete à Verificação Semântica:

* resolver identificadores;
* determinar tipos;
* verificar compatibilidade entre tipos;
* gerar código intermediário;
* produzir a Intermediate Representation (IR).

Essas responsabilidades pertencem às etapas anteriores ou posteriores do pipeline do compilador.

---

## 2.5 Consolidação da HIR

A Verificação Semântica não modifica a estrutura conceitual da HIR.

Seu papel consiste em confirmar que a representação produzida pelas fases anteriores atende integralmente às regras semânticas da linguagem, consolidando-a como representação oficial do Frontend antes do processo de Lowering.

---

## 2.6 Independência Arquitetural

A Verificação Semântica deve permanecer completamente independente:

* da arquitetura de hardware;
* da ABI da plataforma;
* do Backend;
* da Intermediate Representation (IR);
* da geração de código.

Sua responsabilidade limita-se exclusivamente à validação semântica da HIR.

---

# 3. Fluxo da Verificação Semântica

A Verificação Semântica ocorre imediatamente após a Inferência e Verificação de Tipos.

Durante essa etapa, o compilador percorre a HIR enriquecida, aplica as regras semânticas da linguagem, identifica eventuais inconsistências e consolida a representação utilizada pelo Frontend.

Ao final do processo, a HIR encontra-se completamente validada e preparada para o Lowering.

---

## 3.1 Visão Geral

Conceitualmente, o fluxo da Verificação Semântica pode ser representado da seguinte forma:

```text
          HIR Tipada
              │
              ▼
 Identificação das Regras Aplicáveis
              │
              ▼
 Aplicação das Verificações Semânticas
              │
              ▼
 Produção de Diagnósticos
              │
              ▼
     Consolidação da HIR
              │
              ▼
      Lowering para IR
```

A HIR permanece como a representação oficial utilizada durante toda esta etapa.

---

## 3.2 Identificação das Regras Aplicáveis

O primeiro passo da Verificação Semântica consiste em identificar quais regras semânticas são aplicáveis a cada elemento presente na HIR.

Essas regras são determinadas pela especificação da linguagem e independem da estratégia adotada pela implementação.

---

## 3.3 Verificação das Regras

Após identificar as regras aplicáveis, a implementação verifica se cada elemento da HIR satisfaz os requisitos semânticos definidos pela linguagem.

Sempre que alguma dessas regras for violada, deverá ser produzido o diagnóstico correspondente.

Os mecanismos utilizados para realizar essa verificação pertencem exclusivamente à implementação.

Entre as verificações realizadas nesta etapa incluem-se, quando aplicável:

* regras de fluxo de controle;
* regras de utilização das construções da linguagem;
* restrições de contexto;
* demais verificações definidas pela Semântica Operacional.

A relação completa das verificações semânticas pertence à especificação da linguagem.

---

## 3.4 Produção de Diagnósticos

Durante a Verificação Semântica poderão ser identificadas inconsistências que impeçam a correta interpretação do programa.

Sempre que essas inconsistências forem detectadas, deverão ser produzidos diagnósticos suficientemente precisos para permitir sua identificação e correção.

A forma de apresentação desses diagnósticos pertence ao Documento 23 — Diagnósticos e Recuperação de Erros.

---

## 3.5 Consolidação da HIR

Ao término da Verificação Semântica, a HIR deverá representar um programa semanticamente validado.

Essa consolidação constitui o contrato entre o Frontend e o processo de Lowering, garantindo que as fases posteriores possam assumir a consistência semântica da representação.

---

## 3.6 Princípio da Responsabilidade Única

A Verificação Semântica possui responsabilidade única: validar as regras semânticas da linguagem que não pertencem à Resolução de Nomes nem ao Sistema de Tipos.

Ela não resolve identificadores, não realiza inferência de tipos e não produz a Intermediate Representation.

Seu resultado consiste exclusivamente em uma HIR semanticamente validada e pronta para o processo de Lowering.

---

# 4. Regras Semânticas

A Verificação Semântica tem como objetivo validar todas as regras da linguagem que não pertencem à Resolução de Nomes nem ao Sistema de Tipos.

Essas regras estabelecem restrições sobre o uso das construções da linguagem, garantindo que o programa seja semanticamente válido antes da geração da Intermediate Representation (IR).

A definição dessas regras pertence principalmente ao Documento 05 — Semântica Operacional.

---

## 4.1 Objetivo

As regras semânticas possuem os seguintes objetivos:

* validar o uso correto das construções da linguagem;
* preservar a consistência do comportamento do programa;
* impedir construções semanticamente inválidas;
* produzir diagnósticos quando necessário;
* preparar a HIR para o processo de Lowering.

A Verificação Semântica tem como objetivo validar todas as regras semânticas da linguagem que não pertencem à Resolução de Nomes nem ao Sistema de Tipos.

Essas regras são definidas principalmente pelo Documento 05 — Semântica Operacional.

---

## 4.2 Origem das Regras

As regras semânticas são definidas pela especificação da linguagem, especialmente pelo Documento 05 — Semântica Operacional.

Durante a Verificação Semântica, essas regras são aplicadas aos elementos presentes na HIR, independentemente da forma como a implementação realiza essa validação.

---

## 4.3 Natureza das Verificações

As verificações realizadas nesta etapa não determinam novos tipos nem resolvem referências.

Seu objetivo consiste exclusivamente em confirmar que o programa respeita as regras semânticas estabelecidas pela linguagem.

---

## 4.4 Aplicação das Regras

Cada elemento presente na HIR deverá ser validado conforme as regras semânticas aplicáveis ao seu contexto.

A ordem em que essas verificações são realizadas pertence exclusivamente à implementação, desde que o resultado funcional permaneça equivalente.

---

## 4.5 Consistência

Todas as regras semânticas aplicáveis a um elemento deverão ser consideradas antes de concluir sua validação.

Sempre que incompatibilidades forem identificadas, deverão ser produzidos os diagnósticos correspondentes.

---

## 4.6 Independência da Implementação

Os mecanismos utilizados para representar, organizar e aplicar as regras semânticas pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir resultados funcionalmente equivalentes para um mesmo programa.

---

# 5. Verificação das Declarações

Diversas declarações presentes na HIR possuem requisitos semânticos que vão além da simples resolução de nomes ou da verificação de tipos.

Compete à Verificação Semântica validar essas propriedades conforme definido pela especificação da linguagem.

---

## 5.1 Objetivo

A verificação das declarações possui os seguintes objetivos:

* validar a utilização correta das declarações;
* verificar restrições impostas pela linguagem;
* preservar a consistência semântica do programa;
* identificar violações das regras semânticas.

As regras semânticas aplicáveis às declarações são definidas principalmente pelo Documento 05 — Semântica Operacional, observadas também as construções sintáticas estabelecidas pelo Documento 04 — Sintaxe da Linguagem.

---

## 5.2 Declarações Válidas

Uma declaração é considerada semanticamente válida quando satisfaz todas as regras estabelecidas pela linguagem para sua categoria.

Essas regras são independentes da resolução de nomes e da verificação de tipos, já concluídas nas etapas anteriores.

---

## 5.3 Relações entre Declarações

A validade semântica de uma declaração poderá depender da relação estabelecida com outras declarações presentes no programa.

Sempre que essas relações forem relevantes, deverão ser verificadas conforme definido pela especificação da linguagem.

---

## 5.4 Restrições

Entre as restrições verificadas nesta etapa incluem-se, quando aplicável:

* regras de utilização;
* restrições de contexto;
* requisitos de visibilidade semântica;
* restrições definidas pela Semântica Operacional;
* demais regras previstas pela linguagem.

---

## 5.5 Diagnósticos

Sempre que uma declaração violar alguma regra semântica, deverá ser produzido o diagnóstico correspondente.

Sempre que possível, a análise deverá prosseguir normalmente para permitir a identificação de múltiplas inconsistências durante uma única compilação.

---

## 5.6 Independência da Implementação

Os mecanismos utilizados para validar declarações pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar os contratos estabelecidos nesta especificação.

---

# 6. Verificação das Expressões

As expressões representam construções que produzem valores ou participam do comportamento observável do programa.

Após a determinação dos tipos, compete à Verificação Semântica validar as regras relacionadas ao uso dessas expressões, conforme definido pela Semântica Operacional da linguagem.

As regras semânticas aplicáveis às expressões são definidas principalmente pelo Documento 05 — Semântica Operacional, observadas também as construções sintáticas estabelecidas pelo Documento 04 — Sintaxe da Linguagem.

---

## 6.1 Objetivo

A verificação das expressões possui os seguintes objetivos:

* validar seu uso correto;
* verificar restrições semânticas;
* preservar a consistência do comportamento do programa;
* produzir diagnósticos quando necessário.

---

## 6.2 Expressões Válidas

Uma expressão é considerada semanticamente válida quando satisfaz todas as regras definidas pela linguagem para sua construção e utilização.

A compatibilidade de tipos já foi validada pela etapa anterior.

---

## 6.3 Regras Semânticas

Entre as regras aplicáveis às expressões incluem-se, quando previsto pela linguagem:

* restrições de utilização;
* regras de avaliação;
* requisitos de contexto;
* condições definidas pela Semântica Operacional;
* demais verificações semânticas.

---

## 6.4 Restrições

As restrições semânticas das expressões deverão ser verificadas considerando seu contexto de utilização.

Nenhuma expressão poderá ser considerada válida apenas por possuir tipos compatíveis.

---

## 6.5 Diagnósticos

Sempre que uma expressão violar alguma regra semântica da linguagem, deverá ser produzido o diagnóstico correspondente.

Sempre que possível, os diagnósticos poderão incluir informações adicionais que auxiliem a identificação da construção responsável pela inconsistência. A geração dessas informações pertence exclusivamente à implementação.

---

## 6.6 Independência da Implementação

Os mecanismos utilizados para validar expressões pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir resultados funcionalmente equivalentes e preservar integralmente os contratos definidos nesta especificação.

---

# 7. Consistência Semântica

A Verificação Semântica deve assegurar que a HIR represente um programa semanticamente consistente após a conclusão de todas as verificações previstas pela linguagem.

Essa consistência complementa as garantias produzidas pela Resolução de Nomes e pela Inferência e Verificação de Tipos, concluindo a análise semântica do Frontend.

A consistência semântica complementa as garantias produzidas pela análise sintática (Documento 04 — Sintaxe da Linguagem) e pela Inferência e Verificação de Tipos (Documento 18), assegurando que o programa satisfaça também as regras definidas pelo Documento 05 — Semântica Operacional.

---

## 7.1 Objetivo

A consistência semântica possui os seguintes objetivos:

* garantir a validade semântica do programa;
* preservar a coerência entre os elementos da HIR;
* impedir construções semanticamente inválidas;
* preparar a representação para o processo de Lowering.

---

## 7.2 Consistência Global

A consistência global corresponde ao conjunto de propriedades que devem ser satisfeitas pelo programa como um todo.

Essas propriedades independem de elementos isolados e resultam da interação entre diferentes partes da HIR.

As regras que definem essa consistência pertencem à especificação da linguagem.

---

## 7.3 Consistência Local

Além das propriedades globais, cada elemento da HIR deverá satisfazer as regras semânticas aplicáveis ao seu próprio contexto.

A verificação local garante que cada construção individual permaneça semanticamente válida.

---

## 7.4 Relação entre Elementos

A validade semântica de um elemento poderá depender da existência ou do comportamento de outros elementos presentes na HIR.

Sempre que essas relações forem relevantes para a linguagem, deverão ser verificadas durante esta etapa.

A forma de representar essas relações pertence exclusivamente à implementação.

---

## 7.5 Inconsistências

Sempre que alguma regra semântica não puder ser satisfeita, deverá ser produzido o diagnóstico correspondente.

Sempre que possível, a análise deverá prosseguir normalmente para permitir que múltiplas inconsistências sejam identificadas durante uma única compilação.

---

## 7.6 Independência da Implementação

Os mecanismos utilizados para verificar a consistência semântica pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir resultados funcionalmente equivalentes para um mesmo programa.

---

# 8. Restrições Semânticas

A Verificação Semântica baseia-se em um conjunto de restrições estabelecidas pela especificação da linguagem.

Essas restrições determinam quais construções são semanticamente válidas e quais configurações devem ser rejeitadas durante a compilação.

Este documento define apenas o papel dessas restrições na arquitetura do compilador, não impondo qualquer modelo específico para sua representação interna.

---

## 8.1 Objetivo

As restrições semânticas possuem os seguintes objetivos:

* preservar a coerência da linguagem;
* impedir construções semanticamente inválidas;
* orientar a validação da HIR;
* produzir diagnósticos quando necessário;
* garantir a preparação da representação para o Lowering.

---

## 8.2 Regras Obrigatórias

As regras obrigatórias correspondem às restrições cuja violação torna o programa semanticamente inválido.

Essas regras decorrem das construções definidas pelo Documento 04 — Sintaxe da Linguagem e das regras de comportamento estabelecidas pelo Documento 05 — Semântica Operacional.

Toda implementação deverá verificar integralmente essas regras antes da conclusão da análise semântica.

---

## 8.3 Regras Específicas da Linguagem

Além das restrições gerais, a linguagem poderá definir regras semânticas específicas para determinadas construções.

Sempre que aplicáveis, essas regras deverão ser verificadas conforme estabelecido pela especificação da linguagem.

---

## 8.4 Acumulação das Restrições

Um mesmo elemento da HIR poderá estar sujeito simultaneamente a diversas restrições semânticas.

A implementação deverá considerar o conjunto completo das restrições aplicáveis antes de concluir sua validação.

A estratégia utilizada para organizar essas verificações pertence exclusivamente à implementação.

---

## 8.5 Violação das Restrições

Sempre que uma restrição obrigatória for violada, deverá ser produzido o diagnóstico correspondente.

Nenhuma implementação poderá considerar semanticamente válida uma construção que viole regras obrigatórias definidas pela linguagem.

---

## 8.6 Independência da Implementação

Os mecanismos utilizados para representar, organizar e verificar restrições semânticas pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar integralmente os contratos definidos nesta especificação.

---

# 9. HIR Validada

Ao término da Verificação Semântica, a HIR representa um programa completamente validado do ponto de vista semântico.

Essa representação constitui o resultado final do Frontend e estabelece o contrato com o processo de Lowering para a Intermediate Representation (IR).

---

## 9.1 Objetivo

A HIR validada possui os seguintes objetivos:

* representar um programa semanticamente consistente;
* preservar todas as informações produzidas pelas etapas anteriores;
* servir como entrada para o Lowering;
* manter a rastreabilidade com o código-fonte original.

---

## 9.2 Informações Consolidadas

Ao final desta etapa, a HIR deverá conter de forma consistente:

* referências resolvidas;
* informações de tipos;
* propriedades semânticas verificadas;
* diagnósticos produzidos durante a análise, quando aplicável;
* demais informações necessárias às fases posteriores.

---

## 9.3 Preservação da Estrutura

A Verificação Semântica não altera a estrutura conceitual da HIR.

Seu papel consiste exclusivamente em validar a representação produzida pelas fases anteriores e consolidar suas informações semânticas.

---

## 9.4 Estado Consistente

Uma HIR validada representa um estado semanticamente consistente da análise do programa.

Caso inconsistências sejam identificadas, elas deverão permanecer representadas de forma consistente, juntamente com os diagnósticos correspondentes, permitindo a continuidade do pipeline quando apropriado.

---

## 9.5 Preparação para o Lowering

A HIR produzida ao término desta etapa constitui a entrada do processo de Lowering definido no Documento 20.

O processo de Lowering poderá assumir que todas as verificações semânticas previstas pela linguagem já foram concluídas.

---

## 9.6 Independência da Implementação

Os mecanismos utilizados para consolidar a HIR pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir uma representação semanticamente equivalente e compatível com o processo de Lowering.

---

# 10. Interface da Verificação Semântica

A Verificação Semântica constitui a etapa final do Frontend responsável pela validação da HIR antes do processo de Lowering.

Sua interface estabelece o contrato entre a Inferência e Verificação de Tipos e o início do Middle-end, assegurando que a representação semântica esteja completamente validada.

Este documento define apenas o comportamento esperado dessa interface, não impondo qualquer estratégia específica para sua implementação.

---

## 10.1 Objetivo

A interface da Verificação Semântica possui os seguintes objetivos:

* receber a HIR enriquecida pela Inferência e Verificação de Tipos;
* validar todas as regras semânticas da linguagem;
* preservar a consistência da HIR;
* disponibilizar uma representação preparada para o processo de Lowering.

---

## 10.2 Entrada

A entrada da Verificação Semântica consiste na HIR produzida pela Inferência e Verificação de Tipos.

Essa representação contém:

* referências resolvidas;
* informações de tipos;
* propriedades do Sistema de Tipos;
* informações de rastreabilidade;
* demais informações produzidas pelas etapas anteriores.

A implementação poderá utilizar estruturas auxiliares adicionais, desde que preserve os contratos estabelecidos para a HIR.

---

## 10.3 Saída

Ao término da Verificação Semântica, a saída consiste em uma HIR semanticamente validada contendo:

* todas as verificações semânticas concluídas;
* propriedades semânticas consolidadas;
* diagnósticos produzidos durante esta etapa, quando aplicável;
* todas as informações necessárias para o processo de Lowering.

---

## 10.4 Relação com Diagnósticos

A Verificação Semântica poderá identificar situações como:

* violações das regras semânticas;
* utilização inválida de construções da linguagem;
* inconsistências de contexto;
* restrições semânticas não satisfeitas;
* demais violações previstas pela especificação.

Os mecanismos gerais para emissão, organização e recuperação de diagnósticos são definidos pelo Documento 23 — Diagnósticos e Recuperação de Erros.

---

## 10.5 Contrato entre Etapas

O processo de Lowering deve assumir que toda a validação semântica do programa já foi concluída.

Da mesma forma, a Verificação Semântica não deve antecipar transformações pertencentes ao processo de Lowering.

Essa separação reduz o acoplamento entre os componentes do compilador e preserva o Princípio da Responsabilidade Única.

---

## 10.6 Independência da Implementação

Os mecanismos utilizados para implementar a interface da Verificação Semântica pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar os contratos definidos neste documento e produzir comportamento funcionalmente equivalente.

---

# 11. Diagnósticos

Durante a Verificação Semântica poderão ser identificadas inconsistências relacionadas às regras semânticas da linguagem.

Sempre que essas inconsistências forem detectadas, deverão ser produzidos diagnósticos suficientemente precisos para permitir sua identificação e correção.

Os mecanismos gerais de emissão de diagnósticos pertencem ao Documento 23.

---

## 11.1 Objetivo

Os diagnósticos desta etapa possuem os seguintes objetivos:

* identificar violações das regras semânticas;
* localizar precisamente os elementos envolvidos;
* preservar a continuidade da análise;
* fornecer informações suficientes para a correção do programa.

Sempre que possível, os diagnósticos poderão incluir sugestões que auxiliem na correção do problema. A geração dessas sugestões pertence exclusivamente à implementação.

---

## 11.2 Violações Semânticas

Sempre que uma construção violar alguma regra semântica definida pela linguagem, deverá ser produzido o diagnóstico correspondente.

O diagnóstico deverá indicar, sempre que possível:

* os elementos envolvidos;
* a regra semântica violada;
* a localização correspondente no código-fonte.

---

## 11.3 Informações Obrigatórias

Todo diagnóstico produzido durante a Verificação Semântica deverá conter informações suficientes para permitir a identificação da inconsistência.

A forma como essas informações são representadas pertence exclusivamente à implementação.

---

## 11.4 Continuidade da Análise

Sempre que puder ser realizado com segurança, a Verificação Semântica deverá continuar sua execução após identificar inconsistências.

Essa continuidade permite que múltiplos problemas sejam identificados durante uma única compilação.

As estratégias de recuperação pertencem exclusivamente à implementação.

---

## 11.5 Relação com o Documento 23

Este documento especifica apenas os requisitos relacionados à identificação das inconsistências produzidas durante a Verificação Semântica.

A apresentação dos diagnósticos, sua organização, recuperação de erros e demais mecanismos gerais pertencem ao Documento 23 — Diagnósticos e Recuperação de Erros.

---

## 11.6 Independência da Implementação

Os mecanismos utilizados para produzir diagnósticos pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão identificar inconsistências equivalentes e produzir resultados funcionalmente equivalentes.

---

# 12. Consistência da HIR

Ao término da Verificação Semântica, a HIR deverá permanecer em um estado completamente consistente e preparada para o processo de Lowering.

Essa consistência representa o contrato final do Frontend com o Middle-end do compilador.

---

## 12.1 Objetivo

A consistência da HIR possui os seguintes objetivos:

* preservar a integridade da representação semântica;
* garantir que todas as verificações tenham sido concluídas;
* preparar a HIR para o processo de Lowering;
* manter a continuidade do pipeline do compilador.

---

## 12.2 Estado Inicial

Antes da Verificação Semântica, a HIR contém referências resolvidas e todas as informações relacionadas ao Sistema de Tipos.

Ainda poderão existir, entretanto, regras semânticas da linguagem que dependam de validação.

Essas verificações constituem o ponto de partida desta etapa.

---

## 12.3 Estado Final

Após concluída a Verificação Semântica, todas as regras semânticas obrigatórias deverão ter sido verificadas.

Toda inconsistência identificada deverá possuir o respectivo diagnóstico e permanecer representada de forma consistente para permitir a continuidade da compilação, quando aplicável.

---

## 12.4 Contrato Arquitetural

A Verificação Semântica estabelece o contrato arquitetural entre o Frontend e o processo de Lowering.

Esse contrato garante que:

* todas as verificações semânticas obrigatórias foram concluídas;
* todas as inconsistências identificadas possuem diagnóstico correspondente;
* a HIR permanece consistente;
* o processo de Lowering pode concentrar-se exclusivamente na transformação da representação semântica para a Intermediate Representation (IR).

---

## 12.5 Preparação para o Lowering

A HIR produzida ao final desta etapa constitui a entrada para o processo de Lowering.

As fases posteriores deverão utilizar diretamente essa representação, sem necessidade de repetir verificações semânticas pertencentes ao Frontend.

---

## 12.6 Independência da Implementação

Os mecanismos utilizados para preservar a consistência da HIR pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir uma HIR semanticamente consistente e funcionalmente equivalente ao término da Verificação Semântica.

---

# 13. Testabilidade

A Verificação Semântica constitui a etapa final da análise semântica do Frontend e desempenha papel fundamental na validação da consistência global do programa.

Seu comportamento determinístico permite a construção de testes independentes, garantindo elevado nível de confiabilidade e facilitando a evolução da implementação.

A implementação deve manter uma suíte abrangente de testes automatizados, incluindo testes de regressão para cada erro semântico identificado e posteriormente corrigido. Esses testes garantem que problemas anteriormente solucionados não sejam reintroduzidos em versões futuras do compilador.

---

## 13.1 Objetivo

Os testes da Verificação Semântica possuem os seguintes objetivos:

* validar as regras semânticas da linguagem;
* verificar a consistência da HIR;
* detectar regressões;
* garantir comportamento determinístico;
* assegurar a preparação da representação para o processo de Lowering.

---

## 13.2 Testes Unitários

Cada componente responsável pela Verificação Semântica deverá possuir testes específicos.

Entre eles incluem-se, quando aplicável:

* validação das regras semânticas;
* verificação das restrições da linguagem;
* consistência da HIR;
* emissão de diagnósticos;
* integração entre verificações relacionadas.

---

## 13.3 Golden Tests

A implementação poderá utilizar *Golden Tests* para validar a HIR produzida ao término da Verificação Semântica.

Cada execução compara a representação obtida com uma versão previamente validada, permitindo detectar alterações inesperadas nas propriedades semânticas registradas durante esta etapa.

---

## 13.4 Testes de Integração

Além dos testes unitários, deverão existir testes destinados a verificar a integração da Verificação Semântica com:

* a Inferência e Verificação de Tipos;
* o processo de Lowering;
* os mecanismos de diagnósticos.

Esses testes asseguram a continuidade arquitetural entre o Frontend e o Middle-end.

---

## 13.5 Benchmarks

A implementação poderá incluir benchmarks destinados a acompanhar o desempenho da Verificação Semântica.

Entre os aspectos monitorados encontram-se:

* tempo de validação;
* consumo de memória;
* desempenho em projetos de grande porte;
* impacto das verificações adicionais.

Esses benchmarks auxiliam a evolução da implementação sem alterar os contratos definidos nesta especificação.

---

## 13.6 Determinismo

Todos os testes relacionados à Verificação Semântica devem pressupor comportamento determinístico.

Para um mesmo programa, utilizando a mesma versão do compilador e as mesmas opções de compilação, as verificações realizadas e os diagnósticos produzidos deverão ser funcionalmente equivalentes.

---

# 14. Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência da Verificação Semântica do compilador oficial da linguagem Capi.

Seu papel consiste em concluir a análise semântica do programa antes do processo de Lowering para a Intermediate Representation (IR).

---

## 14.1 Papel no Frontend

Dentro do Frontend do compilador, a Verificação Semântica sucede a Inferência e Verificação de Tipos e representa a última etapa da análise semântica.

Sua responsabilidade consiste exclusivamente em validar as regras semânticas da linguagem e consolidar a HIR antes do início do Middle-end.

---

## 14.2 Relação com a HIR

A HIR permanece como a representação semântica oficial durante toda esta etapa.

A Verificação Semântica não modifica sua estrutura conceitual, limitando-se a validar as propriedades semânticas da representação produzida pelas fases anteriores.

---

## 14.3 Relação com o Lowering

O processo de Lowering recebe como entrada a HIR produzida ao término da Verificação Semântica.

Ele poderá assumir que todas as verificações obrigatórias da linguagem já foram realizadas, concentrando-se exclusivamente na transformação da HIR para a Intermediate Representation (IR).

---

## 14.4 Independência entre Implementações

Implementações distintas poderão utilizar estratégias diferentes para realizar a Verificação Semântica.

Independentemente da técnica adotada, todas deverão preservar:

* os contratos definidos neste documento;
* o comportamento determinístico;
* a consistência da HIR;
* a equivalência funcional dos resultados produzidos.

---

## 14.5 Evolução Futura

A arquitetura definida neste documento foi concebida para permitir evolução futura da implementação sem alterar os contratos estabelecidos para a Verificação Semântica.

Novas estratégias de validação, otimizações internas e melhorias de desempenho poderão ser incorporadas desde que preservem integralmente as garantias desta especificação.

---

## 14.6 Síntese Arquitetural

A Verificação Semântica representa a etapa responsável por concluir a validação do programa antes da geração da Intermediate Representation.

Ao término desta fase, a HIR encontra-se completamente validada e preparada para o processo de Lowering.

---

# 15. Limites da Verificação Semântica

A arquitetura do compilador da Capi adota uma divisão clara de responsabilidades entre as fases da análise semântica.

A Verificação Semântica constitui a etapa final desse processo, devendo concentrar-se exclusivamente na validação das regras semânticas que não pertencem às etapas anteriores nem às fases posteriores do pipeline.

Essa separação reduz o acoplamento entre os componentes do compilador e facilita sua evolução.

---

## 15.1 Objetivo

A definição dos limites da Verificação Semântica possui os seguintes objetivos:

* preservar a responsabilidade única da etapa;
* evitar sobreposição entre fases;
* garantir a previsibilidade do pipeline;
* facilitar a evolução da implementação.

---

## 15.2 O que Pertence a esta Etapa

Compete exclusivamente à Verificação Semântica:

* validar as regras semânticas da linguagem;
* verificar restrições semânticas;
* produzir diagnósticos relacionados a essas regras;
* consolidar a HIR como representação semanticamente válida.

---

## 15.3 O que Não Pertence a esta Etapa

Não compete à Verificação Semântica:

* resolver identificadores;
* determinar ou inferir tipos;
* verificar compatibilidade entre tipos;
* transformar a HIR em IR;
* gerar código.

Essas responsabilidades pertencem a outras fases do compilador.

---

## 15.4 Relação com o Documento 18

A Verificação Semântica assume que todas as informações relacionadas ao Sistema de Tipos já foram determinadas pela fase de Inferência e Verificação de Tipos, especificada no Documento 18.

Nenhuma responsabilidade pertencente àquele documento deverá ser repetida nesta etapa.

---

## 15.5 Relação com o Documento 20

Após a conclusão da Verificação Semântica, a HIR é entregue ao processo de Lowering definido no Documento 20.

Essa etapa não realiza transformações estruturais na representação, limitando-se a validar sua consistência semântica antes do início do Middle-end.

---

## 15.6 Independência da Implementação

Os mecanismos utilizados para delimitar as responsabilidades da Verificação Semântica pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar integralmente a separação de responsabilidades estabelecida nesta especificação.

---

# 16. Princípios Arquiteturais

A Verificação Semântica constitui a etapa final da análise semântica do Frontend e representa o último ponto de validação antes do início do Middle-end.

Sua arquitetura baseia-se em princípios que garantem previsibilidade, consistência e independência entre as diferentes fases do compilador.

Esses princípios orientam todas as implementações compatíveis com esta especificação.

---

## 16.1 Responsabilidade Única

A Verificação Semântica possui responsabilidade única: validar as regras semânticas da linguagem que não pertencem à Resolução de Nomes nem ao Sistema de Tipos.

Essa delimitação reduz o acoplamento entre componentes e simplifica a evolução do compilador.

---

## 16.2 Consistência Semântica

Toda implementação deverá assegurar que a HIR permaneça semanticamente consistente ao término desta etapa.

Nenhuma regra obrigatória da linguagem poderá permanecer sem verificação antes do processo de Lowering.

---

## 16.3 Determinismo

A Verificação Semântica deverá produzir resultados determinísticos.

Para um mesmo programa, utilizando a mesma versão do compilador e as mesmas opções de compilação, as verificações realizadas e os diagnósticos produzidos deverão ser funcionalmente equivalentes.

---

## 16.4 Evolução Incremental

A arquitetura foi concebida para permitir evolução incremental da implementação.

Novas estratégias de validação poderão ser incorporadas desde que preservem integralmente os contratos definidos nesta especificação.

---

## 16.5 Separação de Responsabilidades

Cada etapa do pipeline possui responsabilidades claramente definidas.

A Verificação Semântica não deve assumir responsabilidades pertencentes à Resolução de Nomes, à Inferência e Verificação de Tipos ou ao processo de Lowering.

Essa separação constitui um dos princípios fundamentais da arquitetura do compilador da Capi.

---

## 16.6 Independência da Implementação

Os mecanismos utilizados para implementar a Verificação Semântica pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar os princípios arquiteturais definidos neste documento.

---

# 17. Considerações sobre a Arquitetura

A arquitetura da Verificação Semântica representa a conclusão do Frontend do compilador da linguagem Capi.

Ao término desta etapa, a HIR encontra-se completamente validada, permitindo que o Middle-end concentre-se exclusivamente na transformação da representação semântica para a Intermediate Representation (IR).

---

## 17.1 Papel no Pipeline

Dentro do pipeline de compilação, a Verificação Semântica representa o último estágio dedicado à validação do programa.

Seu resultado estabelece a fronteira entre o Frontend e o Middle-end.

---

## 17.2 Relação com as Fases Anteriores

A Verificação Semântica depende integralmente das informações produzidas pelas fases anteriores.

Ela assume que:

* todos os identificadores foram resolvidos;
* todas as informações de tipos foram determinadas;
* todas as verificações relacionadas ao Sistema de Tipos foram concluídas.

Dessa forma, pode concentrar-se exclusivamente nas regras semânticas restantes.

---

## 17.3 Relação com as Fases Posteriores

As fases posteriores do compilador recebem como entrada uma HIR completamente validada.

O processo de Lowering pode assumir que nenhuma verificação semântica obrigatória permanece pendente, concentrando-se apenas na transformação da representação.

---

## 17.4 Estabilidade da HIR

Ao término da Verificação Semântica, a HIR representa a forma semântica definitiva do programa dentro do Frontend.

As fases posteriores poderão transformá-la para outras representações internas, mas não deverão alterar as garantias semânticas estabelecidas nesta etapa.

---

## 17.5 Evolução Futura

A arquitetura descrita neste documento foi projetada para permitir evolução futura da implementação sem alterar os contratos estabelecidos para a Verificação Semântica.

Melhorias internas, otimizações e novas estratégias de validação poderão ser incorporadas desde que preservem integralmente as garantias desta especificação.

---

## 17.6 Síntese Arquitetural

A Verificação Semântica conclui o processo de análise do programa iniciado pelo Frontend.

Seu resultado é uma HIR completamente validada, semanticamente consistente e preparada para servir como entrada do processo de Lowering.

---

# 18. Considerações Finais

A Verificação Semântica constitui a etapa responsável por concluir a validação semântica do programa no compilador da linguagem Capi.

Sua responsabilidade consiste em verificar todas as regras semânticas da linguagem que não pertencem à Resolução de Nomes nem ao Sistema de Tipos, consolidando a HIR como representação oficial do Frontend.

---

## 18.1 Síntese da Arquitetura

A Verificação Semântica é caracterizada pelos seguintes princípios:

* responsabilidade única;
* validação determinística;
* consistência semântica da HIR;
* separação entre análise e transformação;
* independência da implementação.

Esses princípios garantem previsibilidade, estabilidade e facilidade de evolução do compilador.

---

## 18.2 Princípios Preservados

Toda implementação compatível com esta especificação deverá preservar:

* equivalência funcional entre implementações;
* consistência semântica da HIR;
* comportamento determinístico;
* separação entre validação semântica e transformação da representação;
* independência entre Frontend e Middle-end.

Esses princípios asseguram interoperabilidade entre diferentes implementações do compilador.

---

## 18.3 Relação com o Documento Seguinte

Este documento conclui a análise semântica do Frontend, validando todas as regras semânticas da linguagem e consolidando a HIR como representação oficial do programa.

O documento seguinte, **20 — Lowering para IR**, especifica a transformação da HIR validada para a Intermediate Representation (IR), preservando todas as garantias semânticas estabelecidas pelas etapas anteriores e preparando o programa para o Middle-end do compilador.

A partir desse ponto, o pipeline deixa de realizar validações da linguagem e passa a concentrar-se exclusivamente na transformação e otimização da representação intermediária.

---

## 18.4 Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência da Verificação Semântica do compilador oficial da linguagem Capi.

Outras implementações poderão adotar estratégias diferentes para validar as regras semânticas e organizar seus mecanismos internos, desde que preservem integralmente:

* os contratos definidos neste documento;
* o comportamento determinístico;
* a consistência semântica da HIR;
* a separação entre validação semântica e transformação da representação;
* a compatibilidade com o processo de Lowering.

Dessa forma, este documento conclui a especificação da análise semântica do compilador da linguagem Capi e estabelece a base para o início do Middle-end, responsável pela transformação da HIR em Intermediate Representation (IR).
