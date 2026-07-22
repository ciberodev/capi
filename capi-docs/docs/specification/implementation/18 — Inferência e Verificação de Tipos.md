# 18 — Inferência e Verificação de Tipos

## Especificação de Implementação da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a especificação de implementação da fase de Inferência e Verificação de Tipos utilizada pelo compilador oficial da linguagem Capi.

Seu objetivo é estabelecer como os elementos presentes na Representação Semântica de Alto Nível (HIR) recebem informações de tipos, têm sua consistência verificada e são preparados para as etapas posteriores da análise semântica.

A Inferência e Verificação de Tipos constitui a segunda fase ativa da análise semântica do compilador, sucedendo a Resolução de Nomes e Escopos e antecedendo a Verificação Semântica.

---

## 1.2 Escopo

Este documento especifica:

* o papel da Inferência e Verificação de Tipos na arquitetura do compilador;
* os princípios gerais da inferência de tipos;
* a verificação da consistência entre tipos;
* o enriquecimento da HIR com informações de tipos;
* a interface com as fases posteriores do Frontend.

Não fazem parte deste documento:

* a Resolução de Nomes;
* a verificação das regras semânticas da linguagem;
* o processo de Lowering;
* a definição da Intermediate Representation (IR).

Esses assuntos são tratados em seus respectivos documentos da especificação de implementação.

---

## 1.3 Relação com os Documentos Anteriores

A Inferência e Verificação de Tipos atua diretamente sobre a HIR enriquecida pela Resolução de Nomes definida no Documento 17.

Sua implementação deve preservar integralmente as garantias estabelecidas por:

| Documento    | Responsabilidade              |
| ------------ | ----------------------------- |
| Documento 02 | Sistema de Tipos              |
| Documento 04 | Sintaxe da Linguagem          |
| Documento 05 | Semântica Operacional         |
| Documento 13 | Estrutura do Compilador       |
| Documento 16 | Representação Semântica (HIR) |
| Documento 17 | Resolução de Nomes e Escopos  |

O Documento 17 estabelece as associações entre referências e declarações.

Este documento especifica como essas associações são enriquecidas com informações de tipos, preparando a representação para a Verificação Semântica.

---

## 1.4 Filosofia da Implementação

A Inferência e Verificação de Tipos possui uma responsabilidade única: determinar e validar os tipos associados aos elementos da HIR.

Essa fase não resolve identificadores nem verifica regras semânticas que não estejam diretamente relacionadas ao sistema de tipos.

Seu papel consiste exclusivamente em enriquecer a representação semântica com informações de tipos e verificar sua consistência conforme o Sistema de Tipos definido para a linguagem.

---

## 1.5 Separação entre Especificação e Implementação

Este documento define os contratos funcionais da Inferência e Verificação de Tipos, mas não impõe algoritmos específicos para sua implementação.

Uma implementação poderá utilizar, por exemplo:

* propagação de tipos;
* sistemas de restrições;
* unificação;
* algoritmos de inferência;
* outras estratégias equivalentes.

Independentemente da técnica adotada, todas as implementações deverão produzir resultados funcionalmente equivalentes para um mesmo programa válido e preservar os contratos definidos nesta especificação.

---

# 2. Papel da Inferência e Verificação de Tipos

A Inferência e Verificação de Tipos constitui a segunda etapa da análise semântica do compilador.

Seu objetivo é determinar os tipos associados aos elementos da HIR, verificar sua compatibilidade e enriquecer a representação semântica com informações necessárias para as fases posteriores.

Ao término desta etapa, toda informação de tipos necessária para a análise semântica deverá estar disponível na HIR.

---

## 2.1 Entrada

A entrada da Inferência e Verificação de Tipos consiste na HIR produzida pela Resolução de Nomes.

Nessa etapa, todas as referências válidas já se encontram associadas às respectivas declarações, permitindo que a análise concentre-se exclusivamente no Sistema de Tipos.

---

## 2.2 Saída

Ao término da Inferência e Verificação de Tipos, a HIR deverá conter informações de tipos consistentes para todos os elementos tipáveis do programa.

Essa representação enriquecida constitui a entrada para a Verificação Semântica.

---

## 2.3 Responsabilidades

Compete à Inferência e Verificação de Tipos:

* determinar os tipos dos elementos da HIR;
* propagar informações de tipos quando necessário;
* verificar compatibilidade entre tipos;
* identificar inconsistências relacionadas ao Sistema de Tipos;
* enriquecer a HIR com informações tipadas.

A verificação das demais regras semânticas da linguagem pertence à etapa seguinte do Frontend.

---

## 2.4 Limitações

Não compete à Inferência e Verificação de Tipos:

* resolver identificadores;
* construir escopos;
* validar regras semânticas independentes do Sistema de Tipos;
* gerar código;
* produzir a Intermediate Representation (IR).

Essas responsabilidades pertencem às demais fases do compilador.

---

## 2.5 Enriquecimento da HIR

A Inferência e Verificação de Tipos não altera a estrutura conceitual da HIR.

Seu papel consiste em associar informações de tipos aos elementos já existentes, preservando a organização construída nas fases anteriores e preparando a representação para a Verificação Semântica.

---

## 2.6 Independência Arquitetural

A Inferência e Verificação de Tipos deve permanecer completamente independente:

* da geração de código;
* da arquitetura de hardware;
* da ABI da plataforma;
* do Backend;
* da Intermediate Representation (IR).

Sua responsabilidade limita-se exclusivamente ao enriquecimento da HIR com informações relacionadas ao Sistema de Tipos.

---

# 3. Fluxo da Inferência de Tipos

A Inferência e Verificação de Tipos ocorre imediatamente após a Resolução de Nomes.

Durante essa etapa, o compilador identifica os elementos tipáveis da HIR, determina seus tipos, verifica sua consistência e registra essas informações na representação semântica.

Ao final do processo, a HIR encontra-se preparada para a Verificação Semântica.

---

## 3.1 Visão Geral

Conceitualmente, o fluxo da Inferência e Verificação de Tipos pode ser representado da seguinte forma:

```text
        HIR com Nomes Resolvidos
                  │
                  ▼
    Identificação dos Elementos Tipáveis
                  │
                  ▼
         Inferência dos Tipos
                  │
                  ▼
 Verificação da Compatibilidade dos Tipos
                  │
                  ▼
      HIR Enriquecida com Tipos
                  │
                  ▼
      Verificação Semântica
```

A HIR permanece como a representação compartilhada entre todas as fases do Frontend.

---

## 3.2 Identificação dos Elementos Tipáveis

O primeiro passo da Inferência e Verificação de Tipos consiste em identificar todos os elementos da HIR que possuem ou dependem de informações de tipos.

Esses elementos incluem declarações, expressões e demais construções definidas pelo Sistema de Tipos da linguagem.

A estratégia utilizada para identificá-los pertence exclusivamente à implementação.

---

## 3.3 Inferência dos Tipos

Após identificar os elementos tipáveis, a implementação determina os tipos associados a cada um deles.

Essa determinação poderá utilizar informações explícitas presentes no programa, informações previamente registradas na HIR e relações estabelecidas pelo Sistema de Tipos.

Os mecanismos utilizados para realizar essa inferência pertencem exclusivamente à implementação.

---

## 3.4 Verificação da Compatibilidade

Uma vez determinados os tipos dos elementos do programa, a implementação verifica sua compatibilidade conforme as regras estabelecidas pelo Sistema de Tipos da linguagem.

Sempre que incompatibilidades forem identificadas, deverão ser produzidos os diagnósticos apropriados.

---

## 3.5 Enriquecimento da HIR

As informações produzidas durante a Inferência e Verificação de Tipos são incorporadas à HIR como informações semânticas adicionais.

Esse enriquecimento preserva a estrutura existente da representação e prepara o programa para a Verificação Semântica.

---

## 3.6 Princípio da Responsabilidade Única

A Inferência e Verificação de Tipos possui responsabilidade única: determinar e validar os tipos dos elementos presentes na HIR.

Ela não resolve identificadores, não interpreta regras semânticas que extrapolem o Sistema de Tipos e não modifica a estrutura lógica da HIR.

Seu resultado consiste exclusivamente em uma representação semanticamente enriquecida quanto às informações de tipos.

---

# 4. Tipos na HIR

Após a Resolução de Nomes, a HIR passa a conter todas as associações entre referências e declarações necessárias para a análise do Sistema de Tipos.

A Inferência e Verificação de Tipos utiliza essa representação para associar informações tipadas aos elementos do programa, preservando a estrutura semântica já estabelecida.

A HIR permanece como a representação oficial compartilhada entre todas as fases do Frontend.

---

## 4.1 Objetivo

A representação dos tipos na HIR possui os seguintes objetivos:

* associar informações de tipos aos elementos do programa;
* preservar a consistência do Sistema de Tipos;
* permitir a propagação de informações tipadas;
* servir como base para a Verificação Semântica;
* preparar a HIR para o processo de Lowering.

---

## 4.2 Tipos Conhecidos

Alguns elementos da HIR possuem tipos explicitamente declarados no código-fonte, enquanto outros terão seus tipos determinados durante a Inferência de Tipos.

Entre os elementos que podem possuir tipos explicitamente declarados incluem-se, quando aplicável:

* tipos declarados;
* parâmetros tipados;
* variáveis com tipo explícito;
* constantes tipadas;
* assinaturas de funções;
* demais construções previstas pelo Sistema de Tipos.

A HIR deverá preservar a distinção entre informações de tipo explicitamente declaradas e informações inferidas, permitindo que diagnósticos e ferramentas de desenvolvimento identifiquem a origem de cada informação tipada.

A definição desses tipos pertence ao Documento 02 — Sistema de Tipos.

---

## 4.3 Tipos Ainda Não Determinados

Outros elementos poderão iniciar esta etapa sem possuir um tipo completamente determinado.

Nesses casos, compete à Inferência e Verificação de Tipos determinar as informações necessárias para completar sua representação tipada.

Os mecanismos utilizados para essa determinação pertencem exclusivamente à implementação.

---

## 4.4 Associação entre Elementos e Tipos

Todo elemento tipável da HIR deverá manter associação explícita com as informações de tipos correspondentes.

Essa associação constitui parte integrante da representação semântica enriquecida e será utilizada pelas fases posteriores do Frontend.

A forma utilizada para representar essa associação pertence exclusivamente à implementação.

---

## 4.5 Rastreabilidade

As informações de tipos adicionadas à HIR deverão preservar a rastreabilidade com os elementos semânticos já existentes.

Essa propriedade permite que diagnósticos, ferramentas de desenvolvimento e fases posteriores da compilação identifiquem corretamente a origem das informações tipadas.

---

## 4.6 Independência da Implementação

Os mecanismos utilizados para representar informações de tipos na HIR pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir representações semanticamente equivalentes para um mesmo programa válido.

---

# 5. Inferência de Tipos

A Inferência de Tipos consiste no processo de determinar informações tipadas que não se encontram explicitamente especificadas no código-fonte, utilizando as regras definidas pelo Sistema de Tipos da linguagem.

Seu objetivo é completar a representação tipada da HIR antes da Verificação Semântica.

---

## 5.1 Objetivo

A Inferência de Tipos possui os seguintes objetivos:

* determinar tipos ausentes;
* propagar informações de tipos;
* reduzir redundâncias na escrita do código;
* preservar a consistência do Sistema de Tipos;
* enriquecer semanticamente a HIR.

---

## 5.2 Informações Utilizadas

A Inferência de Tipos poderá utilizar informações provenientes de:

* declarações previamente tipadas;
* assinaturas de funções;
* tipos conhecidos;
* expressões;
* restrições impostas pelo Sistema de Tipos;
* demais informações disponíveis na HIR.

As regras utilizadas para combinar essas informações pertencem ao Sistema de Tipos definido pela linguagem.

---

## 5.3 Propagação de Tipos

Durante a inferência, informações de tipos poderão propagar-se entre elementos semanticamente relacionados.

Essa propagação deverá preservar integralmente as regras estabelecidas pelo Documento 02 — Sistema de Tipos.

Os mecanismos utilizados para realizar essa propagação pertencem exclusivamente à implementação.

---

## 5.4 Restrições

Toda inferência deverá respeitar as restrições impostas pelo Sistema de Tipos.

Nenhuma inferência poderá produzir resultados incompatíveis com as garantias definidas pela linguagem.

Sempre que uma inferência válida não puder ser realizada, deverão ser produzidos os diagnósticos correspondentes.

---

## 5.5 Resultado da Inferência

Ao término do processo, todos os elementos cuja tipagem possa ser determinada deverão possuir informações de tipos consistentes registradas na HIR.

Quando a determinação do tipo não for possível ou resultar em inconsistências, essas situações deverão permanecer representadas de forma consistente para permitir a continuidade da análise, quando aplicável.

---

## 5.6 Independência da Implementação

Os algoritmos utilizados para inferir tipos pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir resultados funcionalmente equivalentes para um mesmo programa válido.

---

# 6. Restrições de Tipagem

A Inferência e Verificação de Tipos baseia-se em restrições estabelecidas pelo Sistema de Tipos da linguagem.

Essas restrições definem as relações que devem ser satisfeitas para que os elementos do programa possam ser considerados tipicamente consistentes.

Este documento especifica apenas o papel dessas restrições durante a análise semântica, não impondo qualquer modelo específico para sua representação ou processamento.

---

## 6.1 Objetivo

As restrições de tipagem possuem os seguintes objetivos:

* estabelecer relações entre tipos;
* orientar a inferência;
* validar compatibilidade entre elementos;
* identificar inconsistências;
* preservar as garantias do Sistema de Tipos.

---

## 6.2 Origem das Restrições

As restrições de tipagem são definidas pelo Documento 02 — Sistema de Tipos e aplicadas conforme as regras estabelecidas pelo Documento 05 — Semântica Operacional.

Durante a Inferência e Verificação de Tipos, essas restrições poderão ser originadas por diferentes construções da linguagem.

Entre elas incluem-se, quando aplicável:

* declarações;
* atribuições;
* expressões;
* chamadas de funções;
* operadores;
* demais construções previstas pelo Sistema de Tipos.

---

## 6.3 Acumulação das Restrições

Ao longo da Inferência e Verificação de Tipos, diferentes elementos da HIR poderão contribuir com novas restrições.

A implementação deverá considerar conjuntamente todas as restrições aplicáveis antes de concluir a determinação dos tipos correspondentes.

A estratégia utilizada para acumular essas informações pertence exclusivamente à implementação.

---

## 6.4 Consistência

O conjunto de restrições associado a um programa deverá permanecer consistente.

Sempre que restrições incompatíveis forem identificadas, deverão ser produzidos os diagnósticos apropriados, preservando a continuidade da análise sempre que isso puder ser realizado com segurança.

---

## 6.5 Restrições Incompatíveis

Quando duas ou mais restrições não puderem ser satisfeitas simultaneamente conforme o Sistema de Tipos da linguagem, a Inferência e Verificação de Tipos deverá considerar essa situação inconsistente.

Nenhuma solução arbitrária poderá ser adotada para eliminar incompatibilidades.

---

## 6.6 Independência da Implementação

Os mecanismos utilizados para representar, acumular e resolver restrições pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir resultados funcionalmente equivalentes e preservar integralmente os contratos definidos nesta especificação.

---

# 7. Compatibilidade de Tipos

A Inferência e Verificação de Tipos deve assegurar que todas as operações realizadas pelo programa envolvam tipos compatíveis conforme as regras definidas pelo Sistema de Tipos da linguagem.

Essa verificação garante que a HIR permaneça semanticamente consistente antes da etapa de Verificação Semântica.

A definição das regras de compatibilidade pertence ao Documento 02 — Sistema de Tipos.

---

## 7.1 Objetivo

A verificação da compatibilidade de tipos possui os seguintes objetivos:

* validar relações entre tipos;
* impedir operações incompatíveis;
* preservar as garantias do Sistema de Tipos;
* preparar a HIR para as fases posteriores;
* produzir diagnósticos precisos quando necessário.

---

## 7.2 Compatibilidade entre Tipos

A compatibilidade entre dois tipos deverá ser determinada exclusivamente pelas regras estabelecidas pelo Sistema de Tipos da linguagem.

A implementação não poderá introduzir critérios adicionais de compatibilidade além daqueles definidos pela especificação.

---

## 7.3 Tipos Compatíveis

Quando dois tipos forem considerados compatíveis pelas regras da linguagem, a Inferência e Verificação de Tipos deverá permitir sua utilização conjunta nas construções previstas.

A forma como essa compatibilidade é representada pertence exclusivamente à implementação.

---

## 7.4 Tipos Incompatíveis

Sempre que dois tipos incompatíveis forem utilizados em uma construção que exija compatibilidade, deverá ser produzido o diagnóstico correspondente.

Sempre que possível, a análise deverá prosseguir normalmente para permitir a identificação de múltiplos problemas em uma única compilação.

---

## 7.5 Preservação da Consistência

A verificação de compatibilidade não altera a estrutura da HIR.

Seu resultado consiste apenas na confirmação da consistência dos tipos envolvidos ou na produção dos diagnósticos apropriados.

---

## 7.6 Independência da Implementação

Os mecanismos utilizados para verificar compatibilidade entre tipos pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir resultados funcionalmente equivalentes para um mesmo programa.

---

# 8. Expressões Tipadas

Toda expressão presente na HIR possui um tipo associado, explícito ou inferido.

Compete à Inferência e Verificação de Tipos determinar esse tipo, verificar sua consistência e registrá-lo na representação semântica.

A determinação do tipo das expressões deverá respeitar integralmente o Sistema de Tipos e a Semântica Operacional definidos para a linguagem.

---

## 8.1 Objetivo

A tipagem das expressões possui os seguintes objetivos:

* determinar o tipo resultante de cada expressão;
* verificar a consistência entre operandos;
* preservar as regras do Sistema de Tipos;
* enriquecer a HIR com informações tipadas.

---

## 8.2 Determinação dos Tipos

O tipo de uma expressão poderá ser determinado a partir de:

* tipos explícitos;
* tipos inferidos;
* operadores utilizados;
* chamadas de funções;
* restrições do Sistema de Tipos;
* demais informações presentes na HIR.

As regras utilizadas para essa determinação pertencem ao Documento 02 — Sistema de Tipos.

---

## 8.3 Operadores

Os operadores definidos pela linguagem impõem restrições sobre os tipos dos operandos e sobre o tipo produzido como resultado.

A Inferência e Verificação de Tipos deverá aplicar essas restrições conforme estabelecido pelo Documento 05 — Semântica Operacional.

---

## 8.4 Chamadas de Funções

Durante a análise de chamadas de funções, a implementação deverá verificar a compatibilidade entre:

* os argumentos fornecidos;
* os parâmetros declarados;
* o tipo de retorno esperado;
* demais restrições definidas pela linguagem.

A interpretação dessas regras pertence ao Sistema de Tipos.

---

## 8.5 Expressões Compostas

Expressões compostas poderão combinar múltiplas subexpressões.

A Inferência e Verificação de Tipos deverá garantir que todas elas permaneçam compatíveis entre si e que o tipo resultante seja determinado conforme as regras da linguagem.

---

## 8.6 Independência da Implementação

Os mecanismos utilizados para determinar os tipos das expressões pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir tipos funcionalmente equivalentes para expressões semanticamente equivalentes.

---

# 9. Tipagem de Declarações

Além das expressões, diversos elementos declarativos do programa possuem informações de tipos que devem ser verificadas durante esta etapa.

A Inferência e Verificação de Tipos deverá validar essas declarações conforme as regras estabelecidas pelo Sistema de Tipos da linguagem.

---

## 9.1 Objetivo

A tipagem das declarações possui os seguintes objetivos:

* validar declarações tipadas;
* determinar tipos ausentes quando permitido;
* preservar a consistência das declarações;
* enriquecer semanticamente a HIR.

---

## 9.2 Variáveis

Toda variável deverá possuir um tipo determinado ao término desta etapa.

Esse tipo poderá ser explícito ou inferido, conforme permitido pela linguagem.

Em ambos os casos, deverá respeitar integralmente o Sistema de Tipos.

---

## 9.3 Constantes

As constantes deverão possuir tipos compatíveis com seus respectivos valores e com as regras estabelecidas pela linguagem.

Sempre que incompatibilidades forem identificadas, deverão ser produzidos os diagnósticos correspondentes.

---

## 9.4 Parâmetros

Os parâmetros declarados em funções ou outras construções equivalentes deverão possuir tipos consistentes com sua utilização.

A Inferência e Verificação de Tipos deverá validar essas informações antes da Verificação Semântica.

---

## 9.5 Funções

As funções deverão apresentar consistência entre:

* tipos dos parâmetros;
* tipo de retorno;
* restrições impostas pelo Sistema de Tipos;
* demais propriedades definidas pela linguagem.

A verificação do comportamento interno da função pertence à Verificação Semântica.

---

## 9.6 Independência da Implementação

Os mecanismos utilizados para validar a tipagem das declarações pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir resultados funcionalmente equivalentes e preservar integralmente os contratos definidos nesta especificação.

---

# 10. Verificação de Tipos

Após a determinação dos tipos dos elementos presentes na HIR, a Inferência e Verificação de Tipos deve validar que todas as relações estabelecidas pelo programa respeitam as regras do Sistema de Tipos da linguagem.

Essa etapa confirma que a representação semântica enriquecida permanece consistente e apta a prosseguir para a Verificação Semântica.

---

## 10.1 Objetivo

A Verificação de Tipos possui os seguintes objetivos:

* validar a consistência dos tipos determinados;
* identificar incompatibilidades entre elementos;
* preservar as garantias do Sistema de Tipos;
* produzir diagnósticos precisos;
* preparar a HIR para a etapa seguinte da análise semântica.

---

## 10.2 Verificações Obrigatórias

A Inferência e Verificação de Tipos deverá realizar todas as verificações exigidas pelo Sistema de Tipos da linguagem.

Entre elas incluem-se, quando aplicável:

* compatibilidade entre atribuições;
* compatibilidade entre operandos;
* compatibilidade entre argumentos e parâmetros;
* compatibilidade entre retornos e assinaturas;
* demais verificações definidas pelo Sistema de Tipos.

Caso a linguagem venha a admitir mecanismos de resolução entre múltiplas assinaturas compatíveis, essa resolução deverá respeitar integralmente as regras definidas pelo Sistema de Tipos. A forma de implementação pertence exclusivamente à implementação.

As regras específicas pertencem ao Documento 02 — Sistema de Tipos.

---

## 10.3 Operações Tipadas

Toda operação realizada pelo programa deverá respeitar as restrições impostas pelos tipos envolvidos.

A Inferência e Verificação de Tipos deverá verificar que cada operação é válida conforme as regras definidas pelo Documento 05 — Semântica Operacional.

---

## 10.4 Compatibilidade entre Atribuições

Sempre que um valor for atribuído a outro elemento do programa, deverá ser verificada a compatibilidade entre o tipo do valor produzido e o tipo esperado pelo destino da atribuição.

Sempre que essa compatibilidade não puder ser estabelecida, deverá ser produzido o diagnóstico correspondente.

---

## 10.5 Consistência dos Tipos

Ao término desta etapa, todas as informações de tipos registradas na HIR deverão permanecer consistentes.

Nenhuma inconsistência conhecida poderá permanecer sem o respectivo diagnóstico.

---

## 10.6 Independência da Implementação

Os mecanismos utilizados para verificar a consistência dos tipos pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir resultados funcionalmente equivalentes e preservar os contratos definidos nesta especificação.

---

# 11. Capacidade de Mutação

Além da compatibilidade entre tipos, a Inferência e Verificação de Tipos deverá preservar as propriedades associadas à Capacidade de Mutação definidas pelo Sistema de Tipos da linguagem.

Essa verificação assegura que operações de leitura e escrita respeitem as restrições estabelecidas para cada elemento do programa.

---

## 11.1 Objetivo

A verificação da Capacidade de Mutação possui os seguintes objetivos:

* preservar as garantias do Sistema de Tipos;
* validar operações de escrita;
* impedir modificações incompatíveis;
* enriquecer semanticamente a HIR.

A Capacidade de Mutação é definida pelo Documento 02 — Sistema de Tipos e sua aplicação operacional é regulada pelo Documento 05 — Semântica Operacional, que estabelece as condições sob as quais operações de escrita são permitidas.

---

## 11.2 Informações Provenientes do Sistema de Tipos

As propriedades relacionadas à Capacidade de Mutação são definidas pelo Documento 02 — Sistema de Tipos.

A Inferência e Verificação de Tipos deverá utilizar essas informações durante a análise sempre que forem relevantes para determinar a validade das operações realizadas pelo programa.

---

## 11.3 Operações de Escrita

Sempre que uma construção da linguagem realizar uma operação de escrita sobre um elemento do programa, deverá ser verificada sua compatibilidade com a Capacidade de Mutação associada ao elemento correspondente.

As regras específicas pertencem ao Sistema de Tipos da linguagem.

---

## 11.4 Restrições

As restrições impostas pela Capacidade de Mutação deverão ser consideradas juntamente com as demais restrições do Sistema de Tipos.

Nenhuma operação incompatível poderá ser considerada válida apenas por possuir tipos compatíveis.

---

## 11.5 Diagnósticos

Sempre que uma operação violar as regras relacionadas à Capacidade de Mutação, deverá ser produzido o diagnóstico correspondente.

Sempre que possível, a análise deverá prosseguir normalmente para permitir a identificação de múltiplos problemas durante uma única compilação.

---

## 11.6 Independência da Implementação

Os mecanismos utilizados para representar e verificar a Capacidade de Mutação pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar as garantias estabelecidas pelo Sistema de Tipos da linguagem.

---

# 12. HIR Enriquecida

Ao término da Inferência e Verificação de Tipos, a HIR passa a representar o programa com todas as informações de tipos necessárias para a continuação da análise semântica.

Essa representação constitui o principal resultado desta etapa e estabelece o contrato com a Verificação Semântica.

---

## 12.1 Objetivo

O enriquecimento da HIR possui os seguintes objetivos:

* registrar as informações de tipos determinadas;
* preservar a consistência da representação semântica;
* disponibilizar informações para as fases posteriores;
* manter a rastreabilidade com os elementos do programa.

---

## 12.2 Informações Adicionadas

Entre as informações que poderão ser associadas aos elementos da HIR incluem-se, quando aplicável:

* tipos determinados;
* informações de compatibilidade;
* propriedades do Sistema de Tipos;
* atributos relacionados à Capacidade de Mutação;
* demais informações necessárias às fases posteriores.

A representação concreta dessas informações pertence exclusivamente à implementação.

---

## 12.3 Preservação da Estrutura

A Inferência e Verificação de Tipos não modifica a estrutura conceitual da HIR.

Seu papel limita-se ao enriquecimento dos elementos existentes com novas informações semânticas relacionadas ao Sistema de Tipos.

---

## 12.4 Consistência

Ao término desta etapa, todas as informações tipadas registradas na HIR deverão permanecer consistentes entre si.

Essa consistência constitui um dos contratos fundamentais entre a Inferência e Verificação de Tipos e a Verificação Semântica.

---

## 12.5 Preparação para a Verificação Semântica

A HIR enriquecida produzida nesta etapa constitui a entrada para a Verificação Semântica.

As fases posteriores deverão utilizar diretamente as informações de tipos registradas nesta etapa, sem necessidade de repetir o processo de inferência.

---

## 12.6 Independência da Implementação

Os mecanismos utilizados para enriquecer a HIR pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar a equivalência funcional da representação semântica produzida ao término da Inferência e Verificação de Tipos.

---

# 13. Interface da Inferência e Verificação de Tipos

A Inferência e Verificação de Tipos constitui uma etapa intermediária da análise semântica do Frontend.

Sua interface estabelece o contrato entre a HIR enriquecida pela Resolução de Nomes e a HIR tipada que será utilizada pela Verificação Semântica.

Este documento define apenas o comportamento esperado dessa interface, não impondo qualquer estratégia específica para sua implementação.

---

## 13.1 Objetivo

A interface da Inferência e Verificação de Tipos possui os seguintes objetivos:

* receber a HIR produzida pela Resolução de Nomes;
* enriquecer a representação com informações de tipos;
* preservar a consistência da HIR;
* disponibilizar uma representação preparada para a Verificação Semântica.

---

## 13.2 Entrada

A entrada da Inferência e Verificação de Tipos consiste na HIR enriquecida pela Resolução de Nomes.

Essa representação contém:

* referências resolvidas;
* declarações identificadas;
* estrutura de escopos;
* informações de rastreabilidade;
* demais informações semânticas produzidas nas fases anteriores.

A implementação poderá utilizar estruturas auxiliares adicionais, desde que preserve os contratos estabelecidos para a HIR.

---

## 13.3 Saída

Ao término da Inferência e Verificação de Tipos, a saída consiste em uma HIR enriquecida contendo:

* informações de tipos;
* associações entre elementos e seus respectivos tipos;
* resultados da verificação de compatibilidade;
* propriedades relevantes do Sistema de Tipos;
* diagnósticos produzidos durante esta etapa, quando aplicável.

---

## 13.4 Relação com Diagnósticos

A Inferência e Verificação de Tipos poderá identificar situações como:

* incompatibilidade entre tipos;
* inferência impossível;
* violações das regras do Sistema de Tipos;
* incompatibilidades relacionadas à Capacidade de Mutação;
* demais inconsistências tipadas.

Os mecanismos gerais para emissão, organização e recuperação de diagnósticos são definidos pelo Documento 23 — Diagnósticos e Recuperação de Erros.

---

## 13.5 Contrato entre Etapas

A Verificação Semântica deve assumir que toda informação relacionada ao Sistema de Tipos já se encontra corretamente determinada e registrada na HIR.

Da mesma forma, a Inferência e Verificação de Tipos não deve antecipar verificações que pertençam exclusivamente à Verificação Semântica.

Essa separação reduz o acoplamento entre os componentes do compilador e preserva o Princípio da Responsabilidade Única.

---

## 13.6 Independência da Implementação

Os mecanismos utilizados para implementar a interface da Inferência e Verificação de Tipos pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão preservar os contratos definidos neste documento e produzir comportamento funcionalmente equivalente.

---

# 14. Diagnósticos

Durante a Inferência e Verificação de Tipos poderão ser identificadas inconsistências relacionadas ao Sistema de Tipos da linguagem.

Sempre que essas inconsistências forem detectadas, deverão ser produzidos diagnósticos suficientemente precisos para permitir sua identificação e correção.

Os mecanismos gerais de emissão de diagnósticos pertencem ao Documento 23.

---

## 14.1 Objetivo

Os diagnósticos desta etapa possuem os seguintes objetivos:

* identificar inconsistências relacionadas aos tipos;
* localizar precisamente os elementos envolvidos;
* preservar a continuidade da análise;
* fornecer informações suficientes para correção do programa.

Sempre que possível, os diagnósticos poderão incluir sugestões que auxiliem na correção do problema. A geração dessas sugestões pertence exclusivamente à implementação.

---

## 14.2 Tipos Incompatíveis

Sempre que dois ou mais elementos forem utilizados em uma construção que exija compatibilidade entre seus tipos e essa compatibilidade não puder ser estabelecida, deverá ser produzido o diagnóstico correspondente.

O diagnóstico deverá indicar, sempre que possível:

* os elementos envolvidos;
* os tipos determinados;
* a localização correspondente no código-fonte.

---

## 14.3 Inferência Impossível

Quando as informações disponíveis não forem suficientes para determinar o tipo de determinado elemento conforme as regras do Sistema de Tipos, deverá ser produzido o diagnóstico correspondente.

Essa situação não poderá ser resolvida por decisões arbitrárias da implementação.

---

## 14.4 Continuidade da Análise

Sempre que puder ser realizado com segurança, a Inferência e Verificação de Tipos deverá continuar sua execução após identificar inconsistências.

Essa continuidade permite que múltiplos problemas sejam identificados em uma única compilação.

As estratégias de recuperação pertencem exclusivamente à implementação.

---

## 14.5 Relação com o Documento 23

Este documento especifica apenas os requisitos relacionados à identificação das inconsistências produzidas durante a Inferência e Verificação de Tipos.

A apresentação dos diagnósticos, sua organização, recuperação de erros e demais mecanismos gerais pertencem ao Documento 23 — Diagnósticos e Recuperação de Erros.

---

## 14.6 Independência da Implementação

Os mecanismos utilizados para produzir diagnósticos pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão identificar inconsistências equivalentes e produzir resultados funcionalmente equivalentes.

---

# 15. Consistência da HIR

Ao término da Inferência e Verificação de Tipos, a HIR deverá permanecer em um estado consistente e preparada para utilização pela Verificação Semântica.

Essa consistência constitui um dos contratos fundamentais entre a Inferência e Verificação de Tipos e as fases posteriores do Frontend.

---

## 15.1 Objetivo

A consistência da HIR possui os seguintes objetivos:

* preservar a integridade da representação semântica;
* garantir a consistência das informações de tipos;
* preparar a HIR para a Verificação Semântica;
* manter a continuidade do pipeline do compilador.

---

## 15.2 Estado Inicial

Antes da Inferência e Verificação de Tipos, a HIR contém referências resolvidas, porém ainda pode possuir elementos sem informações completas de tipos.

Esses elementos representam o ponto de partida para a inferência e validação tipada.

---

## 15.3 Estado Final

Após concluída esta etapa, todos os elementos cuja tipagem possa ser determinada deverão possuir informações de tipos consistentes registradas na HIR.

Inconsistências relacionadas ao Sistema de Tipos deverão possuir os respectivos diagnósticos e permanecer representadas de forma consistente para permitir a continuidade da análise, quando aplicável.

---

## 15.4 Contrato Arquitetural

A Inferência e Verificação de Tipos estabelece um contrato arquitetural com a Verificação Semântica.

Esse contrato garante que:

* todas as informações de tipos estejam determinadas quando possível;
* todas as incompatibilidades identificadas possuam diagnóstico correspondente;
* a HIR permaneça consistente;
* a Verificação Semântica possa concentrar-se exclusivamente nas regras semânticas da linguagem.

---

## 15.5 Preparação para a Verificação Semântica

A HIR produzida ao final desta etapa constitui a entrada para a Verificação Semântica.

As fases posteriores deverão utilizar diretamente as informações de tipos registradas durante a Inferência e Verificação de Tipos, sem necessidade de repetir esse processo.

---

## 15.6 Independência da Implementação

Os mecanismos utilizados para preservar a consistência da HIR pertencem exclusivamente à implementação.

Independentemente da estratégia adotada, todas as implementações deverão produzir uma HIR semanticamente consistente e funcionalmente equivalente ao término da Inferência e Verificação de Tipos.

---

# 16. Testabilidade

A Inferência e Verificação de Tipos constitui uma das etapas centrais da análise semântica do compilador.

Seu comportamento determinístico permite a construção de testes independentes, garantindo elevado nível de confiabilidade e facilitando a evolução da implementação.

A implementação deve manter uma suíte abrangente de testes automatizados, incluindo testes de regressão para cada erro relacionado à Inferência e Verificação de Tipos identificado e posteriormente corrigido. Esses testes garantem que problemas anteriormente solucionados não sejam reintroduzidos em versões futuras do compilador.

---

## 16.1 Objetivo

Os testes da Inferência e Verificação de Tipos possuem os seguintes objetivos:

* validar a inferência de tipos;
* verificar a compatibilidade entre tipos;
* detectar regressões;
* garantir comportamento determinístico;
* assegurar a consistência da HIR produzida.

---

## 16.2 Testes Unitários

Cada componente responsável pela Inferência e Verificação de Tipos deverá possuir testes específicos.

Entre eles incluem-se, quando aplicável:

* inferência de tipos;
* propagação de tipos;
* verificação de compatibilidade;
* validação das restrições de tipagem;
* verificação da Capacidade de Mutação.

---

## 16.3 Golden Tests

A implementação poderá utilizar *Golden Tests* para validar a HIR enriquecida produzida após a Inferência e Verificação de Tipos.

Cada execução compara a representação produzida com uma versão previamente validada, permitindo detectar alterações inesperadas nas informações de tipos registradas durante esta etapa.

---

## 16.4 Testes de Integração

Além dos testes unitários, deverão existir testes destinados a verificar a integração da Inferência e Verificação de Tipos com:

* a Resolução de Nomes;
* a Verificação Semântica;
* o processo de Lowering.

Esses testes asseguram a continuidade arquitetural do Frontend.

---

## 16.5 Benchmarks

A implementação poderá incluir benchmarks destinados a acompanhar o desempenho da Inferência e Verificação de Tipos.

Entre os aspectos monitorados encontram-se:

* tempo de inferência;
* tempo de verificação;
* consumo de memória;
* desempenho em projetos de grande porte.

Esses benchmarks auxiliam a evolução da implementação sem alterar os contratos definidos nesta especificação.

---

## 16.6 Determinismo

Todos os testes relacionados à Inferência e Verificação de Tipos devem pressupor comportamento determinístico.

Para um mesmo programa válido, utilizando a mesma versão do compilador e as mesmas opções de compilação, as informações de tipos registradas na HIR deverão ser funcionalmente equivalentes.

---

# 17. Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência da Inferência e Verificação de Tipos do compilador oficial da linguagem Capi.

Seu papel consiste em enriquecer a HIR com todas as informações relacionadas ao Sistema de Tipos antes da etapa de Verificação Semântica.

---

## 17.1 Papel no Frontend

Dentro do Frontend do compilador, a Inferência e Verificação de Tipos sucede a Resolução de Nomes e antecede a Verificação Semântica.

Sua responsabilidade consiste exclusivamente na determinação, propagação e validação das informações de tipos presentes na HIR.

---

## 17.2 Relação com a HIR

A HIR permanece como a representação semântica oficial durante toda esta etapa.

A Inferência e Verificação de Tipos apenas adiciona novas informações semânticas relacionadas ao Sistema de Tipos, preservando integralmente a estrutura produzida pelas fases anteriores.

---

## 17.3 Relação com a Verificação Semântica

A Verificação Semântica recebe como entrada a HIR enriquecida produzida nesta etapa.

Ela deve assumir que todas as informações relacionadas ao Sistema de Tipos já se encontram corretamente determinadas, permitindo concentrar-se exclusivamente nas demais regras semânticas da linguagem.

---

## 17.4 Independência entre Implementações

Implementações distintas poderão utilizar algoritmos diferentes para realizar a Inferência e Verificação de Tipos.

Independentemente da estratégia adotada, todas deverão preservar:

* os contratos definidos neste documento;
* o comportamento determinístico;
* a consistência da HIR;
* a equivalência funcional dos resultados produzidos.

---

## 17.5 Evolução Futura

A arquitetura definida neste documento foi concebida para permitir evolução futura da implementação sem alterar os contratos estabelecidos para a Inferência e Verificação de Tipos.

Novas estratégias de inferência, otimizações internas e melhorias de desempenho poderão ser incorporadas desde que preservem integralmente as garantias desta especificação.

---

## 17.6 Síntese Arquitetural

A Inferência e Verificação de Tipos representa a etapa responsável por completar a representação tipada do programa.

Ao término desta fase, a HIR encontra-se semanticamente enriquecida com todas as informações necessárias para que a Verificação Semântica possa validar as regras restantes da linguagem.

---

# 18. Considerações Finais

A Inferência e Verificação de Tipos constitui a segunda etapa ativa da análise semântica do compilador da linguagem Capi.

Sua responsabilidade consiste em determinar os tipos dos elementos do programa, verificar sua compatibilidade conforme o Sistema de Tipos e enriquecer a HIR com as informações necessárias para as fases posteriores do Frontend.

---

## 18.1 Síntese da Arquitetura

A Inferência e Verificação de Tipos é caracterizada pelos seguintes princípios:

* responsabilidade única;
* inferência determinística;
* verificação consistente dos tipos;
* enriquecimento progressivo da HIR;
* preservação do Sistema de Tipos;
* independência da implementação.

Esses princípios garantem previsibilidade, consistência e facilidade de evolução da implementação.

---

## 18.2 Princípios Preservados

Toda implementação compatível com esta especificação deverá preservar:

* equivalência funcional entre implementações;
* consistência das informações de tipos;
* comportamento determinístico;
* enriquecimento progressivo da HIR;
* separação entre Inferência e Verificação de Tipos e Verificação Semântica.

Esses princípios asseguram interoperabilidade entre diferentes implementações do compilador.

---

## 18.3 Relação com o Documento Seguinte

Este documento define a etapa responsável por enriquecer a HIR com todas as informações relacionadas ao Sistema de Tipos.

O documento seguinte, **19 — Verificação Semântica**, especifica a fase responsável por validar as regras semânticas da linguagem que não pertencem ao Sistema de Tipos, utilizando a HIR enriquecida produzida nesta etapa.

Ao término da Verificação Semântica, a HIR estará completamente validada e preparada para servir como entrada do processo de **Lowering para IR**, definido no Documento 20.

---

## 18.4 Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência da Inferência e Verificação de Tipos do compilador oficial da linguagem Capi.

Outras implementações poderão adotar estratégias diferentes para realizar a inferência, validar compatibilidades e representar informações de tipos, desde que preservem integralmente:

* os contratos definidos neste documento;
* o comportamento determinístico;
* a consistência da HIR;
* a separação entre Inferência e Verificação de Tipos e Verificação Semântica;
* a compatibilidade com o processo de Lowering.

Dessa forma, este documento conclui a especificação da Inferência e Verificação de Tipos e estabelece a base para a **Verificação Semântica**, responsável por concluir a validação semântica do programa antes da geração da Intermediate Representation (IR).
