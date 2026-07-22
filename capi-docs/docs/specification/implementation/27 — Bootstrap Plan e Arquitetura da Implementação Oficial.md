# 27 — Bootstrap Plan e Arquitetura da Implementação Oficial

## Plano de Engenharia do Compilador Oficial da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define o plano de engenharia para a construção, evolução e auto-hospedagem do compilador oficial da linguagem Capi.

Seu objetivo é transformar os contratos arquiteturais estabelecidos pelos documentos anteriores em uma estratégia concreta de implementação, determinando:

* as tecnologias utilizadas na implementação inicial;
* a organização física do compilador;
* a sequência de desenvolvimento dos componentes;
* o subconjunto inicial da linguagem;
* os marcos técnicos do projeto;
* a estratégia de evolução dos backends;
* a implementação inicial do Runtime;
* a migração progressiva do compilador para a própria linguagem Capi;
* os critérios para conclusão do bootstrap e da auto-hospedagem.

A implementação inicial será desenvolvida em **Rust**, utilizando **Cranelift** como primeiro backend de geração de código nativo.

Após a estabilização da arquitetura, da Intermediate Representation, do Runtime e do modelo de execução da linguagem, será incorporado um backend baseado em **LLVM**, destinado principalmente à geração de código mais otimizado e à ampliação do suporte a plataformas e arquiteturas.

Este documento estabelece, portanto, o caminho concreto adotado pelo Projeto Capi para transformar sua especificação em uma implementação funcional, testável, evolutiva e, posteriormente, auto-hospedada.

---

## 1.2 Escopo

Este documento abrange as decisões e estratégias relacionadas à implementação oficial do compilador da Capi.

Fazem parte de seu escopo:

* a linguagem utilizada para implementar o primeiro compilador;
* a plataforma inicial de desenvolvimento e execução;
* a arquitetura física do repositório;
* a organização interna dos módulos do compilador;
* a integração entre Frontend, Middle-end, Backend e Runtime;
* a escolha e a evolução dos backends;
* a estratégia de implementação incremental;
* os critérios para definição do subconjunto inicial;
* a construção da Biblioteca Mínima de Bootstrap;
* a implementação inicial dos Domains;
* a representação inicial de `ObjectId`;
* a implementação da Capacidade de Mutação;
* a evolução do modelo de objetos;
* a estratégia de testes;
* os marcos de implementação;
* o processo de reescrita gradual do compilador em Capi;
* os estágios de auto-hospedagem;
* os critérios de conclusão do bootstrap.

Este documento não redefine:

* a sintaxe da linguagem;
* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* a Intermediate Representation oficial da linguagem;
* a ABI;
* os contratos do Runtime Mínimo;
* os contratos arquiteturais das fases do compilador.

Esses elementos permanecem definidos pelos Documentos 00 a 26.

O Documento 27 utiliza esses contratos como base e estabelece exclusivamente as decisões concretas adotadas pela implementação oficial.

---

## 1.3 Relação com os Documentos Anteriores

A documentação da Capi encontra-se organizada em três grupos complementares.

### Documentos 00 a 12 — Especificação da Linguagem e da Plataforma

Os Documentos 00 a 12 definem:

* os fundamentos da linguagem;
* o Sistema de Tipos;
* o Modelo de Memória;
* a sintaxe;
* a Semântica Operacional;
* a arquitetura conceitual do compilador;
* a Intermediate Representation da linguagem;
* a Biblioteca Padrão;
* o Runtime Mínimo;
* a ABI e a interoperabilidade;
* a Toolchain;
* o ecossistema e o gerenciamento de pacotes.

Esses documentos respondem à pergunta:

> O que é a linguagem Capi e quais garantias uma implementação compatível deve preservar?

---

### Documentos 13 a 26 — Especificação de Implementação

Os Documentos 13 a 26 definem os contratos arquiteturais da implementação do compilador.

Eles especificam:

* a estrutura lógica do compilador;
* o Lexer;
* o Parser;
* a AST;
* a HIR;
* a Resolução de Nomes;
* a Inferência e Verificação de Tipos;
* a Verificação Semântica;
* o Lowering;
* a Intermediate Representation da implementação;
* as otimizações;
* os diagnósticos;
* o Backend;
* os testes;
* o processo arquitetural de bootstrap.

Esses documentos respondem à pergunta:

> Quais componentes devem existir em uma implementação do compilador e quais contratos devem ser preservados entre eles?

---

### Documento 27 — Plano da Implementação Oficial

Este documento define as decisões concretas da implementação oficial.

Ele responde à pergunta:

> Como o compilador oficial da Capi será efetivamente construído?

Entre essas decisões encontram-se:

* Rust como linguagem hospedeira inicial;
* Cranelift como primeiro backend;
* LLVM como backend de evolução;
* Linux x86-64 como plataforma inicial;
* monorepo como organização inicial;
* implementação incremental por cortes verticais;
* Runtime inicialmente implementado em Rust;
* monomorfização como estratégia inicial para generics;
* migração progressiva dos componentes do compilador para Capi;
* auto-hospedagem validada por múltiplos estágios.

O Documento 27 complementa os documentos anteriores, mas não modifica seus contratos.

---

## 1.4 Natureza do Documento

Este documento possui natureza de **plano de engenharia da implementação oficial**.

Ele não constitui parte normativa da definição da linguagem Capi.

As decisões aqui registradas orientam exclusivamente o desenvolvimento do compilador mantido oficialmente pelo Projeto Capi.

Outras implementações poderão adotar:

* outras linguagens hospedeiras;
* outros backends;
* outras estratégias de Runtime;
* outras organizações físicas;
* outros modelos de construção;
* outras ferramentas de desenvolvimento.

Essas implementações serão consideradas compatíveis desde que preservem integralmente:

* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* a ABI aplicável;
* os contratos da Intermediate Representation;
* o comportamento observável da linguagem;
* as garantias definidas pelos Documentos 00 a 26.

As escolhas registradas neste documento não transformam Rust, Cranelift, LLVM ou qualquer outra tecnologia em parte da linguagem Capi.

Elas representam apenas os mecanismos utilizados pela implementação oficial para materializar as garantias definidas pela especificação.

---

## 1.5 Motivação

A existência de uma especificação completa não garante, isoladamente, que uma linguagem possa ser implementada com sucesso.

A implementação prática deverá validar, entre outros aspectos:

* a coerência entre o Sistema de Tipos e o Modelo de Memória;
* a viabilidade do modelo baseado em Domains;
* a estabilidade e a validade dos `ObjectId`;
* a implementação da Capacidade de Mutação;
* a representação de herança e despacho dinâmico;
* a integração entre compilador e Runtime;
* a preservação das garantias durante as otimizações;
* a compatibilidade entre diferentes backends;
* a viabilidade da auto-hospedagem.

Sem um plano explícito, o desenvolvimento poderia evoluir de maneira fragmentada, incorporando recursos sem ordem definida, produzindo dependências prematuras ou misturando decisões temporárias com contratos permanentes da linguagem.

Este documento existe para reduzir esses riscos.

Ele estabelece:

* uma direção técnica clara;
* uma sequência de desenvolvimento;
* limites de escopo;
* critérios objetivos de avanço;
* mecanismos de validação;
* uma estratégia de evolução tecnológica.

A implementação deverá evoluir de forma controlada, preservando continuamente a capacidade de produzir artefatos funcionais e testáveis.

---

## 1.6 Premissas

O plano definido neste documento parte das seguintes premissas.

### Premissa 1 — A especificação precede a implementação

Os Documentos 00 a 26 constituem a base arquitetural do projeto.

A implementação não deverá alterar silenciosamente os contratos definidos por esses documentos.

Caso a implementação revele uma inconsistência, lacuna ou inviabilidade na especificação, a questão deverá ser registrada, analisada e corrigida formalmente.

---

### Premissa 2 — A primeira implementação não precisa suportar toda a linguagem

O compilador inicial será construído sobre um subconjunto compatível da Capi.

Esse subconjunto deverá permitir a validação progressiva do pipeline, sem introduzir semântica incompatível com a linguagem completa.

---

### Premissa 3 — A correção possui prioridade sobre a otimização

Durante as primeiras etapas, a prioridade será:

1. correção;
2. previsibilidade;
3. testabilidade;
4. clareza da implementação;
5. desempenho.

Otimizações mais avançadas serão incorporadas somente após a estabilização das representações e dos contratos fundamentais.

---

### Premissa 4 — O backend não define a linguagem

Cranelift e LLVM serão consumidores da Intermediate Representation da Capi.

Nenhuma regra semântica da linguagem poderá depender exclusivamente de um backend específico.

---

### Premissa 5 — A auto-hospedagem será incremental

O compilador não será reescrito integralmente em Capi em uma única etapa.

A migração ocorrerá progressivamente, permitindo a coexistência temporária de módulos implementados em Rust e módulos implementados em Capi.

---

### Premissa 6 — Todo avanço deverá ser verificável

Cada marco deverá possuir:

* escopo definido;
* entradas válidas;
* entradas inválidas;
* testes obrigatórios;
* artefatos esperados;
* limitações conhecidas;
* critérios objetivos de conclusão.

---

### Premissa 7 — O primeiro target será deliberadamente restrito

A implementação inicial será direcionada a:

```text
Sistema operacional: Linux
Arquitetura: x86-64
Formato de objeto e executável: ELF
ABI: System V AMD64
```

A ampliação para outras plataformas ocorrerá apenas após a estabilização do pipeline inicial.

---

## 1.7 Terminologia

Para os fins deste documento, adotam-se as definições apresentadas a seguir.

### Implementação oficial

Implementação do compilador, do Runtime e dos componentes associados mantida diretamente pelo Projeto Capi.

---

### Linguagem hospedeira

Linguagem utilizada para implementar uma versão do compilador.

A linguagem hospedeira inicial será Rust.

---

### Compilador hospedeiro

Compilador utilizado para construir determinada versão do compilador Capi.

Durante o Stage 0, o compilador hospedeiro será o compilador Rust.

Nas etapas posteriores, uma versão anterior do compilador Capi poderá atuar como compilador hospedeiro.

---

### Stage 0

Primeira implementação funcional do compilador, escrita em Rust.

O Stage 0 será responsável por compilar o subconjunto inicial da linguagem e produzir as primeiras versões dos componentes implementados em Capi.

---

### Stage 1

Primeira versão do compilador escrita parcial ou integralmente em Capi e compilada pelo Stage 0.

---

### Stage 2

Versão do compilador produzida quando o Stage 1 compila novamente o código-fonte do compilador Capi.

---

### Stage 3

Recompilação adicional, quando necessária, destinada a validar estabilidade, convergência ou reprodutibilidade do processo de bootstrap.

---

### Subconjunto inicial

Conjunto temporário e reduzido de funcionalidades da Capi implementado nas primeiras versões do compilador.

Esse subconjunto não constitui uma variante independente da linguagem.

---

### Corte vertical

Incremento funcional que atravessa todas as fases necessárias do compilador, desde o código-fonte até a execução do programa.

Exemplo:

```text
Sintaxe
→ AST
→ HIR
→ Tipagem
→ IR
→ Backend
→ Executável
```

---

### Backend

Componente responsável por transformar a Intermediate Representation da Capi em artefatos executáveis ou intermediários compatíveis com uma plataforma de destino.

---

### Backend Cranelift

Primeiro backend da implementação oficial, utilizado prioritariamente para validar o pipeline e produzir código nativo com menor complexidade inicial.

---

### Backend LLVM

Backend posterior, destinado principalmente à geração de código mais otimizado, à ampliação de targets e à integração com uma infraestrutura madura de otimização e geração de código.

---

### Auto-hospedagem

Condição em que o compilador da Capi é capaz de compilar integralmente seu próprio código-fonte.

---

### Auto-hospedagem funcional

Situação em que uma versão do compilador escrita em Capi consegue compilar seu próprio código-fonte e produzir outra versão funcionalmente equivalente.

---

### Auto-hospedagem reproduzível

Situação em que o processo de recompilação pode ser repetido em condições documentadas, produzindo resultados equivalentes de maneira consistente.

---

### Auto-hospedagem confiável

Situação em que o processo de auto-hospedagem, além de funcional e reproduzível, encontra-se validado por uma suíte ampla de testes, comparações entre estágios e verificações de conformidade.

---

# 2. Objetivos do Bootstrap

## 2.1 Objetivo Principal

O objetivo principal do bootstrap é produzir progressivamente um compilador oficial capaz de:

* processar integralmente a linguagem Capi;
* preservar todas as garantias definidas pela especificação;
* produzir código nativo;
* utilizar diferentes backends;
* compilar seu próprio código-fonte;
* evoluir de forma independente da implementação inicial em Rust.

O processo deverá iniciar com uma implementação restrita, mas funcional, e evoluir continuamente até atingir uma implementação completa e auto-hospedada.

---

## 2.2 Objetivos Técnicos

O bootstrap deverá validar tecnicamente os principais elementos da linguagem.

Entre eles:

* a análise léxica;
* a análise sintática;
* a construção da AST;
* a construção e o enriquecimento da HIR;
* a Resolução de Nomes;
* a Inferência e Verificação de Tipos;
* a Verificação Semântica;
* o Lowering;
* a Intermediate Representation;
* a geração de código;
* a integração com o Runtime;
* o modelo de Domains;
* a estabilidade de `ObjectId`;
* a Capacidade de Mutação;
* a destruição determinística;
* a herança;
* o despacho dinâmico;
* as interfaces;
* as traits;
* os generics;
* a concorrência baseada em Domains.

Cada um desses elementos deverá ser incorporado por etapas, com critérios de validação próprios.

---

## 2.3 Validação da Especificação

A implementação deverá funcionar também como instrumento de validação da documentação existente.

Durante o desenvolvimento, poderão ser identificadas:

* ambiguidades;
* omissões;
* conflitos entre documentos;
* regras insuficientemente detalhadas;
* requisitos tecnicamente incompatíveis;
* decisões que necessitem de formalização adicional.

Essas situações não deverão ser resolvidas apenas por convenções internas do código.

Quando uma decisão afetar o comportamento da linguagem ou seus contratos públicos, ela deverá resultar em revisão formal da especificação correspondente.

O compilador não deverá tornar-se a única fonte de verdade da linguagem.

A especificação continuará sendo normativa, enquanto a implementação oficial constituirá sua materialização de referência.

---

## 2.4 Validação do Modelo de Domains

A arquitetura baseada em Domains constitui o principal diferencial da Capi.

O bootstrap deverá demonstrar, na prática, que esse modelo permite:

* alocação previsível;
* destruição determinística;
* grafos arbitrários de objetos;
* ciclos entre objetos;
* identidade compartilhada;
* ausência de contagem de referências;
* ausência de Garbage Collector;
* validade estática dos acessos;
* isolamento de escrita;
* futura concorrência segura.

A implementação inicial dos Domains poderá ser simples e conservadora.

Entretanto, deverá preservar integralmente os contratos semânticos definidos pelo Modelo de Memória.

---

## 2.5 Validação de `ObjectId`

O bootstrap deverá demonstrar que `ObjectId` pode representar identidade sem assumir:

* endereço de memória;
* ownership;
* autoridade de escrita;
* lifetime independente.

A implementação inicial deverá garantir:

* unicidade durante o lifetime;
* estabilidade;
* associação ao Domain correto;
* preservação durante upcasts;
* impossibilidade de acesso válido após a destruição do Domain;
* impossibilidade de reutilização indevida de uma identidade antiga.

A representação física inicial poderá utilizar identificadores de Domain, slots, índices, gerações ou mecanismos equivalentes.

Esses mecanismos permanecerão opacos à linguagem.

---

## 2.6 Validação da Capacidade de Mutação

O bootstrap deverá materializar a regra segundo a qual a escrita pertence ao Domain.

A implementação deverá ser capaz de:

* distinguir operações leitoras e mutantes;
* associar operações mutantes ao Domain correto;
* impedir mutações incompatíveis;
* impedir leitura e escrita concorrentes incompatíveis;
* representar a autoridade de mutação na HIR e na IR;
* eliminar custos de Runtime quando a garantia puder ser comprovada estaticamente.

A primeira implementação poderá adotar regras mais conservadoras.

Ela não poderá, entretanto, permitir comportamentos incompatíveis com as garantias permanentes da linguagem.

---

## 2.7 Produção de Código Nativo

A implementação oficial deverá produzir código nativo sem depender de:

* máquina virtual;
* ambiente de execução instalável equivalente à JVM ou CLR;
* Garbage Collector;
* Reference Counting automático.

A primeira geração de código será realizada com Cranelift.

Posteriormente, o LLVM será incorporado como backend adicional.

Os dois backends deverão consumir a mesma Intermediate Representation da Capi e preservar comportamento observável equivalente.

---

## 2.8 Independência entre Frontend e Backend

O bootstrap deverá preservar uma separação rigorosa entre:

* Frontend;
* Middle-end;
* Backend;
* Runtime.

O Frontend deverá compreender a linguagem.

O Middle-end deverá operar sobre a IR da Capi.

O Backend deverá traduzir a IR para a tecnologia de geração de código escolhida.

O Runtime deverá fornecer exclusivamente os serviços necessários à execução.

Essa separação permitirá:

* substituir Cranelift por LLVM sem reescrever o Frontend;
* manter os dois backends simultaneamente;
* incorporar novos targets;
* testar backends de forma diferencial;
* evoluir a geração de código sem alterar a linguagem.

---

## 2.9 Objetivos Arquiteturais

A implementação oficial deverá buscar:

* modularidade;
* baixo acoplamento;
* interfaces internas explícitas;
* ausência de dependências circulares;
* representações intermediárias validáveis;
* diagnósticos centralizados;
* testabilidade independente de cada fase;
* determinismo;
* reprodutibilidade;
* possibilidade futura de compilação incremental;
* suporte à inspeção dos artefatos intermediários.

Cada módulo deverá possuir responsabilidade claramente delimitada.

---

## 2.10 Objetivos de Qualidade

A implementação deverá ser desenvolvida com foco em:

* correção;
* clareza;
* previsibilidade;
* manutenção;
* testabilidade;
* rastreabilidade;
* documentação técnica;
* diagnósticos de qualidade.

A velocidade de implementação não deverá justificar violações dos contratos definidos pela arquitetura.

Da mesma forma, otimizações prematuras não deverão comprometer a legibilidade nem a validação do compilador.

---

## 2.11 Objetivos de Longo Prazo

Após a conclusão do bootstrap, a implementação deverá fornecer uma base para:

* evolução contínua da linguagem;
* manutenção do compilador em Capi;
* coexistência entre Cranelift e LLVM;
* suporte a múltiplas plataformas;
* expansão da Biblioteca Padrão;
* integração com a Toolchain;
* desenvolvimento de ferramentas;
* compilação incremental;
* LSP;
* depuração;
* otimizações específicas da Capi;
* builds reproduzíveis;
* distribuição oficial do compilador.

Esses objetivos não pertencem necessariamente ao primeiro ciclo de implementação.

O plano deverá preservar a possibilidade de incorporá-los sem exigir reconstrução completa da arquitetura.

---

## 2.12 Critérios Gerais de Sucesso

O processo de bootstrap será considerado bem-sucedido quando atingir, de forma progressiva, os seguintes resultados.

### Primeiro pipeline completo

O compilador deverá transformar um programa Capi mínimo em um executável nativo.

---

### Primeiro subconjunto funcional

O compilador deverá suportar funções, variáveis, expressões, controle de fluxo e tipos fundamentais.

---

### Primeiro modelo de objetos

O compilador deverá criar objetos, acessar campos, executar métodos e preservar identidade.

---

### Primeira implementação de Domains

O compilador e o Runtime deverão criar, utilizar e destruir Domains deterministicamente.

---

### Primeira validação da Capacidade de Mutação

O compilador deverá rejeitar acessos incompatíveis e permitir mutações válidas conforme as regras da linguagem.

---

### Orientação a objetos completa

O compilador deverá suportar herança, subtipagem, override e despacho dinâmico.

---

### Biblioteca suficiente para bootstrap

A linguagem deverá possuir strings, coleções, tratamento de erros, IO e recursos suficientes para implementar partes do compilador.

---

### Primeiro módulo do compilador escrito em Capi

Um componente real do compilador deverá ser reimplementado em Capi e comparado com a implementação equivalente em Rust.

---

### Compilador híbrido

O processo deverá permitir uma versão composta por módulos em Rust e Capi.

---

### Auto-hospedagem funcional

Uma versão do compilador escrita em Capi deverá ser capaz de compilar seu próprio código-fonte.

---

### Auto-hospedagem confiável

Os diferentes estágios deverão passar pela mesma suíte de conformidade e produzir comportamento equivalente.

---

### Backend LLVM

O compilador deverá incorporar um backend LLVM sem modificar os contratos do Frontend ou da IR.

---

# 3. Princípios da Implementação Oficial

## 3.1 Desenvolvimento Incremental

A implementação será construída em etapas pequenas e verificáveis.

Nenhuma fase deverá tentar implementar integralmente todos os recursos previstos pela linguagem antes de produzir resultados executáveis.

O compilador deverá evoluir por incrementos funcionais, preservando em cada etapa:

* capacidade de compilação;
* testes;
* diagnósticos;
* documentação;
* consistência interna.

Cada nova funcionalidade deverá ser incorporada somente quando seus requisitos anteriores estiverem suficientemente estáveis.

---

## 3.2 Desenvolvimento por Cortes Verticais

A implementação adotará prioritariamente cortes verticais.

Em vez de concluir todo o Frontend antes de iniciar o Backend, cada incremento deverá, sempre que possível, atravessar o pipeline completo.

Exemplo:

```text
Código-fonte
      │
      ▼
Lexer
      │
      ▼
Parser
      │
      ▼
AST
      │
      ▼
HIR
      │
      ▼
Tipagem
      │
      ▼
IR
      │
      ▼
Cranelift
      │
      ▼
Executável
```

Um primeiro corte poderá suportar apenas:

```capi
function main() {
}
```

Cortes posteriores acrescentarão:

* retorno;
* literais;
* expressões;
* variáveis;
* controle de fluxo;
* funções;
* tipos por valor;
* objetos;
* Domains;
* herança;
* generics.

Essa abordagem permitirá detectar problemas de integração antecipadamente.

---

## 3.3 Correção Antes de Desempenho

O primeiro objetivo da implementação é produzir comportamento correto.

A ordem de prioridade será:

1. preservação da semântica;
2. segurança;
3. previsibilidade;
4. testabilidade;
5. clareza arquitetural;
6. desempenho do compilador;
7. desempenho do código gerado.

A existência futura do backend LLVM não deverá provocar otimizações prematuras durante a fase Cranelift.

O Cranelift será utilizado inicialmente para validar o pipeline e gerar código funcional.

O LLVM será incorporado posteriormente, quando as representações e os contratos já estiverem suficientemente estabilizados.

---

## 3.4 Testes Desde o Início

Nenhum componente será considerado concluído apenas por produzir resultados em casos simples.

Cada fase deverá possuir testes apropriados à sua responsabilidade.

Entre eles:

* testes unitários;
* testes de integração;
* testes funcionais;
* testes negativos;
* Golden Tests;
* testes de regressão;
* testes diferenciais;
* fuzzing, quando aplicável.

Todo erro corrigido deverá produzir um caso permanente de regressão.

---

## 3.5 Determinismo

A implementação deverá produzir resultados determinísticos.

Dadas:

* a mesma versão do compilador;
* as mesmas entradas;
* as mesmas dependências;
* as mesmas opções;
* a mesma plataforma de destino;

os artefatos produzidos deverão ser semanticamente equivalentes.

Sempre que possível, também deverão ser reproduzíveis em nível binário.

Estruturas internas que possam introduzir variação não controlada, como iteração não determinística sobre mapas, deverão ser tratadas explicitamente quando influenciarem artefatos observáveis.

---

## 3.6 Representações Explícitas e Inspecionáveis

Tokens, AST, HIR e IR deverão possuir formas apropriadas de inspeção.

A implementação deverá permitir, em modo de desenvolvimento, a emissão de representações textuais para:

* sequência de tokens;
* AST;
* HIR inicial;
* HIR com nomes resolvidos;
* HIR tipada;
* HIR validada;
* tabela de símbolos;
* restrições de tipos;
* IR;
* IR após cada passe;
* Cranelift IR;
* LLVM IR;
* metadados de objetos e Domains.

Esses recursos serão fundamentais para:

* depuração;
* testes;
* análise de regressões;
* compreensão da implementação;
* comparação entre backends;
* comparação entre estágios do bootstrap.

---

## 3.7 Validação das Representações Internas

Cada representação interna deverá possuir invariantes explicitamente definidos.

Sempre que aplicável, deverão existir validadores capazes de confirmar:

* integridade estrutural;
* referências válidas;
* tipos consistentes;
* blocos de controle completos;
* terminadores presentes;
* associações corretas entre objetos e Domains;
* uso válido de capacidades;
* ausência de elementos pendentes após a fase correspondente.

Em modo de desenvolvimento, essas validações poderão ser executadas após cada transformação relevante.

---

## 3.8 Separação entre Garantias e Mecanismos

A implementação oficial continuará respeitando o princípio estabelecido pela especificação:

> Garantias pertencem à linguagem. Mecanismos pertencem à implementação.

Por exemplo:

* estabilidade de `ObjectId` é garantia;

* uso de índice e geração é mecanismo;

* destruição determinística é garantia;

* uso de arena é mecanismo;

* exclusividade de mutação é garantia;

* representação por capacidades lineares, estados da HIR ou análise de empréstimos é mecanismo;

* despacho dinâmico é garantia;

* uso de vtables é mecanismo;

* compatibilidade de herança é garantia;

* layout prefixado concreto é mecanismo de materialização.

A implementação poderá evoluir seus mecanismos sem modificar o comportamento observável.

---

## 3.9 Independência da IR da Capi

A Intermediate Representation da Capi deverá permanecer independente de Cranelift e LLVM.

O fluxo será:

```text
HIR
      │
      ▼
IR da Capi
      │
      ├──────────────► Tradutor Cranelift
      │
      └──────────────► Tradutor LLVM
```

A IR deverá preservar os conceitos necessários à linguagem antes de ser reduzida às operações entendidas por cada backend.

Cranelift IR e LLVM IR serão representações de backend, não substitutos da IR oficial da Capi.

---

## 3.10 Interface Comum de Backend

Os backends deverão implementar um contrato interno comum.

Esse contrato deverá permitir:

* seleção do backend;
* configuração do target;
* recebimento da IR validada;
* geração de código objeto;
* integração com o linker;
* emissão de informações de depuração;
* apresentação de diagnósticos internos;
* declaração de recursos suportados.

Nenhuma fase anterior deverá acessar diretamente APIs específicas do Cranelift ou do LLVM.

---

## 3.11 Runtime Mínimo

O Runtime inicial deverá permanecer pequeno.

Deverá conter apenas serviços que não possam ser resolvidos integralmente pelo compilador ou pela Biblioteca Padrão.

A implementação deverá evitar que o Runtime se transforme em:

* máquina virtual;
* framework de uso geral;
* biblioteca padrão paralela;
* camada obrigatória excessivamente pesada.

Funcionalidades de alto nível deverão ser implementadas preferencialmente na Biblioteca Padrão.

---

## 3.12 Escopo Controlado

O primeiro ciclo de implementação não deverá incluir todos os recursos desejáveis para uma linguagem madura.

Funcionalidades não essenciais deverão ser adiadas deliberadamente.

Entre elas poderão estar:

* múltiplas plataformas;
* backend LLVM desde o início;
* otimizações agressivas;
* compilação incremental;
* LSP completo;
* depurador próprio;
* profiler;
* JIT;
* reflection completa;
* concorrência completa;
* backend nativo próprio;
* compatibilidade binária definitiva.

O adiamento não representa rejeição permanente.

Representa apenas controle de complexidade.

---

## 3.13 Dependências Externas Reduzidas

A implementação deverá evitar dependências externas desnecessárias.

Cada dependência deverá ser avaliada considerando:

* finalidade;
* manutenção;
* licença;
* estabilidade;
* segurança;
* impacto no build;
* risco de abandono;
* possibilidade de substituição.

Cranelift e LLVM constituem dependências arquiteturais deliberadas dos respectivos backends.

Nenhuma delas deverá se espalhar para módulos que não participem da geração de código.

---

## 3.14 Rastreabilidade

Toda transformação relevante deverá preservar informações suficientes para relacionar os artefatos internos ao código-fonte original.

Essa rastreabilidade deverá permitir:

* emissão de diagnósticos;
* geração futura de informações de depuração;
* navegação por ferramentas;
* inspeção da origem de elementos da IR;
* comparação entre versões;
* análise de falhas internas.

O conceito de `Span` deverá permanecer disponível durante todo o Frontend e, quando necessário, durante o Middle-end e o Backend.

---

## 3.15 Erros Internos e Erros do Programa

A implementação deverá distinguir claramente:

### Erros do programa

Problemas presentes no código-fonte, como:

* token inválido;
* sintaxe inválida;
* símbolo inexistente;
* incompatibilidade de tipos;
* violação semântica.

Esses erros deverão produzir diagnósticos para o desenvolvedor.

### Erros internos do compilador

Violações dos invariantes da implementação, como:

* HIR inconsistente;
* IR malformada;
* referência interna inválida;
* operação não suportada após validação;
* divergência entre fases.

Esses erros deverão ser identificados como falhas internas, sem serem apresentados como erros do código Capi.

---

## 3.16 Compatibilidade entre Backends

Todos os backends oficiais deverão preservar comportamento observável equivalente.

Um mesmo programa válido, compilado com Cranelift ou LLVM, deverá apresentar equivalência em relação a:

* valores produzidos;
* saída;
* códigos de retorno;
* efeitos externos;
* destruição de Domains;
* identidade;
* despacho;
* comportamento de erros definidos pela linguagem.

Diferenças de desempenho, tamanho de binário e organização interna são permitidas.

Diferenças semânticas não são permitidas.

---

## 3.17 Migração Gradual para Capi

A auto-hospedagem será construída por substituição progressiva de componentes.

A implementação em Rust continuará disponível durante a migração para:

* comparação funcional;
* regressão;
* produção do compilador hospedeiro;
* validação dos módulos reescritos;
* recuperação diante de falhas.

Nenhum módulo deverá ser removido da implementação Rust antes que sua versão Capi esteja suficientemente validada.

---

## 3.18 Evolução Controlada das Decisões

As decisões registradas neste documento poderão evoluir.

Mudanças relevantes deverão ser:

* explicitamente justificadas;
* registradas em ADRs;
* avaliadas quanto ao impacto;
* refletidas na documentação;
* acompanhadas por atualizações nos testes e marcos.

Decisões técnicas importantes não deverão existir apenas no histórico do código ou em conversas informais.

---

## 3.19 Documentação como Parte da Implementação

A documentação técnica será considerada parte integrante do compilador.

Cada módulo deverá registrar:

* responsabilidade;
* entradas;
* saídas;
* invariantes;
* dependências;
* condições de erro;
* relação com as fases anteriores e posteriores.

A documentação deverá acompanhar a evolução da implementação.

---

## 3.20 Simplicidade Inicial e Evolução Posterior

As primeiras soluções deverão privilegiar simplicidade e correção.

Por exemplo:

* arena simples antes de alocadores especializados;
* tabela explícita antes de representações compactadas;
* monomorfização antes de estratégias complexas de compartilhamento de generics;
* vtables simples antes de caches sofisticados;
* linker da plataforma antes de um linker próprio;
* análise conservadora antes de inferências agressivas.

Essas escolhas poderão ser substituídas posteriormente, desde que os contratos da linguagem permaneçam preservados.

---

## 3.21 Princípio de Encerramento de Etapas

Uma etapa somente deverá ser considerada concluída quando:

* seu escopo estiver integralmente implementado;
* os testes correspondentes forem aprovados;
* os diagnósticos necessários existirem;
* os artefatos forem inspecionáveis;
* os invariantes forem validáveis;
* as limitações estiverem documentadas;
* não existirem bloqueadores conhecidos para a etapa seguinte.

O encerramento de uma etapa não exige perfeição absoluta.

Exige estabilidade suficiente para que a próxima fase possa depender de seus contratos.

---

# 4. Linguagem Hospedeira e Ambiente de Desenvolvimento

## 4.1 Linguagem Hospedeira Inicial

A implementação inicial do compilador oficial da linguagem Capi será desenvolvida em **Rust**.

Rust será utilizado na construção do compilador Stage 0, incluindo inicialmente:

* infraestrutura compartilhada;
* Driver;
* Source Manager;
* sistema de diagnósticos;
* Lexer;
* Parser;
* AST;
* HIR;
* Resolução de Nomes;
* Inferência e Verificação de Tipos;
* Verificação Semântica;
* Lowering;
* Intermediate Representation;
* passes iniciais de otimização;
* integração com Cranelift;
* Runtime inicial;
* ferramentas auxiliares necessárias ao bootstrap.

A escolha de Rust pertence exclusivamente à implementação oficial.

Rust não constitui parte:

* da especificação da linguagem Capi;
* da semântica observável;
* da ABI pública da Capi;
* da Intermediate Representation oficial;
* do modelo de objetos;
* do Modelo de Memória;
* do processo permanente de auto-hospedagem.

Outras implementações compatíveis da Capi poderão ser desenvolvidas em linguagens diferentes.

---

## 4.2 Justificativa da Escolha do Rust

Rust foi escolhido como linguagem hospedeira inicial por combinar características técnicas adequadas à construção de compiladores e afinidade com os objetivos arquiteturais do Projeto Capi.

Entre os principais motivos encontram-se:

* segurança de memória sem Garbage Collector;
* controle explícito sobre alocação e representação de dados;
* abstrações de custo previsível;
* sistema de tipos expressivo;
* suporte adequado à modelagem de estados internos;
* integração natural com bibliotecas de geração de código;
* ecossistema consolidado para desenvolvimento de compiladores;
* suporte a testes, benchmarks e análise estática;
* facilidade de distribuição através do Cargo;
* disponibilidade de bibliotecas para integração com Cranelift;
* possibilidade de integração posterior com LLVM;
* suporte consistente às plataformas inicialmente pretendidas.

A afinidade técnica do coordenador do Projeto Capi com Rust também reduz o custo inicial de desenvolvimento e facilita a avaliação crítica das decisões tomadas durante a implementação.

---

## 4.3 Alinhamento entre Rust e a Implementação da Capi

A utilização de Rust favorece a implementação segura de diversos componentes internos do compilador.

Entre eles:

* identidades internas;
* arenas;
* árvores e grafos;
* referências entre representações;
* tabelas de símbolos;
* estruturas de tipos;
* blocos básicos;
* instruções da IR;
* interfaces entre módulos;
* recursos do Runtime;
* integração com APIs de baixo nível.

Esse alinhamento não significa que o modelo da Capi será implementado como uma adaptação direta do modelo de ownership do Rust.

Os conceitos fundamentais da Capi deverão possuir representação própria.

Em particular:

* `ObjectId` não será representado semanticamente como referência Rust;
* Domain não será tratado como mero lifetime Rust;
* Capacidade de Mutação não será reduzida ao mecanismo de empréstimos da linguagem hospedeira;
* o Sistema de Tipos da Capi permanecerá independente do sistema de tipos de Rust;
* as regras semânticas da Capi deverão ser validadas explicitamente pelo compilador.

Rust será utilizado como mecanismo seguro de implementação, não como definição indireta da linguagem.

---

## 4.4 Limites da Linguagem Hospedeira

Nenhuma regra da Capi poderá ser considerada válida apenas porque a implementação em Rust impede determinada situação.

As garantias da linguagem deverão ser representadas explicitamente nos artefatos do compilador.

Por exemplo, mesmo que a implementação Rust impeça uma referência interna inválida, o compilador ainda deverá possuir regras próprias para validar:

* lifetime de Domains;
* validade de `ObjectId`;
* exclusividade da Capacidade de Mutação;
* compatibilidade entre Domains;
* segurança de acesso;
* fluxo de inicialização;
* regras de herança;
* subtipagem;
* despacho dinâmico.

Da mesma forma, limitações da implementação Rust não deverão ser transferidas indevidamente para a linguagem Capi.

A especificação permanece a autoridade normativa.

---

## 4.5 Rust como Base Temporária do Bootstrap

O compilador Stage 0 escrito em Rust deverá permanecer disponível durante toda a fase inicial do bootstrap.

Ele será utilizado para:

* compilar o subconjunto inicial da Capi;
* produzir os primeiros executáveis;
* validar a especificação;
* gerar as primeiras versões dos módulos escritos em Capi;
* comparar implementações equivalentes;
* servir como referência funcional;
* permitir recuperação diante de regressões no compilador auto-hospedado.

A dependência de Rust deverá diminuir progressivamente conforme os componentes forem reimplementados em Capi.

Entretanto, a remoção dessa dependência não deverá ocorrer prematuramente.

A implementação Stage 0 poderá permanecer preservada como:

* compilador histórico de bootstrap;
* implementação de referência;
* ferramenta de validação;
* mecanismo de reconstrução inicial;
* base para comparação diferencial.

---

## 4.6 Toolchain Rust

A implementação inicial utilizará a Toolchain oficial do ecossistema Rust.

Entre seus componentes encontram-se:

* `rustc`;
* Cargo;
* `rustfmt`;
* Clippy;
* ferramentas de teste;
* ferramentas de benchmark;
* ferramentas de documentação;
* mecanismos de bloqueio de dependências.

O Cargo será utilizado inicialmente para:

* organização do workspace;
* gerenciamento das dependências;
* construção dos componentes;
* execução dos testes;
* geração da documentação técnica;
* definição de perfis de build;
* empacotamento dos binários iniciais.

A estrutura do projeto deverá evitar dependência desnecessária de mecanismos específicos do Cargo fora da camada de implementação.

---

## 4.7 Versão Mínima do Rust

A implementação oficial deverá definir uma **Minimum Supported Rust Version** — MSRV.

A MSRV possui como objetivos:

* tornar o ambiente de desenvolvimento previsível;
* reduzir divergências entre colaboradores;
* facilitar integração contínua;
* preservar builds reproduzíveis;
* controlar a adoção de recursos recentes da linguagem.

A versão exata poderá ser definida no início da implementação e revisada formalmente ao longo do projeto.

A política deverá obedecer aos seguintes princípios:

* a MSRV deverá ser explicitamente documentada;
* atualizações deverão ser deliberadas;
* dependências deverão permanecer compatíveis com a versão selecionada;
* alterações da MSRV deverão ser registradas;
* o pipeline de integração contínua deverá validar a versão mínima suportada.

---

## 4.8 Edições do Rust

A edição do Rust utilizada deverá ser declarada na configuração do workspace.

A adoção de uma nova edição deverá ocorrer somente quando:

* houver benefício arquitetural claro;
* todas as dependências forem compatíveis;
* o impacto sobre a implementação estiver compreendido;
* a migração puder ser validada pela suíte de testes.

A atualização de edição não altera a arquitetura da Capi.

Trata-se exclusivamente de uma evolução da implementação hospedeira.

---

## 4.9 Gerenciamento de Dependências Rust

Todas as dependências externas deverão possuir versões controladas.

O arquivo de bloqueio de dependências deverá ser preservado para os componentes executáveis e para o processo de bootstrap.

A seleção de dependências deverá considerar:

* licença;
* manutenção ativa;
* estabilidade da API;
* histórico de segurança;
* aderência ao código aberto;
* impacto no tempo de compilação;
* impacto no tamanho dos artefatos;
* quantidade de dependências transitivas;
* possibilidade de substituição;
* necessidade arquitetural real.

Dependências críticas deverão permanecer encapsuladas atrás de interfaces internas.

Em particular:

* Cranelift deverá permanecer isolado no backend correspondente;
* LLVM deverá permanecer isolado em seu backend;
* bibliotecas de CLI não deverão influenciar o Driver interno;
* bibliotecas de diagnóstico não deverão definir a representação semântica dos diagnósticos;
* bibliotecas de parsing, caso utilizadas, não deverão definir a AST oficial.

---

## 4.10 Código `unsafe` em Rust

A implementação deverá restringir o uso de código `unsafe`.

Seu uso poderá ser necessário em componentes como:

* integração com APIs nativas;
* Runtime;
* FFI;
* integração com LLVM;
* gerenciamento especializado de memória;
* execução de código gerado;
* manipulação de buffers de baixo nível.

Todo bloco `unsafe` deverá:

* possuir finalidade claramente identificada;
* ser mantido no menor escopo possível;
* documentar suas pré-condições;
* documentar os invariantes preservados;
* possuir testes específicos;
* permanecer isolado atrás de interfaces seguras sempre que possível.

O uso de `unsafe` não poderá substituir verificações necessárias à semântica da Capi.

---

## 4.11 Plataforma Inicial de Desenvolvimento

A plataforma inicial de desenvolvimento e validação será:

```text
Sistema operacional: Linux
Arquitetura: x86-64
Formato de código objeto: ELF
ABI de plataforma: System V AMD64
```

Essa escolha reduz a quantidade de variáveis durante o bootstrap.

Ela permite concentrar esforços em:

* Frontend;
* modelo semântico;
* IR;
* Runtime;
* integração com Cranelift;
* ligação dos artefatos;
* execução dos testes.

O suporte inicial a uma única plataforma não representa limitação permanente da linguagem.

---

## 4.12 Justificativa do Linux x86-64

Linux x86-64 foi escolhido como primeiro target por apresentar:

* ampla disponibilidade;
* documentação consolidada;
* suporte maduro em Rust;
* suporte estável no Cranelift;
* suporte abrangente no LLVM;
* ferramentas maduras de depuração;
* formato ELF amplamente documentado;
* ABI System V AMD64 consolidada;
* disponibilidade de linkers e utilitários de baixo nível;
* facilidade de automação em integração contínua;
* compatibilidade com containers.

Além disso, o ambiente de desenvolvimento principal do projeto já utiliza Linux, reduzindo o custo operacional inicial.

---

## 4.13 Formato ELF

A primeira implementação produzirá artefatos compatíveis com o formato ELF.

Inicialmente, a geração poderá resultar em:

* arquivos objeto;
* executáveis;
* bibliotecas estáticas;
* metadados auxiliares.

O compilador não deverá implementar inicialmente um linker próprio.

A integração com o formato ELF poderá ser realizada através:

* do próprio backend;
* de bibliotecas especializadas;
* de ferramentas da plataforma;
* de linkers externos compatíveis.

A escolha concreta será detalhada na arquitetura do backend.

---

## 4.14 ABI System V AMD64

O backend inicial deverá respeitar a ABI System V AMD64 para:

* passagem de parâmetros;
* retorno de valores;
* uso de registradores;
* alinhamento da pilha;
* preservação de registradores;
* representação dos símbolos;
* integração com código externo;
* chamadas ao Runtime;
* ligação com bibliotecas da plataforma.

A ABI interna entre o código Capi e o Runtime poderá introduzir convenções próprias, desde que seja corretamente adaptada à ABI da plataforma.

---

## 4.15 Ambiente Baseado em Containers

O projeto poderá fornecer um ambiente oficial baseado em containers para:

* padronizar a Toolchain;
* simplificar a entrada de novos colaboradores;
* reproduzir builds;
* executar a integração contínua localmente;
* preservar versões de ferramentas;
* isolar dependências do sistema.

O uso de containers não será obrigatório para o desenvolvimento cotidiano.

O build também deverá ser possível em um ambiente Linux compatível devidamente configurado.

---

## 4.16 Ferramentas de Sistema

A implementação inicial poderá utilizar ferramentas externas da plataforma, incluindo:

* linker;
* assembler;
* `ar`;
* `objdump`;
* `readelf`;
* `nm`;
* `gdb`;
* `lldb`;
* utilitários de inspeção de ELF;
* ferramentas de benchmark;
* sanitizers;
* analisadores de memória.

Essas ferramentas pertencem ao ambiente de implementação.

Não constituem dependências da linguagem Capi nem contratos públicos da Toolchain futura.

---

## 4.17 Build Reproduzível

Desde as primeiras fases, a arquitetura deverá favorecer builds reproduzíveis.

Para isso, deverão ser controlados:

* versão do Rust;
* versões das dependências;
* versão do Cranelift;
* versão do LLVM, quando incorporado;
* flags de compilação;
* linker utilizado;
* ambiente de build;
* scripts auxiliares;
* variáveis relevantes.

A reprodução binária exata poderá não ser um requisito inicial.

Entretanto, a reprodução funcional deverá existir desde os primeiros marcos.

---

## 4.18 Ambientes de Build

A implementação poderá definir diferentes perfis.

### Perfil de desenvolvimento

Prioriza:

* compilação rápida;
* validação de invariantes;
* informações de depuração;
* logs;
* dumps intermediários;
* assertions internas.

### Perfil de teste

Prioriza:

* determinismo;
* validação ampla;
* instrumentação;
* sanitizers;
* execução de casos negativos.

### Perfil de release

Prioriza:

* desempenho do compilador;
* redução de logs;
* otimizações da implementação Rust;
* geração de artefatos distribuíveis.

Esses perfis pertencem ao compilador hospedeiro e não devem ser confundidos com os futuros perfis de compilação de programas Capi.

---

# 5. Estratégia dos Backends

## 5.1 Objetivo

A implementação oficial adotará uma arquitetura de múltiplos backends.

O primeiro backend utilizará **Cranelift**.

Após a estabilização do compilador, será incorporado um backend baseado em **LLVM**.

A estratégia possui como objetivos:

* reduzir a complexidade inicial;
* produzir código nativo desde cedo;
* preservar a independência do Frontend;
* validar a IR da Capi;
* permitir evolução posterior da qualidade do código gerado;
* ampliar o suporte a plataformas;
* evitar dependência estrutural de uma única tecnologia.

---

## 5.2 Princípio de Independência do Backend

Nenhuma fase anterior ao backend deverá depender de APIs, tipos ou estruturas específicas do Cranelift ou do LLVM.

O fluxo arquitetural será:

```text
Código-fonte Capi
        │
        ▼
Frontend
        │
        ▼
HIR
        │
        ▼
Lowering
        │
        ▼
IR da Capi
        │
        ├──────────────► Backend Cranelift
        │
        └──────────────► Backend LLVM
```

A IR da Capi constitui o contrato comum entre o Middle-end e os backends.

Cranelift IR e LLVM IR são representações internas dos respectivos backends.

---

## 5.3 Backend Inicial com Cranelift

Cranelift será utilizado durante a primeira fase da implementação oficial.

Seus principais objetivos são:

* produzir o primeiro código nativo;
* validar o pipeline de ponta a ponta;
* permitir cortes verticais executáveis;
* reduzir o tempo de integração;
* oferecer ciclos rápidos de compilação;
* testar o Runtime;
* validar chamadas de função;
* validar layout de dados;
* validar operações sobre Domains;
* validar o modelo de objetos;
* fornecer base para os primeiros estágios de bootstrap.

A prioridade inicial será correção funcional, não otimização máxima.

---

## 5.4 Justificativa da Escolha do Cranelift

Cranelift foi escolhido como primeiro backend porque apresenta:

* implementação em Rust;
* integração natural com o ecossistema Cargo;
* API adequada à geração programática de código;
* tempo de compilação rápido;
* suporte a código nativo;
* suporte ao target Linux x86-64;
* menor complexidade de integração em comparação com LLVM;
* abstrações já disponíveis para geração de código objeto;
* suporte a chamadas externas;
* desenvolvimento ativo;
* utilização em projetos relevantes do ecossistema WebAssembly.

Essas características permitem que o projeto concentre seus esforços iniciais nos elementos que realmente diferenciam a Capi.

---

## 5.5 Papel do Cranelift no Bootstrap

O Cranelift não será apenas um protótipo descartável.

Ele será um backend oficial da implementação inicial.

Durante o bootstrap, será responsável por:

* gerar o Stage 0 executável;
* gerar os primeiros programas Capi;
* compilar os módulos iniciais reimplementados em Capi;
* apoiar o compilador híbrido;
* produzir o Stage 1;
* potencialmente produzir o Stage 2;
* executar testes de conformidade;
* servir como backend de desenvolvimento rápido.

Sua continuidade após a incorporação do LLVM será avaliada conforme os critérios definidos neste documento.

A estratégia inicial prevê sua manutenção.

---

## 5.6 Limitações Deliberadas do Backend Cranelift

A primeira implementação poderá aceitar limitações como:

* suporte a uma única arquitetura;
* conjunto reduzido de otimizações;
* informações de depuração limitadas;
* ausência inicial de vetorização;
* ausência de otimizações interprocedimentais avançadas;
* suporte progressivo aos tipos da linguagem;
* suporte progressivo à ABI;
* integração inicial com linker externo;
* menor qualidade de código em comparação com LLVM.

Essas limitações deverão ser documentadas.

Elas não poderão resultar em diferenças semânticas.

---

## 5.7 Tradução da IR da Capi para Cranelift IR

O backend Cranelift possuirá um componente responsável por traduzir a IR da Capi para Cranelift IR.

Esse tradutor deverá:

* consumir exclusivamente IR validada;
* mapear tipos;
* mapear valores;
* mapear blocos básicos;
* mapear terminadores;
* mapear chamadas;
* mapear acessos à memória;
* mapear operações sobre objetos;
* gerar chamadas ao Runtime;
* preservar ordem de avaliação;
* preservar efeitos;
* preservar a ABI aplicável;
* emitir diagnósticos internos quando ocorrer uma impossibilidade de tradução.

Nenhum conceito semântico pendente deverá ser resolvido pelo tradutor.

---

## 5.8 Introdução Posterior do LLVM

O LLVM será incorporado apenas após a estabilização dos principais componentes do compilador.

Entre as condições esperadas para sua introdução encontram-se:

* Frontend funcional;
* HIR estabilizada;
* IR da Capi suficientemente madura;
* backend Cranelift operacional;
* Runtime validado;
* modelo de Domains funcional;
* modelo de objetos funcional;
* testes de conformidade disponíveis;
* primeiro subconjunto relevante da linguagem implementado;
* interface comum de backend consolidada.

A introdução antecipada do LLVM aumentaria a complexidade antes da validação dos elementos centrais da Capi.

---

## 5.9 Objetivos do Backend LLVM

O backend LLVM terá como objetivos:

* melhorar a qualidade do código gerado;
* disponibilizar otimizações avançadas;
* ampliar o suporte a targets;
* melhorar integração com informações de depuração;
* oferecer vetorização;
* oferecer otimizações interprocedimentais;
* integrar-se a ferramentas maduras;
* permitir análise mais aprofundada do código produzido;
* fornecer um perfil de compilação orientado a release.

O LLVM não deverá alterar a semântica da linguagem.

---

## 5.10 LLVM como Backend Adicional

A introdução do LLVM não implicará necessariamente a remoção do Cranelift.

A arquitetura deverá permitir a coexistência dos dois backends.

Uma possível política futura será:

```text
Compilações de desenvolvimento
Backend Cranelift

Compilações de release
Backend LLVM
```

Essa política não deverá ser considerada definitiva antes da avaliação prática dos dois backends.

---

## 5.11 Tradução da IR da Capi para LLVM IR

O backend LLVM possuirá um tradutor próprio:

```text
IR da Capi
      │
      ▼
LLVM IR
      │
      ▼
Passes LLVM
      │
      ▼
Código objeto
```

Esse tradutor deverá permanecer isolado do backend Cranelift.

Não deverá existir uma tradução:

```text
Cranelift IR → LLVM IR
```

nem:

```text
LLVM IR → Cranelift IR
```

Ambos deverão consumir diretamente a IR oficial da Capi.

---

## 5.12 Interface Comum de Backend

A implementação deverá definir uma abstração comum para os backends.

Conceitualmente, essa interface deverá oferecer operações equivalentes a:

```text
Backend
├── identificar o backend
├── declarar targets suportados
├── validar configuração
├── receber IR validada
├── configurar ABI
├── configurar nível de otimização
├── emitir código objeto
├── emitir representação intermediária
├── produzir informações de depuração
├── reportar falhas internas
└── declarar recursos suportados
```

A forma concreta dessa interface pertence à implementação Rust.

---

## 5.13 Configuração de Backend

O Driver deverá ser capaz de selecionar explicitamente o backend.

Conceitualmente:

```text
capic --backend cranelift
capic --backend llvm
```

Durante a fase inicial, apenas Cranelift estará disponível.

A seleção padrão poderá evoluir por perfil de compilação.

A ausência de suporte de um backend a determinado target ou recurso deverá produzir diagnóstico claro.

---

## 5.14 Níveis de Otimização

Os níveis públicos de otimização deverão representar intenções independentes do backend.

Por exemplo:

```text
O0 — sem otimizações relevantes
O1 — otimizações básicas
O2 — otimizações gerais
O3 — otimizações agressivas
Os — redução de tamanho
Oz — redução máxima de tamanho
```

Cada backend poderá mapear esses níveis para capacidades próprias.

O comportamento semântico deverá permanecer equivalente em todos os níveis.

Durante o bootstrap inicial, poderá existir apenas um subconjunto desses perfis.

---

## 5.15 Responsabilidades do Middle-end

O Middle-end deverá realizar otimizações independentes de backend.

Entre elas poderão estar:

* simplificação de controle de fluxo;
* propagação de constantes;
* eliminação de código inalcançável;
* canonicalização;
* eliminação de operações redundantes;
* especializações próprias da Capi;
* otimizações relacionadas a Domains;
* eliminação de verificações desnecessárias;
* devirtualização quando comprovável.

Os backends poderão executar otimizações adicionais.

Entretanto, a IR entregue a cada backend deverá representar um programa semanticamente completo.

---

## 5.16 Responsabilidades Específicas do Backend

Cada backend será responsável por:

* tradução para sua representação interna;
* seleção de instruções;
* alocação de registradores;
* atendimento à ABI;
* emissão de código objeto;
* relocação compatível;
* produção de símbolos;
* integração com o linker;
* informações de depuração, quando suportadas;
* otimizações específicas de target.

Nenhum backend deverá redefinir regras de tipagem ou semântica.

---

## 5.17 Integração com o Linker

Na fase inicial, o compilador poderá utilizar um linker externo.

Entre as possibilidades encontram-se:

* linker padrão do sistema;
* LLVM `lld`;
* outro linker compatível com ELF e System V AMD64.

O Driver deverá coordenar:

* arquivos objeto;
* Runtime;
* bibliotecas;
* símbolos externos;
* argumentos do linker;
* produção do executável final.

A implementação de um linker próprio está fora do escopo inicial.

---

## 5.18 Artefatos Intermediários

Cada backend deverá permitir, quando aplicável, a emissão de artefatos para depuração.

Backend Cranelift:

* Cranelift IR;
* código objeto;
* assembly, quando disponível;
* informações de relocação.

Backend LLVM:

* LLVM IR textual;
* bitcode;
* assembly;
* código objeto;
* informações de otimização.

Esses artefatos não fazem parte da semântica pública da linguagem.

---

## 5.19 Compatibilidade Funcional entre Backends

Os backends oficiais deverão ser funcionalmente equivalentes.

A conformidade será validada através de:

* suíte comum de testes;
* programas funcionais;
* testes de Runtime;
* testes de Domains;
* testes de objetos;
* testes de ABI;
* testes diferenciais.

O comportamento observável deverá permanecer igual.

---

## 5.20 Testes Diferenciais de Backend

Após a introdução do LLVM, o mesmo programa poderá ser compilado pelos dois backends.

Serão comparados:

* saída padrão;
* saída de erro;
* código de retorno;
* efeitos sobre arquivos;
* comportamento de funções;
* comportamento dos Domains;
* ordem de finalização definida pela linguagem;
* resultados numéricos;
* resultados de identidade;
* comportamento de despacho;
* falhas controladas.

Diferenças deverão ser classificadas como:

* diferença permitida de implementação;
* limitação documentada;
* erro de backend;
* ambiguidade da especificação.

---

## 5.21 Recursos Específicos de Backend

Nenhum recurso da linguagem deverá existir apenas em um backend.

Caso uma plataforma ou backend ofereça capacidades adicionais, elas deverão ser tratadas como:

* extensões explicitamente identificadas;
* recursos de target;
* funcionalidades experimentais;
* intrínsecos controlados.

Programas que dependam dessas capacidades deverão declarar sua dependência de forma explícita.

---

## 5.22 Política de Falhas de Backend

Uma falha de tradução de uma IR previamente validada representa, em regra, um erro interno do compilador.

Ela não deverá ser apresentada como erro semântico do programa.

Os backends deverão distinguir:

* target não suportado;
* configuração inválida;
* recurso ainda não implementado;
* falha do backend externo;
* inconsistência da IR;
* erro interno do adaptador.

---

## 5.23 Backends Futuros

A arquitetura poderá suportar futuramente:

* WebAssembly;
* ARM;
* AArch64;
* RISC-V;
* backend C;
* backend nativo próprio;
* JIT;
* outros ambientes.

Essas possibilidades não fazem parte do primeiro ciclo de implementação.

A arquitetura comum deverá apenas evitar impedimentos desnecessários à sua incorporação futura.

---

# 6. Decisões Complementares de Implementação

## 6.1 Objetivo

Este capítulo registra decisões iniciais necessárias para orientar a implementação oficial.

Essas decisões complementam a escolha de Rust, Cranelift, LLVM e Linux x86-64.

Elas representam estratégias iniciais.

Poderão evoluir conforme a implementação revelar novas necessidades, desde que:

* as garantias da linguagem sejam preservadas;
* as mudanças sejam documentadas;
* os impactos sejam avaliados;
* os testes sejam atualizados;
* decisões relevantes sejam registradas por ADR.

---

## 6.2 Estratégia Inicial para Generics

A implementação oficial adotará inicialmente **monomorfização** para generics.

Isso significa que o compilador poderá produzir uma versão especializada de uma função ou tipo genérico para cada combinação concreta de tipos utilizada pelo programa.

Exemplo conceitual:

```text
Vector<Int32>
Vector<String>
```

poderão resultar em representações especializadas distintas.

---

## 6.3 Justificativa da Monomorfização

A monomorfização foi escolhida inicialmente porque:

* simplifica a geração de código;
* facilita integração com Cranelift;
* facilita integração com LLVM;
* permite otimizações especializadas;
* evita dependência inicial de metadados genéricos em Runtime;
* reduz complexidade da ABI interna;
* preserva tipos concretos durante o Lowering;
* facilita o bootstrap da Biblioteca Padrão.

A principal desvantagem esperada é o aumento potencial do tamanho dos binários.

Essa questão será avaliada posteriormente.

---

## 6.4 Limites da Estratégia de Generics

A monomorfização constitui uma decisão da implementação inicial.

Ela não deverá ser exposta como garantia semântica da linguagem.

Implementações futuras poderão adotar:

* compartilhamento de código;
* dicionários de tipos;
* especialização parcial;
* estratégias híbridas.

A ABI pública deverá considerar cuidadosamente a representação de elementos genéricos antes de assumir estabilidade definitiva.

---

## 6.5 Runtime Inicial em Rust

O Runtime inicial será implementado em Rust.

Ele deverá fornecer apenas os serviços necessários para materializar:

* inicialização do programa;
* encerramento;
* Domain raiz;
* criação e destruição de Domains;
* alocação de objetos;
* metadados essenciais;
* suporte inicial a `ObjectId`;
* finalização;
* erros fatais;
* primitivas mínimas de IO;
* integração com o código gerado.

Partes do Runtime poderão utilizar `unsafe` quando necessário, respeitando a política definida neste documento.

---

## 6.6 Evolução do Runtime

O Runtime poderá ser parcialmente reimplementado em Capi no futuro.

Entretanto, componentes profundamente dependentes de:

* sistema operacional;
* ABI;
* alocação de baixo nível;
* inicialização de processo;
* FFI;
* instruções específicas;
* suporte ao backend;

poderão permanecer em Rust ou em código de plataforma.

A auto-hospedagem do compilador não exige necessariamente que todo o Runtime seja implementado em Capi.

---

## 6.7 Estratégia Inicial de Alocação dos Domains

A primeira implementação dos Domains utilizará uma estratégia simples baseada em:

* arenas;
* blocos de memória;
* tabela de objetos;
* metadados por Domain;
* destruição coletiva.

A prioridade será:

* correção;
* estabilidade de identidade;
* previsibilidade;
* facilidade de depuração;
* baixo acoplamento.

Não será objetivo inicial produzir o alocador mais eficiente possível.

---

## 6.8 Estrutura Conceitual Inicial de Domain

Conceitualmente, um Domain poderá manter:

```text
Domain
├── identificador
├── estado de lifetime
├── blocos de memória
├── tabela de objetos
├── gerações
├── metadados de tipos
├── informações de finalização
└── estado de mutação
```

Essa estrutura é uma decisão inicial de engenharia.

Sua representação concreta poderá mudar.

---

## 6.9 Representação Inicial de `ObjectId`

A representação inicial de `ObjectId` deverá ser opaca para o programa Capi.

Conceitualmente, poderá conter:

```text
ObjectId
├── identificador do Domain
├── índice ou slot do objeto
└── geração
```

O identificador do Domain permite localizar o contexto de armazenamento.

O slot permite localizar o registro correspondente ao objeto.

A geração impede que um identificador antigo passe a representar validamente um novo objeto após reutilização interna.

---

## 6.10 Garantias da Representação de `ObjectId`

A implementação deverá assegurar:

* identidade estável;
* unicidade durante o lifetime;
* associação imutável ao Domain;
* preservação durante upcast;
* comparação determinística;
* invalidação após destruição do Domain;
* proteção contra reutilização indevida;
* independência do endereço físico.

Um `ObjectId` não deverá conceder automaticamente:

* acesso direto à memória;
* autoridade de mutação;
* extensão de lifetime;
* ownership.

---

## 6.11 Tipagem de `ObjectId`

A HIR e a IR deverão preservar informações suficientes para distinguir:

```text
ObjectId<Conta>
ObjectId<Cliente>
ObjectId<Animal>
```

Upcasts deverão preservar a identidade subjacente.

Downcasts deverão respeitar as regras definidas pelo Sistema de Tipos e pela Semântica Operacional.

A representação física poderá ser compartilhada.

A interpretação estática permanecerá tipada.

---

## 6.12 Capacidade de Mutação

A Capacidade de Mutação deverá ser representada explicitamente nas fases semânticas.

A implementação deverá identificar:

* o Domain associado à operação;
* a natureza leitora ou mutante do acesso;
* conflitos de autoridade;
* sobreposição de operações;
* escapes;
* transferências entre contextos;
* requisitos para chamadas mutantes.

A representação poderá utilizar:

* estados associados à HIR;
* identificadores internos de capacidade;
* restrições;
* análise de fluxo;
* efeitos;
* operações explícitas na IR.

A estratégia concreta será detalhada nos capítulos dedicados ao Runtime e ao modelo de Domains.

---

## 6.13 Capacidade de Mutação na IR

A IR deverá preservar informações suficientes para:

* validar o Lowering;
* impedir reordenações inválidas;
* orientar otimizações;
* representar efeitos;
* eliminar operações puramente conceituais quando seguras;
* inserir suporte de Runtime quando necessário.

Quando a exclusividade for inteiramente comprovada em compilação, a capacidade poderá não possuir representação física no código final.

---

## 6.14 Análise Conservadora Inicial

A primeira implementação da Capacidade de Mutação poderá adotar uma análise conservadora.

Isso significa que o compilador poderá rejeitar alguns programas que seriam teoricamente seguros, caso ainda não consiga comprovar essa segurança.

A análise conservadora poderá reduzir ergonomia temporariamente.

Ela não poderá permitir programas inseguros.

A evolução posterior deverá ampliar progressivamente os casos aceitos sem enfraquecer as garantias.

---

## 6.15 Estratégia Inicial de Layout de Objetos

A implementação inicial deverá preservar o layout prefixado definido pela especificação.

Conceitualmente:

```text
Classe Base
├── metadados aplicáveis
└── campos da classe base

Classe Derivada
├── representação da classe base
└── campos adicionais
```

Essa organização deverá permitir:

* upcast de custo constante;
* preservação da identidade;
* acesso consistente aos campos herdados;
* despacho compatível;
* ABI previsível.

---

## 6.16 Reordenação de Campos

O compilador poderá reorganizar fisicamente campos quando permitido pela especificação.

Entretanto, durante o bootstrap inicial, poderá preservar a ordem declarada ou adotar regras simples de alinhamento.

Essa escolha facilita:

* depuração;
* validação;
* inspeção de memória;
* comparação entre representações.

Otimizações de layout poderão ser introduzidas posteriormente.

---

## 6.17 Estratégia Inicial de Despacho Dinâmico

A primeira implementação utilizará **tabelas virtuais**, ou mecanismo estruturalmente equivalente, para chamadas dinâmicas de métodos.

Conceitualmente, cada tipo polimórfico poderá possuir uma tabela contendo:

* identificador do tipo;
* ponteiros para métodos virtuais;
* informações de layout;
* rotina de finalização;
* metadados auxiliares.

As vtables pertencem à implementação.

A garantia pública é o despacho dinâmico correto.

---

## 6.18 Chamadas Estáticas e Dinâmicas

O Lowering deverá distinguir:

* chamada direta de função;
* chamada estática de método;
* chamada virtual;
* chamada por interface;
* chamada de trait;
* chamada de intrínseco;
* chamada ao Runtime;
* chamada FFI.

Essa distinção permitirá que os backends traduzam cada modalidade sem reinterpretar a semântica da linguagem.

---

## 6.19 Interfaces

A representação inicial de interfaces poderá utilizar:

* referência à identidade do objeto;
* referência à tabela de métodos da interface;
* metadados de tipo;
* informações de Domain quando necessárias.

Conceitualmente, uma referência por interface poderá ser representada como uma estrutura equivalente a:

```text
InterfaceRef
├── ObjectId
└── tabela de despacho da interface
```

A representação concreta será definida durante a implementação.

---

## 6.20 Traits

Traits deverão ser resolvidas predominantemente durante o Frontend e o Lowering.

Sempre que possível, métodos fornecidos por traits serão transformados em:

* métodos concretos;
* funções auxiliares;
* chamadas estáticas;
* implementações especializadas.

Traits não deverão introduzir estado físico adicional nos objetos.

Conflitos deverão ser resolvidos antes da geração da IR final.

---

## 6.21 Estratégia Inicial para Strings

A representação inicial de strings deverá priorizar:

* imutabilidade;
* Unicode;
* interoperabilidade com IO;
* previsibilidade;
* possibilidade de implementação posterior em Capi.

Inicialmente, operações essenciais poderão depender do Runtime Rust.

Uma representação conceitual poderá incluir:

```text
String
├── ponteiro ou referência interna
├── tamanho em bytes
└── metadados necessários
```

A semântica pública não deverá depender dessa representação.

---

## 6.22 Estratégia Inicial para Arrays

Arrays de tamanho fixo poderão ser representados diretamente no layout do valor ou objeto que os contém.

Arrays de tipos por identidade armazenarão identidades.

Arrays de tipos por valor armazenarão valores diretamente.

A estratégia deverá respeitar:

* alinhamento;
* tamanho;
* inicialização completa;
* acesso seguro;
* limites;
* associação ao Domain quando aplicável.

---

## 6.23 Estratégia Inicial para Coleções

Coleções dinâmicas da Biblioteca Mínima poderão ser inicialmente implementadas com apoio do Runtime.

A evolução deverá conduzir progressivamente a implementações em Capi.

Entre as coleções prioritárias encontram-se:

* `Vector<T>`;
* `Map<K,V>`;
* buffers;
* estruturas auxiliares do compilador.

A implementação inicial poderá privilegiar simplicidade em vez de máxima eficiência.

---

## 6.24 Representação de `Optional<T>`

A implementação poderá utilizar:

* enum discriminada;
* nichos de representação;
* ponteiros especiais;
* outras estratégias.

Inicialmente, recomenda-se uma representação explícita e previsível.

O uso de otimizações de nicho poderá ser incorporado posteriormente.

A semântica deverá permanecer uniforme.

---

## 6.25 Representação de `Result<T,E>`

`Result<T,E>` será tratado inicialmente como um tipo discriminado.

Sua representação deverá preservar:

* variante ativa;
* valor de sucesso;
* valor de erro;
* destruição correta;
* pattern matching;
* propagação explícita.

O tratamento de erros não dependerá de exceções implícitas.

---

## 6.26 Estratégia Inicial de Erros

A implementação inicial utilizará mecanismos explícitos de erro na linguagem.

No compilador escrito em Rust poderão ser utilizados:

* tipos `Result`;
* diagnósticos estruturados;
* erros internos diferenciados;
* propagação controlada.

Ao migrar componentes para Capi, deverão ser utilizadas as abstrações oficiais da linguagem.

---

## 6.27 Estratégia de FFI

A FFI completa não fará parte do primeiro subconjunto.

Inicialmente, será implementada apenas a interoperabilidade necessária para:

* chamadas ao Runtime;
* ligação com funções básicas da plataforma;
* IO mínimo;
* alocação;
* encerramento;
* integração com o linker.

A interface será ampliada somente após a estabilização da ABI inicial.

---

## 6.28 Intrínsecos

A implementação poderá utilizar intrínsecos para operações que:

* não possam ser expressas no subconjunto inicial;
* exijam suporte direto do backend;
* exijam instruções específicas;
* materializem conceitos fundamentais;
* forneçam ponte temporária para a Biblioteca Padrão.

Todo intrínseco deverá:

* possuir identificação explícita;
* possuir contrato documentado;
* permanecer restrito;
* ser representado na HIR ou IR;
* possuir testes;
* ser substituído por código Capi comum quando possível.

---

## 6.29 Identificadores Internos

A implementação deverá utilizar identificadores internos estáveis para representar elementos como:

* arquivos;
* módulos;
* símbolos;
* tipos;
* funções;
* blocos;
* valores;
* Domains;
* objetos;
* diagnósticos.

Esses identificadores deverão evitar dependência excessiva de referências Rust diretas entre módulos.

Essa estratégia favorece:

* arenas;
* serialização;
* testes;
* compilação incremental futura;
* rastreabilidade;
* reimplementação em Capi.

---

## 6.30 Internamento de Strings

Identificadores textuais recorrentes poderão ser internados.

O Interner deverá:

* produzir identificadores compactos;
* permitir comparação eficiente;
* preservar o texto original;
* manter determinismo;
* integrar-se aos spans;
* permanecer independente das fases semânticas.

O internamento não deverá normalizar silenciosamente identificadores distintos.

---

## 6.31 Arenas Internas do Compilador

A implementação em Rust poderá utilizar arenas para:

* AST;
* HIR;
* tipos;
* símbolos;
* IR;
* diagnósticos auxiliares.

As arenas internas do compilador não possuem relação semântica direta com os Domains da linguagem Capi.

Essa distinção deverá ser explicitamente documentada para evitar confusão entre:

```text
Arena de implementação do compilador
```

e:

```text
Domain definido pela linguagem Capi
```

---

## 6.32 Paralelismo do Compilador

A primeira implementação poderá ser sequencial.

A arquitetura deverá evitar impedimentos desnecessários à paralelização futura.

Entretanto, não será prioridade inicial implementar:

* parsing paralelo;
* compilação paralela de módulos;
* otimizações paralelas;
* geração de código paralela;
* cache distribuído.

O determinismo deverá ser preservado caso o paralelismo seja introduzido posteriormente.

---

## 6.33 Compilação Incremental

A compilação incremental está fora do primeiro ciclo.

Ainda assim, a arquitetura deverá favorecer futuramente:

* identificação estável de módulos;
* artefatos intermediários isolados;
* hashes de conteúdo;
* dependências explícitas;
* interfaces pequenas;
* ausência de estado global desnecessário;
* serialização possível de artefatos.

Nenhuma decisão inicial deverá exigir a implementação imediata dessas funcionalidades.

---

## 6.34 Política de Otimização Inicial

O compilador deverá iniciar com otimizações mínimas.

Prioridades iniciais:

* canonicalização;
* simplificação trivial;
* eliminação de código claramente inalcançável;
* propagação de constantes simples;
* validação da IR;
* preparação para o backend.

Otimizações agressivas deverão aguardar:

* estabilidade da IR;
* testes funcionais;
* testes diferenciais;
* backend LLVM;
* benchmarks representativos.

---

## 6.35 Política de Compatibilidade Binária

A ABI interna inicial poderá evoluir durante o bootstrap.

Não haverá compromisso de estabilidade binária definitiva nas primeiras versões.

Cada alteração deverá:

* ser documentada;
* atualizar o Runtime;
* atualizar os backends;
* atualizar os testes;
* manter consistência entre todos os componentes produzidos pela mesma versão.

A estabilidade formal da ABI será tratada antes da primeira versão estável do compilador.

---

## 6.36 Política de Dependências de Plataforma

Código específico de plataforma deverá permanecer isolado.

Conceitualmente:

```text
platform/
├── linux/
├── unix/
├── windows/
└── common/
```

Durante a fase inicial, apenas Linux será implementado.

As interfaces deverão permitir expansão futura sem espalhar condicionais de plataforma por todo o compilador.

---

## 6.37 Política de Logs

O compilador poderá possuir infraestrutura estruturada de logs para uso interno.

Os logs deverão ser distintos de:

* diagnósticos do programa;
* saída normal da Toolchain;
* erros internos;
* dumps de representações.

A ativação de logs deverá ser configurável por:

* fase;
* módulo;
* severidade;
* sessão;
* arquivo.

Logs não deverão alterar artefatos produzidos.

---

## 6.38 Política de Dumps

A emissão de representações intermediárias deverá ser determinística.

Cada dump deverá:

* identificar a fase;
* indicar o módulo ou função correspondente;
* preservar identidades internas de forma estável quando possível;
* evitar endereços de memória não determinísticos;
* permitir comparação em Golden Tests;
* possuir formato legível.

Dumps não constituem formato público estável, salvo decisão posterior.

---

## 6.39 Política de Diagnósticos

Os diagnósticos deverão ser estruturados internamente.

Cada diagnóstico deverá possuir, quando aplicável:

* código;
* severidade;
* mensagem principal;
* span principal;
* spans relacionados;
* notas;
* sugestões;
* origem da fase;
* categoria.

A apresentação textual será separada da representação interna.

Essa decisão permitirá futuramente:

* saída terminal;
* JSON;
* integração com IDE;
* testes estáveis;
* localização;
* internacionalização, se adotada.

---

## 6.40 Política de Erros Internos

Falhas internas deverão produzir informações suficientes para reprodução.

Entre elas:

* fase;
* módulo;
* versão do compilador;
* backend;
* target;
* opções relevantes;
* identificação do artefato;
* stack trace, quando disponível;
* instruções para obtenção de dumps.

Erros internos não deverão expor dados sensíveis do ambiente além do necessário.

---

## 6.41 Política de Licenças das Dependências

Todas as dependências deverão ser compatíveis com a licença adotada para o compilador oficial.

Antes da incorporação de uma dependência relevante, deverão ser avaliados:

* licença;
* obrigações de redistribuição;
* compatibilidade com GPLv3 e Runtime Exception, caso confirmadas;
* distribuição de código-fonte;
* vinculação estática ou dinâmica;
* impacto sobre o Runtime;
* uso em ferramentas de desenvolvimento.

As decisões deverão ser registradas quando apresentarem impacto jurídico ou de distribuição.

---

## 6.42 Registro das Decisões

As decisões arquiteturais principais deste bloco deverão possuir ADRs correspondentes.

Inicialmente:

```text
ADR-001 — Rust como linguagem hospedeira
ADR-002 — Linux x86-64 como primeiro target
ADR-003 — Cranelift como backend inicial
ADR-004 — LLVM como backend de evolução
ADR-005 — IR da Capi independente dos backends
ADR-006 — Monomorfização inicial de generics
ADR-007 — Runtime inicial em Rust
ADR-008 — Arena como estratégia inicial de Domain
ADR-009 — ObjectId baseado em Domain, slot e geração
ADR-010 — Vtables como estratégia inicial de despacho
```

O Documento 27 registra a estratégia geral.

Os ADRs deverão registrar contexto, alternativas e consequências detalhadas.

---

## 6.43 Caráter Evolutivo das Decisões

As decisões deste capítulo representam o ponto de partida oficial.

Nenhuma delas deverá ser considerada imutável quando se referir exclusivamente a mecanismo de implementação.

Uma decisão poderá ser revisada quando:

* testes demonstrarem inadequação;
* novas evidências técnicas surgirem;
* a evolução da linguagem exigir adaptação;
* uma dependência deixar de ser mantida;
* uma alternativa oferecer benefícios significativos;
* a auto-hospedagem alterar requisitos da implementação.

Toda revisão deverá preservar os contratos permanentes da linguagem e ser formalmente documentada.

---

# 7. Organização do Repositório

## 7.1 Objetivo

A organização física da implementação oficial tem como objetivo favorecer:

* separação clara de responsabilidades;
* baixo acoplamento entre componentes;
* evolução incremental;
* facilidade de manutenção;
* testes independentes;
* reutilização de componentes;
* integração contínua;
* futura auto-hospedagem.

A estrutura do repositório deverá refletir diretamente a arquitetura lógica definida nos Documentos 13 a 26.

O objetivo não é apenas organizar arquivos, mas tornar explícitas as fronteiras arquiteturais da implementação.

---

## 7.2 Estratégia de Monorepo

A implementação oficial adotará inicialmente uma estratégia de **monorepo**.

Todos os componentes oficiais serão mantidos em um único repositório.

Essa decisão busca facilitar:

* sincronização entre módulos;
* evolução coordenada;
* controle de versões;
* testes integrados;
* bootstrap;
* integração contínua;
* rastreabilidade entre alterações.

Durante o bootstrap, a separação em múltiplos repositórios aumentaria significativamente a complexidade operacional sem oferecer benefícios proporcionais.

---

## 7.3 Estrutura Geral do Repositório

Conceitualmente, o repositório poderá possuir a seguinte organização:

```text
capi/
├── compiler/
├── runtime/
├── std/
├── bootstrap/
├── tests/
├── examples/
├── tools/
├── docs/
├── scripts/
└── spec/
```

Cada diretório representa um domínio arquitetural distinto.

A existência física desses diretórios poderá evoluir ao longo da implementação, desde que suas responsabilidades permaneçam preservadas.

---

## 7.4 Diretório `compiler`

Contém a implementação do compilador propriamente dito.

Não deverá conter:

* Runtime;
* Biblioteca Padrão;
* exemplos;
* testes funcionais completos;
* documentação da linguagem;
* scripts auxiliares.

Seu objetivo é transformar programas Capi em artefatos intermediários ou executáveis.

---

## 7.5 Diretório `runtime`

Contém a implementação do Runtime oficial.

Inclui componentes responsáveis por:

* inicialização;
* encerramento;
* Domains;
* gerenciamento de objetos;
* suporte mínimo de execução;
* integração com código gerado.

O Runtime deverá permanecer separado do compilador.

Essa separação facilita:

* manutenção;
* testes;
* futura reimplementação parcial;
* adaptação para novos targets.

---

## 7.6 Diretório `std`

Contém a Biblioteca Padrão.

Inicialmente poderá existir em duas formas:

* implementações em Rust utilizadas durante o bootstrap;
* implementações progressivamente reescritas em Capi.

A existência temporária dessas duas implementações não representa duplicação definitiva.

Ela constitui parte da estratégia de migração.

---

## 7.7 Diretório `bootstrap`

Contém materiais específicos do processo de bootstrap.

Exemplo conceitual:

```text
bootstrap/
├── stage0/
├── stage1/
├── stage2/
├── scripts/
└── validation/
```

Seu conteúdo poderá incluir:

* scripts de bootstrap;
* procedimentos de validação;
* comparação entre estágios;
* geração dos primeiros compiladores;
* automação da auto-hospedagem.

---

## 7.8 Diretório `tests`

Contém a suíte oficial de testes.

Inclui:

* testes de conformidade;
* testes funcionais;
* Golden Tests;
* programas inválidos;
* programas válidos;
* benchmarks;
* testes diferenciais.

Os testes deverão permanecer independentes da implementação específica dos módulos.

---

## 7.9 Diretório `examples`

Contém programas utilizados como documentação executável da linguagem.

Esses programas não substituem a suíte de testes.

Seu objetivo principal é demonstrar o uso da linguagem.

---

## 7.10 Diretório `tools`

Contém ferramentas auxiliares utilizadas durante o desenvolvimento.

Exemplos:

* geradores;
* conversores;
* validadores;
* analisadores;
* ferramentas de benchmark;
* inspeção de artefatos.

Essas ferramentas não fazem parte do compilador propriamente dito.

---

## 7.11 Diretório `docs`

Contém documentação técnica complementar.

Inclui:

* ADRs;
* documentos internos;
* diagramas;
* especificações auxiliares;
* documentação da implementação.

Não substitui a documentação normativa da linguagem.

---

## 7.12 Diretório `spec`

Contém cópia ou referência oficial aos Documentos 00–27 utilizados durante o desenvolvimento.

Seu objetivo é manter a especificação próxima da implementação.

Isso facilita:

* consulta;
* revisão;
* rastreabilidade;
* validação da conformidade.

---

# 8. Arquitetura Física do Compilador

## 8.1 Objetivo

A arquitetura física do compilador organiza a implementação em componentes especializados, cada um responsável por uma única etapa do pipeline.

Essa organização busca:

* modularidade;
* baixo acoplamento;
* alta coesão;
* facilidade de testes;
* possibilidade de evolução independente.

---

## 8.2 Organização Geral

Conceitualmente:

```text
compiler/
├── driver/
├── session/
├── source/
├── diagnostics/
├── lexer/
├── parser/
├── ast/
├── hir/
├── resolver/
├── typechecker/
├── semantic/
├── lowering/
├── ir/
├── optimizer/
├── backend/
├── common/
└── util/
```

Essa organização reflete diretamente a arquitetura especificada nos Documentos 13 a 26.

---

## 8.3 Driver

O Driver coordena todo o processo de compilação.

Entre suas responsabilidades encontram-se:

* interpretar opções de linha de comando;
* iniciar a sessão de compilação;
* selecionar backend;
* organizar o pipeline;
* controlar múltiplos módulos;
* acionar Runtime e linker;
* produzir artefatos finais.

O Driver não implementa regras semânticas.

Ele apenas coordena os componentes especializados.

---

## 8.4 Session

A Session representa o contexto global de uma compilação.

Pode conter:

* configuração;
* target;
* backend selecionado;
* opções de otimização;
* cache interno;
* identificadores globais;
* estatísticas;
* registradores de diagnóstico.

Todas as fases deverão acessar informações globais através da Session.

---

## 8.5 Source Manager

Responsável pela representação dos arquivos-fonte.

Entre suas responsabilidades:

* leitura dos arquivos;
* identificação de módulos;
* offsets;
* linhas;
* colunas;
* spans;
* associação entre diagnósticos e código-fonte.

Nenhuma outra fase deverá acessar arquivos diretamente.

---

## 8.6 Diagnostics

Toda emissão de diagnósticos deverá ocorrer através de uma infraestrutura única.

Ela deverá oferecer:

* mensagens;
* severidade;
* códigos;
* spans;
* notas;
* sugestões;
* categorias;
* apresentação.

A infraestrutura deverá permanecer independente da saída textual.

---

## 8.7 Lexer

Responsável por transformar caracteres em tokens.

Seu resultado constitui a entrada do Parser.

Não realiza:

* resolução de nomes;
* tipagem;
* verificações semânticas.

---

## 8.8 Parser

Responsável por construir a AST.

Deverá implementar:

* recuperação de erros;
* associação de spans;
* produção de uma árvore sintática consistente.

Não deverá realizar validações semânticas.

---

## 8.9 AST

Representação sintática da linguagem.

Mantém fidelidade ao código-fonte.

É utilizada apenas como representação intermediária entre Parser e HIR.

Não deverá acumular informações semânticas complexas.

---

## 8.10 HIR

A HIR constitui a representação canônica utilizada pelo restante do compilador.

Ao longo do pipeline, ela será progressivamente enriquecida por:

* resolução de nomes;
* tipos;
* verificações semânticas;
* capacidades;
* informações necessárias ao Lowering.

---

## 8.11 Resolver

Responsável por:

* escopos;
* símbolos;
* imports;
* namespaces;
* resolução de referências;
* associação entre declarações e usos.

Produz a primeira HIR semanticamente conectada.

---

## 8.12 Type Checker

Responsável por:

* inferência;
* verificação;
* compatibilidade;
* operadores;
* chamadas;
* generics;
* constraints.

Produz a HIR tipada.

---

## 8.13 Semantic Checker

Responsável pelas regras que dependem do significado completo do programa.

Entre elas:

* Domains;
* Capacidade de Mutação;
* inicialização;
* retornos;
* uso de objetos;
* regras de herança;
* regras de interfaces;
* traits.

Produz a HIR semanticamente validada.

---

## 8.14 Lowering

Transforma a HIR validada na IR da implementação.

Essa transformação elimina construções exclusivamente sintáticas e preserva apenas os elementos necessários à geração de código.

---

## 8.15 IR

Representação intermediária oficial da implementação.

Serve como contrato entre:

* Middle-end;
* otimizações;
* backends.

Nenhum backend deverá modificar diretamente a IR.

---

## 8.16 Optimizer

Executa otimizações independentes de backend.

Exemplos:

* propagação de constantes;
* simplificação;
* eliminação de código morto;
* canonicalização.

O Optimizer não deverá introduzir otimizações específicas de Cranelift ou LLVM.

---

## 8.17 Backend

Responsável por traduzir a IR da Capi para o backend selecionado.

Sua implementação será organizada como:

```text
backend/
├── common/
├── cranelift/
└── llvm/
```

Cada backend deverá permanecer isolado.

---

## 8.18 Common

Contém componentes compartilhados.

Exemplos:

* identificadores;
* arenas;
* interner;
* utilidades;
* abstrações;
* estruturas auxiliares.

Não deverá acumular funcionalidades específicas de outras fases.

---

## 8.19 Util

Contém funções utilitárias reutilizáveis.

Não deverá conter lógica pertencente a módulos especializados.

---

# 9. Contratos entre os Componentes

## 9.1 Objetivo

A arquitetura do compilador é baseada em contratos explícitos entre suas fases.

Cada componente:

* recebe uma representação;
* realiza uma transformação;
* produz uma nova representação;
* preserva invariantes definidos.

Nenhuma fase deverá depender de detalhes internos das demais.

---

## 9.2 Fluxo Geral

O pipeline completo é representado conceitualmente por:

```text
Código-fonte
        │
        ▼
Lexer
        │
        ▼
Parser
        │
        ▼
AST
        │
        ▼
HIR
        │
        ▼
Resolver
        │
        ▼
Type Checker
        │
        ▼
Semantic Checker
        │
        ▼
Lowering
        │
        ▼
IR
        │
        ▼
Optimizer
        │
        ▼
Backend
        │
        ▼
Executável
```

Cada seta representa um contrato.

---

## 9.3 Contrato Source → Lexer

Entrada:

* arquivos;
* spans;
* sequência de caracteres.

Saída:

* sequência de tokens;
* diagnósticos léxicos.

---

## 9.4 Contrato Lexer → Parser

Entrada:

* tokens válidos;
* tokens inválidos recuperáveis.

Saída:

* AST;
* diagnósticos sintáticos.

---

## 9.5 Contrato Parser → AST

A AST deverá:

* representar integralmente a estrutura sintática;
* preservar spans;
* manter ordem do código;
* permanecer semanticamente neutra.

---

## 9.6 Contrato AST → HIR

A construção da HIR deverá:

* eliminar redundâncias sintáticas;
* normalizar estruturas;
* preservar rastreabilidade;
* manter correspondência com os elementos da AST.

---

## 9.7 Contrato Resolver → HIR

Após a Resolução de Nomes:

* todas as referências válidas deverão estar associadas às respectivas declarações;
* ambiguidades deverão estar resolvidas ou diagnosticadas.

---

## 9.8 Contrato Type Checker → HIR

Após a Tipagem:

* todas as expressões deverão possuir tipo;
* chamadas deverão estar resolvidas;
* operadores deverão estar validados;
* constraints deverão estar satisfeitas ou diagnosticadas.

---

## 9.9 Contrato Semantic Checker → HIR

Após a Verificação Semântica:

* o programa deverá satisfazer todas as regras da linguagem;
* Domains deverão estar consistentes;
* capacidades deverão estar válidas;
* regras de herança deverão estar verificadas.

---

## 9.10 Contrato Lowering → IR

A IR produzida deverá:

* ser semanticamente completa;
* não depender da AST;
* não depender da sintaxe original;
* estar pronta para otimizações.

---

## 9.11 Contrato Optimizer → Backend

O Optimizer deverá entregar uma IR válida.

O Backend poderá assumir que:

* invariantes foram preservados;
* tipos estão corretos;
* blocos estão completos;
* terminadores existem;
* a IR foi validada.

---

## 9.12 Contrato Backend → Runtime

A comunicação entre Backend e Runtime ocorrerá exclusivamente através da ABI interna definida para a implementação.

O Backend não deverá acessar estruturas privadas do Runtime.

Toda interação deverá ocorrer por interfaces documentadas.

---

## 9.13 Contrato Runtime → Programa

O Runtime deverá fornecer apenas os serviços mínimos necessários à execução.

Programas Capi não deverão depender de detalhes internos de sua implementação.

---

## 9.14 Invariantes Arquiteturais

Cada fase deverá declarar explicitamente:

* pré-condições;
* pós-condições;
* invariantes;
* erros recuperáveis;
* falhas internas.

Esses contratos constituem a principal garantia de evolução independente dos componentes e permitem que a arquitetura permaneça estável mesmo diante da futura substituição ou evolução de módulos individuais.

---

# 10. Sequência de Desenvolvimento dos Componentes

## 10.1 Objetivo

A implementação do compilador oficial da Capi será realizada em uma sequência incremental, orientada por dependências arquiteturais e por resultados executáveis.

A ordem de desenvolvimento deverá reduzir riscos técnicos, evitar a construção prematura de componentes sem uso imediato e permitir que cada novo recurso atravesse o pipeline completo.

A sequência definida neste capítulo não representa uma divisão rígida e imutável do trabalho.

Pequenos ajustes poderão ocorrer conforme a implementação revelar dependências adicionais.

Entretanto, alterações significativas deverão preservar os seguintes princípios:

* infraestrutura antes das fases que dependem dela;
* representações estáveis antes das transformações que as consomem;
* validação antes de otimização;
* código executável desde os primeiros marcos;
* expansão progressiva do subconjunto da linguagem;
* separação entre Frontend, Middle-end, Backend e Runtime;
* ausência de dependências circulares entre componentes.

---

## 10.2 Estratégia Geral de Implementação

A implementação não deverá seguir uma abordagem puramente horizontal na qual cada fase seja concluída integralmente antes do início da fase seguinte.

Em vez disso, será adotada uma combinação de:

* fundação horizontal mínima;
* cortes verticais progressivos;
* expansão incremental de cada fase;
* validação contínua entre os componentes.

A fundação horizontal inicial deverá fornecer apenas os elementos compartilhados indispensáveis.

Em seguida, o primeiro corte vertical deverá percorrer:

```text
Código-fonte
        │
        ▼
Lexer
        │
        ▼
Parser
        │
        ▼
AST
        │
        ▼
HIR
        │
        ▼
Verificação mínima
        │
        ▼
IR
        │
        ▼
Cranelift
        │
        ▼
Executável
```

Cada corte posterior deverá ampliar esse pipeline.

---

## 10.3 Ordem Geral das Etapas

A sequência geral de desenvolvimento será:

```text
1. Workspace e infraestrutura comum
2. Source Manager e Spans
3. Diagnósticos
4. Lexer
5. Parser e AST
6. HIR
7. Resolução de Nomes
8. Inferência e Verificação de Tipos
9. Verificação Semântica
10. Lowering
11. IR
12. Backend Cranelift
13. Linkedição
14. Runtime inicial
15. Biblioteca mínima
16. Modelo de objetos
17. Domains e ObjectId
18. Capacidade de Mutação
19. Herança e despacho dinâmico
20. Generics
21. Componentes necessários ao bootstrap
22. Reimplementação progressiva em Capi
23. Backend LLVM
```

Essa sequência representa dependências predominantes.

Alguns componentes poderão evoluir em paralelo quando seus contratos estiverem suficientemente claros.

---

## 10.4 Etapa 1 — Workspace e Infraestrutura Comum

A primeira etapa deverá estabelecer o ambiente mínimo da implementação.

Deverão ser criados:

* workspace Cargo;
* crates ou módulos iniciais;
* convenções de nomenclatura;
* estrutura do repositório;
* pipeline básico de build;
* formatação;
* análise estática;
* execução de testes;
* documentação técnica inicial.

Também deverão ser definidos:

* MSRV;
* edição do Rust;
* política de dependências;
* política de `unsafe`;
* convenções para erros;
* convenções para identificadores internos;
* convenções para testes.

O objetivo dessa etapa não é criar abstrações genéricas antecipadamente.

Somente deverão ser adicionados componentes cuja necessidade já esteja identificada.

---

## 10.5 Etapa 2 — Identificadores Internos

Antes das representações principais, deverão ser definidos identificadores internos para elementos como:

* arquivos;
* módulos;
* nós da AST;
* nós da HIR;
* símbolos;
* tipos;
* funções;
* blocos básicos;
* valores da IR;
* diagnósticos.

Conceitualmente:

```text
FileId
ModuleId
AstNodeId
HirNodeId
SymbolId
TypeId
FunctionId
BlockId
ValueId
```

Esses identificadores deverão:

* ser opacos;
* evitar o uso de índices crus fora do componente responsável;
* permitir comparação eficiente;
* facilitar rastreabilidade;
* permanecer independentes de endereços de memória;
* favorecer serialização futura.

Nem todos precisarão existir desde a primeira linha de código.

Deverão ser introduzidos conforme as respectivas representações forem implementadas.

---

## 10.6 Etapa 3 — Source Manager

O Source Manager deverá ser um dos primeiros componentes funcionais.

Ele será responsável por:

* carregar arquivos;
* atribuir `FileId`;
* armazenar o conteúdo original;
* localizar offsets;
* calcular linha e coluna;
* fornecer trechos de código;
* associar posições aos diagnósticos;
* controlar nomes lógicos e caminhos físicos.

O Source Manager deverá suportar inicialmente um único arquivo.

O suporte a múltiplos arquivos e módulos será incorporado posteriormente.

---

## 10.7 Etapa 4 — Spans

O conceito de `Span` deverá ser introduzido junto ao Source Manager.

Conceitualmente:

```text
Span
├── FileId
├── posição inicial
└── posição final
```

Spans deverão ser:

* compactos;
* copiáveis ou referenciáveis com baixo custo;
* independentes do texto armazenado;
* preservados ao longo do Frontend;
* combináveis quando necessário;
* adequados à emissão de diagnósticos.

A implementação deverá prever também um mecanismo para elementos sintéticos sem origem direta no código-fonte.

---

## 10.8 Etapa 5 — Infraestrutura de Diagnósticos

A infraestrutura de diagnósticos deverá ser implementada antes da expansão do Lexer e do Parser.

Ela deverá oferecer, inicialmente:

* severidade;
* código opcional;
* mensagem principal;
* span principal;
* notas;
* spans relacionados;
* saída textual simples.

A representação interna deverá permanecer separada da renderização.

Isso permitirá que os mesmos diagnósticos sejam utilizados futuramente em:

* terminal;
* JSON;
* LSP;
* testes;
* ferramentas gráficas.

---

## 10.9 Etapa 6 — Interner

Um mecanismo de internamento deverá ser introduzido quando o Lexer começar a reconhecer identificadores.

O Interner deverá associar textos a identificadores compactos.

Conceitualmente:

```text
Symbol
    │
    ▼
texto internado
```

O Interner não deverá realizar:

* resolução de nomes;
* diferenciação entre declarações;
* associação de escopos;
* classificação semântica.

Seu papel será exclusivamente representar textos recorrentes de forma eficiente e determinística.

---

## 10.10 Etapa 7 — Lexer Inicial

O primeiro Lexer deverá reconhecer apenas os tokens necessários ao primeiro corte vertical.

Inicialmente:

* palavras reservadas essenciais;
* identificadores;
* delimitadores;
* pontuação;
* literais inteiros mínimos;
* operadores necessários;
* espaços;
* comentários;
* fim de arquivo.

O Lexer deverá produzir uma sequência estruturada de tokens.

Conceitualmente:

```text
Token
├── espécie
├── Span
└── valor associado, quando aplicável
```

Erros léxicos deverão ser representados por diagnósticos, permitindo recuperação quando possível.

---

## 10.11 Etapa 8 — Testes do Lexer

Antes da expansão do Parser, o Lexer deverá possuir testes para:

* tokens isolados;
* sequências válidas;
* espaços e quebras de linha;
* comentários;
* literais;
* identificadores;
* palavras reservadas;
* operadores;
* caracteres inválidos;
* fim inesperado;
* rastreabilidade por spans.

Também deverão existir Golden Tests para sequências completas de tokens.

---

## 10.12 Etapa 9 — Parser Inicial

O primeiro Parser deverá reconhecer apenas a estrutura necessária ao primeiro programa.

Exemplo inicial:

```capi
function main() {
}
```

A primeira gramática deverá suportar:

* unidade de compilação;
* declaração de função;
* nome;
* lista vazia de parâmetros;
* bloco vazio.

A expansão deverá ocorrer progressivamente.

O Parser deverá consumir tokens e produzir AST.

---

## 10.13 Etapa 10 — AST Inicial

A primeira AST deverá representar diretamente a sintaxe reconhecida.

Exemplo conceitual:

```text
CompilationUnit
└── FunctionDeclaration
    ├── name
    ├── parameters
    ├── return type
    └── body
```

A AST deverá preservar:

* spans;
* ordem;
* elementos opcionais;
* informações necessárias à recuperação de erros.

Não deverá conter tipos inferidos, símbolos resolvidos ou informações de backend.

---

## 10.14 Etapa 11 — Recuperação Sintática

A recuperação de erros deverá ser incorporada progressivamente.

Inicialmente, poderá utilizar pontos de sincronização simples, como:

* fechamento de bloco;
* separador de declaração;
* início de nova declaração;
* fim do arquivo.

O objetivo será produzir mais de um diagnóstico por execução sem comprometer a consistência da AST.

Nós inválidos ou ausentes poderão possuir representações explícitas.

---

## 10.15 Etapa 12 — Construção Inicial da HIR

Após a AST mínima, deverá ser implementada a HIR inicial.

A HIR deverá normalizar diferenças sintáticas irrelevantes.

Mesmo no primeiro corte, deverá estabelecer:

* identidade própria dos nós;
* associação ao span de origem;
* estrutura canônica para funções;
* estrutura canônica para blocos;
* representação uniforme de tipos;
* separação em relação à AST.

A construção da HIR não deverá realizar ainda toda a verificação semântica.

---

## 10.16 Etapa 13 — Tabela de Módulos Inicial

Mesmo que o primeiro programa utilize apenas um arquivo, deverá existir uma representação mínima de módulo.

Isso evitará que conceitos globais sejam acoplados diretamente a arquivos.

Conceitualmente:

```text
Project
└── Module
    ├── declarations
    └── source files
```

A primeira versão poderá mapear:

```text
um arquivo = um módulo
```

Essa relação poderá evoluir conforme a especificação do sistema de módulos.

---

## 10.17 Etapa 14 — Resolução de Nomes Inicial

A primeira Resolução de Nomes deverá suportar:

* declaração de `main`;
* nomes de funções;
* parâmetros;
* variáveis locais;
* referências locais;
* chamadas de funções simples.

Deverão ser criadas estruturas para:

* escopos;
* símbolos;
* namespaces;
* declarações;
* referências.

A resolução inicial poderá ser restrita a um único módulo.

---

## 10.18 Etapa 15 — Tipos Primitivos Iniciais

O primeiro sistema de tipos deverá suportar apenas os tipos necessários aos cortes iniciais.

Por exemplo:

* `Unit`;
* `Bool`;
* `Int32`;
* tipos de função mínimos.

Deverão ser implementados:

* representação canônica dos tipos;
* `TypeId`;
* tabela ou arena de tipos;
* compatibilidade;
* igualdade;
* resolução de anotações;
* associação de tipos às expressões.

---

## 10.19 Etapa 16 — Verificação de Funções

O Type Checker inicial deverá validar:

* tipo dos parâmetros;
* tipo de retorno;
* chamadas;
* quantidade de argumentos;
* tipos dos argumentos;
* tipo do valor retornado;
* ausência de retorno incompatível.

A função `main` deverá possuir um contrato inicial claramente definido.

Esse contrato poderá evoluir conforme a Toolchain e a Biblioteca Padrão forem incorporadas.

---

## 10.20 Etapa 17 — Expressões e Operadores

O compilador deverá incorporar progressivamente:

* literais;
* referências locais;
* operadores aritméticos;
* operadores de comparação;
* operadores lógicos;
* agrupamento;
* atribuição;
* chamadas.

Cada novo operador deverá ser implementado simultaneamente em:

* Lexer;
* Parser;
* AST;
* HIR;
* tipagem;
* IR;
* Backend;
* testes.

---

## 10.21 Etapa 18 — Controle de Fluxo

Após expressões básicas, deverão ser adicionados:

* blocos;
* `if`;
* `else`;
* `while`;
* retorno;
* interrupções de fluxo previstas pelo subconjunto.

A Verificação Semântica deverá analisar:

* caminhos de retorno;
* blocos inalcançáveis;
* tipos de condições;
* escopo;
* inicialização de variáveis.

---

## 10.22 Etapa 19 — Lowering Inicial

O primeiro Lowering deverá transformar a HIR validada em uma IR mínima.

Deverá lidar inicialmente com:

* funções;
* parâmetros;
* constantes;
* operações aritméticas;
* retorno;
* chamadas;
* blocos;
* desvios condicionais.

O Lowering deverá assumir que:

* nomes estão resolvidos;
* tipos estão definidos;
* regras semânticas foram verificadas.

Nenhuma validação de alto nível deverá ser repetida desnecessariamente nessa fase.

---

## 10.23 Etapa 20 — IR Inicial

A primeira IR deverá ser pequena, explícita e validável.

Conceitualmente:

```text
Program
└── Function
    ├── parameters
    ├── local values
    └── basic blocks
        ├── instructions
        └── terminator
```

As primeiras instruções poderão incluir:

* constante;
* soma;
* subtração;
* multiplicação;
* comparação;
* chamada;
* retorno;
* salto;
* salto condicional.

---

## 10.24 Etapa 21 — Validador da IR

Antes da integração completa com Cranelift, deverá existir um validador da IR.

Ele deverá confirmar, inicialmente:

* identificação válida de funções;
* referências válidas a blocos;
* referências válidas a valores;
* compatibilidade de tipos;
* terminador em cada bloco;
* ausência de instruções após terminadores;
* quantidade correta de argumentos;
* tipo correto de retorno.

O backend deverá consumir somente IR validada.

---

## 10.25 Etapa 22 — Backend Cranelift Mínimo

O primeiro backend deverá traduzir:

* função `main`;
* valores inteiros;
* retorno;
* operações aritméticas básicas;
* blocos;
* desvios;
* chamadas simples.

O objetivo será produzir o primeiro código objeto válido.

Nenhum suporte a objetos, Domains ou generics será necessário nesse momento.

---

## 10.26 Etapa 23 — Emissão de Código Objeto

A geração inicial deverá produzir um arquivo objeto compatível com:

```text
Linux
x86-64
ELF
System V AMD64
```

O backend deverá controlar:

* assinatura das funções;
* símbolos;
* linkage;
* tipos nativos;
* relocations;
* chamadas externas necessárias.

O objeto produzido deverá ser inspecionável com ferramentas da plataforma.

---

## 10.27 Etapa 24 — Linkedição

O Driver deverá invocar um linker externo para produzir o executável final.

A primeira integração deverá suportar:

* arquivo objeto do programa;
* objeto ou biblioteca do Runtime;
* símbolos de entrada;
* bibliotecas essenciais;
* caminho de saída;
* diagnóstico em caso de falha.

O processo de linkedição deverá ser registrável em modo de desenvolvimento.

---

## 10.28 Etapa 25 — Primeiro Executável

O primeiro marco executável será atingido quando o compilador processar um programa mínimo e produzir um executável nativo válido.

Exemplo:

```capi
function main() {
}
```

O executável deverá:

* iniciar corretamente;
* executar `main`;
* encerrar corretamente;
* retornar o código esperado ao sistema operacional.

Esse marco valida o primeiro pipeline completo.

---

## 10.29 Etapa 26 — Runtime Mínimo Inicial

Após o primeiro executável, deverá ser estabelecido um Runtime mínimo.

Inicialmente poderá conter:

* função de inicialização;
* função de encerramento;
* adaptação da entrada do processo;
* suporte a falha fatal;
* futuras âncoras para o Domain raiz.

O Runtime deverá crescer apenas quando novos cortes exigirem serviços adicionais.

---

## 10.30 Etapa 27 — Variáveis Locais

A implementação deverá adicionar:

* declarações locais;
* inicialização;
* leitura;
* atribuição;
* escopo;
* shadowing, conforme especificação;
* verificação de uso antes de inicialização.

Na IR inicial, variáveis poderão ser reduzidas a valores SSA ou slots de memória conforme a estratégia escolhida.

---

## 10.31 Etapa 28 — Funções Completas

Deverão ser adicionados:

* múltiplos parâmetros;
* múltiplas funções;
* visibilidade inicial;
* chamadas entre funções;
* recursão;
* valores de retorno;
* funções sem retorno útil;
* validação da função `main`.

Esse marco deverá permitir programas estruturados de pequeno porte.

---

## 10.32 Etapa 29 — Tipos por Valor

Após as funções, deverão ser introduzidos os tipos por valor previstos para o subconjunto correspondente.

Inicialmente:

* structs;
* tuples;
* enums simples;
* arrays;
* `Optional`;
* `Result`.

Cada tipo deverá atravessar:

* sintaxe;
* HIR;
* layout;
* tipagem;
* semântica;
* IR;
* Backend;
* ABI interna;
* testes.

---

## 10.33 Etapa 30 — Layout de Tipos

Antes da geração de código para tipos compostos, deverá existir uma camada responsável por calcular:

* tamanho;
* alinhamento;
* offsets;
* discriminantes;
* estratégia de passagem;
* estratégia de retorno;
* representação ABI.

Essa camada deverá ser independente do backend sempre que possível.

O backend poderá adaptar o layout às exigências específicas do target.

---

## 10.34 Etapa 31 — Biblioteca Mínima

Com tipos por valor e funções suficientemente estáveis, deverá ser iniciada a Biblioteca Mínima de Bootstrap.

Prioridades:

* `String`;
* `Vector<T>`;
* `Optional<T>`;
* `Result<T,E>`;
* buffers;
* IO mínimo;
* acesso a argumentos;
* leitura e escrita de arquivos.

Parte dessa funcionalidade poderá ser inicialmente fornecida pelo Runtime.

---

## 10.35 Etapa 32 — Modelo de Objetos

O modelo de objetos deverá ser introduzido após a estabilização de:

* funções;
* tipos por valor;
* layout;
* chamadas;
* alocação básica;
* Runtime.

A primeira versão deverá suportar:

* classes;
* campos;
* métodos;
* construtores;
* identidade;
* acesso a membros.

Inicialmente, herança e despacho dinâmico poderão ser adiados.

---

## 10.36 Etapa 33 — Domains

Domains deverão ser introduzidos após o modelo de objetos básico.

A implementação deverá incluir:

* criação de Domain;
* Domain raiz;
* alocação de objetos;
* registro de objetos;
* destruição coletiva;
* finalização;
* invalidação;
* associação de objetos ao Domain.

Essa etapa deverá ser acompanhada por ferramentas de inspeção e testes específicos.

---

## 10.37 Etapa 34 — `ObjectId`

Após a alocação inicial em Domains, deverá ser implementado `ObjectId`.

Deverão ser validados:

* estabilidade;
* comparação;
* associação ao Domain;
* resolução de identidade;
* geração;
* invalidação;
* proteção contra reutilização.

O código-fonte Capi não deverá observar a representação física.

---

## 10.38 Etapa 35 — Capacidade de Mutação

A Capacidade de Mutação deverá ser incorporada após a existência de objetos e Domains.

A implementação será distribuída entre:

* HIR;
* Resolução de Nomes, quando aplicável;
* sistema de tipos;
* Verificação Semântica;
* análise de fluxo;
* IR;
* Runtime, quando necessário.

A primeira versão poderá ser conservadora.

---

## 10.39 Etapa 36 — Herança e Subtipagem

Após o modelo de identidade e mutação, deverão ser incorporados:

* herança simples;
* subtipagem;
* upcast;
* override;
* regras de visibilidade;
* classes abstratas;
* `final`;
* `sealed`.

O layout prefixado deverá ser validado pelos backends.

---

## 10.40 Etapa 37 — Despacho Dinâmico

O despacho dinâmico deverá ser introduzido com:

* vtables;
* slots estáveis durante uma compilação;
* chamadas virtuais;
* metadados de tipo;
* finalização polimórfica.

A geração deverá ser validada inicialmente em Cranelift.

---

## 10.41 Etapa 38 — Interfaces e Traits

Interfaces e traits deverão ser adicionadas somente após o modelo de classes estar funcional.

Deverão ser tratadas:

* declaração;
* implementação;
* compatibilidade;
* despacho;
* conflitos;
* métodos padrão;
* coerência;
* Lowering.

A representação física poderá ser introduzida progressivamente.

---

## 10.42 Etapa 39 — Generics

Generics deverão ser incorporados após:

* tipos;
* funções;
* classes;
* interfaces;
* traits;
* layout;
* backend;
* Biblioteca Mínima.

A primeira implementação utilizará monomorfização.

Deverão ser desenvolvidos:

* parâmetros de tipo;
* argumentos de tipo;
* constraints;
* inferência;
* instanciação;
* cache de especializações;
* controle de recursão;
* geração de símbolos.

---

## 10.43 Etapa 40 — Subconjunto Suficiente para o Compilador

Antes da migração de módulos para Capi, a linguagem deverá possuir recursos suficientes para implementar software não trivial.

Entre eles:

* módulos;
* funções;
* tipos por valor;
* classes;
* generics;
* strings;
* coleções;
* `Optional`;
* `Result`;
* IO;
* tratamento explícito de erros;
* pattern matching necessário;
* acesso controlado ao sistema;
* testes.

O conjunto exato será formalizado no capítulo dedicado ao subconjunto de bootstrap.

---

## 10.44 Etapa 41 — Primeiros Módulos em Capi

A reimplementação deverá começar por módulos de baixa dependência.

Possíveis candidatos:

* estruturas de identificadores;
* utilidades textuais;
* Lexer;
* partes do Parser;
* estruturas da AST.

Cada módulo reimplementado deverá ser comparado com sua versão Rust.

---

## 10.45 Etapa 42 — Backend LLVM

O backend LLVM deverá ser introduzido somente quando:

* a IR estiver estabilizada;
* o backend Cranelift estiver funcional;
* a suíte de testes for ampla;
* o Runtime estiver validado;
* o modelo de objetos estiver funcional;
* o processo inicial de bootstrap estiver encaminhado.

Sua introdução não deverá interromper a evolução do Cranelift.

---

## 10.46 Evolução Iterativa das Etapas

Nenhuma etapa deverá ser entendida como completamente encerrada após sua primeira implementação.

Por exemplo, o Lexer será criado cedo, mas continuará evoluindo quando forem introduzidos:

* novos literais;
* novos operadores;
* interpolação;
* atributos;
* novos modificadores.

Da mesma forma:

* a HIR será enriquecida;
* o sistema de tipos será ampliado;
* a IR receberá novas operações;
* o Runtime receberá novos serviços;
* os backends ganharão novos mappings.

A conclusão de uma etapa significa apenas que existe uma base estável para a fase seguinte.

---

# 11. Estratégia de Desenvolvimento Vertical

## 11.1 Objetivo

O desenvolvimento vertical tem como objetivo produzir resultados funcionais desde as primeiras fases do projeto.

Cada corte vertical deverá implementar um conjunto pequeno de recursos através de todas as camadas necessárias.

Isso reduz o risco de:

* incompatibilidade entre fases;
* representações inadequadas;
* interfaces excessivamente abstratas;
* validações tardias;
* acúmulo de componentes não executados;
* dependência prematura de funcionalidades futuras.

---

## 11.2 Definição de Corte Vertical

Um corte vertical é um incremento que começa no código-fonte e termina em um resultado observável.

Conceitualmente:

```text
Sintaxe
        │
        ▼
Representação
        │
        ▼
Validação
        │
        ▼
Lowering
        │
        ▼
Geração
        │
        ▼
Execução
```

O resultado observável poderá ser:

* executável;
* saída;
* código de retorno;
* diagnóstico;
* dump de representação;
* comportamento de Runtime.

---

## 11.3 Requisitos de um Corte Vertical

Cada corte deverá definir:

* objetivo;
* sintaxe envolvida;
* semântica;
* tipos envolvidos;
* alterações na AST;
* alterações na HIR;
* alterações na IR;
* suporte de backend;
* suporte de Runtime;
* testes válidos;
* testes inválidos;
* critério de conclusão.

Um corte não deverá ser iniciado apenas com base em uma alteração sintática.

---

## 11.4 Corte Vertical 0 — Infraestrutura

O primeiro corte não produz ainda código Capi.

Seu objetivo é validar a infraestrutura do projeto.

Entregas:

* workspace;
* build;
* teste inicial;
* Source Manager mínimo;
* diagnóstico mínimo;
* execução da CLI;
* integração contínua básica.

Exemplo de comando:

```text
capic --version
```

Esse corte valida o ambiente de engenharia.

---

## 11.5 Corte Vertical 1 — Programa Vazio

Programa de referência:

```capi
function main() {
}
```

Esse corte deverá implementar:

* tokens de declaração de função;
* identificador;
* parênteses;
* bloco;
* AST de função;
* HIR de função;
* tipo `Unit`;
* função `main`;
* IR de função vazia;
* retorno;
* geração Cranelift;
* linkedição;
* execução.

Critério principal:

```text
o executável inicia e encerra com sucesso
```

---

## 11.6 Corte Vertical 2 — Retorno Inteiro

Programa de referência:

```capi
function main() -> Int32 {
    return 0;
}
```

Esse corte deverá adicionar:

* anotação de retorno;
* literal inteiro;
* instrução `return`;
* tipo `Int32`;
* validação de retorno;
* constante na IR;
* retorno inteiro na ABI.

Critério principal:

```text
o executável retorna o código esperado
```

---

## 11.7 Corte Vertical 3 — Expressões Aritméticas

Programa de referência:

```capi
function main() -> Int32 {
    return 2 + 3 * 4;
}
```

Esse corte deverá adicionar:

* operadores;
* precedência;
* associatividade;
* expressão binária;
* verificação de operandos;
* instruções aritméticas;
* avaliação correta.

Critério principal:

```text
o resultado respeita precedência e semântica numérica
```

---

## 11.8 Corte Vertical 4 — Variáveis Locais

Programa de referência:

```capi
function main() -> Int32 {
    let value: Int32 = 10;
    return value;
}
```

Esse corte deverá adicionar:

* declaração local;
* anotação de tipo;
* inicialização;
* escopo local;
* resolução de referência;
* uso de valor na IR.

Critério principal:

```text
declaração e leitura locais produzem o valor esperado
```

---

## 11.9 Corte Vertical 5 — Atribuição

Programa de referência:

```capi
function main() -> Int32 {
    var value: Int32 = 10;
    value = 20;
    return value;
}
```

Esse corte deverá adicionar:

* distinção entre variável mutável e imutável;
* atribuição;
* validação de mutabilidade;
* atualização na representação intermediária.

Critério principal:

```text
atribuições válidas são aceitas e atribuições inválidas são rejeitadas
```

---

## 11.10 Corte Vertical 6 — Condicionais

Programa de referência:

```capi
function main() -> Int32 {
    if true {
        return 1;
    } else {
        return 0;
    }
}
```

Esse corte deverá adicionar:

* `Bool`;
* literal booleano;
* `if`;
* `else`;
* blocos;
* fluxo de controle;
* saltos condicionais;
* verificação de retorno por caminhos.

Critério principal:

```text
cada caminho executa conforme a condição
```

---

## 11.11 Corte Vertical 7 — Laços

Programa de referência:

```capi
function main() -> Int32 {
    var value: Int32 = 0;

    while value < 10 {
        value = value + 1;
    }

    return value;
}
```

Esse corte deverá adicionar:

* `while`;
* comparação;
* blocos de laço;
* back-edge na IR;
* fluxo de variável;
* análise de inicialização.

Critério principal:

```text
o laço produz o resultado esperado e mantém IR válida
```

---

## 11.12 Corte Vertical 8 — Funções

Programa de referência:

```capi
function sum(a: Int32, b: Int32) -> Int32 {
    return a + b;
}

function main() -> Int32 {
    return sum(2, 3);
}
```

Esse corte deverá adicionar:

* parâmetros;
* múltiplas funções;
* símbolos globais;
* chamadas;
* passagem de argumentos;
* retorno de chamadas;
* ABI interna.

Critério principal:

```text
funções podem chamar outras funções corretamente
```

---

## 11.13 Corte Vertical 9 — Estrutura por Valor

Programa de referência:

```capi
struct Point {
    x: Int32;
    y: Int32;
}
```

O uso concreto deverá validar:

* declaração;
* construção;
* campos;
* layout;
* acesso;
* passagem;
* retorno;
* destruição trivial.

Critério principal:

```text
valores compostos preservam layout e semântica
```

---

## 11.14 Corte Vertical 10 — Enumeração

Programa de referência conceitual:

```capi
enum Status {
    Active;
    Inactive;
}
```

Esse corte deverá validar:

* variantes;
* discriminante;
* construção;
* comparação;
* pattern matching mínimo, quando disponível;
* layout.

Critério principal:

```text
a variante ativa é preservada e interpretada corretamente
```

---

## 11.15 Corte Vertical 11 — `Optional`

Programa de referência:

```capi
function find() -> Optional<Int32> {
    return Some(10);
}
```

Esse corte deverá adicionar:

* tipo genérico inicial ou forma temporariamente especializada;
* variantes `Some` e `None`;
* construção;
* leitura;
* pattern matching ou operação equivalente.

Critério principal:

```text
presença e ausência são representadas sem valores especiais implícitos
```

---

## 11.16 Corte Vertical 12 — `Result`

Programa de referência:

```capi
function calculate() -> Result<Int32, Error> {
    return Ok(10);
}
```

Esse corte deverá validar:

* sucesso;
* erro;
* propagação explícita;
* destruição correta;
* layout discriminado.

Critério principal:

```text
os caminhos de sucesso e erro permanecem semanticamente distintos
```

---

## 11.17 Corte Vertical 13 — String Mínima

Programa de referência:

```capi
function main() {
    let message: String = "Olá, Capi";
}
```

Esse corte deverá adicionar:

* literal textual;
* representação de `String`;
* UTF-8 ou codificação definida;
* integração inicial com Runtime;
* destruição;
* comparação mínima.

Critério principal:

```text
strings válidas são representadas e preservadas corretamente
```

---

## 11.18 Corte Vertical 14 — Saída Padrão

Programa de referência:

```capi
function main() {
    println("Olá, Capi");
}
```

Esse corte deverá integrar:

* Biblioteca Mínima;
* função de saída;
* string;
* Runtime;
* chamada externa.

Critério principal:

```text
o programa produz a saída esperada
```

---

## 11.19 Corte Vertical 15 — Classe Básica

Programa de referência conceitual:

```capi
class Person {
    name: String;
}
```

O corte deverá adicionar:

* declaração de classe;
* campos;
* construção;
* alocação;
* identidade;
* acesso a campo;
* metadados básicos.

Critério principal:

```text
objetos possuem identidade e armazenamento corretos
```

---

## 11.20 Corte Vertical 16 — Métodos

Programa de referência:

```capi
class Counter {
    value: Int32;

    function get() -> Int32 {
        return self.value;
    }
}
```

Esse corte deverá adicionar:

* `self`;
* resolução de método;
* chamada de método;
* acesso a campo;
* tipo do receptor.

Critério principal:

```text
métodos operam sobre a instância correta
```

---

## 11.21 Corte Vertical 17 — Domain Básico

Programa de referência conceitual:

```capi
domain data {
    let person = Person(...);
}
```

Esse corte deverá adicionar:

* criação de Domain;
* associação de objeto;
* registro;
* destruição ao fim do escopo;
* invalidação.

Critério principal:

```text
objetos são destruídos deterministicamente com o Domain
```

---

## 11.22 Corte Vertical 18 — `ObjectId`

O corte deverá permitir:

* obtenção de identidade;
* armazenamento;
* comparação;
* resolução dentro do Domain;
* preservação em chamadas.

Critério principal:

```text
a identidade permanece estável mesmo quando a representação física muda
```

---

## 11.23 Corte Vertical 19 — Método Mutante

Programa de referência conceitual:

```capi
mutating function increment() {
    self.value = self.value + 1;
}
```

Esse corte deverá validar:

* declaração mutante;
* autoridade de mutação;
* acesso exclusivo;
* rejeição de chamada sem capacidade;
* efeito na IR.

Critério principal:

```text
somente contextos autorizados podem modificar o objeto
```

---

## 11.24 Corte Vertical 20 — Herança

Programa de referência:

```capi
class Animal {
}

class Dog extends Animal {
}
```

Esse corte deverá adicionar:

* relação base-derivada;
* subtipagem;
* layout prefixado;
* upcast;
* acesso a membros herdados.

Critério principal:

```text
um objeto derivado pode ser utilizado como base sem perda de identidade
```

---

## 11.25 Corte Vertical 21 — Despacho Dinâmico

Programa de referência conceitual:

```capi
class Animal {
    function speak() -> String;
}

class Dog extends Animal {
    override function speak() -> String {
        return "woof";
    }
}
```

Esse corte deverá adicionar:

* override;
* vtable;
* chamada virtual;
* seleção pelo tipo concreto.

Critério principal:

```text
a implementação executada corresponde ao tipo concreto do objeto
```

---

## 11.26 Corte Vertical 22 — Interface

Esse corte deverá validar:

* declaração de interface;
* implementação;
* conversão;
* chamada por interface;
* tabela de métodos correspondente.

Critério principal:

```text
o despacho por interface preserva identidade e comportamento
```

---

## 11.27 Corte Vertical 23 — Generic Simples

Programa de referência:

```capi
struct Box<T> {
    value: T;
}
```

Esse corte deverá adicionar:

* parâmetro de tipo;
* argumento concreto;
* instanciação;
* monomorfização;
* geração de símbolo especializado.

Critério principal:

```text
cada instanciação concreta é tipada e gerada corretamente
```

---

## 11.28 Corte Vertical 24 — Coleção Genérica

O corte deverá validar uma coleção necessária ao compilador, como:

```text
Vector<T>
```

Deverá incluir:

* alocação dinâmica;
* crescimento;
* acesso;
* iteração;
* destruição;
* monomorfização.

Critério principal:

```text
a linguagem dispõe de uma coleção genérica utilizável na implementação do compilador
```

---

## 11.29 Corte Vertical 25 — Primeiro Componente do Compilador em Capi

O primeiro módulo reimplementado deverá:

* possuir escopo reduzido;
* depender apenas de recursos já estáveis;
* ter equivalente em Rust;
* compartilhar casos de teste;
* produzir resultados comparáveis.

Critério principal:

```text
a versão Capi apresenta comportamento funcional equivalente à versão Rust
```

---

## 11.30 Critério de Priorização dos Cortes

A ordem dos cortes poderá ser ajustada conforme:

* dependências técnicas;
* riscos identificados;
* necessidade de validar conceitos centrais;
* requisitos do bootstrap;
* disponibilidade da Biblioteca Mínima.

Entretanto, deverá ser preservada a preferência por cortes que:

* produzam resultado executável;
* testem integração real;
* tenham escopo pequeno;
* validem conceitos centrais;
* evitem trabalho especulativo.

---

## 11.31 Proibição de Cortes Excessivamente Amplos

Um corte não deverá agrupar simultaneamente muitos conceitos independentes.

Por exemplo, não deverá ser criado um único corte para:

```text
classes + Domains + herança + generics + concorrência
```

Cada conceito deverá possuir introdução e validação próprias.

Essa separação facilita:

* diagnóstico de regressões;
* revisão;
* testes;
* documentação;
* reversão de mudanças.

---

## 11.32 Integração Contínua dos Cortes

Cada corte deverá ser integrado ao ramo principal somente quando:

* compilar;
* passar pelos testes;
* não quebrar cortes anteriores;
* possuir documentação mínima;
* possuir diagnóstico para casos inválidos;
* respeitar os contratos arquiteturais.

O ramo principal deverá permanecer funcional.

---

## 11.33 Demonstrações Executáveis

Alguns cortes poderão produzir pequenos programas de demonstração.

Esses programas deverão ser mantidos em `examples/`.

Eles não substituirão os testes.

Servirão para:

* comunicação do progresso;
* validação manual;
* documentação;
* apresentação pública;
* comparação entre versões.

---

# 12. Política de Conclusão de Componentes

## 12.1 Objetivo

Esta política define os critérios mínimos para considerar um componente, etapa ou corte suficientemente concluído para sustentar o avanço da implementação.

A conclusão não significa que o componente esteja definitivo.

Significa que seus contratos atuais são estáveis o bastante para serem utilizados pelas etapas seguintes.

---

## 12.2 Definição de Conclusão

Um componente será considerado concluído em determinado marco quando:

* seu escopo estiver explicitamente definido;
* suas responsabilidades estiverem implementadas;
* suas entradas e saídas estiverem documentadas;
* seus invariantes forem conhecidos;
* os testes obrigatórios forem aprovados;
* seus diagnósticos estiverem disponíveis;
* as limitações conhecidas estiverem registradas;
* não houver bloqueadores críticos para os consumidores seguintes.

---

## 12.3 Critérios Funcionais

O componente deverá executar corretamente todos os comportamentos previstos no marco correspondente.

Deverão existir casos para:

* entradas válidas comuns;
* entradas válidas limítrofes;
* entradas inválidas;
* ausência de entrada;
* combinações relevantes;
* casos de regressão conhecidos.

Comportamentos não implementados deverão ser explicitamente declarados.

---

## 12.4 Critérios de Correção

A implementação deverá preservar os contratos definidos pelos documentos correspondentes.

Não será suficiente produzir o resultado esperado em exemplos simples.

Deverão ser considerados:

* invariantes;
* ordem de avaliação;
* efeitos;
* tipos;
* lifetimes;
* Domains;
* identidade;
* mutação;
* ABI;
* determinismo.

Quando o componente não envolver determinado conceito, isso deverá ser claramente delimitado.

---

## 12.5 Testes Unitários

Cada componente deverá possuir testes unitários para comportamentos isoláveis.

Exemplos:

Lexer:

* reconhecimento de token;
* posição;
* literal;
* erro.

Parser:

* produção de nó;
* precedência;
* recuperação.

Type Checker:

* compatibilidade;
* inferência;
* erro.

IR:

* validação;
* tipos;
* blocos;
* terminadores.

Runtime:

* alocação;
* identidade;
* destruição.

---

## 12.6 Testes de Integração

Todo componente que se comunica com outras fases deverá possuir testes de integração.

Exemplos:

* Source Manager com Diagnostics;
* Lexer com Parser;
* AST com HIR;
* Resolver com Type Checker;
* HIR com Lowering;
* IR com Backend;
* Backend com Runtime;
* Driver com linker.

A integração deverá testar o contrato, não apenas chamadas isoladas.

---

## 12.7 Testes Funcionais

Recursos observáveis da linguagem deverão possuir testes funcionais completos.

Um teste funcional deverá conter:

* programa Capi;
* opções de compilação;
* resultado de compilação;
* execução, quando aplicável;
* saída esperada;
* código de retorno;
* efeitos esperados.

Testes funcionais deverão utilizar o compilador como um usuário externo.

---

## 12.8 Testes Negativos

Cada regra relevante deverá possuir casos inválidos.

Exemplos:

* sintaxe inválida;
* nome inexistente;
* tipo incompatível;
* retorno ausente;
* atribuição a valor imutável;
* acesso inválido a Domain;
* mutação sem capacidade;
* override incompatível;
* uso incorreto de generic.

O teste deverá validar não apenas a rejeição, mas também a categoria do diagnóstico.

---

## 12.9 Golden Tests

Representações textuais poderão ser validadas por Golden Tests.

Aplicações prioritárias:

* tokens;
* AST;
* HIR;
* tabela de símbolos;
* tipos;
* IR;
* diagnósticos;
* Cranelift IR;
* LLVM IR.

Golden Tests deverão evitar dados não determinísticos.

Endereços de memória, caminhos absolutos e identificadores instáveis deverão ser normalizados.

---

## 12.10 Testes de Regressão

Todo defeito confirmado deverá gerar um teste permanente.

O teste deverá:

* reproduzir o problema;
* falhar antes da correção;
* passar após a correção;
* permanecer na suíte.

O histórico de regressão não deverá depender apenas da descrição de issues.

---

## 12.11 Critérios de Diagnóstico

Um componente responsável por rejeitar entradas inválidas deverá produzir diagnóstico apropriado.

O diagnóstico deverá possuir, quando aplicável:

* código;
* categoria;
* mensagem principal;
* span correto;
* informação relacionada;
* sugestão;
* ausência de mensagens enganosas.

Falhas internas não deverão ser apresentadas como erros do usuário.

---

## 12.12 Qualidade dos Spans

Os spans deverão apontar para a menor região útil do código-fonte.

Deverão ser testados:

* início;
* fim;
* linha;
* coluna;
* múltiplos arquivos;
* Unicode;
* tokens compostos;
* elementos sintéticos.

Erros de localização deverão ser tratados como defeitos funcionais do compilador.

---

## 12.13 Critérios de Representação

Cada representação deverá possuir documentação sobre:

* finalidade;
* momento de criação;
* consumidores;
* invariantes;
* elementos temporários;
* dados preservados;
* dados eliminados.

Uma representação não deverá acumular responsabilidades de múltiplas fases sem justificativa.

---

## 12.14 Critérios de Validação Interna

Representações como HIR e IR deverão possuir validadores quando seus invariantes forem suficientemente complexos.

O validador deverá ser executado:

* em testes;
* em builds de desenvolvimento;
* após passes críticos;
* antes do backend;
* durante comparação de estágios, quando aplicável.

Em perfil de release do compilador, algumas validações poderão ser desativadas após avaliação.

---

## 12.15 Critérios de Determinismo

O componente deverá produzir resultados determinísticos quando receber entradas equivalentes.

Devem ser controladas fontes de variação como:

* ordem de mapas;
* geração de identificadores;
* paralelismo;
* timestamps;
* caminhos absolutos;
* aleatoriedade;
* ordem de descoberta de arquivos.

Quando alguma variação for permitida, ela deverá ser explicitamente documentada e não poderá afetar a semântica.

---

## 12.16 Critérios de Desempenho

O desempenho não será o critério principal durante o bootstrap.

Entretanto, um componente não deverá possuir comportamento claramente inviável para o estágio atual.

Deverão ser observados:

* crescimento assintótico;
* consumo de memória;
* cópias excessivas;
* recursão não limitada;
* explosão de especializações;
* repetição desnecessária de análises.

Problemas conhecidos poderão ser aceitos temporariamente quando documentados.

---

## 12.17 Critérios de Segurança

Componentes que utilizem `unsafe`, FFI ou operações de baixo nível deverão possuir critérios adicionais.

Deverão ser documentados:

* invariantes de segurança;
* pré-condições;
* pós-condições;
* ownership interno;
* validade de ponteiros;
* alinhamento;
* lifetime;
* condições de concorrência.

Testes e ferramentas de análise deverão ser utilizados sempre que aplicável.

---

## 12.18 Critérios de Integração com o Runtime

Um recurso que dependa do Runtime somente será considerado concluído quando:

* a ABI interna estiver definida;
* a chamada estiver gerada corretamente;
* o Runtime validar os argumentos;
* o retorno estiver correto;
* erros estiverem tratados;
* a ligação estiver automatizada;
* os testes funcionarem em executável real.

Mocks não serão suficientes como única validação.

---

## 12.19 Critérios de Integração com o Backend

Um recurso de linguagem somente será considerado executável quando:

* estiver representado na IR;
* a IR estiver validada;
* o backend realizar a tradução;
* o código objeto for válido;
* o linker produzir o executável;
* o comportamento final estiver testado.

A existência do recurso apenas na AST ou HIR não caracteriza conclusão funcional.

---

## 12.20 Critérios de Portabilidade Interna

Mesmo durante o suporte exclusivo a Linux x86-64, componentes independentes de plataforma não deverão utilizar detalhes do target sem necessidade.

Deverão ser mantidos separados:

* tipos semânticos;
* layout;
* ABI;
* emissão de objeto;
* chamadas de sistema;
* caminhos de arquivo específicos;
* ferramentas externas.

Essa separação será verificada durante revisões arquiteturais.

---

## 12.21 Critérios de Documentação

Cada componente deverá possuir documentação mínima contendo:

* propósito;
* responsabilidade;
* entradas;
* saídas;
* dependências;
* invariantes;
* erros;
* limitações;
* relação com os documentos de especificação.

Decisões não óbvias deverão ser justificadas.

APIs internas públicas entre crates deverão possuir documentação apropriada.

---

## 12.22 Critérios de Manutenção

O código deverá respeitar:

* formatação oficial;
* análise estática;
* ausência de warnings não justificados;
* organização modular;
* nomenclatura consistente;
* tratamento explícito de erros;
* ausência de código morto não documentado;
* testes próximos ao comportamento correspondente.

Warnings temporários deverão ser registrados e resolvidos antes do encerramento do marco, salvo justificativa formal.

---

## 12.23 Critérios de Dependências

Um componente não será considerado concluído se depender de biblioteca externa sem avaliação mínima.

Deverão ser verificados:

* licença;
* versão;
* manutenção;
* escopo de uso;
* dependências transitivas;
* isolamento arquitetural.

Dependências experimentais deverão ser explicitamente marcadas.

---

## 12.24 Critérios de Revisão Arquitetural

Antes do encerramento de um marco relevante, deverá ser realizada revisão para verificar:

* aderência aos Documentos 13 a 27;
* separação de responsabilidades;
* ausência de acoplamento indevido;
* estabilidade dos contratos;
* impacto sobre etapas seguintes;
* consistência com o princípio de separação entre garantias e mecanismos.

A revisão poderá resultar em refatoração antes da continuação.

---

## 12.25 Critérios para APIs Internas

Uma API interna entre módulos será considerada estável para o marco quando:

* sua finalidade estiver definida;
* seus tipos representarem corretamente o contrato;
* não expuser detalhes desnecessários;
* erros forem explícitos;
* invariantes forem documentados;
* testes de integração existirem.

Estabilidade interna de marco não significa estabilidade pública permanente.

---

## 12.26 Critérios para Marcação de Funcionalidade Experimental

Uma funcionalidade poderá ser integrada como experimental quando:

* houver necessidade de validação prática;
* seu contrato ainda estiver em evolução;
* não for necessária à estabilidade do marco;
* estiver isolada;
* estiver desativada por padrão, quando apropriado;
* possuir testes próprios;
* não comprometer comportamentos estáveis.

A marcação experimental deverá ser removida ou a funcionalidade deverá ser descartada após avaliação.

---

## 12.27 Critérios para Limitações Conhecidas

Toda limitação aceita deverá ser registrada com:

* descrição;
* impacto;
* motivo;
* fase afetada;
* possibilidade de workaround;
* marco previsto para revisão;
* risco associado.

Limitações críticas não poderão ser ocultadas em comentários locais.

---

## 12.28 Classificação de Defeitos

Os defeitos poderão ser classificados como:

### Bloqueador

Impede compilação, execução ou evolução da etapa seguinte.

### Crítico

Viola garantia semântica, segurança, identidade, Domain ou ABI.

### Alto

Produz comportamento incorreto em recurso suportado.

### Médio

Produz diagnóstico inadequado, limitação relevante ou inconsistência não crítica.

### Baixo

Afeta ergonomia, mensagem, organização ou caso marginal sem risco semântico.

Marcos não poderão ser concluídos com defeitos bloqueadores ou críticos conhecidos.

---

## 12.29 Critérios de Encerramento de Corte Vertical

Um corte vertical será considerado concluído quando:

* a sintaxe estiver implementada;
* a AST representar o recurso;
* a HIR estiver adequada;
* nomes estiverem resolvidos;
* tipos estiverem validados;
* regras semânticas estiverem verificadas;
* o Lowering estiver implementado;
* a IR estiver validada;
* o backend gerar código;
* o Runtime fornecer suporte, quando necessário;
* testes válidos e inválidos passarem;
* limitações estiverem documentadas.

Nem todas as fases precisarão sofrer alterações em todos os cortes.

Quando uma fase não for afetada, isso deverá ser confirmado.

---

## 12.30 Critérios de Encerramento de Componente

Um componente será considerado estável para determinado estágio quando:

* todos os cortes previstos para o estágio estiverem suportados;
* sua interface com componentes vizinhos estiver validada;
* sua suíte de testes estiver completa para o escopo;
* seus diagnósticos estiverem adequados;
* seus dumps estiverem disponíveis, quando aplicável;
* seus invariantes estiverem documentados;
* não houver dependências circulares;
* não houver falhas críticas conhecidas.

---

## 12.31 Critérios de Encerramento de Milestone

Um milestone será considerado concluído quando:

* todos os componentes previstos estiverem concluídos para o escopo;
* todos os cortes verticais correspondentes estiverem concluídos;
* a suíte de testes estiver aprovada;
* a integração contínua estiver aprovada;
* artefatos forem produzidos;
* exemplos correspondentes funcionarem;
* limitações estiverem consolidadas;
* documentação estiver atualizada;
* riscos para o próximo milestone estiverem avaliados.

---

## 12.32 Proibição de Critérios Subjetivos

Não deverão ser utilizados como únicos critérios expressões como:

* “parece funcionar”;
* “está suficientemente bom”;
* “a maior parte está pronta”;
* “funciona nos exemplos principais”;
* “pode ser corrigido depois”.

Todo encerramento deverá possuir evidências verificáveis.

---

## 12.33 Evidências de Conclusão

As evidências poderão incluir:

* relatórios de testes;
* saída da integração contínua;
* executáveis;
* dumps;
* benchmarks;
* exemplos;
* documentação;
* ADRs;
* comparações diferenciais;
* registros de conformidade.

Essas evidências deverão permanecer associadas à versão ou milestone correspondente.

---

## 12.34 Reabertura de Etapas

Uma etapa concluída poderá ser reaberta quando:

* uma regressão for identificada;
* a especificação for alterada;
* uma premissa se mostrar inválida;
* o componente seguinte revelar inadequação do contrato;
* um risco crítico surgir;
* um backend demonstrar incompatibilidade.

A reabertura deverá ser registrada e não será considerada falha do processo.

Ela representa aplicação do desenvolvimento incremental.

---

## 12.35 Princípio Final de Conclusão

Nenhum componente deverá ser considerado concluído apenas porque seu código foi escrito.

A conclusão exige a combinação de:

```text
implementação
+ validação
+ integração
+ testes
+ diagnósticos
+ documentação
+ ausência de bloqueadores
```

Somente essa combinação permite que as etapas seguintes dependam do componente com segurança.

---

# 13. Subconjunto Inicial da Linguagem

## 13.1 Objetivo

O subconjunto inicial da Capi define o menor conjunto de recursos necessário para construir, validar e evoluir o compilador oficial.

Esse subconjunto deverá permitir:

* validar o pipeline completo;
* produzir executáveis nativos;
* testar o Frontend;
* testar a HIR;
* testar a IR;
* testar o backend Cranelift;
* testar a linkedição;
* introduzir progressivamente o Runtime;
* estabelecer a base necessária para recursos mais complexos;
* preparar a futura implementação de componentes do compilador em Capi.

O subconjunto inicial não constitui uma linguagem separada.

Também não representa uma versão simplificada permanente da Capi.

Ele é uma seleção progressiva de recursos pertencentes à linguagem completa, implementados em uma ordem que minimize dependências e riscos técnicos.

---

## 13.2 Princípio de Compatibilidade Progressiva

Todo recurso introduzido no subconjunto inicial deverá preservar compatibilidade com a semântica definitiva da Capi.

Não será permitido implementar temporariamente uma regra incompatível apenas para simplificar o compilador inicial.

Por exemplo, o subconjunto não poderá:

* atribuir semântica diferente aos tipos;
* utilizar `null` onde a linguagem define `Optional`;
* introduzir Garbage Collector temporário;
* representar objetos como valores quando a especificação define identidade;
* permitir mutação sem Capacidade de Mutação;
* ignorar regras de lifetime dos Domains;
* tratar herança como cópia de objetos;
* introduzir exceções implícitas onde a linguagem utiliza `Result`.

Quando determinado recurso ainda não estiver implementado, a estratégia correta será restringir o programa aceito, e não alterar a semântica.

---

## 13.3 Princípio do Menor Subconjunto Útil

O subconjunto inicial deverá ser pequeno, mas deverá produzir resultados úteis e observáveis.

Não será suficiente implementar apenas:

* Lexer;
* Parser;
* AST;
* validações isoladas.

O primeiro subconjunto deverá alcançar a geração de executáveis.

A evolução será orientada pelo seguinte princípio:

> cada expansão do subconjunto deverá permitir escrever, compilar ou validar uma nova categoria real de programa.

---

## 13.4 Critérios de Inclusão de Recursos

Um recurso poderá ser incluído no subconjunto quando atender a pelo menos uma das seguintes condições:

* for necessário ao primeiro pipeline executável;
* for dependência direta de outro recurso prioritário;
* validar um conceito central da linguagem;
* permitir implementar programas não triviais;
* for necessário à Biblioteca Mínima de Bootstrap;
* for necessário à reimplementação do compilador em Capi;
* reduzir dependência temporária do Runtime Rust;
* permitir validar a arquitetura do modelo de objetos;
* permitir validar Domains, `ObjectId` ou Capacidade de Mutação.

A inclusão também deverá considerar:

* complexidade;
* risco técnico;
* dependências;
* capacidade de teste;
* impacto na IR;
* impacto no Runtime;
* impacto nos backends.

---

## 13.5 Critérios de Adiamento

Um recurso deverá ser adiado quando:

* depender de várias funcionalidades ainda não estabilizadas;
* não contribuir diretamente para o bootstrap;
* aumentar significativamente a complexidade do Runtime;
* exigir suporte de múltiplas plataformas;
* exigir otimizações avançadas;
* puder ser implementado posteriormente sem alterar a arquitetura;
* não puder ainda ser testado adequadamente;
* não possuir contratos suficientemente maduros.

O adiamento não deverá ser utilizado para ignorar conceitos centrais da Capi.

Domains, identidade e mutação, por exemplo, deverão ser implementados antes da auto-hospedagem, ainda que não pertençam ao primeiro executável.

---

## 13.6 Níveis do Subconjunto Inicial

O subconjunto será dividido em níveis progressivos.

Conceitualmente:

```text
Nível 0 — Estrutura mínima
Nível 1 — Expressões e controle de fluxo
Nível 2 — Funções e módulos
Nível 3 — Tipos por valor
Nível 4 — Biblioteca mínima
Nível 5 — Objetos e identidade
Nível 6 — Domains e mutação
Nível 7 — Orientação a objetos completa
Nível 8 — Generics e coleções
Nível 9 — Subconjunto de bootstrap
```

Cada nível dependerá predominantemente do anterior.

A implementação poderá introduzir elementos preparatórios de níveis posteriores quando necessário, mas não deverá antecipar sua complexidade integral.

---

## 13.7 Nível 0 — Estrutura Mínima do Programa

O primeiro nível deverá conter:

* unidade de compilação;
* módulo implícito inicial;
* declaração de função;
* função `main`;
* bloco;
* retorno implícito ou explícito de `Unit`;
* comentários;
* identificadores;
* delimitadores;
* erros léxicos;
* erros sintáticos.

Programa mínimo:

```capi
function main() {
}
```

Esse nível deverá permitir:

* compilação;
* geração de código objeto;
* linkedição;
* execução;
* encerramento correto.

---

## 13.8 Contrato Inicial da Função `main`

A função `main` deverá possuir contrato explícito.

Durante o primeiro corte, poderá ser aceita uma assinatura mínima equivalente a:

```capi
function main() {
}
```

Posteriormente, poderá ser admitida uma forma com retorno inteiro:

```capi
function main() -> Int32 {
    return 0;
}
```

A entrada com argumentos de linha de comando deverá ser adicionada somente após a existência das abstrações necessárias da Biblioteca Mínima.

A forma definitiva da função `main` deverá permanecer alinhada ao Documento 11 — Toolchain e aos demais documentos normativos aplicáveis.

---

## 13.9 Nível 1 — Tipos Primitivos Essenciais

Os primeiros tipos deverão ser:

* `Unit`;
* `Bool`;
* `Int32`.

A inclusão de outros tipos inteiros poderá ocorrer em seguida.

A implementação deverá possuir desde o início uma representação extensível de tipos, evitando codificar o sistema como um conjunto rígido de casos exclusivos para esses três tipos.

---

## 13.10 `Unit`

`Unit` representará ausência de valor significativo.

Deverá ser utilizado em:

* funções sem retorno útil;
* blocos sem valor;
* expressões cujo resultado seja apenas efeito;
* operações de inicialização e encerramento.

A representação física poderá não ocupar espaço quando o contexto permitir.

Entretanto, `Unit` deverá permanecer um tipo real no sistema.

---

## 13.11 `Bool`

`Bool` deverá possuir apenas dois valores semânticos:

```text
true
false
```

A representação física poderá utilizar inteiro, byte ou outra estratégia de backend.

Valores diferentes da representação válida não poderão ser observados por programas Capi.

---

## 13.12 `Int32`

`Int32` será o primeiro tipo numérico utilizado no pipeline.

Deverá suportar inicialmente:

* literais;
* soma;
* subtração;
* multiplicação;
* divisão, quando incorporada;
* resto, quando incorporado;
* comparação;
* retorno;
* parâmetros;
* variáveis locais.

As regras de overflow deverão seguir a especificação da linguagem e não poderão depender silenciosamente do comportamento padrão de Rust, Cranelift ou LLVM.

---

## 13.13 Expansão dos Tipos Numéricos

Após a validação de `Int32`, deverão ser incorporados progressivamente:

* demais inteiros assinados;
* inteiros não assinados;
* tipos de ponto flutuante;
* conversões explícitas;
* literais tipados;
* inferência numérica definida pela especificação.

A ordem deverá priorizar os tipos necessários à Biblioteca Mínima e ao compilador.

---

## 13.14 Nível 1 — Expressões

O subconjunto deverá suportar progressivamente:

* literais;
* referências;
* agrupamento;
* operadores unários;
* operadores binários;
* chamadas;
* atribuição;
* acesso a campos, quando objetos forem introduzidos;
* construção de valores.

A precedência e a associatividade deverão seguir integralmente a sintaxe oficial.

---

## 13.15 Operadores Iniciais

Os primeiros operadores deverão incluir, conforme necessidade dos cortes verticais:

```text
+
-
*
/
%
==
!=
<
<=
>
>=
&&
||
!
=
```

Nem todos precisam ser implementados no primeiro corte.

Cada operador deverá possuir:

* regra sintática;
* tipos de operandos;
* tipo de resultado;
* semântica;
* diagnóstico;
* representação na HIR;
* representação ou lowering na IR;
* suporte de backend.

---

## 13.16 Variáveis Locais

O subconjunto inicial deverá distinguir:

* valores imutáveis;
* variáveis mutáveis.

Conceitualmente:

```capi
let value: Int32 = 10;
var counter: Int32 = 0;
```

A sintaxe definitiva deverá seguir o Documento 04.

O compilador deverá validar:

* declaração;
* inicialização;
* escopo;
* mutabilidade;
* uso antes de inicialização;
* atribuição;
* shadowing, conforme as regras oficiais.

---

## 13.17 Inicialização Obrigatória

Durante o subconjunto inicial, recomenda-se exigir inicialização no momento da declaração sempre que a especificação permitir essa restrição.

Isso reduz a complexidade inicial da análise de fluxo.

Quando declarações sem inicialização fizerem parte do subconjunto, o compilador deverá comprovar inicialização antes de todo uso.

Nenhum valor não inicializado poderá ser observado.

---

## 13.18 Controle de Fluxo

O subconjunto inicial deverá incorporar:

* blocos;
* `if`;
* `else`;
* `while`;
* `return`.

Posteriormente poderão ser adicionados:

* `loop`;
* `for`;
* `break`;
* `continue`;
* pattern matching;
* demais construções previstas pela sintaxe.

Cada construção deverá ser reduzida a uma forma explícita na HIR ou IR.

---

## 13.19 Condições Booleanas

Condições de `if` e `while` deverão possuir tipo `Bool`.

Não deverá existir conversão implícita de:

* inteiros;
* ponteiros;
* objetos;
* strings;
* coleções;

para condição booleana, salvo regra normativa explícita em sentido contrário.

---

## 13.20 Retorno

O subconjunto deverá validar:

* retorno em funções com valor;
* retorno sem valor em funções `Unit`;
* incompatibilidade de tipos;
* retorno ausente em caminhos obrigatórios;
* código inalcançável, quando diagnosticável;
* retorno em contexto inválido.

A HIR deverá representar retorno de forma uniforme.

A IR deverá utilizar terminadores explícitos.

---

## 13.21 Nível 2 — Funções

Após o pipeline mínimo, deverão ser suportadas:

* funções livres;
* parâmetros;
* tipos de parâmetros;
* tipo de retorno;
* chamadas;
* recursão;
* múltiplas funções;
* sobrecarga, quando introduzida pelo sistema completo;
* visibilidade;
* símbolos externos controlados.

Funções serão fundamentais para todos os níveis posteriores.

---

## 13.22 Convenção de Chamadas Interna

Durante o subconjunto inicial, funções Capi deverão utilizar uma convenção de chamadas interna coerente e documentada.

Essa convenção deverá definir:

* passagem de parâmetros;
* retorno;
* representação de tipos compostos;
* tratamento de `Unit`;
* ligação com o Runtime;
* adaptação à ABI System V AMD64.

A convenção poderá evoluir antes da estabilidade formal da ABI.

---

## 13.23 Nível 2 — Módulos

A primeira implementação poderá utilizar um único módulo.

Entretanto, o subconjunto de bootstrap deverá obrigatoriamente suportar:

* múltiplos módulos;
* imports;
* visibilidade;
* resolução entre arquivos;
* dependências;
* inicialização, se aplicável;
* detecção de ciclos inválidos.

O compilador não poderá ser reimplementado adequadamente em um único arquivo ou módulo monolítico.

---

## 13.24 Evolução do Sistema de Módulos

A implementação deverá evoluir em estágios.

### Estágio inicial

```text
um arquivo = um módulo
```

### Estágio intermediário

```text
múltiplos arquivos
+ imports explícitos
+ visibilidade
```

### Estágio de bootstrap

```text
pacotes internos
+ hierarquia de módulos
+ dependências
+ resolução completa
```

A relação exata entre arquivo, módulo e pacote deverá seguir os Documentos 04, 11 e 12.

---

## 13.25 Nível 3 — Tipos por Valor

Os primeiros tipos compostos deverão ser tipos por valor.

Essa escolha permite validar:

* layout;
* alinhamento;
* cópia ou movimentação;
* passagem por parâmetro;
* retorno;
* destruição;
* pattern matching;
* agregados na IR.

Os tipos prioritários serão:

* structs;
* tuples;
* enums;
* arrays;
* `Optional`;
* `Result`.

---

## 13.26 Structs

Structs deverão suportar:

* campos nomeados;
* tipos de campos;
* construção;
* acesso;
* atribuição quando permitida;
* passagem;
* retorno;
* igualdade quando definida;
* destruição.

A primeira implementação poderá exigir inicialização explícita de todos os campos.

---

## 13.27 Tuples

Tuples deverão ser introduzidas quando necessárias para:

* retornos múltiplos conceituais;
* estruturas auxiliares;
* pattern matching;
* APIs internas do compilador.

A representação deverá preservar ordem e tipos.

---

## 13.28 Enums

Enums deverão suportar inicialmente:

* variantes sem dados;
* discriminante;
* construção;
* comparação;
* pattern matching mínimo.

Posteriormente deverão ser adicionadas variantes com dados associados.

Enums serão essenciais para representar:

* tokens;
* nós;
* resultados;
* estados;
* instruções;
* diagnósticos.

---

## 13.29 Arrays de Tamanho Fixo

Arrays de tamanho fixo deverão suportar:

* tamanho conhecido em compilação;
* elementos homogêneos;
* inicialização completa;
* indexação;
* verificação de limites;
* iteração futura;
* layout contíguo.

A representação de tamanho deverá fazer parte do tipo quando definido pela especificação.

---

## 13.30 Pattern Matching

O pattern matching será necessário para uso adequado de enums, `Optional` e `Result`.

A implementação poderá começar com:

* correspondência por variante;
* binding simples;
* caso curinga;
* verificação de exaustividade básica.

Posteriormente poderão ser adicionados:

* padrões aninhados;
* guards;
* padrões de structs;
* padrões de tuples;
* refinamentos.

O subconjunto necessário ao compilador deverá possuir pattern matching suficientemente expressivo para processar árvores e enums.

---

## 13.31 `Optional<T>`

`Optional<T>` deverá representar explicitamente presença ou ausência.

Conceitualmente:

```text
Optional<T>
├── Some(T)
└── None
```

Não deverá ser substituído por:

* valor sentinela;
* ponteiro nulo;
* zero;
* referência inválida.

A implementação poderá utilizar otimizações físicas posteriormente, desde que a semântica permaneça explícita.

---

## 13.32 `Result<T,E>`

`Result<T,E>` deverá representar operações que podem falhar.

Conceitualmente:

```text
Result<T,E>
├── Ok(T)
└── Error(E)
```

Ele será essencial para:

* IO;
* parsing interno;
* operações da Biblioteca Padrão;
* componentes do compilador reimplementados em Capi;
* propagação explícita de erros.

A linguagem deverá fornecer mecanismo ergonômico de propagação quando previsto na especificação.

---

## 13.33 Nível 4 — Recursos Básicos da Biblioteca

Antes do modelo de objetos completo, deverão existir abstrações mínimas para:

* texto;
* buffers;
* coleções;
* IO;
* argumentos;
* erros.

Esses recursos poderão utilizar implementações temporárias apoiadas pelo Runtime Rust.

O objetivo será permitir programas reais sem exigir ainda a implementação completa da Biblioteca Padrão.

---

## 13.34 Nível 5 — Classes Básicas

Após a estabilização dos tipos por valor, deverão ser introduzidas classes sem herança.

O subconjunto inicial de classes deverá suportar:

* declaração;
* campos;
* métodos;
* construtores;
* visibilidade mínima;
* identidade;
* alocação em Domain;
* acesso a `self`;
* métodos leitores;
* métodos mutantes em etapa posterior.

Classes não deverão ser representadas como structs copiáveis.

---

## 13.35 Identidade

Todo objeto de classe deverá possuir identidade independente de seu conteúdo.

A identidade deverá:

* permanecer estável;
* ser preservada em chamadas;
* ser preservada em coleções;
* ser preservada em upcasts futuros;
* não depender do endereço físico;
* estar associada ao Domain.

O subconjunto deverá validar identidade antes da introdução de herança.

---

## 13.36 Construtores

A primeira implementação de construtores deverá assegurar:

* inicialização completa;
* associação ao Domain;
* criação de identidade;
* validação de campos;
* ausência de observação do objeto parcialmente inicializado;
* tratamento de falhas conforme a semântica.

Construtores complexos poderão ser adicionados posteriormente.

---

## 13.37 Métodos Leitores

Métodos leitores deverão poder acessar o estado do objeto sem adquirir autoridade de mutação.

A implementação deverá representar explicitamente o receptor e seu tipo.

Chamadas leitoras deverão ser possíveis em contextos de leitura compartilhada permitidos pela linguagem.

---

## 13.38 Nível 6 — Domains

Domains deverão fazer parte do subconjunto antes da reimplementação relevante do compilador.

O suporte mínimo deverá incluir:

* Domain raiz;
* criação de Domain;
* bloco ou escopo associado;
* alocação de objetos;
* identidade;
* destruição determinística;
* invalidação;
* acesso controlado;
* diagnóstico de escape inválido.

A sintaxe deverá seguir a especificação oficial.

---

## 13.39 Domain Raiz

Todo programa deverá possuir um contexto inicial compatível com o conceito de Domain raiz, conforme definido pelo Modelo de Memória.

O Domain raiz será criado durante a inicialização do programa e encerrado durante sua finalização.

Sua existência deverá ser materializada pelo Runtime, mas preservada semanticamente pelo compilador.

---

## 13.40 Escopo de Domain

A implementação inicial poderá associar a vida de um Domain a um escopo léxico explícito.

Isso facilita:

* análise;
* diagnóstico;
* destruição;
* validação de escapes;
* testes.

A linguagem poderá permitir formas mais amplas conforme a especificação.

A implementação não deverá restringir permanentemente o modelo a uma única forma de construção.

---

## 13.41 Destruição Determinística

Ao término do lifetime de um Domain, deverão ocorrer:

* finalizações necessárias;
* invalidação das identidades;
* liberação dos blocos;
* atualização do estado interno;
* impossibilidade de novos acessos válidos.

A ordem de finalização deverá seguir os contratos definidos pela especificação.

---

## 13.42 Grafos e Ciclos

O subconjunto de Domains deverá permitir validar grafos de objetos, incluindo ciclos.

Exemplo conceitual:

```text
Objeto A → Objeto B
Objeto B → Objeto A
```

A destruição do Domain deverá liberar o grafo sem:

* Garbage Collector;
* contagem de referências;
* vazamento causado apenas pelo ciclo.

---

## 13.43 Nível 6 — `ObjectId`

`ObjectId` deverá ser incorporado junto ao modelo de Domains.

O subconjunto deverá permitir:

* obter identidade;
* armazenar identidade;
* comparar identidades;
* utilizar identidade em campos e coleções;
* resolver acesso dentro do contexto válido;
* preservar tipo;
* invalidar acesso após o término do Domain.

`ObjectId` não deverá ser convertido implicitamente em ponteiro.

---

## 13.44 Nível 6 — Capacidade de Mutação

Após o suporte de leitura, deverá ser introduzida a Capacidade de Mutação.

O subconjunto deverá suportar:

* métodos mutantes;
* campos mutáveis;
* aquisição de autoridade;
* propagação controlada;
* chamadas mutantes;
* rejeição de conflito;
* encerramento da autoridade;
* análise conservadora.

A Capacidade de Mutação deverá ser utilizada por programas do próprio compilador quando este for reimplementado em Capi.

---

## 13.45 Nível 7 — Herança e Subtipagem

A orientação a objetos completa deverá ser expandida com:

* herança simples;
* subtipagem;
* override;
* upcast;
* despacho dinâmico;
* classes abstratas;
* `final`;
* `sealed`;
* visibilidade herdada.

A herança não deverá ser introduzida antes da estabilização de identidade, layout e Domains.

---

## 13.46 Upcast

O upcast deverá:

* preservar a mesma identidade;
* preservar o mesmo Domain;
* manter acesso ao prefixo da classe base;
* selecionar corretamente métodos virtuais;
* não criar novo objeto.

Essa propriedade deverá ser testada explicitamente.

---

## 13.47 Override

O compilador deverá validar:

* existência do método base;
* compatibilidade de assinatura;
* modificador `override`;
* regras de retorno;
* visibilidade;
* mutabilidade;
* efeitos;
* posição na tabela de despacho.

Erros deverão ser diagnosticados antes do Lowering.

---

## 13.48 Despacho Dinâmico

O subconjunto deverá permitir chamadas através de tipos base e interfaces.

O método executado deverá corresponder ao tipo concreto.

A representação inicial utilizará vtables ou mecanismo equivalente.

---

## 13.49 Nível 7 — Interfaces

Interfaces deverão suportar:

* declaração;
* métodos requeridos;
* implementação;
* subtipagem;
* conversão;
* despacho;
* uso em parâmetros;
* uso em campos e coleções quando permitido.

A representação deverá preservar identidade e Domain.

---

## 13.50 Nível 7 — Traits

Traits deverão ser incorporadas quando necessárias para a Biblioteca Padrão e o compilador.

O subconjunto deverá incluir:

* declaração;
* implementação;
* métodos padrão;
* constraints;
* resolução de conflitos;
* chamadas estáticas ou dinâmicas conforme o contrato.

Traits não deverão introduzir estado por conta própria.

---

## 13.51 Nível 8 — Generics

Generics serão fundamentais para:

* coleções;
* `Optional`;
* `Result`;
* estruturas do compilador;
* algoritmos reutilizáveis.

O subconjunto deverá suportar:

* parâmetros de tipo;
* argumentos explícitos;
* inferência local;
* constraints;
* monomorfização;
* structs genéricas;
* funções genéricas;
* classes genéricas, quando necessárias;
* interfaces e traits genéricas, conforme o estágio.

---

## 13.52 Ordem de Implementação de Generics

Recomenda-se a seguinte ordem:

```text
1. Struct genérica
2. Função genérica
3. Enum genérica
4. Constraints simples
5. Inferência
6. Coleções genéricas
7. Classes genéricas
8. Interfaces e traits genéricas
```

Essa ordem poderá ser ajustada conforme a Biblioteca Mínima.

---

## 13.53 Constraints

A primeira implementação de constraints deverá ser suficiente para expressar requisitos de operações genéricas.

Exemplos conceituais:

* igualdade;
* ordenação;
* conversão textual;
* iteração;
* hashing;
* cópia ou movimentação, quando aplicável.

Constraints deverão ser verificadas estaticamente.

---

## 13.54 Monomorfização

Cada instanciação concreta deverá produzir uma especialização quando necessário.

O compilador deverá controlar:

* identidade da especialização;
* reutilização;
* geração de símbolos;
* recursão infinita;
* dependências;
* layout;
* instâncias não utilizadas.

O sistema deverá evitar gerar duplicações equivalentes desnecessárias.

---

## 13.55 Nível 9 — Subconjunto de Bootstrap

O subconjunto de bootstrap corresponde ao conjunto mínimo capaz de implementar componentes reais do compilador em Capi.

Ele deverá incluir, no mínimo:

* módulos;
* imports;
* visibilidade;
* funções;
* métodos;
* structs;
* enums;
* classes, quando necessárias;
* interfaces ou traits necessárias;
* generics;
* strings;
* vetores;
* mapas;
* `Optional`;
* `Result`;
* pattern matching;
* IO de arquivos;
* argumentos de linha de comando;
* buffers;
* conversões numéricas necessárias;
* tratamento explícito de erros;
* testes;
* diagnósticos básicos;
* Domains;
* identidade;
* mutação controlada.

O subconjunto exato poderá ser refinado conforme os primeiros módulos candidatos forem selecionados.

---

## 13.56 Recursos Não Necessariamente Exigidos para o Primeiro Bootstrap

O primeiro compilador auto-hospedado poderá não exigir:

* concorrência completa;
* reflexão;
* metaprogramação avançada;
* macros;
* compilação incremental;
* FFI completa;
* múltiplos backends implementados em Capi;
* debugger;
* profiler;
* todas as coleções da Biblioteca Padrão;
* todas as arquiteturas;
* todas as otimizações.

Esses recursos poderão ser incorporados depois da auto-hospedagem funcional.

---

## 13.57 Matriz de Recursos

O projeto deverá manter uma matriz relacionando:

* recurso;
* nível;
* milestone;
* dependências;
* estado;
* suporte no Frontend;
* suporte na IR;
* suporte no Cranelift;
* suporte no LLVM;
* suporte no Runtime;
* suporte na Biblioteca Padrão;
* testes.

Exemplo conceitual:

```text
Recurso        Nível   Frontend   IR   Cranelift   Runtime   Testes
Int32          1       Sim        Sim  Sim         Não       Sim
String         4       Sim        Sim  Sim         Sim       Sim
Class          5       Sim        Sim  Sim         Sim       Sim
Domain         6       Sim        Sim  Sim         Sim       Sim
Generics       8       Sim        Sim  Sim         Parcial   Sim
```

A matriz será detalhada em apêndice específico.

---

## 13.58 Controle de Recursos Incompletos

Recursos parcialmente implementados não deverão ser aceitos silenciosamente.

O compilador deverá:

* rejeitar sintaxe ainda não suportada;
* emitir diagnóstico claro;
* distinguir recurso inválido de recurso ainda não implementado;
* evitar gerar código parcialmente correto;
* não permitir ativação acidental em builds estáveis.

Recursos experimentais poderão ser protegidos por flags internas.

---

## 13.59 Critério de Conclusão do Subconjunto Inicial

O subconjunto inicial será considerado concluído quando:

* todos os níveis necessários ao primeiro bootstrap estiverem implementados;
* programas válidos forem compilados;
* programas inválidos forem rejeitados;
* a Biblioteca Mínima estiver disponível;
* pelo menos um módulo real do compilador puder ser escrito em Capi;
* o backend Cranelift suportar todos os recursos utilizados;
* o Runtime fornecer os serviços necessários;
* a suíte de testes estiver aprovada;
* as limitações estiverem documentadas.

---

# 14. Evolução Progressiva dos Recursos da Linguagem

## 14.1 Objetivo

A evolução dos recursos da linguagem deverá ocorrer de forma coordenada entre todas as fases do compilador.

Nenhum recurso será considerado implementado apenas por existir no Parser.

Cada evolução deverá considerar:

* sintaxe;
* AST;
* HIR;
* resolução;
* tipos;
* semântica;
* Lowering;
* IR;
* layout;
* backend;
* Runtime;
* Biblioteca Padrão;
* diagnósticos;
* testes;
* documentação.

---

## 14.2 Ciclo de Implementação de um Recurso

Todo novo recurso deverá seguir, de forma adaptada, o seguinte ciclo:

```text
1. Confirmar contrato normativo
2. Definir escopo do corte
3. Definir representação sintática
4. Definir representação na HIR
5. Definir regras de resolução
6. Definir regras de tipos
7. Definir regras semânticas
8. Definir Lowering
9. Definir operações da IR
10. Definir impacto no layout
11. Implementar backend
12. Implementar suporte de Runtime
13. Implementar diagnósticos
14. Criar testes válidos
15. Criar testes inválidos
16. Documentar limitações
```

Etapas não aplicáveis poderão ser omitidas explicitamente.

---

## 14.3 Estabilidade Antes de Expansão

Um recurso deverá atingir estabilidade mínima antes de servir de base para outro recurso mais complexo.

Por exemplo:

* funções antes de métodos;
* structs antes de classes;
* classes antes de herança;
* identidade antes de upcast;
* Domains antes de Capacidade de Mutação;
* enums antes de `Optional` e `Result`;
* generics antes de coleções genéricas;
* coleções antes da reimplementação ampla do compilador.

Essa política reduz retrabalho estrutural.

---

## 14.4 Evolução dos Tipos Primitivos

Os tipos primitivos deverão ser incorporados em grupos coerentes.

### Grupo inicial

* `Unit`;
* `Bool`;
* `Int32`.

### Grupo de inteiros

* inteiros assinados;
* inteiros não assinados;
* tamanhos de plataforma, se previstos.

### Grupo de ponto flutuante

* tipos definidos pela especificação;
* operações;
* conversões;
* valores especiais.

### Grupo textual básico

* `Char`, se definido;
* literais;
* codificação.

Cada grupo deverá possuir regras explícitas de conversão e promoção.

---

## 14.5 Conversões

A implementação deverá distinguir:

* conversão implícita permitida;
* conversão explícita;
* coerção;
* upcast;
* downcast;
* reinterpretation, quando permitida;
* conversão FFI.

O backend não deverá decidir conversões semânticas.

Toda conversão deverá estar explícita na HIR ou IR antes da geração de código.

---

## 14.6 Evolução dos Tipos por Valor

A evolução sugerida será:

```text
Structs simples
→ Tuples
→ Enums simples
→ Enums com dados
→ Arrays
→ Optional
→ Result
→ Pattern matching avançado
```

Cada etapa deverá reutilizar a infraestrutura de layout e destruição.

---

## 14.7 Movimentação e Cópia

A implementação deverá respeitar a distinção entre:

* tipos copiáveis;
* tipos movidos;
* tipos com destruição;
* tipos contendo objetos por identidade;
* tipos contendo recursos.

A semântica não poderá ser reduzida ao comportamento automático de tipos Rust.

A HIR e a IR deverão tornar explícitas as operações relevantes.

---

## 14.8 Destruição de Tipos por Valor

Tipos por valor que contenham recursos não triviais deverão possuir destruição correta.

A implementação deverá definir:

* ordem de destruição;
* destruição parcial em caso de falha de construção;
* destruição em retorno antecipado;
* destruição em branches;
* interação com `Result`;
* interação com Domains.

O Lowering deverá introduzir operações de limpeza quando necessário.

---

## 14.9 Evolução do Modelo de Objetos

A evolução recomendada será:

```text
Classe sem herança
→ Campos
→ Construtores
→ Métodos leitores
→ Identidade
→ Alocação em Domain
→ Métodos mutantes
→ Herança
→ Override
→ Despacho dinâmico
→ Classes abstratas
→ Interfaces
→ Traits
→ Sealed e final
```

Essa ordem valida progressivamente cada conceito.

---

## 14.10 Metadados de Tipo

Os metadados necessários ao modelo de objetos deverão ser introduzidos apenas conforme o uso.

Poderão incluir:

* identificador de tipo;
* tamanho;
* alinhamento;
* função de destruição;
* vtable;
* tabela de interface;
* informações de herança;
* nome para diagnóstico ou depuração.

Reflection não será introduzida automaticamente por esses metadados.

---

## 14.11 Evolução de Domains

A evolução dos Domains deverá ocorrer em etapas.

### Etapa 1 — Domain raiz

Contexto mínimo de execução.

### Etapa 2 — Domain explícito

Criação e destruição léxica.

### Etapa 3 — Alocação de objetos

Registro e identidade.

### Etapa 4 — Grafos e ciclos

Referências arbitrárias internas.

### Etapa 5 — Escape e lifetime

Validação de acessos.

### Etapa 6 — Capacidade de Mutação

Escrita exclusiva.

### Etapa 7 — Transferência e concorrência

Somente após o modelo sequencial estar estável.

---

## 14.12 Evolução de `ObjectId`

A implementação de `ObjectId` deverá começar com uma representação simples e verificável.

A evolução poderá incluir:

* compactação;
* melhoria da resolução;
* caches;
* especialização por Domain;
* remoção de verificações redundantes;
* representação otimizada de geração;
* integração com depuração.

Nenhuma otimização poderá comprometer identidade ou validade.

---

## 14.13 Evolução da Capacidade de Mutação

A primeira análise poderá ser lexical e conservadora.

Posteriormente poderá incorporar:

* análise de fluxo;
* efeitos;
* inferência;
* interproceduralidade;
* eliminação de capacidades temporárias;
* melhor diagnóstico;
* aceitação de padrões mais complexos;
* integração com concorrência.

A evolução deverá aumentar expressividade sem reduzir segurança.

---

## 14.14 Evolução da Herança

A implementação deverá começar com:

* herança simples;
* layout prefixado;
* métodos virtuais básicos.

Posteriormente:

* classes abstratas;
* métodos abstratos;
* `final`;
* `sealed`;
* covariância e contravariância quando previstas;
* downcast;
* interfaces múltiplas;
* otimizações de despacho.

Herança múltipla de implementação não deverá ser introduzida se não fizer parte da especificação.

---

## 14.15 Evolução das Interfaces

A implementação poderá começar com interfaces puramente comportamentais.

Posteriormente deverá incorporar:

* herança entre interfaces;
* métodos padrão, se definidos;
* generics;
* constraints;
* conversões;
* otimização de chamadas;
* devirtualização.

A ABI de referências por interface deverá ser documentada antes de sua estabilização.

---

## 14.16 Evolução das Traits

Traits deverão ser introduzidas após a resolução de:

* coerência;
* conflitos;
* especialização, se aplicável;
* relação com interfaces;
* relação com generics;
* métodos padrão.

A implementação inicial deverá evitar mecanismos excessivamente complexos de seleção.

---

## 14.17 Evolução de Generics

Generics deverão evoluir de casos simples para casos compostos.

### Fase inicial

* um parâmetro;
* tipos concretos;
* structs;
* funções.

### Fase intermediária

* múltiplos parâmetros;
* enums;
* constraints;
* inferência.

### Fase de bootstrap

* coleções;
* classes;
* interfaces;
* traits;
* métodos genéricos;
* especializações recursivas controladas.

---

## 14.18 Controle da Explosão de Monomorfização

O compilador deverá possuir mecanismos para:

* reutilizar especializações;
* detectar ciclos;
* limitar diagnósticos repetidos;
* calcular dependências;
* medir crescimento;
* evitar especializações não utilizadas;
* identificar símbolos deterministicamente.

Benchmarks deverão acompanhar o crescimento de binários e tempo de compilação.

---

## 14.19 Evolução do Pattern Matching

A implementação deverá começar com enums simples.

Posteriormente deverá suportar:

* dados associados;
* binding;
* padrões aninhados;
* tuples;
* structs;
* ranges, se previstos;
* guards;
* exaustividade;
* detecção de casos inalcançáveis.

O pattern matching será importante para o Parser, HIR, IR e diagnósticos escritos em Capi.

---

## 14.20 Evolução do Tratamento de Erros

A primeira Biblioteca Mínima deverá utilizar `Result`.

Posteriormente deverão ser adicionados:

* operador ou mecanismo de propagação;
* conversão de erros;
* composição;
* contexto;
* mensagens;
* integração com IO;
* diagnósticos internos;
* erros de sistema.

O compilador reimplementado em Capi deverá evitar abortos em erros recuperáveis.

---

## 14.21 Evolução do IO

A evolução sugerida será:

```text
Saída padrão
→ Saída de erro
→ Argumentos
→ Leitura de arquivo
→ Escrita de arquivo
→ Diretórios
→ Metadados
→ Paths
→ Streams e buffers
```

Somente os recursos necessários ao compilador deverão compor a Biblioteca Mínima.

---

## 14.22 Evolução da FFI

A FFI inicial será restrita às chamadas necessárias ao Runtime.

Posteriormente deverá evoluir para:

* tipos C básicos;
* funções;
* structs compatíveis;
* ponteiros controlados;
* callbacks;
* bibliotecas dinâmicas;
* convenções de chamada;
* erros;
* segurança.

A FFI pública não deverá ser definida apenas por necessidades internas do Runtime.

---

## 14.23 Evolução da Concorrência

A concorrência deverá ser introduzida somente após:

* Domains sequenciais estáveis;
* identidade estável;
* Capacidade de Mutação validada;
* Runtime confiável;
* testes de destruição;
* semântica operacional consolidada.

A primeira implementação poderá suportar apenas execução sequencial.

Essa restrição não altera a arquitetura futura.

---

## 14.24 Critério de Promoção de Recursos

Um recurso experimental será promovido ao subconjunto oficial quando:

* seu contrato estiver definido;
* sua implementação atravessar o pipeline;
* seus testes passarem;
* seus diagnósticos forem adequados;
* suas limitações forem conhecidas;
* não introduzir regressões;
* sua integração com Runtime e backend estiver validada;
* a documentação estiver atualizada.

---

## 14.25 Remoção de Implementações Temporárias

Durante o bootstrap poderão existir implementações temporárias.

Exemplos:

* `String` fornecida pelo Runtime;
* `Vector` implementado em Rust;
* operações de IO intrínsecas;
* enums especializadas antes de generics;
* funções internas especiais.

Essas implementações deverão possuir plano de remoção.

Nenhuma ponte temporária deverá se transformar silenciosamente em arquitetura permanente.

---

## 14.26 Compatibilidade dos Programas do Subconjunto

Programas válidos escritos no subconjunto deverão continuar válidos quando os recursos completos forem incorporados, salvo correção formal da especificação.

Mudanças incompatíveis decorrentes de erro do compilador deverão ser documentadas.

O projeto não deverá prometer estabilidade pública antes da versão definida para esse compromisso.

---

## 14.27 Critério de Conclusão da Evolução para Bootstrap

A evolução será considerada suficiente para iniciar a migração do compilador quando:

* o subconjunto de bootstrap estiver disponível;
* as coleções necessárias estiverem funcionais;
* strings e IO forem confiáveis;
* erros puderem ser tratados;
* módulos forem suportados;
* generics estiverem disponíveis;
* a compilação de programas médios for estável;
* o backend Cranelift suportar todo o conjunto;
* o Runtime estiver suficientemente maduro;
* os testes de conformidade forem abrangentes.

---

# 15. Biblioteca Mínima de Bootstrap

## 15.1 Objetivo

A Biblioteca Mínima de Bootstrap deverá fornecer as abstrações necessárias para:

* escrever programas Capi úteis;
* validar os recursos da linguagem;
* reduzir dependência de intrínsecos;
* reimplementar componentes do compilador;
* executar testes;
* manipular arquivos;
* representar texto;
* representar coleções;
* tratar erros.

Ela não corresponde à Biblioteca Padrão completa.

Seu escopo deverá ser estritamente orientado às necessidades do bootstrap.

---

## 15.2 Princípios

A Biblioteca Mínima deverá seguir os seguintes princípios:

* API pequena;
* comportamento previsível;
* ausência de dependências desnecessárias;
* implementação progressiva em Capi;
* suporte explícito a erros;
* integração mínima com o Runtime;
* possibilidade de testes independentes;
* ausência de abstrações prematuras;
* compatibilidade futura com a Biblioteca Padrão.

---

## 15.3 Relação com a Biblioteca Padrão

A Biblioteca Mínima será um subconjunto funcional da futura Biblioteca Padrão.

Suas APIs poderão evoluir durante o bootstrap.

Entretanto, deverão ser projetadas para permitir migração sem reconstrução completa dos programas internos.

A organização conceitual poderá seguir módulos como:

```text
core
collections
text
io
path
system
testing
```

Os nomes definitivos deverão seguir o Documento 08.

---

## 15.4 Camadas da Biblioteca Mínima

A implementação poderá ser organizada em três camadas.

### Camada intrínseca

Operações que exigem suporte direto do compilador ou backend.

### Camada de Runtime

Operações de baixo nível fornecidas pelo Runtime Rust.

### Camada Capi

Abstrações de alto nível implementadas na própria linguagem.

Conceitualmente:

```text
Biblioteca em Capi
        │
        ▼
Runtime mínimo
        │
        ▼
Sistema operacional / Backend
```

A evolução deverá transferir progressivamente funcionalidades para a camada Capi.

---

## 15.5 Núcleo `core`

O módulo central deverá possuir abstrações fundamentais.

Entre elas:

* tipos primitivos expostos;
* `Unit`;
* `Bool`;
* tipos numéricos;
* `Optional<T>`;
* `Result<T,E>`;
* operações fundamentais;
* traits essenciais;
* funções de abortamento controlado;
* suporte a comparação;
* suporte a hashing quando necessário;
* suporte a conversão textual mínima.

O núcleo deverá evitar dependência de IO.

---

## 15.6 `Optional<T>`

A Biblioteca Mínima deverá oferecer operações básicas como:

* criação;
* verificação de presença;
* extração segura;
* transformação;
* valor padrão;
* pattern matching.

Operações que abortem em ausência deverão ser claramente identificadas e evitadas no compilador sempre que o erro puder ser tratado.

---

## 15.7 `Result<T,E>`

A Biblioteca Mínima deverá oferecer:

* `Ok`;
* `Error`;
* verificação;
* transformação;
* propagação;
* composição;
* conversão de erro;
* extração controlada.

`Result` será a principal abstração de falha recuperável.

---

## 15.8 Traits Fundamentais

Conforme o suporte da linguagem, poderão existir traits equivalentes a:

* igualdade;
* ordenação;
* hashing;
* conversão textual;
* clone ou cópia controlada;
* iteração;
* destruição, se exposta;
* conversão de tipos.

A nomenclatura e a semântica deverão seguir a especificação da Biblioteca Padrão.

Somente traits necessárias ao bootstrap deverão ser incluídas inicialmente.

---

## 15.9 Tipo `String`

`String` é requisito central da Biblioteca Mínima.

Deverá suportar:

* criação a partir de literal;
* tamanho;
* verificação de vazio;
* comparação;
* concatenação controlada;
* acesso seguro;
* conversão para bytes;
* construção a partir de bytes válidos;
* integração com IO;
* destruição.

A codificação deverá seguir a decisão normativa da linguagem.

---

## 15.10 Imutabilidade de `String`

Recomenda-se que `String` represente texto imutável ou que possua semântica claramente controlada de mutação.

Operações de construção incremental poderão utilizar um tipo separado, como:

```text
StringBuilder
```

ou buffer equivalente.

Essa separação favorece:

* previsibilidade;
* compartilhamento;
* segurança;
* eficiência;
* APIs claras.

---

## 15.11 Slices e Visões Textuais

Caso a linguagem defina slices ou visões, a Biblioteca Mínima poderá oferecer uma representação textual não proprietária.

Essa representação deverá preservar:

* validade;
* limites;
* codificação;
* lifetime;
* associação ao armazenamento.

Se esse recurso aumentar excessivamente a complexidade inicial, poderá ser adiado e substituído temporariamente por operações que retornem `String`.

---

## 15.12 `StringBuilder`

O compilador precisará construir mensagens e representações textuais.

Um builder deverá suportar:

* criação;
* anexação de texto;
* anexação de caracteres;
* anexação de números;
* obtenção da `String`;
* crescimento;
* destruição.

A implementação inicial poderá utilizar um buffer de bytes interno.

---

## 15.13 Buffers de Bytes

Um tipo de buffer será necessário para:

* leitura de arquivos;
* escrita;
* codificação;
* geração de artefatos;
* processamento de fontes.

Deverá suportar:

* criação;
* capacidade;
* tamanho;
* anexação;
* acesso;
* slice ou cópia;
* conversão controlada para texto.

---

## 15.14 `Vector<T>`

`Vector<T>` será a primeira coleção genérica prioritária.

Deverá suportar:

* criação vazia;
* capacidade;
* tamanho;
* inserção ao final;
* remoção ao final;
* acesso por índice;
* acesso mutável controlado;
* iteração;
* crescimento;
* destruição dos elementos;
* movimentação;
* monomorfização.

O compilador dependerá extensivamente dessa estrutura.

---

## 15.15 Segurança de Índices

Acesso por índice deverá possuir comportamento definido.

Poderá retornar:

* valor;
* referência controlada;
* `Optional`;
* `Result`;

conforme a API oficial.

Acesso fora dos limites não poderá produzir comportamento indefinido silencioso.

Uma operação interna não verificada poderá existir apenas com contrato restrito e explícito.

---

## 15.16 Iteração

A Biblioteca Mínima deverá possuir mecanismo de iteração suficiente para:

* vetores;
* strings ou bytes;
* mapas;
* ranges;
* coleções internas do compilador.

A implementação inicial poderá ser simples.

A sintaxe de `for` poderá ser adicionada após a existência dos contratos de iteração.

---

## 15.17 `Map<K,V>`

Um mapa será necessário para:

* tabelas de símbolos;
* caches;
* módulos;
* tipos;
* especializações;
* diagnósticos;
* arquivos.

A implementação inicial deverá suportar:

* criação;
* inserção;
* consulta;
* atualização;
* remoção;
* verificação de chave;
* iteração;
* destruição;
* hashing ou ordenação.

A estratégia física poderá ser:

* hash map;
* árvore;
* vetor associativo temporário.

A API deverá permanecer independente da estratégia.

---

## 15.18 Estratégia Inicial de `Map`

Durante o primeiro bootstrap, poderá ser utilizada uma implementação simples em Rust exposta ao código Capi.

Posteriormente, deverá ser reimplementada em Capi.

A escolha entre hash map e árvore deverá considerar:

* complexidade de implementação;
* determinismo;
* desempenho;
* necessidade do compilador;
* disponibilidade de traits.

Se a determinismo for prioritário, uma estrutura ordenada poderá ser utilizada inicialmente em algumas fases.

---

## 15.19 `Set<T>`

Um conjunto poderá ser incluído quando necessário para:

* análise de fluxo;
* detecção de ciclos;
* dependências;
* conjuntos de símbolos;
* blocos visitados.

Ele poderá ser implementado sobre `Map<T, Unit>` inicialmente.

---

## 15.20 `Deque<T>` ou Fila

Uma fila ou deque poderá ser necessária para:

* worklists;
* análise de grafos;
* traversal;
* otimizações;
* diagnósticos.

Não deverá ser incluída antes de existir necessidade concreta.

Uma implementação interna simplificada poderá ser utilizada inicialmente.

---

## 15.21 Paths

O compilador precisará manipular caminhos de arquivo de forma estruturada.

A Biblioteca Mínima deverá oferecer:

* criação a partir de texto;
* composição;
* nome de arquivo;
* extensão;
* diretório pai;
* normalização controlada;
* conversão para representação do sistema;
* comparação.

Paths não deverão ser tratados apenas como strings em todas as APIs.

---

## 15.22 Portabilidade de Paths

A primeira implementação poderá ser direcionada a Linux.

Mesmo assim, a API deverá evitar incorporar detalhes desnecessários como:

* separador fixo espalhado pelo código;
* interpretação manual de `..`;
* concatenação textual direta;
* pressupostos sobre encoding.

O Runtime poderá fornecer a adaptação à plataforma.

---

## 15.23 IO de Arquivos

O subconjunto de IO deverá oferecer:

* leitura integral de arquivo;
* escrita integral de arquivo;
* abertura controlada;
* fechamento;
* erro explícito;
* metadados mínimos;
* verificação de existência;
* criação de diretório quando necessário.

Operações assíncronas estão fora do escopo inicial.

---

## 15.24 Leitura de Código-Fonte

A API de leitura deverá permitir:

```text
Path
→ Result<ByteBuffer, IoError>
→ validação textual
→ String ou SourceText
```

A codificação inválida deverá produzir erro explícito.

O Source Manager do compilador em Capi dependerá dessa capacidade.

---

## 15.25 Escrita de Artefatos

A Biblioteca Mínima deverá permitir escrever:

* dumps;
* arquivos temporários;
* objetos produzidos externamente;
* logs;
* resultados de testes;
* arquivos gerados.

A escrita deverá retornar `Result`.

---

## 15.26 Saída Padrão e Saída de Erro

Deverão existir operações para:

* escrever texto;
* escrever linha;
* escrever erro;
* flush quando necessário.

Essas operações poderão ser expostas por abstrações simples durante o bootstrap.

---

## 15.27 Argumentos de Linha de Comando

A Biblioteca Mínima deverá permitir obter os argumentos do processo.

Conceitualmente:

```text
arguments() -> Vector<String>
```

ou representação equivalente.

A memória fornecida pelo sistema operacional deverá ser convertida para tipos seguros da Capi.

---

## 15.28 Variáveis de Ambiente

O acesso a variáveis de ambiente poderá ser incluído quando necessário para:

* configuração do compilador;
* localização de toolchain;
* testes;
* Runtime.

A API deverá retornar ausência ou erro de forma explícita.

---

## 15.29 Erros de IO

Deverá existir um tipo inicial de erro de IO.

Poderá conter:

* categoria;
* código de sistema;
* mensagem;
* path relacionado;
* operação.

A representação deverá evitar dependência pública excessiva de códigos específicos do Linux.

---

## 15.30 Conversões Numéricas

O compilador precisará converter:

* inteiros para texto;
* texto para inteiros;
* tamanhos;
* offsets;
* índices.

A Biblioteca Mínima deverá oferecer operações explícitas, com `Result` quando houver possibilidade de falha.

---

## 15.31 Formatação Textual

Um sistema completo de formatação poderá ser adiado.

Inicialmente deverão existir operações suficientes para:

* concatenar;
* converter números;
* construir diagnósticos;
* produzir dumps;
* imprimir mensagens.

Macros ou interpolação avançada não serão requisitos do primeiro bootstrap, salvo decisão posterior.

---

## 15.32 Unicode

A Biblioteca Mínima deverá respeitar a política de Unicode definida pela linguagem.

Deverá distinguir claramente:

* bytes;
* unidades de codificação;
* caracteres;
* índices;
* comprimento em bytes;
* comprimento lógico.

O compilador não deverá assumir que um caractere ocupa um byte.

---

## 15.33 Ranges

Ranges serão úteis para:

* loops;
* spans;
* slices;
* iteração.

Uma representação inicial poderá incluir:

```text
Range<T>
├── início
└── fim
```

As regras de inclusão e exclusão deverão ser explícitas.

---

## 15.34 Spans na Biblioteca de Bootstrap

O compilador em Capi precisará de uma estrutura equivalente a:

```text
Span
├── FileId
├── início
└── fim
```

Essa estrutura pertencerá inicialmente ao próprio compilador, mas poderá utilizar tipos fundamentais da Biblioteca Mínima.

Não deverá ser confundida com slices de memória.

---

## 15.35 Arena para o Compilador

A reimplementação do compilador poderá necessitar de uma coleção de arena.

Ela deverá ser distinguida dos Domains da linguagem.

Uma arena de compilador poderá oferecer:

* inserção;
* identificador estável;
* acesso;
* iteração;
* destruição coletiva.

Sua implementação poderá utilizar um Domain internamente, mas sua semântica será de estrutura de dados, não de mecanismo global da linguagem.

---

## 15.36 Tipos de Identificador

A Biblioteca ou o compilador deverá permitir wrappers tipados sobre inteiros.

Exemplo conceitual:

```text
FileId
SymbolId
TypeId
NodeId
```

Esses wrappers deverão impedir mistura acidental de identificadores distintos.

A implementação poderá utilizar structs por valor com um campo inteiro.

---

## 15.37 Ordenação e Comparação

Estruturas internas exigirão:

* igualdade;
* ordenação;
* hashing.

As traits ou interfaces correspondentes deverão estar disponíveis.

A semântica deverá ser determinística.

---

## 15.38 Hashing

Caso `Map` utilize hashing, deverão ser definidos:

* contrato de hash;
* igualdade consistente;
* política de seed;
* comportamento determinístico;
* resistência a entradas adversariais, quando relevante.

Durante o bootstrap, poderá ser preferido um hash determinístico simples para testes e builds reproduzíveis.

Questões de segurança contra ataques de colisão poderão ser tratadas posteriormente conforme o ambiente de uso.

---

## 15.39 Ordenação Estável

O compilador precisará produzir saídas determinísticas.

A Biblioteca Mínima deverá fornecer ordenação estável ou mecanismos equivalentes quando necessário.

Isso será utilizado em:

* diagnósticos;
* dumps;
* símbolos;
* dependências;
* geração de artefatos.

---

## 15.40 Testes

A Biblioteca Mínima deverá possuir suporte básico a testes.

Inicialmente, poderá existir:

* função de asserção;
* igualdade;
* falha;
* agrupamento;
* código de retorno;
* descoberta por convenção ou Toolchain.

O sistema completo de `capi test` será desenvolvido conforme o Documento 11.

---

## 15.41 Assertions

Deverão existir operações para:

* afirmar condição;
* afirmar igualdade;
* afirmar erro;
* interromper teste;
* apresentar localização.

Assertions de desenvolvimento e assertions públicas poderão possuir comportamentos diferentes.

---

## 15.42 Panic e Abortamento

A Biblioteca Mínima poderá possuir mecanismo de abortamento para situações irrecuperáveis.

Esse mecanismo não deverá substituir `Result`.

Seu uso deverá ser restrito a:

* invariantes internos;
* falhas impossíveis após validação;
* corrupção;
* erros fatais sem recuperação definida.

A semântica deverá ser documentada.

---

## 15.43 Tempo e Sistema

Funcionalidades como relógio poderão ser incluídas apenas se necessárias para:

* benchmarks;
* testes;
* logs.

Não pertencem ao núcleo obrigatório do primeiro bootstrap.

---

## 15.44 Alocação

A Biblioteca Mínima não deverá expor diretamente o alocador do Runtime em APIs de alto nível, salvo necessidade específica.

Coleções deverão gerenciar sua memória através das abstrações oficiais da linguagem e dos Domains aplicáveis.

Operações de baixo nível deverão permanecer restritas.

---

## 15.45 Integração com Domains

Tipos da Biblioteca Mínima deverão declarar claramente:

* se são valores;
* se possuem identidade;
* em qual Domain armazenam recursos;
* como são destruídos;
* se podem ser movidos;
* se podem compartilhar armazenamento;
* quais operações exigem Capacidade de Mutação.

Essa classificação será especialmente importante para:

* `String`;
* `Vector`;
* `Map`;
* buffers;
* arquivos;
* builders.

---

## 15.46 Implementação Inicial em Rust

Durante as primeiras fases, alguns tipos poderão ser implementados em Rust e expostos à Capi por intrínsecos ou ABI interna.

Candidatos:

* `String`;
* buffer de bytes;
* IO;
* `Vector`;
* `Map`;
* paths;
* argumentos.

Essa estratégia deverá ser temporária.

Cada componente deverá possuir:

* contrato público definido;
* encapsulamento;
* testes;
* plano de migração;
* ausência de exposição de tipos Rust.

---

## 15.47 Migração para Capi

A ordem recomendada de migração será:

```text
1. Tipos auxiliares por valor
2. Optional e Result
3. Vector
4. StringBuilder
5. String
6. Buffers
7. Map e Set
8. Paths
9. IO de alto nível
10. Testes
```

Operações de baixo nível poderão continuar no Runtime.

---

## 15.48 Critérios para Migrar um Tipo

Um tipo poderá ser migrado de Rust para Capi quando:

* a linguagem suportar os recursos necessários;
* a implementação Capi puder preservar a API;
* os testes puderem ser compartilhados;
* o desempenho mínimo for aceitável;
* a interação com o Runtime estiver definida;
* a destruição estiver correta;
* a nova implementação não bloquear o bootstrap.

---

## 15.49 Limite entre Runtime e Biblioteca

Deverão permanecer no Runtime apenas operações como:

* chamadas de sistema;
* entrada do processo;
* encerramento;
* acesso ao ambiente;
* primitivas de alocação;
* suporte mínimo a Domains;
* ponte de FFI;
* operações impossíveis de expressar inicialmente.

Deverão migrar para a Biblioteca:

* algoritmos;
* coleções;
* builders;
* formatação;
* parsing;
* manipulação de paths de alto nível;
* iteração;
* composição de erros.

---

## 15.50 Intrínsecos da Biblioteca

Todo intrínseco utilizado pela Biblioteca deverá possuir:

* nome interno;
* assinatura;
* semântica;
* tipos válidos;
* efeitos;
* comportamento em erro;
* backend responsável;
* testes;
* plano de permanência ou remoção.

Intrínsecos não deverão ser tratados como funções comuns sem identificação explícita.

---

## 15.51 Estabilidade das APIs

As APIs da Biblioteca Mínima poderão evoluir durante o bootstrap.

Entretanto, alterações deverão ser coordenadas com:

* compilador;
* testes;
* exemplos;
* módulos já reimplementados;
* documentação.

A estabilidade pública definitiva será definida apenas antes da versão estável da linguagem.

---

## 15.52 Dependências Internas

A Biblioteca Mínima deverá evitar ciclos entre módulos.

Uma dependência conceitual possível será:

```text
core
├── sem dependência de IO
│
collections
├── depende de core
│
text
├── depende de core e collections mínimas
│
path
├── depende de text
│
io
├── depende de core, text e path
│
testing
└── depende de core, text e io
```

A estrutura concreta deverá ser validada durante a implementação.

---

## 15.53 Testes da Biblioteca Mínima

Cada tipo deverá possuir:

* testes unitários;
* testes de erro;
* testes de destruição;
* testes de limite;
* testes de integração;
* testes com generics;
* testes de Domain;
* testes de mutação;
* testes diferenciais contra a versão Rust, durante a migração.

---

## 15.54 Benchmarks

Tipos críticos deverão possuir benchmarks.

Prioridades:

* `String`;
* `Vector`;
* `Map`;
* leitura de arquivos;
* parsing numérico;
* concatenação;
* iteração.

Benchmarks serão utilizados para identificar regressões, não para bloquear prematuramente soluções simples.

---

## 15.55 Documentação

Cada tipo da Biblioteca Mínima deverá documentar:

* finalidade;
* categoria de memória;
* ownership;
* relação com Domain;
* mutabilidade;
* complexidade esperada;
* erros;
* invariantes;
* exemplos.

Essa documentação será necessária para os primeiros contribuidores e para o desenvolvimento do compilador auto-hospedado.

---

## 15.56 Critérios de Conclusão

A Biblioteca Mínima de Bootstrap será considerada concluída quando:

* todos os tipos necessários ao compilador estiverem disponíveis;
* strings e buffers forem confiáveis;
* vetores e mapas forem funcionais;
* IO de arquivos estiver disponível;
* paths estiverem disponíveis;
* argumentos puderem ser lidos;
* `Optional` e `Result` estiverem estáveis;
* erros puderem ser tratados;
* testes puderem ser escritos;
* as APIs estiverem documentadas;
* as implementações temporárias estiverem identificadas;
* pelo menos um módulo real do compilador puder utilizar exclusivamente essas abstrações.

---

## 15.57 Resultado Esperado

Ao final deste bloco de implementação, deverá ser possível escrever em Capi um programa estruturalmente semelhante a um componente de compilador, capaz de:

* receber argumentos;
* localizar arquivos;
* ler código-fonte;
* armazenar texto;
* produzir tokens ou estruturas;
* utilizar vetores e mapas;
* tratar falhas;
* emitir saída;
* executar testes;
* encerrar corretamente.

Esse resultado estabelecerá a base prática para a migração progressiva do compilador Stage 0 para a própria linguagem Capi.

---

# 16. Arquitetura do Runtime Inicial

## 16.1 Objetivo

O Runtime inicial da Capi será responsável por fornecer os mecanismos mínimos necessários para executar programas compilados, materializar as garantias de memória que não puderem ser inteiramente reduzidas a código estático e estabelecer a fronteira entre o código gerado e os serviços de baixo nível da plataforma.

Seu objetivo principal não será oferecer uma máquina virtual, um ambiente gerenciado amplo ou uma camada abstrata equivalente a uma plataforma de execução completa.

A Capi permanecerá uma linguagem compilada antecipadamente, com geração de código nativo e Runtime mínimo.

O Runtime deverá existir apenas para prover mecanismos que dependam de:

* inicialização e encerramento do processo;
* integração com o sistema operacional;
* criação e destruição de Domains;
* alocação de objetos por identidade;
* materialização de `ObjectId`;
* manutenção dos metadados mínimos dos objetos;
* finalização determinística;
* suporte ao modelo de despacho;
* tratamento de falhas fatais;
* interoperabilidade com funções externas;
* serviços transitórios necessários ao bootstrap.

Sempre que uma garantia puder ser comprovada e eliminada estaticamente pelo compilador, o Runtime não deverá reproduzir essa verificação sem necessidade.

---

## 16.2 Natureza do Runtime

O Runtime da Capi não será:

* uma máquina virtual;
* um interpretador;
* um coletor de lixo;
* um sistema de contagem automática de referências;
* um scheduler obrigatório;
* um gerenciador de threads;
* um mecanismo de reflexão completa;
* um container de componentes;
* uma plataforma intermediária semelhante a uma VM de bytecode.

O Runtime será uma biblioteca nativa vinculada ao programa compilado.

Conceitualmente:

```text
Programa Capi
      │
      ▼
Código nativo gerado
      │
      ▼
Runtime Capi
      │
      ▼
Sistema operacional e bibliotecas da plataforma
```

A presença do Runtime não deverá impedir a geração de executáveis autônomos.

---

## 16.3 Princípio do Runtime Mínimo

O Runtime deverá seguir o princípio:

> somente permanecerá no Runtime aquilo que não puder ser implementado de modo adequado, seguro ou estável na própria linguagem ou no código gerado.

Esse princípio busca:

* reduzir a superfície de execução;
* diminuir dependências;
* facilitar auditoria;
* preservar previsibilidade;
* simplificar distribuição;
* reduzir consumo de memória;
* favorecer portabilidade;
* permitir futura reimplementação parcial em Capi.

Toda nova funcionalidade proposta para o Runtime deverá justificar por que não pode permanecer:

* no compilador;
* na Biblioteca Padrão;
* em código Capi comum;
* em uma biblioteca externa;
* em uma camada específica de plataforma.

---

## 16.4 Relação com a Especificação

O Runtime implementará mecanismos necessários às garantias definidas principalmente nos documentos:

* 02 — Sistema de Tipos;
* 03 — Modelo de Memória;
* 05 — Semântica Operacional;
* 07 — Intermediate Representation;
* 08 — Biblioteca Padrão;
* 09 — Runtime Mínimo;
* 10 — ABI e Interoperabilidade;
* 26 — Bootstrap e Subconjunto Inicial.

O Documento 27 não redefine essas garantias.

Ele estabelece como a implementação oficial inicial pretende materializá-las.

Caso exista divergência entre uma decisão deste capítulo e uma garantia normativa anterior, prevalecerá a especificação da linguagem.

---

## 16.5 Runtime Inicial em Rust

A primeira versão do Runtime será implementada em Rust.

A escolha busca aproveitar:

* controle de memória;
* ausência de Garbage Collector;
* integração com o compilador Stage 0;
* interoperabilidade com Cranelift;
* acesso a APIs de baixo nível;
* suporte a FFI;
* sistema de tipos robusto;
* capacidade de encapsular código `unsafe`;
* facilidade de geração de bibliotecas estáticas.

A implementação em Rust não deverá expor tipos Rust na ABI pública da Capi.

A fronteira será definida apenas por tipos de representação estável.

---

## 16.6 Forma de Distribuição

Inicialmente, o Runtime poderá ser distribuído como:

* biblioteca estática;
* conjunto de arquivos objeto;
* biblioteca compartilhada experimental;
* fonte compilada juntamente com a Toolchain.

A forma prioritária será a vinculação estática.

Essa escolha reduz dependências externas e facilita a produção de executáveis autônomos.

Conceitualmente:

```text
program.o
runtime.a
bibliotecas auxiliares
        │
        ▼
linker
        │
        ▼
executável
```

A utilização futura de biblioteca compartilhada poderá ser avaliada quando a ABI estiver estabilizada.

---

## 16.7 Componentes do Runtime

A organização conceitual inicial poderá ser:

```text
runtime/
├── entry/
├── process/
├── domain/
├── object/
├── metadata/
├── allocation/
├── finalization/
├── panic/
├── io/
├── ffi/
├── platform/
├── diagnostics/
└── testing/
```

Nem todos os componentes precisarão existir como crates separados.

A estrutura deverá preservar as fronteiras conceituais.

---

## 16.8 Componente `entry`

O componente `entry` será responsável por adaptar a entrada do sistema operacional à função principal da Capi.

Suas responsabilidades poderão incluir:

* receber argumentos do processo;
* inicializar estado global mínimo;
* criar o Domain raiz;
* inicializar serviços do Runtime;
* chamar a função `main` da Capi;
* converter o resultado de `main` em código de saída;
* executar finalização;
* retornar ao sistema operacional.

O código gerado não deverá replicar essa sequência em cada programa de forma descoordenada.

---

## 16.9 Ponto de Entrada da Plataforma

Em Linux x86-64, o executável poderá utilizar a convenção tradicional de entrada através de:

```text
main
```

fornecida pela biblioteca C da plataforma, ou um ponto de entrada mais baixo, caso futuramente seja necessário.

A estratégia inicial recomendada é utilizar a entrada convencional do sistema e do linker.

Isso reduz complexidade relacionada a:

* inicialização do processo;
* TLS;
* argumentos;
* ambiente;
* bibliotecas vinculadas;
* encerramento.

A remoção da dependência da biblioteca C não será objetivo do primeiro bootstrap.

---

## 16.10 Função de Entrada do Runtime

Conceitualmente, o Runtime poderá expor uma função equivalente a:

```text
capi_runtime_start(
    argc,
    argv,
    main_function
) -> exit_code
```

Essa função deverá:

1. validar os parâmetros essenciais;
2. inicializar o Runtime;
3. construir a representação segura dos argumentos;
4. criar o Domain raiz;
5. chamar `main`;
6. finalizar recursos;
7. destruir o Domain raiz;
8. retornar o código de saída.

A assinatura física exata será definida pela ABI interna.

---

## 16.11 Assinaturas da Função `main`

A implementação inicial poderá suportar progressivamente assinaturas como:

```capi
function main() {
}
```

```capi
function main() -> Int32 {
    return 0;
}
```

Posteriormente:

```capi
function main(args: Vector<String>) -> Int32 {
    return 0;
}
```

O Driver deverá validar a assinatura antes da geração de código.

A adaptação para a ABI de processo será responsabilidade do Runtime e do código de entrada gerado.

---

## 16.12 Inicialização do Runtime

A inicialização deverá ser explícita e determinística.

Ela poderá incluir:

* validação de compatibilidade de versão;
* configuração de metadados;
* inicialização de tabelas internas;
* configuração do tratador de falhas fatais;
* criação do Domain raiz;
* configuração de serviços de plataforma;
* registro de tipos estáticos;
* preparação de diagnósticos de Runtime.

Nenhuma thread auxiliar deverá ser criada implicitamente no primeiro estágio.

---

## 16.13 Estado Global

O Runtime deverá minimizar o uso de estado global mutável.

Quando inevitável, esse estado deverá:

* permanecer encapsulado;
* possuir inicialização única;
* possuir ordem de destruição definida;
* evitar dependência de inicialização estática não controlada;
* possuir acesso sincronizado quando concorrência for introduzida;
* permanecer pequeno.

Exemplos possíveis:

* versão do Runtime;
* registrador global de metadados estáticos;
* configuração de falha fatal;
* estado de diagnóstico.

Domains comuns não deverão ser armazenados em uma lista global apenas por conveniência.

---

## 16.14 Contexto de Execução

O Runtime poderá utilizar uma estrutura explícita de contexto.

Conceitualmente:

```text
RuntimeContext
├── configuração
├── Domain raiz
├── registrador de tipos
├── adaptador de plataforma
├── estado de erro fatal
└── serviços auxiliares
```

O contexto poderá ser passado explicitamente às funções internas.

Isso reduz dependência de variáveis globais e favorece:

* testes;
* isolamento;
* múltiplas instâncias futuras;
* embutimento;
* execução controlada.

A existência física desse contexto não constitui requisito da linguagem.

---

## 16.15 Domain Raiz

A inicialização deverá criar o Domain raiz do programa.

O Domain raiz será responsável por:

* armazenar objetos cujo lifetime se estenda até o encerramento do processo;
* servir como ancestral de Domains criados pelo programa, quando aplicável;
* fornecer contexto inicial de alocação;
* manter metadados necessários;
* participar da finalização global.

O Domain raiz não deverá ser confundido com um heap global irrestrito.

As regras da linguagem continuam válidas.

---

## 16.16 Encerramento Normal

O encerramento normal deverá seguir uma ordem controlada.

Conceitualmente:

```text
retorno de main
        │
        ▼
finalização de valores locais restantes
        │
        ▼
finalização dos serviços da biblioteca
        │
        ▼
destruição do Domain raiz
        │
        ▼
encerramento do Runtime
        │
        ▼
retorno ao sistema operacional
```

A ordem exata deverá respeitar a Semântica Operacional.

---

## 16.17 Encerramento Anormal

O Runtime deverá distinguir:

* erro recuperável;
* falha fatal;
* erro interno do Runtime;
* abortamento solicitado;
* sinal externo;
* corrupção de invariantes.

Erros recuperáveis deverão ser representados por `Result` ou mecanismo equivalente.

Falhas fatais poderão encerrar o processo.

O Runtime não deverá tentar continuar após violação comprovada de invariantes fundamentais.

---

## 16.18 Componente `process`

O componente `process` poderá fornecer:

* argumentos de linha de comando;
* variáveis de ambiente;
* código de saída;
* diretório de trabalho;
* identificadores básicos do processo;
* encerramento controlado.

Essas operações de baixo nível poderão ser expostas à Biblioteca Padrão através de APIs seguras.

---

## 16.19 Argumentos do Processo

Os argumentos recebidos da plataforma deverão ser convertidos para uma representação segura.

A conversão deverá tratar:

* ponteiros nulos inválidos;
* quantidade de argumentos;
* codificação;
* memória pertencente ao processo;
* cópia ou empréstimo;
* falhas de conversão.

Inicialmente, o Runtime poderá copiar os argumentos para strings pertencentes ao Domain raiz.

Essa abordagem simplifica lifetime e segurança.

---

## 16.20 Variáveis de Ambiente

O Runtime poderá oferecer primitivas para:

* consultar variável;
* listar variáveis;
* definir variável, quando suportado;
* remover variável;
* tratar ausência;
* tratar codificação inválida.

As APIs públicas deverão estar na Biblioteca Padrão.

O Runtime deverá apenas materializar o acesso à plataforma.

---

## 16.21 Componente `platform`

Todo código específico de sistema operacional ou arquitetura deverá permanecer isolado.

Conceitualmente:

```text
platform/
├── linux/
│   ├── process
│   ├── memory
│   ├── io
│   └── ffi
├── unix/
│   └── common
└── common/
```

Durante o primeiro estágio, apenas Linux x86-64 será implementado.

O restante da arquitetura não deverá depender diretamente de funções Linux.

---

## 16.22 Interface de Plataforma

A camada de plataforma deverá oferecer contratos internos para:

* alocação bruta;
* desalocação;
* páginas de memória, quando necessárias;
* leitura e escrita;
* argumentos;
* ambiente;
* encerramento;
* relógio, quando necessário;
* carregamento de bibliotecas, futuramente;
* mensagens de erro do sistema.

Esses contratos deverão retornar erros estruturados.

---

## 16.23 Dependência da Biblioteca C

A primeira implementação poderá utilizar funções da biblioteca C para:

* alocação;
* IO mínimo;
* strings de sistema;
* processo;
* encerramento.

Essa dependência deverá ser encapsulada.

A Biblioteca Padrão da Capi não deverá expor diretamente APIs C.

A remoção ou redução da dependência poderá ser avaliada posteriormente.

---

## 16.24 Componente `allocation`

O componente de alocação será responsável por fornecer memória bruta para:

* blocos dos Domains;
* metadados;
* tabelas;
* estruturas internas do Runtime;
* buffers temporários.

Ele não será responsável sozinho por implementar o modelo de memória da linguagem.

A distinção será:

```text
Allocator
    fornece memória bruta

Domain
    organiza lifetime, identidade e destruição
```

---

## 16.25 Alocador Inicial

A implementação inicial poderá utilizar o alocador global de Rust ou o alocador da plataforma.

Essa escolha deverá permanecer escondida atrás de uma interface interna.

O objetivo inicial será:

* correção;
* alinhamento;
* tratamento de falha;
* integração simples;
* instrumentação.

A implementação de um alocador próprio não será requisito do primeiro estágio.

---

## 16.26 Interface Conceitual do Alocador

Conceitualmente:

```text
Allocator
├── allocate(size, alignment)
├── deallocate(pointer, size, alignment)
├── reallocate(pointer, old_size, new_size, alignment)
└── allocate_zeroed(size, alignment)
```

A API física deverá impedir o uso inseguro fora de módulos autorizados.

A maior parte do Runtime deverá operar sobre abstrações seguras.

---

## 16.27 Falha de Alocação

A política de falha de alocação deverá ser explícita.

Possibilidades:

* retorno de erro;
* falha fatal;
* abortamento imediato;
* mecanismo configurável.

A primeira implementação poderá tratar falha de alocação como erro fatal, especialmente em caminhos internos.

APIs públicas específicas poderão oferecer falha recuperável no futuro.

---

## 16.28 Blocos de Domain

O alocador deverá fornecer blocos ao componente de Domains.

Um Domain poderá solicitar blocos com:

* tamanho;
* alinhamento;
* política de crescimento;
* finalidade.

O Domain organizará os objetos dentro desses blocos.

Detalhes dessa estratégia serão tratados no Capítulo 17.

---

## 16.29 Alocação de Metadados

Metadados internos poderão ser alocados separadamente dos objetos.

Essa separação poderá facilitar:

* estabilidade de endereço;
* compactação futura;
* inspeção;
* proteção;
* atualização independente.

Entretanto, a primeira implementação deverá evitar estruturas excessivamente fragmentadas.

A escolha concreta será guiada por simplicidade e testes.

---

## 16.30 Componente `metadata`

O Runtime deverá possuir metadados mínimos para os tipos que exigem suporte dinâmico.

Esses metadados poderão incluir:

```text
TypeMetadata
├── TypeId
├── tamanho
├── alinhamento
├── destrutor
├── vtable
├── informações de base
├── tabelas de interfaces
└── flags
```

Nem todos os campos serão necessários para todos os tipos.

---

## 16.31 Metadados Estáticos

Metadados de tipos conhecidos em compilação poderão ser emitidos como dados estáticos no executável.

O backend poderá gerar estruturas de dados constantes contendo:

* identificador;
* tamanho;
* alinhamento;
* ponteiros para funções;
* relacionamentos.

O Runtime receberá referências a essas estruturas.

Essa estratégia reduz inicialização dinâmica.

---

## 16.32 Registro de Tipos

O Runtime poderá possuir um registrador opcional de tipos.

Ele será necessário apenas para operações como:

* validação de cast;
* inspeção interna;
* finalização;
* interfaces;
* diagnósticos;
* testes.

O registrador não deverá se transformar em reflection pública completa.

---

## 16.33 Identificação de Tipos

Cada tipo relevante poderá possuir um `TypeId`.

O `TypeId` deverá ser:

* estável durante a execução;
* único no artefato;
* determinístico quando possível;
* não dependente de endereço;
* compatível com linkedição.

A estabilidade entre compilações não deverá ser prometida antes da definição formal da ABI.

---

## 16.34 Metadados e Generics

Tipos monomorfizados poderão possuir metadados próprios.

Exemplo:

```text
Vector<Int32>
Vector<String>
```

poderão gerar:

```text
TypeMetadata<Vector<Int32>>
TypeMetadata<Vector<String>>
```

O Runtime não precisará interpretar parâmetros genéricos abstratos quando a monomorfização produzir tipos concretos.

---

## 16.35 Metadados de Destruição

Tipos não triviais deverão possuir uma rotina de destruição associada.

Conceitualmente:

```text
destroy(pointer, context)
```

A rotina poderá:

* destruir campos;
* liberar recursos;
* executar finalizador;
* atualizar estado;
* registrar diagnóstico interno.

Para objetos em Domain, a rotina não deverá desalocar individualmente o bloco principal, salvo estruturas auxiliares externas.

---

## 16.36 Componente `object`

O componente `object` será responsável pelos mecanismos comuns a objetos por identidade.

Entre eles:

* cabeçalho interno, se utilizado;
* associação ao Domain;
* associação a metadados;
* acesso à identidade;
* resolução de `ObjectId`;
* integração com despacho;
* estado de finalização;
* validação interna.

A representação detalhada será definida nos Capítulos 17 e 18.

---

## 16.37 Cabeçalho de Objeto

A primeira implementação poderá utilizar um cabeçalho interno.

Conceitualmente:

```text
ObjectHeader
├── TypeMetadata*
├── ObjectId interno ou slot
├── estado
└── flags
```

Alternativamente, parte desses dados poderá permanecer na tabela do Domain.

A escolha deverá considerar:

* tamanho por objeto;
* velocidade de acesso;
* movimentação;
* estabilidade;
* capacidade de inspeção.

A linguagem não observará o cabeçalho diretamente.

---

## 16.38 Cabeçalho Inline ou Externo

Duas estratégias principais poderão ser avaliadas.

### Cabeçalho inline

```text
[header][campos do objeto]
```

Vantagens:

* acesso direto;
* simplicidade;
* metadados próximos.

Desvantagens:

* overhead por objeto;
* impacto no layout.

### Metadados externos

```text
tabela do Domain
    │
    └── aponta para dados do objeto
```

Vantagens:

* layout mais limpo;
* flexibilidade de movimentação.

Desvantagens:

* acesso indireto adicional;
* estrutura externa mais complexa.

A implementação inicial poderá adotar uma estratégia híbrida.

---

## 16.39 Estado do Objeto

O Runtime poderá registrar estados como:

```text
Allocated
Initializing
Alive
Finalizing
Destroyed
```

Esses estados poderão ser utilizados em:

* assertions internas;
* testes;
* builds de desenvolvimento;
* diagnóstico de corrupção.

Em builds otimizados, alguns estados poderão ser reduzidos ou eliminados.

---

## 16.40 Inicialização de Objetos

A criação de um objeto deverá seguir etapas controladas.

Conceitualmente:

```text
reservar slot
        │
        ▼
reservar memória
        │
        ▼
marcar como inicializando
        │
        ▼
inicializar campos
        │
        ▼
associar metadados
        │
        ▼
marcar como vivo
        │
        ▼
publicar ObjectId
```

Um objeto parcialmente inicializado não deverá ser observável fora do contexto autorizado.

---

## 16.41 Falha Durante a Inicialização

Caso a inicialização falhe:

* campos já inicializados deverão ser destruídos;
* o slot deverá ser invalidado;
* a geração deverá ser atualizada conforme necessário;
* a memória deverá ser recuperada ou marcada;
* o objeto não deverá ser publicado;
* o erro deverá ser propagado.

O Runtime e o código gerado deverão cooperar para preservar esse comportamento.

---

## 16.42 Componente `domain`

O componente `domain` será responsável por:

* criar Domains;
* manter sua identidade;
* controlar blocos;
* registrar objetos;
* gerenciar slots;
* controlar gerações;
* iniciar finalização;
* destruir objetos;
* liberar memória;
* invalidar acessos;
* preservar relação entre Domains.

Esse componente será o núcleo do Modelo de Memória em tempo de execução.

---

## 16.43 Responsabilidades Estáticas e Dinâmicas dos Domains

A implementação deverá separar:

### Responsabilidades estáticas do compilador

* validar lifetime;
* impedir escapes inválidos;
* validar Capacidade de Mutação;
* verificar acesso;
* inserir operações corretas;
* determinar destruição.

### Responsabilidades dinâmicas do Runtime

* fornecer armazenamento;
* manter slots;
* materializar identidade;
* executar destruição;
* invalidar identificadores;
* verificar invariantes residuais;
* liberar blocos.

O Runtime não deverá substituir a análise estática.

---

## 16.44 Componente `finalization`

A finalização deverá possuir componente próprio.

Ele será responsável por:

* ordenar destruição;
* invocar destrutores;
* tratar objetos com finalização;
* impedir reentrada inválida;
* registrar falhas;
* finalizar Domains;
* tratar destruição parcial.

A finalização não deverá ser dispersa em múltiplos módulos sem coordenação.

---

## 16.45 Destrutores e Finalizadores

A implementação deverá distinguir, quando aplicável:

* destruição estrutural;
* finalizador definido pelo usuário;
* liberação de recursos externos;
* liberação de armazenamento do Domain.

Conceitualmente:

```text
finalizador do usuário
        │
        ▼
destruição dos campos
        │
        ▼
marcação como destruído
        │
        ▼
liberação coletiva do armazenamento
```

A ordem definitiva deverá seguir a especificação.

---

## 16.46 Finalização Determinística

A destruição de Domains será determinística.

Isso significa que, ao término do lifetime:

* o momento de início da finalização será conhecido;
* os objetos serão processados conforme regras definidas;
* recursos serão encerrados;
* os identificadores serão invalidados;
* a memória será liberada.

Não haverá espera por ciclos de Garbage Collector.

---

## 16.47 Falhas Durante Finalização

Falhas durante finalização exigem política explícita.

A primeira implementação poderá considerar como falha fatal:

* panic em destrutor;
* violação de invariantes;
* reentrada inválida;
* corrupção de metadados;
* acesso a objeto destruído detectado internamente.

A linguagem deverá evitar finalizadores que dependam de propagação comum de erros.

Recursos recuperáveis deverão ser encerrados explicitamente antes da destruição, quando necessário.

---

## 16.48 Ordem de Finalização

A implementação deverá seguir a ordem definida pela Semântica Operacional.

Quando a especificação permitir liberdade, o Runtime deverá adotar uma ordem determinística.

Possibilidades:

* ordem inversa de criação;
* ordem topológica controlada;
* ordem de registro;
* grupos por tipo.

A escolha concreta não deverá ser presumida por programas, salvo garantia normativa.

---

## 16.49 Reentrada Durante Finalização

O Runtime deverá impedir ou controlar situações em que:

* um finalizador tente acessar objeto já destruído;
* um objeto seja finalizado duas vezes;
* um Domain em finalização receba nova alocação;
* um finalizador recrie referência inválida;
* um Domain destrua a si mesmo recursivamente.

Essas situações deverão produzir falha interna ou diagnóstico adequado.

---

## 16.50 Componente `panic`

O Runtime deverá possuir um mecanismo de falha fatal.

Esse mecanismo poderá ser utilizado para:

* violações internas;
* abortamento explícito;
* falha de alocação;
* corrupção de Runtime;
* operações intrínsecas impossíveis;
* assertions de Runtime.

A API pública da linguagem deverá distinguir esse mecanismo de `Result`.

---

## 16.51 Representação de Falha Fatal

Conceitualmente, uma falha fatal poderá conter:

```text
RuntimeFailure
├── código
├── mensagem
├── localização opcional
├── tipo de falha
├── thread ou contexto
└── informações internas
```

A primeira implementação poderá produzir saída textual simples.

Posteriormente poderão ser adicionados:

* stack trace;
* símbolos;
* dumps;
* integração com debugger;
* formato estruturado.

---

## 16.52 Falha Fatal e Segurança

O tratador de falhas deverá evitar:

* alocação excessiva;
* uso de estruturas já corrompidas;
* recursão;
* execução de lógica complexa;
* continuação insegura.

Em situações críticas, poderá escrever diretamente na saída de erro e abortar.

---

## 16.53 Assertions de Runtime

O Runtime poderá possuir assertions internas em builds de desenvolvimento.

Exemplos:

* Domain válido;
* slot dentro dos limites;
* geração correspondente;
* objeto vivo;
* alinhamento correto;
* tipo esperado;
* destruição única.

Essas assertions deverão ajudar a identificar defeitos do compilador ou Runtime.

Não deverão substituir garantias públicas.

---

## 16.54 Componente `diagnostics`

O Runtime poderá emitir diagnósticos próprios para falhas detectadas durante execução.

Eles deverão ser diferenciados de:

* diagnósticos de compilação;
* erros de aplicação;
* erros de IO;
* falhas fatais.

Um diagnóstico de Runtime deverá indicar, quando disponível:

* categoria;
* operação;
* tipo envolvido;
* Domain;
* `ObjectId`;
* local de origem;
* versão do Runtime.

---

## 16.55 Localização no Código-Fonte

O compilador poderá emitir metadados opcionais de localização para operações suscetíveis a falha em desenvolvimento.

Exemplos:

* acesso a índice;
* resolução de identidade;
* cast;
* falha explícita;
* assert.

O Runtime poderá utilizar essas informações para produzir mensagens mais úteis.

Em builds de release, parte desses dados poderá ser removida.

---

## 16.56 Componente `io`

O Runtime deverá fornecer primitivas mínimas de IO.

Inicialmente:

* escrita em saída padrão;
* escrita em saída de erro;
* leitura integral de arquivo;
* escrita integral de arquivo;
* abertura e fechamento básicos;
* consulta de erro da plataforma.

APIs de alto nível deverão permanecer na Biblioteca Padrão.

---

## 16.57 Separação entre Runtime e Biblioteca de IO

O Runtime deverá expor primitivas como:

```text
runtime_read_file
runtime_write_file
runtime_stdout_write
runtime_stderr_write
```

A Biblioteca Padrão deverá oferecer abstrações como:

```text
File
Reader
Writer
Path
IoError
```

Essa separação permite reimplementar a camada de alto nível em Capi.

---

## 16.58 Handles de Recursos Externos

Recursos do sistema operacional, como arquivos, poderão ser representados por handles opacos.

O Runtime deverá:

* criar;
* validar;
* utilizar;
* fechar;
* impedir fechamento duplicado;
* retornar erros.

A Biblioteca Padrão associará esses handles a tipos seguros.

---

## 16.59 Integração de Recursos com Domains

Recursos externos não devem depender exclusivamente da liberação de memória do Domain.

Um objeto que representa arquivo, socket ou handle deverá:

* possuir fechamento explícito quando necessário;
* possuir destrutor de segurança;
* impedir uso após fechamento;
* integrar-se à finalização.

O Domain garante destruição determinística, mas a API deverá incentivar encerramento explícito quando erros precisarem ser tratados.

---

## 16.60 Componente `ffi`

O Runtime deverá fornecer a ponte inicial entre o código gerado e funções externas.

Inicialmente, a FFI interna será utilizada para:

* chamadas ao próprio Runtime;
* funções da plataforma;
* biblioteca C;
* linker;
* utilidades essenciais.

A FFI pública da linguagem será construída sobre contratos mais amplos definidos no Documento 10.

---

## 16.61 ABI Interna do Runtime

Todas as funções chamadas pelo código gerado deverão possuir ABI documentada.

Cada função deverá definir:

* nome do símbolo;
* convenção de chamada;
* tipos dos parâmetros;
* representação dos retornos;
* ownership;
* lifetime;
* possibilidade de falha;
* efeitos;
* requisitos de alinhamento;
* compatibilidade de versão.

---

## 16.62 Convenção de Nomes dos Símbolos

Os símbolos internos do Runtime deverão utilizar prefixo reservado.

Exemplo conceitual:

```text
capi_rt_init
capi_rt_shutdown
capi_rt_domain_create
capi_rt_domain_destroy
capi_rt_object_allocate
capi_rt_object_resolve
capi_rt_panic
```

A nomenclatura definitiva deverá evitar colisões com símbolos do programa.

---

## 16.63 Versionamento da ABI Interna

Durante o bootstrap, a ABI poderá evoluir.

Ainda assim, o Runtime e o compilador deverão possuir mecanismo de compatibilidade.

Possibilidades:

* número de versão embutido;
* símbolo de versão;
* hash de ABI;
* verificação durante linkedição;
* verificação durante inicialização.

O objetivo é impedir que um programa seja executado com Runtime incompatível.

---

## 16.64 Tipos na Fronteira da ABI

A ABI deverá utilizar apenas representações bem definidas.

Exemplos:

* inteiros de largura fixa;
* ponteiros opacos;
* structs `repr(C)` equivalentes;
* slices descritos por ponteiro e tamanho;
* códigos de erro;
* handles.

Não deverão atravessar a fronteira:

* `String` de Rust;
* `Vec` de Rust;
* referências Rust;
* enums Rust sem representação fixada;
* objetos genéricos de Rust;
* traits objects de Rust.

---

## 16.65 Ponteiros Opaques

Estruturas internas do Runtime poderão ser expostas como ponteiros opacos.

Exemplo conceitual:

```text
CapiRuntimeContext*
CapiDomain*
CapiTypeMetadata*
```

O código gerado não deverá acessar seus campos diretamente, salvo estruturas explicitamente compartilhadas pela ABI.

---

## 16.66 Retorno de Erros

Funções de Runtime que possam falhar de modo recuperável deverão utilizar uma convenção explícita.

Possibilidades:

```text
status + parâmetro de saída
```

ou:

```text
estrutura de resultado
```

Exemplo:

```text
RuntimeStatus capi_rt_read_file(
    PathView path,
    ByteBuffer* output
)
```

A escolha deverá facilitar integração com Cranelift e LLVM.

---

## 16.67 Erros Não Recuperáveis

Funções cujo contrato não admite falha recuperável poderão chamar o mecanismo de panic do Runtime.

Exemplos potenciais:

* corrupção de slot;
* violação de geração;
* alinhamento inválido;
* tipo inconsistente;
* double finalization;
* ABI incompatível.

---

## 16.68 Código `unsafe`

O Runtime será a principal região de uso de `unsafe`.

Esse uso deverá ser controlado por políticas rígidas.

Cada bloco deverá possuir:

* justificativa;
* invariantes;
* pré-condições;
* pós-condições;
* escopo mínimo;
* testes;
* encapsulamento.

Módulos de alto nível não deverão depender de `unsafe` diretamente.

---

## 16.69 Fronteiras Seguras Internas

Sempre que possível, o Runtime deverá transformar uma operação insegura em uma API interna segura.

Exemplo:

```text
unsafe allocate_raw(...)
        │
        ▼
safe DomainAllocator::allocate_object(...)
```

Somente a camada mais baixa deverá manipular ponteiros crus.

---

## 16.70 Validação de Ponteiros

Operações de FFI deverão validar, quando possível:

* nulo;
* alinhamento;
* tamanho;
* intervalo;
* lifetime;
* sobreposição;
* propriedade.

Nem toda propriedade poderá ser comprovada dinamicamente.

Os contratos deverão ser documentados.

---

## 16.71 Instrumentação

A primeira implementação deverá permitir instrumentação opcional do Runtime.

Possíveis informações:

* quantidade de Domains;
* quantidade de objetos;
* bytes alocados;
* bytes liberados;
* quantidade de slots;
* falhas de resolução;
* finalizadores executados;
* duração da finalização;
* operações de IO;
* chamadas ao Runtime.

Essa instrumentação auxiliará:

* testes;
* benchmarks;
* diagnóstico de vazamentos;
* validação do modelo de memória.

---

## 16.72 Modos do Runtime

O Runtime poderá possuir modos de build.

### Desenvolvimento

* assertions;
* validação de gerações;
* metadados ampliados;
* logs;
* preenchimento de memória;
* rastreamento.

### Teste

* determinismo;
* contadores;
* injeção de falhas;
* validações máximas.

### Release

* metadados mínimos;
* caminhos rápidos;
* menor logging;
* verificações elimináveis.

Todos os modos deverão preservar a mesma semântica pública.

---

## 16.73 Preenchimento de Memória

Em builds de desenvolvimento, o Runtime poderá preencher:

* memória nova;
* memória liberada;
* slots invalidados;

com padrões conhecidos.

Isso ajuda a detectar:

* uso antes de inicialização;
* uso após destruição;
* leitura de memória antiga;
* corrupção.

Esse mecanismo deverá ser desativável.

---

## 16.74 Injeção de Falhas

O modo de teste poderá permitir simular:

* falha de alocação;
* falha de IO;
* falha de crescimento;
* erro de sistema;
* falha de inicialização.

Essa capacidade será útil para validar caminhos de erro.

Não deverá existir em builds normais sem controle explícito.

---

## 16.75 Determinismo do Runtime

O Runtime deverá evitar fontes desnecessárias de não determinismo.

Deverão ser controlados:

* ordem de finalização;
* alocação de identificadores;
* geração de metadados;
* ordem de logs;
* seeds;
* traversal de tabelas.

Endereços físicos poderão variar, mas não deverão afetar resultados observáveis permitidos.

---

## 16.76 Concorrência

O primeiro Runtime poderá ser exclusivamente sequencial.

Isso significa que não deverá oferecer inicialmente:

* threads gerenciadas;
* sincronização de Domains entre threads;
* scheduler;
* tasks;
* execução paralela automática.

Entretanto, a arquitetura deverá evitar dependência desnecessária de thread única global.

A concorrência será introduzida somente após o modelo sequencial estar validado.

---

## 16.77 Thread Local Storage

O uso de TLS deverá ser evitado inicialmente.

Se necessário para:

* contexto atual;
* diagnóstico;
* panic;
* Domain atual;

deverá ser encapsulado e documentado.

A utilização de TLS não deverá substituir a passagem explícita de contexto quando esta for viável.

---

## 16.78 Runtime Embutível

A arquitetura deverá considerar, sem implementar integralmente no primeiro estágio, a possibilidade de embutir código Capi em outro processo.

Isso poderá exigir futuramente:

* inicialização explícita;
* múltiplos contextos;
* ausência de encerramento global;
* entrada sem `main`;
* controle externo do Domain raiz;
* tratamento de falhas sem abortar o host.

O primeiro Runtime poderá assumir processo próprio.

Essa suposição deverá permanecer localizada no componente de entrada.

---

## 16.79 Testes do Runtime

O Runtime deverá possuir testes em múltiplos níveis.

### Testes unitários

* alocador;
* slots;
* gerações;
* metadados;
* estados;
* erros.

### Testes de integração

* criação de Domain;
* alocação de objeto;
* destruição;
* finalização;
* IO;
* ABI.

### Testes funcionais

* executáveis Capi;
* saída;
* código de retorno;
* falha fatal;
* destruição observável.

### Testes de estresse

* muitos objetos;
* muitos Domains;
* grafos;
* ciclos;
* reutilização de slots;
* falhas de alocação.

---

## 16.80 Testes da ABI

Cada função da ABI interna deverá possuir testes que validem:

* assinatura;
* tamanhos;
* alinhamento;
* calling convention;
* retorno;
* erro;
* interoperabilidade com código gerado.

Esses testes deverão ser executados para os backends oficiais.

---

## 16.81 Testes Diferenciais entre Backends

Quando LLVM estiver disponível, o Runtime deverá ser testado com executáveis produzidos por:

* Cranelift;
* LLVM.

O comportamento deverá ser equivalente.

Diferenças de layout permitidas não poderão alterar a ABI compartilhada.

---

## 16.82 Testes com Sanitizers

A implementação Rust e os artefatos nativos deverão ser testados, quando possível, com:

* AddressSanitizer;
* UndefinedBehaviorSanitizer;
* LeakSanitizer;
* MemorySanitizer, quando viável;
* ferramentas equivalentes.

Esses testes não substituem a validação semântica.

---

## 16.83 Miri e Ferramentas Rust

Componentes Rust adequados poderão ser testados com Miri.

O uso será especialmente útil para:

* aliasing;
* ponteiros;
* inicialização;
* `unsafe`;
* layout.

Limitações da ferramenta deverão ser consideradas.

---

## 16.84 Fuzzing

O Runtime poderá ser submetido a fuzzing em operações como:

* resolução de `ObjectId`;
* tabelas de slots;
* metadados;
* parsing de dados de ABI;
* combinações de criação e destruição;
* sequências inválidas de API interna.

A API pública gerada pelo compilador deverá impedir sequências impossíveis.

O fuzzing auxiliará a validar a defesa interna.

---

## 16.85 Benchmarks do Runtime

Benchmarks prioritários:

* criação de Domain;
* destruição de Domain;
* alocação de objeto;
* resolução de `ObjectId`;
* chamada virtual;
* chamada por interface;
* finalização;
* crescimento de blocos;
* IO básico.

Esses benchmarks deverão comparar versões ao longo do tempo.

---

## 16.86 Métricas de Tamanho

O projeto deverá acompanhar:

* tamanho do Runtime estático;
* overhead mínimo por executável;
* tamanho de metadados;
* overhead por objeto;
* overhead por Domain;
* crescimento por tipo monomorfizado.

Essas métricas ajudarão a manter o Runtime mínimo.

---

## 16.87 Logs do Runtime

Os logs deverão ser opcionais e categorizados.

Categorias possíveis:

```text
runtime.init
runtime.domain
runtime.object
runtime.allocation
runtime.finalization
runtime.ffi
runtime.io
runtime.panic
```

A ativação deverá ocorrer por configuração de desenvolvimento ou variável controlada.

Logs não deverão alterar a semântica.

---

## 16.88 Integração com o Compilador

O compilador deverá conhecer apenas:

* símbolos da ABI;
* tipos da ABI;
* metadados que precisa emitir;
* operações de Runtime disponíveis;
* versão requerida.

Não deverá depender da estrutura interna do código Rust do Runtime.

---

## 16.89 Arquivo de Descrição da ABI

A implementação deverá manter uma descrição central da ABI interna.

Ela poderá ser expressa em:

* documento;
* módulo Rust compartilhado;
* arquivo declarativo;
* geração de bindings;
* combinação desses mecanismos.

O objetivo será evitar duplicação manual entre:

* Runtime;
* backend Cranelift;
* backend LLVM;
* testes;
* ferramentas.

---

## 16.90 Geração de Bindings Internos

Quando viável, estruturas e assinaturas da ABI poderão ser geradas a partir de uma fonte comum.

Isso reduzirá divergências em:

* tamanhos;
* alinhamento;
* nomes;
* versões;
* enumerações;
* códigos de erro.

A geração não deverá obscurecer os contratos.

---

## 16.91 Compatibilidade com Linkedição

O Runtime deverá controlar:

* visibilidade de símbolos;
* símbolos públicos internos;
* símbolos privados;
* relocations;
* seção de metadados;
* dependências externas;
* ordem de linkedição.

Símbolos internos não necessários não deverão ser exportados.

---

## 16.92 Seções Especiais do Executável

Futuramente, o compilador poderá emitir seções específicas para:

* metadados de tipos;
* tabelas de finalização;
* informações de debug;
* versão do Runtime;
* registro de módulos.

A primeira implementação deverá evitar depender prematuramente de seções personalizadas.

Metadados poderão ser representados como dados estáticos comuns.

---

## 16.93 Inicialização de Módulos

Se módulos exigirem inicialização, o Runtime deverá coordenar:

* ordem;
* execução única;
* detecção de ciclos inválidos;
* tratamento de falha;
* finalização.

A primeira versão poderá restringir inicializadores globais para reduzir complexidade.

---

## 16.94 Objetos Globais

Objetos globais mutáveis deverão ser evitados no subconjunto inicial.

Quando forem introduzidos, deverão possuir:

* Domain definido;
* ordem de inicialização;
* ordem de destruição;
* controle de mutação;
* sincronização futura.

O Domain raiz poderá hospedar objetos globais permitidos.

---

## 16.95 Constantes Estáticas

Constantes puras poderão ser emitidas diretamente no executável.

Elas não deverão exigir registro no Runtime quando:

* não possuem identidade;
* não possuem destruição;
* não possuem estado mutável;
* não dependem de inicialização dinâmica.

---

## 16.96 Strings Literais

Strings literais poderão ser emitidas como dados estáticos.

A representação deverá definir:

* encoding;
* tamanho;
* nul termination, se necessária;
* ownership;
* conversão para `String`;
* lifetime.

A Biblioteca Padrão poderá construir uma `String` ou view a partir desses dados.

---

## 16.97 Runtime e Exceções

O Runtime inicial não dependerá de exceções do sistema ou de unwinding para o tratamento comum de erros.

A linguagem utilizará mecanismos explícitos.

Falhas fatais poderão:

* abortar;
* utilizar unwind interno controlado;
* utilizar mecanismo específico da plataforma.

A estratégia inicial recomendada é abortamento, por simplicidade e previsibilidade.

---

## 16.98 Unwinding

Unwinding completo exigiria:

* tabelas;
* integração com ABI;
* limpeza;
* compatibilidade com FFI;
* suporte dos backends;
* semântica definida.

Ele não será requisito inicial.

Retornos antecipados e propagação de erros deverão gerar destruição explícita no código.

---

## 16.99 Limpeza Gerada pelo Compilador

O compilador será responsável por inserir operações de limpeza para:

* valores locais;
* recursos;
* temporários;
* branches;
* retornos;
* propagação de erros.

O Runtime fornecerá apenas as primitivas necessárias.

A ausência de unwinding não elimina a destruição determinística.

---

## 16.100 Runtime e Otimizações

O Runtime deverá expor contratos que permitam otimizações.

Exemplos:

* funções puras identificadas;
* operações `noreturn`;
* destrutores conhecidos;
* resolução de objeto inlinável;
* metadados constantes;
* caminhos rápidos.

Essas propriedades deverão ser comunicadas aos backends sem comprometer a ABI.

---

## 16.101 Fast Paths e Slow Paths

Algumas operações poderão possuir:

* caminho rápido inline;
* caminho lento no Runtime.

Exemplo conceitual para resolução:

```text
validar slot e geração inline
        │
        ├── válido → acessar
        └── inválido → chamar Runtime
```

A primeira implementação poderá utilizar apenas o caminho no Runtime.

Otimizações deverão ser introduzidas após estabilização.

---

## 16.102 Runtime e Debug

O modo de debug poderá fornecer:

* nomes de tipos;
* identificação de Domains;
* estado de objetos;
* rastreamento de alocações;
* stack de criação;
* visualização de `ObjectId`;
* relatório de finalização;
* detecção de uso inválido.

Essas capacidades deverão ser opcionais.

---

## 16.103 Inspeção de Domains

Uma ferramenta de desenvolvimento poderá solicitar ao Runtime:

* listar Domains ativos;
* contar objetos;
* listar tipos;
* medir memória;
* mostrar relação pai-filho;
* mostrar estado de finalização.

Essa interface não fará parte da API pública estável inicialmente.

---

## 16.104 Relatórios de Encerramento

Em modo de teste, o Runtime poderá emitir relatório contendo:

```text
Domains criados
Domains destruídos
Objetos criados
Objetos destruídos
Bytes alocados
Bytes liberados
Handles abertos
Handles fechados
```

Diferenças poderão indicar vazamentos ou falhas de finalização.

---

## 16.105 Variação por Target

O Runtime poderá possuir diferenças entre targets em:

* alinhamento;
* páginas;
* chamadas de sistema;
* entrada;
* arquivos;
* encoding;
* ABI;
* símbolos.

Essas diferenças deverão permanecer na camada de plataforma.

Domains, identidade e semântica deverão permanecer equivalentes.

---

## 16.106 Portabilidade Futura

A expansão para novos targets deverá exigir principalmente:

* nova camada de plataforma;
* adaptação de ABI;
* backend correspondente;
* testes.

O núcleo do Runtime deverá permanecer reutilizável.

---

## 16.107 Relação com WebAssembly

Um target WebAssembly poderá exigir Runtime diferente em aspectos como:

* processo;
* arquivos;
* entrada;
* memória linear;
* host functions;
* carregamento.

A arquitetura atual deverá evitar assumir que todo target possui processos POSIX.

Entretanto, o primeiro desenvolvimento será otimizado para Linux.

---

## 16.108 Relação com Bibliotecas Dinâmicas

O suporte a bibliotecas dinâmicas poderá introduzir:

* metadados entre módulos;
* compatibilidade de ABI;
* registro de tipos;
* inicialização;
* descarregamento;
* identidade entre bibliotecas.

Esse suporte está fora do primeiro Runtime.

A implementação inicial poderá gerar executáveis e bibliotecas estáticas.

---

## 16.109 Relação com Plugins

Plugins carregados dinamicamente exigiriam decisões adicionais sobre:

* ABI estável;
* versão;
* metadados;
* tipos compartilhados;
* Domains;
* ownership;
* descarregamento seguro.

Não farão parte do bootstrap inicial.

---

## 16.110 Segurança do Runtime

O Runtime deverá ser tratado como componente de alta criticidade.

Uma falha poderá comprometer:

* segurança de memória;
* identidade;
* destruição;
* isolamento de Domains;
* integridade de recursos.

Por isso, deverão ser exigidos:

* revisão;
* testes;
* isolamento de `unsafe`;
* documentação;
* análise estática;
* fuzzing;
* validação de ABI.

---

## 16.111 Superfície de Ataque

A superfície deverá ser reduzida através de:

* poucas funções exportadas;
* validação de entradas;
* tipos opacos;
* ausência de parser complexo;
* ausência de serviços de rede;
* ausência de reflexão ampla;
* ausência de carregamento automático de plugins;
* Runtime mínimo.

---

## 16.112 Dados Não Confiáveis

Entradas vindas de:

* arquivos;
* ambiente;
* FFI;
* bibliotecas externas;
* sistema operacional;

deverão ser tratadas como não confiáveis.

O Runtime deverá validar tamanhos, códigos e ponteiros conforme possível.

---

## 16.113 Compatibilidade com Código FFI Inseguro

Código externo poderá violar invariantes.

A FFI pública deverá delimitar operações inseguras.

O Runtime não poderá garantir segurança quando código externo:

* mantém ponteiros inválidos;
* altera memória sem contrato;
* libera objetos;
* forja `ObjectId`;
* utiliza ABI incorreta.

Essas operações deverão exigir blocos ou APIs explicitamente inseguras na linguagem.

---

## 16.114 Selagem de `ObjectId`

A representação de `ObjectId` deverá dificultar sua falsificação.

Possibilidades:

* tipo opaco;
* campos privados;
* geração;
* identificador do Domain;
* validação;
* checksum interno futuro.

Programas Capi comuns não deverão construir identidades arbitrárias.

---

## 16.115 Integridade de Metadados

Metadados emitidos pelo compilador serão considerados confiáveis quando produzidos pela Toolchain oficial.

Ainda assim, builds de desenvolvimento poderão validar:

* tamanho;
* alinhamento;
* ponteiros;
* tabelas;
* relações de herança;
* funções de destruição.

Metadados vindos de FFI ou módulos externos exigirão política adicional.

---

## 16.116 Atualização do Runtime

A atualização deverá ser coordenada com o compilador.

Cada versão distribuída deverá declarar:

* versão;
* ABI suportada;
* target;
* recursos;
* modo de build;
* compatibilidade.

O compilador deverá selecionar o Runtime correspondente.

---

## 16.117 Runtime Instalado ou Embutido

A Toolchain poderá adotar:

### Runtime embutido

O compilador distribui os objetos ou bibliotecas necessários.

Vantagens:

* compatibilidade;
* simplicidade;
* reprodutibilidade.

### Runtime instalado

O sistema possui Runtime compartilhado.

Vantagens:

* redução de duplicação;
* atualizações centralizadas.

A estratégia inicial recomendada é Runtime embutido na Toolchain.

---

## 16.118 Localização pelo Driver

O Driver deverá localizar o Runtime através de caminhos controlados.

Possibilidades:

* diretório relativo ao compilador;
* sysroot;
* configuração explícita;
* variável interna;
* argumento avançado.

Não deverá depender do diretório de trabalho do usuário.

---

## 16.119 Sysroot

A Toolchain poderá possuir um sysroot contendo:

```text
sysroot/
├── runtime/
├── std/
├── targets/
├── libraries/
└── metadata/
```

O Runtime específico de cada target poderá ser selecionado a partir dessa estrutura.

---

## 16.120 Build do Runtime

O build deverá ser integrado ao workspace.

Deverá produzir:

* artefato estático;
* cabeçalhos ou descrições de ABI;
* metadados de versão;
* símbolos;
* arquivos para testes.

O processo deverá ser reproduzível.

---

## 16.121 Cross-compilation

No futuro, a Toolchain poderá compilar Runtime para targets diferentes do host.

Isso exigirá:

* toolchain de target;
* linker;
* biblioteca de plataforma;
* configuração;
* testes.

O primeiro estágio não exigirá cross-compilation.

---

## 16.122 Evolução para Runtime Parcialmente em Capi

Após a maturidade da linguagem, partes do Runtime poderão ser reimplementadas em Capi.

Candidatos:

* estruturas auxiliares;
* registradores;
* algoritmos;
* diagnósticos;
* wrappers de IO;
* containers;
* testes;
* ferramentas de inspeção.

Componentes de baixo nível poderão permanecer em Rust.

---

## 16.123 Critérios para Migração

Um componente do Runtime poderá migrar para Capi quando:

* a linguagem suportar os recursos necessários;
* a implementação não introduzir dependência circular de bootstrap;
* a ABI puder ser preservada;
* os testes puderem ser reutilizados;
* o desempenho for aceitável;
* a segurança estiver validada.

---

## 16.124 Núcleo Permanente de Baixo Nível

É aceitável que permaneçam fora da Capi:

* ponto de entrada;
* chamadas de sistema;
* alocação bruta;
* manipulação de ponteiros;
* adaptação de ABI;
* primitivas de FFI;
* operações específicas de plataforma.

Auto-hospedagem não significa ausência absoluta de código hospedeiro.

---

## 16.125 Dependência Circular de Bootstrap

A reimplementação do Runtime deverá evitar o ciclo:

```text
compilador Capi precisa do Runtime Capi
Runtime Capi precisa ser compilado pelo compilador Capi
```

Esse ciclo será controlado através de:

* Stage 0;
* artefatos preservados;
* ordem de build;
* subconjunto;
* componentes mínimos em Rust.

---

## 16.126 Documentação do Runtime

Cada módulo deverá documentar:

* responsabilidade;
* ABI;
* invariantes;
* uso de `unsafe`;
* estados;
* falhas;
* testes;
* dependências;
* relação com a especificação.

As decisões relevantes deverão possuir ADRs.

---

## 16.127 ADRs do Runtime

Deverão ser considerados ADRs para:

```text
ADR — Runtime estático inicial
ADR — Dependência inicial da libc
ADR — Estratégia de entrada do processo
ADR — Política de falha fatal
ADR — Ausência inicial de unwinding
ADR — Interface do alocador
ADR — Representação de metadados
ADR — Política de versionamento da ABI
ADR — Separação Runtime/Biblioteca Padrão
ADR — Estratégia de instrumentação
```

---

## 16.128 Critérios de Conclusão do Runtime Inicial

O Runtime inicial será considerado funcional quando:

* puder ser vinculado a um programa Capi;
* inicializar corretamente;
* criar o Domain raiz;
* chamar `main`;
* encerrar corretamente;
* produzir código de saída;
* oferecer falha fatal;
* fornecer alocação básica;
* oferecer ABI documentada;
* possuir testes;
* ser compatível com Cranelift;
* não depender de tipos Rust na fronteira.

---

## 16.129 Critérios de Conclusão do Runtime de Bootstrap

O Runtime de bootstrap estará suficientemente maduro quando também:

* suportar Domains explícitos;
* suportar objetos por identidade;
* materializar `ObjectId`;
* executar finalização determinística;
* suportar vtables;
* suportar interfaces necessárias;
* fornecer IO mínimo;
* integrar-se à Biblioteca Mínima;
* possuir instrumentação;
* passar em testes de estresse;
* suportar os recursos utilizados pelo compilador escrito em Capi.

---

## 16.130 Resultado Esperado

Ao final da implementação descrita neste capítulo, a Toolchain deverá possuir uma camada de Runtime pequena, auditável e claramente separada do compilador e da Biblioteca Padrão.

Essa camada deverá permitir:

* iniciar programas;
* criar o contexto de memória inicial;
* materializar mecanismos essenciais;
* integrar código gerado à plataforma;
* encerrar recursos deterministicamente;
* evoluir sem definir a linguagem por detalhes de Rust.

O Runtime deverá permanecer um mecanismo de suporte.

As garantias da Capi continuarão sendo definidas pela especificação e verificadas, sempre que possível, pelo compilador.

---

# 17. Implementação de Domains e `ObjectId`

## 17.1 Objetivo

Este capítulo define a estratégia inicial para materializar, no Runtime oficial, os conceitos de:

* Domain;
* alocação de objetos;
* identidade;
* `ObjectId`;
* slots;
* gerações;
* resolução de identidade;
* invalidação;
* finalização;
* destruição coletiva.

A implementação deverá preservar as garantias estabelecidas pelo Modelo de Memória da Capi sem transformar essas garantias em detalhes públicos de representação.

Os mecanismos descritos neste capítulo pertencem à implementação oficial inicial.

Eles poderão evoluir desde que preservem:

* identidade estável;
* acesso válido apenas durante o lifetime permitido;
* ausência de Garbage Collector;
* ausência de contagem automática de referências;
* destruição determinística;
* liberação de grafos cíclicos;
* proteção contra reutilização indevida de identidades;
* independência entre identidade lógica e endereço físico.

---

## 17.2 Natureza de um Domain

Um Domain será uma unidade explícita de:

* armazenamento;
* lifetime;
* identidade;
* registro de objetos;
* finalização;
* destruição coletiva.

Conceitualmente:

```text
Domain
├── identidade própria
├── relação hierárquica opcional
├── blocos de memória
├── tabela de slots
├── objetos registrados
├── estado
├── política de alocação
└── informações de finalização
```

Um Domain não deverá ser tratado apenas como:

* um alocador;
* uma arena genérica;
* um heap comum;
* um container;
* uma região lexical sem representação;
* uma lista de ponteiros.

Embora possa utilizar técnicas de arena, sua semântica é mais ampla.

---

## 17.3 Separação entre Garantia e Representação

A linguagem garante o comportamento de Domains.

A implementação inicial poderá utilizar:

* tabelas;
* índices;
* gerações;
* ponteiros;
* blocos;
* cabeçalhos;
* listas;
* bitsets;
* arenas.

Programas Capi não poderão depender de:

* tamanho de um Domain;
* formato dos slots;
* largura da geração;
* endereço do objeto;
* ordem física dos blocos;
* política de crescimento;
* posição do objeto na tabela;
* algoritmo de resolução.

Essa separação permitirá futuras otimizações.

---

## 17.4 Representação Conceitual Inicial

Uma representação inicial poderá seguir o modelo:

```text
Domain
├── DomainId
├── estado
├── parent
├── allocator
├── blocks
├── slots
├── free_slots
├── creation_order
├── finalization_state
└── statistics
```

Cada elemento deverá possuir responsabilidade definida.

---

## 17.5 `DomainId`

Cada Domain ativo deverá possuir uma identidade interna.

O `DomainId` será utilizado para:

* distinguir Domains;
* validar `ObjectId`;
* detectar associação incorreta;
* produzir diagnósticos;
* auxiliar inspeção;
* evitar resolução no Domain errado.

O `DomainId` não precisa ser diretamente observável por programas Capi.

---

## 17.6 Geração de `DomainId`

A primeira implementação poderá utilizar um contador monotônico global ou associado ao `RuntimeContext`.

Exemplo conceitual:

```text
next_domain_id += 1
```

O valor deverá:

* não ser reutilizado durante a execução, quando possível;
* possuir largura suficiente;
* evitar colisões;
* ser gerado de forma determinística dentro da execução;
* permanecer independente do endereço do Domain.

Em caso de esgotamento, o Runtime deverá falhar de forma controlada.

---

## 17.7 Reutilização de Identidade de Domain

A implementação inicial não deverá reutilizar `DomainId` durante a mesma execução.

Essa escolha reduz risco de:

* identidade obsoleta parecer válida;
* colisão de `ObjectId`;
* diagnósticos ambíguos;
* erros em ferramentas de inspeção.

Uma implementação futura poderá adotar gerações também para Domains, caso isso seja necessário.

---

## 17.8 Estados de um Domain

O Runtime deverá representar estados explícitos.

Conceitualmente:

```text
Created
Active
Finalizing
Destroyed
```

Poderão existir estados internos adicionais, como:

```text
Initializing
Failed
```

Transições válidas:

```text
Created
   │
   ▼
Active
   │
   ▼
Finalizing
   │
   ▼
Destroyed
```

Transições inversas não serão permitidas.

---

## 17.9 Estado `Created`

O estado `Created` representa um Domain cuja estrutura interna já foi reservada, mas que ainda não foi publicado ao programa.

Nesse estado poderão ocorrer:

* inicialização do alocador;
* criação da tabela de slots;
* associação ao pai;
* configuração de metadados;
* validação interna.

Nenhum objeto deverá ser alocado antes da transição adequada.

---

## 17.10 Estado `Active`

No estado `Active`, o Domain poderá:

* receber alocações;
* criar objetos;
* resolver identidades;
* executar operações válidas;
* criar Domains filhos, quando permitido;
* registrar recursos.

Esse será o estado normal de execução.

---

## 17.11 Estado `Finalizing`

No estado `Finalizing`, o Domain:

* não deverá aceitar novas alocações comuns;
* não deverá publicar novos objetos;
* deverá impedir criação de novos filhos;
* executará finalizadores;
* destruirá objetos;
* invalidará slots;
* encerrará recursos.

A tentativa de uso comum durante essa fase deverá ser rejeitada.

---

## 17.12 Estado `Destroyed`

Após a destruição:

* todos os objetos estarão inválidos;
* slots não poderão ser resolvidos;
* blocos terão sido liberados;
* filhos terão sido destruídos;
* o Domain não poderá retornar a estado ativo;
* seu ponteiro interno não poderá ser reutilizado pelo programa.

O Runtime poderá manter apenas dados mínimos de diagnóstico, se necessário.

---

## 17.13 Hierarquia de Domains

A implementação inicial poderá organizar Domains em hierarquia.

Conceitualmente:

```text
Domain raiz
├── Domain A
│   ├── Domain A1
│   └── Domain A2
└── Domain B
```

A hierarquia poderá auxiliar:

* lifetime;
* ordem de destruição;
* validação de dependências;
* agrupamento;
* inspeção.

A existência de hierarquia física deverá seguir a semântica definida no Documento 03.

---

## 17.14 Relação Pai-Filho

Quando houver relação pai-filho:

* o pai deverá viver pelo menos tanto quanto o filho;
* o filho deverá ser destruído antes do pai;
* o pai poderá registrar seus filhos;
* o filho poderá utilizar serviços herdados permitidos;
* a destruição do pai deverá incluir os filhos ativos.

A implementação não deverá permitir que um filho permaneça ativo após a destruição do pai.

---

## 17.15 Registro de Filhos

Um Domain poderá manter:

* vetor de filhos;
* lista intrusiva;
* handles;
* tabela de identificadores.

A primeira implementação poderá utilizar uma coleção simples.

A ordem deverá ser determinística.

---

## 17.16 Criação de Domain

A criação deverá seguir uma sequência semelhante a:

```text
validar contexto
        │
        ▼
reservar estrutura
        │
        ▼
gerar DomainId
        │
        ▼
inicializar alocador
        │
        ▼
inicializar slots
        │
        ▼
associar ao pai
        │
        ▼
marcar como Active
        │
        ▼
publicar handle
```

Falhas antes da publicação deverão liberar todos os recursos parciais.

---

## 17.17 Handle de Domain

O código gerado poderá manipular um handle opaco.

Conceitualmente:

```text
CapiDomain*
```

ou:

```text
DomainHandle
```

O handle deverá:

* referenciar a estrutura interna;
* não expor seus campos;
* ser validado em operações de Runtime;
* possuir lifetime controlado pelo compilador;
* não ser copiável arbitrariamente para contextos inválidos.

---

## 17.18 Domain Atual

Algumas operações poderão receber explicitamente o Domain de destino.

Exemplo conceitual:

```text
allocate_object(domain, type_metadata)
```

A implementação não deverá depender exclusivamente de um “Domain atual” global.

A passagem explícita favorece:

* clareza;
* testes;
* múltiplos Domains;
* futura concorrência;
* embutimento.

---

## 17.19 Escopo Léxico e Handle

Quando a sintaxe criar um Domain lexical, o compilador deverá:

* emitir a criação;
* manter o handle em variável interna;
* utilizar o handle em alocações;
* inserir destruição em todos os caminhos;
* impedir escape inválido;
* finalizar no ponto correto.

O Runtime não deverá inferir o escopo léxico.

---

## 17.20 Estratégia de Alocação

A primeira implementação deverá priorizar:

* correção;
* alinhamento;
* previsibilidade;
* facilidade de destruição;
* instrumentação.

Uma estratégia adequada será a alocação por blocos.

Conceitualmente:

```text
Domain
├── Block 0
├── Block 1
├── Block 2
└── ...
```

Cada bloco poderá armazenar múltiplos objetos.

---

## 17.21 Blocos de Memória

Um bloco poderá possuir:

```text
MemoryBlock
├── base
├── capacidade
├── posição utilizada
├── alinhamento máximo
├── próximo bloco
└── flags
```

A alocação comum poderá avançar um cursor.

Essa abordagem reduz chamadas ao alocador global.

---

## 17.22 Política Inicial de Crescimento

A primeira política poderá utilizar:

* tamanho mínimo;
* crescimento geométrico;
* bloco dedicado para objetos grandes.

Exemplo conceitual:

```text
primeiro bloco: 4 KiB
seguintes: crescimento até limite configurado
objetos grandes: bloco exclusivo
```

Os valores concretos deverão ser definidos por medição.

Não constituirão contrato público.

---

## 17.23 Alocação Linear

Dentro de um bloco, a alocação poderá seguir:

```text
offset alinhado
        │
        ▼
reservar tamanho
        │
        ▼
avançar cursor
```

Essa estratégia oferece:

* baixo custo;
* simplicidade;
* boa localidade;
* destruição coletiva eficiente.

Ela não permite, por padrão, reutilização individual imediata do espaço.

---

## 17.24 Desalocação Individual

Objetos comuns de Domain não deverão exigir desalocação individual de seu armazenamento principal.

Quando um objeto for logicamente destruído antes do Domain:

* seu slot poderá ser invalidado;
* seus recursos poderão ser finalizados;
* o espaço físico poderá permanecer reservado;
* uma implementação futura poderá reutilizá-lo.

A ausência de desalocação individual não deverá impedir destruição lógica.

---

## 17.25 Objetos Grandes

Objetos acima de determinado limiar poderão receber bloco dedicado.

Isso evita:

* desperdício no bloco comum;
* fragmentação interna excessiva;
* crescimento desproporcional.

O bloco dedicado permanecerá associado ao Domain.

---

## 17.26 Alinhamento

Toda alocação deverá respeitar o alinhamento definido pelo layout do tipo.

O Runtime deverá validar:

* alinhamento não nulo;
* potência de dois, quando exigida;
* overflow;
* capacidade;
* endereço resultante.

Falhas deverão ser tratadas como erro interno ou falha de alocação.

---

## 17.27 Overflow de Tamanho

Operações como:

```text
tamanho + alinhamento
capacidade * crescimento
índice * tamanho
```

deverão verificar overflow.

O Runtime não poderá confiar em wraparound silencioso.

---

## 17.28 Tabela de Slots

Cada Domain deverá possuir uma tabela de slots para representar identidades de objetos.

Conceitualmente:

```text
slots[slot_index] = Slot
```

Cada objeto publicado terá um slot associado.

---

## 17.29 Estrutura de Slot

Um slot inicial poderá conter:

```text
Slot
├── pointer
├── generation
├── type_metadata
├── state
└── flags
```

Campos opcionais:

* tamanho;
* localização de criação;
* ordem de criação;
* índice de finalização;
* debug name.

Em release, a estrutura poderá ser compactada.

---

## 17.30 Estado de Slot

Estados possíveis:

```text
Vacant
Reserved
Initializing
Alive
Finalizing
Destroyed
```

A implementação poderá simplificar essa enumeração.

Entretanto, deverá distinguir pelo menos:

* slot livre;
* objeto válido;
* objeto inválido.

---

## 17.31 Slot Livre

Um slot livre:

* não aponta para objeto vivo;
* possui geração atual;
* pode ser reutilizado;
* não deve resolver `ObjectId` antigo;
* poderá fazer parte de uma free list.

---

## 17.32 Slot Reservado

Um slot reservado foi separado para um novo objeto, mas ainda não foi publicado.

Nesse estado:

* a identidade ainda não deverá escapar;
* a memória poderá estar sendo inicializada;
* uma falha deverá devolver o slot ao estado livre;
* a geração deverá ser tratada corretamente.

---

## 17.33 Slot Vivo

Um slot vivo deverá possuir:

* ponteiro válido;
* geração correspondente;
* tipo correto;
* estado de objeto ativo;
* associação ao Domain.

Somente esse estado poderá ser resolvido como objeto normal.

---

## 17.34 Slot em Finalização

Durante a finalização:

* acessos comuns deverão ser bloqueados;
* o destrutor poderá receber acesso interno controlado;
* o slot não poderá ser reutilizado;
* sua identidade deverá estar em processo de invalidação.

---

## 17.35 Free List

Slots livres poderão ser organizados por:

* vetor;
* pilha;
* lista ligada por índice;
* bitmap.

A primeira implementação poderá utilizar uma pilha de índices.

Conceitualmente:

```text
free_slots.push(slot_index)
```

A reutilização deverá sempre alterar a geração.

---

## 17.36 Índice de Slot

O índice identifica a posição na tabela do Domain.

Ele deverá:

* ser compacto;
* possuir largura suficiente;
* ser validado;
* não ser usado sem o `DomainId`;
* não ser exposto isoladamente.

Um índice válido em um Domain poderá referir-se a objeto diferente em outro.

---

## 17.37 Geração

A geração protege contra reutilização de slot.

Exemplo:

```text
slot 42, geração 7
```

Após destruição e reutilização:

```text
slot 42, geração 8
```

Um `ObjectId` antigo contendo geração 7 não será aceito.

---

## 17.38 Incremento da Geração

Quando um slot for invalidado para reutilização:

```text
generation = generation + 1
```

A operação deverá tratar overflow.

Possíveis políticas:

* nunca reutilizar o slot após overflow;
* falhar fatalmente;
* ampliar a largura;
* retirar o slot permanentemente.

A primeira implementação deverá preferir largura suficientemente grande e retirada do slot em caso extremo.

---

## 17.39 Geração Zero

A implementação poderá reservar geração zero para:

* estado inválido;
* `ObjectId` nulo interno;
* slot nunca utilizado.

Essa decisão deverá ser consistente.

Programas Capi não deverão observar o significado numérico.

---

## 17.40 Representação de `ObjectId`

Um `ObjectId` inicial poderá conter:

```text
ObjectId
├── DomainId
├── slot_index
└── generation
```

Opcionalmente:

```text
├── TypeId
└── flags
```

O tipo esperado poderá permanecer fora da representação e ser garantido estaticamente.

---

## 17.41 Forma Compacta

Uma representação física possível:

```text
128 bits:
├── 64 bits DomainId
├── 32 bits slot
└── 32 bits generation
```

Outra possibilidade:

```text
64 bits:
├── bits de Domain
├── bits de slot
└── bits de geração
```

A escolha deverá considerar:

* quantidade de Domains;
* quantidade de objetos;
* custo de passagem;
* ABI;
* alinhamento;
* longevidade.

A primeira implementação poderá preferir 128 bits por simplicidade.

---

## 17.42 Representação Não Normativa

A largura e divisão de bits não serão contrato da linguagem.

Uma implementação futura poderá utilizar:

* handles;
* ponteiros autenticados;
* índices globais;
* tabelas segmentadas;
* representação comprimida;
* geração ampliada.

A semântica deverá permanecer inalterada.

---

## 17.43 `ObjectId` Tipado

Conceitualmente, `ObjectId<T>` poderá preservar o tipo estático.

Exemplo:

```text
ObjectId<Person>
```

Na representação física, `T` poderá ser apagado após a verificação.

O compilador deverá impedir uso como tipo incompatível.

---

## 17.44 Ausência de Ponteiro Público

`ObjectId` não deverá ser um ponteiro exposto.

O programa não poderá:

* realizar aritmética;
* converter livremente para inteiro;
* acessar memória;
* forjar valor;
* desalocar objeto;
* alterar geração.

Toda resolução deverá ocorrer por operação controlada.

---

## 17.45 Criação de `ObjectId`

A identidade somente deverá ser publicada após:

* slot reservado;
* memória reservada;
* campos inicializados;
* metadados associados;
* objeto marcado como vivo.

Conceitualmente:

```text
slot + generation + DomainId
        │
        ▼
ObjectId
```

---

## 17.46 Identidade Estável

Enquanto o objeto estiver vivo:

* seu `DomainId` permanecerá;
* seu índice lógico permanecerá;
* sua geração permanecerá;
* o endereço físico poderá mudar futuramente;
* o `ObjectId` continuará igual.

A implementação inicial poderá não mover objetos, mas não deverá transformar endereço em identidade pública.

---

## 17.47 Comparação de Identidade

A igualdade entre identidades deverá comparar os componentes lógicos.

Conceitualmente:

```text
same_domain
&& same_slot
&& same_generation
```

A comparação não deverá resolver o objeto nem acessar seus campos.

Ela poderá permanecer válida como comparação de valores mesmo após invalidação, conforme definição normativa.

A possibilidade de comparar identidades obsoletas deverá seguir a especificação.

---

## 17.48 Hashing de `ObjectId`

Quando utilizado como chave, o hash deverá derivar de:

* DomainId;
* slot;
* geração.

Ele não deverá depender do ponteiro físico.

A implementação deverá preservar consistência com igualdade.

---

## 17.49 Serialização

A serialização de `ObjectId` não deverá ser suportada como persistência geral no primeiro estágio.

Uma identidade é válida dentro de:

* uma execução;
* um contexto;
* um lifetime.

Persistir seu valor poderia criar falsa expectativa de validade futura.

Ferramentas de debug poderão imprimi-lo.

---

## 17.50 Formato de Debug

Um formato conceitual poderá ser:

```text
ObjectId(domain=12, slot=42, generation=7)
```

Ou forma compacta:

```text
12:42:7
```

Esse formato não será estável para intercâmbio.

---

## 17.51 Resolução de `ObjectId`

Resolver um `ObjectId` significa localizar o objeto vivo correspondente.

Conceitualmente:

```text
ObjectId
   │
   ▼
localizar Domain
   │
   ▼
validar estado
   │
   ▼
validar slot
   │
   ▼
validar geração
   │
   ▼
validar objeto
   │
   ▼
retornar acesso controlado
```

---

## 17.52 Resolução com Domain Explícito

A estratégia preferencial é resolver com um Domain conhecido.

Exemplo conceitual:

```text
resolve(domain, object_id)
```

O Runtime deverá verificar:

```text
object_id.domain_id == domain.id
```

Isso evita pesquisa global.

---

## 17.53 Resolução por Contexto

Em alguns casos, o compilador poderá provar o Domain e emitir uma operação especializada.

Exemplo:

```text
resolve_known_domain(slot, generation)
```

Essa otimização poderá ser adicionada posteriormente.

A primeira implementação deverá utilizar a forma mais segura e explícita.

---

## 17.54 Validação do Domain

A resolução deverá rejeitar:

* handle nulo;
* Domain destruído;
* Domain em estado incompatível;
* `DomainId` diferente;
* estrutura corrompida.

---

## 17.55 Validação do Índice

O índice deverá ser menor que o tamanho da tabela.

Acesso fora dos limites deverá ser tratado como:

* identidade inválida;
* defeito interno;
* violação de FFI;

conforme a origem.

---

## 17.56 Validação da Geração

A geração do `ObjectId` deverá coincidir com a geração do slot.

Se não coincidir, o identificador é obsoleto ou inválido.

A resolução não poderá retornar o novo objeto que reutilizou o slot.

---

## 17.57 Validação do Estado

Somente slots vivos deverão produzir acesso normal.

Slots:

* livres;
* reservados;
* em inicialização;
* finalizando;
* destruídos;

deverão ser rejeitados.

---

## 17.58 Validação de Tipo

Quando necessária dinamicamente, a resolução poderá validar:

* `TypeId` exato;
* subtipo permitido;
* interface implementada;
* metadados compatíveis.

Sempre que o tipo for garantido estaticamente, essa verificação poderá ser eliminada.

---

## 17.59 Resultado da Resolução

Uma operação interna poderá retornar:

```text
pointer + metadata
```

ou um handle seguro equivalente.

O ponteiro retornado deverá possuir uso limitado ao contexto da operação.

O código não poderá armazená-lo além do lifetime permitido sem análise.

---

## 17.60 Empréstimo Temporário

A resolução deverá produzir um acesso temporário.

Conceitualmente:

```text
ObjectId<T>
        │
        ▼
borrowed object access
```

A relação entre resolução e borrowing será controlada pelo compilador.

O Runtime apenas materializará o endereço.

---

## 17.61 Cache de Resolução

Uma implementação futura poderá armazenar:

* ponteiro em cache;
* versão do Domain;
* geração;
* caminho rápido.

A primeira implementação não deverá depender de cache.

Qualquer cache deverá invalidar corretamente.

---

## 17.62 Resolução Inline

Após estabilização, o backend poderá emitir a verificação diretamente:

```text
load slot
compare generation
check state
load pointer
```

Em caso de falha, chamará o Runtime.

Inicialmente, a chamada ao Runtime será preferida para simplificar auditoria.

---

## 17.63 Falha de Resolução

A política dependerá do contexto.

Uma identidade inválida originada de código Capi seguro indicará:

* defeito do compilador;
* defeito do Runtime;
* corrupção;
* violação por FFI insegura.

A primeira implementação poderá tratar como falha fatal.

APIs que modelarem busca opcional por identidade deverão possuir semântica explícita e não reutilizar silenciosamente a mesma operação.

---

## 17.64 `ObjectId` Inválido Opcional

Se a linguagem precisar representar ausência de identidade, deverá utilizar:

```text
Optional<ObjectId<T>>
```

e não um `ObjectId` especial nulo, salvo otimização interna invisível.

---

## 17.65 Tabela Segmentada

Para evitar realocações de uma tabela única, os slots poderão ser armazenados em segmentos.

Conceitualmente:

```text
SlotTable
├── Segment 0
├── Segment 1
├── Segment 2
└── ...
```

Isso oferece:

* endereços internos mais estáveis;
* crescimento incremental;
* menor custo de realocação;
* possibilidade de índices compostos.

A primeira versão poderá utilizar `Vec<Slot>` se não expuser ponteiros para elementos.

---

## 17.66 Estabilidade da Tabela

A realocação da tabela não deverá invalidar `ObjectId`, pois ele contém índice, não ponteiro para slot.

Entretanto, operações internas não poderão manter referência ao slot durante crescimento concorrente ou reentrante.

No Runtime sequencial inicial, esse risco será menor.

---

## 17.67 Ordem de Criação

O Domain deverá manter informação suficiente para executar destruição determinística.

Uma estratégia simples será registrar a ordem de criação:

```text
creation_order = [slot0, slot5, slot2, ...]
```

Na destruição:

```text
iterar em ordem inversa
```

A adequação dessa ordem deverá seguir a especificação.

---

## 17.68 Registro de Objetos

Após publicação, o slot deverá ser incluído na estrutura de finalização.

O registro deverá ocorrer uma única vez.

Falha nessa etapa deverá impedir publicação.

---

## 17.69 Objetos Sem Destruição

Tipos triviais poderão não exigir chamada de destrutor.

Ainda assim, seu slot deverá ser invalidado.

A finalização poderá testar uma flag:

```text
has_destructor
```

---

## 17.70 Objetos com Recursos Externos

Objetos que contenham:

* arquivos;
* sockets;
* memória externa;
* FFI;
* locks futuros;

deverão possuir destrutor.

O Domain deverá invocar esse destrutor antes de liberar os blocos.

---

## 17.71 Grafos de Objetos

Objetos poderão referenciar outros objetos do mesmo Domain conforme a semântica.

Exemplo:

```text
A → B
B → C
C → A
```

O Runtime não precisará percorrer o grafo para descobrir alcançabilidade.

Todos os objetos registrados serão destruídos com o Domain.

---

## 17.72 Ausência de Coleta de Ciclos

A existência de ciclos não requer:

* tracing;
* marcação;
* contagem de referências;
* detector de ciclos.

O Domain conhece todos os objetos que deve destruir.

---

## 17.73 Referências entre Domains

Referências entre Domains deverão obedecer às regras estáticas.

O Runtime poderá manter apenas validações residuais.

A implementação deverá distinguir:

* referência permitida;
* identidade armazenada;
* empréstimo temporário;
* escape inválido;
* relação pai-filho.

---

## 17.74 Referência de Filho para Pai

Conforme a semântica, um objeto em Domain filho poderá referenciar objeto no pai se o pai viver mais.

Essa relação não impede destruição do filho.

---

## 17.75 Referência de Pai para Filho

Uma referência duradoura do pai para objeto do filho poderá violar lifetime.

O compilador deverá rejeitá-la quando inválida.

O Runtime não deverá manter o filho vivo automaticamente.

---

## 17.76 Ausência de Promoção Implícita

Um objeto não deverá ser movido automaticamente para outro Domain apenas porque escapou.

Também não deverá ocorrer:

* promoção para heap global;
* prolongamento automático de lifetime;
* retenção implícita.

O programa deverá utilizar mecanismos explícitos permitidos.

---

## 17.77 Transferência entre Domains

Caso a linguagem permita transferência, ela deverá possuir semântica específica.

Uma transferência poderá exigir:

* mover valor;
* recriar objeto;
* preservar ou alterar identidade;
* validar referências;
* atualizar slots;
* impedir acessos anteriores.

Esse mecanismo não fará parte da primeira implementação, salvo necessidade normativa imediata.

---

## 17.78 Movimentação Física

A implementação inicial poderá garantir que objetos não se movem dentro do Domain.

Isso simplifica:

* ponteiros temporários;
* FFI;
* finalização;
* layout.

Entretanto, a identidade pública continuará sendo `ObjectId`.

---

## 17.79 Compactação Futura

Uma implementação futura poderá compactar blocos.

Nesse caso, deverá:

* atualizar ponteiros nos slots;
* preservar `ObjectId`;
* respeitar empréstimos ativos;
* atualizar referências físicas internas, se existirem;
* coordenar com FFI.

Essa possibilidade justifica não usar endereço como identidade.

---

## 17.80 Alocação de Tipos por Valor

Tipos por valor locais não precisam ser alocados em Domain.

Poderão residir em:

* registradores;
* pilha;
* agregados;
* memória temporária.

Domains são prioritariamente relevantes para objetos por identidade e recursos cuja semântica os utilize.

---

## 17.81 Valores Dentro de Objetos

Campos por valor estarão fisicamente dentro do objeto, salvo otimização.

Sua destruição deverá ocorrer como parte da destruição do objeto.

---

## 17.82 Identidades Dentro de Objetos

Campos de identidade armazenarão `ObjectId` ou representação equivalente.

Eles não armazenarão obrigatoriamente ponteiro direto.

Isso permite:

* validação;
* estabilidade;
* ciclos;
* movimentação futura.

---

## 17.83 Referências Temporárias Dentro da IR

Durante uma operação, a IR poderá utilizar ponteiro temporário resolvido.

Esse ponteiro não deverá escapar para armazenamento permanente sem contrato.

O validador e o sistema de tipos deverão distinguir:

* identidade persistente;
* acesso resolvido temporário.

---

## 17.84 Reserva de Slot

A criação de objeto poderá iniciar reservando um slot.

Sequência:

```text
obter slot livre ou crescer tabela
        │
        ▼
marcar Reserved
        │
        ▼
capturar geração atual
```

Nenhum `ObjectId` deverá ser retornado ainda.

---

## 17.85 Reserva de Memória

Após o slot:

```text
obter layout
        │
        ▼
alocar memória
        │
        ▼
associar pointer temporário
        │
        ▼
marcar Initializing
```

Se a alocação falhar, o slot deverá voltar ao estado livre sem publicar identidade.

---

## 17.86 Inicialização de Campos

O código gerado deverá inicializar os campos.

O Runtime poderá fornecer:

* memória não inicializada;
* memória zerada;
* operação de construção.

A política deverá ser explícita.

Zerar memória não substitui inicialização semântica.

---

## 17.87 Bitmap de Inicialização

Para objetos complexos sujeitos a falha de construção, poderá existir controle de campos inicializados.

Possibilidades:

* código gerado conhece o progresso;
* bitmap temporário;
* caminhos de cleanup específicos.

A primeira implementação deverá preferir cleanup gerado pelo compilador.

---

## 17.88 Publicação do Objeto

Após sucesso:

* metadados estarão completos;
* estado mudará para vivo;
* slot será registrado na ordem de criação;
* `ObjectId` será construído;
* identidade será retornada.

A publicação deverá ser atômica do ponto de vista sequencial.

---

## 17.89 Falha Antes da Publicação

Se qualquer etapa falhar:

* destruir campos inicializados;
* liberar recursos externos;
* marcar slot livre;
* preservar ou atualizar geração;
* não registrar na ordem de destruição;
* não retornar identidade.

---

## 17.90 Reutilização de Slot após Falha

Como o `ObjectId` não foi publicado, a geração poderá permanecer.

Entretanto, para simplificar invariantes, a implementação poderá incrementá-la.

A decisão deverá ser documentada e testada.

---

## 17.91 Destruição Antecipada de Objeto

Se a linguagem permitir destruir objeto antes do Domain:

```text
Alive
   │
   ▼
Finalizing
   │
   ▼
Destroyed/Vacant
```

O slot deverá ser invalidado.

O espaço físico poderá permanecer.

---

## 17.92 Remoção da Ordem de Criação

Um objeto destruído antecipadamente deverá:

* ser removido da estrutura de finalização; ou
* permanecer marcado como já destruído.

A segunda estratégia pode ser mais simples.

Na destruição do Domain, entradas destruídas serão ignoradas.

---

## 17.93 Destruição do Domain

A destruição deverá seguir sequência semelhante a:

```text
validar estado Active
        │
        ▼
marcar Finalizing
        │
        ▼
destruir filhos
        │
        ▼
finalizar objetos em ordem definida
        │
        ▼
invalidar slots
        │
        ▼
liberar blocos
        │
        ▼
desassociar do pai
        │
        ▼
marcar Destroyed
        │
        ▼
liberar estrutura
```

---

## 17.94 Destruição de Filhos

Domains filhos ativos deverão ser destruídos antes do pai.

A ordem entre irmãos deverá ser determinística.

Uma possibilidade é ordem inversa de criação.

---

## 17.95 Bloqueio de Novas Alocações

Ao entrar em `Finalizing`, qualquer tentativa de alocar deverá falhar.

Isso inclui chamadas indiretas de finalizadores.

Caso a especificação permita alocação limitada durante finalização, deverá existir mecanismo específico.

A primeira implementação deverá proibi-la.

---

## 17.96 Finalização Reentrante

O Runtime deverá detectar chamada de destruição do mesmo Domain durante sua finalização.

Essa situação deverá resultar em:

* no-op controlado, se semanticamente seguro; ou
* falha fatal.

A primeira implementação deverá preferir falha explícita.

---

## 17.97 Ordem de Destruição de Objetos

A estratégia inicial recomendada será ordem inversa de publicação.

Exemplo:

```text
criação: A, B, C
destruição: C, B, A
```

Essa ordem é:

* simples;
* determinística;
* semelhante à destruição lexical;
* adequada a muitos recursos.

Ela somente deverá ser adotada se compatível com a especificação.

---

## 17.98 Dependência entre Objetos

A ordem inversa de criação não garante resolver qualquer dependência lógica.

Programas não deverão depender de acesso a objetos arbitrários durante finalização, salvo regra definida.

A semântica de finalizadores deverá restringir comportamentos perigosos.

---

## 17.99 Invalidação Antes ou Depois do Destrutor

Existem duas estratégias:

### Invalidação antes

Impede acessos durante o destrutor.

### Invalidação depois

Permite acesso controlado ao próprio objeto.

A implementação poderá marcar como `Finalizing`, permitindo somente acesso interno específico.

Após o destrutor, passará a inválido.

---

## 17.100 Destruição de Campos

A rotina de destruição deverá:

1. executar finalizador definido, se houver;
2. destruir campos na ordem definida;
3. encerrar recursos;
4. marcar objeto como destruído.

A ordem entre finalizador do usuário e campos deverá seguir a especificação.

---

## 17.101 Finalizador que Acessa Outros Objetos

Esse comportamento deverá ser estritamente definido.

Riscos:

* acessar objeto já finalizado;
* depender de ordem;
* ressuscitar identidade;
* criar ciclos de finalização;
* falhar.

A primeira implementação poderá limitar finalizadores a recursos próprios.

---

## 17.102 Ressurreição

Um objeto em finalização não deverá ser tornado vivo novamente.

O `ObjectId` não deverá ser republicado.

A ressurreição não fará parte do modelo inicial.

---

## 17.103 Invalidação de Slots

Após destruição:

```text
pointer = null
state = Vacant ou Destroyed
generation += 1
```

A ordem deverá impedir resolução concorrente futura.

No modelo sequencial, a atualização será direta.

---

## 17.104 Liberação dos Blocos

Após todos os destruidores:

* blocos comuns serão liberados;
* blocos dedicados serão liberados;
* buffers internos serão liberados;
* tabela de slots será liberada;
* estruturas de debug serão liberadas.

---

## 17.105 Falha Durante Destruição

Uma falha fatal durante finalização poderá impedir conclusão completa.

A política inicial poderá:

* registrar a falha;
* tentar evitar reentrada;
* abortar imediatamente.

Continuar destruindo após corrupção pode ser inseguro.

---

## 17.106 Destruição em Caso de Erro Recuperável

Erros comuns do programa não deverão impedir destruição.

Retorno de `Result` deverá conduzir por caminhos de cleanup gerados pelo compilador.

O Domain será destruído normalmente ao fim do escopo.

---

## 17.107 Destruição em Panic

Como o Runtime inicial poderá abortar sem unwinding, uma falha fatal poderá não executar destruição completa.

Isso é aceitável para encerramento imediato do processo.

A semântica deverá distinguir:

* encerramento normal;
* propagação de erro;
* falha fatal.

---

## 17.108 Recursos Persistentes do Sistema

Ao abortar o processo, o sistema operacional recuperará memória e handles do processo.

Entretanto, operações externas parcialmente concluídas poderão permanecer.

Por isso, `panic` não deverá ser utilizado para fluxo comum.

---

## 17.109 Validação Estática de Lifetime

O compilador deverá impedir:

* retorno de identidade inválida;
* armazenamento em contexto mais longo;
* captura indevida;
* referência de pai para filho inválida;
* uso após fechamento de escopo.

O Runtime deverá atuar como defesa residual.

---

## 17.110 Validação Dinâmica Residual

A implementação inicial poderá manter verificações em release para:

* Domain correto;
* índice válido;
* geração;
* estado vivo.

Posteriormente, algumas poderão ser eliminadas quando comprovadas.

---

## 17.111 Eliminação de Verificações

O compilador poderá eliminar uma verificação quando provar:

* Domain conhecido;
* slot não reutilizado;
* lifetime válido;
* acesso dominado por resolução;
* ausência de FFI insegura.

Essa otimização deverá ser conservadora.

---

## 17.112 Performance da Resolução

O custo esperado inicial será:

```text
1 acesso ao Domain
+ 1 acesso à tabela
+ comparações
+ 1 carga de ponteiro
```

Esse custo deverá ser medido.

O projeto deverá evitar otimização prematura antes de validar a semântica.

---

## 17.113 Localidade da Tabela

Slots contíguos favorecem cache.

Entretanto, grandes Domains poderão produzir tabelas extensas.

Segmentação poderá melhorar crescimento.

A decisão deverá ser orientada por benchmarks.

---

## 17.114 Overhead por Objeto

O projeto deverá medir:

* tamanho do slot;
* tamanho do cabeçalho;
* alinhamento;
* metadados;
* ordem de criação.

O overhead não deverá ser escondido.

---

## 17.115 Objetos Pequenos

Para objetos muito pequenos, metadados podem superar o tamanho dos campos.

Possíveis otimizações futuras:

* slots compactos;
* cabeçalho externo;
* agrupamento por tipo;
* geração reduzida;
* alocação especializada.

A primeira versão priorizará clareza.

---

## 17.116 Pools por Tipo

Uma implementação futura poderá organizar blocos por tipo.

Vantagens:

* layout uniforme;
* melhor localidade;
* slots compactos;
* destruição especializada.

Desvantagens:

* complexidade;
* fragmentação entre tipos;
* gerenciamento adicional.

Não será requisito inicial.

---

## 17.117 Tabela de Metadados por Slot

O slot poderá armazenar ponteiro para `TypeMetadata`.

Alternativamente, o objeto poderá possuir cabeçalho.

A duplicação deverá ser evitada quando possível.

---

## 17.118 Objetos de Tipo Final

Para classes `final`, algumas informações dinâmicas poderão ser simplificadas.

Exemplos:

* vtable constante;
* ausência de checagem de subtipo;
* destrutor direto.

Essas otimizações serão tratadas após o modelo básico.

---

## 17.119 Domains Vazios

Criar e destruir Domain vazio deverá ser válido.

O custo deverá ser baixo.

A estrutura poderá adiar alocação de blocos até o primeiro objeto.

---

## 17.120 Inicialização Preguiçosa

Elementos como:

* tabela de slots;
* primeiro bloco;
* free list;

poderão ser criados sob demanda.

Isso reduz custo de Domains vazios.

---

## 17.121 Domain com Muitos Objetos

Testes deverão cobrir:

* milhões de objetos pequenos;
* crescimento de slots;
* múltiplos blocos;
* reutilização;
* destruição.

A implementação deverá evitar complexidade quadrática.

---

## 17.122 Domain com Muitos Filhos

A hierarquia também deverá ser testada.

A destruição não deverá:

* estourar pilha facilmente;
* possuir traversal incorreto;
* perder filhos.

Uma implementação iterativa poderá ser necessária para profundidade elevada.

---

## 17.123 Profundidade de Hierarquia

O Runtime poderá definir limite defensivo.

Entretanto, limites públicos deverão ser documentados.

A primeira implementação deverá evitar recursão não controlada.

---

## 17.124 Concorrência Futura

O modelo inicial será sequencial.

Para concorrência futura, deverão ser considerados:

* acesso à tabela;
* geração;
* destruição;
* transferência;
* ownership de Domain por thread;
* sincronização;
* resolução.

A estrutura atual deverá evitar globals que inviabilizem essa evolução.

---

## 17.125 Domain Confinado

Uma estratégia futura provável será Domain confinado a uma unidade de execução.

Isso reduzirá necessidade de locks.

A transferência exigirá operação explícita.

---

## 17.126 Resolução Concorrente

Se Domains compartilhados forem permitidos futuramente, a resolução poderá exigir:

* atomics;
* epochs;
* locks;
* read guards;
* hazard pointers;
* regras mais restritas.

Nada disso será introduzido no bootstrap sequencial.

---

## 17.127 IDs Globais e Concorrência

O gerador de `DomainId` poderá usar contador atômico no futuro.

Inicialmente, um contador comum no `RuntimeContext` será suficiente.

---

## 17.128 FFI e `ObjectId`

A FFI pública não deverá expor o layout interno de `ObjectId` sem contrato específico.

Opções:

* handle opaco;
* struct estável;
* ponte de funções;
* wrapper C.

A primeira FFI poderá tratar identidades como valor opaco de largura fixa.

---

## 17.129 Ponteiros para FFI

Quando código externo precisar de ponteiro para objeto:

* o objeto deverá permanecer imóvel durante a chamada;
* o lifetime deverá ser delimitado;
* o ponteiro não poderá ser retido;
* o Domain deverá permanecer ativo;
* a operação será insegura.

A API deverá explicitar essas condições.

---

## 17.130 Retenção Externa

Código externo não poderá prolongar lifetime do objeto automaticamente.

Se retenção for necessária, deverá existir mecanismo específico de handle externo.

Isso está fora do primeiro estágio.

---

## 17.131 Forja de Identidade por FFI

Valores recebidos de código externo deverão ser validados.

A capacidade de construir `ObjectId` arbitrário deverá ser restrita a APIs inseguras.

---

## 17.132 Testes Unitários da Tabela de Slots

Deverão cobrir:

* criação;
* reserva;
* publicação;
* resolução;
* invalidação;
* reutilização;
* incremento de geração;
* overflow;
* índice inválido;
* estado inválido.

---

## 17.133 Testes de `ObjectId`

Deverão validar:

* igualdade;
* desigualdade;
* hash;
* estabilidade;
* Domain diferente;
* slot diferente;
* geração diferente;
* uso após destruição;
* reutilização de slot.

---

## 17.134 Testes de Alocação

Deverão cobrir:

* diferentes tamanhos;
* diferentes alinhamentos;
* múltiplos blocos;
* objetos grandes;
* falha;
* overflow;
* Domain vazio;
* crescimento.

---

## 17.135 Testes de Inicialização Parcial

Deverão simular falha:

* no primeiro campo;
* no campo intermediário;
* após recurso externo;
* antes da publicação;
* após reserva do slot.

O teste deverá confirmar:

* cleanup;
* ausência de identidade publicada;
* slot reutilizável;
* ausência de double destruction.

---

## 17.136 Testes de Destruição

Deverão validar:

* ordem;
* destruição única;
* slots inválidos;
* filhos antes do pai;
* grafos cíclicos;
* objetos triviais;
* recursos externos;
* destruição antecipada.

---

## 17.137 Testes de Ciclos

Exemplos:

```text
A → A
A → B → A
A → B → C → A
```

Todos deverão ser liberados com o Domain.

---

## 17.138 Testes de Hierarquia

Deverão incluir:

```text
raiz
├── filho
└── filho
    └── neto
```

Validar:

* criação;
* destruição automática;
* ordem;
* rejeição de uso posterior.

---

## 17.139 Testes de Identidade Obsoleta

Cenário:

```text
criar A no slot 10 geração 4
destruir A
criar B no slot 10 geração 5
resolver identidade de A
```

Resultado:

```text
falha
```

Nunca deverá retornar B.

---

## 17.140 Testes de Domain Incorreto

Um `ObjectId` de Domain A não deverá resolver em Domain B, mesmo que:

* slot coincida;
* geração coincida;
* tipo coincida.

---

## 17.141 Fuzzing de Slots

O fuzzing poderá gerar sequências como:

* reservar;
* cancelar;
* publicar;
* destruir;
* reutilizar;
* resolver;
* finalizar Domain.

O objetivo será encontrar transições inválidas.

---

## 17.142 Property-Based Testing

Propriedades úteis:

* identidade publicada resolve enquanto viva;
* identidade destruída nunca resolve;
* identidades diferentes não se tornam iguais;
* destruição do Domain invalida todos os objetos;
* cada objeto é destruído no máximo uma vez;
* bytes liberados não excedem bytes alocados;
* filhos não sobrevivem ao pai.

---

## 17.143 Sanitizers

Os testes deverão ser executados com ferramentas para detectar:

* use-after-free;
* double free;
* out-of-bounds;
* desalinhamento;
* vazamento;
* comportamento indefinido.

---

## 17.144 Modo de Verificação Máxima

Em testes, cada operação poderá validar:

* magic number do Domain;
* estado;
* ownership;
* ponteiro pertencente a bloco;
* metadados;
* geração;
* ordem.

Esse modo não será necessário em release.

---

## 17.145 Magic Numbers

Estruturas internas poderão possuir valores sentinela em debug.

Exemplo:

```text
DOMAIN_ALIVE_MAGIC
DOMAIN_DEAD_MAGIC
```

Isso auxilia a detectar handles inválidos.

Não constitui proteção de segurança forte.

---

## 17.146 Rastreamento de Origem

Em debug, slots poderão registrar:

* arquivo;
* linha;
* função;
* stack resumida;
* ordem de criação.

Essas informações ajudam a investigar vazamentos lógicos e finalização.

---

## 17.147 Estatísticas por Domain

Poderão incluir:

```text
objetos criados
objetos vivos
objetos destruídos
slots reutilizados
bytes reservados
bytes utilizados
blocos
pico de memória
```

---

## 17.148 Dump de Domain

Uma ferramenta interna poderá produzir:

```text
Domain 12
State: Active
Parent: 1
Blocks: 4
Slots: 120
Alive: 97
Destroyed: 23
Bytes reserved: ...
```

Opcionalmente, listar tipos.

---

## 17.149 Dump de `ObjectId`

Para um identificador:

```text
Domain: 12
Slot: 42
Generation: 7
Slot state: Alive
Type: Person
Pointer: 0x...
```

O endereço deverá aparecer apenas em debug.

---

## 17.150 Métricas de Performance

Benchmarks deverão medir:

* criação de Domain;
* primeira alocação;
* alocação subsequente;
* resolução;
* destruição;
* reutilização de slot;
* overhead por objeto;
* memória desperdiçada por bloco.

---

## 17.151 Comparação com Alocação Tradicional

Benchmarks poderão comparar:

* Domain;
* `Box`;
* `Vec`;
* malloc/free;
* arena simples.

O objetivo não será copiar outro modelo, mas compreender custos.

---

## 17.152 Critérios de Aceitação de Performance

Durante o bootstrap, será aceitável algum overhead de validação.

Entretanto, deverão ser evitados:

* busca linear para resolver identidade;
* alocação global por objeto sem necessidade;
* destruição quadrática;
* tabela com realocação excessiva;
* locks globais no modo sequencial.

---

## 17.153 Otimizações Futuras

Possíveis:

* slots segmentados;
* free list intrusiva;
* metadados compactos;
* blocos por tamanho;
* cache de último slot;
* resolução inline;
* remoção de validação comprovada;
* cabeçalho reduzido;
* alocação especializada;
* agrupamento por tipo.

---

## 17.154 Compatibilidade com Cranelift

O backend deverá conhecer as assinaturas das operações como:

```text
domain_create
domain_destroy
object_allocate
object_publish
object_resolve
object_destroy
```

A primeira implementação poderá utilizar chamadas de função.

---

## 17.155 Compatibilidade com LLVM

LLVM deverá consumir a mesma ABI.

Poderá aplicar:

* inlining;
* devirtualização;
* eliminação de verificações;
* propagação de metadados.

O comportamento deverá ser equivalente.

---

## 17.156 Operações da IR

A IR da Capi poderá representar operações de alto nível como:

```text
domain.create
domain.destroy
object.allocate
object.publish
object.resolve
object.identity
object.destroy
```

O Lowering de backend decidirá entre:

* chamada ao Runtime;
* expansão inline;
* instruções nativas.

---

## 17.157 Validação da IR

O validador deverá verificar:

* uso de Domain válido;
* publicação única;
* resolução com tipo correto;
* destruição no fluxo adequado;
* ausência de objeto usado antes de publicação;
* ausência de operação após destruição comprovada.

---

## 17.158 Cleanup na IR

A destruição de Domain deverá aparecer em todos os caminhos relevantes.

Exemplo:

```text
create domain
    │
    ├── caminho normal → destroy
    ├── retorno → destroy
    └── erro → destroy
```

O compilador deverá construir blocos de cleanup.

---

## 17.159 Otimização de Domain Vazio

Se o compilador provar que um Domain:

* não recebe objetos;
* não possui efeitos;
* não possui filhos;

poderá eliminar sua criação.

---

## 17.160 Eliminação de Alocação

Objetos cuja identidade não é observada e que não escapam poderão, futuramente, sofrer:

* stack allocation;
* scalar replacement;
* eliminação.

A semântica deverá ser preservada.

---

## 17.161 Identidade Observável

Uma otimização não poderá eliminar ou fundir objetos quando:

* `ObjectId` é obtido;
* igualdade de identidade é observada;
* há despacho dinâmico dependente;
* há FFI;
* há finalizador observável.

---

## 17.162 Escape Analysis

A análise futura poderá determinar:

* objeto restrito à função;
* objeto restrito ao bloco;
* objeto pertencente ao Domain;
* identidade não observada.

Isso poderá reduzir custos.

Não será requisito inicial.

---

## 17.163 Domain Especializado

O compilador poderá futuramente gerar Domains especializados quando conhece:

* quantidade de objetos;
* tipos;
* tamanho;
* ausência de filhos.

A primeira implementação utilizará estrutura genérica.

---

## 17.164 Domain Raiz Especial

O Domain raiz poderá possuir política diferente:

* lifetime do processo;
* maior duração;
* objetos globais;
* argumentos;
* serviços.

Ainda assim, deverá respeitar as mesmas garantias básicas.

---

## 17.165 Destruição do Domain Raiz

No encerramento normal:

* módulos serão finalizados;
* objetos globais serão destruídos;
* serviços serão encerrados;
* o Domain raiz será destruído.

O Runtime deverá evitar depender de destrutores estáticos de Rust fora dessa ordem.

---

## 17.166 Domains Internos do Runtime

O Runtime poderá utilizar estruturas de alocação internas.

Elas não deverão ser expostas como Domains da linguagem.

A distinção deverá permanecer clara.

---

## 17.167 Arena do Compilador

Uma arena usada pelo compilador escrito em Capi poderá ser construída sobre Domain, mas sua API será de coleção.

Ela poderá:

* alocar nós;
* retornar `NodeId`;
* iterar;
* destruir coletivamente.

Não deverá expor `ObjectId` quando índices tipados forem mais apropriados.

---

## 17.168 Diferença entre `ObjectId` e IDs de Arena

`ObjectId` representa identidade de objetos da linguagem.

IDs como:

```text
NodeId
TypeId
SymbolId
```

representam índices de estruturas de dados.

Eles não deverão ser confundidos.

---

## 17.169 Segurança contra ABA

A geração evita problema semelhante a ABA:

```text
slot contém A
slot fica livre
slot contém B
```

Sem geração, o índice parece igual.

Com geração, A e B permanecem distintos.

---

## 17.170 Largura da Geração

A largura deverá tornar wraparound praticamente impossível.

Sugestão inicial:

```text
32 ou 64 bits
```

A decisão deverá considerar o tamanho total do `ObjectId`.

---

## 17.171 Esgotamento de Slots

Quando a tabela atingir o máximo representável:

* novas alocações deverão falhar;
* não poderá ocorrer wrap;
* o erro deverá ser claro.

Isso será extremamente improvável com largura adequada.

---

## 17.172 Esgotamento de `DomainId`

Também deverá produzir falha controlada.

O contador não poderá voltar a zero silenciosamente.

---

## 17.173 Configuração de Limites

O modo de teste poderá utilizar limites pequenos para forçar:

* crescimento;
* overflow;
* falha;
* reutilização.

Em produção, os limites serão amplos.

---

## 17.174 Licenciamento e Dependências

A implementação deverá evitar dependências complexas para:

* arena;
* slots;
* generational index;
* alocação.

Uma implementação própria pequena poderá ser preferível devido à criticidade e integração.

Caso biblioteca externa seja utilizada, deverá ser auditada.

---

## 17.175 ADRs Recomendados

Deverão ser criados ADRs para:

```text
ADR — Representação inicial de Domain
ADR — Alocação por blocos
ADR — Tabela de slots geracionais
ADR — Formato inicial de ObjectId
ADR — Política de geração
ADR — Ordem de destruição
ADR — Proibição de alocação durante finalização
ADR — Estratégia de resolução
ADR — Ausência inicial de movimentação
ADR — Hierarquia de Domains
```

---

## 17.176 Documentação dos Invariantes

O módulo deverá documentar invariantes como:

1. todo objeto vivo possui exatamente um slot vivo;
2. todo slot vivo aponta para memória pertencente ao Domain;
3. a geração publicada coincide com a geração do slot;
4. um slot livre não aponta para objeto vivo;
5. um objeto publicado aparece na estrutura de finalização;
6. um objeto é finalizado no máximo uma vez;
7. um Domain destruído não possui slots vivos;
8. nenhum filho permanece ativo após destruição do pai;
9. um `ObjectId` antigo nunca resolve para novo objeto;
10. endereço físico não define identidade.

---

## 17.177 Encapsulamento dos Invariantes

Somente o componente de Domains deverá modificar:

* estado de slots;
* geração;
* ponteiros;
* free list;
* ordem de criação.

Outros módulos deverão utilizar APIs controladas.

---

## 17.178 APIs Internas Seguras

Exemplos conceituais:

```text
Domain::create(parent)
Domain::reserve_object(type_metadata)
ObjectReservation::initialize(...)
ObjectReservation::publish()
Domain::resolve(object_id)
Domain::destroy_object(object_id)
Domain::destroy()
```

O padrão de reserva poderá garantir cleanup automático em caso de falha no código Rust.

---

## 17.179 RAII Interno em Rust

Embora Capi não use ARC ou GC, o Runtime Rust poderá utilizar RAII internamente para:

* desfazer reserva;
* liberar bloco temporário;
* cancelar slot;
* proteger inicialização parcial.

Isso é detalhe de implementação.

---

## 17.180 Guard de Reserva

Uma estrutura Rust poderá representar:

```text
ObjectReservation
```

Enquanto não for publicada:

* seu `Drop` cancela a reserva;
* destrói campos temporários;
* libera recursos necessários.

Após publicação, o guard é consumido.

---

## 17.181 Guard de Finalização

A finalização poderá utilizar guard para garantir que o estado não permaneça incorreto em caminhos internos.

Entretanto, falhas fatais com abortamento não executarão `Drop`.

Os invariantes deverão ser estabelecidos antes de operações arriscadas.

---

## 17.182 Ausência de Panic Rust na ABI

Panics Rust não deverão atravessar a fronteira FFI.

Funções exportadas deverão:

* evitar panic;
* capturar quando necessário;
* converter para falha fatal;
* abortar de forma controlada.

---

## 17.183 Poisoning Interno

Se uma operação interna falhar parcialmente, o Domain poderá ser marcado como corrompido.

Nesse estado:

* não aceitará novas operações;
* somente poderá abortar ou finalizar defensivamente.

A primeira implementação poderá abortar imediatamente.

---

## 17.184 Validação na Criação

A criação deverá validar:

* pai ativo;
* Runtime inicializado;
* configuração válida;
* ausência de overflow;
* alocação das estruturas.

---

## 17.185 Validação na Destruição

A destruição deverá validar:

* handle válido;
* estado ativo;
* ausência de destruição anterior;
* relacionamento com pai;
* filhos consistentes.

---

## 17.186 Destruição Explícita e Automática

O código gerado poderá chamar explicitamente:

```text
domain_destroy
```

O Runtime também destruirá o Domain raiz no encerramento.

Não deverá existir coletor que descubra Domains abandonados.

---

## 17.187 Domain Perdido

Código Capi seguro não deverá perder o handle de um Domain ativo sem destruí-lo.

O compilador deverá garantir o cleanup.

Em debug, o Runtime poderá detectar Domains ativos no encerramento.

---

## 17.188 Relatório de Domains Não Destruídos

Antes de destruir o Domain raiz, o Runtime poderá verificar filhos inesperados.

Em modo de teste, isso deverá falhar.

Em release, poderá destruir defensivamente.

---

## 17.189 Domains e Exceções Externas

Se FFI utilizar mecanismo de exceção externo, ele não poderá atravessar código Capi sem adaptação.

Caso contrário, o cleanup de Domains poderia ser ignorado.

A FFI deverá capturar e converter.

---

## 17.190 Sinais do Sistema

Sinais fatais como falha de segmentação não permitirão destruição segura.

O Runtime poderá registrar informação mínima, mas não deverá tentar finalizar estruturas possivelmente corrompidas.

---

## 17.191 Cancelamento Futuro

Em concorrência ou tarefas, cancelamento deverá executar cleanup estruturado.

Esse recurso dependerá de integração posterior.

---

## 17.192 Persistência e Domains

Domains são estruturas de execução.

Persistir um grafo exigirá serialização explícita.

Não será possível salvar diretamente:

* slots;
* ponteiros;
* gerações;
* `DomainId`;
* `ObjectId`.

---

## 17.193 Clonagem de Grafos

Uma biblioteca futura poderá copiar objetos entre Domains.

Isso exigirá:

* mapeamento de identidades;
* reconstrução;
* tratamento de ciclos;
* validação de tipos.

Não será responsabilidade do Runtime inicial.

---

## 17.194 Snapshot de Debug

Ferramentas poderão gerar snapshot descritivo:

* tipos;
* relações;
* identidades;
* tamanhos.

Esse snapshot não será restaurável como estado de execução.

---

## 17.195 Compatibilidade de Versão

O formato de `ObjectId` poderá variar entre versões da Toolchain.

Como não é persistente, isso é aceitável antes da ABI estável.

Entre módulos vinculados, a versão deverá coincidir.

---

## 17.196 Bibliotecas Estáticas Separadas

Caso código de diferentes bibliotecas gere objetos, todos deverão usar ABI compatível.

O `DomainId` e os slots pertencerão ao Runtime do processo.

Não deverá haver múltiplas cópias independentes incompatíveis do Runtime no mesmo executável.

---

## 17.197 Runtime Único por Processo

A linkedição deverá evitar duplicar estado global do Runtime.

Bibliotecas estáticas deverão resolver para uma única instância.

A arquitetura com `RuntimeContext` poderá permitir múltiplos contextos futuramente, mas isso será explícito.

---

## 17.198 Testes de Múltiplos Contextos

Não serão exigidos no primeiro estágio.

Se o RuntimeContext for explícito, testes unitários poderão criar contextos isolados.

---

## 17.199 Critérios de Conclusão de Domains Básicos

A implementação básica será considerada concluída quando:

* Domains puderem ser criados;
* blocos puderem ser alocados;
* objetos puderem ser registrados;
* slots possuírem gerações;
* `ObjectId` puder ser criado;
* identidades puderem ser resolvidas;
* slots reutilizados não aceitarem IDs antigos;
* Domain puder ser destruído;
* todos os objetos forem invalidados;
* grafos cíclicos forem liberados;
* testes unitários e funcionais passarem.

---

## 17.200 Critérios de Conclusão para Bootstrap

O suporte estará pronto para o bootstrap quando também:

* hierarquia de Domains estiver funcional;
* finalização determinística estiver validada;
* objetos com recursos externos forem suportados;
* diagnósticos de Runtime estiverem disponíveis;
* instrumentação estiver disponível;
* Cranelift utilizar a ABI;
* LLVM puder utilizar a mesma ABI;
* a Biblioteca Mínima puder armazenar objetos;
* o compilador escrito em Capi puder utilizar Domains;
* testes de estresse e sanitizers forem aprovados.

---

## 17.201 Resultado Esperado

Ao final deste capítulo, a implementação oficial deverá possuir um modelo concreto no qual:

```text
Domain
    controla armazenamento e lifetime

Slot
    representa uma posição lógica reutilizável

Geração
    impede reutilização indevida

ObjectId
    representa identidade estável

Resolução
    materializa acesso temporário

Finalização
    encerra recursos deterministicamente

Destruição coletiva
    libera todo o grafo sem GC ou ARC
```

Esse modelo deverá permitir que objetos mantenham identidade mesmo quando participem de grafos cíclicos, sem depender de endereço físico como contrato público.

A implementação deverá permanecer suficientemente simples para auditoria e suficientemente abstrata para permitir otimizações futuras.

---

# 18. Modelo de Objetos, Layout e Despacho Dinâmico

## 18.1 Objetivo

Este capítulo define a estratégia inicial para materializar, na implementação oficial da Capi, o modelo de objetos descrito pela especificação da linguagem.

A primeira parte concentra-se em:

* representação física de objetos;
* distinção entre valores e objetos por identidade;
* cabeçalhos;
* layout de campos;
* alinhamento;
* herança;
* prefixo de classe base;
* metadados;
* relação com Domains;
* relação com `ObjectId`;
* inicialização física;
* acesso a campos;
* estabilidade de layout;
* preparação para despacho dinâmico.

As partes seguintes deste capítulo detalharão:

* vtables;
* tabelas de interfaces;
* chamadas virtuais;
* upcasts;
* downcasts;
* construção;
* destruição;
* evolução e otimização do modelo.

As representações descritas neste capítulo pertencem à implementação oficial inicial.

Elas poderão evoluir desde que preservem as garantias normativas da Capi.

---

## 18.2 Princípios do Modelo de Objetos

A implementação deverá preservar os seguintes princípios:

1. objetos de classe possuem identidade;
2. identidade não é definida pelo conteúdo;
3. identidade não é definida pelo endereço físico;
4. objetos pertencem a um Domain;
5. upcasts preservam a identidade;
6. herança não cria cópia do objeto base;
7. métodos virtuais operam sobre o mesmo objeto concreto;
8. campos herdados fazem parte do mesmo objeto;
9. destruição ocorre de forma determinística;
10. objetos de classe não são valores copiáveis implicitamente;
11. tipos por valor não devem receber identidade automaticamente;
12. detalhes de layout não são observáveis por código Capi comum.

---

## 18.3 Separação entre Semântica e Representação

A linguagem define:

* o que é um objeto;
* quando ele existe;
* qual é sua identidade;
* como herança e subtipagem funcionam;
* quais campos pertencem a cada classe;
* quais operações são permitidas;
* quando o objeto é destruído.

A implementação define:

* posição física dos campos;
* existência de cabeçalho;
* posição da vtable;
* tamanho de padding;
* formato dos metadados;
* estratégia de resolução;
* ordem interna das tabelas;
* forma de representação de referências temporárias.

Programas Capi não poderão depender de:

* offset numérico de campo;
* tamanho do cabeçalho;
* endereço da vtable;
* posição física do objeto;
* representação da classe base;
* padding;
* largura de ponteiros internos;
* ordem física de metadados;
* estratégia de despacho.

---

## 18.4 Categorias de Representação

A implementação deverá distinguir explicitamente:

```text
Tipos por valor
Objetos por identidade
Referências temporárias
ObjectId
Metadados de tipo
Handles externos
```

Cada categoria possui semântica diferente.

A representação não deverá utilizar um único tipo genérico de ponteiro para todos esses conceitos.

---

## 18.5 Tipos por Valor

Tipos por valor poderão ser armazenados:

* em registradores;
* na pilha;
* inline em outros valores;
* inline em objetos;
* em buffers;
* em memória temporária;
* em agregados retornados.

Exemplos:

* inteiros;
* booleanos;
* enums;
* structs;
* tuples;
* arrays de tamanho fixo;
* wrappers tipados;
* `Optional<T>`;
* `Result<T,E>`.

Tipos por valor não possuirão `ObjectId` apenas por existirem.

---

## 18.6 Objetos por Identidade

Instâncias de classes serão objetos por identidade.

Um objeto por identidade deverá possuir:

* associação a um Domain;
* slot;
* geração;
* metadados de tipo;
* armazenamento para campos;
* estado de inicialização;
* estado de finalização;
* identidade estável.

A forma lógica será:

```text
ObjectId<T>
        │
        ▼
slot do Domain
        │
        ▼
objeto físico
```

---

## 18.7 Ausência de Cópia Implícita de Objetos

Uma variável contendo identidade de objeto não armazenará uma cópia dos campos.

Exemplo conceitual:

```capi
let a = person;
let b = a;
```

Quando permitido pela semântica, `a` e `b` referem-se à mesma identidade.

Não será produzido:

```text
Objeto A
Objeto B com campos copiados
```

A representação deverá preservar essa propriedade.

---

## 18.8 Representações Possíveis de uma Referência de Objeto

A implementação poderá utilizar diferentes representações conforme o contexto.

### Representação persistente

```text
ObjectId<T>
```

Utilizada em:

* variáveis;
* campos;
* coleções;
* retornos;
* parâmetros duradouros;
* grafos de objetos.

### Representação temporária resolvida

```text
ObjectRef<T>
```

ou ponteiro interno equivalente.

Utilizada durante:

* leitura de campo;
* chamada de método;
* execução de destrutor;
* acesso autorizado;
* operações internas.

A representação temporária não deverá escapar do lifetime validado.

---

## 18.9 `ObjectId` como Representação Persistente

Campos que apontem para outros objetos deverão armazenar, preferencialmente:

```text
ObjectId<T>
```

ou representação lógica equivalente.

Isso permite:

* grafos cíclicos;
* validação de geração;
* preservação da identidade;
* independência do endereço;
* movimentação futura;
* diagnóstico;
* invalidação.

Ponteiros crus não deverão ser armazenados como referências persistentes por código Capi seguro.

---

## 18.10 Referência Resolvida Temporária

Após a resolução de um `ObjectId`, o Runtime poderá fornecer:

```text
pointer to object payload
```

ou:

```text
pointer to object header
```

A escolha deverá ser uniforme dentro da ABI.

O acesso temporário deverá estar vinculado:

* ao Domain;
* à validade do slot;
* ao lifetime do empréstimo;
* à autoridade de leitura ou mutação.

---

## 18.11 Representação Física Geral

A implementação inicial poderá utilizar:

```text
Object Allocation
├── ObjectHeader
└── ObjectPayload
    ├── campos herdados
    └── campos da classe concreta
```

Conceitualmente:

```text
┌──────────────────────────────┐
│ ObjectHeader                 │
├──────────────────────────────┤
│ Campos da classe base        │
├──────────────────────────────┤
│ Campos da classe derivada    │
└──────────────────────────────┘
```

Essa organização favorece herança por prefixo.

---

## 18.12 Cabeçalho Inline

Na estratégia de cabeçalho inline, o armazenamento será semelhante a:

```text
[header][payload]
```

O ponteiro armazenado no slot poderá apontar:

* para o início do cabeçalho; ou
* para o início do payload.

A convenção deverá ser única e documentada.

---

## 18.13 Ponteiro para Cabeçalho

Caso o slot aponte para o cabeçalho:

```text
slot.pointer → ObjectHeader
```

O acesso aos campos exigirá:

```text
payload = header + header_size
```

Vantagens:

* metadados acessíveis diretamente;
* destruição simples;
* validação centralizada;
* conversão uniforme.

Desvantagens:

* cálculo adicional para acessar payload;
* cabeçalho participa do alinhamento total.

---

## 18.14 Ponteiro para Payload

Caso o slot aponte para o payload:

```text
slot.pointer → primeiro campo
```

O cabeçalho poderá estar imediatamente antes.

Vantagens:

* acesso de campo mais direto;
* `self` pode apontar para payload.

Desvantagens:

* acesso ao cabeçalho exige offset negativo;
* maior risco de erros em FFI;
* convenção menos explícita.

A primeira implementação deverá priorizar clareza e segurança.

---

## 18.15 Estratégia Inicial Recomendada

Recomenda-se que o slot aponte para o início da alocação, isto é, para o cabeçalho.

Conceitualmente:

```text
slot.pointer
     │
     ▼
ObjectHeader
     │
     ▼
ObjectPayload
```

O Runtime poderá fornecer uma operação interna para obter o payload.

Essa estratégia facilita:

* validação;
* finalização;
* inspeção;
* obtenção de metadados;
* casts;
* debugging.

---

## 18.16 Estrutura Conceitual do Cabeçalho

O cabeçalho inicial poderá conter:

```text
ObjectHeader
├── type_metadata
├── state
├── flags
├── slot_index opcional
├── generation opcional
└── informações de debug opcionais
```

Nem todos os campos serão obrigatórios em release.

---

## 18.17 Metadados no Cabeçalho

O campo mais importante será uma referência para os metadados do tipo concreto:

```text
TypeMetadata*
```

Essa referência permitirá:

* localizar vtable;
* localizar destrutor;
* obter layout;
* verificar tipo concreto;
* realizar cast;
* resolver interfaces;
* produzir diagnóstico.

---

## 18.18 Slot e Geração no Cabeçalho

A implementação poderá duplicar no cabeçalho:

* índice do slot;
* geração;
* DomainId.

Isso poderá auxiliar:

* assertions;
* debug;
* validação reversa;
* inspeção.

Entretanto, essa duplicação aumenta overhead.

Na primeira versão, esses campos poderão existir apenas em builds de desenvolvimento.

---

## 18.19 Estado no Cabeçalho

O cabeçalho poderá registrar estados como:

```text
Initializing
Alive
Finalizing
Destroyed
```

Essa informação poderá duplicar o estado do slot.

Se houver duplicação, ambas as representações deverão permanecer consistentes.

Uma alternativa é manter o estado apenas no slot.

---

## 18.20 Cabeçalho Mínimo em Release

Uma representação mínima poderá conter apenas:

```text
ObjectHeader
└── type_metadata
```

Outras informações permaneceriam no slot.

Essa estratégia reduziria overhead por objeto.

Sua adoção deverá depender de medições e da facilidade de validação.

---

## 18.21 Cabeçalho Ampliado em Debug

Em desenvolvimento:

```text
ObjectHeader
├── magic
├── type_metadata
├── domain_id
├── slot_index
├── generation
├── state
├── allocation_site
└── flags
```

Esse formato auxiliará na identificação de:

* corrupção;
* ponteiro incorreto;
* acesso após destruição;
* tipo incompatível;
* associação ao Domain errado.

---

## 18.22 Tamanho do Cabeçalho

O tamanho deverá respeitar o alinhamento necessário ao payload.

O layout total deverá considerar:

```text
header_size
+ padding até alinhamento do payload
+ payload_size
+ padding final
```

O compilador ou Runtime deverá calcular essa composição de forma segura.

---

## 18.23 Descritor de Layout do Objeto

O backend deverá produzir ou acessar uma estrutura lógica semelhante a:

```text
ObjectLayout
├── header_size
├── header_alignment
├── payload_offset
├── payload_size
├── payload_alignment
├── allocation_size
└── allocation_alignment
```

Esses valores deverão ser calculados durante a compilação.

---

## 18.24 Layout do Payload

O payload conterá os campos de instância.

Exemplo:

```capi
class Person {
    var age: Int32;
    let active: Bool;
}
```

Representação conceitual:

```text
Person payload
├── age
├── active
└── padding
```

A ordem física poderá seguir a ordem de declaração, salvo regra normativa ou otimização autorizada.

---

## 18.25 Ordem dos Campos

A estratégia inicial deverá preservar a ordem de declaração.

Vantagens:

* simplicidade;
* diagnóstico;
* previsibilidade interna;
* facilidade de depuração;
* compatibilidade com herança prefixada;
* menor risco de inconsistência.

Reordenação automática poderá ser avaliada futuramente.

---

## 18.26 Reordenação de Campos

Reordenar campos pode reduzir padding.

Entretanto, isso pode afetar:

* ABI;
* debug;
* FFI;
* herança;
* metadados;
* estabilidade entre versões.

A primeira implementação não deverá reordenar campos de classes comuns.

Tipos explicitamente compatíveis com C deverão seguir regras próprias.

---

## 18.27 Cálculo de Offsets

Para cada campo:

```text
current_offset
        │
        ▼
align_up(current_offset, field_alignment)
        │
        ▼
field_offset
        │
        ▼
field_offset + field_size
```

O cálculo deverá verificar overflow.

---

## 18.28 Padding Interno

Padding poderá existir:

* entre o cabeçalho e o payload;
* entre campos;
* ao final da classe base;
* entre base e derivada;
* ao final do objeto.

Esse espaço não poderá ser observado por código seguro.

---

## 18.29 Padding e Inicialização

A implementação não precisará atribuir valor semântico ao padding.

Em debug, poderá preenchê-lo com padrão conhecido.

Não deverá ocorrer:

* comparação binária de objetos;
* hashing automático dos bytes totais;
* serialização bruta;
* exposição por FFI segura.

---

## 18.30 Alinhamento do Objeto

O alinhamento total deverá ser pelo menos o máximo entre:

* alinhamento do cabeçalho;
* alinhamento do payload;
* alinhamento de todos os campos;
* exigência da classe base.

Conceitualmente:

```text
object_alignment = max(header_alignment, payload_alignment)
```

---

## 18.31 Tamanho Total

O tamanho total deverá ser arredondado para o alinhamento do objeto.

Conceitualmente:

```text
allocation_size =
    align_up(payload_offset + payload_size, object_alignment)
```

---

## 18.32 Tipos de Tamanho Zero

Campos por valor de tamanho zero poderão não ocupar bytes.

Entretanto, deverão preservar:

* ordem semântica;
* destruição, se houver;
* identidade de localização quando necessária;
* regras de FFI.

Classes por identidade não deixarão de possuir alocação lógica apenas porque não possuem campos.

---

## 18.33 Classe Sem Campos

Uma classe vazia ainda possuirá:

* identidade;
* slot;
* Domain;
* metadados;
* possível cabeçalho;
* comportamento.

Exemplo:

```capi
class Marker {
}
```

Duas instâncias deverão possuir identidades diferentes.

---

## 18.34 Campos Estáticos

Campos estáticos não farão parte do payload da instância.

Eles deverão possuir armazenamento próprio.

Sua implementação deverá definir:

* módulo responsável;
* inicialização;
* mutabilidade;
* Domain;
* destruição;
* sincronização futura.

O primeiro subconjunto poderá restringi-los.

---

## 18.35 Constantes de Classe

Constantes puras poderão ser emitidas como dados estáticos.

Elas não precisarão de cabeçalho nem identidade.

---

## 18.36 Herança Simples

A implementação oficial inicial utilizará herança simples de classes, conforme a especificação.

Conceitualmente:

```text
Base
  │
  ▼
Derived
```

Uma classe derivada terá exatamente uma classe base direta.

Interfaces não participarão do layout como bases de estado.

---

## 18.37 Layout Prefixado

A estratégia inicial recomendada será o layout prefixado.

Exemplo:

```capi
class Animal {
    var age: Int32;
}

class Dog extends Animal {
    var weight: Int32;
}
```

Payload de `Dog`:

```text
┌──────────────────────────────┐
│ Animal.age                   │
├──────────────────────────────┤
│ padding da base, se houver   │
├──────────────────────────────┤
│ Dog.weight                   │
└──────────────────────────────┘
```

O payload da classe base será prefixo do payload derivado.

---

## 18.38 Vantagens do Layout Prefixado

O layout prefixado permite:

* upcast sem criação de novo objeto;
* acesso aos campos da base nos mesmos offsets;
* reutilização de métodos;
* ABI simples de receptor;
* preservação da identidade;
* implementação direta de herança simples.

---

## 18.39 Identidade Única na Herança

Um objeto derivado possui uma única identidade.

Não existirão identidades separadas para:

* subobjeto base;
* objeto derivado;
* cada interface.

Conceitualmente:

```text
ObjectId<Dog>
```

e um upcast para:

```text
ObjectId<Animal>
```

representam o mesmo objeto lógico.

---

## 18.40 Ausência de Objeto Base Separado

A criação de `Dog` não deverá criar:

```text
um objeto Animal
+
um objeto Dog
```

A classe base é uma região estrutural dentro do mesmo payload.

---

## 18.41 Offset da Base

Na estratégia inicial, o payload da classe base começará no mesmo offset do payload derivado.

Conceitualmente:

```text
derived_payload_pointer == base_payload_pointer
```

Isso torna o upcast estruturalmente simples.

---

## 18.42 Cabeçalho Único

O objeto derivado terá apenas um cabeçalho principal.

Não haverá um cabeçalho por nível de herança.

O cabeçalho apontará para os metadados do tipo concreto.

---

## 18.43 Metadados Concretos após Upcast

Mesmo quando o tipo estático for base:

```text
Animal
```

os metadados no objeto continuarão representando:

```text
Dog
```

Isso permite:

* despacho virtual;
* destrutor correto;
* cast;
* inspeção interna.

---

## 18.44 Tamanho da Classe Base

O layout da classe derivada deverá começar seus campos após o final reservado para a classe base.

Conceitualmente:

```text
derived_fields_start =
    align_up(base_payload_size, derived_field_alignment)
```

A classe base deverá possuir tamanho de payload estabilizado dentro da compilação.

---

## 18.45 Tail Padding da Classe Base

A implementação deverá decidir se campos derivados podem ocupar padding final da base.

### Reutilização de tail padding

Pode reduzir tamanho.

### Preservação do tail padding

Simplifica ABI e estabilidade.

A primeira implementação deverá preservar o tamanho completo da classe base antes de adicionar campos derivados.

---

## 18.46 Razão para Preservar Tail Padding

Preservar o tail padding evita:

* conflitos com ABI;
* mudanças quando a base evolui;
* dificuldade de depuração;
* regras complexas de layout;
* dependência entre módulos.

O custo de memória inicial é aceitável em favor de simplicidade.

---

## 18.47 Campos Privados da Base

Campos privados continuam fisicamente presentes no objeto derivado.

A visibilidade é regra do compilador, não mecanismo de ocultação física.

Código da classe derivada não poderá acessá-los diretamente quando proibido.

---

## 18.48 Campos com Mesmo Nome

Caso a linguagem permita ocultação de campos, cada campo terá offset próprio.

Entretanto, a primeira implementação deverá seguir as regras normativas e evitar ambiguidade.

A resolução deverá determinar exatamente qual declaração é acessada.

---

## 18.49 Herança Profunda

Para:

```text
A
└── B
    └── C
```

O payload de `C` será:

```text
[A fields][B fields][C fields]
```

Os campos de `A` manterão os mesmos offsets em:

* A;
* B;
* C.

---

## 18.50 Limite de Profundidade

A implementação não deverá definir limite sem necessidade.

Entretanto, o compilador deverá detectar:

* ciclos;
* hierarquia inválida;
* profundidade patológica, se necessária para defesa.

---

## 18.51 Ciclo de Herança

O resolver semântico deverá rejeitar:

```text
A extends B
B extends A
```

O cálculo de layout somente poderá ocorrer após validação acíclica.

---

## 18.52 Ordem de Cálculo de Layout

Os layouts deverão ser calculados em ordem topológica:

```text
Object
   │
   ▼
classes base
   │
   ▼
classes derivadas
```

Nenhuma classe derivada poderá ter layout final antes de sua base.

---

## 18.53 Layout Incompleto

Durante análise, uma classe poderá possuir estado:

```text
Declared
Resolving
LayoutPending
LayoutReady
Failed
```

Isso ajuda a detectar ciclos e dependências incompletas.

---

## 18.54 Tipos Recursivos por Identidade

Uma classe poderá referenciar a si mesma por `ObjectId`.

Exemplo:

```capi
class Node {
    var next: Optional<ObjectId<Node>>;
}
```

Isso possui tamanho finito.

---

## 18.55 Tipos Recursivos por Valor

Uma classe ou struct não poderá conter a si mesma diretamente por valor se isso produzir tamanho infinito.

Exemplo inválido:

```capi
struct Node {
    next: Node;
}
```

A menos que exista indireção explícita definida.

---

## 18.56 Campos de Classe

Um campo cujo tipo seja classe deverá armazenar identidade, não payload inline, salvo construção específica da linguagem.

Conceitualmente:

```capi
class Employee {
    var manager: ObjectId<Employee>;
}
```

Fisicamente:

```text
manager: ObjectId representation
```

---

## 18.57 Campos Opcionais de Objeto

Um campo opcional deverá utilizar:

```text
Optional<ObjectId<T>>
```

A implementação poderá otimizar a representação se existir valor inválido reservado.

Essa otimização não alterará a semântica.

---

## 18.58 Campos por Valor

Structs e outros valores poderão ser incorporados inline:

```capi
class Person {
    var address: Address;
}
```

Payload:

```text
Person
├── campos de Address inline
└── demais campos
```

A destruição de `Address` ocorrerá como parte de `Person`.

---

## 18.59 Campos Genéricos

Após monomorfização, o layout deverá utilizar o tipo concreto.

Exemplo:

```capi
class Box<T> {
    var value: T;
}
```

Instanciações:

```text
Box<Int32>
Box<String>
```

terão layouts potencialmente diferentes.

---

## 18.60 Classes Genéricas Monomorfizadas

Cada especialização concreta poderá possuir:

* tamanho próprio;
* alinhamento próprio;
* metadados próprios;
* vtable própria;
* destrutor próprio.

---

## 18.61 Compartilhamento de Código Genérico

Uma implementação futura poderá compartilhar código quando layouts forem compatíveis.

A primeira versão deverá priorizar monomorfização simples e correta.

---

## 18.62 Campos Imutáveis

A imutabilidade de um campo será garantida pelo compilador.

Fisicamente, o campo poderá residir em memória gravável.

O Runtime não precisará aplicar proteção de página.

---

## 18.63 Campos Mutáveis

O acesso de escrita deverá exigir a Capacidade de Mutação apropriada.

O layout não distingue obrigatoriamente campos mutáveis por representação.

A diferença estará nas operações permitidas.

---

## 18.64 Campos Voláteis ou Atômicos

Não deverão ser introduzidos implicitamente.

Se fizerem parte da linguagem futura, terão:

* tipos específicos;
* alinhamento;
* operações;
* semântica de memória.

---

## 18.65 Campos com Recursos

Um campo poderá conter recurso que exija destruição.

Exemplos:

* `String`;
* `Vector`;
* arquivo;
* buffer;
* handle;
* valor com destrutor.

Os metadados da classe deverão indicar que sua destruição não é trivial.

---

## 18.66 Bitmap de Campos

A implementação poderá manter metadados descrevendo quais campos:

* possuem destrutor;
* são identidades;
* são valores;
* exigem tracing de debug;
* possuem layout especial.

Não haverá tracing para Garbage Collection.

---

## 18.67 Descritores de Campo

Para debug ou reflexão interna mínima, poderá existir:

```text
FieldMetadata
├── name opcional
├── type_metadata
├── offset
├── flags
└── declaring_type
```

Esses dados não precisarão existir em release.

---

## 18.68 Reflection

A existência de descritores internos não significa reflection pública.

O compilador poderá omitir:

* nomes;
* visibilidade;
* anotações;
* estrutura completa.

Somente dados necessários ao Runtime serão obrigatórios.

---

## 18.69 `TypeMetadata`

Uma estrutura conceitual poderá conter:

```text
TypeMetadata
├── type_id
├── kind
├── size
├── alignment
├── payload_offset
├── base_type
├── destructor
├── vtable
├── interface_table
├── flags
└── debug_info
```

---

## 18.70 Campo `kind`

O tipo poderá ser classificado como:

```text
Primitive
Value
Enum
Class
Interface
Trait
Array
Function
Other
```

O Runtime deverá utilizar somente as categorias necessárias.

---

## 18.71 Metadados da Classe Base

Uma classe derivada deverá referenciar os metadados de sua base:

```text
DogMetadata.base_type → AnimalMetadata
```

Essa cadeia permitirá:

* validação de subtipo;
* downcast;
* diagnóstico;
* inspeção;
* construção de tabelas.

---

## 18.72 Identificador de Tipo

O `TypeId` deverá identificar a especialização concreta.

Exemplos distintos:

```text
Box<Int32>
Box<String>
```

deverão possuir `TypeId` distintos.

---

## 18.73 Geração de `TypeId`

A geração deverá ser determinística dentro do artefato.

Possibilidades:

* índice global;
* hash de nome canônico;
* símbolo interno;
* tabela de tipos.

Durante o bootstrap, um índice atribuído pelo compilador será suficiente.

---

## 18.74 Colisão de `TypeId`

Se hashes forem utilizados, colisões deverão ser tratadas.

A primeira implementação poderá evitar o problema utilizando índices únicos no artefato.

---

## 18.75 Tamanho e Alinhamento nos Metadados

Os metadados deverão registrar o tamanho necessário para alocação e destruição.

Deverá ficar claro se `size` representa:

* payload;
* alocação completa;
* valor inline.

Campos distintos são preferíveis.

---

## 18.76 Flags de Tipo

Flags possíveis:

```text
is_final
is_abstract
has_destructor
has_virtual_methods
has_interfaces
is_generic_instance
is_zero_sized
is_trivially_destructible
```

A implementação deverá evitar flags redundantes.

---

## 18.77 Metadados Emitidos pelo Backend

Os metadados poderão ser emitidos como constantes globais.

Conceitualmente:

```text
@Dog_type_metadata = constant ...
```

O Runtime receberá ponteiros para essas constantes.

---

## 18.78 Imutabilidade dos Metadados

Metadados de tipo deverão ser imutáveis após a inicialização.

Isso favorece:

* compartilhamento;
* segurança;
* otimização;
* leitura concorrente futura;
* armazenamento em segmento somente leitura.

---

## 18.79 Registro Dinâmico

A primeira implementação não precisará construir metadados dinamicamente para tipos comuns.

Tipos monomorfizados conhecidos serão emitidos estaticamente.

---

## 18.80 Linkedição de Metadados

O linker deverá resolver referências entre:

* metadados da derivada;
* metadados da base;
* vtables;
* destrutores;
* tabelas de interface.

A convenção de símbolos deverá ser determinística.

---

## 18.81 Nome Mangled dos Tipos

Cada tipo concreto deverá possuir nome interno exclusivo.

Exemplo conceitual:

```text
_Capi_Type_Dog
_Capi_Type_Box_Int32
```

A regra definitiva deverá ser definida pela ABI.

---

## 18.82 Visibilidade dos Metadados

Metadados utilizados apenas internamente poderão possuir visibilidade restrita.

Metadados necessários entre módulos deverão ser exportados conforme a linkedição.

---

## 18.83 Layout entre Módulos

Uma classe pública utilizada por múltiplos módulos deverá possuir layout consistente.

O compilador deverá garantir que todos os módulos consumam a mesma descrição.

Durante compilação monolítica, isso será direto.

Compilação separada exigirá metadados de interface.

---

## 18.84 Estabilidade de Layout

Antes da ABI pública estável, o layout poderá mudar entre versões do compilador.

Dentro do mesmo build, deverá ser consistente.

Após a estabilização, classes expostas por ABI poderão exigir regras adicionais.

---

## 18.85 Layout e FFI

Classes Capi comuns não deverão ser expostas diretamente como structs C.

Seu layout inclui:

* cabeçalho;
* identidade;
* metadados;
* regras de Domain.

A FFI deverá utilizar:

* handles;
* wrappers;
* tipos `repr(C)` específicos;
* funções de acesso.

---

## 18.86 Tipos Compatíveis com C

Se a linguagem oferecer tipos de layout C, eles deverão ser tipos por valor com regras explícitas.

Eles não deverão herdar automaticamente o modelo de classes.

---

## 18.87 Representação de `self`

Dentro de um método, `self` poderá ser representado como:

```text
resolved object pointer
```

acompanhado implicitamente ou explicitamente de:

* metadados;
* autoridade;
* Domain;
* tipo estático.

---

## 18.88 `self` em Método Leitor

Um método leitor deverá receber acesso que não permita mutação não autorizada.

Fisicamente, poderá ser o mesmo ponteiro.

A diferença será garantida pelo tipo da função e pela análise.

---

## 18.89 `self` em Método Mutante

Um método mutante deverá receber:

* ponteiro resolvido;
* prova ou token de Capacidade de Mutação;
* contexto necessário.

A ABI poderá passar a capacidade de forma:

* apagada após verificação;
* explícita;
* implícita no contrato interno.

A decisão será definida na implementação da Capacidade de Mutação.

---

## 18.90 Ponteiro de `self`

Recomenda-se que `self` aponte para o payload.

Isso facilita acesso aos campos por offset.

Entretanto, o Runtime poderá converter a partir do cabeçalho.

A ABI de métodos deverá adotar uma convenção única.

---

## 18.91 Alternativa: `self` Aponta para Cabeçalho

Se `self` apontar para o cabeçalho:

* metadados são acessados diretamente;
* campos exigem payload offset.

Essa abordagem também é válida.

A escolha deverá considerar uniformidade com vtables e destrutores.

---

## 18.92 Convenção Recomendada para Métodos

Uma convenção possível:

```text
method(self_payload_pointer, other_parameters...)
```

Os metadados concretos poderão ser obtidos por:

```text
header = self - payload_offset
```

Essa operação precisa ser segura e uniforme.

---

## 18.93 Convenção Alternativa Segura

Outra opção:

```text
method(object_pointer, other_parameters...)
```

Onde `object_pointer` aponta para o cabeçalho.

A rotina de acesso a campo calcula:

```text
payload = object_pointer + fixed_payload_offset
```

Essa forma reduz aritmética negativa.

Para o bootstrap, pode ser preferível.

---

## 18.94 Payload Offset Uniforme

Se todos os objetos utilizarem cabeçalho de tamanho e alinhamento uniformes, o payload offset poderá ser constante.

Isso simplifica:

* chamadas;
* acesso;
* casts;
* backend.

Entretanto, tipos com alinhamento elevado podem exigir padding variável.

---

## 18.95 Cabeçalho com Alinhamento Máximo

Uma estratégia seria alinhar o cabeçalho ao maior alinhamento suportado.

Isso reduz variação, mas aumenta desperdício.

A primeira implementação deverá calcular corretamente o offset por tipo.

---

## 18.96 Offset do Payload nos Metadados

O `TypeMetadata` poderá conter:

```text
payload_offset
```

Métodos e Runtime poderão utilizá-lo.

Para desempenho, o compilador conhecerá o valor estaticamente na maioria das chamadas.

---

## 18.97 Acesso Estático a Campo

Quando o tipo estático e o campo forem conhecidos, o backend emitirá:

```text
base_pointer + field_offset
```

Não será necessária chamada ao Runtime.

---

## 18.98 Acesso a Campo Herdado

O offset do campo da base será o mesmo em qualquer derivada.

Exemplo:

```text
Animal.age offset = 0
Dog.age offset = 0
```

considerando o início do payload.

---

## 18.99 Acesso após Upcast

Após upcast, o receptor continuará apontando para o mesmo objeto.

O acesso a campos da base utilizará os mesmos offsets.

Nenhum ajuste será necessário na herança simples com base no início do payload.

---

## 18.100 Acesso Dinâmico a Campo

A linguagem não deverá realizar acesso de campo por nome em Runtime.

A resolução ocorre em compilação.

Reflection futura poderá possuir API separada.

---

## 18.101 Escrita de Campo

A escrita deverá:

1. resolver o objeto;
2. validar autoridade de mutação;
3. calcular offset;
4. destruir valor anterior, quando necessário;
5. mover ou copiar o novo valor;
6. preservar invariantes.

---

## 18.102 Substituição de Campo Não Trivial

Para um campo com destrutor:

```text
destroy old value
move new value
```

A operação deverá tratar falha de construção do novo valor.

Uma estratégia segura será construir primeiro em temporário.

---

## 18.103 Autoatribuição

Operações de atribuição deverão lidar com:

* origem e destino iguais;
* alias permitido;
* movimento;
* cópia;
* destruição.

O compilador poderá detectar casos estáticos.

---

## 18.104 Inicialização Física de um Objeto

A criação física deverá seguir:

```text
obter metadados concretos
        │
        ▼
reservar slot
        │
        ▼
alocar header + payload
        │
        ▼
inicializar cabeçalho
        │
        ▼
inicializar parte da base
        │
        ▼
inicializar campos derivados
        │
        ▼
marcar objeto vivo
        │
        ▼
publicar ObjectId
```

---

## 18.105 Inicialização do Cabeçalho

Antes da execução do construtor, o cabeçalho deverá possuir ao menos:

* metadados concretos;
* estado `Initializing`;
* informações de slot necessárias.

O objeto ainda não estará publicamente acessível.

---

## 18.106 Inicialização da Classe Base

A parte da classe base deverá ser inicializada antes dos campos exclusivos da derivada, salvo regra normativa diferente.

Conceitualmente:

```text
Base initialization
        │
        ▼
Derived initialization
```

---

## 18.107 Metadados Concretos Durante Construção

Mesmo enquanto o construtor da base executa em um objeto derivado, o cabeçalho poderá conter metadados do tipo concreto.

Isso afeta chamadas virtuais durante construção.

A linguagem deverá definir se despacho virtual é permitido nesse estado.

A implementação não deverá decidir silenciosamente.

---

## 18.108 Chamadas Virtuais em Construtores

Existem riscos quando a base chama método sobrescrito antes da inicialização da derivada.

A especificação deverá:

* proibir;
* limitar;
* definir despacho não virtual;
* ou definir estado seguro.

A implementação inicial deverá adotar a opção normativa mais conservadora.

---

## 18.109 Inicialização de Campos

Cada campo deverá ser inicializado exatamente uma vez antes da publicação.

O compilador deverá validar:

* cobertura de todos os caminhos;
* ordem permitida;
* uso de `self`;
* leitura antes de inicialização;
* falha parcial.

---

## 18.110 Estado Parcial de Inicialização

Durante construção, o compilador deverá saber quais campos estão inicializados.

Isso poderá ser representado por análise de fluxo.

O Runtime não precisará manter bitmap permanente.

---

## 18.111 Cleanup de Construção Parcial

Se a construção falhar após inicializar alguns campos:

```text
destruir campos derivados inicializados
        │
        ▼
destruir campos da base inicializados
        │
        ▼
cancelar slot
        │
        ▼
invalidar alocação
```

A ordem deverá ser inversa à inicialização.

---

## 18.112 Memória do Objeto após Falha

Como o Domain utiliza alocação linear, a memória poderá permanecer reservada.

Entretanto:

* nenhum slot vivo apontará para ela;
* nenhum `ObjectId` terá sido publicado;
* recursos internos serão destruídos;
* o espaço será considerado perdido até o fim do Domain, salvo reutilização futura.

---

## 18.113 Publicação Tardia

A identidade deverá ser publicada apenas após o objeto estar completamente inicializado.

Isso impede:

* observação parcial;
* ciclo prematuro;
* método chamado antes da conclusão;
* armazenamento externo da identidade incompleta.

---

## 18.114 Construção de Grafos Cíclicos

Grafos cíclicos podem exigir referência entre objetos durante construção.

A linguagem deverá fornecer mecanismo seguro, como:

* fase de reserva;
* builders;
* inicialização em etapas controladas;
* campos opcionais;
* API específica de Domain.

O Runtime não deverá publicar objetos incompletos apenas para facilitar ciclos.

---

## 18.115 Placeholder de Identidade

Uma possível estratégia futura é reservar identidades antes da publicação.

Essas identidades seriam restritas ao contexto de construção.

Não poderão ser utilizadas como `ObjectId` comum até a conclusão.

---

## 18.116 Layout de Classes Abstratas

Classes abstratas possuem layout normal para sua parte de instância.

Elas não poderão ser instanciadas diretamente.

Classes derivadas incluirão seus campos.

---

## 18.117 Métodos Abstratos e Layout

Métodos abstratos não alteram o payload.

Eles afetam:

* vtable;
* validação;
* instanciação;
* metadados.

---

## 18.118 Classes `final`

Classes `final` utilizarão o mesmo modelo físico.

Entretanto, poderão permitir:

* chamada direta;
* metadados simplificados;
* ausência de subclasses;
* melhor devirtualização.

---

## 18.119 Classes `sealed`

Classes `sealed` também manterão layout comum.

A lista conhecida de subclasses poderá permitir:

* análise exaustiva;
* otimização de cast;
* despacho por conjunto fechado;
* devirtualização.

---

## 18.120 Interfaces sem Estado

Interfaces não deverão adicionar campos ao payload.

Um objeto implementa interfaces através de:

* metadados;
* tabelas;
* adaptadores de chamada.

Essa matéria será detalhada na Parte 6C.2.

---

## 18.121 Traits sem Estado

Traits também não deverão introduzir armazenamento próprio, salvo se a especificação definir mecanismo explícito.

Métodos padrão serão código, não subobjetos.

---

## 18.122 Composição

A composição será representada por campos.

Exemplo:

```capi
class Car {
    var engine: Engine;
}
```

Se `Engine` for valor:

```text
inline
```

Se for classe:

```text
ObjectId<Engine>
```

---

## 18.123 Delegação

Delegação é comportamento de chamadas e não altera automaticamente o layout.

Ela poderá ser implementada por campos e métodos gerados.

---

## 18.124 Encapsulamento Físico

A privacidade não será implementada por separação física.

Todos os campos permanecem no mesmo payload.

O compilador controla o acesso.

---

## 18.125 Segurança contra Acesso por Offset Forjado

Código Capi seguro não poderá calcular offsets arbitrários.

Somente:

* backend;
* Runtime;
* FFI insegura;

poderão manipular o layout físico.

---

## 18.126 Layout de Arrays de Objetos

Uma coleção de objetos de classe armazenará normalmente:

```text
ObjectId<T>
ObjectId<T>
ObjectId<T>
```

e não payloads completos inline.

Isso preserva:

* identidade;
* polimorfismo;
* tamanho uniforme;
* estabilidade.

---

## 18.127 Coleção de Valores

Uma coleção de structs armazenará valores inline quando possível.

Exemplo:

```text
Vector<Point>
```

poderá conter:

```text
[Point][Point][Point]
```

---

## 18.128 Coleção Polimórfica

Uma coleção de tipo base:

```text
Vector<ObjectId<Animal>>
```

poderá armazenar identidades de:

* `Dog`;
* `Cat`;
* outras subclasses.

Cada identidade resolverá para objeto concreto.

---

## 18.129 Tamanho Uniforme de Identidades

O tamanho de `ObjectId<T>` deverá ser independente de `T`.

Isso facilita coleções polimórficas.

---

## 18.130 Nullable Interno

A implementação poderá reservar um padrão inválido para otimizar:

```text
Optional<ObjectId<T>>
```

Exemplo:

```text
DomainId = 0
```

Essa otimização deverá ser invisível à linguagem.

---

## 18.131 Nichos de Representação

O compilador poderá utilizar valores impossíveis para compactar:

* `Optional`;
* enums;
* referências;
* handles.

Essas otimizações deverão ser documentadas na ABI interna.

---

## 18.132 Layout de Enums com Objetos

Um enum como:

```text
Optional<ObjectId<T>>
```

poderá utilizar:

* tag explícita;
* niche;
* representação combinada.

A escolha não altera o layout do objeto referido.

---

## 18.133 Objetos e Pilha

A especificação define identidade e Domain.

A implementação inicial alocará objetos de classe no armazenamento do Domain.

Futuramente, escape analysis poderá permitir alocação física na pilha quando a identidade for preservável.

---

## 18.134 Stack Allocation de Objetos

Uma otimização futura poderá alocar fisicamente um objeto na pilha se:

* não escapar;
* não participar de identidade duradoura;
* `ObjectId` não for observado externamente;
* lifetime for lexical;
* despacho permitir;
* destruição for conhecida.

Essa otimização não deverá mudar a semântica.

---

## 18.135 Scalar Replacement

Um objeto poderá ser decomposto em valores escalares quando:

* sua identidade não for observável;
* seus campos forem conhecidos;
* não houver FFI;
* não houver despacho dinâmico relevante;
* não houver finalizador observável.

---

## 18.136 Identidade e Otimização

Dois objetos distintos nunca poderão ser fundidos se a identidade puder ser comparada.

Exemplo:

```capi
let a = new Item();
let b = new Item();

assert(a.id != b.id);
```

A otimização deverá preservar o resultado.

---

## 18.137 Eliminação de Objeto Não Observável

Um objeto poderá ser eliminado se:

* não escapa;
* nenhum efeito é observado;
* identidade não é comparada;
* construtor e destrutor não possuem efeitos;
* campos não são usados.

---

## 18.138 Layout de Fechamentos

Se closures forem objetos por identidade ou valores, deverão possuir representação específica.

Isso não será definido automaticamente pelo modelo de classes.

O primeiro bootstrap poderá restringir closures.

---

## 18.139 Layout de Objetos de Biblioteca

Tipos como:

* `String`;
* `Vector`;
* `Map`;

poderão ser:

* structs por valor;
* classes;
* wrappers sobre recursos;
* combinações.

Sua categoria deverá ser definida pela Biblioteca Padrão, não inferida pelo nome.

---

## 18.140 `String` como Valor

Uma possível representação:

```text
String
├── pointer
├── length
├── capacity ou metadata
└── ownership
```

Isso não implica identidade de classe.

---

## 18.141 Objetos Externos

Handles de FFI poderão ser envolvidos por classes Capi.

O payload armazenará:

* handle opaco;
* estado;
* metadados necessários.

A identidade Capi será distinta da identidade externa.

---

## 18.142 Pinning

A primeira implementação não moverá objetos, tornando-os fisicamente estáveis dentro do Domain.

Mesmo assim, o conceito público de pinning não deverá ser introduzido sem necessidade.

---

## 18.143 Ponteiros Internos para Campos

Código seguro não deverá armazenar ponteiro persistente para campo de objeto.

Empréstimos temporários poderão existir.

Isso preserva possibilidade de movimentação futura.

---

## 18.144 Self-References Físicas

Um objeto não deverá armazenar ponteiro cru para seu próprio campo em código seguro.

Deverá utilizar:

* offsets;
* `ObjectId`;
* estrutura segura;
* mecanismo específico.

---

## 18.145 Layout e Mutação

A mutação não altera o layout.

Entretanto, operações mutantes devem garantir:

* acesso exclusivo conforme regra;
* destruição adequada do valor anterior;
* atualização consistente;
* ausência de observação intermediária inválida.

---

## 18.146 Estado Invariante do Objeto

Após cada operação pública, o objeto deverá satisfazer seus invariantes.

Durante método mutante, um estado intermediário poderá existir sob autoridade exclusiva.

Exceções e retornos de erro deverão preservar consistência.

---

## 18.147 Falha em Método Mutante

Se uma operação falhar após modificar parcialmente:

* o método deverá restaurar estado; ou
* deixar estado válido documentado; ou
* consumir o objeto; ou
* tratar como falha fatal.

A linguagem não fornecerá rollback automático.

---

## 18.148 Zero Initialization

A memória poderá ser zerada pelo Runtime por conveniência.

Entretanto, um objeto não estará inicializado apenas por estar zerado.

Campos como:

* `ObjectId`;
* enums;
* recursos;
* strings;

podem não aceitar zero como valor válido.

---

## 18.149 Memória Não Inicializada

A implementação poderá usar memória não inicializada para desempenho.

O compilador deverá garantir que nenhum campo seja lido antes da inicialização.

O uso de `unsafe` deverá permanecer no Runtime e backend.

---

## 18.150 Representação de Booleanos em Campos

`Bool` poderá ocupar um byte.

A ABI deverá impedir valores inválidos observáveis.

O backend poderá normalizar escritas.

---

## 18.151 Packing de Booleanos

Compactar múltiplos booleanos em bits poderá reduzir tamanho, mas complicar:

* acesso;
* mutação;
* borrowing;
* ABI.

Não será utilizado inicialmente.

---

## 18.152 Layout de Enums em Campos

Enums deverão possuir layout calculado a partir de:

* tag;
* maior payload;
* alinhamento;
* niches.

A classe apenas incorpora o valor resultante.

---

## 18.153 Objetos e SIMD

Campos SIMD poderão exigir alinhamento elevado.

O alocador de Domain deverá suportar o alinhamento calculado.

Não será requisito do bootstrap inicial, mas o cálculo não deverá assumir alinhamento máximo baixo.

---

## 18.154 Limite de Alinhamento

O Runtime deverá definir suporte para alinhamentos compatíveis com o target.

Alinhamentos extraordinários poderão exigir bloco dedicado.

---

## 18.155 Objetos Excessivamente Grandes

O compilador ou Runtime poderá rejeitar layouts cujo tamanho:

* exceda o espaço de endereçamento;
* provoque overflow;
* exceda limite de ABI;
* seja impossível de alocar.

O diagnóstico deverá apontar o tipo.

---

## 18.156 Layout Recursivo

O cálculo deverá detectar ciclos por valor.

Uma cadeia como:

```text
A contains B
B contains A
```

é inválida se ambos forem valores inline.

Referências por `ObjectId` quebram o ciclo físico.

---

## 18.157 Cache de Layouts

O compilador deverá armazenar o layout calculado por tipo.

Isso evita recomputação.

O cache deverá lidar com estados de resolução.

---

## 18.158 Chave do Cache

A chave deverá identificar:

* tipo concreto;
* argumentos genéricos;
* target;
* ABI;
* opções de layout relevantes.

---

## 18.159 Layout Dependente do Target

Tamanho e alinhamento poderão variar entre:

* x86-64;
* ARM64;
* WebAssembly;
* outros.

A semântica não poderá depender desses valores.

---

## 18.160 Data Layout do Backend

O cálculo deverá utilizar uma descrição central do target.

Não deverá duplicar regras entre:

* Frontend;
* Cranelift;
* LLVM;
* Runtime.

---

## 18.161 Fonte de Verdade do Layout

O compilador deverá possuir um módulo de layout independente do backend.

Esse módulo deverá produzir descrições consumidas por:

* IR;
* Cranelift;
* LLVM;
* Runtime;
* debug;
* testes.

---

## 18.162 Validação pelo Backend

O backend poderá verificar se os tipos gerados correspondem ao layout calculado.

Divergências deverão ser tratadas como erro interno.

---

## 18.163 Testes de Layout

Deverão cobrir:

* classe vazia;
* um campo;
* campos com alinhamentos diferentes;
* herança;
* herança profunda;
* structs inline;
* enums;
* generics;
* objetos grandes;
* alinhamento elevado;
* classes abstratas;
* classes `final`.

---

## 18.164 Golden Tests de Layout

O compilador poderá gerar dumps como:

```text
class Dog
allocation size: 32
alignment: 8
payload offset: 8
fields:
  Animal.age @ 0
  Dog.weight @ 8
```

Esses dumps poderão ser comparados em golden tests.

---

## 18.165 Testes de Upcast Estrutural

Um teste deverá confirmar que:

```text
payload pointer de Dog
==
payload pointer após upcast para Animal
```

na implementação prefixada.

A identidade também deverá permanecer igual.

---

## 18.166 Testes de Campo Herdado

Escrever um campo herdado através da derivada e ler através da base deverá observar o mesmo valor, respeitando mutabilidade.

---

## 18.167 Testes de Identidade

Exemplo:

```text
Dog d
Animal a = upcast(d)

identity(d) == identity(a)
```

deverá ser verdadeiro.

---

## 18.168 Testes de Metadados

Após upcast, os metadados concretos deverão continuar apontando para `Dog`.

---

## 18.169 Testes de Alinhamento

Cada campo deverá estar em endereço compatível com seu alinhamento.

Os testes deverão utilizar diferentes combinações.

---

## 18.170 Testes de Construção Parcial

Deverão validar:

* falha na base;
* falha no primeiro campo derivado;
* falha no último campo;
* cleanup;
* ausência de publicação;
* destruição única.

---

## 18.171 Testes com Sanitizers

O layout deverá ser testado para detectar:

* out-of-bounds;
* desalinhamento;
* leitura não inicializada;
* use-after-free;
* offset incorreto.

---

## 18.172 Testes Diferenciais Cranelift/LLVM

Ambos os backends deverão produzir comportamento equivalente.

O dump de layout deverá ser comum.

---

## 18.173 Fuzzing de Layout

O compilador poderá gerar combinações aleatórias de:

* campos;
* herança;
* alinhamentos;
* tipos genéricos;
* arrays.

As propriedades deverão incluir:

* ausência de sobreposição;
* tamanho suficiente;
* alinhamento válido;
* prefixo da base preservado.

---

## 18.174 Propriedades Formais do Layout

Para cada campo:

```text
field_offset % field_alignment == 0
```

Para cada par de campos distintos:

```text
ranges do not overlap
```

Para a base:

```text
base_layout is prefix of derived_layout
```

Para o objeto:

```text
allocation_size % allocation_alignment == 0
```

---

## 18.175 Overlap Permitido

Somente tipos de tamanho zero ou otimizações explicitamente definidas poderão compartilhar endereço.

A primeira implementação poderá atribuir posições distintas mesmo a tipos de tamanho zero para simplificar debug.

---

## 18.176 Instrumentação de Layout

Em debug, o Runtime poderá validar:

* metadados;
* tamanho;
* alinhamento;
* offsets;
* tipo concreto;
* cabeçalho.

---

## 18.177 Dump de Objeto

Uma ferramenta interna poderá apresentar:

```text
Object
├── ObjectId
├── concrete type
├── Domain
├── header
├── payload size
└── fields
```

A leitura dos campos dependerá de metadados de debug.

---

## 18.178 Privacidade em Dumps

Ferramentas de debug poderão mostrar campos privados apenas em contexto autorizado.

Essa capacidade não constitui reflection comum.

---

## 18.179 Otimização de Cabeçalho

Possíveis evoluções:

* remover estado duplicado;
* mover metadados para slot;
* compactar flags;
* usar vtable como metadado principal;
* separar debug info;
* compartilhar cabeçalho por bloco.

---

## 18.180 Metadados no Slot

Uma estratégia futura poderá manter:

```text
Slot
├── pointer
├── type_metadata
└── generation
```

e eliminar `type_metadata` do cabeçalho.

Vantagem:

* menor objeto.

Desvantagem:

* método com ponteiro não obtém tipo diretamente;
* FFI e destruição dependem do slot;
* chamadas virtuais precisam de contexto.

---

## 18.181 Vtable no Cabeçalho

Outra estratégia seria armazenar diretamente:

```text
vtable_pointer
```

no cabeçalho.

Isso torna chamada virtual rápida.

Entretanto, metadados adicionais ainda poderão ser necessários.

A decisão será detalhada na Parte 6C.2.

---

## 18.182 Cabeçalho Compartilhado por Tipo

Não é possível compartilhar integralmente o cabeçalho entre objetos porque o objeto pode exigir estado próprio.

Mas metadados e vtable serão compartilhados.

---

## 18.183 Headerless Objects

Classes `final` sem virtualização poderiam, futuramente, omitir cabeçalho e depender do slot.

Essa otimização aumentaria complexidade.

Não será utilizada inicialmente.

---

## 18.184 Object Layout ABI

A ABI interna deverá definir:

* posição lógica do cabeçalho;
* forma de obter payload;
* formato mínimo dos metadados;
* assinatura de destrutores;
* representação de `self`.

Esses contratos deverão ser compartilhados pelos backends.

---

## 18.185 Versionamento do Layout

O Runtime e o compilador deverão concordar sobre uma versão de layout.

Mudanças incompatíveis deverão alterar a versão da ABI interna.

---

## 18.186 Compatibilidade entre Artefatos

Arquivos objeto produzidos por compiladores com layouts incompatíveis não deverão ser vinculados silenciosamente.

A Toolchain poderá emitir símbolo ou metadado de versão.

---

## 18.187 Compilação Incremental Futura

Caches deverão incluir a versão de layout.

Uma alteração em:

* base;
* campos;
* tipo de campo;
* alinhamento;
* generics;

deverá invalidar dependentes.

---

## 18.188 Hash de Layout

O compilador poderá produzir hash contendo:

* tamanho;
* alinhamento;
* offsets;
* base;
* metadados relevantes.

Esse hash poderá auxiliar compatibilidade.

---

## 18.189 Evolução de Classes Públicas

Após estabilidade de ABI, alterar campos de classe pública poderá quebrar compatibilidade binária.

A política será definida futuramente.

Durante o bootstrap, recompilação completa será aceitável.

---

## 18.190 Relação com Hot Reload

Hot reload exigiria compatibilidade de layout entre versões ativas.

Não fará parte da implementação inicial.

---

## 18.191 Relação com Serialização

Serialização não deverá usar bytes brutos do payload.

Ela deverá operar por:

* campos;
* esquema;
* APIs;
* metadados específicos.

Padding e cabeçalho não são dados serializáveis.

---

## 18.192 Relação com Hashing e Igualdade

Objetos por identidade utilizarão igualdade de identidade por padrão, conforme a especificação.

Não deverá ser utilizada comparação binária do payload.

Igualdade estrutural deverá ser método explícito ou trait.

---

## 18.193 Clone de Objetos

Se a linguagem oferecer clonagem, ela deverá criar novo objeto com nova identidade.

A cópia de campos não preservará `ObjectId`.

---

## 18.194 Movimento de Identidade

Mover uma variável contendo `ObjectId` não move o objeto.

Apenas transfere ou copia o valor da identidade conforme a semântica.

---

## 18.195 Clonagem e Grafos

Clonar grafos exigirá política sobre:

* objetos compartilhados;
* ciclos;
* identidades;
* Domains.

Não será operação automática do Runtime.

---

## 18.196 ADRs Recomendados

Deverão ser criados ADRs para:

```text
ADR — Separação entre ObjectId e ponteiro resolvido
ADR — Cabeçalho inline inicial
ADR — Slot apontando para o cabeçalho
ADR — Layout prefixado de herança
ADR — Preservação do tail padding da base
ADR — Ordem de declaração dos campos
ADR — Metadados estáticos por tipo concreto
ADR — Publicação tardia de objetos
ADR — Ausência inicial de movimentação física
ADR — Convenção inicial de self
```

---

## 18.197 Invariantes do Layout

A implementação deverá documentar, no mínimo:

1. todo objeto de classe possui uma identidade;
2. todo objeto vivo possui um tipo concreto;
3. o cabeçalho pertence à mesma alocação do payload, quando inline;
4. o payload respeita o alinhamento calculado;
5. cada campo ocupa intervalo válido;
6. campos distintos não se sobrepõem, salvo regra explícita;
7. a parte da base é prefixo da derivada;
8. upcast não cria novo objeto;
9. upcast não altera a identidade;
10. metadados representam o tipo concreto;
11. objetos incompletos não são publicados;
12. endereço físico não é identidade pública.

---

## 18.198 Critérios de Conclusão do Layout Básico

O layout básico estará concluído quando:

* classes sem herança puderem ser alocadas;
* cabeçalho e payload forem calculados;
* campos puderem ser inicializados;
* campos puderem ser lidos;
* campos mutáveis puderem ser escritos sob autoridade;
* objetos puderem ser registrados no Domain;
* `ObjectId` puder ser resolvido;
* metadados concretos estiverem associados;
* destruição estrutural puder localizar campos;
* testes de alinhamento passarem.

---

## 18.199 Critérios de Conclusão da Herança Física

A herança física estará concluída quando:

* layouts de bases forem calculados antes das derivadas;
* campos herdados preservarem offsets;
* campos derivados forem adicionados após a base;
* upcast preservar o ponteiro lógico;
* upcast preservar a identidade;
* metadados concretos permanecerem disponíveis;
* herança profunda funcionar;
* ciclos forem rejeitados;
* testes diferenciais passarem.

---

## 18.200 Resultado Esperado da Primeira Parte

Ao final desta etapa, a implementação oficial deverá possuir uma representação concreta em que:

```text
ObjectId
    representa identidade persistente

Slot
    localiza o armazenamento atual

ObjectHeader
    associa o objeto ao tipo concreto

ObjectPayload
    contém campos da base e da derivada

Layout prefixado
    permite upcast sem cópia

TypeMetadata
    descreve tamanho, alinhamento e comportamento

Domain
    controla lifetime e destruição
```

Essa base deverá permitir implementar, na etapa seguinte:

* vtables;
* override;
* despacho dinâmico;
* referências de interface;
* casts;
* devirtualização;
* chamadas polimórficas.

O modelo físico deverá permanecer simples, auditável e compatível com a evolução futura da Toolchain.

---

## Segunda Parte — VTables, Interfaces, Casts e Despacho Dinâmico

## 18.201 Objetivo

Esta parte define a estratégia inicial para materializar, na implementação oficial da Capi:

* métodos virtuais;
* sobrescrita;
* vtables;
* slots virtuais;
* chamadas polimórficas;
* interfaces;
* tabelas de implementação;
* referências por interface;
* upcasts;
* downcasts;
* verificação dinâmica de tipo;
* RTTI mínimo;
* chamadas a métodos padrão;
* devirtualização;
* integração com Cranelift e LLVM.

A implementação deverá preservar a semântica definida pela linguagem sem transformar a disposição física das tabelas em contrato observável por código Capi comum.

---

## 18.202 Princípios do Despacho Dinâmico

A implementação deverá preservar os seguintes princípios:

1. o tipo estático determina quais operações podem ser solicitadas;
2. o tipo concreto determina qual implementação virtual será executada;
3. sobrescrita não altera a identidade do objeto;
4. upcast não cria novo objeto;
5. a classe derivada preserva os slots virtuais herdados;
6. métodos não virtuais não deverão pagar custo de despacho dinâmico;
7. interfaces não introduzem estado adicional no objeto;
8. uma mesma instância poderá ser observada por diferentes tipos estáticos;
9. o destrutor correto deverá ser selecionado pelo tipo concreto;
10. verificações dinâmicas deverão ser eliminadas quando o compilador puder prová-las desnecessárias;
11. detalhes de vtable e RTTI não deverão constituir reflection pública ampla.

---

## 18.203 Tipos de Chamada de Método

A implementação deverá distinguir pelo menos:

```text
chamada estática
chamada virtual
chamada por interface
chamada direta ao método da base
chamada de método final
chamada de método abstrato resolvida
chamada intrínseca
```

Cada categoria poderá utilizar lowering diferente.

---

## 18.204 Chamada Estática

Uma chamada será estática quando a implementação for conhecida em compilação.

Exemplos:

* função livre;
* método `static`;
* método privado não virtual;
* método de classe `final`;
* método marcado como não sobrescrevível;
* chamada explicitamente qualificada à implementação da base;
* chamada devirtualizada.

Conceitualmente:

```text
call Dog::run(self, arguments)
```

Não será necessário consultar vtable.

---

## 18.205 Chamada Virtual

Uma chamada será virtual quando:

* o método puder ser sobrescrito;
* o receptor possuir tipo estático que não determina sozinho a implementação;
* a análise não puder devirtualizar com segurança.

Conceitualmente:

```text
receiver
    │
    ▼
metadados concretos
    │
    ▼
vtable
    │
    ▼
slot do método
    │
    ▼
função concreta
```

---

## 18.206 Chamada por Interface

Uma chamada por interface deverá localizar a implementação da interface correspondente ao tipo concreto.

Conceitualmente:

```text
interface reference
        │
        ├── object identity ou object pointer
        └── interface implementation table
                    │
                    ▼
             method slot
                    │
                    ▼
             concrete function
```

A representação exata será definida adiante.

---

## 18.207 Método Virtual

Um método virtual será uma operação cuja implementação poderá variar conforme o tipo concreto do receptor.

A linguagem deverá determinar:

* se métodos são virtuais por padrão;
* se exigem modificador;
* quais métodos podem ser sobrescritos;
* quais métodos são `final`;
* quais métodos são abstratos.

A implementação apenas materializará essas decisões.

---

## 18.208 Método Abstrato

Um método abstrato:

* possui assinatura;
* ocupa posição lógica na hierarquia;
* não possui implementação concreta na classe abstrata, salvo mecanismo específico;
* deverá ser implementado antes da instanciação de classe concreta.

Nenhum objeto concreto poderá possuir slot virtual não resolvido.

---

## 18.209 Método `final`

Um método `final` não poderá ser sobrescrito.

A implementação poderá:

* omiti-lo da vtable, se nunca precisar de slot herdado;
* manter o slot por compatibilidade interna;
* chamá-lo diretamente;
* devirtualizá-lo sempre.

A escolha deverá preservar a ABI interna.

---

## 18.210 Override

Uma sobrescrita deverá corresponder a um método herdado compatível.

O compilador deverá validar:

* nome;
* parâmetros;
* retorno;
* visibilidade;
* mutabilidade;
* efeitos;
* genericidade;
* exceções ou erros, se aplicável;
* regras de covariância e contravariância;
* modificador `override`.

A implementação não deverá criar novo slot quando se tratar do mesmo método virtual herdado.

---

## 18.211 Identidade Lógica do Método Virtual

Cada método virtual deverá possuir uma identidade lógica independente da implementação concreta.

Exemplo:

```text
Animal.speak
```

é a operação virtual.

Implementações:

```text
Animal::speak
Dog::speak
Cat::speak
```

ocupam o mesmo slot lógico na hierarquia.

---

## 18.212 Slot Virtual

Um slot virtual é uma posição estável dentro da vtable de uma hierarquia.

Exemplo:

```text
slot 0 → destructor
slot 1 → speak
slot 2 → move
slot 3 → name
```

Classes derivadas poderão substituir o ponteiro de um slot herdado.

---

## 18.213 Estabilidade dos Slots Herdados

Uma vez atribuído um slot na classe base, esse slot deverá manter a mesma posição nas classes derivadas.

Exemplo:

```text
Animal.vtable[1] → Animal::speak
Dog.vtable[1]    → Dog::speak
Cat.vtable[1]    → Cat::speak
```

Isso permite chamadas virtuais através do tipo base.

---

## 18.214 Inclusão de Novos Métodos Virtuais

Quando uma classe derivada introduzir novo método virtual:

* os slots herdados permanecerão;
* o novo método receberá posição após os slots existentes, na estratégia inicial.

Exemplo:

```text
Animal:
  0 destructor
  1 speak

Dog:
  0 destructor
  1 speak
  2 fetch
```

---

## 18.215 Ordem de Atribuição dos Slots

A estratégia inicial deverá utilizar ordem determinística.

Uma possibilidade:

1. slots reservados do Runtime;
2. métodos virtuais herdados;
3. novos métodos virtuais na ordem de declaração.

Essa regra deverá ser aplicada uniformemente.

---

## 18.216 Slots Reservados

A vtable poderá reservar slots para:

* destruição;
* obtenção de metadados;
* operações internas;
* funções de cast;
* operações de debug.

Entretanto, deverão ser reservados apenas quando necessários.

A primeira implementação poderá manter destrutor fora da sequência comum se `TypeMetadata` já possuir ponteiro próprio.

---

## 18.217 VTable Conceitual

Uma vtable inicial poderá ser representada como:

```text
VTable
├── type_metadata
├── method_0
├── method_1
├── method_2
└── ...
```

Alternativamente, os metadados poderão apontar para a vtable:

```text
TypeMetadata
└── vtable
```

A segunda forma evita duplicação.

---

## 18.218 Estratégia Inicial Recomendada

Recomenda-se:

```text
ObjectHeader
└── TypeMetadata*

TypeMetadata
├── destructor
├── vtable
├── base_type
└── interface_table
```

Assim, o objeto mantém apenas referência aos metadados concretos.

---

## 18.219 Estrutura Física da VTable

Fisicamente, uma vtable poderá ser um array imutável de ponteiros de função.

Conceitualmente:

```text
VTableEntry[] = {
    function_pointer_0,
    function_pointer_1,
    function_pointer_2
}
```

A assinatura exata de cada slot será conhecida pelo compilador.

---

## 18.220 Heterogeneidade das Assinaturas

Métodos diferentes possuem assinaturas distintas.

Uma vtable física poderá armazenar ponteiros opacos de tamanho uniforme.

O backend deverá converter cada entrada para a assinatura correta antes da chamada.

Essa conversão será interna e controlada.

---

## 18.221 Segurança de Assinaturas

O compilador deverá garantir que:

* um slot sempre recebe função de assinatura ABI compatível;
* a sobrescrita preserva a convenção de chamada;
* o receptor possui representação compatível;
* o retorno é representado corretamente;
* parâmetros ocultos permanecem consistentes.

Uma incompatibilidade será erro interno do compilador.

---

## 18.222 ABI de Método Virtual

Uma assinatura conceitual poderá ser:

```text
method(
    self_object_pointer,
    explicit_arguments...
) -> return_value
```

Parâmetros ocultos adicionais poderão incluir:

* RuntimeContext;
* Domain;
* Capacidade de Mutação;
* informações genéricas;
* buffer de retorno.

A ABI deverá ser uniforme entre base e override.

---

## 18.223 Receptor em Overrides

Um override de `Dog::speak` poderá ser declarado semanticamente com receptor `Dog`.

Entretanto, o slot herdado pode ser chamado por referência estática `Animal`.

A ABI física deverá aceitar o mesmo ponteiro de objeto.

Como o layout de `Animal` é prefixo de `Dog`, o mesmo endereço lógico será válido.

---

## 18.224 Thunks de Ajuste

Na herança simples prefixada, normalmente não será necessário ajustar o ponteiro de `self`.

Mesmo assim, thunks poderão ser necessários para:

* covariância de retorno;
* adaptação de ABI;
* generics;
* interfaces;
* métodos padrão;
* futuras formas de herança.

---

## 18.225 Thunk

Um thunk é uma pequena função intermediária que adapta uma chamada.

Conceitualmente:

```text
caller ABI
    │
    ▼
thunk
    ├── ajusta self
    ├── ajusta parâmetros
    ├── chama implementação
    └── ajusta retorno
```

Thunks deverão ser gerados somente quando necessários.

---

## 18.226 Construção da VTable da Classe Base

Para uma classe base:

1. enumerar métodos virtuais;
2. validar métodos abstratos;
3. atribuir slots;
4. associar implementações disponíveis;
5. marcar slots abstratos;
6. emitir tabela.

Classes abstratas poderão possuir entradas especiais para métodos abstratos.

---

## 18.227 Entrada Abstrata

Uma entrada abstrata não deverá ser chamada em objeto concreto.

Possíveis representações:

* ponteiro nulo;
* função de falha fatal;
* símbolo especial.

A primeira implementação deverá preferir uma função de falha fatal em builds de desenvolvimento.

Classes concretas deverão substituir todas as entradas abstratas obrigatórias.

---

## 18.228 Construção da VTable Derivada

Para uma derivada:

1. copiar a estrutura lógica da base;
2. substituir slots sobrescritos;
3. adicionar novos slots;
4. validar métodos abstratos restantes;
5. emitir tabela própria.

Cada classe concreta terá vtable própria quando possuir despacho virtual.

---

## 18.229 Compartilhamento de VTables

Classes sem diferenças virtuais poderiam compartilhar tabelas.

Essa otimização não será necessária inicialmente.

A emissão de uma vtable por tipo concreto simplifica:

* metadados;
* cast;
* debug;
* destruição.

---

## 18.230 VTable de Classe `final`

Uma classe `final` ainda poderá possuir vtable se:

* herdar métodos virtuais;
* implementar interfaces;
* possuir referências através de base.

Entretanto, chamadas com tipo concreto final poderão ser diretas.

---

## 18.231 Classes sem Métodos Virtuais

Uma classe sem métodos virtuais poderá ter:

```text
vtable = null
```

ou tabela vazia.

Os metadados ainda existirão se forem necessários para:

* destruição;
* cast;
* interfaces;
* diagnóstico.

---

## 18.232 Localização da VTable

A sequência inicial de chamada virtual poderá ser:

```text
object_pointer
    │
    ▼
header.type_metadata
    │
    ▼
type_metadata.vtable
    │
    ▼
vtable[slot]
    │
    ▼
call
```

---

## 18.233 Custo Inicial da Chamada Virtual

O custo esperado inclui:

* carga dos metadados;
* carga da vtable;
* carga da entrada;
* chamada indireta.

O compilador poderá manter metadados ou vtable em registrador em sequências repetidas.

---

## 18.234 Chamada Virtual na IR

A IR poderá representar:

```text
virtual_call receiver, method_id, arguments
```

ou:

```text
virtual_call receiver, slot, signature, arguments
```

Recomenda-se preservar inicialmente a identidade lógica do método antes do lowering final.

---

## 18.235 Lowering da Chamada Virtual

O lowering deverá:

1. resolver o receptor;
2. obter ponteiro físico;
3. obter metadados;
4. localizar vtable;
5. carregar slot;
6. converter para assinatura correta;
7. realizar chamada;
8. tratar retorno.

---

## 18.236 Receptor Nulo

Código Capi seguro não deverá chamar método em ausência.

Quando o tipo for:

```text
Optional<ObjectId<T>>
```

a ausência deverá ser tratada antes da chamada.

A representação interna otimizada não deverá permitir acesso silencioso nulo.

---

## 18.237 Receptor Inválido

Uma identidade obsoleta não deverá chegar a uma chamada virtual em código seguro.

Se ocorrer por defeito ou FFI insegura, a resolução deverá falhar antes da consulta à vtable.

---

## 18.238 Chamada Direta à Base

Uma construção equivalente a:

```capi
base.speak()
```

deverá chamar diretamente a implementação da classe base.

Ela não deverá consultar o slot sobrescrito.

Conceitualmente:

```text
call Animal::speak(self)
```

---

## 18.239 `super`

Se a sintaxe utilizar `super`, sua resolução será estática.

O compilador deverá determinar:

* classe base;
* método exato;
* assinatura;
* ajuste de receptor, se necessário.

---

## 18.240 Método Privado

Métodos privados não deverão participar de override entre classes, salvo regra normativa diferente.

Eles poderão ser chamados diretamente.

---

## 18.241 Método Protegido

A visibilidade protegida afeta acesso semântico.

Não altera necessariamente o mecanismo de despacho.

---

## 18.242 Sobrecarga e Override

Sobrecarga e override são mecanismos distintos.

A resolução de sobrecarga ocorre antes do despacho dinâmico.

Sequência:

```text
selecionar assinatura lógica
        │
        ▼
determinar se é virtual
        │
        ▼
despachar implementação
```

---

## 18.243 Identificador de Método

O compilador poderá atribuir um `MethodId` a cada método lógico.

Ele poderá conter ou referenciar:

* tipo declarante;
* nome;
* assinatura;
* genericidade;
* flags;
* slot virtual.

O `MethodId` será interno.

---

## 18.244 Estabilidade do `MethodId`

O `MethodId` precisa ser estável dentro da compilação.

Não deverá ser persistido como API pública antes da estabilização da ABI.

---

## 18.245 Interfaces

Uma interface define um contrato comportamental.

Na implementação inicial, interfaces:

* não possuirão campos de instância;
* não criarão subobjeto;
* não possuirão identidade separada;
* poderão declarar métodos;
* poderão possuir métodos padrão, se permitido;
* poderão estender outras interfaces, conforme especificação.

---

## 18.246 Implementação de Interface

Uma classe que implementa uma interface deverá fornecer implementação para todos os requisitos obrigatórios.

A implementação poderá vir de:

* método próprio;
* método herdado;
* método padrão da interface;
* adaptação gerada;
* trait compatível, se definido.

---

## 18.247 Identidade em Referência de Interface

Converter um objeto para uma interface deverá preservar a identidade.

Conceitualmente:

```text
ObjectId<Dog>
        │
        ▼
InterfaceRef<Runner>
```

A referência de interface continuará representando o mesmo objeto lógico.

---

## 18.248 Representações de Interface

Duas estratégias principais poderão ser consideradas.

### Resolução por metadados a cada chamada

A referência armazena apenas `ObjectId`.

A chamada procura a interface nos metadados do tipo concreto.

### Fat reference

A referência armazena:

```text
object identity ou pointer
+
interface table pointer
```

A segunda estratégia reduz custo por chamada.

---

## 18.249 Estratégia Inicial Recomendada para Referência Persistente

Para campos e valores duradouros, poderá ser utilizada:

```text
InterfaceId<I>
├── ObjectId
└── informação de interface opcional
```

Entretanto, duplicar ponteiro de tabela persistente pode complicar invalidação e movimentação.

Uma estratégia segura é armazenar apenas a identidade e resolver a tabela quando necessário.

---

## 18.250 Estratégia Inicial Recomendada para Chamada

Após resolver a identidade, o compilador poderá produzir temporariamente:

```text
ResolvedInterfaceRef
├── object_pointer
└── interface_vtable_pointer
```

Essa referência será válida apenas durante o empréstimo.

---

## 18.251 Interface VTable

Uma tabela de interface, também chamada de itable, poderá ser:

```text
InterfaceVTable
├── interface_metadata
├── concrete_type_metadata
├── method_0
├── method_1
└── ...
```

Ela será específica para o par:

```text
tipo concreto × interface
```

---

## 18.252 Exemplo de ITable

Para:

```text
Dog implements Runner
Cat implements Runner
```

existirão:

```text
Dog_Runner_ITable
Cat_Runner_ITable
```

Cada uma apontará para as implementações correspondentes.

---

## 18.253 Slots de Interface

Os métodos da interface deverão possuir ordem determinística.

Exemplo:

```text
Runner:
  slot 0 → run
  slot 1 → stop
```

Toda implementação da interface deverá respeitar essa ordem.

---

## 18.254 Herança de Interfaces

Se uma interface estender outra:

```text
AdvancedRunner extends Runner
```

a estratégia poderá:

* prefixar os slots da interface base;
* manter tabelas separadas;
* gerar entradas para ambas.

A implementação inicial deverá adotar uma regra simples e determinística.

---

## 18.255 Estratégia Recomendada para Interfaces Herdadas

Recomenda-se que cada interface possua identidade e tabela próprias.

Uma classe que implementa `AdvancedRunner` também possuirá entrada para `Runner`.

Isso simplifica casts e evita depender de layout prefixado entre interfaces.

---

## 18.256 Tabela de Interfaces do Tipo

O `TypeMetadata` poderá apontar para:

```text
InterfaceImplementation[]
```

Cada entrada:

```text
InterfaceImplementation
├── interface_type_id
└── interface_vtable
```

---

## 18.257 Busca de Interface

A resolução poderá procurar pelo `TypeId` da interface.

Estratégias:

* busca linear;
* tabela ordenada;
* busca binária;
* hash;
* índice perfeito;
* offset pré-calculado.

A primeira implementação poderá utilizar vetor ordenado e busca binária ou busca linear para conjuntos pequenos.

---

## 18.258 Determinismo da Tabela

As entradas deverão ser ordenadas de modo determinístico, por exemplo por `TypeId`.

Isso favorece:

* builds reproduzíveis;
* busca binária;
* testes;
* comparação de artefatos.

---

## 18.259 Custo do Cast para Interface

O cast poderá exigir:

1. resolver objeto;
2. consultar metadados;
3. localizar interface;
4. obter itable.

Se o tipo concreto for conhecido, o compilador poderá selecionar a itable diretamente.

---

## 18.260 Chamada por Interface

A chamada seguirá:

```text
resolved_interface_ref
        │
        ├── object_pointer
        └── itable
               │
               ▼
          method slot
               │
               ▼
             call
```

---

## 18.261 ABI de Método de Interface

A assinatura física deverá ser compatível com a implementação concreta.

Um thunk poderá adaptar:

* receptor;
* retorno;
* genericidade;
* método herdado;
* método padrão.

---

## 18.262 Método de Classe Satisfazendo Interface

Se uma classe já possuir método compatível:

```text
Dog::run
```

a itable poderá apontar diretamente para ele ou para um thunk.

---

## 18.263 Método Herdado Satisfazendo Interface

Se a implementação vier da base:

```text
Animal::run
```

a itable da derivada poderá apontar para a mesma função.

---

## 18.264 Método Padrão de Interface

Se a linguagem permitir implementação padrão, a itable poderá apontar para:

* função padrão da interface;
* thunk de adaptação;
* implementação sobrescrita da classe.

A precedência deverá ser definida semanticamente.

---

## 18.265 Conflito entre Métodos Padrão

Quando duas interfaces fornecerem métodos padrão conflitantes, o compilador deverá exigir resolução explícita conforme a especificação.

O Runtime não deverá decidir em execução.

---

## 18.266 Interface e Mutabilidade

Uma interface poderá declarar métodos:

* leitores;
* mutantes;
* consumidores;
* associados.

A itable deverá preservar a assinatura de autoridade exigida.

---

## 18.267 Interface Genérica

Após monomorfização, interfaces genéricas concretas poderão possuir identidades distintas.

Exemplo:

```text
Comparable<Int32>
Comparable<String>
```

Cada especialização poderá possuir `TypeId` e itable próprios.

---

## 18.268 Interface Existencial

Se a linguagem suportar tipos existenciais, a representação deverá ocultar o tipo concreto mantendo:

* identidade do objeto;
* interface;
* capacidade de chamada.

Isso poderá utilizar a referência de interface descrita.

---

## 18.269 Interface sem Objeto

Interfaces com funções estáticas ou associadas não exigem referência de instância.

Sua implementação deverá utilizar resolução estática ou dicionários genéricos, conforme a linguagem.

---

## 18.270 Traits

Traits poderão compartilhar parte da infraestrutura de interfaces, mas não deverão ser automaticamente tratados como idênticos.

A especificação deverá distinguir:

* contrato nominal;
* implementação;
* método padrão;
* associação;
* composição;
* estado.

---

## 18.271 Trait como Restrição Estática

Quando um trait for usado apenas como bound genérico e monomorfizado, o compilador poderá resolver chamadas estaticamente.

Nesse caso, nenhuma itable de Runtime será necessária.

---

## 18.272 Trait Object

Se a linguagem oferecer trait objects, eles poderão utilizar infraestrutura semelhante a referências de interface.

Isso deverá ser definido explicitamente.

---

## 18.273 Upcast de Classe

Um upcast converte:

```text
ObjectId<Derived>
```

para:

```text
ObjectId<Base>
```

Sem alterar os componentes de identidade.

A conversão será estática quando a relação de herança for conhecida.

---

## 18.274 Implementação do Upcast

No modelo prefixado:

* nenhum novo objeto é criado;
* nenhum campo é copiado;
* o slot permanece;
* a geração permanece;
* o `DomainId` permanece;
* o tipo estático muda;
* o tipo concreto dos metadados permanece.

---

## 18.275 Upcast Múltiplo na Hierarquia

Para:

```text
C extends B
B extends A
```

converter `C` para `A` preservará a mesma identidade.

Não será necessário passar por objetos intermediários.

---

## 18.276 Upcast para Interface

Converter uma classe para interface deverá:

* preservar a identidade;
* verificar estaticamente que a classe implementa a interface, quando conhecida;
* associar ou localizar a itable correspondente.

Nenhuma verificação de falha será necessária quando a implementação for garantida estaticamente.

---

## 18.277 Downcast

Um downcast tenta converter uma referência de tipo mais geral para tipo mais específico.

Exemplo:

```text
ObjectId<Animal>
        │
        ▼
ObjectId<Dog>
```

Ele exigirá verificação do tipo concreto, salvo prova estática.

---

## 18.278 Formas de Downcast

A linguagem poderá oferecer:

* cast seguro que retorna `Optional<T>`;
* cast seguro que retorna `Result<T, CastError>`;
* pattern matching;
* cast assertivo que falha;
* refinamento por teste `is`.

A implementação deverá dar suporte às formas normativas.

---

## 18.279 Verificação de Downcast Exato

Para cast exato:

```text
concrete_type_id == target_type_id
```

Se verdadeiro, a identidade pode ser reinterpretada com tipo estático mais específico.

---

## 18.280 Verificação de Subtipo

Para testar se o objeto é instância de uma classe ou derivada:

```text
walk base_type chain
```

Conceitualmente:

```text
current = concrete_metadata

while current != null:
    if current.type_id == target:
        success
    current = current.base_type
```

---

## 18.281 Custo da Cadeia de Bases

Na herança simples, a busca é linear na profundidade.

Isso deverá ser suficiente inicialmente.

Otimizações futuras:

* depth;
* intervalos de subtipos;
* bitsets;
* tabelas;
* caches.

---

## 18.282 `is`

Uma operação:

```capi
value is Dog
```

deverá verificar a relação de subtipo.

Ela não deverá modificar a identidade.

---

## 18.283 Refinamento de Tipo

Após teste bem-sucedido, o compilador poderá refinar o tipo estático dentro do fluxo.

Exemplo:

```capi
if animal is Dog {
    animal.fetch();
}
```

A chamada poderá evitar nova verificação dentro do bloco quando dominada pelo teste.

---

## 18.284 Cast para Interface

Um cast dinâmico para interface deverá consultar a tabela de interfaces do tipo concreto.

Resultado:

* itable encontrada → sucesso;
* ausência → falha segura.

---

## 18.285 Cast entre Interfaces

Converter de uma interface para outra deverá:

1. preservar a identidade;
2. obter metadados concretos;
3. procurar a interface de destino;
4. retornar nova referência de interface.

A conversão não deve depender da interface de origem implementar diretamente a de destino, se o tipo concreto implementar ambas e a semântica permitir.

---

## 18.286 Cast de Interface para Classe

Deverá consultar o tipo concreto e a cadeia de bases.

A identidade permanecerá.

---

## 18.287 Cast Assertivo

Um cast assertivo inválido poderá produzir falha fatal ou panic definido pela linguagem.

Deverá incluir diagnóstico com:

* tipo de origem estático;
* tipo concreto;
* tipo de destino;
* localização.

---

## 18.288 Cast Seguro

Um cast seguro inválido deverá retornar ausência ou erro sem abortar.

Nenhum objeto será alterado.

---

## 18.289 RTTI Mínimo

O Runtime deverá possuir apenas informações dinâmicas necessárias para:

* despacho;
* destruição;
* cast;
* interface;
* diagnóstico;
* testes.

Isso constitui RTTI mínimo.

---

## 18.290 RTTI Não É Reflection Completa

O RTTI mínimo não deverá oferecer automaticamente:

* enumeração pública de campos;
* leitura de campos por nome;
* invocação arbitrária de métodos;
* criação por nome;
* acesso a privados;
* carregamento dinâmico de classes;
* modificação de metadados.

---

## 18.291 Componentes do RTTI

Poderão incluir:

```text
TypeMetadata
├── TypeId
├── kind
├── base_type
├── vtable
├── interfaces
├── destructor
├── size
├── alignment
└── debug_name opcional
```

---

## 18.292 Nome de Tipo

Nomes legíveis poderão ser incluídos em builds de desenvolvimento.

Em release, poderão ser:

* removidos;
* reduzidos;
* armazenados em seção opcional;
* preservados quando necessários a diagnósticos.

---

## 18.293 Teste de Tipo sem Nome

As operações de cast utilizarão `TypeId`, não comparação de strings.

---

## 18.294 Tipo Concreto

Todo objeto vivo deverá possuir exatamente um tipo concreto.

Esse tipo permanecerá durante todo seu lifetime.

Não haverá mudança dinâmica de classe.

---

## 18.295 Tipo Estático

Uma mesma identidade poderá aparecer sob diferentes tipos estáticos compatíveis.

Exemplo:

```text
Dog
Animal
Object
Runner
```

O tipo estático não altera os metadados concretos.

---

## 18.296 Raiz da Hierarquia

Se a Capi possuir uma classe raiz comum, sua representação deverá seguir as mesmas regras.

Se não houver raiz universal, casts e metadados deverão operar por hierarquias independentes.

---

## 18.297 `Object` Universal

Caso exista tipo `Object`, ele poderá fornecer:

* identidade;
* tipo dinâmico;
* igualdade de identidade;
* hash de identidade;
* operações básicas.

Isso não implica reflection ampla.

---

## 18.298 Destrutor Dinâmico

A destruição de um objeto acessado por tipo base deverá utilizar o destrutor do tipo concreto.

O ponteiro de destruição poderá estar em `TypeMetadata`.

---

## 18.299 Destrutor e VTable

Duas estratégias:

### Destrutor em slot da vtable

Vantagem: modelo semelhante a métodos virtuais.

### Destrutor em metadados

Vantagem: disponível mesmo sem vtable.

A estratégia inicial recomendada é armazenar o destrutor em `TypeMetadata`.

---

## 18.300 Devirtualização

Devirtualização substitui uma chamada indireta por chamada direta quando o compilador prova a implementação.

Exemplo:

```text
virtual_call Animal.speak
```

torna-se:

```text
call Dog::speak
```

---

## 18.301 Casos Simples de Devirtualização

O compilador poderá devirtualizar quando:

* o tipo concreto é conhecido;
* a classe é `final`;
* o método é `final`;
* o receptor foi recém-construído e não escapou;
* análise de fluxo refinou o tipo;
* conjunto de subclasses é fechado e monomórfico;
* chamada ocorre dentro de contexto especializado.

---

## 18.302 Classe `final`

Se o receptor possui tipo estático `Dog` e `Dog` é `final`, a implementação virtual é conhecida.

A chamada poderá ser direta.

---

## 18.303 Método `final`

Mesmo em classe extensível, um método `final` possui implementação conhecida.

---

## 18.304 Chamada Dentro da Própria Classe

Uma chamada a método virtual através de `self` poderá continuar virtual se subclasses puderem sobrescrever.

A implementação não deverá devirtualizar apenas porque o código está na classe base.

---

## 18.305 Devirtualização após `is`

Após:

```text
if value is Dog
```

a chamada dentro do ramo poderá ser direta se `Dog` ou o método forem finais.

Caso `Dog` possua subclasses possíveis, o despacho poderá continuar virtual.

---

## 18.306 Hierarquia `sealed`

Em uma hierarquia fechada, o compilador conhece todas as subclasses permitidas.

Isso poderá permitir:

* despacho por teste de tipo;
* inline cache estático;
* switch por `TypeId`;
* inlining especulativo sem perfil.

---

## 18.307 Despacho por Conjunto Fechado

Para poucas subclasses:

```text
if type == Dog:
    Dog::speak
else if type == Cat:
    Cat::speak
else:
    ...
```

Isso pode ser mais rápido quando permite inlining.

A decisão dependerá do backend.

---

## 18.308 Devirtualização Especulativa

A primeira implementação não dependerá de perfil em Runtime.

Como a Capi é AOT, otimizações especulativas complexas não serão requisito inicial.

LLVM poderá aplicar análises estáticas adicionais.

---

## 18.309 Whole-Program Analysis

Quando todos os módulos forem conhecidos, o compilador poderá determinar:

* classes concretas existentes;
* overrides possíveis;
* interfaces implementadas;
* métodos nunca sobrescritos.

Isso favorece devirtualização.

---

## 18.310 Compilação Separada

Em compilação separada, o compilador deverá ser conservador sobre subclasses externas permitidas.

Modificadores como:

* `final`;
* `sealed`;
* visibilidade de módulo;

ajudarão a preservar otimizações.

---

## 18.311 Link-Time Optimization

LLVM poderá aplicar LTO para:

* devirtualizar;
* remover vtables não usadas;
* eliminar métodos;
* fundir constantes;
* propagar tipos.

A semântica não deverá depender de LTO.

---

## 18.312 Otimizações com Cranelift

Cranelift poderá inicialmente utilizar:

* chamadas diretas quando óbvias;
* vtables para demais casos;
* eliminação simples de verificações.

O objetivo inicial será correção e velocidade de compilação.

---

## 18.313 Otimizações com LLVM

LLVM poderá posteriormente oferecer:

* inlining agressivo;
* devirtualização;
* propagação de constantes;
* eliminação de branches;
* análise interprocedural;
* LTO.

A ABI do Runtime deverá permanecer comum.

---

## 18.314 Inline de Método Virtual Devirtualizado

Após devirtualização, a implementação concreta poderá ser inline.

Isso pode eliminar:

* carga da vtable;
* chamada;
* abstrações de acesso;
* verificações redundantes.

---

## 18.315 Eliminação de Resolução Repetida

Em sequência como:

```text
obj.methodA()
obj.methodB()
obj.field
```

o compilador poderá resolver `ObjectId` uma vez e reutilizar o ponteiro durante o mesmo empréstimo válido.

---

## 18.316 Cache de VTable Local

A vtable poderá ser mantida em registrador durante várias chamadas.

Não deverá ser armazenada de forma persistente além da validade do objeto.

---

## 18.317 Polymorphic Inline Cache

Caches dinâmicos, comuns em VMs, não serão necessários inicialmente.

A Capi utiliza compilação AOT e metadados estáticos.

---

## 18.318 Remoção de VTables Não Usadas

Se análise provar que nenhum método virtual é chamado dinamicamente, a vtable poderá ser omitida.

Essa otimização não será prioridade do bootstrap.

---

## 18.319 Remoção de ITable Não Usada

Implementações de interface não utilizadas poderão ser eliminadas pelo linker ou compilador.

---

## 18.320 Dead Method Elimination

Métodos não alcançáveis poderão ser removidos.

Entretanto, metadados ou FFI podem mantê-los vivos.

Como não haverá reflection ampla por padrão, a eliminação será facilitada.

---

## 18.321 Mangling de Métodos

Símbolos deverão codificar:

* módulo;
* classe;
* nome;
* assinatura;
* especialização genérica;
* versão, se necessário.

Exemplo conceitual:

```text
_Capi_M_Dog_speak
```

A regra definitiva será documentada pela ABI.

---

## 18.322 Símbolos de VTable

Exemplo conceitual:

```text
_Capi_VT_Dog
_Capi_IT_Dog_Runner
```

A nomenclatura deverá ser determinística.

---

## 18.323 VTables como Dados Somente Leitura

Vtables e itables deverão ser emitidas como constantes.

Isso reduz risco de corrupção e permite compartilhamento.

---

## 18.324 Relocations

Entradas das tabelas conterão relocations para funções.

O linker deverá resolver:

* métodos;
* thunks;
* metadados;
* interfaces.

---

## 18.325 Visibilidade de Métodos

Métodos usados apenas internamente poderão possuir visibilidade local.

Métodos presentes em vtables ainda precisam estar acessíveis à tabela no mesmo artefato.

---

## 18.326 ABI entre Módulos

Uma classe pública utilizada entre módulos deverá expor informações suficientes sobre:

* slots virtuais;
* assinaturas;
* interfaces;
* metadados;
* layout.

O formato de metadata de compilação deverá ser a fonte de verdade.

---

## 18.327 Alteração da Ordem de Métodos

Alterar a ordem de declaração de métodos virtuais não deverá necessariamente mudar slots durante uma ABI estável.

No bootstrap, recompilação total poderá permitir mudanças.

Após estabilização, será necessário manter slots persistentes ou versionar a ABI.

---

## 18.328 Adição de Método Virtual à Base

Adicionar método virtual à base pode alterar vtables das derivadas.

Durante o bootstrap, recompilação completa será suficiente.

Após ABI estável, poderá ser quebra binária.

---

## 18.329 Remoção de Método Virtual

Também será potencialmente incompatível.

A política de versionamento será definida futuramente.

---

## 18.330 Alteração de Assinatura

Mudar assinatura de método virtual altera a ABI do slot e deverá ser tratado como mudança incompatível.

---

## 18.331 Overrides Covariantes

Se retornos covariantes forem permitidos, a implementação poderá exigir thunk.

Exemplo:

```text
Base::clone() -> Base
Derived::clone() -> Derived
```

O slot deve continuar compatível com a assinatura esperada pela base.

---

## 18.332 Parâmetros Contravariantes

A linguagem deverá definir se são permitidos.

A implementação não deverá assumir compatibilidade física sem validação.

---

## 18.333 Generics em Métodos Virtuais

Métodos virtuais genéricos podem complicar vtables porque cada especialização possui assinatura concreta.

Possíveis estratégias:

* proibir no bootstrap;
* monomorfizar chamadas com thunks;
* usar dicionários;
* gerar famílias de slots;
* apagar tipos.

A primeira implementação deverá seguir a restrição definida no subconjunto inicial.

---

## 18.334 Métodos Genéricos Não Virtuais

Podem ser monomorfizados normalmente.

---

## 18.335 Classes Genéricas com Métodos Virtuais

Cada especialização concreta da classe poderá possuir vtable própria.

Exemplo:

```text
Box<Int32>
Box<String>
```

---

## 18.336 Compartilhamento de VTables Genéricas

Somente poderá ocorrer se:

* layout;
* ABI;
* implementações;
* metadados;

forem compatíveis.

Não será prioridade inicial.

---

## 18.337 Associated Types

Interfaces ou traits com tipos associados poderão exigir especialização da itable.

A primeira implementação poderá restringir esse recurso.

---

## 18.338 Métodos Estáticos de Interface

Não participam de chamada por instância.

Sua resolução deverá ser estática ou genérica.

---

## 18.339 Propriedades

Se propriedades forem açúcar sintático para métodos, seus getters e setters poderão participar de despacho conforme as regras dos métodos.

O layout não deverá armazenar uma entidade adicional.

---

## 18.340 Indexadores

Também poderão ser métodos virtuais ou de interface.

---

## 18.341 Operadores

Operadores sobrecarregados deverão ser resolvidos semanticamente para métodos ou funções.

Somente utilizarão despacho virtual se o contrato selecionado for virtual.

---

## 18.342 Igualdade de Identidade

A igualdade padrão de objetos não deverá utilizar método virtual, salvo definição normativa diferente.

Ela poderá comparar `ObjectId`.

Igualdade estrutural ou semântica poderá ser método ou interface explícita.

---

## 18.343 Hash de Identidade

Também poderá ser calculado diretamente a partir de `ObjectId`.

---

## 18.344 `toString` ou Formatação

Uma operação de formatação poderá ser interface ou método virtual.

Não deverá exigir reflection automática.

---

## 18.345 Debug Formatting

Metadados de debug poderão fornecer nome do tipo mesmo sem método público de formatação.

---

## 18.346 Despacho Durante Construção

A linguagem deverá definir o comportamento de chamada virtual em construtores.

A estratégia mais segura para o bootstrap é restringir chamadas virtuais antes da conclusão da inicialização.

---

## 18.347 Despacho Durante Destruição

Também deverá ser definido.

Uma estratégia conservadora é:

* durante finalização de uma classe, não despachar para partes já destruídas;
* chamadas virtuais serem limitadas ou estaticamente resolvidas.

---

## 18.348 Estado de Despacho

Se necessário, o Runtime poderá utilizar estado de construção ou destruição para rejeitar chamadas inválidas.

Entretanto, o compilador deverá prevenir a maioria.

---

## 18.349 VTable Temporária de Construção

Algumas linguagens alteram vtable durante construção.

Essa estratégia aumenta complexidade.

Não é recomendada para o bootstrap.

---

## 18.350 Estratégia Conservadora

Recomenda-se:

* proibir publicação antes da construção completa;
* restringir chamadas virtuais em construtores;
* restringir chamadas virtuais em destrutores;
* manter metadados concretos constantes.

---

## 18.351 Interfaces Durante Construção

O objeto não deverá escapar como interface antes da publicação.

---

## 18.352 Interfaces Durante Destruição

Novas referências de interface não deverão ser publicadas durante finalização.

---

## 18.353 Cast Durante Construção

Operações internas podem conhecer o tipo concreto, mas casts públicos não deverão observar objeto incompleto.

---

## 18.354 Cast Durante Finalização

O Runtime poderá reconhecer o tipo, mas o acesso comum deverá ser bloqueado pelo estado.

---

## 18.355 Segurança de Chamadas Indiretas

Antes de uma chamada indireta, deverão ser verdadeiras as invariantes:

* objeto vivo;
* metadados válidos;
* vtable válida;
* slot dentro dos limites;
* função com assinatura compatível;
* receptor correto.

---

## 18.356 Validação em Debug

Builds de desenvolvimento poderão verificar:

* magic da vtable;
* `TypeId`;
* tamanho da tabela;
* slot;
* assinatura codificada;
* interface correta.

---

## 18.357 Signature Hash

Uma entrada de debug poderá possuir hash da assinatura esperada.

Isso ajuda a detectar divergências entre compilador e Runtime.

Não será necessário em release.

---

## 18.358 Control-Flow Integrity

Futuramente, o backend poderá aproveitar mecanismos de CFI da plataforma.

A arquitetura de vtables imutáveis favorece essa proteção.

---

## 18.359 Pointer Authentication

Targets com autenticação de ponteiros poderão proteger entradas de vtable.

Isso será detalhe de plataforma.

---

## 18.360 Falha de Slot Inválido

Uma chamada a slot inexistente representa erro interno.

O Runtime deverá produzir falha fatal com:

* tipo concreto;
* método;
* slot;
* versão da ABI.

---

## 18.361 Falha de Interface Ausente

Em cast seguro, ausência é resultado comum.

Em chamada cujo compilador garantiu a implementação, ausência indica erro interno.

---

## 18.362 Diagnóstico de Cast

Um erro poderá apresentar:

```text
não foi possível converter Animal para Dog
tipo concreto: Cat
```

Nomes dependerão de metadados de debug.

---

## 18.363 Diagnóstico de Chamada Abstrata

Se uma entrada abstrata for alcançada:

```text
método abstrato não implementado
classe concreta: X
método: Y
```

Isso representa defeito da Toolchain ou corrupção.

---

## 18.364 Testes de VTable

Deverão cobrir:

* classe base;
* override;
* novo método virtual;
* herança profunda;
* método final;
* classe abstrata;
* classe concreta;
* slot preservado;
* chamada correta.

---

## 18.365 Teste de Override

Exemplo:

```text
Animal a = Dog()
a.speak()
```

Deverá chamar:

```text
Dog::speak
```

---

## 18.366 Teste sem Override

Se `Dog` não sobrescrever `move`, a chamada deverá usar a implementação herdada.

---

## 18.367 Teste de `super`

Dentro de `Dog::speak`, uma chamada a `super.speak()` deverá chamar `Animal::speak`.

---

## 18.368 Teste de Identidade após Chamada

Chamadas virtuais não deverão alterar `ObjectId`.

---

## 18.369 Testes de Interface

Deverão validar:

* implementação direta;
* implementação herdada;
* múltiplas interfaces;
* método padrão;
* cast seguro;
* cast inválido;
* chamada por interface;
* identidade preservada.

---

## 18.370 Múltiplas Interfaces

Uma classe poderá implementar:

```text
Runner
Printable
Comparable
```

Cada interface possuirá itable própria.

Isso não adicionará cabeçalhos ao objeto.

---

## 18.371 Métodos com Mesmo Nome em Interfaces

O compilador deverá resolver por assinatura e identidade da interface.

Itables distintas poderão apontar para:

* mesma função;
* funções diferentes;
* thunks distintos.

---

## 18.372 Teste de Cast entre Interfaces

Um objeto que implementa duas interfaces deverá converter entre elas preservando identidade.

---

## 18.373 Teste de Downcast

Cenários:

```text
Animal contendo Dog → cast Dog: sucesso
Animal contendo Cat → cast Dog: falha
```

---

## 18.374 Teste de Subtipo Profundo

Um objeto `C` deverá satisfazer testes para:

* C;
* B;
* A;

em uma cadeia `C extends B extends A`.

---

## 18.375 Teste de Tipo Exato

Uma operação de tipo exato deverá distinguir `Dog` de subclasses de `Dog`, se a linguagem expuser essa diferença.

---

## 18.376 Testes de Devirtualização

O compilador deverá possuir testes que confirmem:

* chamada direta para classe final;
* chamada direta para método final;
* manutenção de chamada virtual quando necessária;
* comportamento idêntico.

---

## 18.377 Testes de IR

Dumps poderão verificar:

```text
virtual_call
interface_call
direct_call
type_test
downcast
```

Após otimização, alguns poderão tornar-se chamadas diretas.

---

## 18.378 Testes Diferenciais

O mesmo programa deverá produzir resultado idêntico com:

* Cranelift;
* LLVM;
* otimizações desativadas;
* otimizações ativadas.

---

## 18.379 Testes de ABI

Deverão validar:

* offsets de entradas;
* tamanho de vtables;
* assinaturas;
* thunks;
* symbols;
* relocations;
* alinhamento.

---

## 18.380 Golden Tests de VTable

Exemplo de dump:

```text
class Dog
base: Animal
vtable:
  [0] Animal::move
  [1] Dog::speak
  [2] Dog::fetch
interfaces:
  Runner:
    [0] Dog::run
```

---

## 18.381 Fuzzing de Hierarquias

O compilador poderá gerar hierarquias aleatórias com:

* overrides;
* abstratos;
* finais;
* interfaces;
* métodos padrão.

Propriedades:

* slots herdados preservados;
* classes concretas completas;
* chamadas corretas;
* casts consistentes.

---

## 18.382 Property-Based Testing de Subtipagem

Propriedades:

* todo tipo é subtipo de si;
* subtipagem é transitiva;
* classe derivada é subtipo da base;
* classe não relacionada não é subtipo;
* implementação de interface é reconhecida;
* identidade não muda em cast válido.

---

## 18.383 Testes de Erro

Deverão rejeitar:

* override de método inexistente;
* assinatura incompatível;
* sobrescrita de `final`;
* classe concreta com método abstrato;
* conflito não resolvido de interface;
* cast impossível conhecido estaticamente;
* ciclo de interfaces;
* slot inconsistente.

---

## 18.384 Benchmarks

Benchmarks prioritários:

* chamada direta;
* chamada virtual;
* chamada por interface;
* upcast;
* downcast;
* teste `is`;
* busca de interface;
* chamada devirtualizada;
* hierarquia profunda.

---

## 18.385 Métricas

Deverão ser acompanhados:

* bytes por vtable;
* bytes por itable;
* metadados por classe;
* custo de chamada;
* custo de cast;
* quantidade de thunks;
* métodos devirtualizados;
* tabelas removidas.

---

## 18.386 Critérios de Performance Inicial

No bootstrap, será aceitável:

* consulta a metadados;
* busca simples de interface;
* chamada indireta comum.

Deverão ser evitados:

* busca por nome;
* alocação durante chamada;
* construção dinâmica de tabelas;
* locks globais;
* reflexão;
* lookup linear em toda a hierarquia para cada método.

---

## 18.387 Cache de Interface

Uma implementação futura poderá armazenar, em locais de chamada, cache do último tipo concreto e itable.

Não será necessário inicialmente.

---

## 18.388 Índice Global de Interface

Outra otimização futura poderá atribuir índice global a interfaces.

Cada tipo poderia possuir tabela indexada diretamente.

Isso pode aumentar memória.

---

## 18.389 Perfect Hash

Para tipos com muitas interfaces, o compilador poderá gerar hash perfeito.

Não será prioridade.

---

## 18.390 Tabelas Compactas

Itables poderão omitir metadados redundantes em release.

---

## 18.391 Deduplicação de Thunks

Thunks equivalentes poderão ser compartilhados.

---

## 18.392 Devirtualização de Interface

Uma chamada por interface poderá ser devirtualizada quando o tipo concreto for conhecido.

Exemplo:

```text
Runner r = Dog()
r.run()
```

Se não houver escape, poderá virar:

```text
Dog::run
```

---

## 18.393 Especialização Genérica

Uma função genérica com bound de interface poderá ser monomorfizada para o tipo concreto e chamar diretamente.

---

## 18.394 Dictionary Passing

Caso genericidade reificada seja utilizada futuramente, chamadas poderão receber dicionários de métodos.

Isso não será necessário no modelo monomorfizado inicial.

---

## 18.395 Interação com FFI

Código C não deverá chamar vtables internas diretamente sem ABI documentada.

A FFI deverá expor wrappers.

---

## 18.396 Handles Polimórficos Externos

Uma API C poderá receber handle opaco e chamar funções exportadas da Capi.

O Runtime resolverá a identidade internamente.

---

## 18.397 Exposição de Interfaces à FFI

Poderá exigir:

* struct de função;
* handle;
* contexto;
* versão.

Não será parte do bootstrap inicial.

---

## 18.398 Exceções Externas em Métodos

Nenhuma exceção externa deverá atravessar uma chamada indireta Capi sem adaptação.

---

## 18.399 Unwinding

Como o Runtime inicial não dependerá de unwinding, chamadas virtuais seguirão a mesma política de erro explícito.

---

## 18.400 Retorno de `Result`

Métodos virtuais poderão retornar `Result` normalmente.

A ABI deverá representar o valor conforme o layout calculado.

---

## 18.401 Retorno Indireto

Valores grandes poderão utilizar buffer de retorno oculto.

Todos os overrides do mesmo slot deverão usar convenção compatível.

---

## 18.402 Parâmetros por Valor

Agregados poderão ser passados:

* em registradores;
* por referência;
* divididos;

conforme ABI do target.

A assinatura física deverá ser normalizada pelo backend.

---

## 18.403 Parâmetros de Identidade

`ObjectId<T>` será passado em representação uniforme.

---

## 18.404 Referência de Interface por Valor

Uma referência temporária de interface poderá ser representada por duas palavras:

```text
object_pointer
itable_pointer
```

Essa representação será adequada para parâmetros temporários.

---

## 18.405 Referência Persistente de Interface

Para armazenamento duradouro, a implementação deverá preservar invalidação.

Uma possibilidade:

```text
ObjectId
+
InterfaceTypeId
```

A itable será resolvida ao acessar.

---

## 18.406 Compactação da Referência Persistente

Como o tipo estático da variável já identifica a interface, pode ser suficiente armazenar apenas `ObjectId`.

O compilador conhecerá a interface alvo.

Essa será a estratégia inicial mais simples.

---

## 18.407 Resolução de Interface Persistente

Sequência:

```text
ObjectId<I>
    │
    ▼
resolver objeto
    │
    ▼
obter metadados concretos
    │
    ▼
obter itable de I
    │
    ▼
produzir referência temporária
```

---

## 18.408 Interface em Campo

Um campo do tipo interface armazenará a identidade do objeto.

Não armazenará ponteiro físico persistente.

---

## 18.409 Interface Opcional

Poderá utilizar o mesmo niche de `ObjectId` inválido.

---

## 18.410 Interface e Ciclos

Como campos armazenam identidade, interfaces também poderão participar de grafos cíclicos sem contagem de referências.

---

## 18.411 Interface e Domain

A identidade continua pertencendo ao Domain do objeto concreto.

A interface não cria Domain próprio.

---

## 18.412 Interface e Mutação

Uma interface mutante deverá exigir a mesma autoridade que o objeto concreto.

O cast não poderá ampliar capacidades.

---

## 18.413 Preservação de Capacidades em Casts

Upcasts e casts para interface não deverão conceder:

* maior visibilidade;
* mutabilidade adicional;
* ownership;
* lifetime mais longo.

Apenas o tipo comportamental muda.

---

## 18.414 Cast e Lifetime

O valor resultante deverá manter o mesmo limite de lifetime da identidade de origem.

---

## 18.415 Cast e Domain

O Domain não será alterado.

---

## 18.416 Cast e Geração

A geração não será alterada.

---

## 18.417 Cast e Slot

O slot não será alterado.

---

## 18.418 Cast e Endereço

Nenhum novo endereço lógico será produzido na herança simples.

Referências de interface temporárias poderão acrescentar uma itable, mas o objeto permanece o mesmo.

---

## 18.419 RTTI e Privacidade

O Runtime poderá saber o tipo concreto para despacho.

Isso não significa que o programa possa acessar membros privados.

---

## 18.420 RTTI e Segurança

Metadados deverão ser considerados imutáveis e confiáveis quando emitidos pela Toolchain.

FFI não poderá registrar tipos arbitrários sem API insegura específica.

---

## 18.421 Tipos Dinâmicos Externos

Carregamento dinâmico de novos tipos está fora do bootstrap.

---

## 18.422 Plugins

Plugins exigiriam registro e compatibilidade de metadados.

Não serão suportados inicialmente.

---

## 18.423 Bibliotecas Dinâmicas

Também exigirão ABI estável para vtables e `TypeId`.

A primeira implementação priorizará linkedição estática.

---

## 18.424 Duplicação de `TypeId`

O linker ou compilador deverá impedir colisões entre módulos do mesmo artefato.

---

## 18.425 Registro de Tipos

Se houver registrador global, ele poderá ser usado para debug e cast entre módulos.

Entretanto, chamadas virtuais comuns não deverão depender de busca global.

---

## 18.426 Type Metadata Local

O ponteiro dos metadados no objeto será suficiente para a maioria das operações.

---

## 18.427 Comparação de Metadados

Para tipo exato, poderá ser utilizada:

* comparação de `TypeId`; ou
* comparação de ponteiros de metadados, dentro do mesmo artefato.

A semântica deverá usar `TypeId`.

A comparação de ponteiros será otimização interna.

---

## 18.428 Cast com Ponteiro de Metadados

Se o metadado alvo for conhecido:

```text
object.metadata == Dog.metadata
```

pode testar tipo exato rapidamente.

---

## 18.429 Subtype Range

Uma otimização futura poderá atribuir intervalos DFS à hierarquia.

Um tipo seria subtipo quando seu número estivesse no intervalo da base.

Isso funciona bem para herança simples fechada.

---

## 18.430 Bitsets de Interfaces

Tipos poderão possuir bitsets de interfaces para testes rápidos.

Isso aumenta metadados e depende de universo fechado.

---

## 18.431 Bloom Filters

Poderiam acelerar testes negativos, mas não serão necessários inicialmente.

---

## 18.432 Caches de Cast

Chamadas frequentes de cast poderiam utilizar cache.

Não será prioridade.

---

## 18.433 Análise de Monomorfismo

O compilador poderá identificar locais de chamada com apenas uma implementação possível.

---

## 18.434 Análise de Bimorfismo

Locais com duas implementações poderão ser transformados em branch direto.

Essa otimização é futura.

---

## 18.435 Perfil de Execução

A Toolchain poderá futuramente usar PGO para otimizar despachos.

Não será requisito do bootstrap.

---

## 18.436 Despacho Dinâmico e Tamanho do Binário

Monomorfização e devirtualização podem aumentar código.

O projeto deverá equilibrar:

* velocidade;
* tamanho;
* tempo de compilação.

---

## 18.437 Política Inicial de Otimização

Na fase Cranelift:

* chamadas diretas óbvias;
* vtables simples;
* itables simples;
* casts lineares;
* pouca transformação interprocedural.

Na fase LLVM:

* devirtualização ampliada;
* inlining;
* LTO opcional;
* eliminação de tabelas não usadas.

---

## 18.438 Separação entre Frontend e Backend

O Frontend deverá determinar:

* método lógico;
* validade do override;
* relações de subtipo;
* interfaces;
* slot lógico.

O backend deverá determinar:

* representação física;
* chamada;
* símbolos;
* relocations;
* otimizações.

---

## 18.439 HIR

A HIR poderá registrar:

```text
ResolvedMethod
├── declaring_type
├── signature
├── dispatch_kind
├── virtual_slot
├── interface_slot
└── implementation_hint
```

---

## 18.440 Middle-end

O Middle-end poderá transformar:

* chamadas virtuais em diretas;
* casts redundantes;
* testes repetidos;
* referências de interface temporárias.

---

## 18.441 IR de Alto Nível

Operações possíveis:

```text
object.type_test
object.downcast
object.upcast
interface.cast
virtual.call
interface.call
```

---

## 18.442 IR de Baixo Nível

Após lowering:

```text
load metadata
load vtable
load function pointer
indirect call
```

---

## 18.443 Validador de IR

Deverá verificar:

* slot compatível;
* assinatura;
* tipo do receptor;
* resultado do cast;
* presença de itable;
* autoridade preservada;
* lifetime.

---

## 18.444 Mudança de Tipo Estático

Um cast bem-sucedido deverá produzir novo valor tipado na IR, mas apontando para a mesma identidade.

---

## 18.445 Representação SSA

Em SSA:

```text
%animal = ...
%dog? = downcast %animal to Dog
```

O ramo de sucesso receberá valor refinado.

---

## 18.446 Branch de Cast

Exemplo:

```text
%ok = type_test %animal, Dog
br %ok, success, failure
```

No sucesso, o mesmo `ObjectId` poderá ser reinterpretado como `ObjectId<Dog>`.

---

## 18.447 Eliminação de Cast

Se o compilador já sabe que o tipo é `Dog`, o cast poderá ser removido.

---

## 18.448 Cast Redundante

Sequências repetidas poderão ser simplificadas.

---

## 18.449 Teste de Interface Dominante

Após verificar que um tipo implementa interface, chamadas subsequentes no mesmo fluxo poderão reutilizar a itable.

---

## 18.450 Inlining de ITable

Quando o tipo concreto for conhecido, o ponteiro da itable será constante.

---

## 18.451 Runtime Helpers

O Runtime poderá inicialmente oferecer:

```text
capi_rt_type_is
capi_rt_type_downcast
capi_rt_interface_lookup
capi_rt_invalid_virtual_call
capi_rt_invalid_cast
```

Com o tempo, parte poderá ser inline.

---

## 18.452 Helper de Subtipo

Uma função interna poderá percorrer `base_type`.

---

## 18.453 Helper de Interface

Poderá realizar busca na tabela ordenada.

---

## 18.454 Helper de Falha

Deverá produzir diagnóstico consistente.

---

## 18.455 Ausência de Alocação em Casts

Casts não deverão alocar memória.

---

## 18.456 Ausência de Cópia em Casts

Casts não deverão copiar payload.

---

## 18.457 Ausência de Mudança de Metadados

O cabeçalho continuará apontando para o tipo concreto original.

---

## 18.458 Compatibilidade com Debugger

Símbolos e metadados poderão permitir mostrar:

* tipo estático;
* tipo concreto;
* vtable;
* interfaces;
* método selecionado.

---

## 18.459 Debug de Chamada Virtual

Uma ferramenta poderá registrar:

```text
receiver: ObjectId(...)
static type: Animal
concrete type: Dog
method: speak
slot: 1
target: Dog::speak
```

---

## 18.460 Debug de Interface

Poderá mostrar:

```text
interface: Runner
concrete type: Dog
itable: Dog_Runner
target slot 0: Dog::run
```

---

## 18.461 Logs de Despacho

Serão opcionais devido ao alto volume.

---

## 18.462 Instrumentação

Contadores possíveis:

* chamadas virtuais;
* chamadas por interface;
* chamadas devirtualizadas;
* casts;
* casts falhos;
* lookups de interface;
* cache hits futuros.

---

## 18.463 Relatório de Otimização

O compilador poderá informar:

```text
virtual calls: 120
devirtualized: 73
interface calls: 30
direct calls: ...
```

Isso auxiliará evolução.

---

## 18.464 Sanitizers

Testes deverão detectar:

* função de slot incorreta;
* assinatura incompatível;
* acesso fora da vtable;
* metadados inválidos;
* ponteiro incorreto.

---

## 18.465 Miri

Partes Rust do Runtime poderão validar manipulação de ponteiros e tabelas com Miri.

---

## 18.466 Fuzzing de Metadados

Em APIs internas de teste, metadados inválidos poderão ser injetados para validar defesa.

Código normal não deverá aceitar metadados arbitrários.

---

## 18.467 Segurança de VTables Imutáveis

As tabelas deverão permanecer em memória somente leitura quando suportado.

---

## 18.468 Corrupção de VTable

Uma vtable corrompida representa falha crítica.

O Runtime deverá abortar quando detectável.

---

## 18.469 Índice de Slot Confiável

O slot é emitido pelo compilador e não controlado pelo programa.

Ainda assim, builds de debug poderão validar limites.

---

## 18.470 InterfaceTypeId Confiável

O tipo da interface em chamadas comuns será estático.

Casts dinâmicos deverão validar.

---

## 18.471 Métodos Nativos

Métodos implementados no Runtime ou FFI poderão ocupar slots se respeitarem a ABI.

---

## 18.472 Thunk para Método Nativo

Poderá adaptar a convenção externa para a interna.

---

## 18.473 Métodos Intrínsecos

Intrínsecos normalmente serão resolvidos estaticamente.

Não deverão ocupar vtable sem necessidade.

---

## 18.474 Objetos Proxy

Proxies dinâmicos não farão parte do bootstrap.

Poderão ser implementados futuramente por classes comuns ou geração de código.

---

## 18.475 Interceptação de Métodos

Não será mecanismo automático do Runtime.

---

## 18.476 Reflection de Invocação

Invocação por nome não será suportada por padrão.

---

## 18.477 Serialização Polimórfica

Bibliotecas poderão usar tags explícitas ou interfaces.

Não deverão depender do endereço da vtable.

---

## 18.478 Persistência de `TypeId`

O `TypeId` interno não deverá ser persistido como formato estável antes de política específica.

---

## 18.479 Nome Canônico de Tipo

Serialização poderá utilizar nomes ou identificadores próprios definidos pela aplicação.

---

## 18.480 Versionamento de Interfaces

Alterar métodos de interface pode quebrar itables.

Durante o bootstrap, recompilação total será suficiente.

---

## 18.481 Adição de Método Padrão

Pode ser compatível em alguns modelos, mas a política binária futura deverá ser definida.

---

## 18.482 Remoção de Método de Interface

Será quebra incompatível.

---

## 18.483 Ordem de Slots de Interface

Após ABI estável, deverá permanecer fixa.

---

## 18.484 Hash de Interface

O compilador poderá produzir hash da assinatura completa da interface.

---

## 18.485 Compatibilidade na Linkedição

Módulos com hashes incompatíveis não deverão ser vinculados silenciosamente.

---

## 18.486 ABI Version Symbol

A Toolchain poderá emitir símbolo de versão para:

* layout de vtable;
* itables;
* metadados;
* chamadas.

---

## 18.487 Runtime Único

Todas as classes do processo deverão utilizar a mesma convenção de metadados.

---

## 18.488 Cross-compilation

O tamanho dos ponteiros muda por target, mas a estrutura lógica permanece.

---

## 18.489 Endianness

Entradas são ponteiros e identificadores nativos.

A semântica não depende da ordem de bytes.

---

## 18.490 WebAssembly

Chamadas indiretas em WebAssembly utilizam tabelas de funções.

O lowering deverá adaptar vtables às restrições do target.

---

## 18.491 CFI em WebAssembly

Índices de função e assinaturas tipadas podem oferecer validação adicional.

---

## 18.492 Targets sem Ponteiro de Função Convencional

A ABI deverá abstrair a representação de callable.

---

## 18.493 Portabilidade

A fonte de verdade deverá ser o modelo lógico, não o layout de uma plataforma específica.

---

## 18.494 ADRs Recomendados

Deverão ser considerados:

```text
ADR — VTable por tipo concreto
ADR — Slots virtuais estáveis por hierarquia
ADR — TypeMetadata apontando para VTable
ADR — Destrutor fora da sequência comum de slots
ADR — ITable por par tipo-interface
ADR — Referência persistente de interface baseada em ObjectId
ADR — Referência temporária de interface com object pointer e ITable
ADR — Busca inicial de interfaces
ADR — RTTI mínimo
ADR — Política inicial de devirtualização
ADR — Restrição de chamadas virtuais durante construção e destruição
```

---

## 18.495 Invariantes de VTable

A implementação deverá preservar:

1. cada slot herdado mantém posição;
2. cada override compatível substitui exatamente um slot;
3. cada entrada possui assinatura ABI correta;
4. cada classe concreta possui implementação para todos os slots obrigatórios;
5. vtables são imutáveis após emissão;
6. o tipo concreto determina a vtable utilizada;
7. o tipo estático determina o slot solicitado;
8. chamadas diretas à base não consultam override;
9. devirtualização não altera comportamento;
10. nenhuma chamada virtual modifica a identidade.

---

## 18.496 Invariantes de Interface

1. interfaces não adicionam estado ao payload;
2. cada itable pertence a um par tipo concreto-interface;
3. os slots da interface mantêm ordem estável;
4. referências de interface preservam identidade;
5. casts não ampliam lifetime ou mutabilidade;
6. ausência de interface é tratada de modo seguro;
7. chamadas por interface utilizam implementação compatível;
8. múltiplas interfaces não criam múltiplos objetos;
9. métodos padrão seguem precedência semântica;
10. a itable não é persistida como identidade do objeto.

---

## 18.497 Invariantes de Cast

1. cast não aloca;
2. cast não copia payload;
3. cast não altera `DomainId`;
4. cast não altera slot;
5. cast não altera geração;
6. cast não altera tipo concreto;
7. upcast válido não falha;
8. downcast somente sucede para tipo compatível;
9. cast para interface somente sucede quando implementada;
10. cast seguro inválido não corrompe estado.

---

## 18.498 Critérios de Conclusão das VTables

O suporte básico estará concluído quando:

* slots virtuais forem atribuídos;
* overrides substituírem slots;
* classes derivadas preservarem slots herdados;
* chamadas virtuais selecionarem implementação correta;
* métodos `final` puderem ser chamados diretamente;
* classes abstratas forem validadas;
* metadados apontarem para vtables;
* Cranelift gerar chamadas indiretas corretas;
* testes de herança passarem.

---

## 18.499 Critérios de Conclusão das Interfaces

Estará concluído quando:

* interfaces possuírem slots determinísticos;
* classes emitirem itables;
* chamadas por interface funcionarem;
* múltiplas interfaces forem suportadas;
* casts para interface funcionarem;
* identidade for preservada;
* métodos herdados puderem satisfazer interfaces;
* métodos padrão necessários forem suportados;
* diagnósticos estiverem disponíveis.

---

## 18.500 Critérios de Conclusão dos Casts e RTTI

Estará concluído quando:

* teste de tipo exato funcionar;
* teste de subtipo funcionar;
* downcast seguro funcionar;
* cast assertivo produzir diagnóstico;
* cast entre interfaces funcionar;
* RTTI mínimo estiver emitido;
* nenhuma reflection ampla for necessária;
* testes de hierarquia profunda passarem.

---

## 18.501 Critérios de Conclusão da Devirtualização Inicial

Estará concluída quando o compilador puder transformar em chamada direta:

* receptor de classe `final`;
* método `final`;
* tipo concreto conhecido;
* cast refinado evidente.

O comportamento deverá permanecer idêntico com a otimização desativada.

---

## 18.502 Resultado Esperado da Segunda Parte

Ao final desta etapa, a implementação oficial deverá possuir um modelo em que:

```text
TypeMetadata
    identifica o tipo concreto

VTable
    associa slots virtuais a implementações concretas

Interface Table
    associa contratos de interface ao tipo concreto

ObjectId
    preserva identidade em todos os casts

Upcast
    altera apenas o tipo estático

Downcast
    valida o tipo concreto

RTTI mínimo
    suporta despacho, cast e destruição

Devirtualização
    remove o custo dinâmico quando houver prova estática
```

Esse modelo deverá permitir que uma única instância seja utilizada por:

* seu tipo concreto;
* suas classes base;
* suas interfaces;
* restrições genéricas compatíveis;

sem cópia, sem nova identidade e sem dependência de endereço físico como contrato público.

A etapa seguinte deverá completar o Capítulo 18 com:

* construção completa de objetos;
* encadeamento de construtores;
* destruição de hierarquias;
* finalizadores;
* cleanup parcial;
* integração com Domains;
* evolução da ABI;
* critérios finais para o modelo de objetos do bootstrap.

---

## Terceira Parte — Bloco 1 — Construção de Objetos

## 18.503 Objetivo

Esta parte define a estratégia inicial para materializar a construção de objetos por identidade na implementação oficial da Capi.

Serão definidos:

* alocação;
* reserva de slot;
* criação da identidade;
* inicialização do cabeçalho;
* inicialização da classe base;
* inicialização da classe derivada;
* avaliação de argumentos;
* inicialização de campos;
* execução de construtores;
* construção parcial;
* cleanup em caso de falha;
* publicação da identidade;
* construção de grafos;
* integração com Domains;
* validação estática;
* operações de IR;
* lowering para Runtime e backends.

O objetivo central será garantir que nenhum objeto incompleto seja observado como objeto vivo por código Capi seguro.

---

## 18.504 Princípio da Construção Segura

A construção deverá obedecer ao seguinte princípio:

> Um objeto somente poderá tornar-se observável como identidade válida após a conclusão de todas as etapas obrigatórias de sua inicialização.

Antes da publicação:

* a memória poderá existir;
* o slot poderá estar reservado;
* o cabeçalho poderá estar parcialmente configurado;
* campos poderão estar inicializados;
* construtores poderão estar em execução;

mas o objeto ainda não será considerado vivo para uso geral.

---

## 18.505 Estados Conceituais da Construção

A criação de um objeto poderá atravessar os seguintes estados:

```text
Unallocated
Reserved
Allocated
HeaderInitialized
BaseInitializing
DerivedInitializing
FullyInitialized
Published
Alive
```

Estados de falha poderão conduzir a:

```text
ConstructionFailed
CleaningUp
Cancelled
```

---

## 18.506 Estado `Unallocated`

Nesse estado:

* nenhum slot foi reservado;
* nenhuma memória foi obtida;
* nenhuma identidade foi criada;
* nenhum cleanup específico é necessário.

---

## 18.507 Estado `Reserved`

Nesse estado:

* um slot do Domain foi reservado;
* uma geração foi selecionada;
* nenhuma identidade pública foi retornada;
* a memória do objeto poderá ainda não existir.

---

## 18.508 Estado `Allocated`

Nesse estado:

* o armazenamento físico foi reservado;
* o slot poderá conhecer o ponteiro interno;
* o objeto ainda não possui cabeçalho completo;
* o estado do slot não será `Alive`.

---

## 18.509 Estado `HeaderInitialized`

Nesse estado:

* o cabeçalho contém metadados concretos;
* a associação ao slot está definida;
* o estado interno indica inicialização;
* o payload ainda poderá estar vazio ou parcialmente inicializado.

---

## 18.510 Estado `BaseInitializing`

A região correspondente à classe base estará sendo inicializada.

Nesse estado:

* campos da base poderão ser escritos;
* construtores da base poderão executar;
* campos da derivada ainda não estarão disponíveis para uso geral;
* o objeto não poderá escapar.

---

## 18.511 Estado `DerivedInitializing`

A parte correspondente à classe derivada estará sendo inicializada.

Nesse estado:

* a base já estará inicializada;
* campos derivados poderão ser inicializados;
* invariantes finais ainda poderão não estar satisfeitos;
* a identidade continuará não publicada.

---

## 18.512 Estado `FullyInitialized`

Nesse estado:

* todos os campos obrigatórios foram inicializados;
* construtores necessários terminaram;
* invariantes de construção foram satisfeitos;
* o objeto ainda poderá aguardar publicação atômica.

---

## 18.513 Estado `Published`

No momento da publicação:

* o slot passa a representar objeto vivo;
* o `ObjectId` torna-se válido;
* o objeto entra na ordem de finalização do Domain;
* a identidade pode ser retornada ou armazenada.

---

## 18.514 Construção como Transação Lógica

A criação deverá comportar-se como uma transação lógica:

```text
iniciar construção
        │
        ├── sucesso → publicar
        └── falha   → desfazer recursos inicializados
```

A implementação não precisa oferecer rollback de memória física no alocador linear.

Entretanto, deverá garantir rollback semântico:

* nenhum objeto vivo permanece;
* nenhum recurso inicializado é perdido;
* nenhum slot publicado permanece;
* nenhum campo é destruído duas vezes.

---

## 18.515 Operador de Criação

A sintaxe de criação poderá utilizar uma forma equivalente a:

```capi
new Person(name, age)
```

ou construção associada a um Domain explícito:

```capi
domain.new Person(name, age)
```

A sintaxe definitiva seguirá a especificação.

O lowering deverá sempre identificar o Domain de destino.

---

## 18.516 Domain de Destino

Toda instância de classe deverá ser criada em um Domain válido.

O Domain poderá ser determinado por:

* expressão explícita;
* contexto lexical;
* parâmetro implícito resolvido pelo compilador;
* operação de biblioteca;
* Domain raiz, quando permitido.

A escolha não deverá depender de variável global invisível do Runtime.

---

## 18.517 Validação do Domain

Antes da construção, deverá ser validado que o Domain:

* existe;
* está ativo;
* aceita alocações;
* pertence ao contexto correto;
* não está em finalização;
* satisfaz as regras de lifetime.

---

## 18.518 Avaliação dos Argumentos

Os argumentos do construtor deverão ser avaliados antes da publicação do objeto.

A ordem de avaliação deverá ser definida pela linguagem.

A implementação inicial deverá preservar essa ordem explicitamente na IR.

---

## 18.519 Falha na Avaliação dos Argumentos

Se a avaliação falhar antes da reserva do objeto:

* nenhum slot será criado;
* nenhuma memória será reservada;
* somente temporários já produzidos deverão ser destruídos.

---

## 18.520 Reserva Antes ou Depois dos Argumentos

Duas estratégias são possíveis:

### Avaliar argumentos antes da reserva

Reduz cleanup do objeto.

### Reservar antes da avaliação

Pode facilitar referências recursivas controladas.

A estratégia inicial deverá avaliar argumentos comuns antes da reserva, salvo construção especial de grafos.

---

## 18.521 Seleção do Construtor

O compilador deverá resolver estaticamente:

* classe concreta;
* construtor aplicável;
* sobrecarga;
* argumentos genéricos;
* visibilidade;
* conversões permitidas;
* Domain de destino.

A seleção não deverá ocorrer por nome em Runtime.

---

## 18.522 Classe Concreta

Somente classes concretas poderão ser instanciadas.

O compilador deverá rejeitar:

* interfaces;
* traits não instanciáveis;
* classes abstratas;
* classes incompletas;
* especializações genéricas inválidas.

---

## 18.523 Metadados do Tipo Concreto

O código gerado deverá obter referência para o `TypeMetadata` da classe concreta.

Esses metadados fornecerão:

* tamanho total;
* alinhamento;
* payload offset;
* destrutor;
* vtable;
* interfaces;
* informações de construção necessárias.

---

## 18.524 Reserva do Slot

O compilador ou Runtime deverá solicitar:

```text
reserve_object(domain, type_metadata)
```

A operação deverá retornar uma reserva interna.

Conceitualmente:

```text
ObjectReservation
├── domain
├── slot_index
├── generation
├── type_metadata
├── storage
└── state
```

---

## 18.525 Identidade Interna de Construção

A reserva poderá conhecer:

```text
DomainId
slot
generation
```

Esses componentes ainda não constituirão `ObjectId` público.

Poderá existir um handle interno:

```text
ConstructionId<T>
```

restrito à fase de construção.

---

## 18.526 Ausência de `ObjectId` Público

Antes da publicação, não deverá ser possível:

* retornar a identidade;
* armazená-la em objeto vivo;
* inseri-la em coleção comum;
* expô-la à FFI;
* compará-la como identidade viva;
* resolver acesso comum.

---

## 18.527 Alocação do Armazenamento

Após a reserva, deverá ser alocado:

```text
header + padding + payload
```

conforme o layout concreto.

A operação deverá verificar:

* tamanho;
* alinhamento;
* overflow;
* capacidade do Domain;
* falha do alocador.

---

## 18.528 Memória Inicial do Payload

A memória poderá ser:

* não inicializada;
* preenchida com padrão de debug;
* zerada em modo específico.

Nenhuma dessas opções altera a necessidade de inicialização semântica.

---

## 18.529 Inicialização do Cabeçalho

O cabeçalho deverá ser inicializado antes de executar código de construção que dependa de metadados.

Deverão ser definidos:

* `type_metadata`;
* estado `Initializing`;
* associação ao slot, se armazenada;
* flags iniciais;
* informações de debug opcionais.

---

## 18.530 Metadados Concretos Imutáveis

O campo de metadados deverá apontar para a classe concreta desde o início e permanecer inalterado durante todo o lifetime.

Não deverá haver troca de tipo durante a construção.

---

## 18.531 Cabeçalho e Falha

Se a construção falhar após o cabeçalho ser inicializado:

* o objeto ainda não será considerado vivo;
* o cleanup utilizará os metadados concretos;
* o estado será alterado para cancelamento;
* o slot não será publicado.

---

## 18.532 Inicialização da Parte Base

Na herança simples, a construção deverá iniciar pela classe base.

Para:

```text
C extends B
B extends A
```

a ordem será:

```text
A
B
C
```

---

## 18.533 Razão da Ordem Base-Derivada

Essa ordem garante que:

* a região herdada esteja válida antes da derivada;
* métodos da derivada possam depender da base já construída;
* invariantes herdados sejam estabelecidos;
* cleanup possa ocorrer em ordem inversa.

---

## 18.534 Argumentos do Construtor da Base

A classe derivada deverá especificar ou permitir determinar os argumentos da base.

Esses argumentos deverão ser avaliados de forma definida.

Exemplo conceitual:

```capi
class Dog(name: String) extends Animal(name) {
}
```

---

## 18.535 Chamada Implícita da Base

Se a linguagem permitir construtor padrão da base, o compilador poderá inserir a chamada.

A inserção deverá ser explícita na HIR ou IR.

---

## 18.536 Ausência de Construtor Compatível

Se a base exigir argumentos e nenhum encadeamento válido for fornecido, a compilação deverá falhar.

---

## 18.537 Construtor Privado da Base

A visibilidade deverá ser validada.

Uma classe derivada não poderá invocar construtor inacessível.

---

## 18.538 Construtores Herdados

Construtores não deverão ser herdados automaticamente, salvo regra explícita da linguagem.

A classe derivada construirá o objeto concreto e encadeará a base.

---

## 18.539 Inicialização de Campos da Base

A parte base deverá inicializar todos os seus campos obrigatórios.

A derivada não deverá inicializar diretamente campos privados da base.

---

## 18.540 Campos com Inicializador de Declaração

Um campo poderá possuir inicializador junto à declaração.

Exemplo:

```capi
var active: Bool = true;
```

O compilador deverá incorporar essa expressão ao plano de construção.

---

## 18.541 Ordem entre Inicializadores e Construtor

A especificação deverá definir a ordem.

Uma estratégia recomendada:

1. preparar argumentos da base;
2. construir a base;
3. executar inicializadores de campos da classe atual;
4. executar corpo do construtor da classe atual.

Essa regra deverá ser consistente em todos os níveis.

---

## 18.542 Inicialização Antes do Corpo

Campos com inicializador deverão estar disponíveis no corpo do construtor.

Campos sem inicializador deverão ser atribuídos antes de qualquer leitura.

---

## 18.543 Parâmetros Promovidos a Campos

Se a sintaxe permitir parâmetros de construtor que declaram campos, o compilador deverá tratá-los como inicializações explícitas.

---

## 18.544 Inicialização da Derivada

Após a conclusão da base:

* inicializadores dos campos derivados serão avaliados;
* campos derivados obrigatórios serão escritos;
* o corpo do construtor derivado será executado;
* invariantes finais serão verificados estaticamente quando possível.

---

## 18.545 Ordem dos Campos

A estratégia inicial deverá inicializar campos na ordem de declaração.

Mesmo que os inicializadores apareçam em outra ordem sintática, a regra deverá ser inequívoca.

---

## 18.546 Dependência entre Inicializadores

Um inicializador somente poderá ler campos cuja inicialização esteja garantida.

Exemplo:

```capi
let width = 10;
let area = width * height;
```

será válido apenas se `width` e `height` estiverem inicializados.

---

## 18.547 Leitura Antes da Inicialização

O compilador deverá rejeitar qualquer leitura de campo ainda não inicializado.

Isso inclui:

* acesso direto;
* chamada de método que possa lê-lo;
* passagem de `self`;
* closure;
* FFI;
* formatação;
* logging.

---

## 18.548 Escrita Única em Campos `let`

Campos imutáveis deverão ser inicializados exatamente uma vez durante a construção.

Após a publicação, não poderão ser alterados.

---

## 18.549 Campos `var`

Campos mutáveis também deverão possuir valor inicial antes da publicação.

A mutabilidade não significa ausência de inicialização.

---

## 18.550 Inicialização Condicional

Para campos obrigatórios, todos os caminhos de controle deverão inicializá-los.

Exemplo inválido:

```text
if condition:
    initialize field
else:
    no initialization
```

---

## 18.551 Análise de Inicialização Definida

O compilador deverá executar análise de fluxo para determinar:

* campos inicializados;
* campos possivelmente inicializados;
* campos não inicializados;
* caminhos de retorno;
* caminhos de erro;
* loops;
* branches.

---

## 18.552 Modelo de Estado por Campo

Durante a análise, cada campo poderá possuir:

```text
Uninitialized
Initialized
Moved
Destroyed
MaybeInitialized
```

Estados adicionais poderão ser usados para valores condicionais.

---

## 18.553 Escrita Dupla em Campo Imutável

Deverá ser rejeitada, mesmo durante a construção, salvo mecanismo específico de atribuição condicional comprovadamente exclusiva.

---

## 18.554 Movimento de Campo Durante Construção

Se um campo inicializado for movido antes da publicação, ele deverá ser reinicializado antes da conclusão ou o objeto não poderá ser publicado.

---

## 18.555 Campos Opcionais

Um campo opcional ainda deverá ser inicializado, normalmente com:

```text
None
```

A ausência é um valor, não ausência de inicialização.

---

## 18.556 Campos de Classe

Campos contendo identidade de outra classe deverão receber `ObjectId` válido ou valor opcional apropriado.

Não poderão armazenar identidade em construção comum.

---

## 18.557 Campos por Valor

Campos por valor deverão ser construídos inline.

Seu estado de inicialização fará parte do estado do objeto externo.

---

## 18.558 Construção de Valores Temporários

Um inicializador poderá construir temporários.

Esses temporários deverão:

* ser movidos para o campo; ou
* ser destruídos ao fim da expressão.

---

## 18.559 Movimento para Campo

Quando um valor não copiável for usado para inicializar campo:

```text
temporary → field
```

o temporário será consumido.

O cleanup não deverá destruí-lo duas vezes.

---

## 18.560 Cópia para Campo

Tipos copiáveis poderão ser copiados conforme sua semântica.

---

## 18.561 Inicialização In-Place

Para valores grandes, o compilador poderá construir diretamente no endereço do campo.

Isso evita temporários.

A semântica deverá permanecer igual.

---

## 18.562 Construção In-Place de Campo

A IR poderá fornecer o endereço do campo como destino de construção.

Em caso de falha:

* o campo permanecerá não inicializado;
* subcampos construídos serão limpos;
* o objeto externo continuará em construção.

---

## 18.563 Falha em Inicializador

Se um inicializador retornar erro:

* os campos já inicializados da classe atual serão destruídos em ordem inversa;
* a parte base já construída será destruída;
* o slot será cancelado;
* a identidade não será publicada.

---

## 18.564 Construtores com `Result`

Um construtor falível poderá retornar:

```text
Result<ObjectId<T>, E>
```

ou açúcar sintático equivalente.

No caso de erro, nenhum objeto vivo será retornado.

---

## 18.565 Construtor Infallível

Um construtor infalível somente poderá falhar por condições fatais, como:

* falta de memória;
* erro interno;
* violação de contrato;
* panic.

---

## 18.566 Retorno Explícito no Construtor

A linguagem deverá definir se construtores podem utilizar `return`.

Se permitido, todo retorno bem-sucedido deverá ocorrer somente após os campos obrigatórios estarem inicializados.

---

## 18.567 Propagação de Erro

Uma operação equivalente a `?` dentro do construtor deverá encaminhar para bloco de cleanup da construção.

---

## 18.568 Cleanup Estruturado

O compilador deverá gerar cleanup baseado no conjunto de elementos já inicializados.

Exemplo:

```text
Base construída
Field A construído
Field B falhou
```

Cleanup:

```text
destruir Field A
destruir Base
cancelar objeto
```

---

## 18.569 Ordem de Cleanup

A ordem deverá ser inversa à inicialização.

Para:

```text
Base
Field A
Field B
Field C
```

em falha após `Field C`:

```text
Field C
Field B
Field A
Base
```

considerando apenas elementos efetivamente concluídos.

---

## 18.570 Cleanup da Classe Base

A destruição da base durante falha deverá utilizar rotina de cleanup de construção, que poderá diferir do destrutor público completo.

Isso evita executar lógica que presume objeto totalmente vivo.

---

## 18.571 Destrutor e Objeto Parcial

O destrutor normal do tipo concreto não deverá ser chamado sobre objeto parcialmente construído.

Em vez disso, deverão ser chamadas rotinas específicas para as partes concluídas.

---

## 18.572 Drop Glue Parcial

O compilador poderá gerar funções internas como:

```text
drop_initialized_fields_of_Dog
drop_initialized_fields_of_Animal
```

ou blocos de cleanup inline.

---

## 18.573 Bitmap de Inicialização em Runtime

A primeira implementação deverá preferir controle estático e blocos de cleanup.

Bitmap dinâmico somente será necessário quando o fluxo não puder ser representado de forma eficiente.

---

## 18.574 Flag de Progresso

Em construções complexas, o código gerado poderá manter um discriminante local:

```text
progress = 0
progress = 1
progress = 2
```

No erro, um `switch` selecionará o cleanup correto.

---

## 18.575 Cleanup Idempotente

Cada recurso deverá ser destruído no máximo uma vez.

Os blocos gerados deverão impedir execução duplicada.

---

## 18.576 Falha no Cleanup

Uma falha durante cleanup de construção representa situação crítica.

A política inicial deverá evitar destruidores falíveis.

Caso ocorra panic, o Runtime poderá abortar.

---

## 18.577 Construtores Não Virtuais

Construtores não deverão utilizar despacho virtual comum.

A classe concreta é conhecida no ponto de criação.

---

## 18.578 Encadeamento Estático

Chamadas entre construtores da hierarquia serão resolvidas estaticamente.

---

## 18.579 Chamada Virtual Durante Construção

A estratégia inicial deverá restringir chamadas virtuais sobre `self` enquanto o objeto não estiver completamente inicializado.

Isso evita acesso a campos derivados não inicializados.

---

## 18.580 Restrição Conservadora

Durante construção, poderão ser permitidas apenas:

* chamadas estáticas;
* métodos privados;
* métodos explicitamente seguros para construção;
* métodos da parte já inicializada, sem despacho para derivadas;
* funções livres.

---

## 18.581 Chamada à Base Durante Construção

Um construtor poderá chamar método não virtual da base se os campos necessários estiverem inicializados.

---

## 18.582 Escape de `self`

Antes da publicação, `self` não deverá:

* ser retornado;
* ser armazenado em objeto vivo;
* ser inserido em coleção persistente;
* ser enviado a callback desconhecido;
* ser passado à FFI;
* ser capturado por closure duradoura;
* ser armazenado globalmente.

---

## 18.583 Empréstimo Local de `self`

O compilador poderá permitir uso local controlado de `self` dentro do construtor.

Esse uso deverá respeitar o estado de inicialização dos campos.

---

## 18.584 Referência a Campo Inicializado

Poderá ser permitido emprestar um campo já inicializado sem expor o objeto completo.

---

## 18.585 Métodos de Inicialização

A linguagem poderá oferecer métodos especiais utilizáveis apenas durante construção.

Esses métodos deverão declarar quais campos exigem ou inicializam.

No bootstrap, essa complexidade poderá ser evitada.

---

## 18.586 Construtor Primário

Uma classe poderá definir um construtor principal responsável por estabelecer o objeto completo.

---

## 18.587 Construtores Secundários

Construtores secundários deverão delegar para:

* outro construtor da mesma classe; ou
* a sequência completa de base e campos;

conforme a regra normativa.

---

## 18.588 Delegação entre Construtores

A implementação deverá impedir:

* ciclos;
* inicialização duplicada;
* retorno antes da construção principal;
* publicação intermediária.

---

## 18.589 Ciclo de Delegação

Exemplo inválido:

```text
A() chama A(Int32)
A(Int32) chama A()
```

O compilador deverá detectar estaticamente.

---

## 18.590 Factory Methods

Métodos fábrica não são construtores especiais do Runtime.

Eles poderão:

* validar;
* escolher subtipo;
* retornar cache;
* criar objeto;
* retornar `Result`.

Apenas a operação interna de construção segue este capítulo.

---

## 18.591 Fábrica que Retorna Objeto Existente

Uma factory poderá retornar identidade já existente.

Nesse caso, não haverá construção nova.

---

## 18.592 Construtor e Identidade

Cada criação bem-sucedida de objeto novo deverá produzir identidade nova.

Mesmo que todos os campos sejam iguais, os `ObjectId` serão distintos.

---

## 18.593 Identidade Antes da Publicação

A identidade lógica poderá ser reservada internamente para suportar algoritmos de construção, mas não deverá ser considerada viva.

---

## 18.594 Publicação da Identidade

A publicação deverá ocorrer somente após:

* base construída;
* campos derivados construídos;
* corpo do construtor concluído;
* invariantes satisfeitos;
* metadados válidos;
* slot registrado;
* estado pronto para `Alive`.

---

## 18.595 Operação de Publicação

Conceitualmente:

```text
reservation.publish()
```

A operação deverá:

1. validar estado `FullyInitialized`;
2. registrar o objeto na ordem de criação;
3. marcar o slot como `Alive`;
4. marcar o cabeçalho como `Alive`, se aplicável;
5. construir `ObjectId`;
6. consumir a reserva.

---

## 18.596 Ordem entre Registro e Estado Vivo

A sequência deverá evitar janela lógica em que:

* o objeto esteja vivo mas não registrado para destruição; ou
* esteja registrado como vivo sem estar acessível corretamente.

No modelo sequencial, a publicação poderá ser uma sequência indivisível conceitualmente.

---

## 18.597 Falha Durante Publicação

A publicação deverá ser projetada para não falhar após alterar o estado para vivo.

Recursos necessários ao registro deverão ser reservados antes.

---

## 18.598 Reserva da Estrutura de Finalização

Se a ordem de criação utilizar vetor, sua capacidade poderá ser garantida antes da publicação.

---

## 18.599 Consumo da Reserva

Após publicação, o guard de reserva não deverá executar cleanup automático.

Em Rust, a operação poderá consumir `self` ou marcar estado interno.

---

## 18.600 Retorno do `ObjectId`

Somente após a publicação o construtor retornará:

```text
ObjectId<T>
```

---

## 18.601 Armazenamento Imediato

O chamador poderá então:

* armazenar a identidade;
* realizar upcast;
* inseri-la em coleção;
* chamar métodos;
* criar ciclos;
* passá-la a outras funções.

---

## 18.602 Observabilidade do Construtor

Efeitos externos realizados durante construção podem ocorrer antes da publicação.

Exemplos:

* escrita em arquivo;
* log;
* alteração de recurso externo.

Se a construção falhar, esses efeitos não serão automaticamente revertidos.

---

## 18.603 Recomendação sobre Efeitos

Construtores deverão preferir estabelecer invariantes locais.

Operações externas falíveis ou irreversíveis poderão ser realizadas por factories ou builders.

---

## 18.604 Construtor e Registro Global

Registrar `self` globalmente antes da publicação será proibido.

Após publicação, uma factory poderá realizar registro adicional.

---

## 18.605 Builders

Builders poderão ser utilizados para:

* muitos parâmetros;
* configuração opcional;
* validação;
* construção falível;
* grafos;
* separação de efeitos.

Um builder será um valor ou objeto comum, não parte obrigatória do Runtime.

---

## 18.606 Builder por Valor

Um builder por valor poderá acumular campos e, ao final:

```text
build(domain) -> Result<ObjectId<T>, E>
```

---

## 18.607 Vantagem do Builder

Ele reduz a necessidade de expor objeto parcialmente inicializado.

---

## 18.608 Construção em Duas Fases

Alguns objetos podem exigir:

1. construção estrutural;
2. ativação externa.

Exemplo:

```text
create socket wrapper
connect
```

Essas fases deverão ser modeladas por estados públicos distintos ou tipos distintos, e não por objeto internamente incompleto.

---

## 18.609 Typestate

A linguagem ou biblioteca poderá representar:

```text
UnconnectedSocket
ConnectedSocket
```

como tipos diferentes.

Isso é preferível a flag de inicialização parcialmente pública.

---

## 18.610 Grafos Acíclicos

Grafos acíclicos podem ser construídos naturalmente de folhas para raiz.

Exemplo:

```text
criar C
criar B referenciando C
criar A referenciando B
```

---

## 18.611 Grafos Cíclicos

Grafos cíclicos exigem cuidado porque nenhum objeto incompleto poderá receber referência pública.

Exemplo:

```text
A → B
B → A
```

---

## 18.612 Estratégias para Ciclos

Possibilidades:

* campos opcionais inicializados após publicação;
* builder de grafo;
* reserva controlada de identidades;
* arena específica;
* closure de construção;
* operação transacional do Domain.

A primeira implementação deverá priorizar a estratégia mais simples compatível com a especificação.

---

## 18.613 Ciclos por Campos Opcionais

Uma forma segura:

1. criar `A` com ligação ausente;
2. criar `B` referenciando `A`;
3. mutar `A` para referenciar `B`.

Isso exige:

* campo opcional;
* Capacidade de Mutação;
* invariantes que permitam estado temporário publicado.

---

## 18.614 Limitação da Estratégia Opcional

Essa técnica não é adequada quando o tipo exige o ciclo como invariante desde a criação.

---

## 18.615 Builder de Grafo

Um builder poderá manter nós ainda não publicados em contexto restrito.

Ao final, valida todos os vínculos e publica o conjunto.

---

## 18.616 Publicação Coletiva

Uma operação futura poderá publicar múltiplos objetos atomicamente em um Domain.

Conceitualmente:

```text
GraphConstruction
├── reservations
├── internal identities
├── links
└── commit
```

---

## 18.617 Identidades Internas de Grafo

Durante a construção coletiva, objetos poderão referenciar handles internos.

Esses handles não deverão escapar antes do commit.

---

## 18.618 Commit do Grafo

O commit deverá:

* validar todos os objetos;
* validar vínculos;
* registrar todos para finalização;
* marcar slots vivos;
* publicar identidades externas.

---

## 18.619 Falha do Grafo

Em caso de falha:

* todos os objetos parciais serão limpos;
* slots serão cancelados;
* nenhuma identidade externa será publicada.

---

## 18.620 Escopo Inicial do Bootstrap

A publicação coletiva de grafos poderá ficar fora do primeiro bootstrap.

O subconjunto inicial poderá utilizar:

* campos opcionais;
* mutação posterior controlada;
* estruturas de biblioteca.

---

## 18.621 Auto-Referência

Um objeto que referencia a si próprio também forma ciclo.

A construção segura poderá exigir:

* campo opcional posterior;
* handle interno de construção;
* API especial.

---

## 18.622 Proibição Inicial de Auto-Referência Obrigatória

O bootstrap poderá proibir campo obrigatório que dependa da própria identidade antes da publicação.

Essa restrição deverá ser documentada.

---

## 18.623 Callbacks no Construtor

Passar `self` a callback externo durante construção será proibido, salvo callback explicitamente restrito e analisável.

---

## 18.624 Closures no Construtor

Uma closure que capture `self` não poderá escapar.

Uma closure chamada imediatamente poderá ser permitida se a análise garantir segurança.

---

## 18.625 FFI Durante Construção

O objeto incompleto não deverá ser exposto à FFI.

Podem ser passados valores temporários independentes.

---

## 18.626 Registro de Callback Externo

Registrar callback contendo `self` antes da publicação será proibido.

---

## 18.627 Threads Futuras

O objeto em construção não poderá ser enviado a outra unidade de execução.

---

## 18.628 Construção e Capacidade de Mutação

Durante construção, o construtor possui autoridade exclusiva para inicializar os campos.

Essa autoridade não é equivalente a uma Capacidade de Mutação pública comum.

---

## 18.629 Autoridade de Inicialização

A IR poderá distinguir:

```text
InitializationCapability<T>
```

de:

```text
MutationCapability<T>
```

A primeira permite escrever campos ainda não inicializados.

---

## 18.630 Escrita em Memória Não Inicializada

A operação de inicialização deverá escrever sem destruir valor anterior.

Já uma mutação comum deverá destruir ou substituir o valor existente.

---

## 18.631 Separação de Operações

A IR poderá possuir:

```text
field.init
field.assign
field.move_out
field.read
```

para preservar as diferenças.

---

## 18.632 Uso Indevido de `assign`

Não deverá ser permitido atribuir sobre campo não inicializado por operação que presume valor anterior.

---

## 18.633 Uso Indevido de `init`

Não deverá ser permitido inicializar novamente campo já inicializado.

---

## 18.634 Inicialização de Campo Herdado

A autoridade para inicializar campos da base pertence ao construtor da base.

---

## 18.635 Mutação da Base pela Derivada

Após a conclusão da base, a derivada poderá mutar campos acessíveis somente se a semântica permitir.

Isso não substitui a inicialização original.

---

## 18.636 Invariantes da Base

Quando o construtor da base termina, os invariantes da base deverão estar estabelecidos.

A derivada não deverá quebrá-los.

---

## 18.637 Invariantes da Derivada

Antes da publicação, os invariantes completos do tipo concreto deverão estar estabelecidos.

---

## 18.638 Validação Automática de Invariantes

A linguagem não precisa executar verificações automáticas de invariantes, salvo recurso explícito.

O compilador garantirá principalmente:

* inicialização;
* tipos;
* ownership;
* borrowing;
* capabilities.

---

## 18.639 Assertions de Construção

O código poderá utilizar assertions no construtor.

Falha conduzirá ao cleanup e à política de panic definida.

---

## 18.640 Contratos Futuros

Pré-condições e pós-condições poderão ser adicionadas futuramente.

---

## 18.641 Exceções e Construção

Como a Capi prioriza `Result`, o fluxo falível deverá ser explícito.

Unwinding não será necessário para o mecanismo básico.

---

## 18.642 Cleanup sem Unwinding

A IR deverá conter branches explícitos para cleanup.

---

## 18.643 Exemplo de Fluxo Falível

```text
evaluate args
    │
reserve object
    │
initialize base
    │
initialize field A
    │
initialize field B ──erro──► cleanup A
    │                         cleanup base
initialize field C           cancel reservation
    │
publish
```

---

## 18.644 Blocos de Cleanup Compartilhados

O compilador poderá compartilhar tails de cleanup.

Exemplo:

```text
cleanup_C → cleanup_B → cleanup_A → cleanup_base
```

---

## 18.645 Complexidade do Controle de Fluxo

O lowering deverá evitar explosão excessiva de blocos.

Poderão ser usados:

* discriminantes;
* tabelas de cleanup;
* funções auxiliares;
* regions na IR.

---

## 18.646 Construção de Arrays de Valores

Campos contendo arrays deverão inicializar seus elementos.

Em caso de falha no elemento `n`:

```text
destruir elementos 0..n-1 em ordem inversa
```

---

## 18.647 Construção de Coleções

Coleções de biblioteca deverão possuir suas próprias garantias de cleanup.

---

## 18.648 Construção de Tuples e Structs

Valores agregados seguirão regras semelhantes de inicialização parcial.

Quando usados como campos, seu cleanup integra o objeto externo.

---

## 18.649 Construção de Enums

Somente o payload da variante selecionada será inicializado.

---

## 18.650 Mudança de Variante Durante Construção

Não haverá variante anterior a destruir antes da primeira inicialização.

---

## 18.651 Objetos Sem Campos

Uma classe vazia ainda deverá:

* reservar slot;
* alocar representação mínima;
* inicializar cabeçalho;
* executar construtor;
* publicar identidade.

---

## 18.652 Construtor Padrão

O compilador poderá gerar construtor padrão quando:

* todos os campos possuem inicializador ou valor padrão permitido;
* a base possui construção compatível;
* nenhuma regra impede.

---

## 18.653 Valores Padrão Implícitos

A Capi não deverá presumir zero como valor padrão universal.

Valores padrão somente existirão quando definidos pela linguagem ou pelo tipo.

---

## 18.654 Derivação Automática de Construtor

Poderá ser suportada para tipos simples.

A implementação deverá gerar a mesma sequência segura.

---

## 18.655 Classes Genéricas

Após monomorfização, a construção utilizará:

* layout concreto;
* construtor especializado;
* metadados concretos;
* cleanup concreto.

---

## 18.656 Parâmetros Genéricos com Destrutor

O código especializado conhecerá se o campo exige destruição.

---

## 18.657 Code Sharing Futuro

Se código genérico for compartilhado, poderá receber descritores de construção e destruição.

Não será requisito inicial.

---

## 18.658 Classes `final`

A construção é igual à de outras classes, mas permite otimizações de chamadas após publicação.

---

## 18.659 Classes Abstratas

Sua parte base poderá ser construída dentro de classe concreta.

Não poderão ser publicadas isoladamente.

---

## 18.660 Interfaces

Interfaces não possuem construtor de instância.

A classe concreta é construída e posteriormente convertida para a interface.

---

## 18.661 Traits

Traits sem estado não participam da sequência física de construção.

Métodos auxiliares poderão ser chamados conforme a semântica.

---

## 18.662 Campos Estáticos

A inicialização de campos estáticos pertence ao ciclo de módulos, não à construção de cada objeto.

---

## 18.663 Inicialização de Módulo

O tipo concreto e seus metadados deverão estar disponíveis antes da criação de instâncias.

---

## 18.664 Dependências de Módulo

A Toolchain deverá ordenar inicializações estáticas necessárias.

---

## 18.665 Construção no Domain Raiz

Objetos de longa duração poderão ser criados no Domain raiz quando permitido.

A sequência de construção permanece igual.

---

## 18.666 Construção em Domain Filho

A identidade será associada ao filho.

Ela não poderá escapar para contexto de lifetime maior de forma inválida.

---

## 18.667 Retorno de Objeto de Domain Local

O compilador deverá rejeitar retorno quando o Domain será destruído ao fim da função.

---

## 18.668 Factory com Domain Externo

Uma factory poderá receber Domain do chamador e criar nele.

Exemplo conceitual:

```capi
fn create_person(in domain: Domain, ...) -> ObjectId<Person>
```

---

## 18.669 Lifetime do Retorno

O tipo do retorno deverá refletir sua associação ao Domain.

---

## 18.670 Construção e Borrowing

Durante a criação, argumentos emprestados não poderão ser armazenados além de seu lifetime.

---

## 18.671 Campo Borrowed

Se a linguagem permitir campos emprestados, o objeto deverá possuir lifetime compatível.

Essa funcionalidade poderá ser restrita no bootstrap.

---

## 18.672 Movimento de Argumentos

Argumentos movidos para o objeto não estarão mais disponíveis ao chamador.

---

## 18.673 Falha após Movimento de Argumento

Se a construção falhar após consumir argumento:

* o valor movido deverá ser destruído pelo cleanup do objeto parcial;
* ele não retorna automaticamente ao chamador.

Essa semântica deverá ser clara.

---

## 18.674 Preservação de Argumento em Falha

APIs que desejem preservar o valor deverão utilizar:

* empréstimo;
* clone explícito;
* retorno do valor no erro;
* builder.

---

## 18.675 `Result` com Valor Devolvido

Uma factory poderá retornar:

```text
Result<ObjectId<T>, (E, OriginalValue)>
```

quando necessário.

---

## 18.676 Construção e Allocation Failure

A falha de alocação poderá ser:

* fatal;
* `Result`;
* configurável;

conforme a política do Runtime.

A construção não poderá publicar objeto.

---

## 18.677 Reserva de Slot sem Memória

Se a memória falhar após reserva:

* o slot será cancelado;
* a geração será preservada ou incrementada conforme a política;
* nenhuma identidade será retornada.

---

## 18.678 Memória Reservada sem Publicação

Em alocador linear, bytes poderão permanecer ocupados até o fim do Domain.

Isso é desperdício interno aceitável no bootstrap.

---

## 18.679 Reutilização Futura de Buracos

Uma implementação futura poderá reutilizar espaço de construções canceladas.

Não será necessário inicialmente.

---

## 18.680 Limite de Construções Falhas

Um programa que falha repetidamente em construir objetos em Domain de longa duração poderá desperdiçar memória.

A documentação deverá alertar sobre esse comportamento inicial.

---

## 18.681 Estratégia Alternativa de Bloco Temporário

Objetos falíveis grandes poderiam ser construídos em bloco temporário e transferidos ao Domain após sucesso.

Isso aumenta complexidade e não será prioridade.

---

## 18.682 Alocação Dedicada

Objetos grandes podem utilizar bloco dedicado, permitindo liberação após falha.

---

## 18.683 Instrumentação da Construção

O Runtime poderá registrar:

* tentativas;
* sucessos;
* falhas;
* bytes reservados;
* bytes abandonados;
* tempo;
* tipo;
* Domain.

---

## 18.684 Trace de Construção

Em debug:

```text
begin construction Dog
reserve slot 42
allocate 64 bytes
initialize Animal
initialize Dog
publish ObjectId(...)
```

---

## 18.685 Trace de Falha

```text
construction Dog failed at field owner
cleanup field name
cleanup Animal
cancel slot 42
```

---

## 18.686 Localização de Origem

O código gerado poderá fornecer:

* arquivo;
* linha;
* função;
* expressão de criação.

---

## 18.687 Estatísticas por Tipo

Poderão ser medidas:

* construções;
* falhas;
* tempo médio;
* bytes;
* taxa de cancelamento.

---

## 18.688 Diagnóstico de Campo Não Inicializado

O compilador deverá indicar:

* classe;
* campo;
* caminho de controle;
* construtor;
* ponto de saída.

---

## 18.689 Diagnóstico de Escape de `self`

Deverá indicar onde o objeto incompleto tentou escapar.

---

## 18.690 Diagnóstico de Chamada Virtual Proibida

Deverá explicar que o objeto ainda não está completamente inicializado.

---

## 18.691 Diagnóstico de Delegação Cíclica

Deverá apresentar a cadeia de construtores.

---

## 18.692 Diagnóstico de Base Não Inicializada

Deverá indicar o construtor da base necessário.

---

## 18.693 Testes Unitários de Reserva

Deverão cobrir:

* reserva válida;
* cancelamento;
* publicação;
* `Drop` do guard;
* falha de memória;
* slot reutilizado;
* estado inválido.

---

## 18.694 Testes de Cabeçalho

Deverão validar:

* metadados concretos;
* estado;
* slot;
* alinhamento;
* transição para vivo.

---

## 18.695 Testes de Ordem de Construção

Para uma hierarquia:

```text
A → B → C
```

o log deverá mostrar:

```text
A fields
A constructor
B fields
B constructor
C fields
C constructor
```

conforme a regra adotada.

---

## 18.696 Testes de Ordem de Argumentos

A avaliação deverá seguir a ordem normativa.

---

## 18.697 Testes de Inicialização de Campos

Deverão cobrir:

* todos inicializados;
* campo ausente;
* branch parcial;
* loop;
* retorno precoce;
* escrita dupla;
* leitura prematura.

---

## 18.698 Testes de Construção Parcial

Cenários de falha:

* antes da base;
* durante a base;
* após a base;
* em cada campo derivado;
* no corpo do construtor;
* antes da publicação.

---

## 18.699 Testes de Cleanup

Deverão confirmar:

* ordem inversa;
* destruição única;
* recursos liberados;
* slot cancelado;
* identidade não publicada.

---

## 18.700 Testes de Movimento

Deverão validar argumentos movidos e falha posterior.

---

## 18.701 Testes de Domain

Deverão validar:

* Domain ativo;
* Domain finalizando;
* Domain destruído;
* Domain incorreto;
* lifetime do retorno.

---

## 18.702 Testes de Identidade

Duas construções bem-sucedidas deverão produzir identidades diferentes.

Uma construção falha não deverá produzir identidade válida.

---

## 18.703 Testes de Herança

Deverão confirmar:

* base construída primeiro;
* campos herdados válidos;
* metadados concretos preservados;
* nenhuma publicação da base isolada.

---

## 18.704 Testes de Classe Vazia

Mesmo sem campos, cada instância deverá possuir identidade distinta.

---

## 18.705 Testes de Construtor Falível

Resultados de erro não deverão deixar objetos vivos.

---

## 18.706 Testes de Escape

O compilador deverá rejeitar:

* retorno de `self`;
* registro global;
* callback;
* closure duradoura;
* armazenamento em outro objeto.

---

## 18.707 Testes de Builder

Builders deverão ser testados como biblioteca, não como primitiva do Runtime.

---

## 18.708 Testes de Ciclos

O bootstrap deverá testar ao menos a estratégia permitida para ciclos, como campos opcionais e mutação posterior.

---

## 18.709 Testes Diferenciais

Cranelift e LLVM deverão executar a mesma ordem e cleanup.

---

## 18.710 Golden Tests de IR

Um dump poderá mostrar:

```text
object.reserve
object.allocate
object.header.init
base.construct
field.init
constructor.call
object.publish
```

Em caminho de erro:

```text
field.drop
base.cleanup
object.cancel
```

---

## 18.711 Fuzzing de Construtores

O compilador poderá gerar:

* campos triviais;
* campos não triviais;
* branches;
* erros;
* herança;
* movimentos;
* inicializadores.

Propriedades:

* objeto publicado está completo;
* objeto falho não está vivo;
* cada recurso é destruído uma vez.

---

## 18.712 Property-Based Testing

Propriedades mínimas:

1. publicação implica todos os campos obrigatórios inicializados;
2. falha implica ausência de identidade viva;
3. cleanup ocorre em ordem inversa;
4. base é construída antes da derivada;
5. identidade publicada é estável;
6. slot cancelado não resolve;
7. objeto é registrado antes de retornar;
8. `self` não escapa antes da publicação.

---

## 18.713 Sanitizers

Deverão detectar:

* leitura não inicializada;
* uso após cancelamento;
* escrita fora do payload;
* desalinhamento;
* double drop;
* vazamento de recurso;
* ponteiro incorreto.

---

## 18.714 Modo de Construção Verificada

Em debug, o Runtime poderá manter:

* estado de cada fase;
* campos inicializados, quando disponível;
* magic numbers;
* origem;
* progresso;
* guard ativo.

---

## 18.715 Poison de Memória

Memória não inicializada poderá ser preenchida com padrão.

Após cancelamento, poderá receber outro padrão.

---

## 18.716 Validação de Publicação

Em debug, a publicação poderá verificar:

* cabeçalho;
* slot;
* estado;
* tipo;
* registro de finalização;
* marcador de inicialização completa.

---

## 18.717 Marcador de Inicialização Completa

O código gerado poderá chamar:

```text
reservation.mark_fully_initialized()
```

antes de publicar.

Essa marca não substitui a análise estática, mas ajuda a detectar erros internos.

---

## 18.718 Performance da Construção

O custo inicial incluirá:

* reserva de slot;
* alocação;
* inicialização do cabeçalho;
* execução dos inicializadores;
* registro;
* publicação.

Não deverá existir:

* reflexão;
* busca de construtor por nome;
* alocação auxiliar por campo;
* lock global no modo sequencial.

---

## 18.719 Caminho Rápido

Para construtores triviais, o backend poderá:

* inlinear reserva;
* inlinear cabeçalho;
* inicializar campos diretamente;
* chamar helper apenas para publicação.

---

## 18.720 Construção de Classe `final`

Poderá ser completamente especializada.

---

## 18.721 Eliminação de Construção

Se o objeto não for observável e não possuir efeitos, otimizações futuras poderão eliminá-lo.

---

## 18.722 Stack Allocation

Se permitido por escape analysis, a construção poderá ocorrer em armazenamento de pilha.

Ainda assim, a semântica de inicialização e cleanup permanecerá.

---

## 18.723 Ausência de Slot em Otimização

Quando a identidade não é observável, o compilador poderá eliminar slot e `ObjectId`.

Essa transformação deverá ser comprovadamente equivalente.

---

## 18.724 Scalar Replacement

Campos poderão ser mantidos em SSA sem materializar o objeto.

---

## 18.725 Construção e Devirtualização

Após construção de tipo concreto conhecido, chamadas imediatas poderão ser diretas.

---

## 18.726 Construção em Loop

O compilador deverá garantir cleanup correto em cada iteração.

O Runtime deverá evitar crescimento excessivo da estrutura de finalização por operações canceladas.

---

## 18.727 Construção em Coleções

Uma coleção poderá receber o `ObjectId` somente após publicação.

---

## 18.728 Construção em Massa

Uma API futura poderá reservar múltiplos slots e blocos.

Não será requisito inicial.

---

## 18.729 Paralelismo Futuro

Construção simultânea em Domains distintos poderá ser suportada sem compartilhamento global excessivo.

---

## 18.730 Construção no Mesmo Domain Concorrente

Exigirá sincronização futura.

O modelo inicial será sequencial ou confinado.

---

## 18.731 Reentrância

Um construtor poderá criar outros objetos.

Isso deverá ser permitido quando:

* utiliza Domain ativo;
* não publica `self`;
* cleanup é estruturado.

---

## 18.732 Construção Recursiva

Um construtor poderá chamar criação da mesma classe recursivamente.

Limites normais de pilha e lógica se aplicam.

---

## 18.733 Ordem de Registro em Construções Aninhadas

Se `A` constrói `B` antes de `A` ser publicado:

* `B` poderá ser publicado e registrado;
* se `A` falhar, o destino de `B` deverá ser definido.

---

## 18.734 Objeto Filho Criado Durante Construção

Existem duas possibilidades:

### `B` é campo owned por valor

Será limpo com `A`.

### `B` é objeto por identidade no mesmo Domain

Sua identidade já pode estar viva independentemente.

Isso pode causar efeito persistente se `A` falhar.

---

## 18.735 Construções Aninhadas e Transacionalidade

A construção de `A` não deverá presumir rollback automático de outros objetos já publicados.

Para atomicidade de grafo, será necessário builder ou publicação coletiva.

---

## 18.736 Recomendação

Factories complexas deverão criar recursos auxiliares de modo que possam destruí-los explicitamente em caso de falha.

---

## 18.737 Destruição Antecipada de Dependência

Se `B` foi criado exclusivamente para `A` e `A` falha, a factory poderá destruir `B` antecipadamente quando permitido.

---

## 18.738 Identidades Compartilhadas

Se `B` já era existente ou compartilhado, não deverá ser destruído.

---

## 18.739 Ownership Semântico

O compilador e as APIs deverão distinguir:

* identidade apenas referenciada;
* recurso possuído pelo valor;
* objeto criado para lifetime do Domain;
* dependência externa.

---

## 18.740 Construção e Domains Aninhados

Uma factory poderá criar Domain filho temporário.

Se falhar, destruirá o filho integralmente.

Isso pode oferecer transacionalidade prática.

---

## 18.741 Domain Temporário de Construção

Uma estratégia futura poderá construir grafo em Domain temporário e transferi-lo.

Entretanto, transferência de identidade é complexa.

Não será assumida inicialmente.

---

## 18.742 Construção no Domain Final

A estratégia mais simples permanece construir diretamente no Domain final e executar cleanup explícito.

---

## 18.743 Runtime Helpers Iniciais

O Runtime poderá expor internamente:

```text
capi_rt_object_reserve
capi_rt_object_allocate
capi_rt_object_init_header
capi_rt_object_mark_initialized
capi_rt_object_publish
capi_rt_object_cancel
```

Algumas operações poderão ser combinadas.

---

## 18.744 Helper Combinado de Reserva e Alocação

Uma API possível:

```text
capi_rt_object_begin(domain, type_metadata) -> reservation
```

Ela poderá:

* reservar slot;
* alocar;
* inicializar cabeçalho;
* retornar ponteiro.

---

## 18.745 Helper de Publicação

```text
capi_rt_object_commit(reservation) -> ObjectId
```

---

## 18.746 Helper de Cancelamento

```text
capi_rt_object_abort(reservation)
```

O cleanup de campos deverá ocorrer antes ou ser informado ao helper.

---

## 18.747 Responsabilidade do Runtime

O Runtime será responsável por:

* slot;
* memória;
* cabeçalho;
* registro;
* publicação;
* cancelamento estrutural.

---

## 18.748 Responsabilidade do Código Gerado

O código gerado será responsável por:

* ordem de argumentos;
* chamada da base;
* inicialização dos campos;
* corpo dos construtores;
* cleanup dos valores;
* propagação de erros;
* garantia de publicação tardia.

---

## 18.749 Responsabilidade do Compilador

O compilador deverá validar:

* constructor resolution;
* campos obrigatórios;
* fluxo;
* escape;
* lifetime;
* mutabilidade;
* ownership;
* assinatura;
* hierarquia.

---

## 18.750 HIR de Construção

A HIR poderá representar:

```text
ConstructObject
├── concrete_type
├── domain
├── constructor
├── arguments
└── failure_mode
```

---

## 18.751 Lowering para Sequência Explícita

O Middle-end transformará essa operação em:

```text
reserve
initialize base
initialize fields
execute constructor
publish
cleanup blocks
```

---

## 18.752 Representação de Campos Inicializados na IR

A IR poderá associar drop flags a temporários e campos.

---

## 18.753 Drop Flags

Uma drop flag indica se um valor deve ser destruído.

Ela poderá ser:

* eliminada por análise;
* representada em SSA;
* armazenada em variável local;
* combinada em discriminante.

---

## 18.754 Validação da IR

O validador deverá rejeitar:

* `publish` antes de `mark_initialized`;
* uso comum antes da publicação;
* cancelamento após publicação;
* dupla publicação;
* dupla inicialização;
* campo lido antes de `init`;
* reserva sem caminho de publish ou cancel.

---

## 18.755 Caminhos de Controle

Toda reserva deverá terminar em exatamente uma das operações:

```text
publish
cancel
```

salvo abortamento fatal do processo.

---

## 18.756 Dominância

A publicação deverá dominar todos os usos públicos do `ObjectId`.

---

## 18.757 Post-dominância de Cleanup

Em caminhos falíveis, o cleanup adequado deverá pós-dominar a falha.

---

## 18.758 Verificação de Escape na HIR

O Frontend ou Middle-end deverá marcar valores de construção como não escapáveis.

---

## 18.759 Tipo Interno de Reserva

A reserva deverá ser linear:

* não copiável;
* consumida uma vez;
* não armazenável em estruturas comuns;
* restrita ao código gerado ou Runtime.

---

## 18.760 RAII no Runtime Rust

`ObjectReservation` poderá possuir `Drop` que cancela reserva não publicada.

Isso protege contra erros internos no Runtime.

---

## 18.761 Limite do RAII

O cleanup semântico dos campos Capi não deverá depender apenas do `Drop` Rust.

O código gerado deverá executá-lo explicitamente.

---

## 18.762 Panic Rust

Funções de Runtime não deverão deixar panic atravessar a ABI.

---

## 18.763 Testes de RAII Interno

Deverão validar que uma reserva abandonada em código Rust de teste é cancelada.

---

## 18.764 ABI de Construtor

Construtores poderão ser compilados como funções internas:

```text
construct_Dog(
    reservation_pointer,
    arguments...
) -> Result<Unit, E>
```

ou:

```text
construct_Dog(
    self_pointer,
    arguments...
) -> Result<Unit, E>
```

---

## 18.765 Retorno do Construtor Interno

O construtor interno não deverá retornar `ObjectId`.

A publicação será responsabilidade do wrapper de criação.

---

## 18.766 Wrapper de Criação

Uma função gerada poderá:

1. avaliar argumentos;
2. iniciar reserva;
3. chamar construtor interno;
4. publicar em sucesso;
5. cancelar em erro;
6. retornar `Result<ObjectId<T>, E>`.

---

## 18.767 Vantagem do Wrapper

Essa separação impede que o corpo do construtor publique o objeto diretamente.

---

## 18.768 Construção da Base por Função Interna

O construtor de `Dog` poderá chamar:

```text
construct_Animal_part(self_pointer, base_args)
```

Essa função inicializará somente a região da base.

---

## 18.769 Ausência de Nova Reserva para a Base

A base não deverá reservar slot ou memória adicional.

Ela atua sobre o mesmo objeto.

---

## 18.770 Self da Base

A função receberá o mesmo ponteiro lógico de payload ou objeto.

---

## 18.771 Metadados Durante Base

Continuarão sendo os metadados concretos de `Dog`.

A restrição de chamadas virtuais evita comportamento perigoso.

---

## 18.772 Cleanup da Base

Deverá existir operação para desfazer somente a parte base construída.

---

## 18.773 Múltiplos Construtores da Base

Cada caminho deverá produzir estado final compatível da base.

---

## 18.774 Invariantes de Construção

A implementação deverá preservar:

1. todo objeto publicado está completamente inicializado;
2. nenhum objeto parcial possui `ObjectId` público;
3. a base é inicializada antes da derivada;
4. cada campo obrigatório é inicializado exatamente uma vez;
5. nenhum campo é lido antes da inicialização;
6. cleanup ocorre em ordem inversa;
7. cada valor inicializado é destruído no máximo uma vez;
8. falha não produz objeto vivo;
9. publicação registra o objeto para destruição;
10. casts e chamadas comuns somente ocorrem após publicação;
11. o tipo concreto permanece constante;
12. a identidade publicada utiliza o slot reservado;
13. o Domain permanece ativo durante toda a construção;
14. a reserva termina em publicação ou cancelamento;
15. `self` não escapa antes da publicação.

---

## 18.775 ADRs Recomendados

Deverão ser considerados:

```text
ADR — Publicação tardia da identidade
ADR — ObjectReservation linear
ADR — Base construída antes da derivada
ADR — Inicialização de campos em ordem de declaração
ADR — Construtores internos sem retorno de ObjectId
ADR — Wrapper responsável por commit e cancelamento
ADR — Cleanup estático de construção parcial
ADR — Restrição de chamadas virtuais em construtores
ADR — Proibição de escape de self
ADR — Estratégia inicial para grafos cíclicos
ADR — Separação entre InitializationCapability e MutationCapability
```

---

## 18.776 Critérios de Conclusão da Construção Básica

A construção básica estará concluída quando:

* slots puderem ser reservados;
* armazenamento puder ser alocado;
* cabeçalhos puderem ser inicializados;
* classes sem herança puderem ser construídas;
* campos triviais e não triviais puderem ser inicializados;
* objetos puderem ser publicados;
* `ObjectId` válido puder ser retornado;
* construções falhas puderem ser canceladas;
* nenhum objeto parcial puder ser resolvido;
* testes de sanitizers passarem.

---

## 18.777 Critérios de Conclusão da Construção com Herança

Estará concluída quando:

* construtores da base forem encadeados;
* a base utilizar a mesma alocação;
* a ordem base-derivada for preservada;
* cleanup parcial da base funcionar;
* campos herdados permanecerem válidos;
* metadados concretos forem mantidos;
* chamadas virtuais inseguras forem rejeitadas;
* classes abstratas puderem participar como base;
* testes de herança profunda passarem.

---

## 18.778 Critérios de Conclusão da Construção Falível

Estará concluída quando:

* construtores puderem retornar erro;
* propagação de erro gerar cleanup;
* temporários forem destruídos;
* campos inicializados forem destruídos em ordem inversa;
* argumentos movidos forem tratados corretamente;
* o slot for cancelado;
* nenhuma identidade for publicada;
* falhas em cada etapa forem testadas.

---

## 18.779 Critérios de Conclusão para o Bootstrap

O suporte de construção estará pronto para o bootstrap quando:

* a Biblioteca Mínima puder construir suas classes;
* objetos puderem conter valores não triviais;
* objetos puderem referenciar outras identidades;
* Domains explícitos forem suportados;
* construtores genéricos monomorfizados funcionarem;
* Cranelift gerar todo o fluxo;
* LLVM utilizar a mesma ABI;
* diagnósticos de inicialização estiverem disponíveis;
* builders puderem ser implementados em Capi;
* o compilador escrito em Capi puder construir seus próprios grafos de objetos.

---

## 18.780 Resultado Esperado da Terceira Parte — Bloco 1

Ao final desta etapa, a implementação oficial deverá possuir um processo de construção no qual:

```text
Domain
    fornece o contexto de lifetime

ObjectReservation
    reserva slot e armazenamento sem publicar identidade

ObjectHeader
    registra o tipo concreto e o estado de inicialização

Construtores
    inicializam base, campos e derivada

Initialization Analysis
    impede leitura ou escape prematuro

Cleanup
    desfaz apenas as partes concluídas

Publish'
    transforma a reserva em objeto vivo

ObjectId
    somente existe publicamente após o sucesso
```

O mecanismo deverá garantir que a construção de objetos permaneça:

* determinística;
* auditável;
* segura diante de falhas;
* compatível com herança;
* integrada aos Domains;
* independente de GC;
* independente de ARC;
* adequada à compilação AOT;
* preparada para Cranelift e LLVM.

A parte seguinte deverá definir a sequência inversa: destruição, finalização, cleanup de objetos vivos, destrutores de hierarquia, liberação de recursos e integração com a destruição coletiva dos Domains.

---

## Terceira Parte — Bloco 2 — Destruição, Finalização e Cleanup Determinístico

## 18.781 Objetivo

Esta parte define a estratégia inicial para materializar, na implementação oficial da Capi:

* destruição determinística de objetos;
* finalização de recursos;
* ordem de destruição;
* destrutores de classes;
* destruição de campos;
* destruição de hierarquias;
* cleanup de objetos vivos;
* cleanup de objetos parcialmente construídos;
* destruição antecipada;
* destruição coletiva de Domains;
* invalidação de identidades;
* tratamento de ciclos;
* integração com `ObjectId`;
* interação com Mutation Capability;
* lowering para Runtime;
* integração com Cranelift e LLVM;
* testes, diagnósticos e invariantes.

A destruição deverá encerrar corretamente o lifetime sem depender de:

* Garbage Collection;
* contagem de referências;
* análise dinâmica de alcançabilidade;
* finalização não determinística;
* thread de coleta;
* tracing de grafos.

---

## 18.782 Princípio da Destruição Determinística

A implementação deverá preservar o seguinte princípio:

> Quando o lifetime de um objeto ou de seu Domain termina, todos os recursos sob sua responsabilidade deverão ser finalizados em ordem definida, exatamente uma vez e antes de a identidade deixar de ser válida.

---

## 18.783 Separação entre Lifetime, Finalização e Liberação Física

A implementação deverá distinguir:

```text
fim do lifetime lógico
finalização semântica
destruição dos campos
invalidação da identidade
liberação ou abandono da memória física
```

Essas etapas podem ocorrer em sequência, mas não são semanticamente idênticas.

---

## 18.784 Fim do Lifetime Lógico

O lifetime lógico termina quando o objeto deixa de poder ser utilizado por código Capi seguro.

Após esse ponto:

* novas resoluções de `ObjectId` deverão falhar;
* novos empréstimos não poderão ser criados;
* métodos comuns não poderão ser chamados;
* referências externas não poderão reativar o objeto.

---

## 18.785 Finalização Semântica

A finalização semântica encerra recursos e efeitos associados ao objeto.

Ela poderá incluir:

* fechamento de arquivo;
* liberação de handle;
* flush;
* término de sessão;
* cancelamento de registro externo;
* destruição de buffers;
* execução de lógica definida pela classe.

---

## 18.786 Destruição dos Campos

Após ou durante a finalização da classe, os campos inicializados deverão ser destruídos conforme:

* ordem definida;
* tipo;
* ownership;
* trivialidade;
* estado de inicialização.

---

## 18.787 Invalidação da Identidade

O slot e a geração deverão ser atualizados de modo que o `ObjectId` antigo não volte a resolver.

---

## 18.788 Liberação Física da Memória

Em Domains com alocação linear, a memória física poderá ser liberada coletivamente ao final do Domain.

Na destruição individual, os bytes poderão permanecer reservados.

Isso não preserva o lifetime lógico do objeto.

---

## 18.789 Estados Conceituais da Destruição

Um objeto vivo poderá atravessar:

```text
Alive
DestroyRequested
Finalizing
DroppingDerivedFields
DroppingBase
Destroyed
Invalidated
```

Em destruição coletiva, estados equivalentes poderão ser processados em lote.

---

## 18.790 Estado `Alive`

Nesse estado:

* o slot resolve normalmente;
* métodos podem ser chamados;
* campos podem ser acessados conforme autoridade;
* o objeto participa da ordem de destruição do Domain.

---

## 18.791 Estado `DestroyRequested`

Esse estado poderá existir quando a destruição foi solicitada, mas ainda não iniciou.

Ele será útil para:

* prevenir solicitações duplicadas;
* coordenar destruição antecipada;
* lidar com callbacks;
* detectar reentrância.

---

## 18.792 Estado `Finalizing`

Durante esse estado:

* o destrutor da classe concreta poderá executar;
* o objeto não deverá ser publicado novamente;
* novos acessos comuns deverão ser bloqueados;
* apenas operações internas de finalização serão permitidas.

---

## 18.793 Estado `DroppingDerivedFields`

Os campos declarados na classe concreta serão destruídos em ordem inversa à inicialização.

---

## 18.794 Estado `DroppingBase`

A parte herdada será destruída após a parte derivada.

Para:

```text
C extends B
B extends A
```

a ordem será:

```text
C
B
A
```

---

## 18.795 Estado `Destroyed`

Nesse estado:

* destrutores já terminaram;
* campos foram destruídos;
* recursos não deverão permanecer;
* a identidade ainda poderá aguardar invalidação física do slot.

---

## 18.796 Estado `Invalidated`

Nesse estado:

* o slot não resolve;
* a geração foi atualizada ou o slot foi removido;
* o objeto não pode ser observado;
* a memória poderá permanecer no bloco do Domain.

---

## 18.797 Tipos de Destruição

A implementação deverá distinguir pelo menos:

```text
destruição normal de objeto vivo
destruição antecipada
destruição coletiva do Domain
cleanup de construção parcial
destruição de valor inline
destruição por falha de operação
destruição de temporário
```

---

## 18.798 Destruição Normal

A destruição normal ocorrerá quando o lifetime do objeto termina conforme a semântica da linguagem.

---

## 18.799 Destruição Antecipada

A destruição antecipada poderá ocorrer quando a linguagem ou biblioteca permitir encerrar um objeto antes do Domain.

Ela deverá invalidar a identidade imediatamente.

---

## 18.800 Destruição Coletiva

Quando um Domain for destruído, todos os objetos vivos pertencentes a ele deverão ser finalizados.

A memória será liberada coletivamente.

---

## 18.801 Cleanup de Construção Parcial

Objetos que falharam durante construção não utilizarão o destrutor completo do tipo concreto.

Somente as partes efetivamente inicializadas serão destruídas.

---

## 18.802 Destruição de Valor Inline

Structs, tuples, arrays, enums e outros valores não triviais poderão exigir drop próprio.

Esse drop integrará o fluxo do objeto ou valor que os contém.

---

## 18.803 Destrutor

Um destrutor é a rotina associada à finalização de uma instância.

A sintaxe e semântica públicas serão definidas pela especificação.

A implementação poderá representá-lo internamente como função semelhante a:

```text
destroy_T(self_pointer, runtime_context)
```

---

## 18.804 Destrutor de Classe Concreta

Cada classe concreta que necessitar finalização poderá possuir destrutor próprio.

Mesmo classes sem destrutor explícito poderão exigir drop glue gerado.

---

## 18.805 Drop Glue

Drop glue é código gerado pelo compilador para:

* chamar destrutor explícito;
* destruir campos;
* encadear a base;
* atualizar estado;
* integrar-se ao Runtime.

---

## 18.806 Destrutor Explícito e Drop Glue

A rotina final poderá ser composta por:

```text
destructor body
        │
        ▼
drop fields
        │
        ▼
drop base
```

A ordem exata deverá ser definida normativamente.

---

## 18.807 Estratégia Recomendada de Ordem

Recomenda-se:

1. marcar o objeto como `Finalizing`;
2. executar o corpo do destrutor da classe concreta;
3. destruir campos declarados pela classe concreta em ordem inversa;
4. executar a finalização da base;
5. repetir até a classe raiz;
6. marcar como `Destroyed`;
7. invalidar o slot.

---

## 18.808 Razão para Derivada Antes da Base

A parte derivada poderá depender da base ainda válida durante seu destrutor.

A base não deverá ser destruída antes da derivada concluir sua finalização.

---

## 18.809 Destruição de Campos em Ordem Inversa

Se os campos foram inicializados na ordem:

```text
field_a
field_b
field_c
```

deverão ser destruídos em:

```text
field_c
field_b
field_a
```

---

## 18.810 Ordem entre Corpo do Destrutor e Campos

A estratégia inicial deverá manter os campos da classe disponíveis durante o corpo do destrutor.

Portanto:

```text
destructor body
        │
        ▼
field destruction
```

---

## 18.811 Alternativa Não Recomendada

Destruir campos antes do corpo tornaria o destrutor incapaz de consultá-los com segurança.

Essa estratégia não será usada no bootstrap.

---

## 18.812 Destrutor da Base

Após a destruição dos campos da derivada, o drop glue deverá invocar a rotina da base.

---

## 18.813 Ausência de Nova Identidade para a Base

A base é parte do mesmo objeto.

Seu destrutor opera sobre:

* o mesmo slot;
* a mesma geração;
* a mesma alocação;
* a mesma identidade em finalização.

---

## 18.814 Metadados Durante Destruição

Os metadados no cabeçalho continuarão representando o tipo concreto original.

Eles não deverão mudar para a classe base.

---

## 18.815 Despacho Virtual Durante Destruição

Chamadas virtuais em destrutores podem acessar partes já destruídas ou em destruição.

A implementação inicial deverá restringi-las.

---

## 18.816 Estratégia Conservadora para Chamadas em Destrutores

Durante a finalização, deverão ser permitidas preferencialmente:

* chamadas estáticas;
* métodos privados;
* chamadas qualificadas à implementação atual;
* operações explicitamente seguras para finalização.

Chamadas virtuais comuns sobre `self` deverão ser proibidas ou limitadas pela especificação.

---

## 18.817 Ausência de Troca de VTable

A implementação inicial não deverá trocar vtables durante a destruição.

Isso evita:

* estado mutável no cabeçalho;
* regras complexas;
* divergência entre backends;
* comportamento implícito.

---

## 18.818 Destrutores Virtuais

A destruição iniciada por referência de base deverá selecionar o destrutor do tipo concreto.

Essa seleção ocorrerá por `TypeMetadata`, não necessariamente por slot comum da vtable.

---

## 18.819 Ponteiro de Destrutor nos Metadados

Recomenda-se:

```text
TypeMetadata.destructor
```

contendo a rotina de destruição completa do tipo concreto.

---

## 18.820 Destruição por Tipo Base

Para:

```text
ObjectId<Animal>
```

contendo objeto `Dog`, a destruição deverá chamar:

```text
destroy_Dog
```

---

## 18.821 Destrutor Gerado Mesmo sem Corpo Explícito

Uma classe sem destrutor explícito ainda poderá precisar destruir:

* campos por valor;
* buffers;
* strings;
* structs;
* handles;
* parte base.

---

## 18.822 Destruição Trivial

Um tipo será trivialmente destrutível quando:

* não possui destrutor explícito;
* todos os campos são trivialmente destrutíveis;
* a base é trivialmente destrutível;
* não possui recurso externo;
* nenhuma ação de finalização é necessária.

---

## 18.823 Flag de Trivialidade

O `TypeMetadata` poderá conter:

```text
is_trivially_destructible
```

Essa informação permitirá omitir chamadas.

---

## 18.824 Classe Vazia

Uma classe vazia poderá ter destruição trivial, mas sua identidade ainda deverá ser invalidada.

---

## 18.825 Valores Triviais

Primitivos normalmente não exigirão drop.

---

## 18.826 Valores Não Triviais

Tipos como:

* `String`;
* `Vector`;
* `Map`;
* buffers;
* handles;

poderão exigir drop específico.

---

## 18.827 Campos de Identidade

Um campo contendo `ObjectId<T>` não deverá destruir o objeto referido automaticamente.

O campo armazena identidade, não ownership exclusivo do objeto.

---

## 18.828 Ausência de Contagem de Referências

Remover um `ObjectId` de campo:

* não decrementa contador;
* não destrói objeto referido;
* não executa tracing.

---

## 18.829 Objetos Referidos

Objetos referidos serão destruídos conforme o lifetime de seus Domains ou por destruição antecipada explícita.

---

## 18.830 Campo Opcional de Identidade

Destruir `Optional<ObjectId<T>>` apenas destrói seu valor de representação, que normalmente será trivial.

---

## 18.831 Campo por Valor

Um campo por valor com drop deverá ser destruído como parte do objeto.

---

## 18.832 Campo de Recurso

Um campo que representa ownership de recurso deverá executar sua rotina de drop.

---

## 18.833 Campos Mutáveis

A mutabilidade não altera a ordem de destruição.

O drop utilizará o valor presente no momento final.

---

## 18.834 Campo Movido

Um campo movido para fora não deverá ser destruído novamente.

Entretanto, um objeto publicado não poderá permanecer com campo obrigatório inválido, salvo estado tipado ou operação de consumo.

---

## 18.835 Operação de Consumo

Métodos consumidores poderão tomar o objeto ou um campo.

A semântica deverá definir se o objeto:

* é destruído ao final;
* muda de estado;
* se torna inválido;
* exige substituição do campo.

---

## 18.836 Drop Flags

Quando um valor puder ter sido movido, o código gerado poderá utilizar drop flag para evitar double drop.

---

## 18.837 Drop Flags em Objetos Vivos

O design deverá evitar objetos publicados com muitos campos condicionalmente inicializados.

Quando necessário, a representação deverá usar tipos como:

```text
Optional<T>
```

em vez de estado oculto de inicialização.

---

## 18.838 Destrutores Falíveis

A estratégia inicial deverá proibir ou desencorajar destrutores que retornem erro.

A destruição precisa progredir mesmo quando uma ação externa falha.

---

## 18.839 Falha Externa no Destrutor

Operações como fechamento ou flush podem falhar.

A API deverá preferir um método explícito anterior, por exemplo:

```text
close() -> Result
```

O destrutor poderá executar fallback sem propagar erro recuperável.

---

## 18.840 Política de Erro em Destrutor

O destrutor poderá:

* ignorar erro documentadamente;
* registrar diagnóstico;
* abortar em condição crítica;
* usar callback de Runtime.

Não deverá retornar `Result` para o fluxo comum de destruição automática.

---

## 18.841 Panic em Destrutor

Panic durante destruição é perigoso.

A política inicial deverá:

* impedir unwinding através do Runtime;
* registrar o contexto;
* continuar somente se explicitamente seguro;
* abortar em caso de estado inconsistente.

---

## 18.842 Destruição Durante Outro Panic

Como o modelo inicial não depende de unwinding, essa situação deverá ser rara.

Se ocorrer, a Toolchain poderá abortar imediatamente.

---

## 18.843 Reentrância do Destrutor

Um destrutor não deverá solicitar novamente a destruição do mesmo objeto.

O estado `Finalizing` deverá detectar essa situação.

---

## 18.844 Destruição Recursiva

Um destrutor poderá destruir explicitamente outro objeto quando permitido.

Isso deverá respeitar:

* Domain;
* autoridade;
* estado;
* ausência de dependência futura.

---

## 18.845 Ciclo de Destruição Explícita

Se `A` destrói `B` e `B` tenta destruir `A`, o Runtime deverá detectar estados de finalização.

---

## 18.846 Ressurreição

Um objeto em finalização não poderá ser reintroduzido como objeto vivo.

Portanto, destrutores não poderão:

* republicar `self`;
* registrar nova identidade;
* cancelar a destruição;
* armazenar `self` em estrutura viva.

---

## 18.847 Proibição de Ressurreição

A proibição simplifica:

* invariantes;
* slots;
* geração;
* Domains;
* finalização coletiva.

---

## 18.848 Escape de `self` no Destrutor

O compilador deverá rejeitar ou restringir:

* retorno de `self`;
* armazenamento global;
* callback duradouro;
* envio a outro Domain;
* criação de nova referência persistente.

---

## 18.849 Uso Local de `self`

O corpo do destrutor poderá ler campos ainda válidos e executar operações locais permitidas.

---

## 18.850 Mutação Durante Destruição

O destrutor poderá alterar seus próprios campos quando necessário à finalização.

Essa mutação ocorre sob autoridade exclusiva interna.

---

## 18.851 Mutation Capability na Destruição

A finalização não deverá exigir que o chamador forneça Capacidade de Mutação comum.

O Runtime e o drop glue possuem autoridade interna de destruição.

---

## 18.852 Autoridade de Destruição

A IR poderá representar uma capacidade interna:

```text
DestructionCapability<T>
```

Ela permite:

* acessar campos;
* mover recursos para cleanup;
* marcar estado;
* destruir valores.

---

## 18.853 Capacidade Não Exposta

Essa autoridade não deverá estar disponível a código Capi comum.

---

## 18.854 Destruição Antecipada

Uma API de destruição antecipada poderá ter forma conceitual:

```capi
destroy object;
```

ou operação do Domain.

A sintaxe definitiva será definida posteriormente.

---

## 18.855 Pré-condições da Destruição Antecipada

A operação deverá exigir:

* objeto vivo;
* Domain ativo;
* autoridade apropriada;
* ausência de empréstimos incompatíveis;
* ausência de uso posterior válido;
* regras de ownership satisfeitas.

---

## 18.856 Consumo da Identidade

A destruição antecipada deverá consumir a identidade ou invalidar seu uso no fluxo.

---

## 18.857 Cópias de `ObjectId`

Se outras cópias da identidade existirem, elas se tornarão obsoletas após a destruição.

Ao tentar resolver, deverão falhar por geração ou estado.

---

## 18.858 Segurança Estática e Dinâmica

O compilador deverá prevenir usos posteriores conhecidos.

O Runtime protegerá contra aliases persistentes que ainda carreguem a identidade antiga.

---

## 18.859 Invalidação Antes ou Depois do Destrutor

Duas estratégias:

### Invalidar antes da finalização

Impede novas resoluções durante o destrutor.

### Invalidar depois da finalização

Mantém o slot resolvível internamente.

Recomenda-se separar:

* acesso externo bloqueado antes;
* invalidação final da geração depois.

---

## 18.860 Estado `Finalizing` como Bloqueio Externo

Ao iniciar a destruição:

* o slot permanece associado internamente;
* resolução comum rejeita o estado;
* drop glue ainda acessa o ponteiro diretamente.

---

## 18.861 Atualização da Geração

Após a conclusão da destruição, a geração deverá ser incrementada ou substituída.

---

## 18.862 Overflow de Geração

A política de geração deverá evitar que uma identidade muito antiga volte a se tornar válida após overflow.

Possibilidades:

* geração ampla;
* aposentadoria do slot;
* detecção de overflow;
* nunca reutilizar o slot após limite.

---

## 18.863 Slot Destruído

Após invalidação, o slot poderá:

* entrar na free list;
* permanecer aposentado;
* aguardar destruição do Domain.

---

## 18.864 Reutilização de Slot

Um novo objeto poderá reutilizar o índice, mas com geração diferente.

---

## 18.865 Memória do Objeto Anterior

O slot reutilizado poderá apontar para nova alocação.

O endereço anterior não deverá ser usado.

---

## 18.866 Poison após Destruição

Em debug, a memória do objeto destruído poderá ser preenchida com padrão.

---

## 18.867 Cabeçalho após Destruição

Em debug, o cabeçalho poderá ser marcado:

```text
Destroyed
```

Mesmo que o slot já esteja invalidado.

---

## 18.868 Destruição Coletiva do Domain

A destruição de um Domain deverá:

1. impedir novas alocações;
2. impedir novas publicações;
3. marcar estado `Finalizing`;
4. determinar a ordem de objetos;
5. finalizar objetos vivos;
6. invalidar slots;
7. destruir filhos conforme política;
8. liberar blocos;
9. marcar Domain como `Destroyed`.

---

## 18.869 Bloqueio de Novas Operações

Ao iniciar a finalização do Domain:

* novas construções deverão falhar;
* novos empréstimos deverão ser rejeitados;
* transferências deverão ser bloqueadas;
* publicação pendente deverá ser resolvida ou cancelada.

---

## 18.870 Reservas em Aberto

O Domain não deverá ser finalizado enquanto existirem construções ativas, salvo cancelamento forçado interno.

---

## 18.871 Política para Reservas Ativas

A implementação inicial deverá exigir que:

* toda reserva seja publicada ou cancelada;
* guards internos sejam resolvidos;
* a finalização detecte inconsistências.

---

## 18.872 Ordem de Destruição dos Objetos

A estratégia inicial recomendada será a ordem inversa de publicação:

```text
último objeto publicado
        │
        ▼
primeiro objeto publicado
```

---

## 18.873 Razão da Ordem Inversa

Essa ordem é compatível com o padrão:

```text
construir dependência
construir dependente
```

O dependente será destruído antes da dependência.

---

## 18.874 Limitação da Ordem Inversa

A ordem inversa não representa análise completa de dependências.

O programa não deverá depender de inferência dinâmica de grafo.

---

## 18.875 Dependências entre Objetos

Se a ordem de destruição for semanticamente importante, a aplicação deverá:

* construir na ordem apropriada;
* usar Domains aninhados;
* realizar close explícito;
* usar tipos de recurso;
* evitar dependência em destrutor de objeto arbitrário.

---

## 18.876 Grafos Cíclicos

Ciclos de `ObjectId` não impedem a destruição coletiva.

O Domain não verifica alcançabilidade.

Ele percorre seus objetos registrados e os destrói.

---

## 18.877 Ausência de Tracing

Durante a destruição:

* campos de identidade não serão percorridos para determinar liveness;
* ciclos não serão detectados como lixo;
* referências não controlarão a ordem automaticamente.

---

## 18.878 Identidades para Objetos Já Destruídos

Enquanto o Domain destrói objetos, um objeto finalizado pode ainda ser referenciado por outro ainda não finalizado.

A resolução comum deverá rejeitar a identidade destruída.

---

## 18.879 Acesso a Dependência Durante Finalização

Um destrutor somente poderá usar outro objeto se ele ainda estiver vivo.

A ordem de construção deverá favorecer isso.

---

## 18.880 Diagnóstico de Dependência Inválida

Em debug, uma tentativa de resolver objeto já destruído durante finalização deverá mostrar:

* objeto solicitante;
* objeto alvo;
* ordem de criação;
* estado;
* Domain.

---

## 18.881 Objetos Destruídos Antecipadamente no Domain

A ordem de finalização deverá ignorar slots já destruídos.

---

## 18.882 Registro de Ordem

O Domain poderá manter vetor ou lista de slots publicados.

---

## 18.883 Entradas Obsoletas na Ordem

Se objetos forem destruídos antecipadamente, a entrada poderá permanecer como tombstone.

Durante destruição coletiva será ignorada.

---

## 18.884 Compactação da Ordem

A primeira versão não precisará compactar o vetor de publicação.

---

## 18.885 Custo de Tombstones

Domains de longa duração com muita destruição antecipada podem acumular entradas.

Isso deverá ser medido.

---

## 18.886 Estratégia Futura de Compactação

Possibilidades:

* compactação periódica;
* lista duplamente ligada;
* índice de objetos vivos;
* generations na ordem;
* segmentos.

---

## 18.887 Domains Filhos

A política deverá definir se filhos são destruídos:

* antes dos objetos do pai;
* depois;
* conforme ordem de criação.

Recomenda-se destruir Domains filhos antes da liberação final do pai.

---

## 18.888 Relação Pai-Filho

Um Domain pai não poderá ser destruído deixando filho vivo.

---

## 18.889 Ordem entre Filhos e Objetos

A especificação de implementação deverá escolher uma ordem determinística.

Uma estratégia recomendada:

1. bloquear pai;
2. destruir filhos em ordem inversa de criação;
3. destruir objetos do pai em ordem inversa de publicação;
4. liberar blocos do pai.

---

## 18.890 Motivo para Filhos Primeiro

Objetos do pai podem atuar como recursos globais usados por objetos do filho.

Destruir filhos primeiro mantém recursos do pai disponíveis.

---

## 18.891 Alternativa

Alguns modelos podem preferir intercalar filhos e objetos por ordem temporal.

Essa complexidade não será necessária inicialmente.

---

## 18.892 Domain Raiz

O Domain raiz será destruído no encerramento do processo ou contexto principal.

---

## 18.893 Encerramento do Processo

A sequência deverá permitir:

* finalização da aplicação;
* flush de streams;
* destruição dos Domains;
* emissão de diagnósticos;
* retorno do código de saída.

---

## 18.894 Falha Fatal Durante Destruição do Domain

O Runtime deverá escolher entre:

* continuar com os próximos objetos;
* abortar imediatamente;
* registrar e continuar apenas para drops triviais.

A estratégia inicial mais segura é abortar em corrupção estrutural e registrar falhas externas recuperáveis.

---

## 18.895 Destrutores de Recursos Externos

Devem evitar comprometer a destruição de objetos restantes.

---

## 18.896 Close Explícito

Recursos cuja finalização pode falhar deverão oferecer:

```text
close() -> Result
```

O drop posterior deverá ser idempotente ou reconhecer estado fechado.

---

## 18.897 Estado `Closed`

Objetos de recurso poderão possuir estado lógico fechado.

O destrutor verificará se ainda há recurso ativo.

---

## 18.898 Idempotência de Fechamento

`close` poderá ser idempotente se definido pela API.

O destrutor não deverá fechar duas vezes.

---

## 18.899 Destruição de Buffers

Buffers owned deverão liberar armazenamento conforme sua implementação.

Se o armazenamento estiver no próprio Domain, a rotina poderá apenas destruir elementos.

---

## 18.900 Coleções no Domain

Uma coleção por valor dentro de objeto deverá destruir seus elementos.

Objetos por identidade contidos não serão destruídos pela coleção.

---

## 18.901 `Vector<ObjectId<T>>`

Seu drop libera o buffer do vetor, mas não os objetos referidos.

---

## 18.902 `Vector<ValueT>`

Seu drop deverá destruir cada elemento não trivial e liberar o buffer.

---

## 18.903 Ordem dos Elementos

Elementos poderão ser destruídos em ordem inversa, conforme a Biblioteca Padrão.

---

## 18.904 Arrays Fixos

Arrays inline deverão destruir elementos conforme regra definida.

A estratégia inicial deverá utilizar ordem inversa.

---

## 18.905 Maps e Sets

Deverão destruir chaves e valores owned conforme sua representação.

Identidades armazenadas continuam não possuídas automaticamente.

---

## 18.906 Objetos Parcialmente Inicializados

O mecanismo de cleanup de construção deverá permanecer separado da destruição normal.

---

## 18.907 Diferença Fundamental

Um objeto vivo possui:

* todos os campos obrigatórios;
* invariantes estabelecidos;
* registro no Domain;
* identidade publicada.

Um objeto parcial não possui essas garantias.

---

## 18.908 Drop Glue Parcial

O compilador deverá gerar cleanup específico baseado no progresso.

---

## 18.909 Ausência de Destrutor Explícito Completo em Objeto Parcial

O corpo do destrutor do tipo concreto não deverá executar se sua pré-condição for objeto completo.

---

## 18.910 Cleanup de Base Parcial

Se o construtor da base falhar, somente seus campos concluídos serão destruídos.

---

## 18.911 Cleanup da Derivada Parcial

Se a base foi concluída e a derivada falha:

1. destruir campos derivados concluídos;
2. destruir parte base completa;
3. cancelar reserva.

---

## 18.912 Reutilização de Rotinas

A rotina de destruição da base completa poderá ser reutilizada no cleanup da derivada quando a base foi totalmente construída.

---

## 18.913 Destruição de Temporários

Temporários deverão ser destruídos ao fim de seu escopo ou após seu último uso, conforme a IR.

---

## 18.914 Ordem entre Temporários e Objetos

A ordem deverá respeitar a semântica de avaliação.

---

## 18.915 Temporário Movido

Não será destruído após movimento.

---

## 18.916 Temporário Emprestado

O empréstimo deve terminar antes do drop.

---

## 18.917 Drop no Fim de Bloco

Valores por escopo poderão ser destruídos em ordem inversa de declaração ou criação, conforme a especificação.

---

## 18.918 Retorno de Função

Antes do retorno, valores locais não movidos deverão ser destruídos.

---

## 18.919 Propagação de Erro

Antes de propagar `Result::Error`, o código deverá executar drops dos locais ativos.

---

## 18.920 `break` e `continue`

Essas transferências de controle deverão executar cleanup dos escopos abandonados.

---

## 18.921 Loop

Cada iteração deverá possuir cleanup próprio.

---

## 18.922 Pattern Matching

Valores vinculados em braços deverão ser destruídos ao sair do braço, salvo movimento.

---

## 18.923 Destruição e Borrowing

Nenhum valor poderá ser destruído enquanto houver empréstimo ativo incompatível.

---

## 18.924 Verificação Estática

O compilador deverá rejeitar:

* drop durante borrow;
* retorno de referência a valor destruído;
* destruição do Domain com borrow ativo;
* movimento após drop;
* uso após destruição.

---

## 18.925 Empréstimos Persistentes

A linguagem deverá impedir que referências temporárias resolvidas sobrevivam à destruição do objeto.

---

## 18.926 Destruição do Domain com Referências Resolvidas

A finalização só poderá iniciar quando não houver empréstimos ativos válidos.

---

## 18.927 Runtime e Empréstimos

A primeira implementação poderá depender principalmente da análise estática.

Builds de debug poderão manter contadores ou guards internos.

---

## 18.928 Guards de Resolução

Um `ObjectRef` interno Rust poderá carregar guard associado ao Domain.

---

## 18.929 Bloqueio de Finalização

Enquanto um guard interno existir, a destruição do Domain deverá ser impossível dentro do Runtime seguro.

---

## 18.930 FFI e Empréstimos

Ponteiros fornecidos à FFI deverão possuir lifetime explícito.

A FFI não poderá retê-los após o escopo autorizado.

---

## 18.931 Callback Externo Durante Destruição

Deverá ser evitado ou controlado.

O callback não poderá guardar `self`.

---

## 18.932 Destruição de Handles Externos

A rotina deverá usar a ABI da biblioteca externa e tratar códigos de erro conforme política.

---

## 18.933 Ordem com Recursos Dependentes

Exemplo:

```text
Connection
Statement
```

Se `Statement` depende de `Connection`, o Statement deverá ser destruído primeiro.

Domains aninhados ou ordem de construção poderão garantir isso.

---

## 18.934 Recomendação Arquitetural

Recursos com forte relação hierárquica deverão usar:

* composição por valor;
* Domain filho;
* tipo de guard;
* API de escopo.

---

## 18.935 Destruição por Composição

Campos por valor garantem que o componente seja destruído antes da base externa conforme sua posição.

---

## 18.936 Destruição de Objetos Referenciados

Uma referência por `ObjectId` não expressa destruição por composição.

---

## 18.937 Ownership de Objetos por Identidade

Se a linguagem introduzir ownership exclusivo de identidade, isso deverá ser mecanismo explícito.

Não será inferido de um campo comum.

---

## 18.938 `UniqueObjectId`

Uma evolução futura poderá distinguir identidade exclusivamente possuída.

Isso permitiria destruição automática ao fim do owner.

Não fará parte do bootstrap inicial.

---

## 18.939 Destruição Coletiva como Regra Principal

No modelo inicial, o mecanismo primário para objetos por identidade será o lifetime do Domain.

---

## 18.940 Destruição Antecipada como Recurso Controlado

Deverá ser usada quando necessária, sem substituir o modelo coletivo.

---

## 18.941 Relação com `ObjectId`

A destruição não precisa localizar quem possui cópias do `ObjectId`.

Ela apenas invalida o slot.

---

## 18.942 Identidade Obsoleta

Uma identidade obsoleta poderá permanecer como valor, mas não resolverá.

---

## 18.943 Operações sobre Identidade Obsoleta

A especificação deverá definir se são permitidas:

* comparação;
* hashing;
* logging.

A resolução e o acesso serão proibidos.

---

## 18.944 Igualdade após Destruição

Duas cópias do mesmo `ObjectId` antigo continuarão bitwise iguais.

Isso não significa objeto vivo.

---

## 18.945 Reutilização e Igualdade

Novo objeto em mesmo slot terá geração diferente e não será igual ao antigo.

---

## 18.946 Hash de Identidade

Deverá incluir componentes suficientes para distinguir gerações.

---

## 18.947 Diagnóstico de Identidade Obsoleta

Poderá informar:

* Domain;
* slot;
* geração esperada;
* geração atual;
* tipo anterior, se disponível;
* estado.

---

## 18.948 Metadados após Invalidação

O slot poderá preservar metadados de debug do objeto destruído.

Em release, poderá removê-los.

---

## 18.949 Debugging de Use-After-Destroy

O Runtime deverá produzir erro claro em builds verificadas.

---

## 18.950 Segurança em Release

Mesmo sem diagnósticos completos, a validação de geração deverá impedir resolução indevida.

---

## 18.951 Destruição e Concorrência Futura

Quando Domains forem confinados, a destruição poderá ocorrer sem sincronização global.

---

## 18.952 Transferência de Domain

Um Domain transferido deverá ser destruído somente pelo novo owner.

---

## 18.953 Acesso Concorrente

A destruição não poderá ocorrer enquanto outro executor acessa o objeto.

---

## 18.954 Estado Atômico Futuro

Em modelo concorrente, estados do slot poderão precisar de atomics.

Não será requisito da fase sequencial.

---

## 18.955 Destruição em Thread Específica

Alguns recursos externos exigem finalização em thread específica.

A biblioteca poderá modelar essa restrição.

O Runtime geral não deverá presumir qualquer thread.

---

## 18.956 Deferred Destruction

Uma API futura poderá enfileirar finalização.

Isso alteraria o tempo determinístico e deverá ser explícito.

---

## 18.957 Ausência de Finalizador Assíncrono Implícito

A Capi não deverá criar thread de finalização automática.

---

## 18.958 Runtime Helpers de Destruição

O Runtime poderá oferecer:

```text
capi_rt_object_begin_destroy
capi_rt_object_finish_destroy
capi_rt_object_invalidate
capi_rt_domain_begin_finalize
capi_rt_domain_next_object
capi_rt_domain_finish_finalize
```

---

## 18.959 Helper de Destruição Antecipada

Uma operação possível:

```text
capi_rt_object_destroy(object_id, runtime_context)
```

Ela poderá:

1. resolver e bloquear;
2. marcar `Finalizing`;
3. chamar destrutor concreto;
4. invalidar slot.

---

## 18.960 Responsabilidade do Runtime na Destruição

O Runtime será responsável por:

* validar o slot;
* controlar estados;
* prevenir destruição duplicada;
* obter metadados concretos;
* invalidar identidade;
* organizar destruição coletiva;
* liberar blocos do Domain.

---

## 18.961 Responsabilidade do Código Gerado

O código gerado será responsável por:

* corpo do destrutor;
* ordem dos campos;
* drop glue;
* encadeamento da base;
* drop de valores;
* cleanup de fluxos.

---

## 18.962 Responsabilidade do Compilador

O compilador deverá validar:

* uso após drop;
* destruição durante borrow;
* escape em destrutor;
* ordem estrutural;
* trivialidade;
* chamadas permitidas;
* efeitos de movimento;
* caminhos de cleanup.

---

## 18.963 Assinatura Interna do Destrutor

Uma assinatura possível:

```text
destroy_T(
    self_pointer,
    runtime_context
) -> Unit
```

---

## 18.964 Ponteiro de `self`

A convenção deverá ser a mesma adotada para métodos ou ser adaptada por thunk interno.

---

## 18.965 Destrutor da Base

Poderá receber o mesmo ponteiro de objeto, utilizando offsets conhecidos.

---

## 18.966 Função de Drop de Campo

Para um tipo não trivial:

```text
drop_FieldType(field_pointer, runtime_context)
```

---

## 18.967 Drop Monomorfizado

Tipos genéricos terão drop especializado após monomorfização.

---

## 18.968 Drop de Enum

O código deverá:

1. ler a tag;
2. selecionar a variante;
3. destruir o payload ativo;
4. invalidar o valor quando necessário.

---

## 18.969 Drop de Array

Poderá usar loop reverso.

---

## 18.970 Drop de Tuple

Destruirá componentes em ordem inversa.

---

## 18.971 Drop de Struct

Destruirá campos em ordem inversa de declaração.

---

## 18.972 Otimização de Drop Trivial

O Middle-end poderá remover drops de tipos triviais.

---

## 18.973 Fusão de Drops

Sequências poderão ser inlineadas ou fundidas.

---

## 18.974 Eliminação de Estado Redundante

Em release, transições de estado conhecidas estaticamente poderão ser reduzidas, preservando a validação de slot necessária.

---

## 18.975 Lowering de `drop`

A IR de alto nível poderá conter:

```text
drop.value
drop.object
drop.partial
domain.finalize
```

---

## 18.976 Lowering de Objeto

`drop.object` deverá tornar-se:

```text
resolve
mark finalizing
load concrete destructor
indirect call
invalidate slot
```

---

## 18.977 Chamada Indireta do Destrutor

Será necessária quando o tipo estático não for concreto.

---

## 18.978 Chamada Direta do Destrutor

Quando o tipo concreto for conhecido, o compilador poderá chamar diretamente o drop glue.

Ainda deverá coordenar com o slot.

---

## 18.979 Devirtualização do Destrutor

Classes finais ou objetos recém-criados permitem chamada direta.

---

## 18.980 IR de Cleanup

Blocos de cleanup deverão ser explícitos.

---

## 18.981 Drop Scopes

A IR poderá organizar valores em escopos de destruição.

---

## 18.982 Saída de Escopo

Cada saída deverá executar os drops ativos em ordem inversa.

---

## 18.983 Validador de IR

Deverá rejeitar:

* drop duplicado;
* uso após drop;
* drop de valor não inicializado;
* destruição sem autoridade;
* invalidação antes do drop glue;
* publicação após destruição;
* retorno com reservas ou drops pendentes.

---

## 18.984 Drop Flag em SSA

Quando possível, o estado poderá ser representado por fluxo, sem memória.

---

## 18.985 Drop Flag Materializada

Em controle complexo, poderá ser variável booleana ou discriminante.

---

## 18.986 Código de Cleanup Compartilhado

O compilador poderá reutilizar blocos para reduzir tamanho.

---

## 18.987 Integração com Cranelift

Na fase Cranelift, o lowering deverá priorizar:

* chamadas explícitas;
* blocos claros;
* pouca otimização;
* validação fácil;
* ABI estável.

---

## 18.988 Integração com LLVM

LLVM poderá:

* inlinear drops;
* eliminar estados redundantes;
* remover destructors triviais;
* simplificar branches;
* combinar loops;
* devirtualizar.

---

## 18.989 Semântica Independente do Backend

Ambos deverão preservar:

* mesma ordem;
* mesmos efeitos;
* mesma invalidação;
* mesma política de erro.

---

## 18.990 Otimização de Domain Trivial

Se um Domain contiver apenas objetos trivialmente destrutíveis, a finalização poderá liberar blocos diretamente após invalidar slots.

---

## 18.991 Análise de Trivialidade do Domain

Poderá ser mantida incrementalmente ou calculada.

Não será necessária inicialmente.

---

## 18.992 Destruição em Lote

O Runtime poderá percorrer slots em lote para reduzir overhead.

---

## 18.993 Prefetch e Localidade

A ordem inversa de publicação tende a manter alguma localidade em alocação linear.

---

## 18.994 Objeto Grande Dedicado

Sua memória poderá ser liberada após o destrutor, mesmo antes do fim do Domain.

---

## 18.995 Objetos Pequenos em Blocos

A memória será liberada no final do bloco.

---

## 18.996 Fragmentação

A destruição antecipada pode produzir espaço não reutilizado.

Isso deverá ser medido.

---

## 18.997 Compactação Futura

Movimentação física futura poderá ser considerada porque identidade não depende do endereço.

Não fará parte da implementação inicial.

---

## 18.998 Destruição Antes de Movimento

Um objeto em finalização nunca será movido.

---

## 18.999 Pinning e Destruição

Objetos pinned ainda deverão ser finalizados normalmente.

---

## 18.1000 FFI e Endereço Persistente

A FFI deverá liberar qualquer pin ou registro antes da destruição.

---

## 18.1001 Instrumentação de Destruição

O Runtime poderá registrar:

* objetos destruídos;
* destruições antecipadas;
* destruições coletivas;
* tempo por tipo;
* falhas de finalização;
* slots invalidados;
* bytes liberados;
* tombstones.

---

## 18.1002 Trace de Objeto

Exemplo:

```text
begin destroy Dog ObjectId(...)
run Dog destructor
drop Dog.owner
drop Dog.name
run Animal destructor
drop Animal.age
invalidate slot
```

---

## 18.1003 Trace de Domain

```text
begin finalize Domain 12
destroy child Domain 14
destroy slot 42
destroy slot 39
release 4 blocks
finish Domain 12
```

---

## 18.1004 Relatório de Dependência Inválida

Poderá mostrar:

```text
Dog destructor attempted to access Owner
Owner was already destroyed
```

---

## 18.1005 Diagnóstico de Double Destroy

Deverá incluir:

* identidade;
* estado;
* origem da primeira destruição;
* origem da segunda tentativa.

---

## 18.1006 Diagnóstico de Destruição Durante Borrow

Deverá indicar o empréstimo ainda ativo.

---

## 18.1007 Diagnóstico de Ressurreição

Deverá indicar a operação que tentou publicar ou armazenar `self`.

---

## 18.1008 Diagnóstico de Domain Finalizing

Nova alocação deverá falhar com mensagem clara.

---

## 18.1009 Testes de Destrutor de Classe

Deverão cobrir:

* destrutor explícito;
* destrutor implícito;
* classe vazia;
* campo trivial;
* campo não trivial;
* herança;
* classe final;
* classe abstrata como base.

---

## 18.1010 Testes de Ordem

Para `A → B → C`, deverá ser observado:

```text
C destructor body
C fields
B destructor body
B fields
A destructor body
A fields
```

conforme a política final.

---

## 18.1011 Testes de Campos

Deverão confirmar ordem inversa.

---

## 18.1012 Testes de Identidade

Após destruição:

* resolução falha;
* geração anterior é inválida;
* novo objeto no mesmo slot possui identidade distinta.

---

## 18.1013 Testes de Destruição Antecipada

Deverão validar:

* objeto vivo;
* objeto já destruído;
* aliases;
* borrow ativo;
* Domain finalizando;
* uso posterior.

---

## 18.1014 Testes de Domain

Deverão validar:

* ordem inversa;
* tombstones;
* filhos;
* objetos triviais;
* objetos não triviais;
* reservas abertas;
* objetos destruídos antecipadamente.

---

## 18.1015 Testes de Ciclos

Objetos em ciclo deverão ser destruídos sem vazamento lógico e sem recursão de tracing.

---

## 18.1016 Testes de Dependência

Construção em ordem apropriada deverá permitir uso da dependência no destrutor.

Ordem inadequada deverá produzir falha detectável em debug.

---

## 18.1017 Testes de Recursos

Deverão usar handles falsos para verificar fechamento único.

---

## 18.1018 Testes de `close`

Após close explícito, o destrutor não deverá repetir a operação.

---

## 18.1019 Testes de Panic

Deverão validar a política adotada sem deixar ABI inconsistente.

---

## 18.1020 Testes de Cleanup Parcial

Devem confirmar que o destrutor completo não executa em objeto incompleto.

---

## 18.1021 Testes de Movimento

Campos movidos não deverão sofrer double drop.

---

## 18.1022 Testes de Fluxo

Deverão cobrir:

* return;
* error propagation;
* break;
* continue;
* match;
* loops;
* branches.

---

## 18.1023 Golden Tests de IR

Exemplo:

```text
object.begin_destroy
object.call_destructor
object.invalidate
```

Para valor:

```text
drop.field c
drop.field b
drop.field a
```

---

## 18.1024 Testes Diferenciais

Cranelift e LLVM deverão produzir:

* mesma ordem;
* mesmos logs;
* mesmos recursos liberados;
* mesmas invalidações.

---

## 18.1025 Fuzzing de Hierarquias

Gerar combinações de:

* campos;
* bases;
* drops;
* movimentos;
* destruição antecipada;
* falhas.

---

## 18.1026 Property-Based Testing

Propriedades mínimas:

1. cada objeto publicado é destruído no máximo uma vez;
2. todo objeto vivo no fim do Domain é destruído;
3. identidade destruída não resolve;
4. geração reutilizada difere;
5. campos são destruídos em ordem inversa;
6. derivada é destruída antes da base;
7. objetos parciais não usam destrutor completo;
8. ciclos não impedem finalização coletiva;
9. `ObjectId` não controla liveness;
10. liberação física ocorre após finalização semântica.

---

## 18.1027 Sanitizers

Deverão detectar:

* use-after-destroy;
* double free;
* double drop;
* acesso desalinhado;
* leitura após poison;
* vazamento de handles;
* corrupção de slot.

---

## 18.1028 Miri

O Runtime Rust deverá testar:

* transições de estado;
* ponteiros;
* guards;
* invalidação;
* acesso ao cabeçalho;
* iteração de slots.

---

## 18.1029 Fuzzing de Estados

O Runtime poderá receber sequências aleatórias de:

```text
reserve
publish
destroy
resolve
finalize domain
reuse slot
```

Estados inválidos deverão ser rejeitados.

---

## 18.1030 Benchmarks

Deverão medir:

* drop trivial;
* destrutor simples;
* hierarquia profunda;
* destruição antecipada;
* Domain com muitos objetos;
* Domain com muitos tombstones;
* objetos com muitos campos;
* objetos com recursos;
* ciclos.

---

## 18.1031 Métricas

Acompanhar:

* tempo por objeto;
* tempo por Domain;
* bytes por registro;
* custo de invalidação;
* custo de geração;
* quantidade de drops eliminados;
* tamanho de drop glue.

---

## 18.1032 Critério Inicial de Performance

A primeira versão deverá evitar:

* locks globais;
* reflexão;
* busca por nome;
* alocação durante cada drop;
* tracing de grafo;
* recursão profunda desnecessária.

---

## 18.1033 Iteração sem Recursão

A destruição coletiva do Domain deverá preferir loop sobre a ordem de publicação.

---

## 18.1034 Recursão de Herança

A cadeia de drop poderá ser implementada por chamadas estáticas.

Profundidade patológica deverá ser controlada pelo compilador.

---

## 18.1035 Recursão de Valores

Estruturas profundamente aninhadas podem gerar cadeias de drop.

O backend poderá inlinear ou emitir helpers.

---

## 18.1036 Tamanho do Código

Monomorfização de drops pode aumentar o binário.

Otimizações futuras poderão compartilhar código compatível.

---

## 18.1037 Metadados de Destruição

`TypeMetadata` poderá conter:

```text
destructor
is_trivially_destructible
drop_layout opcional
```

---

## 18.1038 Descritores de Campos

Não serão necessários para drops estáticos comuns.

---

## 18.1039 Drop Interpretado por Metadados

Uma alternativa seria Runtime interpretar descritores de campos.

Isso reduz código, mas aumenta custo e complexidade dinâmica.

Não será a estratégia inicial.

---

## 18.1040 Drop Gerado

A primeira implementação deverá gerar código específico por tipo.

---

## 18.1041 Remoção de Metadados Inúteis

Se o drop for direto e não houver cast, parte dos metadados poderá ser eliminada pelo linker.

---

## 18.1042 ABI do Destrutor

A ABI deverá definir:

* assinatura;
* convenção de `self`;
* RuntimeContext;
* política de panic;
* alinhamento;
* responsabilidade pela invalidação.

---

## 18.1043 Responsabilidade pela Invalidação

Recomenda-se que o wrapper do Runtime invalide o slot após o drop glue concluir.

O drop glue não deverá manipular gerações diretamente.

---

## 18.1044 Razão da Separação

Isso centraliza:

* estados;
* slots;
* gerações;
* diagnóstico;
* reutilização.

---

## 18.1045 Destrutor Chamado Diretamente Internamente

Mesmo em chamada direta, deverá existir wrapper ou sequência equivalente que atualize o slot.

---

## 18.1046 ABI entre Compilador e Runtime

A versão deverá ser validada.

---

## 18.1047 Mudança de Ordem de Drop

Alterar a ordem de campos ou base pode mudar comportamento observável.

Após estabilização, isso deverá ser tratado como decisão semântica e de ABI.

---

## 18.1048 Destrutores e Compatibilidade Binária

Adicionar campo não trivial altera o drop glue.

Módulos dependentes deverão ser recompilados ou usar ABI estável.

---

## 18.1049 Linkedição Estática Inicial

No bootstrap, recompilação integral será aceitável.

---

## 18.1050 Bibliotecas Dinâmicas Futuras

Exigirão compatibilidade entre:

* metadados;
* destrutores;
* layouts;
* Runtime.

---

## 18.1051 FFI de Destruição

Handles exportados deverão possuir função explícita:

```text
capi_handle_destroy
```

Código C não deverá chamar drop glue interno diretamente.

---

## 18.1052 Destruição Duplicada pela FFI

A API deverá invalidar o handle ou retornar erro.

---

## 18.1053 Handles Nulos

A política deverá ser explícita.

---

## 18.1054 Ownership Externo

Ao transferir ownership à FFI, a Capi não deverá destruir o recurso até recebê-lo de volta ou conforme contrato.

---

## 18.1055 Borrow Externo

Ponteiros emprestados não transferem responsabilidade de destruição.

---

## 18.1056 Finalização no Encerramento Anormal

Em abortamento fatal, não há garantia de execução de destrutores.

---

## 18.1057 Garantia de Destruição

A destruição determinística aplica-se ao fluxo normal e às saídas controladas.

Não deverá prometer execução após:

* falha de energia;
* kill abrupto;
* abort;
* corrupção crítica.

---

## 18.1058 Sinais do Sistema

A Toolchain não deverá executar lógica complexa de finalização em signal handler inseguro.

---

## 18.1059 Saída Normal

O entrypoint deverá destruir o RuntimeContext e Domain raiz antes de retornar.

---

## 18.1060 `exit` Imediato

Uma operação de saída imediata poderá não executar destructors.

Deverá ser distinta de retorno normal.

---

## 18.1061 Documentação da Biblioteca

APIs deverão explicar quando close explícito é necessário.

---

## 18.1062 Destrutores e Logging

O logger usado durante finalização deverá permanecer disponível até os objetos dependentes serem destruídos.

---

## 18.1063 Ordem dos Serviços Globais

O RuntimeContext deverá controlar a ordem de:

* stdout;
* stderr;
* logger;
* Domains;
* alocador;
* platform layer.

---

## 18.1064 Destruição do RuntimeContext

Deverá ocorrer por último.

---

## 18.1065 Streams Padrão

Não deverão ser fechados prematuramente se diagnósticos ainda puderem ser emitidos.

---

## 18.1066 Allocator

O alocador deverá permanecer ativo até todos os Domains serem liberados.

---

## 18.1067 Metadata Estática

Não necessita destruição comum.

---

## 18.1068 VTables e ITables

São dados estáticos imutáveis e não exigem finalização.

---

## 18.1069 Registro Global de Tipos

Se existir, será destruído após os objetos.

---

## 18.1070 Relação com Auto-Hospedagem

O compilador escrito em Capi dependerá da destruição correta de:

* AST;
* HIR;
* símbolos;
* strings;
* buffers;
* módulos;
* diagnósticos;
* caches.

---

## 18.1071 Domains por Fase do Compilador

O compilador auto-hospedado poderá usar Domains separados para:

* parsing;
* análise;
* IR;
* backend;
* diagnóstico temporário.

A destruição de cada fase liberará grafos inteiros.

---

## 18.1072 Benefício para o Compilador

Isso evita:

* drops individuais complexos para grafos;
* reference counting;
* GC;
* vazamentos de ciclos.

---

## 18.1073 Recursos Externos do Compilador

Arquivos, buffers mapeados e processos externos ainda exigirão drop específico.

---

## 18.1074 Estratégia de Fases

Objetos temporários deverão pertencer ao Domain da fase mais curta possível.

---

## 18.1075 Diagnóstico de Vazamento Lógico

Ao finalizar um Domain, o Runtime poderá informar objetos ainda vivos apenas como estatística, não como vazamento, pois serão destruídos coletivamente.

---

## 18.1076 Vazamento de Resource Handle

Se um drop não fechar handle, ferramentas de debug deverão detectar quando possível.

---

## 18.1077 Leak Sanitizer

Poderá ser usado para memória externa ao alocador de Domain.

---

## 18.1078 Invariantes de Destruição de Objetos

A implementação deverá preservar:

1. todo objeto publicado participa da política de destruição;
2. um objeto é destruído no máximo uma vez;
3. a derivada é finalizada antes da base;
4. campos são destruídos em ordem inversa à inicialização;
5. a identidade deixa de resolver após a destruição;
6. o tipo concreto permanece conhecido durante a finalização;
7. destruição não depende do número de referências;
8. ciclos não impedem destruição coletiva;
9. objeto em finalização não pode ser ressuscitado;
10. destrutor completo não executa em objeto parcial;
11. memória somente é liberada após o drop semântico;
12. recursos owned são finalizados;
13. campos de identidade não destroem automaticamente os objetos referidos;
14. aliases antigos são neutralizados pela geração;
15. falha estrutural não deixa o slot marcado como vivo silenciosamente.

---

## 18.1079 Invariantes de Destruição de Domain

1. Domain em finalização não aceita novas alocações;
2. filhos não sobrevivem ao pai;
3. objetos vivos são percorridos em ordem determinística;
4. objetos já destruídos são ignorados;
5. reservas abertas são erro interno ou são canceladas;
6. todos os slots são invalidados;
7. todos os blocos são liberados ao final;
8. metadados permanecem válidos durante os drops;
9. RuntimeContext permanece disponível;
10. o Domain termina no estado `Destroyed`.

---

## 18.1080 ADRs Recomendados

Deverão ser considerados:

```text
ADR — Destrutor concreto referenciado por TypeMetadata
ADR — Corpo do destrutor antes do drop dos campos
ADR — Campos destruídos em ordem inversa
ADR — Derivada destruída antes da base
ADR — Proibição de ressurreição
ADR — Restrição de chamadas virtuais em destrutores
ADR — Invalidação do slot após drop glue
ADR — Ordem inversa de publicação no Domain
ADR — Filhos destruídos antes do pai
ADR — Campos ObjectId não controlam lifetime
ADR — Destrutores automáticos infalíveis
ADR — Close explícito para erros recuperáveis
ADR — Drop glue gerado por tipo
```

---

## 18.1081 Critérios de Conclusão da Destruição Básica

A destruição básica estará concluída quando:

* objetos vivos puderem ser marcados como `Finalizing`;
* o destrutor concreto puder ser localizado;
* campos não triviais puderem ser destruídos;
* o slot puder ser invalidado;
* identidades antigas deixarem de resolver;
* classes vazias puderem ser destruídas;
* destruição duplicada for detectada;
* testes de sanitizers passarem.

---

## 18.1082 Critérios de Conclusão da Destruição com Herança

Estará concluída quando:

* a derivada for destruída antes da base;
* campos de cada nível seguirem ordem inversa;
* destrutores explícitos e implícitos funcionarem;
* referência de base selecionar destrutor concreto;
* metadados concretos permanecerem válidos;
* herança profunda funcionar;
* chamadas inseguras durante destruição forem rejeitadas.

---

## 18.1083 Critérios de Conclusão do Cleanup Parcial

Estará concluído quando:

* falhas de construção não chamarem destrutor completo;
* somente campos inicializados forem destruídos;
* drop flags forem respeitadas;
* a base completa puder ser limpa;
* partes incompletas não forem acessadas;
* slots reservados forem cancelados;
* nenhum recurso sofrer double drop.

---

## 18.1084 Critérios de Conclusão da Destruição de Domains

Estará concluída quando:

* o Domain bloquear novas operações;
* filhos forem finalizados;
* objetos forem percorridos em ordem inversa;
* tombstones forem ignorados;
* ciclos forem finalizados;
* todos os slots forem invalidados;
* blocos forem liberados;
* o estado final for `Destroyed`;
* RuntimeContext permanecer válido durante todo o processo.

---

## 18.1085 Critérios de Conclusão para o Bootstrap

O modelo estará pronto para o bootstrap quando:

* a Biblioteca Mínima puder liberar todos os seus recursos;
* o compilador Capi puder destruir grafos inteiros por Domain;
* arquivos e buffers forem finalizados corretamente;
* construções falhas não vazarem recursos;
* Cranelift gerar drops e cleanup;
* LLVM preservar a mesma semântica;
* testes diferenciais passarem;
* referências obsoletas forem detectadas;
* nenhuma contagem de referências ou GC for necessária.

---

## 18.1086 Resultado Esperado da Terceira Parte — Bloco 2

Ao final desta etapa, a implementação oficial deverá possuir um processo em que:

```text
TypeMetadata
    localiza o destrutor concreto

Drop Glue
    finaliza a classe, seus campos e sua base

Domain
    organiza a ordem coletiva de destruição

Slot
    controla estado, geração e invalidação

ObjectId
    deixa de resolver após o fim do lifetime

Campos por valor
    são destruídos estruturalmente

Campos ObjectId
    não controlam o lifetime do objeto referido

Cleanup Parcial
    encerra somente partes efetivamente inicializadas
```

A destruição deverá permanecer:

* determinística;
* ordenada;
* segura diante de aliases;
* compatível com herança;
* compatível com ciclos;
* independente de Garbage Collection;
* independente de ARC;
* integrada aos Domains;
* auditável em debug;
* eficiente em compilação AOT.

A próxima parte deverá concluir o Capítulo 18 por meio da definição de:

* estabilidade e evolução da ABI;
* compatibilidade entre compilador e Runtime;
* versionamento de layouts e metadados;
* evolução futura do modelo de objetos;
* otimizações;
* testes integrados;
* critérios finais de conclusão;
* encerramento normativo do modelo de objetos da implementação oficial.

---

## Terceira Parte — Bloco 3.1 — ABI, Compatibilidade e Evolução

## 18.1087 Objetivo

Esta parte define os contratos de implementação necessários para garantir a evolução controlada do modelo de objetos da Capi.

Serão definidos:

* escopo da ABI do modelo de objetos;
* compatibilidade entre compilador e Runtime;
* compatibilidade entre módulos;
* versionamento de layouts;
* versionamento de metadados;
* estabilidade de vtables e itables;
* evolução de `ObjectId`;
* evolução de Domains;
* compatibilidade entre Cranelift e LLVM;
* validação durante compilação e linkedição;
* política de quebra de compatibilidade;
* migração durante o bootstrap;
* requisitos para futura estabilização binária.

O objetivo não será congelar prematuramente a representação interna, mas impedir que mudanças ocorram de forma implícita, não detectada ou semanticamente divergente.

---

## 18.1088 Princípio da Evolução Controlada

A implementação deverá preservar o seguinte princípio:

> Toda mudança que altere a interpretação de objetos, identidades, metadados, layouts ou chamadas deverá ser explicitamente versionada, validada ou acompanhada por recompilação integral.

---

## 18.1089 Separação entre Semântica e ABI

A semântica da linguagem define:

* o que é um objeto;
* o que é identidade;
* como funciona herança;
* como funciona subtipagem;
* quando ocorre destruição;
* quais operações são válidas.

A ABI define:

* como essas garantias são representadas;
* como módulos trocam valores;
* como funções são chamadas;
* como tabelas são organizadas;
* como metadados são interpretados;
* como o Runtime recebe operações.

Mudanças na ABI não deverão alterar silenciosamente a semântica.

---

## 18.1090 Escopos de Compatibilidade

A Toolchain deverá distinguir pelo menos:

```text
compatibilidade de fonte
compatibilidade semântica
compatibilidade de metadata de compilação
compatibilidade de IR serializada
compatibilidade de objeto binário
compatibilidade de biblioteca estática
compatibilidade de biblioteca dinâmica
compatibilidade de Runtime
compatibilidade de artefato de pacote
```

---

## 18.1091 Compatibilidade de Fonte

Uma mudança é compatível em fonte quando programas válidos continuam compilando com o mesmo significado observável.

---

## 18.1092 Compatibilidade Semântica

Uma mudança é semanticamente compatível quando preserva:

* comportamento;
* garantias de segurança;
* ordem relevante;
* lifetime;
* identidade;
* efeitos observáveis.

---

## 18.1093 Compatibilidade de Metadata

Módulos compilados separadamente poderão trocar descrições de:

* tipos;
* layouts;
* métodos;
* slots;
* interfaces;
* generics;
* símbolos;
* versões.

O formato deverá possuir versão própria.

---

## 18.1094 Compatibilidade de IR

Se a Toolchain serializar HIR, MIR ou IR, cada formato deverá possuir versão independente.

Não deverá ser assumido que a IR permanece estável entre versões do compilador.

---

## 18.1095 Compatibilidade Binária

Uma mudança é binariamente compatível quando artefatos compilados anteriormente continuam funcionando sem recompilação.

Essa garantia não será requisito inicial do bootstrap.

---

## 18.1096 Compatibilidade do Runtime

Um binário ou módulo compilado somente poderá executar com Runtime que compreenda:

* sua versão de ABI;
* seus layouts;
* suas operações;
* seus metadados;
* seu formato de `ObjectId`;
* suas convenções de chamada.

---

## 18.1097 ABI Interna Inicial

Durante o bootstrap, a ABI será considerada:

```text
interna
experimental
versionada
não estabilizada
sujeita a recompilação integral
```

---

## 18.1098 Ausência de Promessa Binária Inicial

As primeiras versões não deverão prometer que:

* bibliotecas antigas funcionem com compiladores novos;
* objetos antigos sejam vinculados com Runtime novo;
* metadados persistidos sejam reutilizáveis;
* plugins binários permaneçam compatíveis.

---

## 18.1099 Recompilação Integral

Durante o bootstrap, mudanças de ABI poderão exigir recompilação de:

* Runtime;
* Biblioteca Padrão;
* compilador;
* ferramentas;
* dependências;
* testes;
* aplicações.

Essa política deverá ser explícita.

---

## 18.1100 Fonte da Verdade da ABI

A ABI deverá ser definida por documentação própria e estruturas versionadas do compilador.

Não deverá ser inferida apenas do código Rust atual do Runtime.

---

## 18.1101 Documento de ABI

O Documento 10 continuará sendo a referência normativa principal da ABI pública.

O Documento 27 definirá o plano de implementação e evolução.

---

## 18.1102 Especificação Executável

Além da documentação, a Toolchain deverá possuir:

* testes de layout;
* testes de símbolos;
* testes de chamadas;
* hashes;
* validadores;
* dumps;
* fixtures binárias.

---

## 18.1103 Componentes da ABI do Modelo de Objetos

A ABI inicial deverá cobrir:

```text
ObjectId
DomainId
Slot
Generation
ObjectHeader
TypeMetadata
VTable
InterfaceTable
destructor ABI
method ABI
constructor ABI
cast ABI
RuntimeContext
allocation ABI
publication ABI
destruction ABI
```

---

## 18.1104 Versão Global da ABI

A Toolchain deverá possuir um identificador global, por exemplo:

```text
CAPI_ABI_VERSION
```

Ele identificará a geração geral da ABI.

---

## 18.1105 Versões Específicas

Também poderão existir versões específicas:

```text
OBJECT_ABI_VERSION
RUNTIME_ABI_VERSION
METADATA_FORMAT_VERSION
IR_FORMAT_VERSION
PACKAGE_METADATA_VERSION
```

Isso permite evoluções independentes.

---

## 18.1106 Representação da Versão

A versão poderá ser codificada como:

* inteiro;
* tupla maior-menor;
* hash;
* estrutura com capabilities;
* símbolo de linkedição.

A estratégia inicial deverá ser simples e inequívoca.

---

## 18.1107 Versão Maior

Mudanças incompatíveis deverão incrementar versão maior.

---

## 18.1108 Versão Menor

Mudanças aditivas compatíveis poderão incrementar versão menor.

Durante o bootstrap, mesmo mudanças menores poderão exigir recompilação.

---

## 18.1109 Símbolo de ABI

O Runtime poderá exportar um símbolo conceitual:

```text
capi_rt_abi_v1
```

O código gerado dependerá desse símbolo.

---

## 18.1110 Falha de Linkedição

Se a versão esperada não existir, a linkedição deverá falhar.

---

## 18.1111 Verificação em Runtime

Quando a linkedição não puder detectar incompatibilidade, o entrypoint poderá validar a versão na inicialização.

---

## 18.1112 Diagnóstico de Incompatibilidade

A mensagem deverá apresentar:

* versão requerida;
* versão encontrada;
* compilador;
* Runtime;
* target;
* artefato incompatível.

---

## 18.1113 Runtime Mínimo Compatível

O metadata de um módulo poderá informar a versão mínima de Runtime exigida.

---

## 18.1114 Capabilities do Runtime

Uma evolução futura poderá declarar recursos como:

```text
domain_v2
object_id_128
interface_lookup_v2
compact_metadata
```

O módulo poderá exigir capabilities específicas.

---

## 18.1115 Compatibilidade por Capabilities

Isso permitirá adicionar recursos sem depender apenas de versão monolítica.

Não será necessário inicialmente.

---

## 18.1116 Target Triplet

A compatibilidade deverá considerar:

* arquitetura;
* sistema operacional;
* ambiente;
* ABI nativa;
* tamanho de ponteiro;
* endianness.

---

## 18.1117 Data Layout do Target

O backend deverá fornecer um `DataLayout` canônico contendo:

* tamanho de ponteiro;
* alinhamentos;
* tamanhos de inteiros;
* convenções de agregados;
* alinhamento de pilha;
* regras de vetor.

---

## 18.1118 Fonte do `DataLayout`

Cranelift e LLVM deverão derivar suas decisões do mesmo modelo de target mantido pela Toolchain.

---

## 18.1119 Divergência entre Backends

Se Cranelift e LLVM calcularem layouts diferentes para o mesmo tipo e target, a compilação deverá falhar em testes ou validação.

---

## 18.1120 Layout Canônico

O layout dos tipos Capi deverá ser calculado pelo compilador antes do backend sempre que possível.

O backend não deverá redefinir a ordem dos campos.

---

## 18.1121 Responsabilidade do Backend

O backend poderá decidir:

* passagem em registradores;
* uso de stack;
* instruções;
* alinhamentos adicionais exigidos;
* lowering da ABI nativa.

Não poderá alterar o layout lógico definido.

---

## 18.1122 `LayoutId`

Cada layout materializado poderá possuir identificador interno.

---

## 18.1123 `LayoutHash`

A Toolchain deverá considerar um hash derivado de:

* tipo;
* tamanho;
* alinhamento;
* offsets;
* base;
* campos;
* representação;
* versão de ABI.

---

## 18.1124 Uso do `LayoutHash`

O hash poderá ser usado para:

* validar módulos;
* detectar metadata obsoleta;
* confirmar fixtures;
* diagnosticar incompatibilidades;
* validar FFI.

---

## 18.1125 Hash Não Público Inicialmente

O algoritmo de hash não deverá ser API pública estável no bootstrap.

---

## 18.1126 Layout de Classe

O layout de classe deverá incluir:

```text
ObjectHeader
BasePayload
DerivedPayload
Padding
```

---

## 18.1127 Evolução do Cabeçalho

Adicionar ou remover campos no `ObjectHeader` altera:

* offset do payload;
* tamanho do objeto;
* acesso aos metadados;
* ABI dos objetos.

Será mudança incompatível, salvo representação indireta.

---

## 18.1128 Estratégia de Cabeçalho Estável

Para reduzir mudanças futuras, o cabeçalho inicial deverá conter somente informações essenciais.

---

## 18.1129 Campos Essenciais do Cabeçalho

Recomenda-se manter:

* ponteiro para `TypeMetadata`;
* flags ou estado estritamente necessários.

Informações recuperáveis pelo slot não deverão ser duplicadas sem necessidade.

---

## 18.1130 Campos de Debug

Informações de debug poderão ser opcionais ou mantidas fora do layout de release.

---

## 18.1131 Cabeçalhos por Perfil

A Toolchain poderá possuir:

```text
debug object header
release object header
```

Entretanto, módulos mistos não poderão compartilhar objetos sem ABI compatível.

---

## 18.1132 Perfil de Build na ABI

Artefatos deverão registrar se utilizam:

* debug ABI;
* checked ABI;
* release ABI;
* sanitizer ABI.

---

## 18.1133 Mistura de Perfis

A linkedição deverá rejeitar combinações incompatíveis.

---

## 18.1134 `ObjectId` como Contrato Crítico

A representação de `ObjectId` afeta:

* campos;
* parâmetros;
* retornos;
* hashing;
* Optional;
* coleções;
* FFI;
* persistência acidental.

Sua evolução deverá ser cuidadosamente versionada.

---

## 18.1135 Componentes do `ObjectId`

A representação inicial deverá conter informação suficiente para identificar:

```text
Domain
Slot
Generation
```

---

## 18.1136 Largura do `ObjectId`

Possibilidades:

* 64 bits;
* 96 bits;
* 128 bits;
* estrutura dependente do target.

A escolha deverá equilibrar:

* capacidade;
* custo;
* alinhamento;
* longevidade;
* portabilidade.

---

## 18.1137 `ObjectId` de 64 Bits

Uma representação compacta reduz custo, mas limita:

* Domains;
* slots;
* gerações.

---

## 18.1138 `ObjectId` de 128 Bits

Oferece maior espaço de versão e segurança contra overflow, mas aumenta:

* memória;
* tráfego;
* custo de comparação em alguns targets.

---

## 18.1139 Estratégia Inicial

A representação escolhida no bootstrap deverá ser documentada como experimental.

---

## 18.1140 Alteração de Largura

Mudar a largura de `ObjectId` será quebra de ABI ampla.

---

## 18.1141 Abstração na IR

A IR de alto nível deverá tratar `ObjectId` como tipo próprio, não como inteiro comum.

Isso permitirá alterar sua representação com menor impacto interno.

---

## 18.1142 Lowering Tardio

A conversão para inteiros ou agregados nativos deverá ocorrer no lowering de backend.

---

## 18.1143 Operações Permitidas

A IR deverá oferecer operações semânticas:

```text
object_id.equal
object_id.hash
object_id.domain
object_id.slot
object_id.generation
object_id.is_invalid
```

---

## 18.1144 Proibição de Aritmética

Código comum não deverá executar aritmética sobre `ObjectId`.

---

## 18.1145 Niches e `Optional`

Se uma representação inválida for usada como `None`, essa regra torna-se parte do layout de `Optional<ObjectId<T>>`.

---

## 18.1146 Mudança do Niche

Alterar o niche pode quebrar ABI de campos, parâmetros e coleções.

---

## 18.1147 Versionamento do Niche

O metadata deverá registrar a estratégia de representação.

---

## 18.1148 Evolução do `DomainId`

O `DomainId` também poderá evoluir em largura ou estrutura.

Sua representação interna não deverá ser exposta desnecessariamente.

---

## 18.1149 `DomainId` na ABI Pública

A FFI deverá usar handle opaco sempre que possível.

---

## 18.1150 Estrutura de Slot

A disposição interna do slot não deverá fazer parte da ABI de módulos Capi.

Somente o Runtime deverá interpretá-la.

---

## 18.1151 Evolução do Slot

O Runtime poderá adicionar:

* flags;
* ponteiro;
* geração;
* metadata;
* dados de debug;
* locks futuros.

Isso será possível sem recompilar módulos se a interação ocorrer por helpers estáveis.

---

## 18.1152 Inlining de Operações de Slot

Inlining excessivo do layout do slot no código gerado reduz flexibilidade.

---

## 18.1153 Estratégia Inicial Recomendada

Operações sensíveis a evolução deverão utilizar helpers do Runtime durante o bootstrap.

---

## 18.1154 Evolução para Inlining

Após estabilização, operações críticas poderão ser inlineadas com versão de ABI explícita.

---

## 18.1155 ABI de Resolução

Uma função conceitual:

```text
capi_rt_object_resolve(
    RuntimeContext*,
    ObjectId
) -> ObjectPointer
```

deverá possuir contrato claro.

---

## 18.1156 Resultado de Resolução

O resultado poderá ser:

* ponteiro;
* `Result`;
* ponteiro nulo em falha;
* estrutura com status.

A escolha deverá ser uniforme.

---

## 18.1157 Resolução Checked e Unchecked

Poderão existir:

```text
resolve_checked
resolve_assume_valid
```

A segunda somente poderá ser usada após prova do compilador.

---

## 18.1158 Estabilidade dos Helpers

Helpers públicos ao código gerado deverão possuir nomes versionados ou tabela de funções.

---

## 18.1159 Tabela de API do Runtime

Uma estratégia futura poderá fornecer:

```text
RuntimeApiV1
├── object_resolve
├── object_publish
├── object_destroy
├── domain_create
└── ...
```

---

## 18.1160 Vantagem da Tabela

Ela permite selecionar implementação e validar versão.

---

## 18.1161 Custo da Tabela

Adiciona indireção e complexidade.

Não será necessária no bootstrap estático.

---

## 18.1162 `TypeMetadata` como Contrato

O layout de `TypeMetadata` será compartilhado entre:

* código gerado;
* Runtime;
* debugger;
* backends;
* ferramentas.

---

## 18.1163 Campos Iniciais de `TypeMetadata`

Poderão incluir:

```text
abi_version
type_id
kind
flags
size
alignment
base_type
vtable
interfaces
destructor
debug_name
```

---

## 18.1164 Ordem dos Campos de Metadata

A ordem deverá ser definida por estrutura canônica, não pelo layout acidental de uma struct Rust sem anotação.

---

## 18.1165 Representação Rust

Se o Runtime representar metadados em Rust, deverá utilizar:

```rust
#[repr(C)]
```

ou geração binária controlada.

---

## 18.1166 Ponteiros em Metadata

Ponteiros deverão respeitar:

* alinhamento;
* relocation;
* visibilidade;
* target;
* seções somente leitura.

---

## 18.1167 Metadata Extensível

Uma estratégia para evolução poderá incluir:

```text
size_of_metadata
abi_version
```

Campos novos poderão ser adicionados ao final.

---

## 18.1168 Compatibilidade por Prefixo

Um Runtime antigo poderia interpretar apenas o prefixo conhecido.

Isso somente será permitido se os campos novos forem realmente opcionais.

---

## 18.1169 Campos Obrigatórios Novos

Se uma nova operação depender de campo adicional, será necessária nova ABI ou capability.

---

## 18.1170 Metadata Imutável

Após linkedição e carregamento, `TypeMetadata` deverá permanecer imutável.

---

## 18.1171 Metadata por Tipo Concreto

Cada especialização concreta deverá possuir metadata própria.

---

## 18.1172 Deduplicação

O linker poderá deduplicar metadados equivalentes somente se preservar identidade de tipo.

---

## 18.1173 `TypeId`

O `TypeId` deverá identificar semanticamente o tipo no artefato.

---

## 18.1174 Formação do `TypeId`

Poderá derivar de:

* módulo;
* caminho nominal;
* parâmetros genéricos;
* versão;
* pacote;
* instância de compilação.

---

## 18.1175 Colisões

A Toolchain deverá detectar colisões dentro do artefato.

---

## 18.1176 `TypeId` Estável entre Builds

Não será exigido inicialmente.

---

## 18.1177 `TypeId` Persistente

Não deverá ser usado por aplicações como identificador persistente sem contrato específico.

---

## 18.1178 Comparação entre Módulos

Módulos compilados separadamente deverão concordar sobre o `TypeId` de tipos compartilhados.

---

## 18.1179 Metadata de Dependências

O pacote exportador deverá fornecer o identificador canônico.

---

## 18.1180 `TypeId` e Generics

Cada especialização monomorfizada poderá possuir `TypeId` distinto.

---

## 18.1181 `TypeId` e Aliases

Aliases transparentes deverão apontar para o mesmo tipo semântico.

Newtypes nominais deverão possuir `TypeId` distinto.

---

## 18.1182 `TypeId` e Classes Derivadas

Cada classe da hierarquia possui `TypeId` próprio.

---

## 18.1183 Versão da Estrutura de Metadata

O primeiro campo deverá permitir validar a interpretação.

---

## 18.1184 Hash de Metadata

Um hash opcional poderá abranger:

* layout;
* vtable;
* interfaces;
* destrutor;
* base;
* flags.

---

## 18.1185 Metadata de Compilação versus Runtime

A Toolchain deverá distinguir:

### Metadata de compilação

Rica, contendo nomes, assinaturas, generics e visibilidade.

### Metadata de Runtime

Mínima, contendo somente o necessário em execução.

---

## 18.1186 Ausência de Reflection Implícita

Nem toda informação de compilação deverá ser emitida no binário.

---

## 18.1187 VTable como Contrato Versionado

A organização dos slots deverá ser consistente entre módulos.

---

## 18.1188 Slot Virtual Canônico

A identidade lógica do método deverá ser mapeada para slot de forma determinística.

---

## 18.1189 Metadata de Exportação de Slots

Tipos públicos deverão exportar:

* métodos virtuais;
* slots;
* assinaturas;
* flags;
* origem;
* versão.

---

## 18.1190 Novo Override

Adicionar override em derivada não altera slots da base.

Pode alterar o alvo da vtable da derivada.

---

## 18.1191 Novo Método Virtual na Derivada

Pode ser adicionado ao final da vtable da derivada, sem alterar prefixo herdado.

---

## 18.1192 Novo Método Virtual na Base

Pode deslocar slots introduzidos por derivadas, se a atribuição for apenas sequencial.

---

## 18.1193 Risco de Inserção na Base

Para ABI estável, a adição de método à base requer estratégia específica.

---

## 18.1194 Estratégias Futuras de Slots Estáveis

Possibilidades:

* slots reservados;
* mapas por `MethodId`;
* tabelas secundárias;
* versionamento de vtable;
* recompilação obrigatória;
* interfaces para extensibilidade.

---

## 18.1195 Estratégia Inicial

Durante o bootstrap, adicionar método virtual à base exigirá recompilação integral.

---

## 18.1196 Remoção de Método Virtual

Será mudança incompatível.

---

## 18.1197 Alteração de Assinatura

Será mudança incompatível, mesmo que o número do slot permaneça.

---

## 18.1198 Alteração de Retorno Covariante

Pode exigir novo thunk e mudança no hash da vtable.

---

## 18.1199 Método Tornado `final`

Pode ser compatível em execução para módulos recompilados, mas altera extensibilidade em fonte.

---

## 18.1200 Método Deixando de Ser `final`

Pode exigir emissão de slot ou mudança de despacho.

---

## 18.1201 Classe Tornada `final`

Altera semântica de extensão, mas pode simplificar ABI interna.

---

## 18.1202 Classe Deixando de Ser `final`

Pode exigir vtable ou metadados adicionais.

---

## 18.1203 VTables como Constantes

Cada vtable deverá ser emitida em seção somente leitura.

---

## 18.1204 Símbolos de VTable

O mangling deverá incluir informação suficiente para evitar colisões.

---

## 18.1205 Visibilidade da VTable

Vtables de tipos privados poderão ser locais.

Vtables de tipos públicos usados entre módulos precisarão de símbolo compartilhável ou metadata relocável.

---

## 18.1206 COMDAT e Deduplicação

O backend poderá utilizar COMDAT ou mecanismo equivalente para especializações genéricas repetidas.

---

## 18.1207 Determinismo

A ordem das entradas não deverá depender de:

* hash map não ordenado;
* ordem de threads;
* endereços;
* ordem de descoberta acidental.

---

## 18.1208 `VTableHash`

A Toolchain poderá gerar hash contendo:

* slots;
* assinaturas;
* métodos;
* thunks;
* versão.

---

## 18.1209 Validação na Linkedição

Módulos que discordem sobre a vtable de um tipo deverão ser rejeitados.

---

## 18.1210 ITable como Contrato

Cada implementação de interface deverá respeitar:

* identidade da interface;
* ordem dos slots;
* assinatura;
* convenção de chamada.

---

## 18.1211 Evolução de Interface

Adicionar, remover ou alterar método poderá modificar todas as itables.

---

## 18.1212 Método Padrão Adicionado

Durante o bootstrap, ainda exigirá recompilação das implementações para gerar tabelas consistentes.

---

## 18.1213 Método Abstrato Adicionado

Todas as implementações deverão ser recompiladas e validadas.

---

## 18.1214 Ordem de Interfaces

A tabela de interfaces no tipo deverá utilizar ordenação canônica.

---

## 18.1215 Busca por `TypeId`

Mudanças na largura ou semântica do `TypeId` alteram a ABI de lookup.

---

## 18.1216 ITable Específica por Par

A identidade da itable deverá derivar de:

```text
ConcreteType × InterfaceType
```

---

## 18.1217 Herança de Interface

Mudanças na hierarquia de interfaces poderão alterar entradas exigidas.

---

## 18.1218 Compatibilidade de Método Padrão

A precedência deverá ser resolvida no compilador e registrada na itable.

---

## 18.1219 Thunks

A ABI de thunks deverá ser a mesma da entrada que representam.

---

## 18.1220 Símbolos de Thunks

Poderão ser internos e deduplicáveis.

---

## 18.1221 Alterações de Thunks

Não afetam callers se a assinatura e a tabela permanecerem compatíveis.

---

## 18.1222 ABI de Métodos

A Toolchain deverá definir para cada método:

* posição de `self`;
* parâmetros ocultos;
* representação do retorno;
* convenção de chamada;
* passagem de agregados;
* tratamento de generics;
* RuntimeContext;
* capabilities.

---

## 18.1223 Método Não Virtual

Pode usar ABI interna otimizada se não cruzar fronteira pública.

---

## 18.1224 Método Virtual

Overrides de um slot deverão compartilhar ABI física compatível.

---

## 18.1225 Método de Interface

A itable deverá apontar para função ou thunk com ABI da interface.

---

## 18.1226 Parâmetros Ocultos

Alterar parâmetros ocultos será quebra de ABI.

---

## 18.1227 RuntimeContext

A Toolchain deverá decidir se o contexto será:

* parâmetro explícito;
* registrador reservado;
* thread-local;
* recuperado do Domain.

A estratégia inicial deverá evitar estado global invisível.

---

## 18.1228 RuntimeContext como Parâmetro

Essa opção é mais explícita e fácil de testar.

---

## 18.1229 Otimização do Contexto

O backend poderá mantê-lo em registrador ou eliminá-lo quando não usado.

---

## 18.1230 Mutation Capability

Métodos mutantes poderão receber capability explícita ou token de IR.

Se fizer parte da ABI física, deverá ser versionada.

---

## 18.1231 Capability Apenas Estática

Quando possível, a capability poderá existir somente no sistema de tipos e ser removida antes do backend.

---

## 18.1232 Capability Reificada

Se necessária para Runtime ou concorrência, sua representação deverá ser definida na ABI.

---

## 18.1233 Retorno por Valor

Valores pequenos poderão ser retornados em registradores conforme ABI nativa.

---

## 18.1234 Retorno Indireto

Valores grandes poderão utilizar `sret` ou buffer oculto.

---

## 18.1235 Consistência entre Backends

Cranelift e LLVM deverão usar a mesma classificação de retorno.

---

## 18.1236 Classificador de ABI

A Toolchain deverá possuir componente compartilhado para classificar:

* parâmetros;
* retornos;
* agregados;
* alinhamentos;
* extensões;
* registradores.

---

## 18.1237 Dependência do Target

O classificador poderá variar por target.

---

## 18.1238 Testes de Classificação

Deverão comparar resultados dos dois backends.

---

## 18.1239 ABI de Construtores

A construção deverá manter separação entre:

* wrapper de criação;
* construtor interno;
* construtor da base;
* publicação.

---

## 18.1240 Mudança no Fluxo de Construção

Alterar a responsabilidade entre wrapper e Runtime poderá exigir nova versão.

---

## 18.1241 ABI de Destrutores

Deverá manter:

* ponteiro do objeto;
* RuntimeContext;
* ausência de retorno recuperável;
* responsabilidade pela invalidação no wrapper.

---

## 18.1242 Mudança na Ordem de Destruição

Será mudança semântica, não apenas otimização.

---

## 18.1243 ABI de Domains

O código gerado deverá interagir com Domains por helpers versionados.

---

## 18.1244 Estrutura Interna do Domain

Poderá evoluir sem quebrar módulos se permanecer opaca.

---

## 18.1245 Handle de Domain

A representação poderá ser ponteiro opaco ou identificador.

---

## 18.1246 Domain em Parâmetros

Funções públicas Capi poderão receber tipo `Domain` conforme sua ABI de valor.

---

## 18.1247 Mudança da Representação de Domain

Será quebra de ABI para funções que o recebem diretamente.

---

## 18.1248 Estratégia de Opacidade

A representação pública deverá ser pequena e delegar ao Runtime.

---

## 18.1249 RuntimeContext e Domain

A relação entre ambos deverá ser explicitamente documentada.

---

## 18.1250 Metadata de Módulo

Cada módulo compilado poderá emitir:

```text
ModuleAbiMetadata
├── compiler_version
├── abi_version
├── runtime_version
├── target
├── data_layout
├── feature_flags
├── dependency_hashes
└── exported_type_hashes
```

---

## 18.1251 Seção de Objeto

Esses dados poderão ser armazenados em seção específica do arquivo objeto.

---

## 18.1252 Uso pelo Linker da Toolchain

O driver Capi poderá ler e validar essas seções antes de chamar o linker nativo.

---

## 18.1253 Validação de Dependências

O módulo deverá registrar hashes ou versões das interfaces públicas usadas.

---

## 18.1254 Metadata Obsoleta

Se uma dependência mudar, o módulo deverá ser recompilado.

---

## 18.1255 Compilação Incremental

O sistema de build poderá usar hashes para decidir recompilação.

---

## 18.1256 Hash de API Pública

Poderá incluir:

* tipos exportados;
* assinaturas;
* layouts públicos;
* slots;
* interfaces;
* constantes relevantes.

---

## 18.1257 Hash de Implementação

Mudanças internas não deverão necessariamente recompilar dependentes.

---

## 18.1258 Separação de Hashes

Poderão existir:

```text
public_api_hash
layout_hash
codegen_hash
implementation_hash
```

---

## 18.1259 Build Reproduzível

Os hashes deverão ser independentes de:

* caminhos absolutos;
* timestamps;
* ordem não determinística;
* endereços.

---

## 18.1260 Caminhos de Fonte

Quando incluídos para debug, deverão ser normalizados ou excluídos dos hashes semânticos.

---

## 18.1261 Compatibilidade entre Versões do Compilador

Duas versões do compilador poderão produzir artefatos compatíveis se:

* usam a mesma ABI;
* emitem metadata compatível;
* concordam sobre layouts;
* usam o mesmo Runtime contract.

---

## 18.1262 Versão do Compilador Não É ABI

Uma nova versão do compilador não implica necessariamente nova ABI.

---

## 18.1263 Build ID

O artefato poderá registrar a versão exata do compilador para diagnóstico.

---

## 18.1264 Mistura de Compiladores

Durante o bootstrap, a mistura deverá ser desencorajada ou rejeitada.

---

## 18.1265 Compilador de Estágio

O processo de auto-hospedagem poderá envolver:

```text
stage0
stage1
stage2
```

Todos deverão declarar a ABI emitida.

---

## 18.1266 Compatibilidade entre Estágios

Stage1 e stage2 deverão produzir artefatos semanticamente equivalentes.

---

## 18.1267 Bootstrap Reproducível

A Toolchain deverá comparar:

* metadata;
* IR normalizada;
* layouts;
* símbolos;
* resultados de testes.

---

## 18.1268 Byte-for-Byte

Igualdade binária completa será desejável, mas poderá ser afetada por:

* identificadores;
* ordem de seções;
* versões de linker;
* debug info.

---

## 18.1269 Comparação Normalizada

Uma ferramenta deverá poder remover diferenças irrelevantes.

---

## 18.1270 Cranelift como Backend Inicial

Na primeira fase, Cranelift deverá implementar toda a ABI canônica definida pelo compilador.

---

## 18.1271 Cranelift Não Define a ABI Capi

Ele será mecanismo de emissão, não fonte normativa.

---

## 18.1272 LLVM como Backend Posterior

LLVM deverá consumir a mesma IR de baixo nível ou o mesmo modelo de ABI.

---

## 18.1273 Transição sem Reescrita Semântica

A introdução de LLVM não deverá exigir redefinir:

* objetos;
* Domains;
* `ObjectId`;
* vtables;
* destruição;
* construção;
* casts.

---

## 18.1274 Camada de Abstração de Backend

A arquitetura deverá possuir interface semelhante a:

```text
Backend
├── declare_function
├── define_function
├── define_data
├── emit_call
├── emit_indirect_call
├── emit_object_file
└── query_target
```

---

## 18.1275 IR Independente

A IR comum deverá representar operações Capi antes de convertê-las em Cranelift IR ou LLVM IR.

---

## 18.1276 Lowering Compartilhado

Transformações como:

* cálculo de layout;
* seleção de slots;
* construção de metadata;
* classificação de ABI;
* mangling;

deverão ser compartilhadas.

---

## 18.1277 Lowering Específico

Cada backend implementará:

* instruções;
* blocos;
* chamadas;
* relocations;
* debug info;
* object emission.

---

## 18.1278 Teste Cruzado de Backend

Todo teste de ABI relevante deverá executar nos dois backends quando LLVM estiver disponível.

---

## 18.1279 Golden Layouts

Os dumps de layout deverão ser idênticos.

---

## 18.1280 Golden Metadata

As estruturas lógicas deverão ser idênticas, mesmo que os arquivos objetos diferenciem.

---

## 18.1281 Golden Symbols

O mangling deverá ser compartilhado e idêntico.

---

## 18.1282 Diferença de Otimização

LLVM poderá produzir código mais otimizado sem alterar a ABI.

---

## 18.1283 Devirtualização

Pode remover chamadas indiretas localmente, mas as vtables públicas ainda deverão ser emitidas quando exigidas pela ABI.

---

## 18.1284 Eliminação de Metadata

Somente poderá ocorrer quando o linker provar que não é necessária externamente.

---

## 18.1285 LTO

LTO poderá atravessar fronteiras de módulos, mas deverá preservar interfaces externas.

---

## 18.1286 ThinLTO

Poderá ser considerado posteriormente.

---

## 18.1287 Cranelift sem LTO

A ausência de LTO não altera a correção.

---

## 18.1288 Fallback entre Backends

Uma opção de compilação poderá selecionar:

```text
--backend=cranelift
--backend=llvm
```

---

## 18.1289 Artefatos Mistos

Módulos gerados por Cranelift e LLVM poderão ser vinculados se compartilham a mesma ABI.

---

## 18.1290 Teste de Artefatos Mistos

Deverão existir testes:

```text
library via Cranelift + application via LLVM
library via LLVM + application via Cranelift
```

---

## 18.1291 Metadata Independente do Backend

O metadata de módulo não deverá depender de estrutura específica de Cranelift ou LLVM.

---

## 18.1292 Marcação do Backend

O artefato poderá registrar o backend apenas para diagnóstico.

---

## 18.1293 ABI Nativa

Ambos deverão obedecer à mesma ABI nativa do target.

---

## 18.1294 Divergências Permitidas

Podem divergir em:

* ordem de instruções;
* registradores;
* inlining;
* tamanho do código;
* seções internas;
* otimizações.

---

## 18.1295 Divergências Proibidas

Não podem divergir em:

* layout;
* símbolos públicos;
* convenções de chamada;
* offsets;
* ordem de destruição;
* formato de metadata;
* comportamento observável.

---

## 18.1296 Verificador de ABI

A Toolchain deverá possuir utilitário capaz de inspecionar artefatos.

---

## 18.1297 Saída do Verificador

Poderá apresentar:

```text
ABI version
Runtime version
Target
Exported symbols
Type layouts
VTable layouts
ITable layouts
Metadata hashes
```

---

## 18.1298 Comparador de Artefatos

Deverá poder comparar dois módulos e classificar mudanças.

---

## 18.1299 Classificação de Mudanças

Exemplo:

```text
source-compatible
binary-compatible
requires dependent recompilation
ABI-breaking
runtime-breaking
```

---

## 18.1300 Uso em CI

Toda alteração no modelo de objetos deverá executar o comparador.

---

## 18.1301 Fixtures de ABI

O repositório deverá manter pequenos programas compilados representando:

* classes;
* herança;
* interfaces;
* generics;
* Domains;
* construção;
* destruição.

---

## 18.1302 Atualização das Fixtures

Mudanças intencionais exigirão atualização explícita e justificativa.

---

## 18.1303 Snapshot de Layout

Cada fixture poderá possuir arquivo textual canônico.

---

## 18.1304 Snapshot de Símbolos

Também deverá existir lista de símbolos exportados.

---

## 18.1305 Snapshot de Metadata

O metadata lógico deverá ser serializável para inspeção.

---

## 18.1306 Testes Negativos

Artefatos incompatíveis deverão ser rejeitados.

---

## 18.1307 ABI e Biblioteca Padrão

A Biblioteca Padrão será compilada contra a mesma ABI do compilador.

---

## 18.1308 Sysroot Versionado

Cada distribuição da Toolchain deverá possuir sysroot correspondente.

---

## 18.1309 Mistura de Sysroots

O driver deverá rejeitar sysroot incompatível.

---

## 18.1310 Runtime e Biblioteca Padrão

Suas versões deverão ser coordenadas.

---

## 18.1311 Manifesto do Sysroot

Poderá conter:

```text
compiler_compatibility
abi_version
runtime_version
stdlib_version
targets
features
```

---

## 18.1312 Seleção do Sysroot

O driver deverá selecionar por:

* versão;
* target;
* profile;
* backend quando necessário.

---

## 18.1313 ABI e Pacotes

O gerenciador de pacotes deverá registrar a versão de Toolchain usada para gerar artefatos.

---

## 18.1314 Distribuição de Fonte Inicial

Durante o bootstrap, pacotes poderão ser distribuídos prioritariamente em fonte.

Isso reduz pressão por estabilidade binária prematura.

---

## 18.1315 Cache Binário

Caches deverão ser invalidados por:

* ABI;
* target;
* compiler version relevante;
* feature flags;
* layout hash;
* profile.

---

## 18.1316 Artefatos Pré-compilados

Só poderão ser reutilizados quando todas as chaves forem compatíveis.

---

## 18.1317 ABI e Features Condicionais

Features que alterem layouts ou métodos deverão participar dos hashes.

---

## 18.1318 Configuração de Target

Flags como:

* pointer width;
* CPU features;
* panic policy;
* sanitizers;
* debug header;

poderão afetar a compatibilidade.

---

## 18.1319 CPU Features

Normalmente alteram instruções, não ABI.

Exceções deverão ser registradas.

---

## 18.1320 Sanitizers

Podem alterar instrumentação e eventualmente layouts.

Artefatos deverão ser tratados como perfil distinto.

---

## 18.1321 Política de Panic

Se alterar convenções de retorno ou unwinding, fará parte da ABI.

---

## 18.1322 Exceções Futuras

A introdução de unwinding seria grande mudança de ABI.

---

## 18.1323 Estado Inicial sem Unwinding

A ABI do bootstrap deverá assumir fluxo de erro explícito.

---

## 18.1324 ABI e FFI

A fronteira com C deverá utilizar tipos e wrappers estáveis.

---

## 18.1325 Objetos Capi na FFI

O layout interno de objetos não deverá ser exposto diretamente.

---

## 18.1326 Handles Opaque

A FFI deverá preferir:

```text
CapiObjectHandle
CapiDomainHandle
```

---

## 18.1327 Versão da API C

Funções exportadas deverão possuir versão ou prefixo estável.

---

## 18.1328 Estruturas C Públicas

Quando necessárias, deverão usar `repr(C)` equivalente e documentação explícita.

---

## 18.1329 Alteração de Estrutura C

Será quebra de ABI externa.

---

## 18.1330 FFI e `ObjectId`

Poderá ser exposto como estrutura opaca de tamanho fixo, se necessário.

---

## 18.1331 Serialização de `ObjectId`

Não deverá ser considerada estável para persistência ou rede.

---

## 18.1332 Processos Diferentes

Um `ObjectId` somente é significativo no RuntimeContext ou processo correspondente.

---

## 18.1333 Dynamic Libraries Futuras

A estabilização exigirá definir:

* ownership do Runtime;
* unicidade dos metadados;
* registro de tipos;
* visibilidade de símbolos;
* unload;
* Domains atravessando bibliotecas.

---

## 18.1334 Unload de Biblioteca

Não poderá ocorrer enquanto existirem objetos cujos metadados ou métodos estejam nela.

---

## 18.1335 Contagem de Módulos

Uma futura infraestrutura poderá controlar lifetime de bibliotecas.

Isso não será confundido com contagem de referências de objetos.

---

## 18.1336 Plugins

Plugins binários permanecerão fora do bootstrap.

---

## 18.1337 Evolução Futura do Modelo de Objetos

Possíveis evoluções incluem:

* compactação de Domains;
* movimentação física;
* stack allocation;
* objetos pinned;
* concorrência;
* libraries dinâmicas;
* ABI estabilizada;
* metadata compacta;
* CFI;
* reflection opt-in.

---

## 18.1338 Preservação da Identidade

Qualquer evolução deverá preservar que:

```text
ObjectId != endereço físico
```

---

## 18.1339 Compactação

A movimentação poderá atualizar slots sem alterar identidades.

---

## 18.1340 Requisito para Compactação

Nenhum ponteiro persistente comum poderá escapar da resolução temporária.

---

## 18.1341 Stack Allocation

Poderá eliminar slot quando a identidade não for observável.

---

## 18.1342 Materialização Tardia

Um objeto poderá permanecer em SSA e ser materializado somente se escapar.

---

## 18.1343 Compatibilidade Semântica das Otimizações

Essas otimizações não poderão mudar:

* identidade observável;
* ordem de efeitos;
* destruição;
* igualdade;
* lifetime.

---

## 18.1344 Objetos Pinned

Poderão exigir flag ou tipo de alocação distinto.

---

## 18.1345 ABI de Pinning

Se exposto entre módulos, deverá ser versionado.

---

## 18.1346 Concorrência

Poderá exigir:

* estados atômicos;
* ownership de Domain;
* sincronização;
* capabilities reificadas.

---

## 18.1347 Compatibilidade de Runtime Concorrente

Um Runtime concorrente poderá manter ABI de helpers, mas alterar internamente slots e Domains.

---

## 18.1348 Metadata Compacta

Campos raros poderão ser movidos para estruturas auxiliares.

---

## 18.1349 Índices em vez de Ponteiros

Targets como WebAssembly poderão usar índices.

O modelo lógico deverá abstrair essa diferença.

---

## 18.1350 Reflection Opt-In

Poderá emitir metadata adicional sob feature explícita.

---

## 18.1351 Reflection e ABI

Metadata reflexiva deverá possuir versão separada.

---

## 18.1352 Serialização Gerada

Deverá preferir código derivado estaticamente, não reflection obrigatória.

---

## 18.1353 Compatibilidade de Dados Persistidos

Não será derivada automaticamente da ABI de memória.

---

## 18.1354 Formatos Persistentes

Deverão possuir schema e versão próprios.

---

## 18.1355 Evolução de Classes Persistidas

Alterar campos não deverá depender do layout físico do objeto.

---

## 18.1356 Política de Estabilização

A ABI somente deverá ser declarada estável quando:

* o modelo de objetos estiver implementado;
* a auto-hospedagem estiver operacional;
* Cranelift e LLVM forem consistentes;
* testes de múltiplos targets passarem;
* formatos de metadata estiverem maduros;
* casos reais forem avaliados.

---

## 18.1357 Marco de Estabilização

A estabilização deverá ser uma decisão explícita do projeto.

---

## 18.1358 Estabilidade Parcial

Algumas camadas poderão estabilizar antes:

```text
FFI C
Runtime API
Object ABI
Package metadata
```

---

## 18.1359 SemVer da Toolchain

A política de versões da Toolchain deverá informar quais garantias são oferecidas.

---

## 18.1360 Antes da Versão Estável

Mudanças incompatíveis serão permitidas, mas documentadas.

---

## 18.1361 Changelog de ABI

Toda alteração deverá registrar:

* componente;
* motivo;
* impacto;
* migração;
* versão anterior;
* versão nova.

---

## 18.1362 ADR Obrigatório

Mudanças estruturais relevantes deverão possuir ADR.

---

## 18.1363 Migração de ABI

Uma mudança poderá ser realizada por:

* recompilação total;
* compatibilidade temporária;
* adaptador;
* dual ABI;
* conversor de metadata.

---

## 18.1364 Recompilação Total como Estratégia Inicial

Será a solução preferida durante o bootstrap.

---

## 18.1365 Dual ABI

Manter duas versões simultâneas aumenta complexidade e não será prioridade.

---

## 18.1366 Adaptadores

Poderão ser usados para FFI ou transições específicas.

---

## 18.1367 Compatibilidade Retroativa do Runtime

Um Runtime novo poderá futuramente suportar módulos de ABI anterior.

Isso deverá ser explícito.

---

## 18.1368 Compatibilidade Progressiva

Módulo novo em Runtime antigo normalmente deverá ser rejeitado.

---

## 18.1369 Matriz de Compatibilidade

A documentação poderá manter tabela:

```text
Compiler
Runtime
Stdlib
ABI
Backend
Supported
```

---

## 18.1370 Ferramenta `capi doctor`

Poderá verificar:

* Toolchain;
* sysroot;
* Runtime;
* ABI;
* target;
* caches.

---

## 18.1371 Ferramenta `capi abi`

Uma ferramenta futura poderá:

* imprimir ABI;
* comparar artefatos;
* validar módulos;
* gerar relatórios.

---

## 18.1372 Dump de Layout

Exemplo:

```text
class Dog
abi: object-v1
size: 64
align: 8
header: 16
base: Animal
fields:
  name @ 32
  owner @ 48
layout-hash: ...
```

---

## 18.1373 Dump de VTable

Exemplo:

```text
vtable Dog
version: 1
slot 0: Animal::move
slot 1: Dog::speak
slot 2: Dog::fetch
```

---

## 18.1374 Dump de ITable

Exemplo:

```text
itable Dog : Runner
version: 1
slot 0: Dog::run
slot 1: Dog::stop
```

---

## 18.1375 Dump de Metadata

Deverá mostrar campos lógicos sem depender de endereços.

---

## 18.1376 Testes de ABI do `ObjectId`

Deverão validar:

* tamanho;
* alinhamento;
* igualdade;
* hashing;
* niche;
* passagem por função;
* retorno;
* armazenamento em struct.

---

## 18.1377 Testes de Cabeçalho

Deverão validar:

* tamanho;
* offset de metadata;
* payload;
* profiles;
* targets.

---

## 18.1378 Testes de Metadata

Deverão validar:

* versão;
* tamanho;
* campos;
* relocations;
* readonly;
* hash.

---

## 18.1379 Testes de VTable

Deverão comparar slots entre módulos.

---

## 18.1380 Testes de ITable

Deverão validar interfaces herdadas e múltiplas.

---

## 18.1381 Testes de Chamada

Deverão cobrir:

* chamada direta;
* virtual;
* interface;
* thunk;
* base;
* retorno agregado;
* parâmetros ocultos.

---

## 18.1382 Testes de Construtor

Deverão confirmar compatibilidade entre wrapper e Runtime.

---

## 18.1383 Testes de Destrutor

Deverão confirmar seleção concreta e invalidação.

---

## 18.1384 Testes Cross-Module

Uma biblioteca deverá definir tipo e uma aplicação deverá:

* construir;
* chamar;
* fazer cast;
* destruir.

---

## 18.1385 Testes Cross-Backend

A biblioteca e a aplicação deverão alternar backends.

---

## 18.1386 Testes Cross-Version

Quando houver duas versões compatíveis, artefatos deverão ser combinados em CI.

---

## 18.1387 Testes de Rejeição

Versões incompatíveis deverão falhar com diagnóstico controlado.

---

## 18.1388 Testes Multi-Target

Pelo menos:

* x86_64 Linux;
* AArch64 Linux;
* WebAssembly quando suportado.

---

## 18.1389 Testes de Pointer Width

Se houver target 32 bits, layouts deverão ser verificados separadamente.

---

## 18.1390 Testes de Endianness

Quando houver infraestrutura disponível.

---

## 18.1391 Fuzzing de Metadata

O loader de metadata deverá rejeitar:

* tamanho inválido;
* versão desconhecida;
* offsets fora de limite;
* hashes incorretos;
* símbolos ausentes.

---

## 18.1392 Fuzzing de Artefatos

Ferramentas de inspeção não deverão falhar de forma insegura em arquivos corrompidos.

---

## 18.1393 Property-Based Testing de Layout

Propriedades:

1. offsets respeitam alinhamento;
2. campos não se sobrepõem;
3. base é prefixo;
4. tamanho cobre todos os campos;
5. backend concorda com layout canônico;
6. hash muda quando layout muda;
7. layout idêntico produz hash idêntico.

---

## 18.1394 Property-Based Testing de ABI

Propriedades:

1. módulos compatíveis são aceitos;
2. versão maior incompatível é rejeitada;
3. target distinto é rejeitado;
4. hash divergente é rejeitado;
5. símbolos públicos permanecem determinísticos;
6. backends produzem contratos equivalentes.

---

## 18.1395 Sanitizers e ABI

Builds instrumentados deverão possuir identificação própria.

---

## 18.1396 Miri no Runtime

Estruturas `repr(C)` deverão ser validadas quando possível.

---

## 18.1397 Bindings de Teste em C

Pequenos programas C poderão validar a FFI pública.

---

## 18.1398 Verificação de Símbolos

Ferramentas como leitor de objetos poderão confirmar:

* nomes;
* visibilidade;
* seções;
* relocations.

---

## 18.1399 CI de Compatibilidade

O pipeline deverá executar:

```text
layout tests
metadata tests
cross-backend tests
cross-module tests
sysroot compatibility
ABI snapshots
```

---

## 18.1400 Bloqueio de Merge

Mudanças não justificadas em snapshots deverão falhar.

---

## 18.1401 Revisão Manual

Mudanças intencionais deverão receber revisão específica de ABI.

---

## 18.1402 Relatório de Impacto

O autor da mudança deverá indicar:

* recompilação necessária;
* componentes afetados;
* impacto em performance;
* impacto em memória;
* impacto em FFI.

---

## 18.1403 Invariantes de Compatibilidade

A implementação deverá preservar:

1. módulos somente são combinados quando suas ABIs são compatíveis;
2. layouts são calculados de forma determinística;
3. Cranelift e LLVM compartilham o mesmo contrato;
4. `ObjectId` não é reinterpretado como inteiro arbitrário;
5. `TypeMetadata` possui versão explícita;
6. slots virtuais e de interface são validados;
7. o Runtime não interpreta metadata desconhecida;
8. o sysroot corresponde à Toolchain;
9. mudanças incompatíveis exigem nova versão ou recompilação;
10. otimizações não alteram contratos públicos;
11. o target faz parte da chave de compatibilidade;
12. profiles incompatíveis não são misturados;
13. hashes não dependem de dados não determinísticos;
14. a ABI interna não é confundida com semântica da linguagem;
15. estabilidade somente é declarada por decisão explícita.

---

## 18.1404 Invariantes da Transição Cranelift → LLVM

1. o modelo de objetos permanece o mesmo;
2. a IR semântica permanece comum;
3. layouts permanecem idênticos;
4. símbolos públicos permanecem idênticos;
5. Runtime e stdlib permanecem compartilháveis;
6. chamadas indiretas preservam assinaturas;
7. construção e destruição preservam ordem;
8. `ObjectId` preserva representação;
9. metadata permanece backend-independent;
10. diferenças limitam-se à qualidade do código gerado.

---

## 18.1405 ADRs Recomendados

Deverão ser considerados:

```text
ADR — ABI interna versionada durante o bootstrap
ADR — Recompilação integral antes da estabilização
ADR — Layout canônico calculado fora do backend
ADR — Metadata de Runtime com versão e tamanho
ADR — ObjectId como tipo próprio na IR
ADR — Slot interno opaco ao código gerado
ADR — Helpers de Runtime versionados
ADR — Hashes separados de API, layout e implementação
ADR — Sysroot vinculado à ABI
ADR — Metadata de módulo em seção própria
ADR — Cranelift e LLVM compartilhando o mesmo classificador de ABI
ADR — Artefatos mistos entre backends
ADR — Estabilização binária somente após auto-hospedagem
```

---

## 18.1406 Critérios de Conclusão da ABI Inicial

A ABI inicial estará definida quando:

* `ObjectId` possuir representação documentada;
* `DomainId` possuir representação documentada;
* `ObjectHeader` possuir layout canônico;
* `TypeMetadata` possuir versão;
* vtables e itables possuírem layouts definidos;
* chamadas possuírem convenções explícitas;
* construtores e destrutores possuírem ABI interna;
* Runtime helpers estiverem versionados;
* o target fizer parte da validação;
* testes de layout passarem.

---

## 18.1407 Critérios de Conclusão da Compatibilidade entre Módulos

Estará concluída quando:

* módulos emitirem metadata de ABI;
* dependências forem validadas;
* hashes públicos forem comparados;
* layouts divergentes forem rejeitados;
* símbolos forem determinísticos;
* cross-module tests passarem;
* diagnósticos forem claros.

---

## 18.1408 Critérios de Conclusão da Compatibilidade entre Toolchain e Runtime

Estará concluída quando:

* o binário declarar a ABI requerida;
* o Runtime declarar a ABI oferecida;
* incompatibilidades falharem antes da execução comum;
* o sysroot for validado;
* a Biblioteca Padrão utilizar a mesma versão;
* helpers tiverem contratos testados;
* profiles incompatíveis forem rejeitados.

---

## 18.1409 Critérios de Conclusão da Transição para LLVM

A transição estará concluída quando:

* LLVM consumir o mesmo modelo de layout;
* LLVM utilizar o mesmo mangling;
* LLVM emitir os mesmos metadados lógicos;
* bibliotecas entre backends forem interoperáveis;
* resultados diferenciais forem idênticos;
* testes de construção e destruição passarem;
* chamadas virtuais e interfaces funcionarem;
* LTO for opcional;
* Cranelift continuar suportado como backend rápido.

---

## 18.1410 Critérios para Estabilização Futura

A ABI poderá ser considerada candidata à estabilização quando:

* a auto-hospedagem estiver completa;
* o compilador Capi compilar a si próprio de forma confiável;
* múltiplos targets forem suportados;
* Cranelift e LLVM estiverem maduros;
* mudanças de layout forem raras;
* ferramentas de inspeção existirem;
* política SemVer estiver definida;
* casos reais de bibliotecas forem avaliados;
* FFI estiver consolidada;
* o projeto aceitar o custo de compatibilidade retroativa.

---

## 18.1411 Resultado Esperado da Terceira Parte — Bloco 3.1

Ao final desta etapa, a implementação oficial deverá possuir uma arquitetura em que:

```text
Semântica
    define as garantias da linguagem

ABI Canônica
    define a representação compartilhada

Metadata Versionada
    permite validação entre artefatos

Layout Hash
    detecta divergências estruturais

Runtime Version
    identifica os contratos executáveis

Sysroot
    mantém Runtime e Biblioteca Padrão coerentes

Cranelift
    implementa o backend inicial de compilação rápida

LLVM
    implementa o backend posterior de otimização avançada

Recompilação Integral
    permite evolução durante o bootstrap

Estabilização Explícita
    impede promessas binárias prematuras
```

O modelo deverá permitir que a implementação evolua sem transformar detalhes acidentais em contratos permanentes.

Ao mesmo tempo, nenhuma mudança capaz de alterar:

* layouts;
* identidades;
* metadados;
* chamadas;
* construção;
* destruição;
* despacho;
* interfaces;
* Runtime;

poderá ocorrer silenciosamente.

A próxima parte deverá tratar das otimizações, instrumentação, verificação, análise de performance e ferramentas necessárias para tornar esse modelo eficiente sem comprometer suas garantias.

---

## Terceira Parte — Bloco 3.2 — Otimizações, Instrumentação e Verificação

## 18.1412 Objetivo

Esta parte define as estratégias de otimização, instrumentação, validação e medição aplicáveis ao modelo de objetos da implementação oficial da Capi.

Serão definidos:

* princípios gerais de otimização;
* limites semânticos das transformações;
* devirtualização;
* eliminação de resoluções redundantes;
* otimização de `ObjectId`;
* especialização de layouts;
* escape analysis;
* stack allocation;
* scalar replacement;
* eliminação de alocações;
* otimização de Domains;
* otimização de construção e destruição;
* inline caches;
* metadata compacta;
* instrumentação de Runtime;
* tracing;
* métricas;
* benchmarks;
* verificadores;
* sanitizers;
* fuzzing;
* property-based testing;
* análise diferencial entre backends;
* ferramentas de inspeção;
* critérios de conclusão.

O objetivo será tornar o modelo de objetos eficiente sem alterar:

* identidade;
* lifetime;
* ordem de efeitos;
* regras de construção;
* regras de destruição;
* subtipagem;
* despacho;
* segurança de memória;
* semântica observável.

---

## 18.1413 Princípio da Otimização Semântica

Toda otimização deverá obedecer ao seguinte princípio:

> Uma transformação somente poderá eliminar, combinar, mover ou especializar operações quando houver prova suficiente de que o comportamento observável do programa permanece equivalente.

---

## 18.1414 Otimização Não É Garantia da Linguagem

A especificação define o comportamento.

A implementação poderá ou não aplicar uma otimização específica.

O programa não poderá depender da presença da otimização.

---

## 18.1415 Independência entre Correção e Performance

A ausência de uma otimização poderá reduzir:

* velocidade;
* uso de memória;
* tamanho de código;

mas não poderá alterar a correção.

---

## 18.1416 Camadas de Otimização

As otimizações poderão ocorrer em:

```text
HIR
MIR
IR Capi
lowering de ABI
backend
linker
Runtime
Biblioteca Padrão
```

---

## 18.1417 Otimizações no Frontend

O Frontend deverá limitar-se a simplificações seguras, como:

* folding de constantes;
* remoção de construções sintáticas;
* normalização;
* resolução estática.

Não deverá decidir detalhes físicos do backend.

---

## 18.1418 Otimizações no Middle-end

O Middle-end será responsável por transformações semânticas como:

* devirtualização;
* escape analysis;
* propagação de tipo concreto;
* eliminação de checks;
* scalar replacement;
* simplificação de Domains;
* especialização de generics.

---

## 18.1419 Otimizações no Backend

Cranelift e LLVM poderão aplicar:

* instruction selection;
* register allocation;
* inlining;
* common subexpression elimination;
* dead code elimination;
* loop optimization;
* vectorization;
* scheduling.

---

## 18.1420 Otimizações no Runtime

O Runtime poderá otimizar:

* lookup de slots;
* alocação;
* free lists;
* metadata lookup;
* interface lookup;
* publicação;
* destruição coletiva.

---

## 18.1421 Limites Semânticos

Nenhuma otimização poderá alterar:

1. igualdade de `ObjectId`;
2. estabilidade da identidade;
3. ordem observável de construtores;
4. ordem observável de destrutores;
5. lifetime de recursos;
6. comportamento de casts;
7. resolução de interfaces;
8. efeitos externos;
9. falhas observáveis;
10. regras de borrowing.

---

## 18.1422 Ordem de Efeitos

Operações que possam produzir efeitos não deverão ser reordenadas de forma observável.

Exemplos:

* I/O;
* logging;
* FFI;
* criação de objetos;
* destruição;
* panic;
* atualização de estado compartilhado futuro.

---

## 18.1423 Operações Puramente Internas

Operações como:

* cálculo de offset;
* leitura de metadata imutável;
* comparação de tipos;
* cálculo de slot;

poderão ser reordenadas quando não alterarem o resultado.

---

## 18.1424 Perfil de Otimização

A Toolchain deverá suportar, no mínimo:

```text
debug
release
release-with-debug
checked
```

---

## 18.1425 Perfil `debug`

Deverá priorizar:

* diagnósticos;
* validações;
* preservação de estrutura;
* símbolos;
* tracing opcional;
* checks de estado.

---

## 18.1426 Perfil `release`

Deverá priorizar:

* performance;
* eliminação de checks provados;
* inlining;
* compactação;
* redução de código redundante.

---

## 18.1427 Perfil `checked`

Poderá manter validações de Runtime mesmo com otimizações.

---

## 18.1428 Perfil `release-with-debug`

Deverá manter informações de depuração sem necessariamente preservar todas as estruturas de alto nível.

---

## 18.1429 Flags de Otimização

A Toolchain poderá oferecer níveis conceituais:

```text
-O0
-O1
-O2
-O3
-Os
-Oz
```

A semântica exata poderá variar entre backends.

---

## 18.1430 Consistência entre Backends

Níveis equivalentes não precisam produzir código idêntico, mas deverão preservar os mesmos contratos.

---

# 18.1431 Devirtualização

Devirtualização é a substituição de uma chamada dinâmica por chamada direta quando o alvo pode ser determinado estaticamente.

---

## 18.1432 Benefícios da Devirtualização

Ela permite:

* eliminar lookup em vtable;
* inlinear o método;
* propagar constantes;
* eliminar casts;
* especializar campos;
* reduzir branches indiretos.

---

## 18.1433 Fontes de Prova para Devirtualização

A Toolchain poderá usar:

* classe `final`;
* método `final`;
* tipo concreto local;
* objeto recém-construído;
* análise de fluxo;
* closed-world analysis;
* monomorfização;
* análise interprocedural;
* LTO.

---

## 18.1434 Classe `final`

Chamadas virtuais sobre instância de classe final poderão ser convertidas diretamente.

---

## 18.1435 Método `final`

Um método final não poderá ser sobrescrito e poderá ser chamado diretamente.

---

## 18.1436 Tipo Concreto Conhecido

Se a análise comprovar que o valor sempre contém `Dog`, uma chamada por referência `Animal` poderá ser devirtualizada.

---

## 18.1437 Objeto Recém-Criado

Após:

```text
new Dog(...)
```

uma chamada imediata ao método virtual poderá utilizar implementação de `Dog`.

---

## 18.1438 Propagação de Tipo

O Middle-end poderá associar conjuntos de tipos possíveis a valores.

---

## 18.1439 Conjunto Unitário

Quando o conjunto possuir um único tipo concreto, a chamada poderá ser direta.

---

## 18.1440 Conjunto Pequeno

Quando houver poucos tipos possíveis, poderá ser gerado despacho por testes explícitos.

---

## 18.1441 Despacho Especulativo

Uma estratégia futura poderá gerar:

```text
if type == Dog:
    call Dog::method
else:
    virtual call
```

---

## 18.1442 Fallback Obrigatório

O caminho genérico deverá permanecer quando a prova não for absoluta.

---

## 18.1443 Closed-World Analysis

Em binários totalmente estáticos, a Toolchain poderá conhecer todos os subtipos existentes.

---

## 18.1444 Limitação do Closed World

A análise não poderá ser usada quando houver:

* bibliotecas dinâmicas;
* plugins;
* carregamento posterior;
* FFI que introduza implementações;
* reflection capaz de criar tipos.

---

## 18.1445 Devirtualização por LTO

LLVM poderá devirtualizar entre módulos quando todos os corpos estiverem disponíveis.

---

## 18.1446 Devirtualização no Cranelift

A fase Cranelift deverá suportar ao menos casos locais e explícitos.

---

## 18.1447 Preservação da VTable

Mesmo que todas as chamadas locais sejam diretas, a vtable deverá ser emitida se:

* o tipo for exportado;
* puder cruzar fronteira de módulo;
* existir cast dinâmico;
* a ABI exigir.

---

## 18.1448 Devirtualização de Interface

Chamadas por interface também poderão ser especializadas quando o tipo concreto for conhecido.

---

## 18.1449 Thunk Eliminado

Se a chamada puder usar diretamente a implementação concreta com ABI compatível, o thunk poderá ser eliminado localmente.

---

## 18.1450 Testes de Devirtualização

Deverão validar:

* classe final;
* método final;
* tipo concreto;
* múltiplos tipos;
* fallback;
* herança;
* interface;
* cross-module;
* LTO.

---

# 18.1451 Inline Caches

Inline cache é uma otimização para chamadas dinâmicas repetidas no mesmo ponto.

---

## 18.1452 Cache Monomórfico

Um call site poderá armazenar:

```text
último TypeId
último target
```

---

## 18.1453 Caminho Rápido

Se o tipo atual corresponder ao cache, o alvo será chamado diretamente.

---

## 18.1454 Cache Polimórfico

Um call site poderá armazenar poucos pares:

```text
TypeId → target
```

---

## 18.1455 Cache Megamórfico

Quando muitos tipos ocorrerem, o call site deverá voltar ao lookup genérico.

---

## 18.1456 Escopo Inicial

Inline caches não serão necessários no bootstrap.

---

## 18.1457 Justificativa

O modelo AOT, a devirtualização estática e a vtable direta deverão ser priorizados antes de caches dinâmicos.

---

## 18.1458 Mutabilidade do Código

Caches não deverão exigir modificação insegura de código executável.

---

## 18.1459 Cache em Dados

Quando implementados, deverão utilizar estrutura de dados apropriada.

---

## 18.1460 Concorrência Futura

Caches compartilhados exigirão sincronização ou atualização atômica.

---

## 18.1461 ABI e Inline Cache

Inline caches deverão ser detalhe de implementação do call site, não parte da ABI pública.

---

# 18.1462 Otimização de Lookup de Interface

O lookup de interface poderá ser otimizado por diferentes estratégias.

---

## 18.1463 Busca Linear

Adequada para tipos com poucas interfaces.

---

## 18.1464 Busca Binária

Requer itables ordenadas por `InterfaceId`.

---

## 18.1465 Hash Table

Pode reduzir lookup, mas aumenta metadata e custo de construção.

---

## 18.1466 Perfect Hash

Poderá ser gerado para tipos com muitas interfaces.

---

## 18.1467 Slot Direto Conhecido

Quando a interface e o tipo concreto forem conhecidos, o lookup poderá ser eliminado.

---

## 18.1468 Cache de Interface no Call Site

Poderá seguir estratégia semelhante a inline cache.

---

## 18.1469 Estratégia Inicial

A implementação inicial deverá usar:

* tabela pequena;
* ordenação determinística;
* busca linear ou binária;
* especialização estática quando possível.

---

## 18.1470 Métrica de Interfaces

O Runtime deverá permitir medir:

* número médio de interfaces por tipo;
* custo de lookup;
* taxa de cache;
* tamanho das itables.

---

# 18.1471 Eliminação de Resoluções Redundantes

Resolver um `ObjectId` repetidamente pode introduzir custo desnecessário.

---

## 18.1472 Resolução Única por Escopo

Quando um empréstimo válido cobre múltiplos acessos, o compilador poderá resolver uma vez.

---

## 18.1473 Reutilização de Ponteiro Resolvido

O ponteiro temporário poderá ser reutilizado enquanto:

* o objeto não puder ser destruído;
* o Domain não puder mover o objeto;
* o borrow permanecer válido;
* nenhuma operação invalidante ocorrer.

---

## 18.1474 Barreira de Invalidação

Operações que podem invalidar o ponteiro incluem:

* destruição antecipada;
* finalização do Domain;
* compactação futura;
* chamada desconhecida com autoridade;
* transferência;
* FFI.

---

## 18.1475 Análise de Alias

O compilador deverá provar que nenhuma operação intermediária invalida o objeto.

---

## 18.1476 Common Subexpression Elimination

Resoluções equivalentes poderão ser combinadas quando suas condições permanecerem válidas.

---

## 18.1477 Hoisting

Uma resolução poderá ser movida para fora de loop se o objeto permanecer estável.

---

## 18.1478 Proibição de Hoisting Inseguro

Não poderá atravessar operação que possa destruir ou mover o objeto.

---

## 18.1479 Resolução Checked

O primeiro acesso poderá executar validação completa.

---

## 18.1480 Resoluções Subsequentes

Poderão usar ponteiro já validado dentro do mesmo borrow.

---

## 18.1481 Build de Debug

Poderá manter checks adicionais para confirmar a prova.

---

## 18.1482 Compactação Futura

Quando objetos puderem mover, o escopo de validade do ponteiro deverá permanecer curto.

---

# 18.1483 Otimização de `ObjectId`

O compilador deverá preservar `ObjectId` como identidade semântica, mas poderá otimizar sua representação local.

---

## 18.1484 Propagação em Registradores

Componentes de `ObjectId` poderão permanecer em registradores separados.

---

## 18.1485 Comparação Especializada

Igualdade poderá ser reduzida a uma ou mais comparações nativas.

---

## 18.1486 Hash Especializado

O hash poderá ser inlineado para coleções.

---

## 18.1487 Eliminação de Extração

Quando o Domain ou slot já forem conhecidos separadamente, não será necessário reconstruir e desmontar o `ObjectId`.

---

## 18.1488 Identidade Constante

Identidades de objetos estáticos futuros poderão ser constantes relocáveis.

---

## 18.1489 Optional com Niche

`Optional<ObjectId<T>>` poderá ocupar o mesmo tamanho de `ObjectId` se houver representação inválida disponível.

---

## 18.1490 Validação do Niche

A otimização deverá estar registrada no layout canônico.

---

## 18.1491 Ausência de Compressão Invisível entre Módulos

Representações compactadas localmente deverão ser convertidas na fronteira ABI.

---

# 18.1492 Escape Analysis

Escape analysis determina se um valor ou objeto ultrapassa determinado escopo.

---

## 18.1493 Tipos de Escape

A análise poderá distinguir:

```text
no escape
escapes to caller
escapes to object
escapes to Domain
escapes globally
escapes to FFI
escapes to unknown call
```

---

## 18.1494 Objeto Sem Escape

Um objeto que não escapa poderá ser candidato a:

* stack allocation;
* scalar replacement;
* eliminação completa;
* remoção de slot;
* devirtualização.

---

## 18.1495 Escape pelo Retorno

Retornar `ObjectId` normalmente implica escape.

---

## 18.1496 Escape por Campo

Armazenar identidade em objeto vivo implica escape para o lifetime desse objeto ou Domain.

---

## 18.1497 Escape por Coleção

Inserir em coleção persistente implica escape.

---

## 18.1498 Escape por FFI

Deverá ser tratado de forma conservadora.

---

## 18.1499 Escape por Chamada Desconhecida

Sem resumo de efeitos, o compilador deverá presumir escape.

---

## 18.1500 Resumos de Função

Funções poderão exportar metadata indicando:

* captura parâmetros;
* armazena identidade;
* retorna alias;
* destrói;
* não escapa;
* pureza.

---

## 18.1501 Anotações Futuras

A linguagem poderá possuir contratos como:

```text
noescape
borrows
consumes
```

---

## 18.1502 Análise Interprocedural

Poderá usar corpos disponíveis ou resumos.

---

## 18.1503 Cranelift

A implementação inicial poderá limitar-se à análise intraprocedural.

---

## 18.1504 LLVM

Poderá aproveitar otimizações interprocedurais adicionais.

---

## 18.1505 Correção Conservadora

Quando houver dúvida, a análise deverá considerar que o objeto escapa.

---

# 18.1506 Stack Allocation

Objetos por identidade poderão ser alocados na pilha somente quando a identidade não precisar sobreviver ao escopo.

---

## 18.1507 Requisito de Não Escape

A stack allocation exige prova de que:

* nenhum `ObjectId` observável escapa;
* nenhum ponteiro é retido;
* nenhuma FFI captura o objeto;
* o lifetime termina antes do frame;
* a identidade não é comparada externamente.

---

## 18.1508 Identidade Local

O compilador poderá manter uma identidade sintética apenas dentro do escopo.

---

## 18.1509 Eliminação de `ObjectId`

Se a identidade não é observável, o objeto poderá ser tratado como valor interno.

---

## 18.1510 Preservação de Identidade Local

Duas referências locais ao mesmo objeto deverão continuar reconhecendo a mesma instância.

---

## 18.1511 Materialização em Domain

Se uma análise posterior detectar escape, o objeto deverá ser materializado no Domain apropriado.

---

## 18.1512 Construção na Pilha

A sequência de construção permanece:

* cabeçalho, se necessário;
* base;
* campos;
* construtor;
* publicação lógica local.

---

## 18.1513 Destruição na Pilha

O drop deverá ocorrer ao fim do escopo em ordem normal.

---

## 18.1514 Ausência de Registro no Domain

Quando completamente local e não observável, poderá não ser registrado no Domain.

---

## 18.1515 Interação com Destruição Antecipada

A destruição poderá ser convertida em drop local.

---

## 18.1516 Classes com FFI

Objetos cujo endereço é passado à FFI não poderão mover e normalmente não serão candidatos.

---

## 18.1517 Classes Grandes

Stack allocation deverá respeitar limites de stack.

---

## 18.1518 Heurística de Tamanho

A Toolchain poderá evitar stack allocation acima de limite configurável.

---

## 18.1519 Loops

Alocações em loops deverão ser analisadas para evitar crescimento de stack.

---

## 18.1520 Recursão

Objetos grandes em funções recursivas podem causar overflow.

---

## 18.1521 Escopo Inicial

Stack allocation não será requisito do primeiro bootstrap.

---

# 18.1522 Scalar Replacement

Scalar replacement substitui um objeto ou agregado por seus campos separados.

---

## 18.1523 Condições

É aplicável quando:

* o objeto não escapa;
* o layout é conhecido;
* a identidade não é observável externamente;
* chamadas podem ser inlineadas ou analisadas;
* não há FFI dependente de endereço.

---

## 18.1524 Representação em SSA

Campos poderão existir como valores SSA independentes.

---

## 18.1525 Ausência de Alocação

Nenhum armazenamento físico será criado.

---

## 18.1526 Chamadas de Método

Métodos poderão ser inlineados e operar diretamente nos valores.

---

## 18.1527 Mutabilidade

Atualizações de campo tornam-se novas versões SSA.

---

## 18.1528 Identidade Local

Comparações de identidade locais deverão continuar consistentes.

---

## 18.1529 Materialização Tardia

Se o endereço ou identidade se tornar necessário, o compilador poderá materializar o objeto.

---

## 18.1530 Destruição

Drops dos campos deverão ocorrer mesmo sem objeto materializado.

---

## 18.1531 Campos de Recurso

Não poderão ser eliminados se possuem efeitos de destruição.

---

## 18.1532 Objeto Sem Efeitos

Se nenhum campo ou construtor possui efeito e o resultado não é usado, toda a construção poderá ser eliminada.

---

# 18.1533 Eliminação de Alocação

Uma alocação poderá ser eliminada quando sua existência física não for observável.

---

## 18.1534 Observabilidade Física

Código Capi seguro não deverá observar endereços permanentes de objetos.

Isso favorece a otimização.

---

## 18.1535 Observabilidade por FFI

A FFI pode tornar o endereço observável e bloquear a eliminação.

---

## 18.1536 Observabilidade por `ObjectId`

O `ObjectId` observa identidade, não endereço.

Ainda assim, se a identidade escapar, a alocação lógica deverá existir.

---

## 18.1537 Eliminação Parcial

O slot poderá existir sem payload materializado em casos altamente especializados.

Não será prioridade.

---

## 18.1538 Alocação Combinada

Objetos locais relacionados poderão compartilhar uma única reserva física quando:

* seus lifetimes coincidem;
* nenhum endereço individual é exposto;
* offsets permanecem conhecidos.

---

## 18.1539 Risco de Alocação Combinada

Destruição antecipada e movimentação podem tornar a estratégia complexa.

---

## 18.1540 Escopo Inicial

Não será requisito do bootstrap.

---

# 18.1541 Otimização de Construção

A sequência de construção poderá ser especializada sem alterar seus estados semânticos.

---

## 18.1542 Reserva e Alocação Combinadas

O helper poderá reservar slot e memória em uma operação.

---

## 18.1543 Inicialização do Cabeçalho Inline

Para layout estável, o código gerado poderá escrever o cabeçalho diretamente.

---

## 18.1544 Publicação Inline

Quando o Runtime contract estiver estabilizado, a publicação poderá ser parcialmente inlineada.

---

## 18.1545 Construtor Trivial

Uma classe com:

* base trivial;
* campos triviais;
* sem corpo;
* sem falha;

poderá ser inicializada diretamente.

---

## 18.1546 Eliminação do Wrapper

O wrapper de criação poderá ser inlineado no call site.

---

## 18.1547 Eliminação de Cleanup Inalcançável

Se a construção não pode falhar, blocos de cleanup poderão ser removidos.

---

## 18.1548 Propagação de Falibilidade

O compilador deverá conhecer quais inicializadores podem falhar.

---

## 18.1549 Drop Flags Estáticas

Quando o fluxo comprova o progresso, flags dinâmicas poderão ser eliminadas.

---

## 18.1550 Construção In-Place

Valores poderão ser construídos diretamente nos campos.

---

## 18.1551 Construção Vetorizada

Arrays de valores triviais poderão utilizar inicialização em bloco.

---

## 18.1552 Zero Initialization

Somente poderá substituir inicialização explícita quando zero representar exatamente o valor definido.

---

## 18.1553 `memset`

Poderá ser usado para campos triviais apropriados.

---

## 18.1554 Cópia em Bloco

Structs copiáveis poderão utilizar cópia eficiente quando a ABI permitir.

---

# 18.1555 Otimização de Destruição

Drops poderão ser simplificados quando sua ausência de efeitos for comprovada.

---

## 18.1556 Eliminação de Drop Trivial

Tipos trivialmente destrutíveis não exigirão chamada.

---

## 18.1557 Inlining de Drop Glue

Drops pequenos poderão ser inlineados.

---

## 18.1558 Fusão de Drops de Campos

Campos contíguos triviais não exigirão operações individuais.

---

## 18.1559 Loop de Drop

Arrays e coleções poderão usar loops especializados.

---

## 18.1560 Eliminação de Estado `Finalizing`

Em objetos sem aliases observáveis e destruição local, certas transições de Runtime poderão ser removidas.

---

## 18.1561 Preservação da Invalidação

Se existe `ObjectId` observável, o slot deverá ser invalidado.

---

## 18.1562 Destruição Coletiva Trivial

Domains contendo somente objetos triviais poderão liberar blocos diretamente após invalidar registros.

---

## 18.1563 Detecção de Domain Trivial

O Domain poderá manter flag:

```text
contains_nontrivial_objects
```

---

## 18.1564 Atualização da Flag

A publicação de objeto não trivial deverá ativá-la.

---

## 18.1565 Limitação

Objetos destruídos antecipadamente podem tornar a flag conservadora.

Isso é aceitável.

---

## 18.1566 Destruição por Tipo Agrupado

Uma otimização futura poderá agrupar objetos por destrutor.

---

## 18.1567 Ordem Observável

O agrupamento somente será permitido se não alterar a ordem normativa.

Portanto, normalmente não será aplicável quando destrutores têm efeitos.

---

# 18.1568 Otimização de Domains

Domains são candidatos a otimizações por sua natureza coletiva.

---

## 18.1569 Alocação em Blocos

O tamanho dos blocos poderá ser adaptado conforme:

* perfil;
* tipo;
* histórico;
* tamanho de objeto;
* padrão de uso.

---

## 18.1570 Bloco Inicial

Domains pequenos deverão evitar reservar grandes volumes desnecessários.

---

## 18.1571 Crescimento Geométrico

Blocos posteriores poderão crescer geometricamente.

---

## 18.1572 Limite Máximo

Deverá existir limite para evitar blocos excessivamente grandes.

---

## 18.1573 Objetos Grandes

Poderão receber bloco dedicado.

---

## 18.1574 Alinhamento

O alocador deverá evitar padding excessivo.

---

## 18.1575 Segregação por Alinhamento

Uma evolução futura poderá manter classes de alinhamento.

---

## 18.1576 Free List de Slots

Slots destruídos antecipadamente poderão ser reutilizados.

---

## 18.1577 Free List de Memória

A primeira implementação poderá não reutilizar memória interna.

---

## 18.1578 Reutilização de Memória

Poderá ser adicionada para Domains longos com destruição antecipada.

---

## 18.1579 Classes de Tamanho

Uma estratégia futura poderá agrupar espaços livres por tamanho.

---

## 18.1580 Fragmentação

Deverão ser medidas:

* fragmentação interna;
* fragmentação externa;
* bytes abandonados;
* slots livres;
* blocos parcialmente usados.

---

## 18.1581 Compactação

A identidade desacoplada do endereço permite compactação futura.

---

## 18.1582 Barreiras para Compactação

Deverão ser controlados:

* ponteiros emprestados;
* FFI;
* pinning;
* concorrência;
* referências internas temporárias.

---

## 18.1583 Compactação em Ponto Seguro

Poderá ocorrer apenas quando não houver borrows ativos.

---

## 18.1584 Atualização de Slots

Após mover o objeto, o slot deverá apontar para o novo endereço.

---

## 18.1585 Campos com Ponteiros Internos

A linguagem deverá evitar ponteiros físicos persistentes entre objetos.

---

## 18.1586 Autorreferência Física

Estruturas autorreferenciais por ponteiro físico impedem movimento e deverão exigir pinning.

---

## 18.1587 Escopo Inicial

Compactação ficará fora do bootstrap.

---

## 18.1588 Pool de Objetos

Tipos muito frequentes e homogêneos poderão usar pools especializados.

---

## 18.1589 Compatibilidade com Domain

O pool deverá permanecer subordinado ao lifetime do Domain.

---

## 18.1590 Pool Não Observável

A escolha de pool não poderá alterar identidade ou semântica.

---

## 18.1591 Pool por Tipo

Pode melhorar:

* localidade;
* tamanho;
* destruição;
* cache.

---

## 18.1592 Risco de Complexidade

A estratégia aumenta:

* caminhos de alocação;
* metadata;
* fragmentação entre pools.

Não será prioridade inicial.

---

# 18.1593 Otimização de Metadata

Metadata deverá ser mínima, mas suficiente.

---

## 18.1594 Eliminação de Nomes em Release

Nomes de debug poderão ser removidos ou colocados em seção separada.

---

## 18.1595 Deduplicação de Strings

Nomes repetidos poderão compartilhar tabela.

---

## 18.1596 Compactação de Flags

Flags booleanas poderão ser agrupadas em bitset.

---

## 18.1597 Índices Relativos

Ponteiros para estruturas próximas poderão ser substituídos por offsets.

---

## 18.1598 Benefício de Offsets

Pode reduzir relocations e tamanho em targets 64 bits.

---

## 18.1599 Risco de Offsets

Exige validação de alcance e base de referência estável.

---

## 18.1600 Metadata Fria

Informações raramente usadas poderão ficar separadas.

---

## 18.1601 Metadata Quente

Campos usados frequentemente, como:

* destructor;
* vtable;
* size;
* flags;

deverão permanecer próximos.

---

## 18.1602 Layout Orientado a Cache

A ordem poderá ser ajustada com base em perfis, desde que a versão da ABI acompanhe.

---

## 18.1603 Tabela Global de Tipos

Uma evolução futura poderá consolidar metadados.

---

## 18.1604 Metadata Local por Módulo

A implementação inicial poderá emitir constantes independentes.

---

## 18.1605 Deduplicação pelo Linker

Poderá reduzir cópias idênticas.

---

# 18.1606 Profile-Guided Optimization

PGO poderá usar dados de execução real.

---

## 18.1607 Dados Relevantes

Poderão ser coletados:

* tipos observados em call sites;
* frequência de métodos;
* tamanho de Domains;
* padrões de alocação;
* taxas de falha;
* destrutores frequentes;
* casts;
* lookups de interface.

---

## 18.1608 Uso para Devirtualização

Call sites monomórficos poderão ser especializados.

---

## 18.1609 Uso para Layout

Campos quentes poderão, futuramente, ser reorganizados apenas se a ABI permitir.

---

## 18.1610 Uso para Inlining

Métodos frequentes poderão ser priorizados.

---

## 18.1611 Uso para Tamanho de Bloco

Domains poderão usar heurísticas baseadas no perfil.

---

## 18.1612 Reprodutibilidade

Builds com PGO deverão registrar o perfil usado.

---

## 18.1613 Ausência de Perfil

O comportamento deverá continuar correto sem PGO.

---

## 18.1614 Perfil Obsoleto

A Toolchain deverá detectar incompatibilidade entre perfil e código.

---

# 18.1615 Instrumentação

A implementação deverá fornecer instrumentação suficiente para compreender o comportamento do modelo de objetos.

---

## 18.1616 Princípio da Instrumentação

A instrumentação deverá ser:

* opcional;
* controlável;
* determinística quando possível;
* de baixo custo quando desativada;
* segura;
* independente da semântica.

---

## 18.1617 Níveis de Instrumentação

Poderão existir:

```text
off
errors
basic
detailed
trace
```

---

## 18.1618 Instrumentação de Compilação

O compilador poderá registrar:

* layouts calculados;
* chamadas devirtualizadas;
* checks eliminados;
* alocações removidas;
* objetos stack-allocated;
* drops eliminados;
* metadata emitida.

---

## 18.1619 Instrumentação de Runtime

O Runtime poderá registrar:

* Domains criados;
* objetos reservados;
* objetos publicados;
* resoluções;
* falhas de resolução;
* destruições;
* bytes alocados;
* blocos;
* slots;
* generations.

---

## 18.1620 Instrumentação por Tipo

As métricas poderão ser agregadas por:

* `TypeId`;
* nome de debug;
* módulo;
* Domain;
* origem de alocação.

---

## 18.1621 Instrumentação por Call Site

A criação e chamada poderão receber identificador de origem.

---

## 18.1622 `SourceLocationId`

O compilador poderá emitir tabela compacta ligando IDs a:

* arquivo;
* linha;
* coluna;
* função;
* módulo.

---

## 18.1623 Build de Release

Esses dados poderão ser omitidos ou reduzidos.

---

## 18.1624 Contadores Globais

Deverão ser evitados quando causarem contenção futura.

---

## 18.1625 Contadores por RuntimeContext

A primeira implementação poderá manter métricas locais ao contexto.

---

## 18.1626 Contadores por Domain

Cada Domain poderá manter:

```text
objects_created
objects_destroyed
bytes_reserved
bytes_used
slots_created
slots_reused
failed_constructions
```

---

## 18.1627 Overhead

Os contadores detalhados deverão poder ser desativados em release.

---

## 18.1628 Amostragem

Uma evolução futura poderá coletar apenas amostras.

---

# 18.1629 Tracing

Tracing registra eventos ordenados de execução.

---

## 18.1630 Eventos de Construção

Poderão incluir:

```text
object.reserve
object.allocate
object.construct.begin
object.construct.end
object.publish
object.construct.fail
```

---

## 18.1631 Eventos de Destruição

Poderão incluir:

```text
object.destroy.begin
object.destructor.call
object.field.drop
object.invalidate
object.destroy.end
```

---

## 18.1632 Eventos de Domain

Poderão incluir:

```text
domain.create
domain.allocate_block
domain.finalize.begin
domain.finalize.object
domain.release_block
domain.finalize.end
```

---

## 18.1633 Eventos de Despacho

Poderão incluir:

```text
virtual.call
interface.call
interface.lookup
cast.success
cast.failure
```

---

## 18.1634 Formato do Trace

Poderá ser:

* texto;
* JSON;
* binário;
* formato compatível com ferramenta externa.

---

## 18.1635 Formato Binário

É preferível para alto volume, mas exige ferramenta de leitura.

---

## 18.1636 Timestamp

Poderá utilizar:

* contador monotônico;
* tempo de alta resolução;
* sequência lógica.

---

## 18.1637 Determinismo

Para testes, uma sequência lógica será mais estável que tempo real.

---

## 18.1638 IDs no Trace

Não deverão expor endereços quando não necessário.

---

## 18.1639 Privacidade e Segurança

Dados de objetos não deverão ser registrados automaticamente.

---

## 18.1640 Valores de Campos

Somente poderão ser incluídos sob modo explícito de debug.

---

## 18.1641 Strings Sensíveis

A instrumentação deverá evitar capturar conteúdo sensível por padrão.

---

## 18.1642 Correlação

Eventos poderão carregar:

* RuntimeContextId;
* DomainId;
* ObjectId;
* TypeId;
* ThreadId futuro;
* SourceLocationId.

---

## 18.1643 Integração com OpenTelemetry

Uma evolução futura poderá exportar métricas e traces.

Não será requisito do Runtime mínimo.

---

# 18.1644 Estatísticas

A Toolchain deverá permitir obter estatísticas consolidadas.

---

## 18.1645 Estatísticas de Objetos

Poderão incluir:

* quantidade criada;
* quantidade viva;
* quantidade destruída;
* pico simultâneo;
* tamanho médio;
* falhas de construção.

---

## 18.1646 Estatísticas de Domains

Poderão incluir:

* quantidade;
* profundidade;
* lifetime;
* objetos por Domain;
* bytes desperdiçados;
* tempo de finalização.

---

## 18.1647 Estatísticas de Despacho

Poderão incluir:

* chamadas diretas;
* chamadas virtuais;
* chamadas por interface;
* devirtualizações;
* lookups;
* casts.

---

## 18.1648 Estatísticas de Otimização

O compilador poderá emitir relatório:

```text
virtual calls eliminated: N
object allocations eliminated: N
slot resolutions reused: N
drop calls removed: N
```

---

## 18.1649 Estatísticas por Função

Poderão ajudar a localizar hotspots.

---

## 18.1650 Formato de Saída

Deverá ser estável o suficiente para CI, mas não precisa ser ABI pública.

---

# 18.1651 Benchmarks

Benchmarks deverão medir componentes isolados e fluxos completos.

---

## 18.1652 Princípio de Benchmark

Todo benchmark deverá:

* possuir cenário claramente definido;
* controlar aquecimento quando aplicável;
* repetir execuções;
* informar variabilidade;
* separar debug e release;
* registrar hardware e target.

---

## 18.1653 Microbenchmarks de `ObjectId`

Deverão medir:

* criação;
* cópia;
* igualdade;
* hashing;
* Optional;
* resolução.

---

## 18.1654 Microbenchmarks de Slot

Deverão medir:

* reserva;
* publicação;
* lookup;
* invalidação;
* reutilização;
* geração.

---

## 18.1655 Microbenchmarks de Alocação

Cenários:

* objeto pequeno;
* objeto médio;
* objeto grande;
* alinhamento elevado;
* alocação em loop;
* múltiplos Domains.

---

## 18.1656 Microbenchmarks de Construção

Cenários:

* classe vazia;
* campos triviais;
* campos não triviais;
* herança;
* construtor falível;
* construção aninhada.

---

## 18.1657 Microbenchmarks de Destruição

Cenários:

* drop trivial;
* destrutor explícito;
* hierarquia;
* Domain grande;
* ciclos;
* tombstones.

---

## 18.1658 Microbenchmarks de Despacho

Deverão comparar:

* chamada direta;
* virtual;
* interface;
* devirtualizada;
* cast;
* lookup.

---

## 18.1659 Benchmarks de Coleções

Deverão medir coleções contendo:

* valores;
* `ObjectId`;
* objetos grandes;
* tipos genéricos.

---

## 18.1660 Benchmarks de Compiler Workload

Cenários reais do compilador deverão incluir:

* criação de AST;
* resolução de símbolos;
* construção de HIR;
* destruição de fase;
* tabelas de tipos;
* diagnósticos.

---

## 18.1661 Comparação entre Backends

Cranelift e LLVM deverão ser comparados em:

* tempo de compilação;
* tempo de execução;
* tamanho do binário;
* uso de memória;
* qualidade de devirtualização.

---

## 18.1662 Baseline

A CI deverá manter baseline de performance.

---

## 18.1663 Regressão Aceitável

Limites deverão ser definidos após maturidade inicial.

---

## 18.1664 Ruído

Benchmarks instáveis não deverão bloquear mudanças automaticamente.

---

## 18.1665 Ambiente Dedicado

Benchmarks de CI críticos deverão usar ambiente controlado.

---

## 18.1666 Relatório Histórico

A evolução deverá ser acompanhada ao longo do tempo.

---

## 18.1667 Métricas de Compilação

Além do programa executado, medir:

* tempo de análise;
* tempo de lowering;
* tempo de backend;
* memória do compilador;
* tamanho da IR.

---

## 18.1668 Benchmark do Bootstrap

Medir:

```text
stage0 compila stage1
stage1 compila stage2
stage2 compila projeto real
```

---

# 18.1669 Verificação Estrutural

A implementação deverá possuir validadores internos em múltiplas fases.

---

## 18.1670 Validador de HIR

Deverá verificar:

* tipos resolvidos;
* herança válida;
* métodos;
* construtores;
* campos;
* visibilidade;
* ausência de nós incompletos.

---

## 18.1671 Validador de MIR

Deverá verificar:

* inicialização definida;
* ownership;
* borrowing;
* drop scopes;
* capabilities;
* escapes.

---

## 18.1672 Validador de IR

Deverá verificar:

* tipos físicos;
* blocos;
* dominância;
* chamadas;
* layouts;
* slots;
* publicação;
* destruição;
* cleanup.

---

## 18.1673 Validador de Metadata

Deverá verificar:

* versão;
* tamanho;
* alinhamento;
* referências;
* vtables;
* itables;
* destrutor;
* base;
* hashes.

---

## 18.1674 Validador de Runtime

Em debug, deverá verificar transições de:

```text
Domain
Slot
Object
Reservation
Borrow
Destruction
```

---

## 18.1675 Validação Após Passes

Passes complexos deverão poder executar o validador após transformação.

---

## 18.1676 Modo `verify-each`

A Toolchain poderá oferecer opção para validar após cada passe.

---

## 18.1677 Custo

Esse modo será destinado a testes e desenvolvimento.

---

## 18.1678 Dump em Falha

Quando um validador falhar, deverá emitir:

* fase;
* função;
* nó;
* tipo;
* passe anterior;
* invariante violado.

---

## 18.1679 Minimização de Caso

A infraestrutura de testes deverá facilitar reduzir programas que provocam falha.

---

# 18.1680 Verificação de Layout

O compilador deverá possuir verificador independente do backend.

---

## 18.1681 Regras Verificadas

Deverá confirmar:

1. offsets alinhados;
2. campos dentro do tamanho;
3. ausência de sobreposição;
4. base como prefixo;
5. header válido;
6. padding consistente;
7. tamanho final alinhado.

---

## 18.1682 Layout de Generics

Cada especialização deverá ser verificada separadamente.

---

## 18.1683 Layout Recursivo

Tipos de valor recursivos infinitos deverão ser rejeitados.

---

## 18.1684 Ciclo por Identidade

Será permitido porque `ObjectId` possui tamanho fixo.

---

## 18.1685 Comparação com Backend

O backend deverá confirmar que aceita o layout.

---

## 18.1686 Assert Estático

Estruturas compartilhadas com Rust ou C deverão possuir asserts de tamanho e offset.

---

# 18.1687 Sanitizers

Sanitizers deverão complementar as garantias estáticas e detectar erros internos de implementação.

---

## 18.1688 Address Sanitizer

Deverá ser usado para detectar:

* out-of-bounds;
* use-after-free físico;
* stack overflow;
* corrupção de heap externo.

---

## 18.1689 Undefined Behavior Sanitizer

Deverá detectar:

* overflow inválido;
* desalinhamento;
* shifts;
* casts incorretos;
* chamadas ABI incompatíveis.

---

## 18.1690 Memory Sanitizer

Quando disponível, deverá detectar leituras de memória não inicializada.

---

## 18.1691 Leak Sanitizer

Deverá ser usado para alocações externas aos Domains e recursos auxiliares.

---

## 18.1692 Thread Sanitizer

Será relevante quando concorrência for implementada.

---

## 18.1693 Sanitizer de Runtime Capi

A Toolchain poderá implementar verificações próprias:

* stale `ObjectId`;
* double destroy;
* invalid Domain;
* borrow violation;
* publication failure;
* slot generation mismatch.

---

## 18.1694 Perfil Sanitized

Artefatos instrumentados deverão ser identificados como perfil separado.

---

## 18.1695 Compatibilidade

Módulos sanitized e não sanitized não deverão ser misturados quando a ABI divergir.

---

# 18.1696 Miri e Verificação do Runtime Rust

O Runtime inicial em Rust deverá ser executado sob Miri quando possível.

---

## 18.1697 Escopo do Miri

Deverão ser testados:

* ponteiros;
* alinhamento;
* `repr(C)`;
* lifetimes internos;
* aliasing;
* `UnsafeCell`;
* transições de slot;
* guards.

---

## 18.1698 Código `unsafe`

Todo bloco `unsafe` deverá possuir justificativa local.

---

## 18.1699 Invariantes do `unsafe`

A documentação deverá explicar:

* pré-condições;
* pós-condições;
* ownership;
* aliasing;
* lifetime;
* alinhamento.

---

## 18.1700 Encapsulamento

Código Capi gerado não deverá acessar diretamente internals inseguros do Runtime.

---

## 18.1701 API Rust Segura

Sempre que possível, os módulos superiores do Runtime deverão usar abstrações seguras.

---

## 18.1702 Auditoria de `unsafe`

A CI poderá limitar ou contar blocos `unsafe`.

---

# 18.1703 Fuzzing

Fuzzing deverá ser aplicado ao compilador, Runtime e formatos.

---

## 18.1704 Fuzzing de Parser

Embora não específico do modelo de objetos, deverá gerar construções sintáticas de classes e métodos.

---

## 18.1705 Fuzzing de Hierarquias

Gerar:

* classes;
* bases;
* interfaces;
* overrides;
* métodos abstratos;
* generics.

---

## 18.1706 Fuzzing de Construção

Gerar sequências de:

* reserva;
* campos;
* falhas;
* publicação;
* cancelamento.

---

## 18.1707 Fuzzing de Destruição

Gerar:

* objetos vivos;
* destruição antecipada;
* Domains;
* ciclos;
* aliases;
* tombstones.

---

## 18.1708 Fuzzing de Casts

Gerar combinações de:

* upcast;
* downcast;
* interface cast;
* falha;
* tipos genéricos.

---

## 18.1709 Fuzzing de Metadata

Entradas corrompidas deverão ser rejeitadas sem UB.

---

## 18.1710 Fuzzing de IR

A IR gerada deverá ser validada antes de backend.

---

## 18.1711 Fuzzing de Runtime API

Sequências aleatórias deverão testar estados inválidos.

---

## 18.1712 Oráculos

Os oráculos poderão incluir:

* ausência de crash;
* ausência de UB;
* validação de invariantes;
* comparação entre backends;
* modelo de referência.

---

## 18.1713 Corpus Inicial

O corpus deverá incluir casos manuais representativos.

---

## 18.1714 Regressões

Todo crash encontrado deverá gerar teste permanente.

---

# 18.1715 Property-Based Testing

Property-based testing deverá gerar estruturas válidas e verificar propriedades gerais.

---

## 18.1716 Propriedades de Identidade

1. identidades publicadas são únicas;
2. cópias permanecem iguais;
3. destruição invalida resolução;
4. reutilização altera geração;
5. movimento físico futuro não altera identidade.

---

## 18.1717 Propriedades de Layout

1. offsets são válidos;
2. base é prefixo;
3. alinhamento é respeitado;
4. tamanho é suficiente;
5. backends concordam.

---

## 18.1718 Propriedades de Despacho

1. chamada virtual seleciona override correto;
2. chamada de interface seleciona implementação correta;
3. método final não é sobrescrito;
4. devirtualização preserva resultado.

---

## 18.1719 Propriedades de Construção

1. objeto publicado está completo;
2. falha não publica;
3. cleanup é inverso;
4. base precede derivada;
5. `self` não escapa.

---

## 18.1720 Propriedades de Destruição

1. drop ocorre no máximo uma vez;
2. derivada precede base;
3. campos seguem ordem inversa;
4. ciclos não impedem Domain;
5. stale IDs não resolvem.

---

## 18.1721 Propriedades de Otimização

1. programa otimizado produz o mesmo resultado;
2. efeitos mantêm ordem;
3. erros observáveis permanecem;
4. identidades observáveis permanecem;
5. destrutores preservam ordem.

---

# 18.1722 Testes Diferenciais

A Toolchain deverá comparar implementações ou níveis de otimização.

---

## 18.1723 Cranelift versus LLVM

O mesmo programa deverá produzir:

* mesma saída;
* mesmo código de retorno;
* mesmos efeitos;
* mesma ordem de trace semântico;
* mesmos erros.

---

## 18.1724 `-O0` versus `-O2`

O comportamento deverá permanecer equivalente.

---

## 18.1725 Runtime Checked versus Release

Diferenças permitidas limitam-se a diagnósticos e performance.

---

## 18.1726 Interpreter Futuro

Uma implementação de referência poderá servir como oráculo.

---

## 18.1727 Modelo Executável

Algumas operações de Domain e slots poderão possuir modelo simplificado em Rust puro.

---

## 18.1728 Comparação de Traces

Eventos semânticos poderão ser normalizados e comparados.

---

## 18.1729 Normalização

Deverá remover:

* endereços;
* timestamps;
* IDs não determinísticos;
* ordem irrelevante.

---

## 18.1730 Divergência

Qualquer divergência deverá gerar caso reproduzível.

---

# 18.1731 Testes Metamórficos

Transformações que deveriam preservar comportamento poderão ser aplicadas automaticamente.

---

## 18.1732 Exemplos de Transformação

* renomear variáveis;
* reordenar declarações independentes;
* inserir upcast seguido de downcast válido;
* substituir chamada direta equivalente;
* alterar backend;
* alterar nível de otimização.

---

## 18.1733 Propriedade

O resultado semântico deverá permanecer igual.

---

# 18.1734 Verificação de Otimizações

Cada passe deverá declarar:

* pré-condições;
* transformação;
* invariantes preservados;
* análises invalidadas;
* testes associados.

---

## 18.1735 Pass Manager

A Toolchain deverá controlar a ordem dos passes.

---

## 18.1736 Dependências de Análise

Passes poderão depender de:

* dominância;
* alias;
* tipos concretos;
* escape;
* efeitos;
* liveness.

---

## 18.1737 Invalidação de Análises

Após transformação, análises obsoletas deverão ser recalculadas.

---

## 18.1738 Modo de Debug do Pass

Poderá emitir IR antes e depois.

---

## 18.1739 Bisect de Otimizações

Uma ferramenta deverá permitir desabilitar passes para localizar regressão.

---

## 18.1740 Identificador de Passe

Cada passe deverá possuir nome estável para diagnóstico.

---

## 18.1741 Estatística por Passe

Poderá informar quantas transformações foram aplicadas.

---

## 18.1742 Passes Determinísticos

Com a mesma entrada e configuração, deverão produzir a mesma saída lógica.

---

## 18.1743 Ordem Não Determinística

Estruturas como hash maps deverão ser ordenadas antes de emissão relevante.

---

# 18.1744 Verificação Formal Futura

Partes críticas poderão ser modeladas formalmente.

---

## 18.1745 Candidatos

* estados de slot;
* geração;
* publicação;
* destruição;
* borrowing de resolução;
* ordem de Domain;
* layout prefixado.

---

## 18.1746 Model Checking

Uma ferramenta poderá explorar transições de estados.

---

## 18.1747 Propriedades de Segurança

Exemplos:

```text
Published implies Initialized
Destroyed implies NotResolvable
Finalizing never returns to Alive
GenerationMismatch implies ResolutionFailure
```

---

## 18.1748 Escopo Inicial

Verificação formal não será requisito do bootstrap, mas o design deverá facilitar modelagem.

---

## 18.1749 Máquinas de Estado Explícitas

Estados deverão ser representados de forma clara, evitando combinações implícitas de flags.

---

# 18.1750 Ferramentas de Inspeção

A Toolchain deverá possuir ferramentas para visualizar estruturas internas.

---

## 18.1751 `capi layout`

Poderá imprimir:

* tamanho;
* alinhamento;
* campos;
* offsets;
* base;
* header;
* niches.

---

## 18.1752 `capi vtable`

Poderá imprimir:

* slots;
* métodos;
* thunks;
* overrides;
* hashes.

---

## 18.1753 `capi interfaces`

Poderá imprimir:

* interfaces implementadas;
* itables;
* precedência;
* métodos padrão.

---

## 18.1754 `capi metadata`

Poderá inspecionar metadata de módulo ou binário.

---

## 18.1755 `capi object-trace`

Poderá interpretar trace de Runtime.

---

## 18.1756 `capi domain-report`

Poderá mostrar:

* árvore de Domains;
* objetos;
* bytes;
* slots;
* fragmentação;
* tempo de finalização.

---

## 18.1757 `capi optimization-report`

Poderá mostrar:

* devirtualizações;
* allocations eliminadas;
* stack allocations;
* checks removidos;
* inlining.

---

## 18.1758 Formato para Ferramentas

Os dumps deverão possuir opção:

```text
human-readable
JSON
```

---

## 18.1759 Estabilidade do JSON

Poderá possuir versão própria.

---

## 18.1760 Integração com IDE

Ferramentas futuras poderão exibir:

* layout;
* hierarquia;
* dispatch target;
* custo estimado;
* escape.

---

# 18.1761 Observabilidade em Produção

A instrumentação de produção deverá evitar overhead elevado.

---

## 18.1762 Métricas Agregadas

Preferíveis a traces completos.

---

## 18.1763 Ativação Dinâmica

Uma evolução futura poderá permitir ativar tracing em execução.

---

## 18.1764 Segurança da Ativação

A mudança não poderá alterar layout nem ABI.

---

## 18.1765 Exportação

Métricas poderão ser exportadas por API opcional da aplicação.

---

## 18.1766 Runtime Mínimo

O núcleo do Runtime não deverá depender de sistema de observabilidade externo.

---

# 18.1767 Diagnósticos de Performance

O compilador poderá emitir avisos opcionais.

---

## 18.1768 Chamada Virtual em Loop Quente

Poderá sugerir classe final, generics ou reorganização, sem exigir alteração.

---

## 18.1769 Alocação em Loop

Poderá indicar alta quantidade de objetos no mesmo Domain.

---

## 18.1770 Domain de Longa Duração

Poderá alertar sobre fragmentação por destruição antecipada.

---

## 18.1771 Objeto Grande na Pilha

Poderá alertar quando stack allocation for arriscada.

---

## 18.1772 Metadata Excessiva

Poderá indicar emissão de reflection ou debug desnecessário.

---

## 18.1773 Avisos Não Normativos

Esses diagnósticos não deverão impedir compilação por padrão.

---

# 18.1774 Segurança das Otimizações

O sistema de tipos não deverá ser contornado por otimizações.

---

## 18.1775 Alias Incompatível

O backend não poderá introduzir aliases proibidos ao reutilizar armazenamento.

---

## 18.1776 Lifetime Encurtado

O compilador poderá antecipar drop somente se nenhum efeito observável mudar.

---

## 18.1777 Lifetime Estendido

Poderá manter memória por mais tempo, mas não atrasar destrutor observável além do ponto definido.

---

## 18.1778 Eliminação de Destrutor

Somente será permitida se o destrutor e os campos forem comprovadamente sem efeitos.

---

## 18.1779 Reordenação de Destrutores

Será proibida quando a ordem for definida.

---

## 18.1780 Eliminação de Check

Somente poderá ocorrer com prova estática.

---

## 18.1781 Check Especulativo

Se houver fallback seguro, poderá ser especializado.

---

## 18.1782 Falhas como Comportamento Observável

Uma otimização não poderá transformar programa válido em falha ou vice-versa quando a falha faz parte da semântica.

---

## 18.1783 UB Interno

Erros do compilador ou Runtime não deverão ser tratados como comportamento permitido do programa.

---

# 18.1784 Segurança de Chamadas Indiretas

Chamadas por vtable e itable são pontos críticos.

---

## 18.1785 Validação de Assinatura

O compilador deverá garantir que o slot aponta para função ABI-compatível.

---

## 18.1786 CFI

Control-Flow Integrity poderá ser adicionada futuramente.

---

## 18.1787 Type Hash no Call Site

Uma implementação checked poderá validar metadata antes da chamada.

---

## 18.1788 Pointer Authentication

Targets compatíveis poderão usar mecanismos de autenticação de ponteiros.

---

## 18.1789 Metadata Read-Only

Vtables e itables deverão permanecer em memória não gravável quando possível.

---

## 18.1790 W^X

A Toolchain deverá respeitar políticas que impedem memória simultaneamente gravável e executável.

---

## 18.1791 Inline Caches e W^X

Caches futuros deverão ficar em dados, não exigir patch inseguro de código.

---

# 18.1792 Instrumentação de Segurança

Builds checked poderão manter:

* canários;
* magic values;
* generation checks;
* type checks;
* bounds;
* borrow counters.

---

## 18.1793 Canário de Cabeçalho

Poderá detectar corrupção.

---

## 18.1794 Canário de Bloco

Poderá detectar overflow no alocador.

---

## 18.1795 Custo

Essas validações deverão ser removíveis.

---

## 18.1796 Diagnóstico de Corrupção

Deverá indicar o objeto e o Domain quando possível.

---

# 18.1797 Verificação de FFI

A FFI deverá possuir testes específicos de layout e lifetime.

---

## 18.1798 Ponteiro Temporário

Deverá ser invalidado após o escopo de borrow.

---

## 18.1799 Handle Opaque

Deverá ser validado antes de cada operação.

---

## 18.1800 Ownership Transfer

Testes deverão confirmar destruição por exatamente um lado.

---

## 18.1801 Callback

Deverá validar que objetos não escapam indevidamente.

---

## 18.1802 ABI C

Tamanhos e offsets deverão ser comparados com `static_assert`.

---

## 18.1803 Fuzzing da Fronteira

Valores inválidos vindos de C deverão ser rejeitados.

---

# 18.1804 Testes de Stress

A implementação deverá ser submetida a cargas prolongadas.

---

## 18.1805 Stress de Alocação

Criar e destruir milhões de objetos.

---

## 18.1806 Stress de Slots

Forçar reutilização e incremento de geração.

---

## 18.1807 Stress de Domains

Criar árvores profundas e largas.

---

## 18.1808 Stress de Ciclos

Criar grandes grafos cíclicos.

---

## 18.1809 Stress de Herança

Usar hierarquias profundas dentro dos limites permitidos.

---

## 18.1810 Stress de Interfaces

Tipos implementando muitas interfaces.

---

## 18.1811 Stress de Generics

Muitas especializações monomorfizadas.

---

## 18.1812 Stress de Falhas

Injetar erro em cada ponto de construção.

---

## 18.1813 Fault Injection

O Runtime deverá permitir falhar artificialmente:

* alocação;
* crescimento de vetor;
* registro;
* I/O;
* helpers.

---

## 18.1814 Propriedade

Nenhuma falha injetada deverá deixar:

* objeto vivo parcial;
* slot inconsistente;
* recurso duplicado;
* Domain impossível de destruir.

---

# 18.1815 Testes de Auto-Hospedagem

O compilador Capi será o principal teste integrado do modelo.

---

## 18.1816 Estruturas Reais

Deverão exercitar:

* AST;
* HIR;
* IR;
* símbolos;
* generics;
* strings;
* diagnósticos;
* backends.

---

## 18.1817 Domains por Compilação

Cada unidade poderá criar e destruir Domains temporários.

---

## 18.1818 Medição

Deverão ser coletados:

* objetos criados;
* bytes;
* tempo;
* pico;
* finalização;
* dispatch.

---

## 18.1819 Comparação com Implementação Rust

O compilador auto-hospedado poderá ser comparado com o bootstrap em Rust.

---

## 18.1820 Equivalência

Deverão produzir:

* mesmos diagnósticos relevantes;
* mesma IR normalizada;
* mesmos artefatos semanticamente;
* mesmos resultados de testes.

---

# 18.1821 Registros de Regressão

Cada bug no modelo de objetos deverá produzir:

* teste mínimo;
* categoria;
* fase;
* causa;
* correção;
* invariante relacionado.

---

## 18.1822 Categorias

Exemplos:

```text
layout
dispatch
construction
destruction
domain
object_id
metadata
backend
optimization
ffi
```

---

## 18.1823 Base de Casos

A Toolchain deverá manter suíte permanente.

---

# 18.1824 Política de Performance

A implementação deverá priorizar na seguinte ordem:

1. correção;
2. segurança;
3. diagnósticos;
4. previsibilidade;
5. tempo de compilação;
6. performance de execução;
7. tamanho de código.

A ordem poderá variar por perfil, mas segurança e correção permanecem obrigatórias.

---

## 18.1825 Cranelift

Deverá priorizar:

* geração rápida;
* arquitetura simples;
* validação;
* suporte ao bootstrap.

---

## 18.1826 LLVM

Deverá priorizar:

* otimização avançada;
* performance final;
* LTO;
* múltiplos targets.

---

## 18.1827 Runtime

Deverá permanecer pequeno e previsível.

---

## 18.1828 Evitar Otimização Prematura

Estruturas complexas não deverão ser adotadas sem benchmark que demonstre necessidade.

---

## 18.1829 Evidência

Toda otimização relevante deverá ser justificada por:

* perfil;
* benchmark;
* tamanho;
* caso real;
* redução mensurável.

---

## 18.1830 Custo de Manutenção

Deverá ser considerado junto ao ganho de performance.

---

# 18.1831 Matriz de Otimizações por Fase

A implementação poderá seguir:

```text
Bootstrap inicial
    devirtualização trivial
    drop trivial
    inlining básico
    resolução reutilizada

Auto-hospedagem inicial
    escape analysis local
    construção in-place
    relatórios de otimização
    benchmarks estáveis

Backend LLVM
    LTO
    devirtualização interprocedural
    scalar replacement
    eliminação avançada de alocação

Pós-estabilização
    stack allocation
    compactação
    inline caches
    PGO
    metadata compacta
```

---

## 18.1832 Critérios para Introduzir Otimização

Uma otimização somente deverá ser incorporada quando:

* a semântica estiver definida;
* houver testes;
* houver verificador;
* houver benchmark;
* o ganho justificar a complexidade;
* ambos os backends permanecerem coerentes ou possuírem fallback.

---

## 18.1833 Fallback

Toda otimização deverá possuir caminho não otimizado correto.

---

## 18.1834 Desativação

Deverá ser possível desativar otimizações para diagnóstico.

---

## 18.1835 Reprodutor

Bugs de otimização deverão poder ser reproduzidos com flags registradas.

---

# 18.1836 Invariantes de Otimização

A implementação deverá preservar:

1. identidade observável não muda;
2. tipo concreto não muda;
3. construção mantém ordem e efeitos;
4. destruição mantém ordem e efeitos;
5. `ObjectId` obsoleto não volta a resolver;
6. devirtualização seleciona o mesmo método;
7. eliminação de resolução respeita lifetime;
8. stack allocation não permite escape;
9. scalar replacement preserva alias lógico;
10. drop de recurso não é eliminado;
11. layout público não é alterado localmente;
12. metadata exigida não é removida;
13. checks só são eliminados com prova;
14. backends preservam equivalência;
15. instrumentação não altera semântica.

---

## 18.1837 Invariantes de Instrumentação

1. instrumentação desativada possui custo mínimo;
2. eventos não expõem dados sensíveis por padrão;
3. IDs permanecem correlacionáveis;
4. tracing não altera ordem semântica;
5. falha na instrumentação não corrompe Runtime;
6. métricas podem ser desativadas;
7. formatos possuem versão;
8. timestamps não definem semântica;
9. contadores não controlam lifetime;
10. diagnósticos não dependem de endereços permanentes.

---

## 18.1838 Invariantes de Verificação

1. toda IR enviada ao backend é válida;
2. todo layout é verificado;
3. metadata desconhecida é rejeitada;
4. transições inválidas de estado são detectadas;
5. otimizações podem ser validadas por passe;
6. regressões geram testes permanentes;
7. backends são comparados;
8. builds checked detectam stale IDs;
9. sanitizers complementam análise estática;
10. fuzzing não depende de comportamento indefinido como oráculo.

---

## 18.1839 ADRs Recomendados

Deverão ser considerados:

```text
ADR — Devirtualização baseada em prova estática
ADR — Fallback obrigatório para especializações especulativas
ADR — Escape analysis conservadora
ADR — Stack allocation somente sem identidade observável
ADR — Scalar replacement preservando identidade local
ADR — Compactação fora do bootstrap
ADR — Metadata quente e fria separadas
ADR — Instrumentação opcional por RuntimeContext
ADR — Trace sem conteúdo de campos por padrão
ADR — Validação após passes complexos
ADR — Comparação diferencial Cranelift versus LLVM
ADR — Fault injection no Runtime
ADR — Benchmarks antes de otimizações estruturais
ADR — Caminho não otimizado sempre disponível
```

---

## 18.1840 Critérios de Conclusão das Otimizações Básicas

As otimizações básicas estarão concluídas quando:

* chamadas de classe final forem diretas;
* métodos finais forem devirtualizados;
* drops triviais forem eliminados;
* resoluções redundantes locais forem combinadas;
* construtores simples puderem ser inlineados;
* checks comprovadamente redundantes puderem ser removidos;
* o comportamento permanecer idêntico em `-O0` e release.

---

## 18.1841 Critérios de Conclusão da Instrumentação

Estará concluída quando:

* Domains puderem emitir métricas;
* construções e destruições puderem ser rastreadas;
* eventos possuírem IDs correlacionáveis;
* o tracing puder ser desativado;
* relatórios não dependerem de endereços;
* dados sensíveis não forem registrados por padrão;
* ferramentas puderem interpretar a saída.

---

## 18.1842 Critérios de Conclusão da Verificação Estrutural

Estará concluída quando:

* HIR, MIR e IR possuírem validadores;
* layouts forem verificados;
* metadata for validada;
* estados do Runtime forem checados em debug;
* `verify-each` estiver disponível;
* dumps em falha forem úteis;
* testes negativos cobrirem invariantes.

---

## 18.1843 Critérios de Conclusão dos Testes Dinâmicos

Estará concluída quando:

* sanitizers forem executados em CI;
* Runtime Rust passar por Miri nos módulos aplicáveis;
* fuzzing cobrir metadata e estados;
* property-based tests cobrirem identidade, construção e destruição;
* fault injection cobrir falhas de alocação;
* regressões possuírem testes permanentes.

---

## 18.1844 Critérios de Conclusão dos Benchmarks

Estará concluída quando existirem baselines para:

* `ObjectId`;
* resolução;
* alocação;
* construção;
* destruição;
* despacho;
* Domains;
* compilação do próprio compilador;
* Cranelift;
* LLVM.

---

## 18.1845 Critérios para Escape Analysis e Stack Allocation

Essas otimizações estarão maduras quando:

* escapes forem classificados;
* chamadas desconhecidas forem conservadoras;
* objetos sem escape forem identificados;
* materialização tardia funcionar;
* FFI bloquear casos inseguros;
* drops forem preservados;
* testes diferenciais passarem;
* benchmarks demonstrarem ganho.

---

## 18.1846 Critérios para Otimizações Avançadas

Inline caches, PGO, compactação e pools somente deverão ser considerados quando:

* o bootstrap estiver estável;
* a auto-hospedagem estiver funcional;
* perfis reais demonstrarem gargalo;
* o modelo básico possuir cobertura ampla;
* a complexidade operacional for justificável.

---

## 18.1847 Critérios de Conclusão para o Bootstrap

Para o bootstrap, será suficiente que:

* o modelo funcione corretamente sem otimizações avançadas;
* devirtualizações triviais existam;
* drops triviais sejam removidos;
* a construção in-place básica funcione;
* o Runtime possua métricas de debug;
* validadores detectem estados inválidos;
* sanitizers e fuzzing básicos estejam ativos;
* benchmarks identifiquem regressões graves;
* Cranelift produza execução correta e previsível.

---

## 18.1848 Resultado Esperado da Terceira Parte — Bloco 3.2

Ao final desta etapa, a implementação oficial deverá possuir uma arquitetura em que:

```text
Middle-end
    aplica otimizações semânticas baseadas em prova

Escape Analysis
    determina se objetos podem ser materializados localmente

Devirtualização
    remove despacho dinâmico quando o alvo é conhecido

Scalar Replacement
    elimina objetos físicos não observáveis

Runtime
    otimiza slots, alocação e Domains

Instrumentação
    mede construção, despacho, memória e destruição

Validadores
    verificam HIR, MIR, IR, layouts e metadata

Sanitizers
    detectam erros internos de memória

Fuzzing
    explora estados e combinações inesperadas

Benchmarks
    fundamentam decisões de performance

Cranelift e LLVM
    são comparados por testes diferenciais
```

As otimizações deverão ser tratadas como refinamentos da implementação, jamais como substitutas das garantias formais do modelo.

A próxima e última parte deverá consolidar:

* invariantes finais;
* requisitos de conformidade;
* critérios de implementação;
* critérios de bootstrap;
* critérios de auto-hospedagem;
* relação com os demais documentos;
* ADRs finais;
* encerramento normativo do Capítulo 18.

---

## Terceira Parte — Bloco 3.3 — Invariantes Finais e Encerramento do Capítulo

## 18.1849 Objetivo

Esta parte consolida os princípios, invariantes e critérios normativos que encerram o Capítulo 18.

Seu propósito não é introduzir novos mecanismos, mas reunir todas as garantias apresentadas ao longo do capítulo em um conjunto consistente de contratos que deverão orientar toda a implementação oficial da linguagem Capi.

---

# 18.1850 Papel Normativo deste Capítulo

O Capítulo 18 estabelece:

* o modelo conceitual dos objetos;
* a representação lógica da identidade;
* o layout dos objetos;
* o despacho dinâmico;
* a construção;
* a destruição;
* a evolução da ABI;
* as regras de otimização;
* os mecanismos de validação.

Ele não define a sintaxe da linguagem nem substitui a especificação semântica.

---

# 18.1851 Fonte das Garantias

As garantias deste capítulo derivam da combinação dos documentos:

* Documento 02 — Sistema de Tipos
* Documento 03 — Modelo de Memória
* Documento 05 — Semântica Operacional
* Documento 07 — Intermediate Representation
* Documento 09 — Runtime Mínimo
* Documento 10 — ABI e FFI

Este documento descreve como essas garantias deverão ser implementadas.

---

# 18.1852 Separação Permanente entre Especificação e Implementação

Toda implementação deverá preservar o princípio fundamental do Projeto Capi:

> A especificação define o comportamento.

> A implementação define apenas a estratégia utilizada para concretizar esse comportamento.

Nenhum detalhe interno do compilador ou do Runtime deverá alterar a semântica da linguagem.

---

# 18.1853 Invariantes Fundamentais

Independentemente da arquitetura utilizada, deverão permanecer verdadeiros os seguintes princípios.

---

## 18.1854 Identidade

Todo objeto possui identidade lógica.

---

## 18.1855 Persistência da Identidade

A identidade permanece constante durante toda a vida do objeto.

---

## 18.1856 Independência do Endereço

Identidade nunca deverá depender do endereço físico.

---

## 18.1857 Endereço Transitório

Endereços físicos poderão mudar futuramente sem alterar a identidade.

---

## 18.1858 Igualdade

A igualdade por identidade deverá permanecer válida durante toda a existência do objeto.

---

## 18.1859 Lifetime

Todo objeto pertence exatamente a um Domain durante sua existência.

---

## 18.1860 Criação

Nenhum objeto poderá ser observado antes de estar completamente construído.

---

## 18.1861 Publicação

Somente objetos completamente válidos poderão ser publicados.

---

## 18.1862 Falha de Construção

Objetos parcialmente construídos jamais poderão escapar.

---

## 18.1863 Destruição

Todo objeto deverá ser destruído exatamente uma vez.

---

## 18.1864 Ordem de Destruição

A ordem normativa definida anteriormente deverá sempre ser preservada.

---

## 18.1865 Finalização

Todo recurso deverá ser liberado antes da remoção definitiva do objeto.

---

## 18.1866 ObjectId

`ObjectId` representa identidade.

Jamais endereço.

---

## 18.1867 Slots

Slots representam localização lógica.

---

## 18.1868 Generation

Generation impede reutilização insegura.

---

## 18.1869 Domains

Domains controlam lifetime coletivo.

---

## 18.1870 Borrow

Borrow controla acesso temporário.

---

## 18.1871 Metadata

Metadata descreve comportamento.

Nunca deverá representar estado mutável.

---

## 18.1872 Header

O cabeçalho identifica o tipo concreto do objeto.

---

## 18.1873 Payload

O payload contém apenas estado do objeto.

---

## 18.1874 ABI

A ABI representa contrato de interoperabilidade.

Não define semântica.

---

# 18.1875 Invariantes do Modelo de Objetos

Toda implementação deverá preservar:

1. identidade única;
2. identidade imutável;
3. tipo concreto consistente;
4. layout válido;
5. construção completa antes da publicação;
6. destruição completa antes da liberação;
7. metadata imutável;
8. despacho correto;
9. subtipagem válida;
10. casts seguros.

---

# 18.1876 Invariantes do Layout

Todo layout deverá obedecer:

* alinhamento correto;
* offsets válidos;
* ausência de sobreposição;
* prefixo da base preservado;
* tamanho consistente;
* representação determinística.

---

# 18.1877 Invariantes do Despacho

Toda chamada dinâmica deverá:

* selecionar exatamente a implementação correta;
* respeitar override;
* respeitar interfaces;
* preservar assinatura;
* preservar ABI.

---

# 18.1878 Invariantes da Construção

A construção deverá obedecer:

```text
reserva

↓

alocação

↓

base

↓

campos

↓

construtor

↓

publicação
```

Nenhuma etapa poderá ser invertida.

---

# 18.1879 Invariantes da Destruição

A destruição deverá obedecer:

```text
início

↓

destrutor

↓

campos

↓

base

↓

invalidação

↓

liberação
```

---

# 18.1880 Invariantes de Domains

Domains deverão garantir:

* destruição coletiva;
* ausência de vazamentos estruturais;
* invalidação de ObjectIds;
* consistência de slots;
* preservação das gerações.

---

# 18.1881 Invariantes do Runtime

O Runtime deverá permanecer:

* pequeno;
* determinístico;
* previsível;
* desacoplado do compilador;
* sem GC;
* sem ARC.

---

# 18.1882 Invariantes do Compilador

O compilador deverá produzir exatamente as garantias descritas pela especificação.

Jamais menos.

Jamais mais.

---

# 18.1883 Invariantes da IR

A IR deverá continuar representando apenas conceitos da linguagem.

Jamais detalhes acidentais do backend.

---

# 18.1884 Invariantes dos Backends

Cranelift e LLVM deverão produzir exatamente o mesmo comportamento observável.

---

# 18.1885 Invariantes das Otimizações

Nenhuma otimização poderá alterar:

* identidade;
* lifetime;
* ordem de construção;
* ordem de destruição;
* efeitos observáveis;
* despacho;
* tipos;
* segurança.

---

# 18.1886 Invariantes da Instrumentação

Instrumentação nunca poderá alterar:

* semântica;
* layout;
* ABI;
* lifetime;
* identidade.

---

# 18.1887 Invariantes de Testes

Todo mecanismo de testes deverá validar contratos da linguagem, não detalhes internos específicos do Runtime atual.

---

# 18.1888 Invariantes da Evolução

A implementação deverá permanecer evolutiva.

Nenhum detalhe acidental deverá tornar-se contrato permanente.

---

# 18.1889 Critérios de Conformidade

Uma implementação será considerada conforme quando:

* respeitar a especificação;
* preservar todos os invariantes;
* produzir diagnósticos corretos;
* executar os testes normativos;
* respeitar a ABI correspondente.

---

# 18.1890 Critérios para o Bootstrap

A implementação inicial não precisará possuir:

* compactação;
* PGO;
* inline caches;
* reflection;
* otimizações avançadas.

Entretanto deverá implementar corretamente todas as garantias fundamentais.

---

# 18.1891 Critérios para Auto-Hospedagem

A implementação poderá ser considerada apta à auto-hospedagem quando:

* compilar a si própria;
* produzir binários corretos;
* preservar comportamento entre estágios;
* executar toda a suíte de testes.

---

# 18.1892 Critérios para Estabilização da ABI

A ABI somente poderá ser considerada estável quando:

* o bootstrap estiver concluído;
* a auto-hospedagem estiver consolidada;
* múltiplos targets forem suportados;
* o Runtime estiver estabilizado;
* os dois backends forem equivalentes.

---

# 18.1893 Critérios para Evoluções Futuras

Novas funcionalidades deverão:

* preservar contratos existentes;
* possuir documentação;
* possuir ADR correspondente;
* possuir testes;
* possuir benchmark quando relevante.

---

# 18.1894 Compatibilidade com Documentos Anteriores

Este capítulo complementa diretamente:

* Documento 02 — Sistema de Tipos
* Documento 03 — Modelo de Memória
* Documento 05 — Semântica Operacional
* Documento 07 — IR
* Documento 09 — Runtime
* Documento 10 — ABI

Sem substituir nenhum deles.

---

# 18.1895 Compatibilidade com Documentos Posteriores

Os capítulos seguintes da implementação deverão assumir todos os contratos estabelecidos neste capítulo.

Nenhum deles poderá redefinir o modelo de objetos.

---

# 18.1896 Relação com o Bootstrap Oficial

O bootstrap em Rust deverá implementar exatamente este modelo.

---

# 18.1897 Relação com a Implementação Auto-Hospedada

A futura implementação escrita em Capi deverá preservar exatamente os mesmos contratos.

---

# 18.1898 Relação entre Backends

Cranelift e LLVM deverão ser considerados apenas mecanismos alternativos de geração de código.

Jamais implementações distintas da linguagem.

---

# 18.1899 ADRs Obrigatórios

Alterações futuras deverão possuir ADR específico quando modificarem:

* ObjectId;
* Domain;
* Layout;
* Header;
* Metadata;
* ABI;
* Construção;
* Destruição;
* Dispatch;
* Runtime.

---

# 18.1900 Documentação Obrigatória

Toda alteração estrutural deverá atualizar:

* documentação;
* testes;
* benchmarks;
* snapshots;
* exemplos.

---

# 18.1901 Testes Obrigatórios

Nenhuma alteração estrutural poderá ser aceita sem:

* testes unitários;
* integração;
* regressão;
* property-based tests;
* fuzzing quando aplicável.

---

# 18.1902 Benchmarks Obrigatórios

Alterações de performance deverão apresentar benchmarks reproduzíveis.

---

# 18.1903 Auditoria

Toda alteração relevante deverá ser revisada quanto a:

* segurança;
* corretude;
* compatibilidade;
* manutenção.

---

# 18.1904 Filosofia do Modelo

O modelo de objetos da Capi foi concebido para unir:

* orientação a objetos tradicional;
* segurança moderna;
* previsibilidade;
* compilação AOT;
* ausência de coleta de lixo.

---

# 18.1905 Filosofia da Implementação

A implementação deverá buscar:

* simplicidade;
* clareza;
* determinismo;
* facilidade de auditoria;
* evolução incremental.

---

# 18.1906 Filosofia do Runtime

O Runtime deverá existir apenas para suportar a linguagem.

Jamais para defini-la.

---

# 18.1907 Filosofia do Compilador

O compilador deverá transformar garantias semânticas em código executável.

Não deverá introduzir novas regras da linguagem.

---

# 18.1908 Filosofia da Evolução

A evolução deverá ocorrer por:

* documentação;
* consenso técnico;
* ADRs;
* testes;
* implementação.

Jamais por comportamento implícito.

---

# 18.1909 Resultado Esperado do Capítulo

Ao término deste capítulo, a implementação oficial deverá possuir um modelo de objetos em que:

```text
Objetos possuem identidade permanente.

Domains controlam o lifetime coletivo.

ObjectId representa identidade.

Slots representam localização lógica.

Generation impede reutilização incorreta.

Metadata descreve comportamento.

Layouts permanecem determinísticos.

Despacho dinâmico preserva subtipagem.

Construção publica apenas objetos completos.

Destruição libera recursos exatamente uma vez.

Runtime permanece mínimo.

Compilador permanece fiel à especificação.

Cranelift e LLVM implementam exatamente o mesmo contrato.

Otimizações refinam a implementação,
mas nunca alteram a semântica da linguagem.
```

---

# 18.1910 Encerramento do Capítulo

O modelo de objetos apresentado neste capítulo constitui a base arquitetural sobre a qual toda a implementação oficial da Capi será construída.

Ele estabelece uma separação rigorosa entre:

* identidade e endereço;
* semântica e representação;
* especificação e implementação;
* garantias da linguagem e estratégias do Runtime.

Essa separação permite que a implementação evolua continuamente — incluindo novos backends, otimizações, mecanismos de instrumentação e melhorias de desempenho — sem comprometer as propriedades fundamentais da linguagem.

Com a conclusão deste capítulo, ficam definidos os contratos necessários para que o bootstrap em Rust, a futura implementação auto-hospedada em Capi e quaisquer evoluções posteriores compartilhem um único modelo de objetos consistente, verificável e compatível com os princípios estabelecidos em toda a especificação da linguagem.

Conclui-se, assim, o Capítulo 18 — **Modelo de Objetos, Layout e Despacho Dinâmico**.

---

# 19. Backend Cranelift

## 19.1 Arquitetura do Backend

### 19.1.1 Arquitetura Geral do Backend Cranelift

#### 19.1.1.1 Objetivos, Princípios e Papel do Backend

##### 19.1.1.1.1 Objetivos do Backend Cranelift

---

## 19.1.1.1.1.1 Objetivo

O Backend Cranelift constitui a primeira implementação oficial do gerador de código da linguagem Capi.

Seu objetivo é transformar a Intermediate Representation (IR) definida no Documento 07 em código nativo executável, preservando integralmente a semântica da linguagem.

---

## 19.1.1.1.1.2 Escopo

O Backend Cranelift é responsável exclusivamente pela geração de código.

Não faz parte de suas responsabilidades:

* análise léxica;
* parsing;
* resolução de nomes;
* inferência de tipos;
* validação semântica;
* construção da IR;
* otimizações pertencentes ao frontend.

---

## 19.1.1.1.1.3 Papel na Arquitetura

O Backend Cranelift representa a camada final do pipeline de compilação.

Sua entrada é a IR validada.

Sua saída consiste em código objeto compatível com a ABI definida pela linguagem.

---

## 19.1.1.1.1.4 Implementação Inicial

A implementação oficial do bootstrap utilizará o Cranelift como backend primário devido às seguintes características:

* arquitetura modular;
* API estável e bem documentada;
* excelente velocidade de compilação;
* simplicidade de integração;
* baixa complexidade de manutenção;
* suporte multiplataforma.

---

## 19.1.1.1.1.5 Justificativa

A escolha do Cranelift não representa uma decisão permanente sobre a tecnologia de geração de código.

Ela representa apenas a melhor alternativa para acelerar a implementação inicial da linguagem.

---

## 19.1.1.1.1.6 Independência

Toda a arquitetura do compilador deverá permanecer independente do Cranelift.

Nenhum componente do frontend poderá depender de estruturas internas específicas dessa biblioteca.

---

## 19.1.1.1.1.7 Backend Substituível

O Backend Cranelift deverá ser tratado como uma implementação concreta da interface de geração de código.

Sua substituição futura não deverá exigir alterações na semântica da linguagem.

---

## 19.1.1.1.1.8 Ausência de Dependência Semântica

Nenhuma regra da linguagem poderá depender de limitações ou capacidades específicas do Cranelift.

Caso exista divergência entre:

* a especificação da linguagem; e
* uma limitação do backend,

a especificação deverá prevalecer.

---

## 19.1.1.1.1.9 Responsabilidade Principal

O backend deverá produzir código nativo que preserve exatamente:

* comportamento observável;
* identidade dos objetos;
* lifetime;
* semântica de Domains;
* regras de ownership;
* contratos da ABI;
* garantias de segurança estabelecidas pela linguagem.

---

## 19.1.1.1.1.10 Determinismo

Dada uma mesma IR, o Backend Cranelift deverá produzir comportamento funcionalmente equivalente em todas as execuções.

Diferenças internas de layout ou otimização não deverão alterar o comportamento observável do programa.

---

## 19.1.1.1.1.11 Independência da Plataforma

O backend deverá abstrair as diferenças entre arquiteturas suportadas.

A IR não deverá conter conhecimento específico de:

* registradores;
* instruções;
* calling conventions nativas;
* formatos de objeto.

---

## 19.1.1.1.1.12 Preservação da IR

O Backend Cranelift não deverá modificar a semântica da IR recebida.

Seu papel consiste apenas em convertê-la para uma representação executável.

---

## 19.1.1.1.1.13 Correção Antes da Performance

O principal objetivo do backend será produzir código correto.

Otimizações deverão ser consideradas apenas após a preservação integral da semântica.

---

## 19.1.1.1.1.14 Performance de Compilação

Durante o bootstrap, a velocidade de compilação terá prioridade sobre otimizações agressivas de código.

Essa decisão busca acelerar:

* o ciclo de desenvolvimento;
* os testes;
* a evolução do compilador;
* a auto-hospedagem inicial.

---

## 19.1.1.1.1.15 Simplicidade

A implementação deverá privilegiar soluções simples, previsíveis e facilmente auditáveis.

---

## 19.1.1.1.1.16 Modularidade

O backend deverá ser composto por componentes independentes, cada um responsável por uma etapa específica da geração de código.

---

## 19.1.1.1.1.17 Isolamento

Nenhum componente interno do compilador deverá acessar diretamente estruturas internas do Cranelift.

Toda interação deverá ocorrer através das abstrações definidas pelo compilador.

---

## 19.1.1.1.1.18 Reprodutibilidade

Compilações equivalentes deverão produzir resultados reproduzíveis dentro das limitações inerentes ao formato de saída e às informações de depuração.

---

## 19.1.1.1.1.19 Escalabilidade

A arquitetura deverá permitir crescimento incremental sem necessidade de reestruturações profundas.

Novos recursos deverão ser adicionados como extensões da arquitetura existente.

---

## 19.1.1.1.1.20 Compatibilidade

O Backend Cranelift deverá implementar todos os contratos estabelecidos pelos seguintes documentos:

* Documento 02 — Sistema de Tipos;
* Documento 03 — Modelo de Memória;
* Documento 05 — Semântica Operacional;
* Documento 07 — Intermediate Representation;
* Documento 09 — Runtime Mínimo;
* Documento 10 — ABI e Interoperabilidade.

---

## 19.1.1.1.1.21 Evolução

A evolução do backend deverá ocorrer sem alterar:

* a IR;
* a semântica da linguagem;
* os contratos do Runtime;
* a ABI estabilizada.

---

## 19.1.1.1.1.22 Compatibilidade com LLVM

Todas as decisões arquiteturais deste backend deverão considerar a futura implementação do Backend LLVM.

Nenhuma decisão poderá dificultar ou impedir essa migração.

---

## 19.1.1.1.1.23 Neutralidade

O restante do compilador não deverá possuir conhecimento sobre qual backend está sendo utilizado.

A seleção do backend deverá ocorrer exclusivamente através da interface definida pelo compilador.

---

## 19.1.1.1.1.24 Objetivo do Bootstrap

Durante o bootstrap, o Backend Cranelift deverá permitir que o compilador:

* compile programas válidos da linguagem;
* gere binários executáveis;
* execute toda a suíte de testes;
* compile a biblioteca padrão;
* evolua progressivamente até a auto-hospedagem.

---

## 19.1.1.1.1.25 Objetivo de Longo Prazo

Após a introdução do Backend LLVM, o Backend Cranelift continuará desempenhando papel estratégico como:

* backend de referência para desenvolvimento;
* backend de compilação rápida;
* backend para testes diferenciais;
* backend para validação de regressões;
* implementação alternativa da interface de geração de código.

---

## 19.1.1.1.1.26 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um Backend Cranelift capaz de:

* consumir a IR produzida pelo compilador;
* gerar código nativo semanticamente equivalente à especificação da linguagem;
* operar de forma independente dos componentes do frontend;
* preservar todos os contratos definidos pela arquitetura da Capi;
* servir como base para o processo de bootstrap e para a futura coexistência com o Backend LLVM.

---

#### 19.1.1.1.2 Princípios Arquiteturais

---

## 19.1.1.1.2.1 Objetivo

Os princípios arquiteturais definidos nesta seção estabelecem as diretrizes que deverão orientar toda a implementação do Backend Cranelift.

Esses princípios são independentes da tecnologia utilizada e deverão permanecer válidos mesmo após a introdução de novos backends.

---

## 19.1.1.1.2.2 Arquitetura em Camadas

O Backend Cranelift deverá ser organizado em camadas bem definidas, de modo que cada uma possua responsabilidades específicas e interfaces claramente estabelecidas.

Nenhuma camada deverá assumir responsabilidades pertencentes a outra.

---

## 19.1.1.1.2.3 Separação de Responsabilidades

A implementação deverá manter uma separação rigorosa entre:

* representação intermediária da linguagem;
* geração de código;
* gerenciamento do Runtime;
* emissão de arquivos objeto;
* integração com o linker.

---

## 19.1.1.1.2.4 Independência da IR

A Intermediate Representation deverá permanecer completamente independente do Backend Cranelift.

A IR representa conceitos da linguagem, enquanto o backend representa apenas uma estratégia de geração de código.

---

## 19.1.1.1.2.5 Lowering como Fronteira

A conversão entre a IR da linguagem e a representação utilizada pelo Cranelift deverá ocorrer exclusivamente na etapa de *lowering*.

Nenhum componente anterior deverá produzir estruturas específicas do Cranelift.

---

## 19.1.1.1.2.6 Backend como Consumidor

O Backend Cranelift deverá atuar exclusivamente como consumidor da IR validada.

Ele não deverá modificar nem reinterpretar a semântica recebida.

---

## 19.1.1.1.2.7 Imutabilidade da Entrada

Após o início da geração de código, a IR deverá ser tratada como uma estrutura conceitualmente imutável.

Alterações estruturais deverão ocorrer apenas nas etapas anteriores do pipeline.

---

## 19.1.1.1.2.8 Neutralidade Tecnológica

Nenhum componente externo ao backend deverá depender:

* da API do Cranelift;
* de seus tipos internos;
* de suas estruturas de dados;
* de sua representação intermediária.

---

## 19.1.1.1.2.9 Encapsulamento

Toda interação com o Cranelift deverá ocorrer através de uma camada de abstração própria do compilador.

Essa camada constitui a fronteira oficial entre o compilador e a biblioteca de geração de código.

---

## 19.1.1.1.2.10 Backend API

O compilador deverá comunicar-se com qualquer backend por meio de uma interface comum.

Essa interface deverá representar operações da linguagem, e não operações específicas do Cranelift.

---

## 19.1.1.1.2.11 Ausência de Acoplamento Inverso

O Cranelift não deverá possuir conhecimento sobre:

* o sistema de tipos da Capi;
* o modelo de Domains;
* o ObjectId;
* a semântica da linguagem.

Esses conceitos deverão ser completamente resolvidos antes da etapa de geração de código.

---

## 19.1.1.1.2.12 Determinismo Arquitetural

Para uma mesma IR e um mesmo conjunto de opções de compilação, o backend deverá produzir um resultado funcionalmente determinístico.

---

## 19.1.1.1.2.13 Evolução Incremental

Novas funcionalidades deverão ser adicionadas sem exigir reestruturações significativas da arquitetura existente.

A implementação deverá privilegiar extensibilidade em detrimento de soluções específicas para casos isolados.

---

## 19.1.1.1.2.14 Modularidade

Os componentes internos do backend deverão ser organizados em módulos independentes, especializados em responsabilidades específicas.

Exemplos incluem:

* lowering;
* geração de funções;
* geração de dados estáticos;
* emissão de símbolos;
* integração com o Runtime;
* emissão de arquivos objeto.

---

## 19.1.1.1.2.15 Coesão

Cada módulo deverá possuir um único propósito arquitetural claramente definido.

---

## 19.1.1.1.2.16 Baixo Acoplamento

As dependências entre módulos deverão ocorrer exclusivamente através de interfaces bem definidas.

Dependências circulares deverão ser evitadas.

---

## 19.1.1.1.2.17 Reutilização

Sempre que possível, componentes deverão ser reutilizáveis por outros backends.

Especial atenção deverá ser dada às etapas independentes da tecnologia de geração de código.

---

## 19.1.1.1.2.18 Compartilhamento de Infraestrutura

Infraestruturas relacionadas a:

* diagnósticos;
* logging;
* estatísticas;
* benchmarking;
* verificação;
* configuração;

deverão ser compartilhadas entre todos os backends.

---

## 19.1.1.1.2.19 Especialização

Componentes específicos do Cranelift deverão permanecer isolados daqueles que representam conceitos gerais do compilador.

---

## 19.1.1.1.2.20 Clareza Arquitetural

A organização do código deverá refletir a arquitetura do compilador.

A estrutura dos módulos deverá permitir identificar facilmente o papel de cada componente.

---

## 19.1.1.1.2.21 Auditabilidade

O fluxo de geração de código deverá ser facilmente auditável.

Cada transformação deverá possuir um ponto claramente identificável na implementação.

---

## 19.1.1.1.2.22 Observabilidade

A arquitetura deverá permitir a instrumentação das principais etapas da geração de código, incluindo:

* lowering;
* construção de funções;
* geração de objetos;
* emissão de símbolos;
* geração de arquivos objeto.

---

## 19.1.1.1.2.23 Diagnósticos

Falhas internas deverão produzir diagnósticos claros, determinísticos e suficientemente detalhados para facilitar sua reprodução.

---

## 19.1.1.1.2.24 Independência de Plataforma

A arquitetura não deverá incorporar decisões específicas de arquiteturas como x86-64, AArch64 ou RISC-V.

Essas diferenças deverão ser tratadas exclusivamente pelo backend e pelo Cranelift.

---

## 19.1.1.1.2.25 Compatibilidade Multiplataforma

A mesma arquitetura deverá permitir a geração de código para múltiplos sistemas operacionais e arquiteturas suportadas pelo Cranelift, sem alterações na IR ou no frontend.

---

## 19.1.1.1.2.26 Independência do Linker

A geração de código deverá permanecer desacoplada da ferramenta de linkedição utilizada.

O backend deverá produzir artefatos compatíveis com os formatos de objeto suportados, deixando a resolução final para o processo de linkedição.

---

## 19.1.1.1.2.27 Preparação para LLVM

Todas as decisões arquiteturais desta implementação deverão permitir que um Backend LLVM reutilize a maior parte da infraestrutura construída.

---

## 19.1.1.1.2.28 Compatibilidade Futura

A introdução de novos backends não deverá exigir alterações na arquitetura do frontend, da IR, do sistema de tipos ou do Runtime.

---

## 19.1.1.1.2.29 Princípio da Substituibilidade

Qualquer backend que implemente integralmente a Backend API deverá poder substituir o Backend Cranelift sem alterar o comportamento observável dos programas compilados.

---

## 19.1.1.1.2.30 Critérios de Conclusão

Esta seção será considerada atendida quando a implementação oficial possuir uma arquitetura que:

* mantenha separação rigorosa entre frontend e backend;
* preserve a independência da IR;
* encapsule completamente o Cranelift;
* permita reutilização de infraestrutura por outros backends;
* favoreça evolução incremental e manutenção de longo prazo;
* assegure que a tecnologia de geração de código permaneça um detalhe de implementação, e não parte da definição da linguagem.

---

#### 19.1.1.1.3 Papel do Backend na Arquitetura do Compilador

---

## 19.1.1.1.3.1 Objetivo

Esta seção define o papel arquitetural do Backend Cranelift dentro da implementação oficial do compilador Capi.

Seu propósito é estabelecer claramente as responsabilidades do backend, seus limites de atuação e sua interação com os demais componentes do compilador.

---

## 19.1.1.1.3.2 Posição no Pipeline

O Backend Cranelift constitui a etapa final do pipeline de compilação.

Ele recebe exclusivamente uma IR completamente validada e produz como resultado código objeto executável.

---

## 19.1.1.1.3.3 Fluxo Arquitetural

De forma conceitual, o pipeline deverá seguir a seguinte organização:

```text
Código-fonte

↓

Lexer

↓

Parser

↓

AST

↓

HIR

↓

Resolução de Nomes

↓

Inferência e Verificação de Tipos

↓

IR

↓

Backend Cranelift

↓

Código Objeto

↓

Linkedição

↓

Executável
```

O Backend Cranelift inicia sua atuação somente após a conclusão das etapas anteriores.

---

## 19.1.1.1.3.4 Dependência Unidirecional

O backend dependerá exclusivamente da IR e das abstrações disponibilizadas pelo compilador.

Nenhuma etapa anterior deverá depender do backend.

---

## 19.1.1.1.3.5 Fronteira Arquitetural

A IR representa a fronteira oficial entre:

* frontend; e
* backend.

Após essa fronteira, nenhuma decisão semântica da linguagem deverá permanecer pendente.

---

## 19.1.1.1.3.6 Entrada do Backend

O Backend Cranelift deverá assumir que a IR recebida:

* é semanticamente válida;
* possui tipos resolvidos;
* possui símbolos resolvidos;
* possui ownership validado;
* possui lifetimes consistentes;
* possui todas as verificações obrigatórias concluídas.

---

## 19.1.1.1.3.7 Ausência de Revalidação

O backend não deverá repetir verificações já realizadas pelas etapas anteriores.

Caso inconsistências sejam detectadas, elas deverão ser tratadas como falhas internas da implementação.

---

## 19.1.1.1.3.8 Responsabilidade Exclusiva

Compete ao Backend Cranelift:

* converter IR em código nativo;
* gerar funções;
* gerar constantes;
* gerar tabelas de dados;
* emitir símbolos;
* produzir arquivos objeto;
* preservar a ABI.

---

## 19.1.1.1.3.9 Responsabilidades Excluídas

Não compete ao backend:

* inferir tipos;
* resolver overloads;
* selecionar métodos virtuais;
* validar ownership;
* validar Domains;
* realizar análise semântica.

Essas atividades pertencem exclusivamente ao frontend.

---

## 19.1.1.1.3.10 Preservação da Semântica

Toda decisão tomada pelo backend deverá preservar exatamente a semântica representada na IR.

Nenhuma transformação poderá modificar o comportamento observável do programa.

---

## 19.1.1.1.3.11 Papel do Lowering

O Backend Cranelift deverá iniciar sua atuação convertendo a IR da linguagem para a representação intermediária utilizada pelo Cranelift (*Cranelift IR*).

Essa conversão constitui o único ponto de adaptação entre a linguagem Capi e a infraestrutura de geração de código.

---

## 19.1.1.1.3.12 Responsabilidade pela Geração de Código

Após o *lowering*, caberá ao backend:

* construir funções;
* organizar blocos básicos;
* emitir instruções;
* materializar chamadas;
* gerar dados estáticos;
* produzir código objeto.

---

## 19.1.1.1.3.13 Integração com o Runtime

Sempre que necessário, o backend deverá gerar chamadas para rotinas do Runtime definidas pelo Documento 09.

Essas chamadas deverão ocorrer exclusivamente através das interfaces públicas do Runtime.

---

## 19.1.1.1.3.14 Ausência de Conhecimento Interno

O Backend Cranelift não deverá depender da implementação interna do Runtime.

Ele deverá conhecer apenas:

* assinaturas públicas;
* contratos de chamada;
* convenções da ABI.

---

## 19.1.1.1.3.15 Geração de Chamadas

Chamadas para:

* construtores;
* destruidores;
* helpers;
* rotinas do Runtime;
* funções externas;

deverão respeitar integralmente a ABI definida para a linguagem.

---

## 19.1.1.1.3.16 Materialização da Identidade

Sempre que a IR representar operações envolvendo identidade de objetos, caberá ao backend materializar essas operações utilizando os mecanismos definidos pelo Runtime.

O backend não deverá redefinir o conceito de identidade.

---

## 19.1.1.1.3.17 Materialização de Domains

Operações relacionadas a Domains deverão ser traduzidas para chamadas ou instruções compatíveis com a implementação oficial do Runtime.

---

## 19.1.1.1.3.18 Materialização de Despacho Dinâmico

Chamadas virtuais deverão ser convertidas em sequências de instruções compatíveis com o modelo de despacho definido no Capítulo 18.

O backend não deverá alterar a estratégia de despacho estabelecida pela arquitetura.

---

## 19.1.1.1.3.19 Materialização de Objetos

A representação física dos objetos deverá seguir rigorosamente o layout definido anteriormente.

O backend não poderá alterar:

* offsets;
* alinhamento;
* organização do cabeçalho;
* representação dos campos.

---

## 19.1.1.1.3.20 Integração com a ABI

Todo símbolo emitido pelo backend deverá obedecer:

* convenções de chamada;
* alinhamento;
* convenções de retorno;
* representação de parâmetros;
* visibilidade;
* linkage.

---

## 19.1.1.1.3.21 Integração com o Linker

O backend deverá produzir artefatos compatíveis com o processo de linkedição suportado pela plataforma.

A resolução final de símbolos pertence ao linker, não ao backend.

---

## 19.1.1.1.3.22 Papel nas Otimizações

O Backend Cranelift poderá executar otimizações de geração de código desde que:

* preservem integralmente a semântica;
* respeitem a ABI;
* mantenham os contratos do Runtime.

---

## 19.1.1.1.3.23 Responsabilidade pela Performance

O backend deverá buscar produzir código eficiente, porém a correção funcional terá prioridade absoluta sobre desempenho.

---

## 19.1.1.1.3.24 Papel durante o Bootstrap

Durante o bootstrap, o Backend Cranelift constitui o backend oficial de produção.

Toda a infraestrutura do compilador deverá ser validada utilizando esse backend.

---

## 19.1.1.1.3.25 Papel após a Introdução do LLVM

Após a implementação do Backend LLVM, o Backend Cranelift permanecerá como:

* backend de referência para desenvolvimento;
* backend de compilação rápida;
* backend de testes diferenciais;
* backend alternativo oficialmente suportado.

---

## 19.1.1.1.3.26 Equivalência Funcional

Independentemente do backend utilizado, o comportamento observável dos programas compilados deverá permanecer funcionalmente equivalente.

Diferenças deverão restringir-se a aspectos de desempenho, tamanho do código ou qualidade das otimizações.

---

## 19.1.1.1.3.27 Relação com a Backend API

Toda funcionalidade específica do Cranelift deverá permanecer encapsulada atrás da Backend API.

Essa interface constitui o único ponto de comunicação entre o restante do compilador e qualquer backend.

---

## 19.1.1.1.3.28 Evolução Arquitetural

A arquitetura deverá permitir a introdução de novos backends sem necessidade de alterações na IR ou nas demais etapas do pipeline.

---

## 19.1.1.1.3.29 Invariantes

Ao longo de toda sua execução, o Backend Cranelift deverá preservar os seguintes invariantes:

* consumir apenas IR válida;
* não modificar a semântica da linguagem;
* respeitar integralmente a ABI;
* manter compatibilidade com o Runtime;
* permanecer desacoplado do frontend;
* encapsular completamente as APIs específicas do Cranelift;
* produzir código objeto compatível com os formatos suportados pela plataforma.

---

## 19.1.1.1.3.30 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um Backend Cranelift que:

* ocupe exclusivamente a etapa final do pipeline de compilação;
* atue apenas sobre a IR validada;
* gere código nativo preservando todos os contratos definidos pela linguagem;
* mantenha separação rigorosa entre frontend, Runtime, Backend API e infraestrutura de geração de código;
* possa ser substituído por outro backend compatível sem qualquer alteração no comportamento observável dos programas compilados.

---

### 19.1.1.2 Arquitetura Interna e Fluxo de Geração de Código

---

## 19.1.1.2.1 Objetivo

Esta seção define a organização interna do Backend Cranelift e o fluxo conceitual adotado durante a geração de código.

Seu propósito é estabelecer uma arquitetura modular, previsível e independente da implementação específica do Cranelift, permitindo que futuras implementações reutilizem a mesma organização geral.

---

## 19.1.1.2.2 Arquitetura em Pipeline

Internamente, o Backend Cranelift deverá ser organizado como um pipeline composto por etapas bem definidas.

Cada etapa deverá consumir exclusivamente a saída produzida pela etapa imediatamente anterior.

---

## 19.1.1.2.3 Fluxo Conceitual

O fluxo interno poderá ser representado conceitualmente da seguinte forma:

```text
IR Validada
        │
        ▼
Lowering
        │
        ▼
Construção da Cranelift IR
        │
        ▼
Geração de Funções
        │
        ▼
Geração de Dados Estáticos
        │
        ▼
Materialização de Símbolos
        │
        ▼
Emissão do Código Objeto
        │
        ▼
Artefato para Linkedição
```

Esse fluxo representa uma divisão lógica de responsabilidades e não impõe uma implementação específica.

---

## 19.1.1.2.4 Independência das Etapas

Cada etapa deverá possuir responsabilidades claramente delimitadas.

Alterações em uma etapa não deverão exigir modificações estruturais nas demais.

---

## 19.1.1.2.5 Comunicação

A comunicação entre etapas deverá ocorrer exclusivamente através de estruturas intermediárias bem definidas.

Nenhuma etapa deverá acessar diretamente o estado interno de outra.

---

## 19.1.1.2.6 Contexto de Compilação

Todo o processo de geração de código deverá ocorrer dentro de um contexto de compilação próprio do backend.

Esse contexto concentrará informações necessárias durante a geração, evitando dependências globais.

---

## 19.1.1.2.7 BackendContext

Conceitualmente, o Backend Cranelift deverá possuir um **BackendContext**, responsável por armazenar informações compartilhadas durante a geração de código, incluindo:

* target;
* configuração de otimizações;
* convenções de chamada;
* referências ao Runtime;
* tabela de símbolos internos;
* estatísticas da compilação.

A estrutura exata permanece um detalhe de implementação.

---

## 19.1.1.2.8 BackendModule

Cada unidade de compilação deverá ser representada internamente por um **BackendModule**.

Esse módulo deverá agrupar todos os artefatos produzidos durante a geração de código para uma unidade lógica da linguagem.

---

## 19.1.1.2.9 BackendFunction

Cada função da IR deverá possuir uma representação intermediária própria durante o processo de geração.

Essa representação deverá permanecer isolada das demais funções.

---

## 19.1.1.2.10 BackendBuilder

A construção incremental do código deverá ocorrer através de componentes especializados, aqui denominados conceitualmente **BackendBuilders**.

Esses componentes serão responsáveis por transformar elementos da IR em instruções do backend.

---

## 19.1.1.2.11 Responsabilidade do Lowering

O lowering constitui a primeira etapa efetiva do backend.

Seu papel consiste em converter os elementos semânticos da IR para construções equivalentes na representação utilizada pelo Cranelift.

---

## 19.1.1.2.12 Neutralidade do Lowering

O lowering não deverá realizar otimizações semânticas.

Sua responsabilidade limita-se à tradução entre representações.

---

## 19.1.1.2.13 Construção da Cranelift IR

Após o lowering, deverá ser construída a representação intermediária utilizada internamente pelo Cranelift.

Essa representação constitui um detalhe de implementação do backend.

---

## 19.1.1.2.14 Organização por Funções

Cada função deverá ser gerada de forma independente.

Essa organização facilita:

* paralelização futura;
* isolamento de erros;
* reutilização de infraestrutura;
* testes unitários.

---

## 19.1.1.2.15 Organização por Módulos

Símbolos pertencentes à mesma unidade lógica deverão ser agrupados em módulos.

O backend deverá preservar essa organização até a emissão do código objeto.

---

## 19.1.1.2.16 Dados Estáticos

Constantes, tabelas e demais estruturas estáticas deverão possuir uma etapa específica de geração.

Essa etapa deverá permanecer desacoplada da geração de funções.

---

## 19.1.1.2.17 Símbolos

A criação de símbolos deverá ocorrer somente após a materialização completa das entidades correspondentes.

Nenhum símbolo deverá representar entidades parcialmente construídas.

---

## 19.1.1.2.18 Dependências

Dependências entre funções deverão ser resolvidas através do mecanismo de símbolos da própria plataforma.

O backend não deverá implementar mecanismos próprios de resolução tardia.

---

## 19.1.1.2.19 Integração com o Runtime

Chamadas ao Runtime deverão ser tratadas como chamadas externas comuns.

O backend deverá conhecer apenas:

* assinatura;
* convenção de chamada;
* visibilidade;
* linkage.

---

## 19.1.1.2.20 Encapsulamento do Cranelift

Todo acesso às APIs do Cranelift deverá permanecer concentrado em componentes específicos do backend.

O restante do compilador jamais deverá depender dessas APIs.

---

## 19.1.1.2.21 Isolamento Tecnológico

A arquitetura deverá permitir que um Backend LLVM implemente exatamente o mesmo fluxo conceitual utilizando outra infraestrutura de geração de código.

---

## 19.1.1.2.22 Organização dos Componentes

Conceitualmente, a arquitetura interna poderá ser organizada em componentes equivalentes aos seguintes:

* BackendContext;
* BackendModule;
* BackendFunction;
* Lowering;
* Function Builder;
* Data Builder;
* Symbol Manager;
* Object Emitter.

Os nomes concretos permanecem livres para a implementação.

---

## 19.1.1.2.23 Ausência de Estado Global

A geração de código não deverá depender de estados globais mutáveis.

Toda informação necessária deverá estar contida no contexto da compilação.

---

## 19.1.1.2.24 Reentrância

Sempre que tecnicamente viável, os componentes internos deverão ser projetados para permitir execução reentrante e futura paralelização.

---

## 19.1.1.2.25 Tratamento de Erros

Erros internos deverão interromper apenas a geração da unidade afetada sempre que isso não comprometer a consistência global da compilação.

---

## 19.1.1.2.26 Observabilidade

Cada etapa do pipeline deverá ser instrumentável de forma independente.

Essa instrumentação poderá registrar:

* tempo de execução;
* memória utilizada;
* quantidade de instruções geradas;
* estatísticas de lowering;
* quantidade de símbolos emitidos.

---

## 19.1.1.2.27 Diagnósticos

Sempre que possível, diagnósticos deverão manter referência aos elementos originais da IR.

Isso facilitará a correlação entre falhas do backend e a representação intermediária produzida pelo frontend.

---

## 19.1.1.2.28 Evolução Incremental

Novas etapas poderão ser introduzidas no pipeline desde que:

* preservem as interfaces existentes;
* não alterem a semântica da linguagem;
* mantenham compatibilidade com os demais backends.

---

## 19.1.1.2.29 Invariantes

Durante toda a geração de código deverão permanecer verdadeiros os seguintes princípios:

* a IR é a única entrada do backend;
* o lowering constitui a única fronteira entre a IR e o Cranelift;
* funções são geradas independentemente;
* dados estáticos possuem fluxo próprio;
* símbolos representam apenas entidades completas;
* o Runtime é acessado exclusivamente por interfaces públicas;
* o Cranelift permanece encapsulado;
* o pipeline interno permanece modular.

---

## 19.1.1.2.30 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma arquitetura interna que:

* organize claramente o fluxo de geração de código;
* mantenha separação entre todas as etapas do backend;
* encapsule completamente a infraestrutura do Cranelift;
* permita reutilização por futuros backends;
* favoreça manutenção, auditoria e evolução incremental;
* preserve integralmente os contratos arquiteturais definidos para o compilador Capi.

---

### 19.1.1.3 Integração com a IR, o Runtime e a Backend API

---

## 19.1.1.3.1 Objetivo

Esta seção estabelece a forma como o Backend Cranelift deverá integrar-se com os demais componentes da arquitetura do compilador.

Seu objetivo é definir contratos claros entre a IR, a Backend API, o Runtime e o processo de geração de código, preservando a independência entre essas camadas.

---

## 19.1.1.3.2 Arquitetura de Integração

O Backend Cranelift deverá interagir exclusivamente com três componentes externos:

* Intermediate Representation (IR);
* Backend API;
* Runtime Oficial.

Qualquer outra dependência deverá ser considerada uma violação arquitetural.

---

## 19.1.1.3.3 Fluxo Conceitual

A integração poderá ser representada conceitualmente da seguinte forma:

```text
                  Frontend
                      │
                      ▼
             Intermediate Representation
                      │
                      ▼
                Backend API
                      │
        ┌─────────────┴─────────────┐
        ▼                           ▼
Backend Cranelift             Backend LLVM
        │                           │
        └─────────────┬─────────────┘
                      ▼
              Código Objeto
                      │
                      ▼
                 Linkedição
                      │
                      ▼
                Runtime Oficial
```

A Backend API constitui a única fronteira oficial entre o compilador e qualquer backend.

---

## 19.1.1.3.4 IR como Contrato

A IR deverá representar o contrato formal entre o frontend e os backends.

Nenhum backend poderá depender de estruturas produzidas antes da IR.

---

## 19.1.1.3.5 Completude da IR

Quando entregue ao backend, a IR deverá conter todas as informações necessárias para a geração de código.

Não deverão existir informações implícitas dependentes do frontend.

---

## 19.1.1.3.6 Imutabilidade Conceitual

Durante a geração de código, a IR deverá ser tratada como conceitualmente imutável.

Caso sejam necessárias estruturas auxiliares, estas deverão ser produzidas pelo próprio backend.

---

## 19.1.1.3.7 Backend API

A Backend API deverá definir todas as operações necessárias para geração de código, abstraindo completamente a tecnologia utilizada pelo backend.

---

## 19.1.1.3.8 Papel da Backend API

A Backend API deverá:

* abstrair o backend utilizado;
* definir contratos estáveis;
* encapsular detalhes tecnológicos;
* permitir múltiplas implementações.

---

## 19.1.1.3.9 Independência da API

Nenhuma operação da Backend API deverá expor:

* tipos do Cranelift;
* tipos do LLVM;
* estruturas específicas de qualquer backend.

---

## 19.1.1.3.10 Representação Conceitual

A Backend API deverá representar operações como:

* criação de função;
* emissão de instruções;
* geração de constantes;
* criação de símbolos;
* chamadas externas;
* emissão de módulos.

Nunca deverá representar diretamente instruções específicas de uma arquitetura.

---

## 19.1.1.3.11 Backend como Implementação

O Backend Cranelift constitui uma implementação concreta da Backend API.

Ele não define a interface.

Apenas a implementa.

---

## 19.1.1.3.12 Ausência de Conhecimento Inverso

A Backend API não deverá possuir conhecimento sobre:

* Cranelift;
* LLVM;
* tecnologias futuras.

Ela deverá permanecer completamente neutra.

---

## 19.1.1.3.13 Integração com o Runtime

O Backend Cranelift deverá considerar o Runtime como um conjunto de serviços públicos disponibilizados para o código gerado.

---

## 19.1.1.3.14 Runtime como Biblioteca

O Runtime não deverá ser tratado como uma extensão do backend.

Ele constitui um componente independente da arquitetura.

---

## 19.1.1.3.15 Comunicação com o Runtime

Toda interação entre o código gerado e o Runtime deverá ocorrer através de chamadas públicas compatíveis com a ABI da linguagem.

---

## 19.1.1.3.16 Ausência de Acesso Interno

O Backend Cranelift não deverá acessar estruturas internas do Runtime.

Somente contratos públicos poderão ser utilizados.

---

## 19.1.1.3.17 Helpers do Runtime

Sempre que a linguagem exigir operações não materializáveis diretamente em código nativo, o backend deverá emitir chamadas para *runtime helpers* apropriados.

---

## 19.1.1.3.18 Helpers como Contrato

Os *runtime helpers* deverão possuir contratos públicos bem definidos.

Sua implementação poderá evoluir sem exigir alterações no backend.

---

## 19.1.1.3.19 Materialização de Operações

Entre as operações normalmente materializadas através do Runtime poderão estar:

* criação de Domains;
* destruição de Domains;
* gerenciamento de ObjectId;
* geração de identifiers;
* verificações opcionais de segurança;
* serviços auxiliares do Runtime.

A lista definitiva permanece definida pelo Documento 09.

---

## 19.1.1.3.20 ABI como Fronteira

Toda comunicação entre backend e Runtime deverá obedecer rigorosamente à ABI oficial da linguagem.

---

## 19.1.1.3.21 Ausência de Convenções Paralelas

O Backend Cranelift não deverá introduzir convenções próprias de chamada para comunicação com o Runtime.

---

## 19.1.1.3.22 Resolução de Símbolos

Símbolos pertencentes ao Runtime deverão ser resolvidos durante a etapa de linkedição.

O backend não deverá resolver endereços concretos.

---

## 19.1.1.3.23 Backend Agnóstico

Sempre que possível, o Runtime não deverá distinguir qual backend gerou o código executado.

---

## 19.1.1.3.24 Equivalência Funcional

Programas gerados pelo Backend Cranelift e pelo Backend LLVM deverão produzir chamadas funcionalmente equivalentes ao Runtime.

---

## 19.1.1.3.25 Integração com Diagnósticos

O backend deverá preservar informações suficientes para que diagnósticos possam ser correlacionados com elementos da IR.

---

## 19.1.1.3.26 Integração com Debug

Informações de depuração deverão manter associação entre:

* IR;
* símbolos;
* código objeto;
* código executável.

---

## 19.1.1.3.27 Integração com Instrumentação

Os mecanismos de instrumentação definidos anteriormente deverão permanecer independentes da tecnologia do backend.

---

## 19.1.1.3.28 Integração com Benchmarks

A infraestrutura de benchmarking deverá medir:

* tempo de lowering;
* tempo de geração;
* tempo de emissão;
* tempo total de backend.

Esses mecanismos deverão ser compartilhados por todos os backends.

---

## 19.1.1.3.29 Integração com Testes

A Backend API deverá permitir que uma mesma suíte de testes seja executada sobre diferentes implementações de backend.

---

## 19.1.1.3.30 Testes Diferenciais

A arquitetura deverá permitir comparar automaticamente a saída produzida pelos diferentes backends.

---

## 19.1.1.3.31 Evolução Independente

Frontend, Runtime e Backend deverão poder evoluir de forma relativamente independente, desde que preservem seus contratos públicos.

---

## 19.1.1.3.32 Compatibilidade Binária

Alterações internas da Backend API não deverão comprometer a compatibilidade binária previamente estabilizada da linguagem.

---

## 19.1.1.3.33 Evolução da Backend API

Novas funcionalidades deverão ser incorporadas através da evolução da Backend API, e não por acessos diretos aos backends.

---

## 19.1.1.3.34 Neutralidade Arquitetural

A arquitetura deverá permitir que novos backends sejam adicionados futuramente sem necessidade de alterações na IR ou no Runtime.

---

## 19.1.1.3.35 Responsabilidades da IR

Compete exclusivamente à IR representar:

* semântica da linguagem;
* fluxo de controle;
* fluxo de dados;
* tipos;
* operações.

---

## 19.1.1.3.36 Responsabilidades da Backend API

Compete exclusivamente à Backend API definir:

* contratos de geração;
* abstrações comuns;
* fronteiras arquiteturais;
* interoperabilidade entre compilador e backends.

---

## 19.1.1.3.37 Responsabilidades do Backend

Compete exclusivamente ao Backend Cranelift:

* implementar a Backend API;
* gerar código objeto;
* integrar-se ao Runtime;
* preservar os contratos da ABI.

---

## 19.1.1.3.38 Responsabilidades do Runtime

Compete exclusivamente ao Runtime fornecer os serviços necessários ao código gerado, permanecendo independente da tecnologia utilizada pelo backend.

---

## 19.1.1.3.39 Invariantes

Durante toda a integração deverão permanecer verdadeiros os seguintes princípios:

* a IR constitui a única entrada do backend;
* a Backend API constitui a única interface entre o compilador e os backends;
* o Runtime constitui a única infraestrutura pública acessível pelo código gerado;
* nenhum componente deverá depender de detalhes internos dos demais;
* todos os contratos deverão ser definidos por interfaces públicas.

---

## 19.1.1.3.40 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma arquitetura em que:

* a IR represente integralmente a semântica da linguagem;
* a Backend API encapsule completamente a infraestrutura de geração de código;
* o Backend Cranelift implemente essa interface sem expor detalhes tecnológicos;
* o Runtime seja acessado exclusivamente através de contratos públicos compatíveis com a ABI;
* novos backends possam ser introduzidos preservando integralmente a arquitetura do compilador e o comportamento observável dos programas compilados.

---

## 19.1.2 Backend API

### 19.1.2.1 Objetivo

Esta seção define a **Backend API** da implementação oficial do compilador Capi.

A Backend API constitui a interface arquitetural que separa o restante do compilador da infraestrutura responsável pela geração de código.

Seu propósito é garantir que diferentes implementações de backend possam coexistir preservando exatamente os mesmos contratos semânticos.

---

## 19.1.2.2 Papel Arquitetural

A Backend API representa a única interface oficial entre:

* a IR da linguagem;
* o gerador de código;
* o processo de emissão de artefatos.

Nenhum outro mecanismo deverá ser utilizado para integrar novos backends ao compilador.

---

## 19.1.2.3 Independência

A Backend API deverá permanecer completamente independente de qualquer tecnologia específica de geração de código.

Ela não deverá depender de:

* Cranelift;
* LLVM;
* GCC;
* WebAssembly;
* qualquer biblioteca externa.

---

## 19.1.2.4 Interface Conceitual

A Backend API deverá representar exclusivamente conceitos arquiteturais da linguagem.

Ela jamais deverá representar instruções específicas de processadores ou de bibliotecas de geração de código.

---

## 19.1.2.5 Estabilidade

A Backend API deverá permanecer significativamente mais estável que suas implementações concretas.

Alterações internas em um backend não deverão exigir alterações nessa interface.

---

## 19.1.2.6 Contrato

A Backend API constitui um contrato formal entre:

* o frontend;
* a IR;
* o Runtime;
* os backends.

Todas as implementações deverão obedecer rigorosamente esse contrato.

---

## 19.1.2.7 Objetivos

A Backend API possui os seguintes objetivos:

* desacoplamento arquitetural;
* reutilização;
* testabilidade;
* substituibilidade;
* manutenção;
* evolução incremental.

---

## 19.1.2.8 Responsabilidade

Compete à Backend API definir:

* operações disponíveis;
* sequência lógica da geração de código;
* contratos de entrada;
* contratos de saída;
* pontos de extensão.

---

## 19.1.2.9 Não Responsabilidades

Não compete à Backend API definir:

* semântica da linguagem;
* sistema de tipos;
* Runtime;
* layout dos objetos;
* otimizações específicas.

---

## 19.1.2.10 Fluxo Arquitetural

Conceitualmente, toda geração de código deverá obedecer ao seguinte fluxo:

```text
IR

↓

Backend API

↓

Backend Concreto

↓

Código Objeto
```

---

## 19.1.2.11 Backend Concreto

Uma implementação concreta deverá ser responsável apenas por traduzir as operações definidas pela Backend API para a infraestrutura utilizada.

---

## 19.1.2.12 Backend Agnóstico

O restante do compilador jamais deverá conhecer qual backend está sendo utilizado.

---

## 19.1.2.13 Seleção do Backend

A seleção do backend deverá ocorrer em um único ponto da arquitetura.

Os demais componentes deverão permanecer completamente independentes dessa decisão.

---

## 19.1.2.14 BackendContext

Toda compilação deverá ocorrer dentro de um contexto próprio do backend.

Esse contexto conterá todas as informações necessárias para a geração de código.

Sua estrutura permanece um detalhe de implementação.

---

## 19.1.2.15 BackendModule

A Backend API deverá representar conceitualmente módulos independentes.

Cada módulo corresponderá a uma unidade lógica de compilação.

---

## 19.1.2.16 BackendFunction

Cada função deverá possuir um contexto próprio de geração.

Esse contexto permanecerá isolado das demais funções.

---

## 19.1.2.17 BackendData

Constantes e demais dados estáticos deverão possuir representação própria na Backend API.

---

## 19.1.2.18 BackendSymbol

Símbolos deverão ser representados como entidades próprias.

A Backend API não deverá depender da implementação do linker.

---

## 19.1.2.19 BackendType

A Backend API poderá possuir uma representação abstrata de tipos destinada exclusivamente à geração de código.

Essa representação não substitui o sistema de tipos da linguagem.

---

## 19.1.2.20 BackendValue

Valores intermediários produzidos durante a geração poderão possuir representação própria.

Essa representação deverá permanecer interna ao backend.

---

## 19.1.2.21 BackendBlock

Blocos básicos deverão ser representados por abstrações próprias.

Sua implementação concreta permanece livre.

---

## 19.1.2.22 BackendBuilder

A construção incremental do código deverá ocorrer através de componentes especializados.

---

## 19.1.2.23 BackendEmitter

A emissão final do código objeto deverá ocorrer por um componente dedicado.

---

## 19.1.2.24 BackendDiagnostics

Diagnósticos internos deverão utilizar infraestrutura comum compartilhada entre todos os backends.

---

## 19.1.2.25 BackendStatistics

Estatísticas de compilação deverão ser produzidas através de uma interface comum.

---

## 19.1.2.26 BackendConfiguration

Configurações do backend deverão ser agrupadas em estrutura própria.

---

## 19.1.2.27 BackendCapabilities

Cada backend deverá declarar explicitamente suas capacidades.

Exemplos:

* otimizações suportadas;
* arquiteturas suportadas;
* formatos de objeto;
* informações de depuração.

---

## 19.1.2.28 Descoberta de Capacidades

O restante do compilador deverá consultar capacidades através da Backend API.

Jamais através de verificações específicas do backend.

---

## 19.1.2.29 Inicialização

Toda implementação deverá possuir uma etapa explícita de inicialização.

---

## 19.1.2.30 Encerramento

Toda implementação deverá possuir uma etapa explícita de finalização.

---

## 19.1.2.31 Ciclo de Vida

O ciclo conceitual de um backend deverá seguir:

```text
Inicialização

↓

Configuração

↓

Criação do Módulo

↓

Geração das Funções

↓

Geração dos Dados

↓

Emissão

↓

Finalização
```

---

## 19.1.2.32 Isolamento

Cada backend deverá manter estado próprio.

Compartilhamento de estado global deverá ser evitado.

---

## 19.1.2.33 Reentrância

Sempre que tecnicamente possível, implementações deverão ser reentrantes.

---

## 19.1.2.34 Paralelização

A Backend API deverá permitir futura geração paralela de funções.

---

## 19.1.2.35 Compartilhamento

Infraestruturas comuns deverão permanecer fora da implementação concreta do backend.

---

## 19.1.2.36 Observabilidade

Todas as operações relevantes deverão poder ser instrumentadas.

---

## 19.1.2.37 Logging

A Backend API deverá utilizar a infraestrutura oficial de logging do compilador.

---

## 19.1.2.38 Benchmarks

Todos os backends deverão utilizar a mesma infraestrutura de benchmarking.

---

## 19.1.2.39 Testabilidade

Cada operação definida pela Backend API deverá poder ser testada independentemente.

---

## 19.1.2.40 Testes Diferenciais

A Backend API deverá permitir a execução da mesma suíte de testes sobre múltiplos backends.

---

## 19.1.2.41 Compatibilidade

Implementações diferentes deverão produzir comportamento funcionalmente equivalente.

---

## 19.1.2.42 Portabilidade

Nenhuma operação da Backend API deverá depender de uma arquitetura específica de processador.

---

## 19.1.2.43 Evolução

Novas funcionalidades deverão ser adicionadas preferencialmente através da evolução da Backend API.

---

## 19.1.2.44 Compatibilidade Retroativa

Sempre que possível, alterações deverão preservar compatibilidade com implementações existentes.

---

## 19.1.2.45 Extensibilidade

A arquitetura deverá permitir a introdução de novos tipos de backend sem alterações estruturais no frontend.

---

## 19.1.2.46 Neutralidade

A Backend API não deverá favorecer nenhuma implementação específica.

---

## 19.1.2.47 Backend Oficial

O Backend Cranelift constitui a implementação oficial inicial dessa interface.

---

## 19.1.2.48 Backend Futuro

O Backend LLVM deverá implementar exatamente os mesmos contratos definidos nesta seção.

---

## 19.1.2.49 Outros Backends

Implementações futuras poderão ser adicionadas desde que respeitem integralmente a Backend API.

---

## 19.1.2.50 Invariantes

Durante toda a evolução da arquitetura deverão permanecer verdadeiros os seguintes princípios:

* a Backend API constitui a única interface oficial entre o compilador e os backends;
* implementações concretas permanecem substituíveis;
* nenhuma tecnologia de geração de código influencia a semântica da linguagem;
* toda funcionalidade comum permanece centralizada na Backend API;
* todos os backends preservam exatamente os mesmos contratos arquiteturais.

---

## 19.1.2.51 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma Backend API que:

* desacople completamente o compilador das implementações concretas de geração de código;
* permita a coexistência de múltiplos backends;
* preserve a estabilidade da arquitetura ao longo da evolução do projeto;
* sirva como contrato único para integração entre a IR, o Runtime e os mecanismos de geração de código;
* assegure que a escolha do backend seja apenas um detalhe de implementação, sem impacto na semântica da linguagem ou no comportamento observável dos programas compilados.

---

## 19.1.3 Organização dos Componentes do Backend

### 19.1.3.1 Objetivo

Esta seção define a organização lógica dos componentes internos do Backend Cranelift.

Seu objetivo é estabelecer uma arquitetura modular, altamente coesa e de baixo acoplamento, permitindo que a implementação evolua continuamente sem comprometer a estabilidade do compilador.

A organização aqui descrita é conceitual e não impõe nomes específicos de módulos, diretórios ou arquivos.

---

## 19.1.3.2 Organização Modular

O Backend Cranelift deverá ser organizado em componentes independentes.

Cada componente deverá possuir uma responsabilidade arquitetural única e claramente definida.

---

## 19.1.3.3 Separação de Responsabilidades

Nenhum componente deverá acumular responsabilidades pertencentes a outro.

Cada responsabilidade deverá possuir um único ponto de implementação.

---

## 19.1.3.4 Organização Conceitual

A arquitetura poderá ser representada da seguinte forma:

```text
Backend API
      │
      ▼
Backend Context
      │
      ▼
┌──────────────────────────────────────┐
│ Lowering                             │
├──────────────────────────────────────┤
│ Function Generation                  │
├──────────────────────────────────────┤
│ Data Generation                      │
├──────────────────────────────────────┤
│ Symbol Generation                    │
├──────────────────────────────────────┤
│ Runtime Integration                  │
├──────────────────────────────────────┤
│ Object File Emission                 │
├──────────────────────────────────────┤
│ Diagnostics                          │
├──────────────────────────────────────┤
│ Statistics                           │
└──────────────────────────────────────┘
```

Essa representação é normativa apenas quanto às responsabilidades, não quanto à estrutura física do código.

---

## 19.1.3.5 Backend Context

O Backend Context constitui o núcleo operacional do backend.

Ele deverá coordenar todo o processo de geração de código.

---

## 19.1.3.6 Responsabilidades do Backend Context

Compete ao Backend Context:

* manter a configuração ativa;
* controlar o ciclo de vida da compilação;
* disponibilizar serviços comuns;
* compartilhar informações entre os componentes internos;
* controlar a geração de módulos.

---

## 19.1.3.7 Estado Compartilhado

Todo estado compartilhado deverá permanecer concentrado no Backend Context.

Componentes individuais deverão evitar estados globais.

---

## 19.1.3.8 Lowering

O componente de Lowering deverá ser responsável exclusivamente pela conversão entre:

* IR da Capi;
* representação intermediária utilizada pelo backend.

---

## 19.1.3.9 Responsabilidades do Lowering

Compete ao Lowering:

* converter tipos;
* converter operações;
* converter fluxo de controle;
* converter chamadas;
* converter constantes;
* converter referências.

---

## 19.1.3.10 Não Responsabilidades do Lowering

O Lowering não deverá:

* realizar análise semântica;
* modificar ownership;
* alterar lifetimes;
* alterar a IR;
* executar otimizações que modifiquem comportamento.

---

## 19.1.3.11 Function Generation

O componente de geração de funções deverá produzir o corpo executável de cada função presente na IR.

---

## 19.1.3.12 Escopo da Geração de Funções

Esse componente deverá ser responsável por:

* blocos básicos;
* instruções;
* chamadas;
* retornos;
* controle de fluxo;
* registradores virtuais.

---

## 19.1.3.13 Independência das Funções

Cada função deverá ser gerada de maneira independente.

Essa independência permitirá futura paralelização.

---

## 19.1.3.14 Data Generation

Constantes e demais estruturas estáticas deverão ser geradas por um componente específico.

---

## 19.1.3.15 Dados Estáticos

Entre os elementos gerados poderão estar:

* strings;
* tabelas;
* constantes;
* metadados;
* estruturas imutáveis.

---

## 19.1.3.16 Symbol Generation

Símbolos deverão ser produzidos por um componente dedicado.

---

## 19.1.3.17 Responsabilidades dos Símbolos

Compete ao componente de símbolos:

* definir nomes;
* definir linkage;
* definir visibilidade;
* registrar exportações;
* registrar importações.

---

## 19.1.3.18 Runtime Integration

Toda comunicação com o Runtime deverá permanecer centralizada em um componente específico.

---

## 19.1.3.19 Responsabilidades do Runtime Integration

Esse componente deverá materializar chamadas para:

* runtime helpers;
* gerenciamento de Domains;
* gerenciamento de ObjectId;
* inicialização do Runtime;
* finalização do Runtime.

---

## 19.1.3.20 Encapsulamento

Os demais componentes não deverão conhecer detalhes da implementação do Runtime.

---

## 19.1.3.21 Object File Emission

A emissão de arquivos objeto deverá ocorrer em um componente dedicado.

---

## 19.1.3.22 Responsabilidades do Emissor

Compete ao emissor:

* produzir arquivos objeto;
* organizar seções;
* emitir símbolos;
* emitir relocations;
* preparar o resultado para a etapa de linkedição.

---

## 19.1.3.23 Diagnósticos

Diagnósticos internos deverão permanecer concentrados em infraestrutura própria.

---

## 19.1.3.24 Estatísticas

A coleta de métricas deverá ocorrer através de um componente específico.

---

## 19.1.3.25 Configuração

Configurações do backend deverão permanecer agrupadas em uma estrutura única.

---

## 19.1.3.26 Componentes Auxiliares

Componentes auxiliares poderão existir para funções específicas como:

* caches;
* builders;
* verificadores;
* instrumentação;
* serialização;
* debug.

Esses componentes não deverão assumir responsabilidades arquiteturais pertencentes aos componentes principais.

---

## 19.1.3.27 Comunicação

A comunicação entre componentes deverá ocorrer através de interfaces explícitas.

Chamadas diretas a estados internos deverão ser evitadas.

---

## 19.1.3.28 Dependências

As dependências deverão formar um grafo acíclico.

Dependências circulares não deverão existir.

---

## 19.1.3.29 Direção das Dependências

As dependências deverão sempre apontar para componentes mais fundamentais.

Componentes de nível inferior jamais deverão depender de componentes de nível superior.

---

## 19.1.3.30 Coesão

Cada componente deverá possuir alta coesão funcional.

---

## 19.1.3.31 Acoplamento

O acoplamento entre componentes deverá permanecer mínimo.

---

## 19.1.3.32 Reutilização

Sempre que possível, componentes deverão ser reutilizáveis por outros backends.

---

## 19.1.3.33 Compartilhamento

Infraestruturas comuns deverão permanecer fora da implementação específica do Cranelift.

---

## 19.1.3.34 Especialização

Todo código específico do Cranelift deverá permanecer isolado dos componentes compartilháveis.

---

## 19.1.3.35 Observabilidade

Todos os componentes deverão poder registrar:

* tempo de execução;
* quantidade de operações;
* consumo de memória;
* estatísticas relevantes.

---

## 19.1.3.36 Instrumentação

A instrumentação deverá ser desacoplada da lógica principal de geração de código.

---

## 19.1.3.37 Testabilidade

Cada componente deverá poder ser testado isoladamente.

---

## 19.1.3.38 Testes de Integração

Além dos testes unitários, deverão existir testes cobrindo a integração entre componentes.

---

## 19.1.3.39 Evolução

Novos componentes poderão ser adicionados desde que preservem:

* modularidade;
* baixo acoplamento;
* interfaces existentes.

---

## 19.1.3.40 Substituição

Qualquer componente poderá ser reimplementado desde que preserve seu contrato arquitetural.

---

## 19.1.3.41 Independência Física

A organização em arquivos, diretórios e pacotes permanece uma decisão de implementação.

Esta especificação define apenas a organização lógica.

---

## 19.1.3.42 Compatibilidade com LLVM

A organização lógica aqui definida deverá ser reutilizada, sempre que possível, pela futura implementação do Backend LLVM.

---

## 19.1.3.43 Compatibilidade com Novos Backends

Backends futuros deverão poder reutilizar integral ou parcialmente esta arquitetura modular.

---

## 19.1.3.44 Invariantes

Durante toda a implementação deverão permanecer verdadeiros os seguintes princípios:

* cada componente possui responsabilidade única;
* o Backend Context centraliza o estado compartilhado;
* o Lowering constitui a fronteira entre a IR e a infraestrutura de geração de código;
* a integração com o Runtime permanece isolada;
* a emissão de arquivos objeto permanece desacoplada da geração das funções;
* diagnósticos, estatísticas e instrumentação permanecem independentes da lógica principal;
* componentes específicos do Cranelift permanecem encapsulados.

---

## 19.1.3.45 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma organização interna em que:

* os componentes sejam altamente coesos e fracamente acoplados;
* as responsabilidades estejam claramente delimitadas;
* a infraestrutura específica do Cranelift permaneça encapsulada;
* componentes compartilháveis possam ser reutilizados por outros backends;
* a evolução futura da arquitetura possa ocorrer de forma incremental, sem comprometer os contratos estabelecidos pela Backend API e pela arquitetura geral do compilador.

---

# 19.2 Lowering da IR para Cranelift IR

## 19.2.1 Objetivo

Esta seção define o processo de *lowering* entre a Intermediate Representation (IR) da linguagem Capi e a representação intermediária utilizada internamente pelo Cranelift.

O objetivo do *lowering* é traduzir a representação independente da linguagem para uma representação adequada à geração de código nativo, preservando integralmente a semântica da IR.

---

## 19.2.2 Definição

Para os fins desta especificação, *lowering* é o processo responsável por transformar estruturas da IR da Capi em construções equivalentes da infraestrutura de geração de código utilizada pelo backend.

---

## 19.2.3 Papel Arquitetural

O *lowering* constitui a fronteira entre:

* a arquitetura do compilador; e
* a infraestrutura de geração de código.

Após essa etapa, nenhuma estrutura específica da linguagem deverá permanecer dependente do frontend.

---

## 19.2.4 Independência

O processo de *lowering* deverá depender exclusivamente da:

* IR validada;
* Backend API;
* informações de configuração do backend.

Ele não deverá depender de componentes do frontend.

---

## 19.2.5 Fluxo Conceitual

O fluxo conceitual poderá ser representado da seguinte forma:

```text
Intermediate Representation
            │
            ▼
     Lowering da IR
            │
            ▼
    Cranelift Intermediate Representation
            │
            ▼
      Geração de Código
```

---

## 19.2.6 Entrada

A entrada do *lowering* deverá consistir exclusivamente de uma IR completamente validada.

---

## 19.2.7 Pré-condições

Antes do início do *lowering*, deverão estar concluídas todas as etapas de:

* resolução de nomes;
* inferência de tipos;
* validação semântica;
* verificação de ownership;
* verificação de lifetimes;
* construção da IR.

---

## 19.2.8 Pós-condições

Ao término do *lowering*, deverá existir uma representação intermediária completamente equivalente à IR original, apta para geração de código.

---

## 19.2.9 Ausência de Alteração Semântica

O *lowering* não deverá modificar:

* comportamento observável;
* tipos;
* identidade;
* lifetime;
* ordem de execução;
* regras de ownership.

---

## 19.2.10 Tradução Estrutural

Toda transformação realizada durante o *lowering* deverá possuir correspondência direta com elementos previamente existentes na IR.

Nenhuma construção nova da linguagem poderá surgir nesta etapa.

---

## 19.2.11 Preservação dos Contratos

Todos os contratos estabelecidos pela IR deverão permanecer válidos após o *lowering*.

---

## 19.2.12 Tradução Determinística

Dada a mesma IR e a mesma configuração do backend, o *lowering* deverá produzir uma representação funcionalmente equivalente em todas as execuções.

---

## 19.2.13 Organização

O *lowering* deverá ser organizado em transformações especializadas.

Exemplos incluem:

* tipos;
* funções;
* constantes;
* expressões;
* blocos;
* controle de fluxo.

---

## 19.2.14 Modularidade

Cada categoria de transformação deverá permanecer isolada das demais.

---

## 19.2.15 Ausência de Estado Global

O *lowering* não deverá depender de estados globais mutáveis.

---

## 19.2.16 Contexto

Toda operação deverá ocorrer dentro de um contexto próprio do backend.

---

## 19.2.17 Escopo

Cada função da IR deverá ser traduzida independentemente.

---

## 19.2.18 Isolamento

Falhas ocorridas durante o *lowering* de uma função não deverão comprometer a consistência das demais funções sempre que isso for tecnicamente possível.

---

## 19.2.19 Correspondência

Toda entidade da IR deverá possuir uma representação equivalente durante o *lowering*.

Essa equivalência deverá ser claramente verificável.

---

## 19.2.20 Representação Física

A representação produzida pelo *lowering* constitui exclusivamente um detalhe da implementação do backend.

Ela não integra a especificação da linguagem.

---

## 19.2.21 Neutralidade

Nenhuma decisão arquitetural da linguagem deverá depender da representação utilizada pelo Cranelift.

---

## 19.2.22 Evolução

A implementação do *lowering* poderá evoluir continuamente desde que preserve todos os contratos definidos pela IR.

---

## 19.2.23 Compartilhamento

Sempre que possível, partes do *lowering* independentes do Cranelift deverão ser reutilizáveis por outros backends.

---

## 19.2.24 Instrumentação

O processo de *lowering* deverá permitir instrumentação independente para coleta de:

* tempo de execução;
* quantidade de entidades traduzidas;
* estatísticas;
* diagnósticos.

---

## 19.2.25 Verificação

Ao término do *lowering*, deverão existir mecanismos capazes de verificar a consistência da representação produzida.

---

## 19.2.26 Diagnósticos

Falhas internas detectadas durante o *lowering* deverão produzir diagnósticos claros e determinísticos, permitindo identificar o elemento correspondente na IR.

---

## 19.2.27 Compatibilidade

A arquitetura definida nesta seção deverá ser igualmente aplicável ao futuro Backend LLVM.

---

## 19.2.28 Invariantes

Durante toda a execução do *lowering* deverão permanecer verdadeiros os seguintes princípios:

* a IR constitui a única entrada;
* nenhuma regra semântica é alterada;
* toda transformação possui correspondência verificável;
* o resultado permanece funcionalmente equivalente à IR original;
* detalhes específicos do Cranelift permanecem encapsulados.

---

## 19.2.29 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um processo de *lowering* que:

* consuma exclusivamente a IR validada;
* produza uma representação equivalente para geração de código;
* preserve integralmente a semântica da linguagem;
* permaneça desacoplado do frontend;
* constitua uma fronteira arquitetural clara entre a linguagem Capi e a infraestrutura de geração de código.

---

# 19.3 Tradução da IR para Cranelift IR

## 19.3.1 Objetivo

Esta seção define as regras normativas para a tradução da Intermediate Representation (IR) da linguagem Capi para a representação intermediária utilizada pelo Cranelift (*Cranelift IR*).

Seu objetivo é estabelecer um processo determinístico, modular e semanticamente equivalente, preservando integralmente os contratos definidos pela linguagem.

---

## 19.3.2 Papel da Tradução

A tradução constitui a principal responsabilidade do Backend Cranelift.

Ela converte construções independentes da arquitetura em uma representação adequada para geração de código nativo.

---

## 19.3.3 Escopo

A tradução compreende:

* funções;
* blocos básicos;
* parâmetros;
* valores;
* constantes;
* operações;
* fluxo de controle;
* chamadas;
* dados estáticos.

A geração efetiva de código de máquina permanece sob responsabilidade do Cranelift.

---

## 19.3.4 Fronteira Arquitetural

Após a conclusão desta etapa, todas as construções específicas da linguagem deverão possuir representação equivalente na Cranelift IR.

Nenhum conceito da IR original deverá permanecer sem materialização.

---

## 19.3.5 Natureza da Tradução

A tradução deverá ser estrutural.

Cada elemento da IR deverá corresponder a um ou mais elementos equivalentes na Cranelift IR.

---

## 19.3.6 Preservação Semântica

A tradução não deverá modificar:

* comportamento observável;
* ordem de execução;
* regras de ownership;
* lifetime;
* identidade;
* contratos da ABI.

---

## 19.3.7 Determinismo

Dada a mesma IR e a mesma configuração do backend, a tradução deverá produzir uma Cranelift IR funcionalmente equivalente em todas as execuções.

---

## 19.3.8 Organização

A tradução deverá ocorrer de maneira incremental.

Cada entidade deverá ser convertida individualmente antes da geração do código objeto.

---

## 19.3.9 Independência

Cada função deverá ser traduzida independentemente.

A tradução de uma função não deverá depender do estado interno da tradução de outra.

---

## 19.3.10 Ordem

A implementação poderá escolher qualquer ordem interna de tradução, desde que preserve integralmente as dependências representadas na IR.

---

## 19.3.11 Contexto

Toda tradução deverá ocorrer dentro de um Backend Context previamente inicializado.

---

## 19.3.12 Backend Module

Cada módulo deverá possuir um contexto próprio durante a tradução.

---

## 19.3.13 Backend Function

Cada função deverá possuir um contexto independente contendo todas as informações necessárias para sua tradução.

---

## 19.3.14 Tradução de Tipos

Os tipos presentes na IR deverão ser convertidos para tipos equivalentes suportados pelo backend.

Essa conversão não altera o sistema de tipos da linguagem.

---

## 19.3.15 Correspondência de Tipos

A representação física utilizada pelo backend poderá diferir da representação lógica da IR.

Entretanto, ambas deverão permanecer semanticamente equivalentes.

---

## 19.3.16 Tipos Compostos

Estruturas compostas deverão preservar:

* tamanho;
* alinhamento;
* offsets;
* layout definido pela ABI.

---

## 19.3.17 Tipos Primitivos

Tipos primitivos deverão ser traduzidos diretamente para representações compatíveis com a arquitetura de destino.

---

## 19.3.18 Tipos por Identidade

Tipos por identidade deverão manter integralmente o modelo definido no Capítulo 18.

---

## 19.3.19 Domains

Domains deverão ser materializados através das interfaces do Runtime.

O processo de tradução não deverá redefinir seu comportamento.

---

## 19.3.20 ObjectId

Operações envolvendo ObjectId deverão ser traduzidas preservando integralmente os contratos estabelecidos anteriormente.

---

## 19.3.21 Tradução de Constantes

Constantes deverão ser convertidas para representações estáticas sempre que possível.

---

## 19.3.22 Tradução de Variáveis

Variáveis locais deverão ser convertidas para valores temporários ou locais apropriados à infraestrutura do backend.

---

## 19.3.23 Tradução de Parâmetros

Parâmetros deverão obedecer rigorosamente às convenções definidas pela ABI.

---

## 19.3.24 Tradução de Retornos

Valores de retorno deverão respeitar integralmente as convenções estabelecidas para chamadas de função.

---

## 19.3.25 Tradução de Blocos

Cada bloco básico da IR deverá corresponder a um bloco equivalente na Cranelift IR.

---

## 19.3.26 Fluxo de Controle

Estruturas de controle deverão preservar exatamente o fluxo representado na IR.

---

## 19.3.27 Saltos

Saltos condicionais e incondicionais deverão manter equivalência funcional.

---

## 19.3.28 Chamadas

Chamadas de função deverão preservar:

* parâmetros;
* retorno;
* convenção de chamada;
* efeitos observáveis.

---

## 19.3.29 Chamadas Virtuais

Chamadas virtuais deverão ser materializadas conforme o modelo de despacho definido no Capítulo 18.

---

## 19.3.30 Chamadas ao Runtime

Chamadas ao Runtime deverão utilizar exclusivamente suas interfaces públicas.

---

## 19.3.31 Dados Estáticos

Dados estáticos deverão ser traduzidos antes da emissão final do módulo.

---

## 19.3.32 Símbolos

Símbolos deverão ser produzidos preservando:

* visibilidade;
* linkage;
* escopo;
* compatibilidade com o linker.

---

## 19.3.33 Metadata

Metadados utilizados durante a tradução não deverão alterar a semântica do programa.

---

## 19.3.34 Tradução Incremental

Sempre que possível, cada entidade deverá ser traduzida exatamente uma vez.

---

## 19.3.35 Reutilização

Informações já traduzidas poderão ser reutilizadas desde que sua validade permaneça garantida.

---

## 19.3.36 Caches

Caches internos poderão ser utilizados para reduzir trabalho redundante.

Sua utilização não poderá alterar o comportamento funcional.

---

## 19.3.37 Instrumentação

A tradução deverá permitir coleta independente de métricas.

---

## 19.3.38 Estatísticas

Entre as estatísticas produzidas poderão estar:

* funções traduzidas;
* blocos produzidos;
* instruções geradas;
* constantes materializadas;
* símbolos emitidos.

---

## 19.3.39 Diagnósticos

Toda falha ocorrida durante a tradução deverá indicar, sempre que possível, o elemento correspondente na IR.

---

## 19.3.40 Verificação

Ao término da tradução de cada função, deverá existir uma etapa de verificação da consistência estrutural da representação produzida.

---

## 19.3.41 Recuperação

Sempre que tecnicamente possível, falhas localizadas deverão permitir a continuidade da tradução das demais unidades.

---

## 19.3.42 Otimizações

Pequenas otimizações locais poderão ocorrer durante a tradução, desde que:

* não modifiquem a semântica;
* não alterem a ABI;
* permaneçam independentes da arquitetura.

---

## 19.3.43 Não Responsabilidades

Esta etapa não deverá:

* redefinir a IR;
* modificar o Runtime;
* alterar layouts;
* alterar a semântica da linguagem.

---

## 19.3.44 Compatibilidade

A arquitetura desta etapa deverá ser suficientemente genérica para permitir implementação equivalente no Backend LLVM.

---

## 19.3.45 Invariantes

Durante toda a tradução deverão permanecer verdadeiros os seguintes princípios:

* cada entidade da IR possui representação correspondente;
* nenhuma transformação altera a semântica;
* a tradução preserva integralmente os contratos da ABI;
* o Runtime permanece acessado apenas por interfaces públicas;
* a tradução é independente da tecnologia de geração de código.

---

## 19.3.46 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um processo de tradução que:

* converta integralmente a IR da Capi para a Cranelift IR;
* preserve todos os contratos definidos pela linguagem;
* mantenha independência entre frontend, Runtime e backend;
* permita instrumentação, verificação e testes independentes;
* constitua uma base reutilizável para a futura implementação do Backend LLVM.

---

# 20. Backend LLVM

## 20.1 Arquitetura do Backend LLVM

### 20.1.1 Objetivo

Esta seção define a arquitetura do Backend LLVM da implementação oficial da linguagem Capi.

Seu objetivo é estabelecer os princípios arquiteturais, responsabilidades e critérios de integração do backend baseado em LLVM, preservando integralmente a arquitetura definida para o compilador e garantindo equivalência funcional em relação ao Backend Cranelift.

---

## 20.1.2 Papel Arquitetural

O Backend LLVM constitui uma implementação da Backend API responsável por transformar a Intermediate Representation (IR) da linguagem Capi em código objeto utilizando a infraestrutura do projeto LLVM.

---

## 20.1.3 Papel no Projeto

O Backend LLVM representa a implementação de geração de código voltada para:

* maior qualidade de otimização;
* maior diversidade de arquiteturas suportadas;
* integração com o ecossistema LLVM;
* produção de binários destinados a ambientes de produção.

---

## 20.1.4 Independência

O Backend LLVM não altera a arquitetura do compilador.

Ele substitui exclusivamente a implementação concreta da geração de código.

---

## 20.1.5 Equivalência

O Backend LLVM deverá preservar exatamente os mesmos contratos implementados pelo Backend Cranelift.

Diferenças deverão restringir-se a aspectos internos da geração de código.

---

## 20.1.6 Compatibilidade

Todo programa compilado pelo Backend LLVM deverá apresentar comportamento observável equivalente ao produzido pelo Backend Cranelift.

---

## 20.1.7 Arquitetura Geral

Conceitualmente, a arquitetura poderá ser representada da seguinte forma:

```text
Frontend

↓

HIR

↓

IR

↓

Backend API

↓

Backend LLVM

↓

LLVM IR

↓

LLVM Pass Manager

↓

Código Objeto

↓

Linkedição

↓

Executável
```

---

## 20.1.8 Backend API

O Backend LLVM deverá implementar integralmente a Backend API definida no Capítulo 19.

Nenhuma extensão específica do LLVM deverá modificar essa interface.

---

## 20.1.9 Backend Concreto

O LLVM constitui apenas uma implementação concreta da Backend API.

Ele não representa um novo modelo arquitetural para o compilador.

---

## 20.1.10 Fronteira Arquitetural

A Backend API permanece sendo a única fronteira oficial entre:

* frontend;
* Runtime;
* geração de código.

---

## 20.1.11 Objetivos

São objetivos do Backend LLVM:

* maximizar qualidade do código gerado;
* reutilizar a infraestrutura LLVM;
* ampliar o suporte multiplataforma;
* disponibilizar otimizações avançadas;
* manter compatibilidade integral com a linguagem.

---

## 20.1.12 Não Objetivos

O Backend LLVM não possui como objetivo:

* redefinir a semântica da linguagem;
* alterar a ABI;
* modificar o Runtime;
* alterar o modelo de objetos;
* substituir a IR.

---

## 20.1.13 Evolução

A introdução do Backend LLVM deverá ocorrer como evolução natural da implementação oficial.

Nenhum documento da especificação deverá precisar ser alterado para acomodar essa mudança.

---

## 20.1.14 Reutilização

Sempre que possível, o Backend LLVM deverá reutilizar integralmente a infraestrutura desenvolvida para o Backend Cranelift.

---

## 20.1.15 Componentes Compartilhados

Deverão permanecer compartilhados:

* Backend API;
* IR;
* HIR;
* frontend;
* sistema de diagnósticos;
* infraestrutura de testes;
* benchmarking;
* Runtime.

---

## 20.1.16 Componentes Específicos

São específicos do Backend LLVM:

* geração da LLVM IR;
* integração com LLVM Pass Manager;
* geração de código LLVM;
* emissão de artefatos através da infraestrutura LLVM.

---

## 20.1.17 Organização Modular

A arquitetura deverá permanecer modular.

Componentes específicos do LLVM deverão permanecer isolados da infraestrutura compartilhada.

---

## 20.1.18 Baixo Acoplamento

Nenhum componente compartilhado deverá depender de APIs específicas do LLVM.

---

## 20.1.19 Encapsulamento

Todo acesso às APIs do LLVM deverá permanecer encapsulado dentro do Backend LLVM.

---

## 20.1.20 Neutralidade

A arquitetura do compilador deverá permanecer neutra quanto à tecnologia de geração de código utilizada.

---

## 20.1.21 LLVM IR

A LLVM IR constitui exclusivamente uma representação intermediária da implementação.

Ela não faz parte da especificação da linguagem.

---

## 20.1.22 Independência da Linguagem

Conceitos específicos da linguagem Capi deverão ser completamente resolvidos antes da geração da LLVM IR.

---

## 20.1.23 Lowering

A tradução da IR da linguagem para LLVM IR deverá ocorrer em etapa própria de *lowering*.

---

## 20.1.24 Responsabilidade do Lowering

O *lowering* deverá preservar integralmente:

* semântica;
* ownership;
* lifetime;
* Domains;
* ObjectId;
* ABI.

---

## 20.1.25 Runtime

O Backend LLVM deverá interagir com o Runtime exclusivamente através das interfaces públicas definidas pelo Documento 09.

---

## 20.1.26 ABI

Todas as chamadas produzidas pelo Backend LLVM deverão obedecer rigorosamente à ABI oficial da linguagem.

---

## 20.1.27 Diagnósticos

O Backend LLVM deverá utilizar a infraestrutura oficial de diagnósticos do compilador.

---

## 20.1.28 Instrumentação

A arquitetura deverá permitir instrumentação equivalente à existente no Backend Cranelift.

---

## 20.1.29 Benchmarks

Benchmarks deverão utilizar exatamente a mesma infraestrutura empregada pelos demais backends.

---

## 20.1.30 Testabilidade

Cada componente do Backend LLVM deverá poder ser testado isoladamente.

---

## 20.1.31 Testes Diferenciais

Toda alteração relevante deverá ser validada por testes diferenciais entre:

* Cranelift;
* LLVM.

---

## 20.1.32 Equivalência Funcional

Diferenças entre backends deverão restringir-se a:

* desempenho;
* tamanho do binário;
* qualidade das otimizações;
* tempo de compilação.

Jamais ao comportamento observável.

---

## 20.1.33 Evolução Incremental

A implementação do Backend LLVM deverá evoluir incrementalmente.

O Backend Cranelift permanecerá disponível durante todo o processo.

---

## 20.1.34 Compatibilidade com Bootstrap

Durante o bootstrap, o Backend LLVM poderá permanecer incompleto.

A implementação oficial continuará utilizando o Backend Cranelift até que os critérios definidos neste documento sejam atendidos.

---

## 20.1.35 Critério de Introdução

O Backend LLVM somente poderá ser considerado oficialmente suportado quando implementar integralmente a Backend API.

---

## 20.1.36 Critério de Produção

O Backend LLVM somente poderá ser recomendado para uso em produção quando demonstrar equivalência funcional em toda a suíte oficial de testes.

---

## 20.1.37 Compatibilidade Futura

A arquitetura deverá permitir a introdução de novas versões do LLVM sem necessidade de alterações estruturais no compilador.

---

## 20.1.38 Independência de Versão

O compilador não deverá depender de detalhes acidentais de versões específicas do LLVM.

Sempre que possível, a integração deverá utilizar APIs estáveis.

---

## 20.1.39 Invariantes

Durante toda a evolução do Backend LLVM deverão permanecer verdadeiros os seguintes princípios:

* a Backend API constitui a única interface oficial entre o compilador e o backend;
* a LLVM IR representa exclusivamente um detalhe de implementação;
* todos os contratos semânticos permanecem definidos pela IR da linguagem;
* o Runtime permanece independente do backend;
* a ABI permanece única para todos os backends;
* diferenças entre backends jamais alteram o comportamento observável dos programas.

---

## 20.1.40 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um Backend LLVM que:

* implemente integralmente a Backend API;
* permaneça completamente desacoplado do frontend e do Runtime;
* reutilize a infraestrutura compartilhada do compilador;
* produza código semanticamente equivalente ao Backend Cranelift;
* esteja preparado para suportar otimizações avançadas e múltiplas arquiteturas sem comprometer os contratos definidos pela especificação da linguagem.

---

# 20.2 Reutilização da Infraestrutura Compartilhada

## 20.2.1 Objetivo

Esta seção estabelece os princípios e critérios para reutilização da infraestrutura já existente do compilador pela implementação do Backend LLVM.

Seu objetivo é minimizar duplicação de código, preservar a consistência arquitetural e assegurar que diferentes implementações de backend compartilhem o maior número possível de componentes comuns.

---

## 20.2.2 Princípio da Reutilização

O Backend LLVM deverá reutilizar toda infraestrutura independente da tecnologia de geração de código.

Componentes específicos do LLVM somente deverão ser implementados quando não existir infraestrutura compartilhável.

---

## 20.2.3 Objetivos

A reutilização da infraestrutura possui os seguintes objetivos:

* reduzir duplicação;
* simplificar manutenção;
* facilitar testes;
* preservar consistência arquitetural;
* acelerar evolução do compilador.

---

## 20.2.4 Arquitetura Compartilhada

Conceitualmente, a arquitetura deverá seguir o seguinte modelo:

```text
                    Frontend
                        │
                        ▼
                       HIR
                        │
                        ▼
                        IR
                        │
                        ▼
                  Backend API
              ┌─────────┴─────────┐
              ▼                   ▼
      Backend Cranelift     Backend LLVM
              │                   │
              └─────────┬─────────┘
                        ▼
             Runtime Compartilhado
```

A infraestrutura localizada acima da Backend API deverá permanecer completamente compartilhada.

---

## 20.2.5 Frontend

O Backend LLVM deverá reutilizar integralmente o frontend oficial do compilador.

Não deverão existir versões específicas do frontend para cada backend.

---

## 20.2.6 Lexer

O analisador léxico deverá permanecer completamente independente do backend utilizado.

---

## 20.2.7 Parser

O parser oficial deverá produzir exatamente a mesma AST independentemente do backend selecionado.

---

## 20.2.8 AST

A Abstract Syntax Tree constitui infraestrutura compartilhada.

Nenhum backend poderá modificá-la.

---

## 20.2.9 HIR

A High-Level Intermediate Representation deverá permanecer comum a todos os backends.

---

## 20.2.10 Sistema de Tipos

O sistema de tipos deverá permanecer completamente compartilhado.

Nenhuma regra específica do LLVM poderá alterar sua implementação.

---

## 20.2.11 Resolução de Nomes

Toda infraestrutura de resolução de nomes deverá ser reutilizada integralmente.

---

## 20.2.12 Verificação Semântica

A validação semântica deverá ocorrer antes da seleção do backend.

O Backend LLVM não deverá repetir essas verificações.

---

## 20.2.13 Ownership

As verificações de ownership deverão permanecer exclusivas do frontend.

---

## 20.2.14 Lifetimes

A resolução de lifetimes deverá estar concluída antes da geração de código.

---

## 20.2.15 Intermediate Representation

A IR constitui a principal infraestrutura compartilhada entre todos os backends.

Nenhum backend deverá produzir uma IR própria da linguagem.

---

## 20.2.16 Neutralidade da IR

A IR não deverá conter construções específicas do LLVM ou do Cranelift.

---

## 20.2.17 Backend API

Toda comunicação entre a IR e os backends deverá ocorrer exclusivamente através da Backend API.

---

## 20.2.18 Diagnósticos

A infraestrutura de diagnósticos deverá permanecer única para todo o compilador.

Backends apenas produzirão informações adicionais quando necessário.

---

## 20.2.19 Logging

O sistema oficial de logging deverá ser compartilhado.

---

## 20.2.20 Estatísticas

A coleta de estatísticas deverá utilizar infraestrutura comum.

---

## 20.2.21 Instrumentação

A instrumentação deverá permanecer independente do backend.

Sempre que possível, deverá produzir métricas comparáveis entre diferentes implementações.

---

## 20.2.22 Benchmarks

Todos os backends deverão utilizar exatamente a mesma infraestrutura de benchmarking.

---

## 20.2.23 Testes

A suíte oficial de testes deverá ser compartilhada.

Um mesmo teste deverá poder ser executado utilizando qualquer backend suportado.

---

## 20.2.24 Testes Diferenciais

A infraestrutura de testes diferenciais deverá permanecer compartilhada.

Ela deverá permitir comparar automaticamente:

* comportamento;
* código produzido;
* desempenho;
* diagnósticos.

---

## 20.2.25 Runtime

O Runtime oficial constitui infraestrutura compartilhada.

O Backend LLVM não deverá possuir uma implementação própria do Runtime.

---

## 20.2.26 Runtime Helpers

Chamadas para *runtime helpers* deverão reutilizar exatamente os mesmos contratos públicos definidos para o Backend Cranelift.

---

## 20.2.27 ABI

A ABI deverá permanecer única.

Nenhum backend poderá introduzir convenções próprias.

---

## 20.2.28 Modelo de Objetos

O modelo de objetos definido no Capítulo 18 deverá permanecer integralmente compartilhado.

---

## 20.2.29 Domains

A implementação de Domains pertence ao Runtime.

O Backend LLVM deverá apenas materializar chamadas compatíveis com essa implementação.

---

## 20.2.30 ObjectId

Toda infraestrutura relacionada ao ObjectId deverá permanecer compartilhada.

---

## 20.2.31 Layout

Layouts deverão ser definidos exclusivamente pela ABI e pelo modelo de objetos.

Nenhum backend poderá alterar offsets, alinhamentos ou organização física.

---

## 20.2.32 Metadata

A infraestrutura de metadados deverá permanecer compartilhada.

---

## 20.2.33 Verificadores

Todos os verificadores arquiteturais deverão permanecer independentes do backend.

---

## 20.2.34 Dumps

Ferramentas de inspeção da IR deverão funcionar independentemente do backend selecionado.

---

## 20.2.35 Configuração

Configurações comuns deverão permanecer centralizadas.

Cada backend deverá possuir apenas configurações específicas de geração de código.

---

## 20.2.36 Compartilhamento de Utilidades

Bibliotecas auxiliares utilizadas pelo compilador deverão permanecer compartilhadas sempre que não dependerem diretamente da infraestrutura LLVM.

---

## 20.2.37 Compartilhamento do Lowering

Sempre que tecnicamente viável, etapas de *lowering* independentes da representação intermediária específica do backend deverão ser compartilhadas.

A arquitetura deverá favorecer um modelo em duas fases:

1. normalização da IR para uma representação intermediária comum ao backend;
2. adaptação dessa representação para a infraestrutura específica do Cranelift ou do LLVM.

Essa organização reduz duplicação de lógica e aumenta a consistência entre backends.

---

## 20.2.38 Compartilhamento de Otimizações

Otimizações independentes da tecnologia de geração de código deverão ocorrer antes da Backend API.

Assim, seus benefícios serão automaticamente aproveitados por todos os backends.

---

## 20.2.39 Componentes Exclusivos

Deverão permanecer exclusivos do Backend LLVM apenas os componentes diretamente relacionados a:

* LLVM IR;
* LLVM Pass Manager;
* APIs do LLVM;
* emissão utilizando LLVM;
* integração específica com a infraestrutura LLVM.

---

## 20.2.40 Componentes Não Compartilháveis

Não se espera compartilhamento dos seguintes elementos:

* estruturas internas do LLVM;
* tipos específicos da LLVM IR;
* passes próprios do LLVM;
* APIs de geração de código do LLVM.

---

## 20.2.41 Evolução

Sempre que uma funcionalidade inicialmente implementada em apenas um backend puder tornar-se genérica, ela deverá ser promovida para a infraestrutura compartilhada.

---

## 20.2.42 Revisão Arquitetural

Toda duplicação significativa entre o Backend Cranelift e o Backend LLVM deverá motivar uma revisão arquitetural para avaliar a extração de componentes compartilhados.

---

## 20.2.43 Critério de Compartilhamento

Um componente deverá ser considerado compartilhável quando:

* representar conceito da linguagem;
* não depender de APIs específicas do backend;
* puder beneficiar múltiplas implementações.

---

## 20.2.44 Critério de Especialização

Um componente deverá permanecer específico do backend quando sua existência decorrer exclusivamente das características da infraestrutura utilizada para geração de código.

---

## 20.2.45 Invariantes

Durante toda a evolução da arquitetura deverão permanecer verdadeiros os seguintes princípios:

* existe apenas um frontend oficial;
* existe apenas uma IR oficial;
* existe apenas um Runtime oficial;
* existe apenas uma ABI oficial;
* existe apenas um modelo de objetos oficial;
* diferenças entre backends restringem-se exclusivamente à implementação da geração de código;
* toda infraestrutura compartilhável deverá permanecer fora dos backends concretos.

---

## 20.2.46 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma arquitetura em que:

* Frontend, HIR, IR, Runtime, ABI, sistema de tipos, diagnósticos e infraestrutura de testes sejam integralmente compartilhados;
* o Backend LLVM reutilize toda a infraestrutura independente da tecnologia de geração de código;
* componentes específicos permaneçam estritamente limitados à integração com o LLVM;
* a evolução de um backend beneficie, sempre que possível, os demais por meio da infraestrutura compartilhada;
* a duplicação de código seja tratada como exceção arquitetural, e não como estratégia de implementação.

---

# 20.3 Lowering da IR para LLVM IR

## 20.3.1 Objetivo

Esta seção define o processo de *lowering* entre a Intermediate Representation (IR) da linguagem Capi e a LLVM Intermediate Representation (LLVM IR).

Seu objetivo é estabelecer uma tradução determinística, modular e semanticamente equivalente, preservando integralmente os contratos definidos pela linguagem e preparando o programa para a infraestrutura de otimização e geração de código do LLVM.

---

## 20.3.2 Definição

Para os fins desta especificação, *lowering* corresponde ao processo responsável por transformar a IR da linguagem Capi em uma representação equivalente utilizando exclusivamente construções da LLVM IR.

---

## 20.3.3 Papel Arquitetural

O *lowering* constitui a fronteira entre a arquitetura da linguagem e a infraestrutura LLVM.

Após sua conclusão, toda a semântica específica da linguagem deverá estar completamente representada na LLVM IR.

---

## 20.3.4 Escopo

O processo de *lowering* compreende a tradução de:

* módulos;
* funções;
* parâmetros;
* tipos;
* constantes;
* variáveis;
* blocos básicos;
* fluxo de controle;
* chamadas;
* dados estáticos.

---

## 20.3.5 Independência

O *lowering* deverá depender exclusivamente de:

* IR validada;
* Backend API;
* configuração do backend.

Nenhuma dependência direta do frontend deverá existir.

---

## 20.3.6 Entrada

A entrada do *lowering* deverá ser exclusivamente uma IR completamente validada.

---

## 20.3.7 Pré-condições

Antes do início do *lowering*, deverão estar concluídas:

* resolução de nomes;
* inferência de tipos;
* verificação semântica;
* validação de ownership;
* validação de lifetimes;
* construção da IR.

---

## 20.3.8 Pós-condições

Ao término do processo deverá existir uma LLVM IR completamente consistente, apta para execução das etapas de otimização e geração de código.

---

## 20.3.9 Preservação Semântica

O *lowering* não poderá alterar:

* comportamento observável;
* sistema de tipos;
* ownership;
* lifetimes;
* Domains;
* ObjectId;
* identidade dos objetos;
* contratos da ABI.

---

## 20.3.10 Neutralidade

A LLVM IR representa exclusivamente uma implementação física da IR da linguagem.

Ela não constitui parte da especificação da Capi.

---

## 20.3.11 Correspondência Estrutural

Toda entidade existente na IR deverá possuir representação equivalente na LLVM IR.

---

## 20.3.12 Tradução Determinística

Dada a mesma IR e a mesma configuração do backend, o *lowering* deverá produzir uma LLVM IR funcionalmente equivalente em todas as execuções.

---

## 20.3.13 Organização

O *lowering* deverá ocorrer de maneira incremental.

Cada entidade deverá ser traduzida exatamente uma vez, sempre que tecnicamente possível.

---

## 20.3.14 Contexto

Toda tradução deverá ocorrer dentro de um contexto próprio do Backend LLVM.

---

## 20.3.15 Módulos

Cada módulo da IR deverá corresponder a um módulo equivalente na LLVM IR.

---

## 20.3.16 Funções

Cada função deverá ser traduzida individualmente.

A tradução de uma função não deverá depender do estado interno de outra função.

---

## 20.3.17 Tipos

Todos os tipos definidos na IR deverão ser convertidos para representações compatíveis com a LLVM IR.

Essa conversão não altera o sistema de tipos da linguagem.

---

## 20.3.18 Tipos Compostos

Estruturas compostas deverão preservar integralmente:

* layout;
* alinhamento;
* tamanho;
* offsets;
* contratos da ABI.

---

## 20.3.19 Tipos por Identidade

Objetos por identidade deverão preservar integralmente o modelo definido no Capítulo 18.

---

## 20.3.20 Constantes

Constantes deverão ser materializadas utilizando mecanismos apropriados da LLVM IR.

---

## 20.3.21 Variáveis

Variáveis locais deverão ser representadas utilizando a infraestrutura apropriada da LLVM IR.

---

## 20.3.22 Parâmetros

Parâmetros deverão obedecer rigorosamente às convenções da ABI oficial.

---

## 20.3.23 Valores de Retorno

Valores retornados deverão preservar exatamente os contratos definidos pela ABI.

---

## 20.3.24 Blocos Básicos

Cada bloco básico presente na IR deverá possuir representação equivalente na LLVM IR.

---

## 20.3.25 Fluxo de Controle

Estruturas condicionais, laços e desvios deverão preservar integralmente o fluxo representado na IR.

---

## 20.3.26 Chamadas

Chamadas de função deverão preservar:

* parâmetros;
* convenção de chamada;
* retorno;
* efeitos observáveis.

---

## 20.3.27 Chamadas Virtuais

Chamadas virtuais deverão utilizar exatamente o modelo de despacho definido para a linguagem.

O Backend LLVM não deverá introduzir mecanismos alternativos de despacho.

---

## 20.3.28 Runtime

Toda interação com o Runtime deverá ocorrer exclusivamente através das interfaces públicas definidas na especificação.

---

## 20.3.29 Domains

Domains deverão ser materializados através das chamadas oficiais ao Runtime.

O Backend LLVM não deverá redefinir seu funcionamento.

---

## 20.3.30 ObjectId

Todas as operações envolvendo ObjectId deverão preservar integralmente os contratos definidos pela linguagem.

---

## 20.3.31 Dados Estáticos

Dados estáticos deverão ser traduzidos antes da emissão final do módulo LLVM.

---

## 20.3.32 Símbolos

Símbolos deverão preservar:

* visibilidade;
* linkage;
* escopo;
* compatibilidade com o linker.

---

## 20.3.33 Metadados

Metadados utilizados durante o *lowering* não poderão alterar o comportamento do programa.

---

## 20.3.34 Preparação para Otimizações

A LLVM IR produzida deverá estar preparada para execução das *passes* de otimização.

Entretanto, o *lowering* não deverá depender de qualquer otimização para preservar a semântica da linguagem.

---

## 20.3.35 Independência das Passes

Toda LLVM IR produzida deverá permanecer semanticamente correta mesmo quando nenhuma *pass* de otimização for executada.

---

## 20.3.36 Reutilização

Sempre que possível, partes do *lowering* independentes da LLVM IR deverão reutilizar componentes compartilhados definidos na Backend API.

---

## 20.3.37 Instrumentação

O processo deverá permitir instrumentação independente para coleta de:

* tempo de tradução;
* número de funções;
* número de blocos;
* número de instruções;
* estatísticas gerais.

---

## 20.3.38 Diagnósticos

Falhas ocorridas durante o *lowering* deverão produzir diagnósticos claros e determinísticos, permitindo identificar a entidade correspondente na IR.

---

## 20.3.39 Verificação

Ao término da tradução de cada função deverá existir uma etapa de verificação estrutural da LLVM IR produzida.

Sempre que possível, deverá ser utilizada a infraestrutura oficial de verificação disponibilizada pelo LLVM.

---

## 20.3.40 Recuperação

Sempre que tecnicamente viável, falhas localizadas deverão permitir a continuidade da tradução das demais unidades de compilação.

---

## 20.3.41 Não Responsabilidades

Esta etapa não deverá:

* redefinir a IR;
* alterar a semântica da linguagem;
* modificar a ABI;
* redefinir o Runtime;
* alterar o modelo de objetos;
* introduzir otimizações dependentes da linguagem.

---

## 20.3.42 Compatibilidade

A arquitetura definida nesta seção deverá permanecer conceitualmente equivalente ao processo de *lowering* do Backend Cranelift, diferenciando-se apenas pela representação intermediária produzida.

---

## 20.3.43 Invariantes

Durante todo o processo de *lowering* deverão permanecer verdadeiros os seguintes princípios:

* a IR constitui a única entrada da tradução;
* toda entidade da IR possui representação equivalente na LLVM IR;
* nenhuma transformação altera a semântica da linguagem;
* o Runtime permanece acessado exclusivamente através de suas interfaces públicas;
* a LLVM IR representa apenas um detalhe de implementação do backend;
* o resultado produzido permanece funcionalmente equivalente ao obtido pelo Backend Cranelift.

---

## 20.3.44 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um processo de *lowering* que:

* traduza integralmente a IR da linguagem para LLVM IR;
* preserve todos os contratos definidos pela especificação da Capi;
* mantenha independência entre frontend, Runtime e backend;
* permita instrumentação, verificação e testes independentes;
* produza uma LLVM IR semanticamente equivalente à gerada pelo Backend Cranelift e apta para utilização da infraestrutura de otimização e geração de código do LLVM.

---

# 20.4 Otimizações

## 20.4.1 Objetivo

Esta seção define os princípios arquiteturais para utilização da infraestrutura de otimização do LLVM pela implementação oficial da linguagem Capi.

Seu objetivo é estabelecer como as otimizações deverão ser empregadas para melhorar desempenho, reduzir consumo de recursos e aumentar a qualidade do código gerado, preservando integralmente a semântica da linguagem.

---

## 20.4.2 Papel das Otimizações

As otimizações constituem uma etapa posterior ao *lowering* da IR da linguagem para LLVM IR.

Seu papel consiste exclusivamente em melhorar a implementação física do programa.

---

## 20.4.3 Não Responsabilidade

As otimizações não deverão alterar:

* semântica da linguagem;
* contratos da ABI;
* modelo de objetos;
* ownership;
* lifetimes;
* Domains;
* ObjectId.

---

## 20.4.4 Independência

A execução correta do programa não poderá depender da aplicação de qualquer otimização.

Todo programa deverá permanecer semanticamente válido mesmo com todas as otimizações desabilitadas.

---

## 20.4.5 Arquitetura

Conceitualmente, o fluxo poderá ser representado da seguinte forma:

```text id="guk1an"
IR da Capi
      │
      ▼
 Lowering
      │
      ▼
 LLVM IR
      │
      ▼
Pass Manager
      │
      ▼
LLVM IR Otimizada
      │
      ▼
Geração de Código
```

---

## 20.4.6 Infraestrutura

As otimizações deverão utilizar exclusivamente a infraestrutura oficial disponibilizada pelo LLVM.

---

## 20.4.7 Pass Manager

A execução das otimizações deverá ocorrer através do LLVM Pass Manager ou de sua evolução oficial.

---

## 20.4.8 Organização

As otimizações deverão ser organizadas como uma sequência ordenada de *passes*.

---

## 20.4.9 Modularidade

Cada *pass* deverá possuir responsabilidade claramente definida.

---

## 20.4.10 Determinismo

Dada a mesma LLVM IR e a mesma configuração do compilador, o conjunto de otimizações deverá produzir resultados funcionalmente equivalentes.

---

## 20.4.11 Configuração

O compilador deverá permitir diferentes níveis de otimização.

Exemplos incluem:

* sem otimização;
* otimização moderada;
* otimização agressiva.

A nomenclatura específica constitui detalhe da implementação.

---

## 20.4.12 Seleção

A seleção das otimizações deverá ocorrer antes da execução do Pass Manager.

---

## 20.4.13 Dependências

Uma *pass* poderá depender de resultados produzidos por outra *pass*, desde que essa dependência permaneça explicitamente definida.

---

## 20.4.14 Reordenação

A implementação poderá reorganizar a ordem das *passes* quando isso não alterar sua correção.

---

## 20.4.15 Eliminação de Código Morto

A infraestrutura poderá eliminar código comprovadamente inalcançável ou sem efeitos observáveis.

---

## 20.4.16 Propagação de Constantes

Valores constantes poderão ser propagados sempre que preservarem integralmente a semântica do programa.

---

## 20.4.17 Dobramento de Constantes

Expressões constantes poderão ser avaliadas antecipadamente.

---

## 20.4.18 Simplificação de Fluxo

Estruturas de controle poderão ser simplificadas quando houver equivalência funcional comprovada.

---

## 20.4.19 Simplificação de Expressões

Expressões redundantes poderão ser reduzidas para formas equivalentes.

---

## 20.4.20 Eliminação de Redundâncias

Operações redundantes poderão ser removidas sempre que não produzirem efeitos observáveis.

---

## 20.4.21 Inlining

Chamadas de função poderão ser substituídas por seu corpo quando a infraestrutura LLVM determinar que essa transformação seja vantajosa.

---

## 20.4.22 Critérios para Inlining

A decisão de realizar *inlining* deverá considerar fatores como:

* tamanho da função;
* frequência estimada de execução;
* impacto no tamanho do código;
* custo de chamada.

Os critérios específicos pertencem à implementação do LLVM.

---

## 20.4.23 Devirtualização

Chamadas virtuais poderão ser convertidas em chamadas diretas quando o tipo concreto puder ser determinado com segurança.

---

## 20.4.24 Escape Analysis

Análises de escape disponibilizadas pelo LLVM poderão ser utilizadas para otimizar alocação e acesso à memória.

---

## 20.4.25 Scalar Replacement

Estruturas compostas poderão ser decompostas em valores escalares quando isso preservar integralmente a semântica do programa.

---

## 20.4.26 Loop Optimizations

A infraestrutura LLVM poderá aplicar otimizações sobre laços de repetição, incluindo transformações de reordenação, simplificação e eliminação de redundâncias.

---

## 20.4.27 Vectorização

Sempre que suportado pela arquitetura de destino, o LLVM poderá aplicar técnicas automáticas de vetorização.

---

## 20.4.28 IPO

Otimizações interprocedimentais (*Interprocedural Optimization*) poderão ser utilizadas quando apropriado.

---

## 20.4.29 LTO

O compilador poderá utilizar *Link Time Optimization* para ampliar oportunidades de otimização entre unidades de compilação.

---

## 20.4.30 ThinLTO

Quando suportado, poderá ser utilizada a infraestrutura ThinLTO como alternativa ao LTO tradicional.

---

## 20.4.31 PGO

A implementação poderá oferecer suporte à otimização guiada por perfil (*Profile Guided Optimization*).

---

## 20.4.32 Otimizações Específicas da Arquitetura

O LLVM poderá aplicar otimizações específicas para a arquitetura de destino.

Essas otimizações não poderão alterar os contratos da linguagem.

---

## 20.4.33 Independência da Linguagem

Nenhuma otimização poderá depender de detalhes não representados na IR da linguagem ou na LLVM IR produzida pelo *lowering*.

---

## 20.4.34 Preservação da ABI

Todas as otimizações deverão preservar integralmente a ABI oficial da linguagem.

---

## 20.4.35 Preservação do Runtime

Chamadas ao Runtime somente poderão ser modificadas quando houver garantia formal de equivalência funcional.

---

## 20.4.36 Observabilidade

As otimizações não poderão modificar o comportamento observável do programa.

---

## 20.4.37 Instrumentação

A implementação deverá permitir instrumentação para coleta de métricas relativas às otimizações aplicadas.

---

## 20.4.38 Estatísticas

Entre as estatísticas disponíveis poderão estar:

* número de *passes* executadas;
* tempo gasto em otimizações;
* quantidade de transformações realizadas;
* impacto estimado sobre desempenho;
* impacto estimado sobre tamanho do binário.

---

## 20.4.39 Diagnósticos

Sempre que possível, deverá ser possível identificar quais otimizações foram aplicadas durante a compilação.

---

## 20.4.40 Comparabilidade

Os resultados produzidos pelo Backend LLVM deverão permanecer comparáveis aos produzidos pelo Backend Cranelift em termos de:

* desempenho;
* tempo de compilação;
* tamanho do código;
* consumo de memória.

---

## 20.4.41 Evolução

Novas *passes* disponibilizadas pelo LLVM poderão ser incorporadas sem necessidade de alteração desta especificação, desde que preservem todos os contratos aqui definidos.

---

## 20.4.42 Invariantes

Durante toda a execução das otimizações deverão permanecer verdadeiros os seguintes princípios:

* nenhuma otimização altera a semântica da linguagem;
* a ABI permanece integralmente preservada;
* o Runtime continua obedecendo aos mesmos contratos públicos;
* ownership, lifetimes, Domains e ObjectId permanecem semanticamente equivalentes;
* toda otimização representa exclusivamente uma melhoria da implementação física do programa.

---

## 20.4.43 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma infraestrutura de otimização que:

* utilize o LLVM Pass Manager;
* preserve integralmente a semântica da linguagem Capi;
* permita diferentes níveis de otimização;
* suporte otimizações locais, interprocedimentais e de linkedição;
* mantenha compatibilidade completa com a ABI, o Runtime e o modelo de objetos definidos pela especificação.

---

# 20.5 Suporte a Arquiteturas e Plataformas

## 20.5.1 Objetivo

Esta seção define os princípios arquiteturais para o suporte do Backend LLVM a diferentes arquiteturas de processador, sistemas operacionais e plataformas de execução.

Seu objetivo é estabelecer um modelo portável e escalável, permitindo que a implementação oficial da linguagem Capi produza código nativo para múltiplos ambientes sem alterar a semântica da linguagem.

---

## 20.5.2 Independência da Linguagem

A linguagem Capi deverá permanecer completamente independente da arquitetura de destino.

Nenhum elemento da sintaxe, da semântica ou da biblioteca padrão deverá depender de características específicas de uma plataforma.

---

## 20.5.3 Responsabilidade do Backend

A adaptação às características da plataforma constitui responsabilidade exclusiva do Backend LLVM e da infraestrutura LLVM.

---

## 20.5.4 Backend Neutro

A Backend API deverá permanecer completamente independente da arquitetura de destino.

---

## 20.5.5 Arquitetura Conceitual

O suporte multiplataforma poderá ser representado da seguinte forma:

```text id="tq7mke"
             IR da Linguagem
                    │
                    ▼
               Backend API
                    │
                    ▼
               Backend LLVM
                    │
       ┌────────────┼────────────┐
       ▼            ▼            ▼
     x86-64      AArch64       RISC-V
       │            │            │
       └────────────┼────────────┘
                    ▼
             Código Objeto
```

---

## 20.5.6 Arquiteturas Suportadas

A implementação oficial poderá oferecer suporte a diferentes arquiteturas de processador.

A lista de arquiteturas suportadas constitui um detalhe evolutivo da implementação.

---

## 20.5.7 Arquiteturas Prioritárias

Durante a evolução da implementação oficial, deverão ser priorizadas arquiteturas amplamente utilizadas pela comunidade.

Exemplos incluem:

* x86-64;
* AArch64;
* RISC-V.

---

## 20.5.8 Outras Arquiteturas

Novas arquiteturas poderão ser adicionadas futuramente sem necessidade de alteração desta especificação.

---

## 20.5.9 Sistemas Operacionais

O Backend LLVM deverá permitir geração de código para diferentes sistemas operacionais suportados pelo LLVM.

---

## 20.5.10 Portabilidade

Programas escritos em Capi deverão permanecer portáveis sempre que utilizarem exclusivamente recursos definidos pela especificação da linguagem e da biblioteca padrão.

---

## 20.5.11 Cross Compilation

A implementação oficial deverá permitir compilação cruzada (*cross compilation*) sempre que suportada pela infraestrutura LLVM.

---

## 20.5.12 Configuração do Target

A seleção da plataforma de destino deverá ocorrer através da configuração do compilador.

Ela não deverá exigir alterações no código-fonte da aplicação.

---

## 20.5.13 Triple de Destino

A identificação da plataforma poderá utilizar os mecanismos de descrição de *target* disponibilizados pelo LLVM.

A representação específica constitui detalhe da implementação.

---

## 20.5.14 ABI

A ABI oficial da linguagem deverá permanecer consistente em cada plataforma suportada.

Quando diferentes convenções de chamada forem exigidas pela arquitetura de destino, sua adaptação deverá ocorrer exclusivamente no backend.

---

## 20.5.15 Layout de Dados

O Backend LLVM deverá respeitar integralmente:

* alinhamento;
* tamanho;
* organização física;
* requisitos da arquitetura.

Sem alterar os contratos definidos pela linguagem.

---

## 20.5.16 Endianness

Diferenças de ordenação de bytes deverão ser tratadas exclusivamente pelo backend e pela infraestrutura LLVM.

---

## 20.5.17 Tamanho de Ponteiros

Diferenças no tamanho de ponteiros deverão permanecer transparentes para os programas escritos em Capi.

---

## 20.5.18 Registradores

A utilização de registradores físicos constitui exclusivamente responsabilidade da infraestrutura LLVM.

---

## 20.5.19 Convenções de Chamada

Convenções específicas da arquitetura deverão ser aplicadas automaticamente pelo backend.

---

## 20.5.20 Instruções Específicas

Sempre que apropriado, o LLVM poderá utilizar instruções específicas da arquitetura para melhorar desempenho.

Essas otimizações não poderão alterar o comportamento observável do programa.

---

## 20.5.21 Extensões do Processador

O Backend LLVM poderá explorar extensões específicas do processador quando explicitamente habilitadas durante a compilação.

---

## 20.5.22 Compatibilidade

Na ausência de configuração específica, deverão ser utilizadas configurações compatíveis com a arquitetura de destino selecionada.

---

## 20.5.23 Runtime

O Runtime deverá permanecer portável.

Adaptações específicas da plataforma deverão permanecer encapsuladas em sua implementação.

---

## 20.5.24 Biblioteca Padrão

A biblioteca padrão deverá oferecer comportamento consistente em todas as plataformas suportadas, ressalvadas limitações inerentes ao sistema operacional ou ao hardware.

---

## 20.5.25 Geração de Código

A geração de código deverá utilizar exclusivamente a infraestrutura oficial disponibilizada pelo LLVM para cada arquitetura.

---

## 20.5.26 Emissão de Objetos

O formato do código objeto deverá ser compatível com o sistema operacional de destino.

A seleção do formato apropriado constitui responsabilidade do backend.

---

## 20.5.27 Linkedição

O processo de linkedição deverá respeitar as convenções da plataforma de destino.

---

## 20.5.28 Debug

Sempre que suportado pelo LLVM, o backend deverá produzir informações de depuração compatíveis com a plataforma de destino.

---

## 20.5.29 Ferramentas Externas

A integração com montadores, *linkers* e demais ferramentas da cadeia de compilação deverá utilizar os mecanismos oficialmente suportados pela plataforma.

---

## 20.5.30 WebAssembly

O Backend LLVM poderá oferecer suporte à geração de código para WebAssembly, desde que todos os contratos da linguagem possam ser preservados.

---

## 20.5.31 Restrições da Plataforma

Quando uma plataforma impuser limitações técnicas incompatíveis com algum recurso da linguagem, essas limitações deverão ser claramente documentadas pela implementação.

---

## 20.5.32 Diagnósticos

Falhas relacionadas à arquitetura ou à plataforma de destino deverão produzir diagnósticos claros e determinísticos.

---

## 20.5.33 Testabilidade

Cada arquitetura suportada deverá possuir uma suíte mínima de testes de validação.

---

## 20.5.34 Testes Cruzados

Sempre que possível, os mesmos programas de teste deverão ser executados em diferentes arquiteturas para verificar equivalência funcional.

---

## 20.5.35 Benchmarks

A infraestrutura de benchmarking deverá permitir comparação entre arquiteturas distintas.

---

## 20.5.36 Evolução

A inclusão de novos *targets* não deverá exigir alterações no frontend, na IR, na Backend API ou no Runtime.

---

## 20.5.37 Escalabilidade

A arquitetura deverá permitir crescimento contínuo da lista de plataformas suportadas sem aumento significativo do acoplamento entre componentes.

---

## 20.5.38 Neutralidade Arquitetural

Nenhuma decisão de projeto da linguagem deverá favorecer uma arquitetura específica de processador.

---

## 20.5.39 Invariantes

Durante todo o suporte multiplataforma deverão permanecer verdadeiros os seguintes princípios:

* a linguagem permanece independente da arquitetura de destino;
* a Backend API permanece neutra em relação ao hardware;
* toda adaptação específica ocorre exclusivamente no Backend LLVM;
* o Runtime preserva seus contratos em todas as plataformas suportadas;
* diferenças entre arquiteturas não alteram a semântica dos programas.

---

## 20.5.40 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma arquitetura capaz de:

* gerar código nativo para múltiplas arquiteturas suportadas pelo LLVM;
* realizar compilação cruzada quando tecnicamente possível;
* preservar integralmente a semântica da linguagem em todas as plataformas;
* manter independência entre frontend, Runtime e infraestrutura de geração de código;
* permitir expansão contínua para novos *targets* sem alterações na especificação da linguagem.

---

# 20.6 Diagnósticos, Depuração e Ferramentas

## 20.6.1 Objetivo

Esta seção define os princípios arquiteturais para produção de diagnósticos, informações de depuração e integração com ferramentas de desenvolvimento no Backend LLVM.

Seu objetivo é assegurar que a utilização do LLVM ofereça uma experiência consistente de desenvolvimento, preservando a qualidade dos diagnósticos produzidos pelo compilador e permitindo integração com ferramentas modernas de análise, depuração e instrumentação.

---

## 20.6.2 Princípio Geral

O Backend LLVM deverá complementar a infraestrutura oficial do compilador, jamais substituí-la.

Diagnósticos relacionados à linguagem deverão continuar sendo responsabilidade das etapas anteriores do pipeline de compilação.

---

## 20.6.3 Escopo

Esta seção compreende:

* diagnósticos internos do backend;
* informações de depuração;
* símbolos de debug;
* integração com depuradores;
* integração com sanitizers;
* coleta de cobertura;
* análise de desempenho;
* instrumentação.

---

## 20.6.4 Separação de Responsabilidades

O Backend LLVM deverá produzir apenas diagnósticos relacionados à geração de código.

Diagnósticos referentes à sintaxe, semântica, sistema de tipos ou modelo de memória permanecem responsabilidade do frontend.

---

## 20.6.5 Diagnósticos Determinísticos

Os diagnósticos produzidos pelo Backend LLVM deverão ser determinísticos.

Dada a mesma entrada e a mesma configuração, deverão produzir resultados equivalentes.

---

## 20.6.6 Associação com a IR

Sempre que possível, cada diagnóstico deverá indicar a entidade correspondente na Intermediate Representation (IR).

---

## 20.6.7 Associação com o Código-Fonte

Quando informações de localização estiverem disponíveis, os diagnósticos deverão ser associados ao trecho correspondente do código-fonte original.

---

## 20.6.8 Clareza

Mensagens produzidas pelo backend deverão ser claras, objetivas e tecnicamente precisas.

---

## 20.6.9 Recuperação

Sempre que tecnicamente possível, falhas localizadas não deverão impedir a continuação da compilação das demais unidades.

---

## 20.6.10 Informações de Depuração

O Backend LLVM deverá ser capaz de produzir informações de depuração compatíveis com as ferramentas suportadas pela plataforma de destino.

---

## 20.6.11 Formato

Sempre que possível, deverão ser utilizados formatos padronizados amplamente suportados pela indústria.

---

## 20.6.12 DWARF

Em plataformas que utilizem DWARF como padrão de depuração, o Backend LLVM deverá gerar informações compatíveis com esse formato.

---

## 20.6.13 Outros Formatos

Em plataformas que utilizem formatos distintos de DWARF, deverão ser empregados os mecanismos oficialmente suportados pela infraestrutura LLVM.

---

## 20.6.14 Correspondência

As informações de depuração deverão preservar a correspondência entre:

* código-fonte;
* AST;
* IR;
* código executável.

Sempre que essa correspondência puder ser estabelecida.

---

## 20.6.15 Funções

As funções produzidas pelo backend deverão manter informações suficientes para facilitar sua identificação durante processos de depuração.

---

## 20.6.16 Variáveis

Sempre que possível, deverão ser preservadas informações sobre variáveis locais para depuração.

---

## 20.6.17 Inlining

Quando ocorrer *inlining*, as informações de depuração deverão preservar a origem lógica da função incorporada.

---

## 20.6.18 Otimizações

A presença de otimizações poderá limitar a precisão das informações de depuração.

Essas limitações deverão decorrer exclusivamente das transformações realizadas pelo LLVM.

---

## 20.6.19 Símbolos

O Backend LLVM deverá produzir símbolos compatíveis com as ferramentas de inspeção da plataforma.

---

## 20.6.20 Linkedição

As informações de depuração deverão permanecer consistentes após a etapa de linkedição.

---

## 20.6.21 Depuradores

Sempre que possível, o código produzido deverá ser compatível com os depuradores suportados pela plataforma de destino.

---

## 20.6.22 Sanitizers

A implementação poderá oferecer integração com os sanitizers disponibilizados pelo LLVM.

---

## 20.6.23 Tipos de Sanitizers

Entre os mecanismos que poderão ser suportados incluem-se:

* Address Sanitizer;
* Undefined Behavior Sanitizer;
* Thread Sanitizer;
* Leak Sanitizer;
* Memory Sanitizer.

A disponibilidade dependerá da plataforma e da infraestrutura LLVM utilizada.

---

## 20.6.24 Uso dos Sanitizers

Os sanitizers deverão constituir ferramentas auxiliares de desenvolvimento.

Eles não fazem parte da semântica da linguagem Capi.

---

## 20.6.25 Instrumentação

O Backend LLVM poderá gerar código instrumentado para coleta de informações de execução.

---

## 20.6.26 Cobertura de Código

A implementação poderá integrar-se aos mecanismos de cobertura disponibilizados pelo LLVM.

---

## 20.6.27 Profiling

O Backend LLVM poderá produzir binários preparados para coleta de perfis de execução.

---

## 20.6.28 PGO

Quando utilizada otimização guiada por perfil (*Profile Guided Optimization*), a infraestrutura de instrumentação deverá permanecer compatível com os mecanismos oficiais do LLVM.

---

## 20.6.29 Benchmarks

A instrumentação para benchmarking deverá permanecer separada da infraestrutura de depuração.

---

## 20.6.30 Logging

O Backend LLVM deverá disponibilizar informações de logging para auxiliar investigação de falhas internas.

---

## 20.6.31 Estatísticas

A implementação poderá disponibilizar estatísticas como:

* funções traduzidas;
* blocos produzidos;
* instruções emitidas;
* tempo de geração;
* tempo de otimização;
* consumo aproximado de memória.

---

## 20.6.32 Dumps

Sempre que habilitado, o Backend LLVM poderá produzir representações intermediárias para inspeção técnica.

Exemplos incluem:

* LLVM IR;
* estatísticas de otimização;
* árvores de passes;
* representações intermediárias auxiliares.

---

## 20.6.33 Verificação

Sempre que possível, deverá ser utilizada a infraestrutura oficial de verificação disponibilizada pelo LLVM para validar a consistência da LLVM IR produzida.

---

## 20.6.34 Integração

Ferramentas auxiliares deverão ser integradas sem introduzir dependências na arquitetura da linguagem.

---

## 20.6.35 Configuração

Os mecanismos de depuração, instrumentação e geração de diagnósticos deverão ser configuráveis independentemente do nível de otimização.

---

## 20.6.36 Produção

A ausência de informações de depuração não poderá alterar o comportamento funcional do programa gerado.

---

## 20.6.37 Testabilidade

Toda funcionalidade definida nesta seção deverá poder ser validada por testes automatizados.

---

## 20.6.38 Evolução

Novas ferramentas disponibilizadas pelo ecossistema LLVM poderão ser incorporadas futuramente sem necessidade de alteração desta especificação, desde que preservem os contratos definidos pela linguagem.

---

## 20.6.39 Independência

Ferramentas específicas do LLVM deverão permanecer encapsuladas dentro do Backend LLVM.

Nenhuma API pública do compilador deverá depender diretamente dessas ferramentas.

---

## 20.6.40 Invariantes

Durante toda a utilização das ferramentas de diagnóstico e depuração deverão permanecer verdadeiros os seguintes princípios:

* diagnósticos da linguagem permanecem responsabilidade do frontend;
* diagnósticos do Backend LLVM restringem-se à geração de código;
* informações de depuração não alteram a semântica dos programas;
* instrumentação constitui funcionalidade opcional da implementação;
* ferramentas específicas do LLVM permanecem encapsuladas pelo backend;
* toda integração preserva a independência arquitetural do compilador.

---

## 20.6.41 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um Backend LLVM capaz de:

* produzir diagnósticos claros e determinísticos;
* gerar informações de depuração compatíveis com a plataforma de destino;
* integrar-se aos mecanismos oficiais de verificação, sanitização e profiling do LLVM;
* oferecer instrumentação para depuração, cobertura e análise de desempenho;
* manter total independência entre a arquitetura da linguagem Capi e as ferramentas específicas disponibilizadas pelo ecossistema LLVM.

---

# 20.7 Critérios para Tornar-se o Backend Principal

## 20.7.1 Objetivo

Esta seção estabelece os critérios normativos que deverão ser atendidos para que o Backend LLVM substitua o Backend Cranelift como backend principal da implementação oficial da linguagem Capi.

Seu objetivo é garantir que essa transição ocorra exclusivamente com base em critérios técnicos, objetivos e verificáveis, preservando a estabilidade da implementação oficial.

---

## 20.7.2 Princípio Geral

O Backend Cranelift constitui o backend oficial durante o processo de bootstrap da linguagem.

A adoção do Backend LLVM deverá ocorrer apenas quando demonstrada sua maturidade técnica.

---

## 20.7.3 Natureza da Transição

A migração para o Backend LLVM representa uma evolução da implementação oficial.

Ela não altera:

* a linguagem;
* a especificação;
* a IR;
* a ABI;
* o Runtime;
* o modelo de objetos.

---

## 20.7.4 Critério Fundamental

O Backend LLVM somente poderá tornar-se o backend principal quando demonstrar equivalência funcional completa em relação ao Backend Cranelift.

---

## 20.7.5 Equivalência Funcional

Considera-se equivalência funcional a produção de programas que apresentem comportamento observável idêntico para uma mesma entrada.

Diferenças internas de implementação não caracterizam divergência funcional.

---

## 20.7.6 Preservação da Linguagem

A adoção do Backend LLVM não poderá introduzir qualquer alteração na semântica da linguagem Capi.

---

## 20.7.7 Preservação da ABI

O Backend LLVM deverá implementar integralmente a ABI oficial da linguagem.

---

## 20.7.8 Preservação do Runtime

Toda interação com o Runtime deverá ocorrer exatamente através das interfaces públicas definidas pela especificação.

---

## 20.7.9 Preservação do Modelo de Objetos

O Backend LLVM deverá preservar integralmente:

* identidade dos objetos;
* despacho virtual;
* herança;
* Domains;
* ObjectId;
* ownership;
* lifetimes.

---

## 20.7.10 Compatibilidade

Todo programa válido compilado pelo Backend Cranelift deverá poder ser compilado pelo Backend LLVM.

---

## 20.7.11 Cobertura da Linguagem

O Backend LLVM deverá implementar integralmente todos os recursos definidos pela especificação oficial da linguagem.

---

## 20.7.12 Biblioteca Padrão

Toda a biblioteca padrão deverá ser suportada sem restrições funcionais.

---

## 20.7.13 Runtime

Todo o Runtime oficial deverá operar sem necessidade de adaptações específicas para utilização do Backend LLVM.

---

## 20.7.14 Suíte Oficial de Testes

O Backend LLVM deverá executar com sucesso toda a suíte oficial de testes da linguagem.

---

## 20.7.15 Testes Diferenciais

A validação deverá utilizar testes diferenciais executando os mesmos programas em:

* Backend Cranelift;
* Backend LLVM.

---

## 20.7.16 Critério de Aprovação

Diferenças observadas entre os resultados produzidos pelos dois backends deverão ser analisadas e justificadas.

Divergências semânticas não serão aceitas.

---

## 20.7.17 Benchmarks

O Backend LLVM deverá participar da mesma infraestrutura oficial de benchmarks utilizada pelo Backend Cranelift.

---

## 20.7.18 Tempo de Compilação

O tempo de compilação não constitui critério obrigatório para substituição do Backend Cranelift.

Entretanto, deverá permanecer compatível com os objetivos da implementação oficial.

---

## 20.7.19 Desempenho

Espera-se que o Backend LLVM produza código de desempenho igual ou superior ao produzido pelo Backend Cranelift.

Esse ganho, entretanto, não substitui os critérios de correção funcional.

---

## 20.7.20 Tamanho do Código

A redução do tamanho dos binários poderá ser considerada vantagem adicional, mas não constitui requisito obrigatório.

---

## 20.7.21 Estabilidade

O Backend LLVM deverá demonstrar estabilidade suficiente para utilização contínua em projetos reais.

---

## 20.7.22 Robustez

Falhas internas deverão ocorrer apenas em situações excepcionais.

---

## 20.7.23 Diagnósticos

Os diagnósticos produzidos deverão possuir qualidade equivalente ou superior aos disponibilizados pelo Backend Cranelift.

---

## 20.7.24 Ferramentas

Integrações com depuração, profiling, sanitizers e demais ferramentas deverão estar plenamente operacionais.

---

## 20.7.25 Suporte a Targets

Os principais *targets* suportados pela implementação oficial deverão estar disponíveis no Backend LLVM.

---

## 20.7.26 Compatibilidade com Bootstrap

O processo de bootstrap deverá poder ser concluído utilizando exclusivamente o Backend LLVM.

---

## 20.7.27 Auto-Hospedagem

Sempre que a implementação oficial atingir a fase de auto-hospedagem, o Backend LLVM deverá ser capaz de compilar integralmente o próprio compilador.

---

## 20.7.28 Regressões

A substituição do backend principal não poderá introduzir regressões funcionais conhecidas.

---

## 20.7.29 Evolução Incremental

A migração poderá ocorrer de maneira gradual.

Durante esse período, ambos os backends poderão coexistir.

---

## 20.7.30 Backend de Referência

Mesmo após a adoção do Backend LLVM como backend principal, o Backend Cranelift poderá permanecer disponível como backend de referência para:

* testes;
* validação;
* comparação;
* desenvolvimento.

---

## 20.7.31 Critério de Descontinuação

A eventual descontinuação do Backend Cranelift constitui decisão de implementação e não faz parte desta especificação.

---

## 20.7.32 Processo de Validação

A validação da migração deverá considerar conjuntamente:

* conformidade funcional;
* estabilidade;
* cobertura de testes;
* qualidade da geração de código;
* maturidade da implementação.

Nenhum desses critérios, isoladamente, será suficiente para justificar a substituição do backend principal.

---

## 20.7.33 Documentação

A adoção do Backend LLVM deverá ser acompanhada da atualização da documentação da implementação oficial, descrevendo:

* status de maturidade;
* plataformas suportadas;
* limitações conhecidas;
* requisitos da ferramenta.

---

## 20.7.34 Transparência

Os critérios utilizados para promoção do Backend LLVM deverão ser públicos e reproduzíveis.

---

## 20.7.35 Evolução Contínua

Após tornar-se o backend principal, o Backend LLVM continuará sujeito aos mesmos critérios de qualidade aplicados às demais partes da implementação oficial.

---

## 20.7.36 Neutralidade Arquitetural

A existência de um backend principal não altera a arquitetura do compilador.

A Backend API continuará permitindo a coexistência de múltiplos backends.

---

## 20.7.37 Compatibilidade Futura

A arquitetura deverá permanecer preparada para suportar novos backends além do Cranelift e do LLVM, desde que implementem integralmente a Backend API.

---

## 20.7.38 Invariantes

Durante todo o processo de transição deverão permanecer verdadeiros os seguintes princípios:

* a especificação da linguagem permanece inalterada;
* a Backend API continua sendo a única interface oficial entre a IR e os backends;
* a ABI permanece única para todos os backends;
* o Runtime permanece independente da infraestrutura de geração de código;
* a substituição do backend principal não altera o comportamento observável dos programas;
* a promoção do Backend LLVM baseia-se exclusivamente em critérios técnicos objetivos e verificáveis.

---

## 20.7.39 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um Backend LLVM que:

* implemente integralmente todos os recursos da linguagem Capi;
* execute com sucesso a suíte oficial de testes;
* demonstre equivalência funcional em relação ao Backend Cranelift;
* preserve integralmente a ABI, o Runtime e o modelo de objetos;
* esteja apto a realizar o bootstrap e a auto-hospedagem do compilador;
* apresente estabilidade suficiente para tornar-se o backend principal da implementação oficial da linguagem Capi.

---

# 21. Política de Backends

## 21.1 Objetivos e Princípios Gerais

### 21.1.1 Objetivo

Esta seção estabelece os princípios arquiteturais permanentes que regem o desenvolvimento, evolução e manutenção dos backends da implementação oficial da linguagem Capi.

Seu objetivo é definir uma política única para todos os backends suportados, garantindo estabilidade da arquitetura do compilador, preservação dos contratos definidos pela especificação da linguagem e independência entre a evolução da linguagem e a evolução das tecnologias de geração de código.

---

## 21.1.2 Escopo

Esta política aplica-se a todo backend oficialmente suportado pela implementação da linguagem Capi, independentemente da tecnologia utilizada para geração de código.

---

## 21.1.3 Papel dos Backends

Os backends constituem componentes responsáveis por transformar a Intermediate Representation (IR) da linguagem em artefatos executáveis compatíveis com a plataforma de destino.

---

## 21.1.4 Papel na Arquitetura

Os backends representam a etapa final do pipeline de compilação.

Eles não participam da definição da linguagem nem de sua semântica.

---

## 21.1.5 Separação entre Linguagem e Implementação

A linguagem Capi é definida exclusivamente por sua especificação oficial.

Os backends constituem implementações dessa especificação.

---

## 21.1.6 Princípio da Independência

A especificação da linguagem deverá permanecer completamente independente da tecnologia utilizada para geração de código.

---

## 21.1.7 Backend como Detalhe de Implementação

A escolha do backend constitui exclusivamente uma decisão de implementação do compilador.

Programas escritos em Capi não deverão depender da existência de um backend específico.

---

## 21.1.8 Neutralidade Tecnológica

A arquitetura do compilador não deverá privilegiar nenhuma infraestrutura específica de geração de código.

Todo backend oficialmente suportado deverá integrar-se através da mesma arquitetura.

---

## 21.1.9 Backend API

A Backend API constitui o único contrato oficial entre a arquitetura do compilador e seus backends.

Toda integração deverá ocorrer exclusivamente através dessa interface.

---

## 21.1.10 Intermediate Representation

A Intermediate Representation (IR) constitui a única representação da linguagem aceita pelos backends.

Nenhum backend poderá consumir diretamente estruturas do frontend.

---

## 21.1.11 Frontend

Lexer, Parser, AST, HIR, resolução de nomes, inferência de tipos, verificações semânticas e construção da IR pertencem exclusivamente ao frontend.

---

## 21.1.12 Runtime

O Runtime constitui componente compartilhado por todos os backends.

Nenhum backend poderá redefinir seu comportamento.

---

## 21.1.13 ABI

A ABI oficial da linguagem deverá permanecer única para todos os backends.

---

## 21.1.14 Modelo de Objetos

Todos os backends deverão implementar exatamente o modelo de objetos definido pela especificação da linguagem.

---

## 21.1.15 Sistema de Tipos

O sistema de tipos constitui responsabilidade exclusiva da linguagem.

Backends apenas materializam sua representação física.

---

## 21.1.16 Ownership

As regras de ownership deverão estar completamente resolvidas antes da geração de código.

---

## 21.1.17 Lifetimes

A resolução de lifetimes constitui responsabilidade do frontend.

Backends não deverão reinterpretar essas informações.

---

## 21.1.18 Domains

Domains deverão ser materializados exclusivamente através das interfaces públicas do Runtime.

---

## 21.1.19 ObjectId

Todos os contratos relacionados ao ObjectId deverão permanecer preservados independentemente do backend utilizado.

---

## 21.1.20 Semântica

A semântica da linguagem jamais poderá depender de decisões específicas de um backend.

---

## 21.1.21 Portabilidade

Programas válidos deverão produzir comportamento observável equivalente em qualquer backend oficialmente suportado.

---

## 21.1.22 Determinismo

Dadas a mesma IR e a mesma configuração do backend, a geração de código deverá produzir resultados funcionalmente equivalentes.

---

## 21.1.23 Evolução

Novos backends poderão ser adicionados sem necessidade de alterações na especificação da linguagem.

---

## 21.1.24 Escalabilidade

A arquitetura deverá permitir coexistência de múltiplos backends simultaneamente.

---

## 21.1.25 Especialização

Cada backend poderá utilizar técnicas próprias de geração de código, desde que preserve integralmente os contratos definidos pela linguagem.

---

## 21.1.26 Otimizações

Otimizações constituem responsabilidade do backend.

Elas não poderão modificar o comportamento observável do programa.

---

## 21.1.27 Ferramentas

Cada backend poderá integrar-se às ferramentas disponibilizadas por sua infraestrutura tecnológica.

Essa integração não poderá alterar a arquitetura pública do compilador.

---

## 21.1.28 Testabilidade

Todo backend deverá permitir validação independente através da infraestrutura oficial de testes.

---

## 21.1.29 Comparabilidade

A arquitetura deverá permitir comparação objetiva entre diferentes backends utilizando:

* suíte oficial de testes;
* benchmarks;
* testes diferenciais;
* diagnósticos;
* métricas de desempenho.

---

## 21.1.30 Reutilização

Toda infraestrutura independente da tecnologia de geração de código deverá permanecer compartilhada.

---

## 21.1.31 Modularidade

Componentes específicos de um backend deverão permanecer encapsulados.

---

## 21.1.32 Baixo Acoplamento

Nenhum componente do frontend, Runtime ou Backend API deverá depender de APIs específicas de um backend concreto.

---

## 21.1.33 Compatibilidade

Backends oficiais deverão permanecer compatíveis entre si quanto ao comportamento funcional dos programas produzidos.

---

## 21.1.34 Estabilidade

A política de backends deverá favorecer estabilidade arquitetural ao longo da evolução da linguagem.

---

## 21.1.35 Evolução Incremental

A adoção de novos backends deverá ocorrer de maneira incremental, preservando compatibilidade com os componentes existentes.

---

## 21.1.36 Transparência

As responsabilidades de cada backend deverão permanecer claramente delimitadas pela arquitetura do compilador.

---

## 21.1.37 Independência da Implementação

Nenhuma decisão de implementação de um backend deverá tornar-se requisito da linguagem Capi.

---

## 21.1.38 Invariantes

Durante toda a evolução da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* a especificação da linguagem permanece independente dos backends;
* a Backend API constitui o único contrato oficial entre a IR e a geração de código;
* existe uma única IR oficial da linguagem;
* existe um único Runtime oficial;
* existe uma única ABI oficial;
* todos os backends implementam exatamente a mesma semântica da linguagem;
* diferenças entre backends restringem-se exclusivamente à implementação da geração de código.

---

## 21.1.39 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma política de backends que:

* preserve integralmente a separação entre linguagem e implementação;
* permita coexistência de múltiplos backends;
* mantenha a Backend API como contrato permanente da arquitetura;
* assegure compatibilidade funcional entre todos os backends oficiais;
* permita evolução contínua da infraestrutura de geração de código sem necessidade de alterações na especificação da linguagem Capi.

---

# 21.2 Backend API como Contrato Permanente

## 21.2.1 Objetivo

Esta seção estabelece a Backend API como o contrato arquitetural permanente entre a infraestrutura do compilador e qualquer implementação de backend oficialmente suportada.

Seu objetivo é garantir estabilidade arquitetural, desacoplamento entre componentes e evolução independente da linguagem e das tecnologias de geração de código.

---

## 21.2.2 Papel da Backend API

A Backend API constitui a única interface oficial entre a Intermediate Representation (IR) da linguagem Capi e os mecanismos responsáveis pela geração de código.

---

## 21.2.3 Contrato Arquitetural

A Backend API representa um contrato permanente da arquitetura do compilador.

Todos os backends deverão implementá-la integralmente.

---

## 21.2.4 Fronteira

A Backend API estabelece a fronteira entre:

* a infraestrutura compartilhada do compilador; e
* as implementações concretas de geração de código.

---

## 21.2.5 Exclusividade

Nenhum backend poderá acessar diretamente componentes internos do frontend.

Toda interação deverá ocorrer exclusivamente através da Backend API.

---

## 21.2.6 Independência

A Backend API deverá permanecer independente de qualquer infraestrutura específica de geração de código.

---

## 21.2.7 Neutralidade

Nenhum elemento da Backend API poderá depender de:

* LLVM;
* Cranelift;
* GCC;
* MLIR;
* qualquer outra tecnologia específica.

---

## 21.2.8 Estabilidade

A Backend API deverá permanecer estável ao longo da evolução da implementação oficial.

Alterações incompatíveis deverão ocorrer apenas em situações excepcionais e devidamente justificadas.

---

## 21.2.9 Evolução

A Backend API poderá evoluir continuamente por meio da adição de novas capacidades, preservando sempre que possível a compatibilidade com implementações existentes.

---

## 21.2.10 Compatibilidade Retroativa

Sempre que tecnicamente viável, novas versões da Backend API deverão manter compatibilidade com versões anteriores.

---

## 21.2.11 Responsabilidades

A Backend API deverá fornecer abstrações suficientes para:

* criação de módulos;
* criação de funções;
* emissão de dados;
* geração de símbolos;
* interação com o Runtime;
* geração de código objeto;
* diagnósticos;
* instrumentação.

---

## 21.2.12 Não Responsabilidades

A Backend API não deverá definir:

* semântica da linguagem;
* sistema de tipos;
* modelo de memória;
* modelo de objetos;
* Runtime;
* ABI.

Esses elementos pertencem à especificação da linguagem.

---

## 21.2.13 Entrada

A única entrada aceita pela Backend API deverá ser a Intermediate Representation validada da linguagem.

---

## 21.2.14 Saída

A Backend API deverá produzir abstrações suficientes para que cada backend gere os artefatos compatíveis com sua infraestrutura de geração de código.

---

## 21.2.15 Backend Context

A Backend API deverá definir um contexto responsável pelo ciclo de vida da geração de código.

---

## 21.2.16 Backend Module

Cada módulo deverá possuir representação própria dentro da Backend API.

---

## 21.2.17 Backend Function

Cada função deverá possuir abstração independente durante o processo de geração de código.

---

## 21.2.18 Backend Builder

A construção incremental das entidades deverá ocorrer através de mecanismos definidos pela Backend API.

---

## 21.2.19 Backend Emitter

A emissão dos artefatos finais deverá ocorrer através das abstrações oficiais da Backend API.

---

## 21.2.20 Runtime

Toda interação com o Runtime deverá ocorrer através de interfaces previstas pela Backend API.

---

## 21.2.21 ABI

A Backend API deverá fornecer mecanismos suficientes para implementação integral da ABI oficial da linguagem.

---

## 21.2.22 Diagnósticos

A Backend API deverá permitir produção padronizada de diagnósticos relacionados à geração de código.

---

## 21.2.23 Estatísticas

A coleta de estatísticas deverá ocorrer através de mecanismos independentes da implementação concreta do backend.

---

## 21.2.24 Instrumentação

A Backend API deverá permitir instrumentação uniforme para todos os backends.

---

## 21.2.25 Configuração

Parâmetros específicos da geração de código deverão ser fornecidos através de estruturas de configuração definidas pela Backend API.

---

## 21.2.26 Extensibilidade

Novas funcionalidades deverão ser incorporadas preferencialmente por extensão da Backend API, evitando alterações incompatíveis.

---

## 21.2.27 Implementações

Toda implementação oficial deverá demonstrar conformidade integral com a Backend API.

Implementações parciais não deverão ser consideradas backends oficialmente suportados.

---

## 21.2.28 Testabilidade

A Backend API deverá permitir validação independente de cada backend.

---

## 21.2.29 Testes Diferenciais

A existência de uma Backend API única deverá permitir execução da mesma suíte de testes utilizando diferentes backends.

---

## 21.2.30 Comparabilidade

A Backend API deverá permitir comparação objetiva entre implementações quanto a:

* comportamento funcional;
* desempenho;
* tempo de compilação;
* consumo de memória;
* qualidade da geração de código.

---

## 21.2.31 Baixo Acoplamento

O frontend não deverá possuir qualquer dependência direta de implementações concretas de backend.

---

## 21.2.32 Encapsulamento

Detalhes específicos de cada tecnologia de geração de código deverão permanecer completamente encapsulados pelas implementações da Backend API.

---

## 21.2.33 Escalabilidade

A Backend API deverá permitir a coexistência de múltiplos backends simultaneamente.

---

## 21.2.34 Inclusão de Novos Backends

A introdução de um novo backend não deverá exigir modificações no frontend, na IR, no Runtime ou na especificação da linguagem.

---

## 21.2.35 Remoção de Backends

A remoção de um backend não deverá produzir impactos arquiteturais nos componentes compartilhados do compilador.

---

## 21.2.36 Independência da Linguagem

A evolução da linguagem Capi deverá permanecer independente da tecnologia utilizada pelos backends.

---

## 21.2.37 Evolução Tecnológica

A substituição de uma infraestrutura de geração de código por outra deverá exigir apenas a implementação da Backend API correspondente.

---

## 21.2.38 Governança

Toda alteração na Backend API deverá ser avaliada considerando seu impacto sobre:

* implementações existentes;
* estabilidade arquitetural;
* compatibilidade retroativa;
* manutenção da separação entre linguagem e implementação.

---

## 21.2.39 Invariantes

Durante toda a evolução da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* existe uma única Backend API oficial;
* toda geração de código ocorre exclusivamente através dessa API;
* nenhuma implementação concreta influencia a arquitetura da linguagem;
* todos os backends possuem exatamente as mesmas responsabilidades arquiteturais;
* componentes compartilhados permanecem independentes das tecnologias de geração de código;
* novos backends podem ser adicionados sem alterar a especificação da linguagem.

---

## 21.2.40 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma Backend API que:

* constitua o único contrato entre a IR e os backends;
* permaneça estável e tecnologicamente neutra;
* permita evolução incremental e compatibilidade sempre que tecnicamente possível;
* suporte múltiplas implementações concretas de geração de código;
* preserve integralmente a separação entre arquitetura da linguagem e infraestrutura de geração de código definida para a linguagem Capi.

---

# 21.3 Critérios para Novos Backends

## 21.3.1 Objetivo

Esta seção estabelece os critérios técnicos e arquiteturais que deverão ser atendidos para que uma nova implementação de backend possa ser considerada compatível com a arquitetura oficial da linguagem Capi.

Seu objetivo é assegurar que a inclusão de novos backends preserve integralmente os contratos definidos pela linguagem, mantendo a estabilidade da arquitetura do compilador e a equivalência funcional entre todas as implementações oficialmente suportadas.

---

## 21.3.2 Escopo

Os critérios definidos nesta seção aplicam-se a qualquer implementação de backend, independentemente da tecnologia utilizada para geração de código.

---

## 21.3.3 Princípio Geral

Um backend somente poderá ser considerado compatível com a implementação oficial quando implementar integralmente os contratos definidos pela Backend API.

---

## 21.3.4 Independência Tecnológica

A aceitação de um backend não dependerá da tecnologia utilizada para geração de código.

O critério de aceitação será exclusivamente sua conformidade com a arquitetura oficial.

---

## 21.3.5 Conformidade Arquitetural

Todo backend deverá respeitar integralmente:

* a Backend API;
* a Intermediate Representation (IR);
* a ABI oficial;
* o Runtime oficial;
* o modelo de objetos;
* a especificação da linguagem.

---

## 21.3.6 Backend API

A implementação deverá utilizar exclusivamente a Backend API como interface de integração com o compilador.

---

## 21.3.7 Intermediate Representation

A única entrada aceita pelo backend deverá ser a IR validada da linguagem.

---

## 21.3.8 Frontend

Nenhum backend poderá depender diretamente de componentes do frontend.

---

## 21.3.9 Runtime

Toda interação com o Runtime deverá ocorrer exclusivamente através das interfaces públicas definidas pela especificação.

---

## 21.3.10 ABI

O backend deverá implementar integralmente a ABI oficial da linguagem.

---

## 21.3.11 Sistema de Tipos

O backend deverá preservar integralmente o sistema de tipos definido pela linguagem.

---

## 21.3.12 Modelo de Objetos

Todo backend deverá implementar exatamente o modelo de objetos especificado para a linguagem Capi.

---

## 21.3.13 Ownership

As regras de ownership deverão permanecer integralmente preservadas.

---

## 21.3.14 Lifetimes

Os lifetimes resolvidos pelo frontend deverão ser respeitados durante toda a geração de código.

---

## 21.3.15 Domains

Domains deverão ser materializados exclusivamente através das interfaces oficiais do Runtime.

---

## 21.3.16 ObjectId

Todos os contratos relacionados ao ObjectId deverão permanecer preservados.

---

## 21.3.17 Semântica

O backend não poderá alterar o comportamento observável dos programas.

---

## 21.3.18 Determinismo

Dada a mesma IR e a mesma configuração, o backend deverá produzir código funcionalmente equivalente em todas as execuções.

---

## 21.3.19 Portabilidade

O backend deverá respeitar integralmente as regras de portabilidade definidas pela especificação da linguagem.

---

## 21.3.20 Diagnósticos

O backend deverá produzir diagnósticos claros, determinísticos e compatíveis com a infraestrutura oficial do compilador.

---

## 21.3.21 Instrumentação

O backend deverá permitir integração com os mecanismos oficiais de instrumentação.

---

## 21.3.22 Estatísticas

A implementação deverá disponibilizar estatísticas compatíveis com a infraestrutura compartilhada do compilador.

---

## 21.3.23 Testabilidade

Cada componente do backend deverá poder ser validado isoladamente.

---

## 21.3.24 Suíte Oficial de Testes

O backend deverá executar com sucesso toda a suíte oficial de testes da linguagem.

---

## 21.3.25 Testes Diferenciais

A implementação deverá participar dos testes diferenciais realizados entre backends.

---

## 21.3.26 Benchmarks

O backend deverá utilizar a infraestrutura oficial de benchmarking.

---

## 21.3.27 Compatibilidade Funcional

Diferenças entre backends somente serão aceitáveis quando restritas a:

* tempo de compilação;
* desempenho;
* consumo de memória;
* tamanho do binário;
* qualidade das otimizações.

---

## 21.3.28 Divergências

Nenhuma divergência semântica entre backends oficialmente suportados será considerada aceitável.

---

## 21.3.29 Reutilização

Sempre que possível, novos backends deverão reutilizar a infraestrutura compartilhada do compilador.

---

## 21.3.30 Encapsulamento

Toda dependência específica da tecnologia utilizada deverá permanecer encapsulada na implementação do backend.

---

## 21.3.31 Baixo Acoplamento

O backend não deverá introduzir dependências entre a infraestrutura compartilhada e tecnologias específicas de geração de código.

---

## 21.3.32 Escalabilidade

A implementação deverá permitir coexistência com os demais backends suportados.

---

## 21.3.33 Compatibilidade Evolutiva

Sempre que possível, novos backends deverão permanecer compatíveis com futuras evoluções da Backend API.

---

## 21.3.34 Estabilidade

A implementação deverá demonstrar estabilidade suficiente para utilização contínua.

---

## 21.3.35 Robustez

Falhas internas deverão ocorrer apenas em situações excepcionais e devidamente diagnosticáveis.

---

## 21.3.36 Documentação

Todo backend oficialmente suportado deverá possuir documentação técnica contendo, no mínimo:

* arquitetura;
* limitações conhecidas;
* plataformas suportadas;
* dependências;
* requisitos de compilação;
* integração com o compilador.

---

## 21.3.37 Processo de Avaliação

A inclusão de um novo backend deverá considerar conjuntamente:

* conformidade arquitetural;
* compatibilidade funcional;
* cobertura de testes;
* estabilidade;
* qualidade da implementação;
* capacidade de manutenção.

---

## 21.3.38 Aprovação

Um backend somente poderá ser considerado oficialmente suportado após atender integralmente aos critérios definidos nesta seção.

---

## 21.3.39 Classificação

A implementação oficial poderá classificar os backends em categorias, tais como:

* experimental;
* suportado;
* principal;
* legado.

Os critérios específicos para essa classificação constituem política de governança da implementação oficial e não alteram os requisitos técnicos definidos nesta especificação.

---

## 21.3.40 Evolução

A introdução de novos backends não deverá exigir alterações na arquitetura do compilador, na especificação da linguagem ou na Backend API, exceto quando estritamente necessárias para evolução geral da arquitetura.

---

## 21.3.41 Invariantes

Durante toda a evolução da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* todo backend implementa integralmente a Backend API;
* existe uma única IR oficial da linguagem;
* existe uma única ABI oficial;
* existe um único Runtime oficial;
* todos os backends preservam exatamente a mesma semântica da linguagem;
* diferenças entre implementações restringem-se exclusivamente à tecnologia empregada na geração de código;
* nenhum backend possui privilégios arquiteturais sobre os demais.

---

## 21.3.42 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir critérios objetivos que permitam avaliar qualquer novo backend quanto à:

* conformidade com a Backend API;
* preservação da semântica da linguagem;
* compatibilidade com a IR, o Runtime e a ABI oficiais;
* qualidade da implementação e estabilidade operacional;
* capacidade de coexistir com os demais backends oficialmente suportados, preservando integralmente a arquitetura da linguagem Capi.

---

# 21.4 Convivência entre Backends

## 21.4.1 Objetivo

Esta seção estabelece as regras para coexistência de múltiplos backends na implementação oficial da linguagem Capi.

Seu objetivo é permitir que diferentes implementações de geração de código coexistam de forma consistente, preservando integralmente a arquitetura do compilador e os contratos definidos pela especificação da linguagem.

---

## 21.4.2 Princípio Geral

A arquitetura da linguagem Capi deverá permitir a existência simultânea de múltiplos backends oficialmente suportados.

A existência de um backend principal não impede a utilização dos demais.

---

## 21.4.3 Independência

Cada backend deverá operar de maneira independente.

A implementação de um backend não deverá depender da existência de outro backend.

---

## 21.4.4 Equivalência Funcional

Todos os backends oficialmente suportados deverão produzir programas funcionalmente equivalentes quando compilarem a mesma Intermediate Representation (IR).

---

## 21.4.5 Backend Principal

A implementação oficial poderá definir um backend principal.

Essa classificação indica apenas a implementação recomendada para uso geral.

Ela não altera a arquitetura da linguagem.

---

## 21.4.6 Backend de Bootstrap

Durante o processo de bootstrap poderá existir um backend especificamente utilizado para viabilizar a evolução inicial da implementação oficial.

---

## 21.4.7 Backend Experimental

A implementação oficial poderá disponibilizar backends classificados como experimentais.

Backends experimentais destinam-se à validação de novas tecnologias ou abordagens de geração de código.

---

## 21.4.8 Backend de Referência

A implementação oficial poderá manter um backend de referência utilizado para:

* testes diferenciais;
* validação arquitetural;
* comparação de desempenho;
* investigação de regressões.

---

## 21.4.9 Backend Legado

Backends que deixarem de ser recomendados poderão permanecer disponíveis como implementações legadas.

Sua manutenção constitui decisão da implementação oficial.

---

## 21.4.10 Critérios de Classificação

A classificação de um backend deverá considerar fatores como:

* estabilidade;
* cobertura da linguagem;
* qualidade da geração de código;
* maturidade;
* manutenção;
* compatibilidade com a suíte oficial de testes.

---

## 21.4.11 Seleção

A seleção do backend deverá ocorrer durante a configuração da compilação.

Essa escolha não deverá exigir modificações no código-fonte da aplicação.

---

## 21.4.12 Configuração

A forma de seleção do backend constitui detalhe da implementação da ferramenta de compilação.

---

## 21.4.13 Transparência

O backend selecionado deverá ser claramente identificado durante o processo de compilação, sempre que apropriado.

---

## 21.4.14 Backend Padrão

Na ausência de configuração explícita, o compilador deverá utilizar o backend padrão definido pela implementação oficial.

---

## 21.4.15 Compatibilidade

Todos os backends oficialmente suportados deverão consumir exatamente a mesma IR produzida pelo frontend.

---

## 21.4.16 Frontend Compartilhado

Todos os backends deverão utilizar integralmente o mesmo frontend.

Não deverão existir variantes do frontend específicas para diferentes backends.

---

## 21.4.17 Runtime Compartilhado

Todos os backends deverão utilizar o Runtime oficial da linguagem.

---

## 21.4.18 ABI Compartilhada

Todos os backends deverão implementar exatamente a mesma ABI.

---

## 21.4.19 Biblioteca Padrão

A biblioteca padrão deverá apresentar comportamento funcional equivalente independentemente do backend utilizado.

---

## 21.4.20 Diagnósticos

Os diagnósticos produzidos pelos diferentes backends deverão permanecer consistentes sempre que se referirem às mesmas condições de geração de código.

---

## 21.4.21 Instrumentação

A infraestrutura de instrumentação deverá funcionar de maneira uniforme para todos os backends.

---

## 21.4.22 Benchmarks

A comparação entre backends deverá utilizar exatamente a mesma infraestrutura oficial de benchmarking.

---

## 21.4.23 Testes

Todos os backends deverão executar a mesma suíte oficial de testes.

---

## 21.4.24 Testes Diferenciais

Sempre que possível, novas funcionalidades deverão ser validadas utilizando testes diferenciais executados em todos os backends oficialmente suportados.

---

## 21.4.25 Evolução

Backends poderão evoluir em velocidades diferentes.

Entretanto, nenhum backend oficialmente suportado poderá deixar de implementar funcionalidades obrigatórias da linguagem.

---

## 21.4.26 Inclusão

A inclusão de um novo backend não deverá produzir impactos arquiteturais nos backends existentes.

---

## 21.4.27 Remoção

A remoção de um backend não deverá afetar a arquitetura compartilhada do compilador.

---

## 21.4.28 Substituição

A substituição do backend principal constitui exclusivamente uma decisão de implementação.

Ela não altera a especificação da linguagem.

---

## 21.4.29 Portabilidade

Programas escritos em Capi deverão poder ser recompilados utilizando qualquer backend oficialmente suportado, respeitadas apenas as limitações da plataforma de destino.

---

## 21.4.30 Comparabilidade

A coexistência de múltiplos backends deverá permitir comparação objetiva quanto a:

* desempenho;
* tempo de compilação;
* tamanho dos binários;
* consumo de memória;
* qualidade das otimizações.

---

## 21.4.31 Governança

A implementação oficial deverá documentar claramente o estado de maturidade de cada backend disponibilizado.

---

## 21.4.32 Descontinuação

A eventual descontinuação de um backend deverá ser precedida por comunicação adequada e período de transição compatível com a política de evolução da implementação oficial.

---

## 21.4.33 Escalabilidade

A arquitetura deverá permitir a coexistência de novos backends sem aumento significativo do acoplamento entre componentes.

---

## 21.4.34 Neutralidade

Nenhum backend deverá possuir privilégios arquiteturais.

A classificação de um backend como principal, experimental ou legado representa exclusivamente uma decisão operacional da implementação oficial.

---

## 21.4.35 Independência da Linguagem

A existência, inclusão, substituição ou remoção de backends não deverá produzir alterações na linguagem Capi nem em sua especificação.

---

## 21.4.36 Evolução Contínua

A convivência entre múltiplos backends deverá favorecer experimentação tecnológica, melhoria contínua da geração de código e validação permanente da arquitetura do compilador.

---

## 21.4.37 Invariantes

Durante toda a evolução da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* todos os backends consomem a mesma Intermediate Representation (IR);
* todos os backends implementam a mesma Backend API;
* todos os backends utilizam o mesmo Runtime oficial;
* todos os backends implementam a mesma ABI;
* todos os backends preservam integralmente a semântica da linguagem;
* a classificação operacional de um backend não altera sua posição arquitetural;
* a coexistência de múltiplos backends fortalece a robustez e a evolutividade da implementação oficial.

---

## 21.4.38 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma arquitetura que:

* permita a coexistência de múltiplos backends independentes;
* mantenha equivalência funcional entre todos os backends oficialmente suportados;
* possibilite seleção transparente do backend durante a compilação;
* preserve integralmente a arquitetura compartilhada do compilador;
* permita evolução, inclusão e substituição de backends sem qualquer impacto sobre a especificação da linguagem Capi.

---

# 21.5 Testes Diferenciais

## 21.5.1 Objetivo

Esta seção estabelece a política oficial de testes diferenciais entre os backends da implementação oficial da linguagem Capi.

Seu objetivo é assegurar que diferentes implementações de geração de código produzam comportamento funcional equivalente, permitindo identificar regressões, inconsistências arquiteturais e divergências de implementação durante toda a evolução do compilador.

---

## 21.5.2 Definição

Para os fins desta especificação, considera-se teste diferencial a execução do mesmo programa utilizando dois ou mais backends distintos, seguida da comparação sistemática de seus resultados.

---

## 21.5.3 Finalidade

Os testes diferenciais possuem como finalidade principal validar a equivalência funcional entre todos os backends oficialmente suportados.

---

## 21.5.4 Independência

Os testes diferenciais constituem infraestrutura de validação da implementação.

Eles não fazem parte da semântica da linguagem.

---

## 21.5.5 Escopo

Os testes diferenciais deverão abranger:

* geração de código;
* execução dos programas;
* comportamento observável;
* diagnósticos relacionados ao backend;
* integração com o Runtime;
* conformidade com a ABI.

---

## 21.5.6 Arquitetura

Conceitualmente, o processo poderá ser representado da seguinte forma:

```text id="ywsu4g"
          Programa Capi
                 │
                 ▼
             Frontend
                 │
                 ▼
                 IR
      ┌──────────┴──────────┐
      ▼                     ▼
Backend A              Backend B
      ▼                     ▼
Executável A          Executável B
      ▼                     ▼
 Execução A            Execução B
      └──────────┬──────────┘
                 ▼
     Comparação de Resultados
```

---

## 21.5.7 Entrada

Todos os backends deverão receber exatamente a mesma Intermediate Representation (IR).

---

## 21.5.8 Frontend Compartilhado

Os testes diferenciais deverão utilizar exatamente o mesmo frontend.

---

## 21.5.9 Runtime Compartilhado

Todos os executáveis deverão utilizar o mesmo Runtime oficial.

---

## 21.5.10 ABI Compartilhada

Todos os backends deverão utilizar exatamente a mesma ABI oficial.

---

## 21.5.11 Comparação Funcional

A principal comparação deverá considerar exclusivamente o comportamento observável dos programas.

---

## 21.5.12 Equivalência

Resultados serão considerados equivalentes quando produzirem exatamente os mesmos efeitos observáveis para as mesmas entradas.

---

## 21.5.13 Não Comparação Binária

A igualdade entre binários produzidos pelos diferentes backends não constitui requisito desta política.

Implementações distintas poderão produzir artefatos diferentes preservando integralmente a semântica da linguagem.

---

## 21.5.14 Critérios de Comparação

Entre os elementos comparados deverão estar, sempre que aplicável:

* valor de retorno;
* saída padrão;
* saída de erro;
* exceções;
* estado final observável;
* comportamento do Runtime.

---

## 21.5.15 Diagnósticos

Quando houver falhas durante a geração de código, os diagnósticos produzidos pelos diferentes backends deverão permanecer funcionalmente consistentes.

---

## 21.5.16 Casos de Teste

A suíte diferencial deverá contemplar, no mínimo:

* tipos primitivos;
* tipos compostos;
* objetos;
* herança;
* despacho virtual;
* interfaces;
* generics;
* ownership;
* lifetimes;
* Domains;
* ObjectId;
* biblioteca padrão;
* Runtime;
* FFI;
* tratamento de erros;
* concorrência, quando suportada.

---

## 21.5.17 Cobertura

A suíte oficial deverá evoluir continuamente para acompanhar a evolução da linguagem.

Toda nova funcionalidade deverá possuir testes diferenciais correspondentes.

---

## 21.5.18 Automatização

Os testes diferenciais deverão ser totalmente automatizados.

---

## 21.5.19 Reprodutibilidade

Toda execução deverá ser reprodutível utilizando a mesma versão do compilador e a mesma configuração dos backends.

---

## 21.5.20 Ambientes

Sempre que possível, os testes deverão ser executados em múltiplas plataformas suportadas.

---

## 21.5.21 Benchmarks

Os testes diferenciais poderão compartilhar infraestrutura com os benchmarks oficiais.

Entretanto, desempenho não deverá ser utilizado como critério de equivalência funcional.

---

## 21.5.22 Comparação de Desempenho

Diferenças de desempenho deverão ser registradas separadamente dos resultados funcionais.

---

## 21.5.23 Comparação de Binários

Diferenças no tamanho ou na organização dos binários gerados não caracterizam falha, desde que o comportamento observável permaneça equivalente.

---

## 21.5.24 Comparação de Otimizações

Backends poderão aplicar estratégias distintas de otimização.

Essas diferenças somente serão aceitáveis quando preservarem integralmente a semântica da linguagem.

---

## 21.5.25 Critério de Aprovação

Um backend será considerado aprovado em um teste diferencial quando produzir comportamento funcional equivalente ao backend de referência.

---

## 21.5.26 Critério de Reprovação

Qualquer divergência semântica identificada deverá ser considerada uma falha até que sua causa seja formalmente determinada.

---

## 21.5.27 Investigação

Toda divergência deverá ser analisada para determinar sua origem.

Essa investigação poderá identificar defeitos:

* no backend;
* no Runtime;
* na Backend API;
* na IR;
* na suíte de testes.

---

## 21.5.28 Regressões

Toda regressão identificada deverá originar um caso permanente na suíte oficial de testes.

---

## 21.5.29 Evolução

A suíte diferencial deverá crescer continuamente ao longo da evolução da implementação oficial.

---

## 21.5.30 Backend Principal

A promoção de um backend à condição de backend principal dependerá, entre outros critérios, do sucesso consistente na suíte oficial de testes diferenciais.

---

## 21.5.31 Novos Backends

Todo novo backend deverá demonstrar conformidade através da execução completa da suíte diferencial.

---

## 21.5.32 Ferramentas

A implementação poderá utilizar ferramentas automatizadas para comparação de resultados, coleta de métricas e geração de relatórios.

---

## 21.5.33 Instrumentação

A infraestrutura de testes diferenciais deverá permitir instrumentação para facilitar identificação de divergências.

---

## 21.5.34 Estatísticas

Entre as estatísticas produzidas poderão estar:

* número de testes executados;
* número de divergências;
* taxa de aprovação;
* tempo de execução;
* cobertura da suíte.

---

## 21.5.35 Documentação

Toda divergência conhecida entre backends deverá permanecer documentada até sua resolução ou formal classificação como limitação da implementação.

---

## 21.5.36 Independência Tecnológica

A política de testes diferenciais deverá permanecer válida independentemente das tecnologias utilizadas pelos backends comparados.

---

## 21.5.37 Escalabilidade

A arquitetura da suíte deverá permitir inclusão de novos backends sem necessidade de reestruturação significativa.

---

## 21.5.38 Neutralidade

A suíte diferencial deverá avaliar conformidade arquitetural e funcional.

Ela não deverá privilegiar qualquer backend específico além daquele temporariamente definido como referência para comparação.

---

## 21.5.39 Invariantes

Durante toda a execução dos testes diferenciais deverão permanecer verdadeiros os seguintes princípios:

* todos os backends recebem exatamente a mesma IR;
* todos utilizam o mesmo Runtime oficial;
* todos implementam a mesma ABI;
* a equivalência funcional constitui o principal critério de validação;
* diferenças de desempenho ou otimização não caracterizam divergência semântica;
* toda regressão identificada passa a integrar permanentemente a suíte oficial de testes.

---

## 21.5.40 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma infraestrutura de testes diferenciais capaz de:

* validar automaticamente todos os backends oficialmente suportados;
* comparar comportamento funcional de programas compilados por diferentes backends;
* identificar regressões arquiteturais e semânticas de forma determinística;
* fornecer métricas e diagnósticos para investigação de divergências;
* assegurar a equivalência funcional permanente entre todas as implementações de backend da linguagem Capi.

---

# 21.6 Evolução da Arquitetura de Backends

## 21.6.1 Objetivo

Esta seção estabelece os princípios que regem a evolução da arquitetura de backends da implementação oficial da linguagem Capi.

Seu objetivo é assegurar que a infraestrutura de geração de código possa evoluir continuamente sem comprometer a estabilidade da arquitetura do compilador, a compatibilidade entre implementações e os contratos definidos pela especificação da linguagem.

---

## 21.6.2 Princípio Geral

A arquitetura de backends deverá ser evolutiva.

Novas tecnologias de geração de código poderão ser incorporadas sem alterar os fundamentos arquiteturais da linguagem Capi.

---

## 21.6.3 Independência da Linguagem

A evolução dos backends não deverá produzir alterações na:

* sintaxe;
* semântica;
* sistema de tipos;
* modelo de memória;
* modelo de objetos;
* Runtime;
* ABI.

---

## 21.6.4 Backend API

Toda evolução da arquitetura deverá preservar a Backend API como contrato permanente entre o compilador e seus backends.

---

## 21.6.5 Intermediate Representation

A Intermediate Representation (IR) continuará sendo a única representação oficial da linguagem consumida pelos backends.

---

## 21.6.6 Infraestrutura Compartilhada

Sempre que possível, novas funcionalidades deverão ser incorporadas à infraestrutura compartilhada do compilador, evitando duplicação entre implementações de backend.

---

## 21.6.7 Evolução Incremental

A arquitetura deverá permitir evolução incremental dos backends.

Nenhuma implementação deverá exigir reescrita completa do compilador para incorporar novas funcionalidades.

---

## 21.6.8 Inclusão de Novos Backends

Novos backends poderão ser adicionados mediante implementação integral da Backend API e atendimento aos critérios definidos neste documento.

---

## 21.6.9 Remoção de Backends

A remoção de um backend não deverá produzir impactos arquiteturais sobre:

* frontend;
* IR;
* Runtime;
* Backend API;
* demais backends.

---

## 21.6.10 Substituição

A substituição de um backend por outro deverá ocorrer exclusivamente no nível da Backend API.

---

## 21.6.11 Compatibilidade

Durante a evolução da implementação oficial deverá ser preservada a compatibilidade funcional entre todos os backends oficialmente suportados.

---

## 21.6.12 Coexistência

A arquitetura deverá permitir a coexistência de diferentes gerações de backends durante períodos de transição.

---

## 21.6.13 Escalabilidade

A introdução de novos backends não deverá aumentar significativamente o acoplamento entre os componentes da arquitetura.

---

## 21.6.14 Modularidade

Cada backend deverá permanecer modular e autocontido.

Mudanças internas não deverão impactar outros backends.

---

## 21.6.15 Reutilização

Toda funcionalidade independente da tecnologia de geração de código deverá ser extraída para componentes compartilhados sempre que tecnicamente viável.

---

## 21.6.16 Redução de Duplicação

Duplicação significativa de código entre diferentes backends deverá motivar revisão arquitetural visando extração de infraestrutura comum.

---

## 21.6.17 Evolução da Backend API

A Backend API poderá evoluir para suportar novos requisitos arquiteturais.

Essa evolução deverá priorizar compatibilidade com implementações existentes.

---

## 21.6.18 Evolução da IR

A evolução da IR deverá ocorrer independentemente da existência de implementações específicas de backend.

Backends deverão adaptar-se às novas versões da IR através da Backend API.

---

## 21.6.19 Evolução do Runtime

Alterações no Runtime deverão permanecer independentes das tecnologias de geração de código utilizadas pelos backends.

---

## 21.6.20 Evolução da ABI

A evolução da ABI deverá considerar seu impacto sobre todos os backends oficialmente suportados.

---

## 21.6.21 Evolução do Modelo de Objetos

Mudanças no modelo de objetos deverão ser refletidas uniformemente em todos os backends.

---

## 21.6.22 Compatibilidade Retroativa

Sempre que tecnicamente possível, a evolução da arquitetura deverá preservar compatibilidade com versões anteriores da implementação oficial.

---

## 21.6.23 Depreciação

Quando um componente arquitetural for considerado obsoleto, deverá existir um processo formal de depreciação antes de sua remoção.

---

## 21.6.24 Período de Transição

Sempre que houver substituição significativa de infraestrutura, deverá existir período de coexistência suficiente para validação da nova implementação.

---

## 21.6.25 Critérios Técnicos

A adoção de novas tecnologias de geração de código deverá considerar, entre outros fatores:

* estabilidade;
* maturidade;
* manutenção;
* desempenho;
* portabilidade;
* compatibilidade arquitetural.

---

## 21.6.26 Neutralidade Tecnológica

A arquitetura não deverá favorecer permanentemente qualquer tecnologia específica.

A adoção de novas soluções deverá ocorrer com base exclusivamente em critérios técnicos.

---

## 21.6.27 Experimentação

Backends experimentais poderão ser utilizados para validação de novas abordagens arquiteturais antes de sua eventual adoção oficial.

---

## 21.6.28 Validação

Toda evolução significativa deverá ser validada através da suíte oficial de testes e dos testes diferenciais entre backends.

---

## 21.6.29 Benchmarks

Mudanças arquiteturais relevantes deverão ser acompanhadas por medições objetivas de desempenho.

---

## 21.6.30 Governança

A evolução da arquitetura deverá ocorrer de maneira documentada, permitindo rastreabilidade das decisões técnicas.

---

## 21.6.31 Documentação

Toda alteração arquitetural deverá atualizar a documentação correspondente da implementação oficial.

---

## 21.6.32 Transparência

Mudanças que afetem a arquitetura de backends deverão possuir motivação técnica claramente documentada.

---

## 21.6.33 Robustez

A arquitetura deverá favorecer implementações resilientes a mudanças tecnológicas ao longo do tempo.

---

## 21.6.34 Sustentabilidade

A evolução da arquitetura deverá priorizar soluções que reduzam complexidade de manutenção e aumentem a longevidade da implementação oficial.

---

## 21.6.35 Independência de Implementação

Nenhuma decisão tomada por um backend específico deverá tornar-se requisito obrigatório para os demais backends.

---

## 21.6.36 Preparação para o Futuro

A arquitetura deverá permanecer preparada para incorporar novas tecnologias de geração de código ainda não existentes, desde que possam implementar integralmente a Backend API e os contratos da linguagem.

---

## 21.6.37 Evolução Contínua

A arquitetura deverá ser continuamente revisada para identificar oportunidades de:

* simplificação;
* modularização;
* reutilização;
* melhoria de desempenho;
* redução de acoplamento.

---

## 21.6.38 Invariantes

Durante toda a evolução da arquitetura deverão permanecer verdadeiros os seguintes princípios:

* a linguagem Capi permanece independente da infraestrutura de geração de código;
* a Backend API constitui o contrato permanente entre o compilador e seus backends;
* existe uma única Intermediate Representation oficial;
* existe um único Runtime oficial;
* existe uma única ABI oficial;
* todos os backends implementam exatamente os mesmos contratos da linguagem;
* novas tecnologias podem ser incorporadas sem alterar a arquitetura fundamental do compilador.

---

## 21.6.39 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma arquitetura de backends que:

* permita evolução contínua e incremental;
* suporte inclusão, substituição e remoção de backends sem impacto sobre a linguagem;
* preserve integralmente a Backend API, a IR, o Runtime e a ABI oficiais;
* favoreça reutilização de infraestrutura compartilhada;
* permaneça preparada para incorporar futuras tecnologias de geração de código, mantendo a estabilidade arquitetural da linguagem Capi.

---

# 21.7 Critérios Normativos Finais

## 21.7.1 Objetivo

Esta seção consolida os princípios normativos estabelecidos ao longo deste capítulo, definindo os critérios finais que deverão reger permanentemente a arquitetura de backends da implementação oficial da linguagem Capi.

Seu objetivo é sintetizar os contratos arquiteturais que garantem a estabilidade, a evolutividade e a independência tecnológica da infraestrutura de geração de código.

---

## 21.7.2 Escopo

Os critérios definidos nesta seção aplicam-se a todos os backends oficialmente suportados, presentes e futuros.

---

## 21.7.3 Unidade Arquitetural

A implementação oficial deverá possuir uma única arquitetura de geração de código, independentemente da quantidade de backends existentes.

---

## 21.7.4 Backend API

A Backend API constitui o único contrato oficial entre a infraestrutura compartilhada do compilador e os backends.

Nenhuma implementação poderá estabelecer interfaces paralelas.

---

## 21.7.5 Intermediate Representation

A Intermediate Representation (IR) constitui a única representação oficial consumida pelos backends.

---

## 21.7.6 Frontend

Todos os backends deverão compartilhar exatamente o mesmo frontend.

---

## 21.7.7 Runtime

Todos os backends deverão utilizar o Runtime oficial da linguagem.

---

## 21.7.8 ABI

Todos os backends deverão implementar exatamente a mesma ABI oficial.

---

## 21.7.9 Modelo de Objetos

Todos os backends deverão preservar integralmente o modelo de objetos definido pela especificação da linguagem.

---

## 21.7.10 Sistema de Tipos

Todos os backends deverão preservar integralmente o sistema de tipos da linguagem Capi.

---

## 21.7.11 Modelo de Memória

As garantias relacionadas ao modelo de memória deverão permanecer inalteradas independentemente do backend utilizado.

---

## 21.7.12 Ownership

As regras de ownership deverão permanecer semanticamente idênticas em todas as implementações.

---

## 21.7.13 Lifetimes

Os lifetimes resolvidos pelo frontend deverão ser integralmente respeitados por todos os backends.

---

## 21.7.14 Domains

A implementação dos Domains deverá ocorrer exclusivamente através do Runtime oficial.

---

## 21.7.15 ObjectId

Todos os contratos relacionados ao ObjectId deverão permanecer invariáveis em todos os backends.

---

## 21.7.16 Neutralidade Tecnológica

A arquitetura da linguagem não deverá depender permanentemente de qualquer infraestrutura específica de geração de código.

---

## 21.7.17 Independência da Linguagem

A evolução dos backends não deverá modificar a especificação da linguagem.

---

## 21.7.18 Independência da Implementação

A evolução da linguagem não deverá depender da tecnologia utilizada por qualquer backend específico.

---

## 21.7.19 Equivalência Funcional

Todos os backends oficialmente suportados deverão produzir comportamento funcional equivalente.

---

## 21.7.20 Testes Diferenciais

A equivalência funcional deverá ser continuamente validada através da suíte oficial de testes diferenciais.

---

## 21.7.21 Reutilização

Toda funcionalidade independente da tecnologia utilizada deverá permanecer na infraestrutura compartilhada do compilador.

---

## 21.7.22 Encapsulamento

Detalhes específicos de cada tecnologia de geração de código deverão permanecer encapsulados exclusivamente na implementação do backend correspondente.

---

## 21.7.23 Modularidade

Cada backend deverá permanecer modular, autocontido e substituível.

---

## 21.7.24 Baixo Acoplamento

Nenhuma implementação de backend poderá introduzir dependências permanentes na arquitetura compartilhada.

---

## 21.7.25 Escalabilidade

A arquitetura deverá permitir inclusão de novos backends sem alterações significativas na infraestrutura existente.

---

## 21.7.26 Evolução

A arquitetura deverá favorecer evolução incremental da infraestrutura de geração de código.

---

## 21.7.27 Compatibilidade

Sempre que tecnicamente possível, a evolução da Backend API deverá preservar compatibilidade com implementações existentes.

---

## 21.7.28 Governança

Toda alteração arquitetural deverá ser documentada e fundamentada em critérios técnicos.

---

## 21.7.29 Documentação

Todo backend oficialmente suportado deverá possuir documentação compatível com os padrões definidos pela implementação oficial.

---

## 21.7.30 Portabilidade

A arquitetura deverá permitir que programas escritos em Capi sejam recompilados utilizando qualquer backend oficialmente suportado, observadas apenas as limitações da plataforma de destino.

---

## 21.7.31 Critérios para Novos Backends

A inclusão de novos backends dependerá exclusivamente do atendimento integral aos contratos arquiteturais definidos nesta especificação.

---

## 21.7.32 Critérios para Descontinuação

A remoção de um backend não deverá produzir alterações na arquitetura compartilhada do compilador nem na especificação da linguagem.

---

## 21.7.33 Backend Principal

A existência de um backend principal constitui decisão operacional da implementação oficial.

Ela não altera a arquitetura nem os contratos definidos pela linguagem.

---

## 21.7.34 Backend Experimental

Backends experimentais poderão coexistir com implementações oficialmente suportadas, desde que sua condição seja claramente documentada.

---

## 21.7.35 Futuras Tecnologias

A arquitetura deverá permanecer preparada para incorporar futuras tecnologias de geração de código que ainda não existam, desde que possam implementar integralmente os contratos estabelecidos pela Backend API.

---

## 21.7.36 Princípio da Separação entre Garantias e Implementação

Os contratos definidos pela especificação da linguagem representam garantias permanentes da arquitetura.

As tecnologias utilizadas para implementar esses contratos constituem detalhes de implementação e poderão evoluir livremente, desde que preservem integralmente o comportamento especificado.

---

## 21.7.37 Invariantes

Durante toda a evolução da implementação oficial da linguagem Capi deverão permanecer verdadeiros os seguintes princípios:

* existe uma única arquitetura oficial de geração de código;
* existe uma única Backend API oficial;
* existe uma única Intermediate Representation oficial;
* existe um único Runtime oficial;
* existe uma única ABI oficial;
* todos os backends implementam exatamente os mesmos contratos da linguagem;
* todos os backends preservam integralmente a semântica da linguagem;
* a inclusão, substituição ou remoção de backends não altera a especificação da linguagem;
* a arquitetura permanece tecnologicamente neutra;
* a separação entre garantias da linguagem e detalhes de implementação é preservada permanentemente.

---

## 21.7.38 Encerramento do Capítulo

Este capítulo estabelece a arquitetura normativa para coexistência, evolução e governança dos backends da implementação oficial da linguagem Capi.

A partir dos princípios aqui definidos, qualquer infraestrutura de geração de código poderá ser integrada à implementação oficial, desde que implemente integralmente a Backend API, respeite a Intermediate Representation, preserve a ABI, utilize o Runtime oficial e mantenha invariáveis todos os contratos definidos pela especificação da linguagem.

Dessa forma, a evolução tecnológica da implementação oficial permanece desacoplada da evolução da linguagem, assegurando estabilidade arquitetural, portabilidade, modularidade e sustentabilidade de longo prazo para o ecossistema da linguagem Capi.

---

# 22 Estratégia de Desenvolvimento Incremental

## 22.1 Objetivo

Esta seção estabelece a estratégia oficial de desenvolvimento incremental da implementação oficial da linguagem Capi.

Seu objetivo é definir os princípios que orientam a evolução do compilador desde sua primeira implementação até a auto-hospedagem, assegurando que cada etapa produza incrementos funcionais, verificáveis e compatíveis com a arquitetura definida por esta especificação.

---

## 22.2 Escopo

A estratégia definida nesta seção aplica-se ao desenvolvimento de todos os componentes da implementação oficial, incluindo:

* frontend;
* middle-end;
* backend;
* Runtime;
* biblioteca padrão;
* toolchain;
* infraestrutura de testes;
* ferramentas auxiliares.

---

## 22.3 Princípio Geral

A implementação oficial deverá evoluir por meio de incrementos pequenos, completos e continuamente verificáveis.

Cada incremento deverá representar uma evolução funcional do compilador.

---

## 22.4 Desenvolvimento Incremental

O desenvolvimento deverá ser organizado em etapas sucessivas, nas quais cada etapa acrescenta novas capacidades à implementação existente sem comprometer as funcionalidades já consolidadas.

---

## 22.5 Funcionalidade Completa

Sempre que possível, cada incremento deverá entregar funcionalidades completas e utilizáveis, ainda que limitadas em escopo.

Implementações parcialmente funcionais deverão ser evitadas quando puderem comprometer a estabilidade da arquitetura.

---

## 22.6 Ordem de Implementação

A ordem de desenvolvimento deverá respeitar as dependências naturais entre os componentes do compilador.

Em particular, componentes de baixo nível deverão ser implementados antes daqueles que deles dependem.

---

## 22.7 Evolução Contínua

A implementação deverá permanecer continuamente evolutiva.

Não deverão existir longos períodos sem uma versão funcional capaz de ser compilada e validada.

---

## 22.8 Estabilidade

Ao término de cada incremento, o compilador deverá permanecer em estado consistente e executável.

---

## 22.9 Regressões

Nenhuma funcionalidade previamente validada deverá deixar de funcionar em decorrência da implementação de novas capacidades.

---

## 22.10 Integração Contínua

Novos componentes deverão ser integrados continuamente à implementação oficial.

Integrações de grande porte realizadas apenas ao final de longos períodos de desenvolvimento deverão ser evitadas.

---

## 22.11 Desenvolvimento Guiado pela Arquitetura

A implementação deverá seguir rigorosamente a arquitetura definida pelos Documentos 13 a 27.

Decisões de implementação não deverão modificar os contratos arquiteturais estabelecidos pela especificação.

---

## 22.12 Desenvolvimento Guiado pela Especificação

A implementação oficial deverá ser conduzida pela especificação da linguagem.

Sempre que houver conflito entre a implementação e a especificação, prevalecerá a especificação.

---

## 22.13 Separação entre Garantias e Implementação

O desenvolvimento incremental deverá preservar continuamente o Princípio da Separação entre Garantias e Implementação.

Incrementos poderão modificar detalhes internos da implementação, desde que mantenham inalteradas as garantias definidas pela linguagem.

---

## 22.14 Critérios para Introdução de Funcionalidades

Novas funcionalidades somente deverão ser incorporadas quando:

* possuírem especificação definida;
* apresentarem integração compatível com a arquitetura;
* puderem ser adequadamente testadas;
* não introduzirem inconsistências arquiteturais.

---

## 22.15 Priorização

A implementação deverá priorizar funcionalidades fundamentais para o funcionamento do compilador antes da implementação de otimizações ou funcionalidades acessórias.

---

## 22.16 Oportunidade das Otimizações

Otimizações deverão ser introduzidas apenas após a estabilização da funcionalidade correspondente.

A corretude deverá sempre possuir prioridade sobre o desempenho.

---

## 22.17 Refatorações

Refatorações poderão ocorrer continuamente, desde que:

* preservem o comportamento funcional;
* reduzam a complexidade;
* aumentem a legibilidade;
* fortaleçam a arquitetura.

---

## 22.18 Reutilização

Sempre que possível, componentes desenvolvidos deverão ser reutilizados em novas etapas da implementação.

Duplicação significativa de código deverá ser evitada.

---

## 22.19 Modularidade

Cada incremento deverá preservar a modularidade da implementação.

Novas funcionalidades deverão ser incorporadas preferencialmente através da extensão de módulos existentes ou da criação de novos módulos independentes.

---

## 22.20 Verificabilidade

Todo incremento deverá ser objetivamente verificável através da suíte oficial de testes.

---

## 22.21 Documentação

Cada incremento deverá ser acompanhado da documentação técnica correspondente.

A documentação constitui parte integrante da entrega.

---

## 22.22 Revisão

Toda funcionalidade implementada deverá ser revisada antes de ser considerada concluída.

---

## 22.23 Critérios para Conclusão de um Incremento

Um incremento somente será considerado concluído quando:

* implementar integralmente seu escopo;
* compilar corretamente;
* executar com sucesso sua suíte de testes;
* preservar a estabilidade da implementação;
* possuir documentação correspondente.

---

## 22.24 Dependências

Incrementos deverão respeitar explicitamente suas dependências técnicas.

Nenhuma funcionalidade deverá depender de componentes ainda inexistentes.

---

## 22.25 Acoplamento

O desenvolvimento incremental deverá minimizar o acoplamento entre componentes.

---

## 22.26 Complexidade

Incrementos deverão manter complexidade compatível com sua finalidade.

Sempre que possível, grandes funcionalidades deverão ser divididas em etapas menores.

---

## 22.27 Evolução Arquitetural

Mudanças arquiteturais significativas deverão ocorrer apenas quando tecnicamente justificadas e compatíveis com a especificação da linguagem.

---

## 22.28 Compatibilidade

Toda evolução deverá preservar compatibilidade com os componentes anteriormente estabilizados, sempre que tecnicamente possível.

---

## 22.29 Preparação para o Bootstrap

Cada incremento deverá aproximar progressivamente a implementação oficial da capacidade de realizar o bootstrap completo.

---

## 22.30 Preparação para a Auto-hospedagem

A estratégia incremental deverá conduzir naturalmente à futura migração da implementação em Rust para a implementação escrita em Capi.

---

## 22.31 Métricas

A evolução da implementação deverá ser acompanhada por métricas objetivas relacionadas à:

* cobertura de testes;
* estabilidade;
* desempenho;
* qualidade do código;
* evolução funcional.

---

## 22.32 Governança

A priorização dos incrementos deverá considerar simultaneamente:

* dependências técnicas;
* estabilidade arquitetural;
* impacto funcional;
* custo de implementação;
* benefícios para o processo de bootstrap.

---

## 22.33 Flexibilidade

A ordem dos incrementos poderá ser ajustada sempre que necessário, desde que sejam preservadas:

* a arquitetura oficial;
* as dependências técnicas;
* os contratos definidos pela especificação.

---

## 22.34 Sustentabilidade

A estratégia de desenvolvimento deverá privilegiar decisões que reduzam o custo de manutenção ao longo do ciclo de vida da implementação oficial.

---

## 22.35 Evolução Contínua

O processo de desenvolvimento deverá favorecer melhoria contínua da implementação, permitindo evolução progressiva sem necessidade de reestruturações completas.

---

## 22.36 Invariantes

Durante toda a evolução da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* o desenvolvimento ocorre de forma incremental;
* cada incremento representa uma funcionalidade completa e verificável;
* a especificação da linguagem prevalece sobre decisões de implementação;
* a arquitetura permanece estável ao longo da evolução do compilador;
* corretude possui prioridade sobre otimização;
* cada incremento aproxima a implementação da auto-hospedagem.

---

## 22.37 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial adotar uma estratégia de desenvolvimento que:

* evolua continuamente por incrementos funcionais;
* preserve integralmente a arquitetura especificada;
* mantenha estabilidade entre as etapas de desenvolvimento;
* permita validação objetiva de cada incremento;
* conduza progressivamente a implementação oficial até o bootstrap completo e a futura auto-hospedagem da linguagem Capi.

---

# 23 Marcos da Implementação (Milestones)

## 23.1 Objetivo

Esta seção estabelece os marcos oficiais (Milestones) da implementação oficial da linguagem Capi.

Seu objetivo é organizar a evolução do compilador em etapas sucessivas, claramente definidas e objetivamente verificáveis, permitindo acompanhar o progresso do bootstrap até a auto-hospedagem da linguagem.

---

## 23.2 Definição

Para os fins desta especificação, um **Milestone** representa um marco de desenvolvimento correspondente à conclusão de um conjunto coerente de funcionalidades da implementação oficial.

Cada Milestone deverá produzir uma evolução funcional do compilador.

---

## 23.3 Características

Todo Milestone deverá possuir:

* objetivo claramente definido;
* escopo delimitado;
* critérios objetivos de aceitação;
* dependências explicitamente identificadas;
* validação por testes;
* documentação correspondente.

---

## 23.4 Sequenciamento

Os Milestones deverão respeitar a ordem natural de construção da arquitetura do compilador.

Um Milestone somente poderá depender de funcionalidades implementadas em Milestones anteriores.

---

## 23.5 Evolução Incremental

Cada Milestone deverá representar uma evolução incremental da implementação oficial.

A conclusão de um Milestone não deverá exigir reestruturação significativa dos componentes anteriormente estabilizados.

---

## 23.6 M0 — Infraestrutura do Projeto

O primeiro Milestone deverá estabelecer a infraestrutura mínima necessária para o desenvolvimento do compilador.

Entre seus objetivos incluem-se:

* organização do projeto;
* sistema de build;
* gerenciamento de dependências;
* estrutura de módulos;
* infraestrutura inicial de testes;
* integração contínua;
* documentação básica.

---

## 23.7 M1 — Análise Léxica

Este Milestone deverá concluir a implementação do analisador léxico da linguagem.

Ao seu término, o compilador deverá ser capaz de produzir corretamente a sequência de tokens definida pela especificação léxica.

---

## 23.8 M2 — Análise Sintática

Este Milestone deverá concluir a implementação do parser.

Ao seu término, programas válidos deverão ser convertidos em árvores sintáticas (AST) compatíveis com a especificação da linguagem.

---

## 23.9 M3 — Árvore Sintática Abstrata

Este Milestone deverá consolidar a representação oficial da AST utilizada pelo frontend.

---

## 23.10 M4 — High-level Intermediate Representation (HIR)

Este Milestone deverá concluir a transformação da AST para a HIR, estabelecendo a representação utilizada pelas etapas semânticas do compilador.

---

## 23.11 M5 — Resolução de Nomes

Este Milestone deverá concluir:

* resolução de identificadores;
* escopos;
* namespaces;
* imports;
* referências entre módulos.

---

## 23.12 M6 — Inferência e Verificação de Tipos

Este Milestone deverá concluir o sistema de inferência e verificação de tipos da linguagem.

Ao seu término, toda a semântica estática da linguagem deverá estar operacional.

---

## 23.13 M7 — Intermediate Representation (IR)

Este Milestone deverá concluir a geração da Intermediate Representation oficial da linguagem.

A IR produzida deverá constituir a única entrada para todos os backends.

---

## 23.14 M8 — Runtime

Este Milestone deverá concluir o Runtime mínimo definido pela especificação.

Entre os componentes esperados incluem-se:

* gerenciamento de Domains;
* ObjectId;
* suporte ao modelo de objetos;
* infraestrutura básica de execução.

---

## 23.15 M9 — Biblioteca Padrão

Este Milestone deverá concluir o subconjunto inicial da biblioteca padrão necessário para o bootstrap da linguagem.

---

## 23.16 M10 — Backend Cranelift

Este Milestone deverá concluir a primeira implementação funcional de geração de código utilizando Cranelift.

Ao seu término, o compilador deverá ser capaz de produzir executáveis compatíveis com a arquitetura suportada.

---

## 23.17 M11 — Backend LLVM

Este Milestone deverá concluir a implementação do backend baseado em LLVM.

A partir deste marco, a implementação oficial passará a dispor de múltiplos backends oficialmente suportados.

---

## 23.18 M12 — Bootstrap Completo

Este Milestone deverá concluir toda a implementação inicial em Rust.

Ao seu término, o compilador deverá implementar integralmente a especificação da linguagem e estar preparado para iniciar o processo de auto-hospedagem.

---

## 23.19 Dependências

Cada Milestone deverá possuir dependências compatíveis com a arquitetura definida neste documento.

Não deverão existir dependências circulares entre Milestones.

---

## 23.20 Critérios de Conclusão

A conclusão de um Milestone dependerá do atendimento integral aos critérios definidos no capítulo seguinte.

---

## 23.21 Revisão

Todo Milestone concluído deverá ser submetido à revisão técnica antes de ser considerado estabilizado.

---

## 23.22 Validação

Cada Milestone deverá ser validado através da suíte oficial de testes correspondente ao seu escopo.

---

## 23.23 Documentação

A documentação técnica deverá evoluir conjuntamente com cada Milestone.

Nenhum marco será considerado concluído sem documentação compatível.

---

## 23.24 Estabilidade

Após a conclusão de um Milestone, a implementação deverá permanecer estável durante o desenvolvimento dos marcos subsequentes.

---

## 23.25 Correções

Correções de defeitos poderão ocorrer continuamente sem alterar a definição arquitetural dos Milestones.

---

## 23.26 Evolução

Novos Milestones poderão ser adicionados futuramente para refletir a evolução da implementação oficial.

Essa inclusão não deverá alterar a sequência lógica dos marcos fundamentais definidos nesta seção.

---

## 23.27 Governança

A criação, divisão, fusão ou redefinição de Milestones constitui decisão de governança da implementação oficial.

Essas alterações deverão preservar os contratos arquiteturais definidos pela especificação.

---

## 23.28 Flexibilidade

A implementação oficial poderá subdividir internamente um Milestone em etapas menores para fins de planejamento, desde que essa subdivisão não altere os objetivos normativos estabelecidos nesta seção.

---

## 23.29 Rastreabilidade

Toda funcionalidade implementada deverá poder ser associada ao Milestone correspondente.

Essa rastreabilidade deverá facilitar o acompanhamento da evolução da implementação oficial.

---

## 23.30 Progresso

O avanço da implementação oficial deverá ser medido pela conclusão sucessiva dos Milestones definidos nesta seção, e não pela quantidade de código produzido.

---

## 23.31 Invariantes

Durante toda a evolução da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* cada Milestone representa uma evolução funcional do compilador;
* os Milestones respeitam as dependências arquiteturais da implementação;
* nenhum Milestone compromete a estabilidade dos anteriores;
* toda funcionalidade implementada possui rastreabilidade até um Milestone;
* cada Milestone é objetivamente verificável;
* a sequência dos Milestones conduz progressivamente ao bootstrap completo da implementação oficial.

---

## 23.32 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um conjunto de Milestones que:

* organize a evolução do compilador em etapas sucessivas;
* permita medir objetivamente o progresso do desenvolvimento;
* respeite a arquitetura definida para a implementação oficial;
* produza incrementos funcionais e verificáveis;
* conduza de forma estruturada a implementação oficial até o bootstrap completo da linguagem Capi.

---

# 24 Critérios de Aceitação dos Marcos

## 24.1 Objetivo

Esta seção estabelece os critérios normativos para aceitação dos Milestones definidos no capítulo anterior.

Seu objetivo é assegurar que a conclusão de cada etapa da implementação oficial seja determinada por critérios técnicos, objetivos e verificáveis, garantindo a estabilidade do compilador durante todo o processo de bootstrap.

---

## 24.2 Escopo

Os critérios definidos nesta seção aplicam-se a todos os Milestones da implementação oficial, independentemente de seu porte ou complexidade.

---

## 24.3 Princípio Geral

Um Milestone somente poderá ser considerado concluído quando atender integralmente aos critérios de aceitação definidos nesta seção.

A implementação parcial de funcionalidades não caracteriza a conclusão de um Milestone.

---

## 24.4 Funcionalidade

Todas as funcionalidades previstas para o Milestone deverão estar integralmente implementadas.

Não deverão existir funcionalidades obrigatórias pendentes.

---

## 24.5 Conformidade com a Especificação

Toda funcionalidade implementada deverá estar em conformidade com a especificação oficial da linguagem Capi.

Em caso de divergência, prevalecerá sempre a especificação.

---

## 24.6 Compilação

O compilador deverá ser compilado com sucesso em todas as plataformas oficialmente suportadas pelo respectivo Milestone.

---

## 24.7 Execução

Os componentes implementados deverão executar corretamente em condições normais de utilização.

---

## 24.8 Estabilidade

O Milestone deverá apresentar comportamento estável durante sua utilização.

Falhas conhecidas que comprometam seu objetivo impedirão sua aprovação.

---

## 24.9 Testes Unitários

Todos os testes unitários relacionados ao escopo do Milestone deverão ser executados com sucesso.

---

## 24.10 Testes de Integração

Todos os testes de integração aplicáveis deverão ser executados com sucesso.

---

## 24.11 Testes Funcionais

As funcionalidades implementadas deverão ser validadas por meio da suíte oficial de testes funcionais.

---

## 24.12 Testes Diferenciais

Sempre que aplicável, as funcionalidades deverão ser validadas através dos testes diferenciais entre os backends oficialmente suportados.

---

## 24.13 Ausência de Regressões

O Milestone não deverá introduzir regressões em funcionalidades anteriormente estabilizadas.

---

## 24.14 Compatibilidade Arquitetural

A implementação deverá preservar integralmente a arquitetura definida nos documentos da implementação oficial.

---

## 24.15 Compatibilidade com a Backend API

Quando aplicável, o Milestone deverá preservar integralmente os contratos definidos pela Backend API.

---

## 24.16 Compatibilidade com a IR

Quando aplicável, a Intermediate Representation deverá permanecer compatível com os contratos arquiteturais estabelecidos.

---

## 24.17 Compatibilidade com o Runtime

Nenhuma funcionalidade implementada deverá comprometer os contratos do Runtime oficial.

---

## 24.18 Compatibilidade com a ABI

Sempre que aplicável, a implementação deverá preservar integralmente a ABI oficial da linguagem.

---

## 24.19 Qualidade do Código

A implementação deverá apresentar organização compatível com os padrões definidos pela implementação oficial.

---

## 24.20 Modularidade

Os componentes implementados deverão preservar a modularidade da arquitetura.

---

## 24.21 Manutenibilidade

A implementação deverá favorecer manutenção futura, evitando acoplamento excessivo, duplicação desnecessária de código e complexidade injustificada.

---

## 24.22 Documentação

Toda funcionalidade implementada deverá possuir documentação técnica compatível com seu escopo.

A ausência de documentação impede a conclusão do Milestone.

---

## 24.23 Diagnósticos

Sempre que aplicável, diagnósticos produzidos pelo compilador deverão ser claros, determinísticos e compatíveis com a especificação.

---

## 24.24 Benchmarks

Quando o Milestone envolver alterações com impacto potencial no desempenho, deverão ser executados os benchmarks oficiais da implementação.

---

## 24.25 Métricas

Sempre que possível, deverão ser registradas métricas objetivas relacionadas ao Milestone, incluindo:

* cobertura de testes;
* tempo de compilação;
* consumo de memória;
* desempenho;
* tamanho do binário;
* evolução funcional.

---

## 24.26 Revisão Técnica

Todo Milestone deverá ser submetido à revisão técnica antes de sua aprovação.

---

## 24.27 Aprovação

A conclusão de um Milestone dependerá da aprovação formal segundo o processo de governança da implementação oficial.

---

## 24.28 Correções

Defeitos identificados durante o processo de aceitação deverão ser corrigidos antes da aprovação do Milestone.

---

## 24.29 Reabertura

Caso sejam identificadas falhas críticas após sua aprovação, um Milestone poderá ser reaberto para correção.

Essa reabertura não altera a sequência arquitetural definida para a implementação oficial.

---

## 24.30 Critério para Evolução

A implementação somente deverá iniciar um novo Milestone após a estabilização do Milestone anterior, salvo quando atividades paralelas forem tecnicamente independentes.

---

## 24.31 Exceções

Exceções aos critérios definidos nesta seção deverão ser raras, tecnicamente justificadas e formalmente registradas.

---

## 24.32 Rastreabilidade

Toda funcionalidade aceita deverá permanecer rastreável ao Milestone correspondente, permitindo auditoria da evolução da implementação oficial.

---

## 24.33 Transparência

Os resultados do processo de aceitação deverão ser documentados de forma clara, permitindo identificar:

* critérios avaliados;
* evidências produzidas;
* pendências identificadas;
* decisão de aprovação ou rejeição.

---

## 24.34 Evolução

Os critérios definidos nesta seção poderão evoluir ao longo da implementação oficial, desde que permaneçam objetivos, verificáveis e compatíveis com a arquitetura da linguagem.

---

## 24.35 Governança

A definição e eventual atualização dos critérios de aceitação constituem responsabilidade da governança da implementação oficial.

Alterações não deverão reduzir o nível de qualidade exigido para aprovação dos Milestones.

---

## 24.36 Invariantes

Durante todo o processo de desenvolvimento da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* nenhum Milestone é considerado concluído sem atender integralmente aos seus critérios de aceitação;
* a conformidade com a especificação prevalece sobre decisões de implementação;
* toda funcionalidade implementada é validada por testes objetivos;
* regressões impedem a aprovação de um Milestone;
* documentação faz parte integrante da entrega;
* a evolução da implementação ocorre apenas sobre uma base previamente estabilizada.

---

## 24.37 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um processo de aceitação de Milestones que:

* utilize critérios técnicos objetivos e verificáveis;
* assegure conformidade com a especificação da linguagem;
* preserve a estabilidade arquitetural do compilador;
* impeça a progressão da implementação sobre funcionalidades incompletas ou instáveis;
* garanta que cada Milestone represente uma evolução funcional plenamente validada da implementação oficial da linguagem Capi.

---

# 25 Estratégia de Migração

## 25.1 Objetivo

Esta seção estabelece a estratégia oficial para migração da implementação inicial do compilador da linguagem Capi, desenvolvida em Rust, para sua implementação escrita na própria linguagem Capi.

Seu objetivo é assegurar que essa transição ocorra de forma incremental, segura, verificável e compatível com toda a arquitetura definida nesta especificação.

---

## 25.2 Escopo

A estratégia definida nesta seção aplica-se exclusivamente à implementação oficial do compilador da linguagem Capi.

Ela não estabelece requisitos para implementações alternativas do compilador.

---

## 25.3 Princípio Geral

A migração deverá ocorrer de forma gradual.

A implementação escrita em Capi deverá substituir progressivamente os componentes originalmente desenvolvidos em Rust, preservando continuamente a estabilidade da implementação oficial.

---

## 25.4 Bootstrap como Meio

A implementação em Rust constitui um mecanismo de bootstrap.

Ela não representa o estado final da implementação oficial.

---

## 25.5 Auto-hospedagem como Objetivo

A implementação oficial deverá evoluir progressivamente até que o compilador possa ser desenvolvido, compilado e mantido utilizando a própria linguagem Capi.

---

## 25.6 Preservação da Arquitetura

A migração não deverá modificar:

* a arquitetura do compilador;
* a Backend API;
* a Intermediate Representation (IR);
* o Runtime;
* a ABI;
* os contratos definidos pela especificação da linguagem.

---

## 25.7 Independência da Especificação

A migração constitui um detalhe da implementação oficial.

Ela não altera a especificação da linguagem Capi.

---

## 25.8 Componentes Elegíveis

Poderão ser migrados para Capi todos os componentes da implementação oficial, incluindo:

* frontend;
* middle-end;
* backend;
* Runtime;
* biblioteca padrão;
* toolchain;
* ferramentas auxiliares.

---

## 25.9 Ordem da Migração

A ordem de migração deverá respeitar as dependências arquiteturais entre os componentes.

Sempre que possível, componentes de menor dependência deverão ser migrados antes daqueles que deles dependem.

---

## 25.10 Migração Incremental

A substituição dos componentes deverá ocorrer individualmente ou em pequenos conjuntos coerentes.

Grandes reescritas simultâneas deverão ser evitadas.

---

## 25.11 Coexistência

Durante o processo de migração poderá existir coexistência entre componentes implementados em Rust e componentes implementados em Capi.

Essa coexistência deverá ser considerada temporária.

---

## 25.12 Compatibilidade

Componentes migrados deverão permanecer totalmente compatíveis com aqueles ainda implementados em Rust.

---

## 25.13 Interoperabilidade

Durante a migração, mecanismos de interoperabilidade poderão ser utilizados para permitir comunicação entre componentes escritos em linguagens diferentes.

Esses mecanismos constituem detalhes de implementação.

---

## 25.14 Critério para Substituição

Um componente somente poderá substituir sua implementação anterior quando demonstrar equivalência funcional através da suíte oficial de testes.

---

## 25.15 Reutilização

Sempre que possível, algoritmos, estruturas e decisões arquiteturais da implementação em Rust deverão ser preservados durante a migração.

---

## 25.16 Refatoração

A migração não deverá ser utilizada como justificativa para alterações arquiteturais desnecessárias.

Refatorações deverão possuir motivação técnica claramente identificável.

---

## 25.17 Estabilidade

Após cada etapa da migração, a implementação oficial deverá permanecer:

* compilável;
* executável;
* testável;
* funcional.

---

## 25.18 Testes

Cada componente migrado deverá executar integralmente sua suíte oficial de testes antes de substituir sua implementação anterior.

---

## 25.19 Testes Diferenciais

Sempre que tecnicamente aplicável, a implementação em Rust e sua correspondente implementação em Capi deverão ser comparadas através de testes diferenciais durante o período de coexistência.

---

## 25.20 Equivalência Funcional

A migração somente será considerada bem-sucedida quando o comportamento observável do componente migrado permanecer equivalente ao da implementação original.

---

## 25.21 Desempenho

Diferenças de desempenho entre as implementações não impedirão a migração, desde que:

* a funcionalidade permaneça correta;
* a estabilidade seja preservada;
* os requisitos mínimos da implementação oficial continuem atendidos.

---

## 25.22 Correções

Correções de defeitos identificados durante a migração deverão ser aplicadas preferencialmente na implementação em Capi.

A manutenção da implementação em Rust deverá restringir-se ao necessário para conclusão do processo de migração.

---

## 25.23 Sincronização

Durante o período de coexistência deverá ser evitada divergência funcional entre as implementações em Rust e em Capi.

---

## 25.24 Documentação

Cada etapa da migração deverá possuir documentação técnica correspondente, registrando:

* componentes migrados;
* dependências;
* limitações conhecidas;
* resultados da validação.

---

## 25.25 Critérios para Remoção

Uma implementação em Rust somente poderá ser removida quando:

* sua substituição em Capi estiver concluída;
* todos os testes forem aprovados;
* a estabilidade da implementação estiver comprovada;
* não existirem dependências remanescentes.

---

## 25.26 Flexibilidade

A ordem de migração poderá ser ajustada conforme necessidades técnicas identificadas durante a evolução da implementação oficial.

Esses ajustes não deverão comprometer a arquitetura definida por esta especificação.

---

## 25.27 Reversibilidade

Sempre que tecnicamente possível, etapas individuais da migração deverão permitir reversão caso sejam identificadas falhas críticas.

---

## 25.28 Governança

A decisão de iniciar, suspender ou concluir a migração de um componente constitui responsabilidade da governança da implementação oficial.

---

## 25.29 Sustentabilidade

A estratégia de migração deverá reduzir progressivamente a dependência da implementação inicial em Rust, preparando a implementação oficial para sua manutenção integral em Capi.

---

## 25.30 Encerramento da Migração

A migração será considerada encerrada quando todos os componentes previstos para a implementação oficial estiverem implementados em Capi e não existirem dependências funcionais da implementação inicial em Rust.

---

## 25.31 Evolução Posterior

Após o encerramento da migração, a implementação oficial continuará sujeita ao processo normal de evolução, manutenção e melhoria contínua.

A auto-hospedagem não representa o término da evolução da implementação.

---

## 25.32 Invariantes

Durante toda a estratégia de migração deverão permanecer verdadeiros os seguintes princípios:

* a implementação em Rust constitui apenas o bootstrap da linguagem;
* a especificação da linguagem permanece inalterada durante toda a migração;
* a arquitetura do compilador permanece preservada;
* componentes podem coexistir temporariamente em Rust e Capi;
* toda substituição é precedida por validação objetiva;
* a migração ocorre de forma incremental, segura e verificável;
* a implementação oficial evolui continuamente em direção à auto-hospedagem completa.

---

## 25.33 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma estratégia de migração que:

* conduza progressivamente a implementação do compilador de Rust para Capi;
* preserve integralmente a arquitetura definida pela especificação;
* permita coexistência temporária entre implementações durante a transição;
* assegure equivalência funcional por meio de validação objetiva;
* prepare a implementação oficial para o processo de auto-hospedagem definido nos capítulos seguintes.

---

# 26 Processo de Auto-hospedagem

## 26.1 Objetivo

Esta seção estabelece o processo oficial de auto-hospedagem (Self-hosting) da implementação oficial da linguagem Capi.

Seu objetivo é definir como a implementação inicial, desenvolvida em Rust, evolui para uma implementação capaz de compilar, manter e evoluir a si própria utilizando exclusivamente a linguagem Capi, preservando continuamente os contratos definidos por esta especificação.

---

## 26.2 Definição

Para os fins desta especificação, considera-se **auto-hospedagem** a capacidade da implementação oficial de compilar integralmente seu próprio código-fonte utilizando um compilador escrito na própria linguagem Capi.

---

## 26.3 Escopo

O processo de auto-hospedagem aplica-se exclusivamente à implementação oficial do compilador.

A linguagem Capi permanece independente da existência de uma implementação auto-hospedada.

---

## 26.4 Princípio Geral

A auto-hospedagem deverá ser alcançada de forma incremental, preservando continuamente a estabilidade, a corretude e a verificabilidade da implementação oficial.

---

## 26.5 Bootstrap Inicial

O processo de auto-hospedagem inicia-se com a implementação oficial desenvolvida em Rust, considerada o compilador de bootstrap.

---

## 26.6 Implementação em Capi

Paralelamente ao compilador de bootstrap, deverá ser desenvolvida uma implementação equivalente utilizando a própria linguagem Capi.

Essa implementação substituirá gradualmente os componentes originalmente escritos em Rust.

---

## 26.7 Coexistência

Durante o processo de auto-hospedagem poderá existir coexistência entre implementações em Rust e em Capi.

Essa coexistência constitui etapa transitória da evolução da implementação oficial.

---

## 26.8 Primeira Compilação

A primeira compilação do compilador escrito em Capi deverá ser realizada utilizando o compilador de bootstrap implementado em Rust.

---

## 26.9 Primeiro Executável

O resultado da primeira compilação deverá ser um compilador funcional escrito em Capi.

Esse compilador constituirá a primeira versão autoexecutável da implementação.

---

## 26.10 Recompilação

Após sua geração inicial, o compilador implementado em Capi deverá ser utilizado para recompilar integralmente seu próprio código-fonte.

---

## 26.11 Bootstrap em Estágios

O processo de auto-hospedagem poderá ser realizado em múltiplos estágios sucessivos.

Cada estágio deverá produzir uma implementação funcional e verificável.

---

## 26.12 Compilação Cruzada

Durante o processo de bootstrap poderão ser utilizados compiladores de diferentes gerações para produzir versões sucessivas da implementação oficial.

---

## 26.13 Cadeia de Compilação

A cadeia de compilação deverá permanecer completamente documentada, permitindo reproduzir todo o processo de bootstrap.

---

## 26.14 Determinismo

Sempre que executado sob as mesmas condições, o processo de auto-hospedagem deverá produzir comportamento funcional equivalente.

---

## 26.15 Equivalência Funcional

A implementação produzida pelo compilador escrito em Capi deverá apresentar comportamento funcional equivalente ao compilador de bootstrap em Rust.

---

## 26.16 Testes

Cada etapa da auto-hospedagem deverá ser validada pela suíte oficial de testes.

---

## 26.17 Testes Diferenciais

Sempre que tecnicamente aplicável, as implementações em Rust e em Capi deverão ser comparadas por meio de testes diferenciais durante todo o período de transição.

---

## 26.18 Recompilação Completa

A implementação auto-hospedada deverá ser capaz de recompilar integralmente:

* o frontend;
* o middle-end;
* os backends;
* o Runtime;
* a biblioteca padrão;
* a toolchain;
* os componentes auxiliares da implementação oficial.

---

## 26.19 Independência Progressiva

Cada etapa concluída deverá reduzir a dependência da implementação de bootstrap.

---

## 26.20 Eliminação da Dependência

Ao término do processo de auto-hospedagem, nenhuma etapa do ciclo normal de desenvolvimento deverá depender da implementação em Rust.

---

## 26.21 Compatibilidade

Durante todo o processo deverão permanecer preservados:

* a Backend API;
* a Intermediate Representation (IR);
* o Runtime;
* a ABI;
* o modelo de objetos;
* o modelo de memória;
* o sistema de tipos.

---

## 26.22 Arquitetura

A auto-hospedagem não deverá introduzir alterações arquiteturais na implementação oficial.

---

## 26.23 Especificação

O processo de auto-hospedagem não altera a especificação da linguagem.

Ele constitui exclusivamente uma etapa da evolução da implementação oficial.

---

## 26.24 Correções

Defeitos identificados durante a auto-hospedagem deverão ser corrigidos prioritariamente na implementação em Capi.

---

## 26.25 Regressões

Nenhuma regressão funcional poderá ser introduzida como consequência do processo de auto-hospedagem.

---

## 26.26 Documentação

Cada etapa da auto-hospedagem deverá possuir documentação técnica contendo, no mínimo:

* estágio do processo;
* componentes migrados;
* dependências remanescentes;
* resultados da validação;
* limitações conhecidas.

---

## 26.27 Reprodutibilidade

O processo completo de bootstrap deverá ser reproduzível por terceiros a partir da documentação oficial e do código-fonte disponibilizado.

---

## 26.28 Continuidade

Após alcançar a auto-hospedagem, toda evolução da implementação oficial deverá ocorrer preferencialmente utilizando o compilador escrito em Capi.

---

## 26.29 Manutenção

A implementação auto-hospedada passará a constituir a referência principal para manutenção e evolução do compilador oficial.

---

## 26.30 Bootstrap Histórico

A implementação em Rust poderá ser preservada como referência histórica, ferramenta de recuperação ou mecanismo alternativo de bootstrap.

Sua manutenção constitui decisão de governança da implementação oficial.

---

## 26.31 Recuperação

A preservação do compilador de bootstrap poderá ser utilizada para recuperação da implementação oficial em situações excepcionais, como regressões críticas ou necessidade de reconstrução completa do ambiente de desenvolvimento.

---

## 26.32 Evolução Posterior

A obtenção da auto-hospedagem não representa o encerramento do desenvolvimento da implementação oficial.

O compilador continuará sujeito ao processo contínuo de manutenção, otimização e evolução arquitetural.

---

## 26.33 Governança

A decisão de declarar concluído cada estágio do processo de auto-hospedagem deverá seguir os critérios estabelecidos pela governança da implementação oficial.

---

## 26.34 Invariantes

Durante todo o processo de auto-hospedagem deverão permanecer verdadeiros os seguintes princípios:

* a implementação em Rust constitui o compilador de bootstrap;
* a implementação em Capi substitui progressivamente a implementação inicial;
* a especificação da linguagem permanece inalterada durante toda a transição;
* toda etapa do processo é objetivamente verificável;
* a equivalência funcional é continuamente validada;
* a dependência da implementação em Rust diminui progressivamente até sua eliminação do ciclo normal de desenvolvimento;
* a arquitetura do compilador permanece integralmente preservada.

---

## 26.35 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um processo de auto-hospedagem que:

* conduza de forma incremental a transição da implementação em Rust para Capi;
* preserve integralmente a arquitetura e a especificação da linguagem;
* permita validação objetiva por meio de testes e recompilações sucessivas;
* elimine a dependência operacional do compilador de bootstrap;
* estabeleça uma implementação oficialmente auto-hospedada, apta a evoluir utilizando exclusivamente a linguagem Capi.

---

# 27 Critérios para Declarar a Auto-hospedagem

## 27.1 Objetivo

Esta seção estabelece os critérios normativos para que a implementação oficial da linguagem Capi possa ser formalmente considerada auto-hospedada.

Seu objetivo é definir requisitos técnicos, objetivos e verificáveis que assegurem que a implementação atingiu maturidade suficiente para evoluir utilizando exclusivamente a própria linguagem Capi.

---

## 27.2 Definição

Para os fins desta especificação, considera-se que a implementação oficial atingiu a auto-hospedagem quando o compilador implementado em Capi é capaz de compilar integralmente seu próprio código-fonte, preservando os contratos definidos pela especificação da linguagem.

---

## 27.3 Natureza da Declaração

A declaração de auto-hospedagem representa um marco da implementação oficial.

Ela não constitui uma nova versão da linguagem nem altera qualquer aspecto de sua especificação.

---

## 27.4 Critério Fundamental

A implementação somente poderá ser declarada auto-hospedada quando todos os critérios definidos nesta seção forem integralmente atendidos.

---

## 27.5 Compilação Completa

O compilador implementado em Capi deverá ser capaz de compilar integralmente:

* o frontend;
* o middle-end;
* todos os backends oficialmente suportados;
* o Runtime;
* a biblioteca padrão;
* a toolchain;
* os componentes auxiliares da implementação oficial.

---

## 27.6 Recompilação

Após sua compilação inicial, o compilador deverá ser capaz de recompilar integralmente seu próprio código-fonte utilizando exclusivamente a implementação escrita em Capi.

---

## 27.7 Independência Operacional

O ciclo normal de desenvolvimento da implementação oficial não deverá depender da implementação de bootstrap em Rust.

---

## 27.8 Preservação Arquitetural

A auto-hospedagem deverá preservar integralmente:

* a Backend API;
* a Intermediate Representation (IR);
* o Runtime;
* a ABI;
* o sistema de tipos;
* o modelo de memória;
* o modelo de objetos.

---

## 27.9 Equivalência Funcional

A implementação auto-hospedada deverá apresentar comportamento funcional equivalente ao compilador de bootstrap durante todo o processo de validação.

---

## 27.10 Testes

Toda a suíte oficial de testes deverá ser executada com sucesso pela implementação auto-hospedada.

---

## 27.11 Testes Diferenciais

Sempre que aplicável, os testes diferenciais entre a implementação em Rust e a implementação em Capi deverão demonstrar equivalência funcional.

---

## 27.12 Estabilidade

A implementação auto-hospedada deverá demonstrar estabilidade suficiente para utilização contínua como compilador oficial.

---

## 27.13 Ausência de Regressões

Não deverão existir regressões funcionais em relação à implementação de bootstrap.

---

## 27.14 Desempenho

Diferenças de desempenho não impedirão a declaração de auto-hospedagem, desde que permaneçam atendidos os requisitos mínimos estabelecidos pela implementação oficial.

---

## 27.15 Qualidade

A implementação auto-hospedada deverá apresentar qualidade de código compatível com os padrões definidos para a implementação oficial.

---

## 27.16 Documentação

Toda a documentação técnica correspondente ao compilador auto-hospedado deverá estar atualizada.

---

## 27.17 Reprodutibilidade

O processo completo de bootstrap deverá poder ser reproduzido a partir da documentação oficial e do código-fonte disponibilizado.

---

## 27.18 Portabilidade

A implementação auto-hospedada deverá permanecer compatível com as plataformas oficialmente suportadas.

---

## 27.19 Compatibilidade

A declaração de auto-hospedagem não altera os requisitos de compatibilidade entre versões da linguagem nem entre implementações do compilador.

---

## 27.20 Manutenção

Após a declaração de auto-hospedagem, a implementação escrita em Capi passará a constituir a referência principal para manutenção e evolução do compilador oficial.

---

## 27.21 Bootstrap Histórico

A implementação em Rust poderá ser mantida como:

* referência histórica;
* ferramenta de recuperação;
* mecanismo alternativo de bootstrap;
* base para estudos e validações.

Sua manutenção constitui decisão da governança da implementação oficial.

---

## 27.22 Continuidade

A obtenção da auto-hospedagem não encerra o desenvolvimento da implementação oficial.

O compilador continuará evoluindo conforme os princípios definidos nesta especificação.

---

## 27.23 Evolução

Após a auto-hospedagem, novas funcionalidades, otimizações e melhorias arquiteturais deverão ser implementadas preferencialmente utilizando a própria linguagem Capi.

---

## 27.24 Governança

A declaração oficial de auto-hospedagem deverá seguir o processo de governança da implementação oficial.

---

## 27.25 Evidências

A declaração de auto-hospedagem deverá ser acompanhada de evidências técnicas suficientes para demonstrar o atendimento dos critérios definidos nesta seção.

Essas evidências poderão incluir, entre outras:

* resultados da suíte oficial de testes;
* resultados dos testes diferenciais;
* registros do processo de bootstrap;
* documentação técnica;
* relatórios de compilação e validação.

---

## 27.26 Transparência

A decisão de declarar a implementação auto-hospedada deverá ser documentada de forma pública, permitindo auditoria técnica do processo.

---

## 27.27 Revogação

Caso sejam posteriormente identificadas falhas críticas que invalidem os critérios estabelecidos nesta seção, a condição de implementação auto-hospedada poderá ser revista pela governança da implementação oficial até que as inconsistências sejam corrigidas.

---

## 27.28 Sustentabilidade

A implementação auto-hospedada deverá demonstrar capacidade de manutenção contínua sem dependência operacional da implementação de bootstrap.

---

## 27.29 Continuidade Arquitetural

A obtenção da auto-hospedagem não autoriza alterações que violem os contratos permanentes definidos pela especificação da linguagem.

---

## 27.30 Neutralidade Tecnológica

A condição de auto-hospedagem independe das tecnologias utilizadas durante o bootstrap inicial.

O requisito fundamental é que a implementação oficial evolua utilizando exclusivamente a linguagem Capi.

---

## 27.31 Auto-hospedagem como Marco

A auto-hospedagem deverá ser compreendida como um marco importante da maturidade da implementação oficial, e não como o encerramento de sua evolução.

---

## 27.32 Invariantes

Durante toda a existência da implementação auto-hospedada deverão permanecer verdadeiros os seguintes princípios:

* o compilador oficial é implementado na linguagem Capi;
* o ciclo normal de desenvolvimento independe operacionalmente da implementação de bootstrap;
* a arquitetura definida pela especificação permanece integralmente preservada;
* a Backend API, a IR, o Runtime e a ABI continuam constituindo os contratos permanentes da implementação;
* toda evolução do compilador continua subordinada à especificação da linguagem;
* a auto-hospedagem representa um estado permanente da implementação oficial, e não uma etapa transitória.

---

## 27.33 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial demonstrar que:

* o compilador escrito em Capi compila integralmente seu próprio código-fonte;
* todo o processo de desenvolvimento pode ser realizado utilizando exclusivamente a linguagem Capi;
* a implementação de bootstrap em Rust deixa de ser necessária para o ciclo normal de evolução do compilador;
* todos os contratos arquiteturais definidos pela especificação permanecem integralmente preservados;
* existam evidências técnicas suficientes para declarar, de forma objetiva e auditável, que a linguagem Capi atingiu sua implementação oficialmente auto-hospedada.

---

# 28 Estratégia de Testes

## 28.1 Objetivo

Esta seção estabelece a estratégia oficial de testes da implementação oficial da linguagem Capi.

Seu objetivo é definir os princípios, métodos e critérios utilizados para validar continuamente o compilador, assegurando sua corretude, estabilidade, portabilidade e conformidade com a especificação da linguagem.

---

## 28.2 Escopo

A estratégia definida nesta seção aplica-se a todos os componentes da implementação oficial, incluindo:

* frontend;
* middle-end;
* backends;
* Runtime;
* biblioteca padrão;
* toolchain;
* infraestrutura de build;
* ferramentas auxiliares.

---

## 28.3 Princípio Geral

Toda funcionalidade implementada deverá ser acompanhada por testes compatíveis com seu escopo.

Nenhum componente deverá ser considerado concluído sem validação objetiva.

---

## 28.4 Objetivos da Estratégia de Testes

A estratégia de testes deverá assegurar:

* conformidade com a especificação da linguagem;
* estabilidade da implementação;
* prevenção de regressões;
* equivalência funcional entre backends;
* confiabilidade da evolução do compilador.

---

## 28.5 Pirâmide de Testes

A implementação oficial deverá priorizar uma estratégia baseada em múltiplos níveis de testes, privilegiando testes automatizados de menor granularidade e complementando-os por testes de integração e testes funcionais.

---

## 28.6 Testes Unitários

Testes unitários deverão validar o comportamento individual de componentes isolados.

Sempre que tecnicamente possível, cada módulo deverá possuir sua própria suíte de testes unitários.

---

## 28.7 Testes de Integração

Testes de integração deverão validar a interação entre componentes da implementação oficial.

Entre outros aspectos, deverão verificar a integração entre:

* frontend;
* middle-end;
* Runtime;
* backends;
* toolchain.

---

## 28.8 Testes Funcionais

Testes funcionais deverão validar o comportamento observável da linguagem.

Esses testes deverão ser escritos utilizando programas Capi representativos das funcionalidades especificadas.

---

## 28.9 Testes da Linguagem

A suíte oficial deverá conter casos de teste destinados especificamente à validação da especificação da linguagem.

Cada funcionalidade especificada deverá possuir casos de teste correspondentes.

---

## 28.10 Testes de Regressão

Toda falha identificada durante o desenvolvimento deverá originar, sempre que possível, um teste permanente de regressão.

Esses testes deverão impedir a reintrodução da mesma falha em versões futuras.

---

## 28.11 Testes Diferenciais

Quando houver múltiplos backends oficialmente suportados, deverão ser executados testes diferenciais conforme definido no Capítulo 21 desta especificação.

---

## 28.12 Testes Cross-platform

Sempre que aplicável, os testes deverão ser executados em todas as plataformas oficialmente suportadas.

---

## 28.13 Testes da Biblioteca Padrão

A biblioteca padrão deverá possuir suíte própria de testes, validando seu comportamento independentemente do restante da implementação.

---

## 28.14 Testes do Runtime

O Runtime deverá possuir testes específicos destinados à validação de:

* Domains;
* gerenciamento de memória;
* ObjectId;
* modelo de objetos;
* interoperabilidade;
* tratamento de erros.

---

## 28.15 Testes da Toolchain

Cada ferramenta integrante da toolchain deverá possuir testes compatíveis com sua finalidade.

---

## 28.16 Testes de Bootstrap

Durante o processo de bootstrap deverão existir testes específicos destinados à validação da implementação inicial em Rust.

---

## 28.17 Testes de Auto-hospedagem

Após o início da implementação em Capi, deverão existir testes destinados à validação do processo de auto-hospedagem.

---

## 28.18 Automatização

Toda a suíte oficial de testes deverá ser executável de forma automatizada.

Execuções manuais deverão restringir-se a situações excepcionais.

---

## 28.19 Reprodutibilidade

Os testes deverão produzir resultados determinísticos sempre que executados sob as mesmas condições.

---

## 28.20 Independência

Sempre que possível, casos de teste deverão ser independentes entre si.

A falha de um teste não deverá impedir a execução dos demais.

---

## 28.21 Isolamento

Cada teste deverá minimizar dependências externas, reduzindo interferências causadas pelo ambiente de execução.

---

## 28.22 Cobertura

A implementação oficial deverá acompanhar continuamente métricas de cobertura da suíte de testes.

Entretanto, percentuais de cobertura não constituem, isoladamente, critério suficiente para avaliação da qualidade da implementação.

---

## 28.23 Benchmarks

Benchmarks poderão compartilhar infraestrutura com a suíte oficial de testes.

Entretanto, medições de desempenho não substituem testes de corretude funcional.

---

## 28.24 Priorização

Testes relacionados às funcionalidades fundamentais da linguagem deverão possuir prioridade máxima durante o desenvolvimento.

---

## 28.25 Evolução da Suíte

A suíte oficial deverá evoluir continuamente em conjunto com a implementação.

Toda nova funcionalidade deverá ser acompanhada pelos testes correspondentes.

---

## 28.26 Organização

Os testes deverão ser organizados de forma modular, permitindo execução:

* individual;
* por componente;
* por categoria;
* por plataforma;
* de toda a suíte.

---

## 28.27 Diagnósticos

Resultados da execução da suíte deverão fornecer informações suficientes para identificação rápida da origem das falhas.

---

## 28.28 Tempo de Execução

A estratégia de testes deverá buscar equilíbrio entre abrangência e tempo de execução, favorecendo feedback rápido durante o desenvolvimento.

---

## 28.29 Integração Contínua

A suíte oficial deverá ser integrada ao processo de Integração Contínua definido nesta especificação.

---

## 28.30 Critérios para Aprovação

Uma alteração somente poderá ser considerada apta à integração quando a suíte de testes correspondente for executada com sucesso.

---

## 28.31 Exceções

Exceções temporárias à política de testes deverão ser tecnicamente justificadas, documentadas e aprovadas conforme o processo de governança da implementação oficial.

---

## 28.32 Documentação

A estratégia de testes, sua organização e seus critérios deverão permanecer documentados e atualizados ao longo da evolução da implementação oficial.

---

## 28.33 Governança

A evolução da suíte oficial de testes constitui responsabilidade da governança da implementação oficial.

Toda alteração deverá preservar a capacidade de validar objetivamente a conformidade da implementação com a especificação da linguagem.

---

## 28.34 Invariantes

Durante toda a evolução da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* toda funcionalidade implementada possui testes correspondentes;
* a corretude funcional possui prioridade sobre o desempenho;
* regressões originam testes permanentes;
* a suíte oficial evolui continuamente com a implementação;
* testes permanecem automatizados, reproduzíveis e independentes;
* a conformidade com a especificação da linguagem é continuamente validada.

---

## 28.35 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma estratégia de testes que:

* valide objetivamente todos os componentes do compilador;
* assegure conformidade com a especificação da linguagem;
* previna regressões ao longo da evolução da implementação;
* permita validação contínua em múltiplas plataformas e backends;
* forneça uma base confiável para a manutenção e evolução da implementação oficial da linguagem Capi.

---

# 29 Diagnósticos e Qualidade

## 29.1 Objetivo

Esta seção estabelece os princípios e requisitos normativos relacionados à produção de diagnósticos e à qualidade da implementação oficial da linguagem Capi.

Seu objetivo é assegurar que o compilador produza informações claras, precisas e consistentes durante todo o processo de compilação, bem como definir critérios permanentes para manutenção da qualidade técnica da implementação.

---

## 29.2 Escopo

Os requisitos definidos nesta seção aplicam-se a todos os componentes da implementação oficial capazes de produzir diagnósticos ou influenciar a qualidade da experiência de desenvolvimento.

---

## 29.3 Princípio Geral

Todo diagnóstico produzido pela implementação oficial deverá auxiliar o desenvolvedor na identificação e resolução do problema correspondente.

Mensagens ambíguas, incompletas ou enganosas deverão ser evitadas.

---

## 29.4 Diagnósticos como Parte da Implementação

A qualidade dos diagnósticos constitui parte integrante da qualidade da implementação oficial.

A produção de código correto não elimina a necessidade de diagnósticos de alta qualidade.

---

## 29.5 Classificação

Os diagnósticos deverão ser classificados conforme sua natureza.

No mínimo, deverão existir as seguintes categorias:

* erro;
* aviso;
* informação;
* observação.

A forma de apresentação constitui detalhe de implementação.

---

## 29.6 Erros

Erros representam condições que impedem a continuação normal da compilação ou invalidam a geração de código correto.

---

## 29.7 Avisos

Avisos representam situações potencialmente problemáticas que não impedem a compilação, mas merecem atenção do desenvolvedor.

---

## 29.8 Informações

Mensagens informativas poderão ser utilizadas para comunicar eventos relevantes do processo de compilação que não caracterizam erros ou avisos.

---

## 29.9 Observações

Observações poderão complementar outros diagnósticos, fornecendo contexto adicional para facilitar sua compreensão.

---

## 29.10 Clareza

Todo diagnóstico deverá utilizar linguagem objetiva, precisa e tecnicamente consistente.

---

## 29.11 Precisão

Sempre que possível, o diagnóstico deverá identificar exatamente o elemento responsável pela ocorrência da condição reportada.

---

## 29.12 Localização

Sempre que tecnicamente possível, os diagnósticos deverão indicar a localização correspondente no código-fonte.

---

## 29.13 Contexto

Sempre que aplicável, os diagnósticos deverão fornecer contexto suficiente para facilitar a compreensão da condição reportada.

---

## 29.14 Consistência

Diagnósticos equivalentes deverão possuir estrutura e terminologia consistentes em toda a implementação oficial.

---

## 29.15 Determinismo

Uma mesma condição deverá produzir diagnósticos funcionalmente equivalentes em execuções sucessivas.

---

## 29.16 Recuperação

Sempre que tecnicamente viável, o compilador deverá continuar a análise após identificar um erro, produzindo diagnósticos adicionais sem comprometer significativamente sua precisão.

---

## 29.17 Acúmulo de Diagnósticos

A implementação deverá evitar interromper prematuramente a compilação quando for possível identificar múltiplas inconsistências de forma confiável.

---

## 29.18 Cascatas de Erros

Mecanismos de recuperação deverão minimizar a produção de diagnósticos redundantes decorrentes de um único erro original.

---

## 29.19 Sugestões

Sempre que tecnicamente possível, diagnósticos poderão incluir sugestões para auxiliar na correção da condição identificada.

Essas sugestões não deverão alterar a semântica definida pela especificação da linguagem.

---

## 29.20 Identificadores

Sempre que apropriado, diagnósticos deverão identificar explicitamente os elementos da linguagem envolvidos na condição reportada.

---

## 29.21 Padronização

O formato geral dos diagnósticos deverá permanecer uniforme em toda a implementação oficial.

---

## 29.22 Internacionalização

A arquitetura deverá permitir internacionalização dos diagnósticos sem alterar sua estrutura ou significado técnico.

A política de idiomas suportados constitui decisão da implementação oficial.

---

## 29.23 Instrumentação

A implementação oficial poderá disponibilizar mecanismos de instrumentação destinados ao diagnóstico interno do compilador.

Esses mecanismos não alteram a semântica da linguagem.

---

## 29.24 Logs

A implementação poderá produzir registros técnicos destinados à depuração do próprio compilador.

Esses registros constituem detalhes de implementação.

---

## 29.25 Profiling

Ferramentas de análise de desempenho poderão ser utilizadas para identificar gargalos durante o processo de compilação.

---

## 29.26 Métricas

A implementação poderá coletar métricas relacionadas à qualidade do compilador, incluindo, entre outras:

* tempo de compilação;
* consumo de memória;
* cobertura de testes;
* quantidade de diagnósticos;
* desempenho da geração de código.

---

## 29.27 Qualidade do Código

A implementação oficial deverá buscar continuamente:

* simplicidade;
* modularidade;
* baixo acoplamento;
* alta coesão;
* legibilidade;
* facilidade de manutenção.

---

## 29.28 Revisões

Alterações significativas na implementação deverão ser submetidas à revisão técnica antes de sua integração.

---

## 29.29 Refatorações

Refatorações destinadas à melhoria da qualidade da implementação deverão preservar integralmente o comportamento observável do compilador.

---

## 29.30 Defeitos

Defeitos identificados deverão ser registrados, analisados e corrigidos conforme o processo de manutenção da implementação oficial.

---

## 29.31 Regressões

Toda regressão relacionada a diagnósticos ou qualidade da implementação deverá originar novos casos na suíte oficial de testes.

---

## 29.32 Monitoramento

A evolução da qualidade da implementação deverá ser acompanhada continuamente através de indicadores objetivos definidos pela implementação oficial.

---

## 29.33 Transparência

Critérios de qualidade adotados pela implementação oficial deverão permanecer documentados e acessíveis aos colaboradores do projeto.

---

## 29.34 Evolução

Os mecanismos de diagnóstico poderão evoluir continuamente, desde que preservem compatibilidade com os contratos definidos pela especificação da linguagem.

---

## 29.35 Governança

A definição dos padrões de qualidade e dos critérios para evolução dos diagnósticos constitui responsabilidade da governança da implementação oficial.

---

## 29.36 Invariantes

Durante toda a evolução da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* diagnósticos auxiliam a compreensão e correção dos problemas identificados;
* mensagens permanecem claras, consistentes e determinísticas;
* a recuperação de erros busca maximizar a quantidade de informações úteis ao desenvolvedor;
* a qualidade da implementação é continuamente monitorada;
* melhorias de qualidade não alteram a semântica da linguagem;
* toda regressão relacionada a diagnósticos origina novos testes permanentes.

---

## 29.37 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma infraestrutura de diagnósticos e qualidade que:

* produza mensagens claras, precisas e consistentes;
* favoreça a rápida identificação e correção de problemas;
* permita recuperação adequada durante o processo de compilação;
* monitore continuamente a qualidade técnica da implementação;
* preserve integralmente os contratos definidos pela especificação da linguagem Capi.

---

# 30 Integração Contínua

## 30.1 Objetivo

Esta seção estabelece a política oficial de Integração Contínua (Continuous Integration — CI) da implementação oficial da linguagem Capi.

Seu objetivo é definir os princípios, requisitos e critérios utilizados para validar automaticamente toda alteração realizada na implementação, assegurando sua estabilidade, corretude e conformidade com a especificação da linguagem.

---

## 30.2 Escopo

A política de Integração Contínua aplica-se a todos os componentes da implementação oficial, incluindo:

* frontend;
* middle-end;
* backends;
* Runtime;
* biblioteca padrão;
* toolchain;
* infraestrutura de build;
* ferramentas auxiliares.

---

## 30.3 Princípio Geral

Toda alteração submetida à implementação oficial deverá ser validada automaticamente antes de sua integração.

Nenhuma modificação deverá comprometer a estabilidade da implementação.

---

## 30.4 Automação

O processo de Integração Contínua deverá ser integralmente automatizado.

Intervenções manuais deverão restringir-se a situações excepcionais previstas pelo processo de governança.

---

## 30.5 Pipeline de Integração

A implementação oficial deverá possuir um pipeline de Integração Contínua capaz de executar automaticamente todas as etapas necessárias para validação de uma alteração.

A organização do pipeline constitui detalhe de implementação.

---

## 30.6 Compilação Automatizada

Toda alteração deverá ser compilada automaticamente antes de sua integração ao repositório principal.

---

## 30.7 Execução da Suíte de Testes

A suíte oficial de testes deverá ser executada automaticamente como parte obrigatória do pipeline de Integração Contínua.

---

## 30.8 Testes Unitários

O pipeline deverá executar automaticamente os testes unitários aplicáveis aos componentes modificados.

---

## 30.9 Testes de Integração

Sempre que aplicável, deverão ser executados testes de integração destinados à validação da interação entre os componentes da implementação oficial.

---

## 30.10 Testes Funcionais

O pipeline deverá validar automaticamente o comportamento funcional da linguagem por meio da suíte oficial de testes.

---

## 30.11 Testes Diferenciais

Quando houver múltiplos backends oficialmente suportados, deverão ser executados automaticamente os testes diferenciais definidos nesta especificação.

---

## 30.12 Testes Cross-platform

Sempre que tecnicamente viável, o pipeline deverá validar a implementação nas plataformas oficialmente suportadas.

---

## 30.13 Benchmarks

Benchmarks poderão ser executados automaticamente como mecanismo de monitoramento contínuo de desempenho.

Resultados de benchmarks não substituem critérios de corretude funcional.

---

## 30.14 Análise Estática

O pipeline poderá executar ferramentas de análise estática destinadas à identificação preventiva de inconsistências na implementação.

---

## 30.15 Formatação

Sempre que aplicável, o pipeline deverá validar automaticamente a conformidade do código-fonte com o padrão oficial de formatação adotado pela implementação.

---

## 30.16 Qualidade do Código

Ferramentas destinadas à avaliação contínua da qualidade do código poderão integrar o pipeline de Integração Contínua.

---

## 30.17 Cobertura

O pipeline poderá produzir métricas de cobertura da suíte oficial de testes.

Essas métricas deverão ser utilizadas como indicadores de qualidade, não como único critério de aprovação.

---

## 30.18 Build Reproduzível

Sempre que tecnicamente possível, o pipeline deverá produzir resultados reproduzíveis quando executado sob as mesmas condições.

---

## 30.19 Artefatos

O pipeline poderá produzir artefatos destinados à validação, distribuição ou análise técnica da implementação oficial.

A política de publicação desses artefatos constitui decisão da implementação oficial.

---

## 30.20 Diagnósticos

Falhas ocorridas durante o processo de Integração Contínua deverão produzir diagnósticos suficientes para facilitar sua identificação e correção.

---

## 30.21 Regressões

Toda regressão identificada pelo pipeline deverá impedir automaticamente a integração da alteração correspondente até sua resolução.

---

## 30.22 Critérios de Aprovação

Uma alteração somente poderá ser integrada ao repositório principal quando atender aos critérios definidos pela política oficial de Integração Contínua.

---

## 30.23 Integração com o Processo de Desenvolvimento

A Integração Contínua deverá constituir parte integrante do fluxo normal de desenvolvimento da implementação oficial.

---

## 30.24 Integração com a Auto-hospedagem

Após a obtenção da auto-hospedagem, o pipeline deverá validar automaticamente a compilação e recompilação da implementação utilizando o compilador escrito em Capi.

---

## 30.25 Escalabilidade

A arquitetura do pipeline deverá permitir evolução contínua sem comprometer sua estabilidade ou sua capacidade de validação.

---

## 30.26 Disponibilidade

A infraestrutura de Integração Contínua deverá buscar elevada disponibilidade, reduzindo impactos no processo de desenvolvimento.

---

## 30.27 Configuração

A configuração do pipeline deverá permanecer versionada juntamente com a implementação oficial.

---

## 30.28 Evolução

Novas etapas poderão ser incorporadas ao pipeline desde que preservem os princípios definidos nesta especificação.

---

## 30.29 Documentação

A arquitetura, o funcionamento e os critérios adotados pela Integração Contínua deverão permanecer documentados e atualizados.

---

## 30.30 Transparência

Os resultados produzidos pela Integração Contínua deverão ser acessíveis aos colaboradores autorizados da implementação oficial.

---

## 30.31 Governança

A evolução da infraestrutura de Integração Contínua constitui responsabilidade da governança da implementação oficial.

Toda alteração deverá preservar a confiabilidade do processo de validação automática.

---

## 30.32 Invariantes

Durante toda a evolução da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* toda alteração é validada automaticamente antes de sua integração;
* a suíte oficial de testes integra obrigatoriamente o pipeline de Integração Contínua;
* regressões impedem automaticamente a integração de alterações;
* o pipeline evolui continuamente em conjunto com a implementação oficial;
* os resultados produzidos pelo processo permanecem reproduzíveis sempre que tecnicamente possível;
* a Integração Contínua constitui mecanismo permanente de garantia da qualidade da implementação.

---

## 30.33 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma infraestrutura de Integração Contínua que:

* valide automaticamente todas as alterações realizadas na implementação;
* execute a suíte oficial de testes e demais mecanismos de validação definidos nesta especificação;
* impeça a integração de alterações que comprometam a estabilidade ou a corretude da implementação;
* forneça diagnósticos claros e objetivos para identificação de falhas;
* constitua uma base confiável para a evolução contínua da implementação oficial da linguagem Capi.

---

# 31 Escopo da Implementação Oficial

## 31.1 Objetivo

Esta seção estabelece o escopo da implementação oficial da linguagem Capi.

Seu objetivo é definir os componentes que integram oficialmente a implementação, os limites de sua responsabilidade e os critérios utilizados para determinar quais funcionalidades pertencem ou não ao projeto oficial.

---

## 31.2 Escopo

A implementação oficial compreende exclusivamente os componentes necessários para implementar, validar, documentar e distribuir a linguagem Capi conforme definido por sua especificação.

---

## 31.3 Princípio Geral

A implementação oficial deverá permanecer fiel à especificação da linguagem.

Nenhum componente deverá ser incorporado caso introduza dependências ou responsabilidades incompatíveis com os objetivos definidos para a implementação oficial.

---

## 31.4 Componentes Oficiais

Constituem componentes oficiais da implementação, no mínimo:

* frontend;
* middle-end;
* backends oficialmente suportados;
* Runtime;
* biblioteca padrão;
* toolchain;
* infraestrutura de build;
* infraestrutura de testes;
* infraestrutura de integração contínua;
* documentação técnica da implementação.

---

## 31.5 Componentes Auxiliares

Ferramentas auxiliares destinadas ao desenvolvimento, depuração, validação ou manutenção poderão integrar a implementação oficial quando contribuírem diretamente para sua qualidade ou evolução.

---

## 31.6 Componentes Experimentais

A implementação oficial poderá incluir componentes experimentais destinados à avaliação de novas tecnologias, arquiteturas ou otimizações.

Esses componentes não constituem parte obrigatória da implementação oficial enquanto não forem formalmente promovidos conforme o processo de governança.

---

## 31.7 Componentes Opcionais

Ferramentas opcionais poderão ser disponibilizadas pela implementação oficial, desde que sua ausência não comprometa a utilização da linguagem conforme definido por esta especificação.

---

## 31.8 Fora de Escopo

Não constituem responsabilidades obrigatórias da implementação oficial, entre outros:

* ambientes integrados de desenvolvimento (IDEs);
* editores de texto;
* plugins para terceiros;
* serviços em nuvem;
* repositórios de pacotes;
* sistemas de integração com plataformas específicas;
* ferramentas pertencentes exclusivamente ao ecossistema da linguagem.

Esses componentes poderão ser desenvolvidos independentemente, respeitando a especificação da linguagem.

---

## 31.9 Implementações Independentes

A existência da implementação oficial não restringe o desenvolvimento de implementações independentes da linguagem Capi.

Toda implementação poderá adotar soluções arquiteturais próprias, desde que preserve a conformidade com a especificação da linguagem.

---

## 31.10 Compatibilidade com a Especificação

A implementação oficial deverá permanecer continuamente compatível com todos os contratos estabelecidos pela especificação da linguagem.

---

## 31.11 Neutralidade Tecnológica

O escopo da implementação oficial não deverá depender permanentemente de tecnologias específicas.

Tecnologias utilizadas durante sua evolução constituem detalhes de implementação.

---

## 31.12 Modularidade

Os componentes da implementação deverão permanecer organizados de forma modular, permitindo evolução independente sempre que tecnicamente possível.

---

## 31.13 Evolução do Escopo

O escopo da implementação oficial poderá evoluir ao longo do tempo.

Toda alteração deverá ser tecnicamente justificada e aprovada conforme o processo de governança.

---

## 31.14 Inclusão de Componentes

Novos componentes somente deverão ser incorporados quando apresentarem contribuição clara para:

* a implementação da linguagem;
* sua manutenção;
* sua validação;
* sua evolução;
* sua distribuição.

---

## 31.15 Remoção de Componentes

Componentes poderão ser removidos quando:

* tornarem-se obsoletos;
* forem substituídos por soluções equivalentes;
* deixarem de atender aos objetivos da implementação oficial;
* aumentarem desnecessariamente a complexidade do projeto.

---

## 31.16 Dependências Externas

A implementação oficial deverá buscar minimizar dependências externas permanentes.

Sempre que tecnicamente viável, deverão ser priorizadas soluções amplamente consolidadas, estáveis e sustentáveis.

---

## 31.17 Complexidade

O crescimento da implementação deverá buscar continuamente equilíbrio entre funcionalidade, simplicidade arquitetural e facilidade de manutenção.

---

## 31.18 Estabilidade

A expansão do escopo não deverá comprometer a estabilidade da implementação oficial.

---

## 31.19 Compatibilidade Retroativa

Sempre que aplicável, alterações no escopo deverão preservar a compatibilidade com versões anteriores da implementação oficial, observadas as políticas de evolução do projeto.

---

## 31.20 Documentação

Todo componente integrante da implementação oficial deverá possuir documentação técnica compatível com sua finalidade.

---

## 31.21 Testabilidade

Componentes incorporados ao escopo da implementação oficial deverão ser passíveis de validação objetiva por meio da infraestrutura oficial de testes.

---

## 31.22 Manutenibilidade

A inclusão de novos componentes deverá considerar seus impactos sobre a manutenção de longo prazo da implementação oficial.

---

## 31.23 Sustentabilidade

O escopo da implementação deverá permanecer compatível com a capacidade de manutenção da comunidade responsável por sua evolução.

---

## 31.24 Transparência

A composição da implementação oficial deverá permanecer claramente documentada, permitindo identificar os componentes considerados oficiais em cada versão do projeto.

---

## 31.25 Governança

Toda alteração significativa no escopo deverá seguir o processo de governança definido para a implementação oficial.

---

## 31.26 Independência da Linguagem

Alterações no escopo da implementação oficial não modificam a especificação da linguagem Capi.

A linguagem permanece conceitualmente independente de qualquer implementação específica.

---

## 31.27 Evolução Contínua

A implementação oficial poderá evoluir continuamente, desde que preserve os contratos permanentes estabelecidos por esta especificação.

---

## 31.28 Invariantes

Durante toda a evolução da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* a implementação oficial permanece subordinada à especificação da linguagem;
* o escopo limita-se aos componentes necessários para implementar, validar e distribuir a linguagem;
* componentes experimentais não integram automaticamente a implementação oficial;
* ferramentas externas ao núcleo da implementação permanecem fora do escopo obrigatório;
* alterações de escopo seguem o processo oficial de governança;
* a evolução do escopo preserva simplicidade, modularidade e sustentabilidade da implementação.

---

## 31.29 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um escopo claramente definido que:

* delimite objetivamente os componentes oficiais do projeto;
* estabeleça critérios para inclusão, evolução e remoção de componentes;
* preserve a independência entre a linguagem e sua implementação;
* impeça a expansão descontrolada das responsabilidades da implementação oficial;
* forneça uma base estável para a evolução sustentável da implementação oficial da linguagem Capi.

---

# 32 Gestão de Riscos Técnicos

## 32.1 Objetivo

Esta seção estabelece a política oficial de gestão de riscos técnicos da implementação oficial da linguagem Capi.

Seu objetivo é definir princípios, critérios e mecanismos destinados à identificação, avaliação, mitigação e monitoramento dos riscos que possam comprometer a estabilidade, a evolução ou a sustentabilidade da implementação oficial.

---

## 32.2 Escopo

A gestão de riscos técnicos aplica-se a todos os componentes da implementação oficial, incluindo:

* frontend;
* middle-end;
* backends;
* Runtime;
* biblioteca padrão;
* toolchain;
* infraestrutura de build;
* infraestrutura de testes;
* infraestrutura de Integração Contínua;
* processos de desenvolvimento.

---

## 32.3 Princípio Geral

Riscos técnicos deverão ser identificados e tratados de forma contínua ao longo de todo o ciclo de vida da implementação oficial.

A prevenção deverá ser priorizada em relação à correção posterior.

---

## 32.4 Identificação de Riscos

A implementação oficial deverá manter mecanismos destinados à identificação contínua de riscos técnicos relacionados à sua arquitetura, implementação, manutenção e evolução.

---

## 32.5 Classificação

Os riscos deverão ser classificados conforme critérios definidos pela implementação oficial.

Os critérios de classificação constituem detalhes de implementação.

---

## 32.6 Avaliação

Todo risco identificado deverá ser avaliado quanto ao seu potencial impacto sobre:

* a estabilidade;
* a corretude;
* o desempenho;
* a segurança;
* a portabilidade;
* a manutenibilidade;
* a evolução da implementação.

---

## 32.7 Priorização

Os riscos deverão ser priorizados considerando seu impacto potencial e a probabilidade de ocorrência.

---

## 32.8 Mitigação

Sempre que tecnicamente viável, deverão ser adotadas medidas destinadas à redução ou eliminação dos riscos identificados.

---

## 32.9 Monitoramento

Os riscos técnicos deverão ser monitorados continuamente durante a evolução da implementação oficial.

---

## 32.10 Arquitetura

Decisões arquiteturais deverão considerar explicitamente os riscos técnicos associados às alternativas avaliadas.

---

## 32.11 Dívida Técnica

A implementação oficial deverá monitorar continuamente a existência de dívida técnica.

Sua redução deverá ocorrer de forma planejada e compatível com a evolução do projeto.

---

## 32.12 Complexidade

O aumento desnecessário da complexidade constitui risco técnico e deverá ser evitado.

Sempre que possível, deverão ser priorizadas soluções arquiteturalmente mais simples.

---

## 32.13 Dependências Externas

Dependências externas deverão ser avaliadas quanto aos riscos relacionados à:

* manutenção;
* estabilidade;
* compatibilidade;
* disponibilidade;
* sustentabilidade.

---

## 32.14 Obsolescência Tecnológica

A implementação oficial deverá acompanhar continuamente a evolução das tecnologias das quais depende, buscando reduzir riscos decorrentes de obsolescência.

---

## 32.15 Continuidade

Sempre que possível, deverão existir estratégias que permitam substituir tecnologias externas cuja continuidade deixe de ser considerada adequada.

---

## 32.16 Portabilidade

Decisões que reduzam significativamente a portabilidade da implementação deverão ser cuidadosamente avaliadas quanto aos riscos introduzidos.

---

## 32.17 Compatibilidade

Alterações que possam comprometer a compatibilidade da implementação deverão ser precedidas de análise técnica compatível com seu impacto.

---

## 32.18 Segurança

Aspectos relacionados à segurança da implementação deverão integrar permanentemente a gestão de riscos técnicos.

---

## 32.19 Desempenho

Riscos relacionados ao desempenho deverão ser continuamente acompanhados por meio de medições e benchmarks compatíveis com a natureza da implementação.

---

## 32.20 Bootstrap

O processo de bootstrap deverá possuir mecanismos destinados à mitigação dos riscos inerentes à transição entre a implementação inicial em Rust e a implementação auto-hospedada em Capi.

---

## 32.21 Auto-hospedagem

A obtenção da auto-hospedagem não elimina a necessidade de monitoramento contínuo dos riscos técnicos associados à evolução da implementação.

---

## 32.22 Testes

A suíte oficial de testes constitui um dos principais mecanismos de mitigação de riscos técnicos.

---

## 32.23 Integração Contínua

O processo de Integração Contínua deverá contribuir para a identificação precoce de riscos introduzidos por alterações na implementação.

---

## 32.24 Recuperação

Sempre que tecnicamente viável, deverão existir mecanismos capazes de permitir recuperação após falhas críticas ocorridas durante o desenvolvimento da implementação oficial.

---

## 32.25 Registro

Riscos técnicos considerados relevantes deverão ser registrados e acompanhados ao longo de sua existência.

A forma desse registro constitui detalhe de implementação.

---

## 32.26 Revisões Técnicas

Revisões técnicas deverão considerar explicitamente os riscos associados às alterações propostas.

---

## 32.27 Evolução

A política de gestão de riscos deverá evoluir continuamente em conjunto com a implementação oficial.

---

## 32.28 Transparência

Sempre que apropriado, informações relativas aos principais riscos técnicos deverão permanecer acessíveis aos colaboradores responsáveis pela evolução da implementação.

---

## 32.29 Documentação

Os critérios utilizados para identificação, avaliação e mitigação de riscos deverão permanecer documentados e atualizados.

---

## 32.30 Governança

A definição das políticas de gestão de riscos técnicos constitui responsabilidade da governança da implementação oficial.

---

## 32.31 Independência da Especificação

A gestão de riscos técnicos aplica-se exclusivamente à implementação oficial.

Ela não modifica os contratos definidos pela especificação da linguagem.

---

## 32.32 Evolução Sustentável

Toda decisão técnica deverá considerar seus impactos sobre a sustentabilidade de longo prazo da implementação oficial.

---

## 32.33 Invariantes

Durante toda a evolução da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* riscos técnicos são continuamente identificados, avaliados e monitorados;
* a prevenção possui prioridade sobre a correção posterior;
* a dívida técnica é acompanhada e reduzida de forma planejada;
* a simplicidade arquitetural constitui mecanismo permanente de mitigação de riscos;
* testes e Integração Contínua representam instrumentos essenciais de redução de riscos;
* a gestão de riscos não altera a especificação da linguagem, restringindo-se à implementação oficial.

---

## 32.34 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir uma política de gestão de riscos técnicos que:

* identifique e acompanhe continuamente riscos relevantes;
* estabeleça critérios objetivos para avaliação e mitigação desses riscos;
* contribua para a estabilidade, a qualidade e a sustentabilidade da implementação oficial;
* integre-se aos processos de desenvolvimento, testes e Integração Contínua;
* forneça uma base consistente para a evolução segura e contínua da implementação oficial da linguagem Capi.

---

# 33 Governança da Implementação Oficial

## 33.1 Objetivo

Esta seção estabelece os princípios e critérios de governança da implementação oficial da linguagem Capi.

Seu objetivo é definir como decisões técnicas relacionadas à implementação oficial são tomadas, registradas e evoluem ao longo do ciclo de vida do projeto, preservando a estabilidade arquitetural, a transparência e a conformidade com a especificação da linguagem.

---

## 33.2 Escopo

A governança definida nesta seção aplica-se exclusivamente à implementação oficial da linguagem Capi.

Ela não estabelece regras para implementações independentes da linguagem.

---

## 33.3 Princípio Geral

Toda decisão relacionada à implementação oficial deverá preservar os contratos definidos pela especificação da linguagem e contribuir para sua evolução sustentável.

---

## 33.4 Responsabilidade

A implementação oficial deverá possuir uma estrutura de governança responsável por sua manutenção, evolução e coordenação técnica.

A composição dessa estrutura constitui detalhe organizacional do projeto.

---

## 33.5 Separação entre Linguagem e Implementação

A governança da implementação oficial não possui autoridade para modificar unilateralmente a especificação da linguagem.

Alterações na linguagem deverão seguir o processo definido pela própria especificação.

---

## 33.6 Processo Decisório

Decisões técnicas relevantes deverão seguir um processo transparente, documentado e fundamentado em critérios técnicos.

---

## 33.7 Fundamentação Técnica

Toda decisão arquitetural significativa deverá apresentar justificativa técnica compatível com seu impacto sobre a implementação oficial.

---

## 33.8 Registro das Decisões

As decisões relevantes deverão permanecer registradas de forma permanente, permitindo rastreabilidade ao longo da evolução da implementação.

A forma desse registro constitui detalhe de implementação.

---

## 33.9 Revisões Técnicas

Alterações significativas deverão ser submetidas à revisão técnica antes de sua incorporação à implementação oficial.

---

## 33.10 Consenso Técnico

Sempre que possível, decisões arquiteturais deverão buscar consenso entre os responsáveis pela evolução da implementação oficial.

Na impossibilidade de consenso, deverá existir mecanismo formal de decisão definido pela governança.

---

## 33.11 Transparência

Os critérios utilizados para tomada de decisões deverão permanecer acessíveis aos colaboradores da implementação oficial.

---

## 33.12 Evolução

A governança deverá favorecer a evolução contínua da implementação oficial, preservando sua estabilidade e sustentabilidade.

---

## 33.13 Modularidade

Decisões relacionadas à evolução da implementação deverão preservar, sempre que tecnicamente possível, a modularidade da arquitetura.

---

## 33.14 Compatibilidade

Alterações aprovadas pela governança deverão considerar seus impactos sobre:

* a Backend API;
* a Intermediate Representation (IR);
* o Runtime;
* a ABI;
* a biblioteca padrão;
* a toolchain.

---

## 33.15 Dependências Externas

A adoção, substituição ou remoção de dependências externas deverá considerar seus impactos sobre a sustentabilidade de longo prazo da implementação oficial.

---

## 33.16 Componentes Experimentais

A incorporação de componentes experimentais à implementação oficial deverá ocorrer somente após avaliação técnica compatível com os critérios definidos pela governança.

---

## 33.17 Depreciação

A descontinuação de componentes da implementação deverá seguir processo documentado que preserve previsibilidade para os colaboradores e usuários.

---

## 33.18 Gestão de Conflitos

Sempre que surgirem conflitos técnicos relevantes, a governança deverá adotar mecanismos destinados à sua resolução objetiva, preservando os princípios definidos por esta especificação.

---

## 33.19 Priorização

A priorização das atividades de desenvolvimento deverá considerar, entre outros fatores:

* estabilidade;
* corretude;
* segurança;
* sustentabilidade;
* evolução arquitetural;
* qualidade da implementação.

---

## 33.20 Planejamento

A evolução da implementação oficial deverá ocorrer de forma planejada, buscando equilíbrio entre novas funcionalidades, correções, otimizações e redução de dívida técnica.

---

## 33.21 Documentação

Toda decisão arquitetural relevante deverá possuir documentação técnica correspondente.

---

## 33.22 Qualidade

A governança deverá promover continuamente:

* qualidade do código;
* qualidade dos diagnósticos;
* qualidade da documentação;
* qualidade da arquitetura;
* qualidade da suíte oficial de testes.

---

## 33.23 Testes

Nenhuma decisão de governança deverá reduzir os requisitos mínimos de validação definidos para a implementação oficial.

---

## 33.24 Integração Contínua

Toda alteração aprovada deverá permanecer sujeita ao processo oficial de Integração Contínua.

---

## 33.25 Auto-hospedagem

Após a obtenção da auto-hospedagem, a governança deverá assegurar que a implementação escrita em Capi permaneça a referência principal para evolução do compilador oficial.

---

## 33.26 Independência das Implementações

A existência da implementação oficial não impede a existência de outras implementações da linguagem Capi.

A governança da implementação oficial não exerce autoridade sobre implementações independentes.

---

## 33.27 Continuidade

A governança deverá buscar garantir a continuidade técnica da implementação oficial, reduzindo riscos decorrentes da dependência de indivíduos, tecnologias ou processos específicos.

---

## 33.28 Sustentabilidade

Toda decisão deverá considerar seus impactos sobre a sustentabilidade de longo prazo da implementação oficial.

---

## 33.29 Evolução da Governança

O próprio modelo de governança poderá evoluir ao longo do tempo, desde que preserve os princípios estabelecidos nesta seção.

---

## 33.30 Independência da Especificação

Alterações na governança da implementação oficial não modificam a especificação da linguagem nem seus contratos permanentes.

---

## 33.31 Registro Histórico

A implementação oficial deverá preservar histórico suficiente das decisões técnicas relevantes para permitir compreender a evolução arquitetural do projeto.

---

## 33.32 Invariantes

Durante toda a evolução da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* a implementação oficial permanece subordinada à especificação da linguagem;
* decisões técnicas são fundamentadas em critérios objetivos e documentados;
* alterações arquiteturais preservam os contratos permanentes da implementação;
* a evolução da implementação permanece transparente e rastreável;
* a governança promove estabilidade, qualidade e sustentabilidade de longo prazo;
* implementações independentes permanecem livres para adotar processos próprios de governança.

---

## 33.33 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um modelo de governança que:

* estabeleça critérios claros para tomada de decisões técnicas;
* preserve a separação entre a especificação da linguagem e sua implementação;
* assegure transparência, rastreabilidade e documentação das decisões relevantes;
* promova a evolução contínua, estável e sustentável da implementação oficial;
* forneça uma base consistente para a manutenção de longo prazo da implementação oficial da linguagem Capi.

---

# 34 Roadmap da Implementação Oficial

## 34.1 Objetivo

Esta seção estabelece o roadmap da implementação oficial da linguagem Capi.

Seu objetivo é definir os princípios que orientarão sua evolução de longo prazo, preservando a estabilidade arquitetural, a conformidade com a especificação da linguagem e a sustentabilidade do projeto.

---

## 34.2 Escopo

O roadmap aplica-se exclusivamente à implementação oficial da linguagem Capi.

Ele não estabelece cronogramas obrigatórios nem impõe restrições à evolução de implementações independentes.

---

## 34.3 Princípio Geral

A implementação oficial deverá evoluir continuamente, preservando os contratos permanentes definidos por esta especificação.

Toda evolução deverá priorizar qualidade, estabilidade e sustentabilidade.

---

## 34.4 Evolução Incremental

A evolução da implementação deverá ocorrer de forma incremental.

Cada etapa deverá produzir uma implementação funcional, validada e compatível com os princípios arquiteturais definidos nesta especificação.

---

## 34.5 Priorização

A definição das prioridades de desenvolvimento deverá considerar, entre outros fatores:

* corretude funcional;
* estabilidade;
* qualidade da implementação;
* desempenho;
* sustentabilidade;
* necessidades da comunidade.

---

## 34.6 Estabilidade Arquitetural

A evolução da implementação deverá preservar a arquitetura estabelecida por esta especificação.

Alterações arquiteturais significativas deverão ser excepcionalmente justificadas e aprovadas conforme o processo de governança.

---

## 34.7 Evolução do Frontend

O frontend poderá evoluir continuamente para ampliar a qualidade da análise léxica, sintática e semântica, desde que preserve integralmente a semântica da linguagem.

---

## 34.8 Evolução do Middle-end

O middle-end poderá incorporar novas análises, verificações e otimizações internas, preservando os contratos estabelecidos para a Intermediate Representation (IR).

---

## 34.9 Evolução dos Backends

Novos backends poderão ser incorporados conforme os critérios definidos no Capítulo 21.

A evolução dos backends deverá preservar a Backend API como contrato permanente da implementação.

---

## 34.10 Evolução do Runtime

O Runtime poderá receber melhorias relacionadas à eficiência, portabilidade e manutenção, desde que preservados os contratos definidos por esta especificação.

---

## 34.11 Evolução da Biblioteca Padrão

A biblioteca padrão poderá evoluir continuamente por meio da inclusão de novas funcionalidades compatíveis com os princípios da linguagem.

Alterações incompatíveis deverão seguir as políticas de evolução definidas pela implementação oficial.

---

## 34.12 Evolução da Toolchain

A toolchain poderá incorporar novas ferramentas destinadas a melhorar o desenvolvimento, a documentação, os testes e a distribuição da linguagem.

Ferramentas opcionais não deverão comprometer o funcionamento do núcleo da implementação oficial.

---

## 34.13 Otimizações

O roadmap deverá prever evolução contínua dos mecanismos de otimização da implementação.

Entretanto, otimizações nunca deverão comprometer a corretude funcional da linguagem.

---

## 34.14 Desempenho

Melhorias de desempenho deverão ser continuamente perseguidas por meio de medições objetivas, benchmarks e análises técnicas.

---

## 34.15 Portabilidade

A implementação deverá buscar ampliar continuamente o conjunto de plataformas oficialmente suportadas, observadas as limitações técnicas e os critérios definidos pela governança.

---

## 34.16 Auto-hospedagem

Após a conclusão da auto-hospedagem, o roadmap deverá priorizar a evolução da implementação escrita em Capi.

---

## 34.17 Qualidade

O roadmap deverá contemplar melhorias contínuas relacionadas à:

* arquitetura;
* diagnósticos;
* testes;
* documentação;
* qualidade do código;
* experiência de desenvolvimento.

---

## 34.18 Sustentabilidade

A evolução da implementação deverá considerar continuamente sua sustentabilidade técnica e organizacional de longo prazo.

---

## 34.19 Inovação

A implementação oficial poderá incorporar novas tecnologias, metodologias e ferramentas sempre que demonstrarem benefícios objetivos para o projeto.

A adoção dessas tecnologias constitui detalhe de implementação.

---

## 34.20 Componentes Experimentais

O roadmap poderá incluir iniciativas experimentais destinadas à avaliação de novas arquiteturas, algoritmos ou tecnologias.

Sua promoção para componentes oficiais dependerá do processo de governança.

---

## 34.21 Compatibilidade

A evolução da implementação deverá preservar, sempre que aplicável:

* a Backend API;
* a Intermediate Representation (IR);
* o Runtime;
* a ABI;
* os contratos da biblioteca padrão.

---

## 34.22 Planejamento

O roadmap deverá permanecer suficientemente flexível para adaptar-se à evolução tecnológica sem comprometer os princípios permanentes da implementação oficial.

---

## 34.23 Revisão Contínua

O roadmap deverá ser revisado periodicamente para refletir a evolução da implementação, da linguagem e das necessidades do projeto.

---

## 34.24 Transparência

Sempre que possível, o roadmap deverá permanecer publicamente documentado, permitindo acompanhar a evolução planejada da implementação oficial.

---

## 34.25 Documentação

Toda evolução significativa prevista pelo roadmap deverá possuir documentação técnica correspondente.

---

## 34.26 Governança

A definição, revisão e priorização do roadmap constituem responsabilidade da governança da implementação oficial.

---

## 34.27 Independência da Especificação

O roadmap trata exclusivamente da evolução da implementação oficial.

Ele não modifica nem complementa a especificação da linguagem Capi.

---

## 34.28 Evolução Permanente

A implementação oficial deverá ser compreendida como um projeto em evolução contínua.

A conclusão de marcos específicos não representa o encerramento de seu desenvolvimento.

---

## 34.29 Neutralidade Tecnológica

O roadmap não estabelece compromisso permanente com tecnologias específicas.

A implementação poderá adotar novas soluções sempre que compatíveis com os princípios arquiteturais definidos nesta especificação.

---

## 34.30 Visão de Longo Prazo

A visão de longo prazo da implementação oficial consiste em manter um compilador:

* totalmente auto-hospedado;
* arquiteturalmente estável;
* tecnicamente sustentável;
* continuamente evolutivo;
* compatível com a especificação da linguagem;
* preparado para acompanhar a evolução das plataformas e tecnologias futuras.

---

## 34.31 Invariantes

Durante toda a evolução da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* a implementação evolui de forma incremental e sustentável;
* a arquitetura permanece subordinada à especificação da linguagem;
* desempenho não possui prioridade sobre corretude funcional;
* a Backend API, a IR, o Runtime e a ABI permanecem contratos permanentes da implementação;
* a adoção de novas tecnologias constitui detalhe de implementação;
* o roadmap representa um planejamento evolutivo, e não um compromisso rígido de cronograma.

---

## 34.32 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir um roadmap que:

* estabeleça uma visão clara para sua evolução de longo prazo;
* preserve os princípios arquiteturais definidos nesta especificação;
* permita incorporar novas tecnologias de forma controlada;
* assegure a continuidade técnica e organizacional do projeto;
* forneça uma base sustentável para a evolução permanente da implementação oficial da linguagem Capi.

---

# 35 Critérios para Versão Estável

## 35.1 Objetivo

Esta seção estabelece os critérios normativos para que uma versão da implementação oficial da linguagem Capi possa ser formalmente declarada estável.

Seu objetivo é definir requisitos técnicos, objetivos e verificáveis que assegurem maturidade suficiente para utilização em ambientes de produção.

---

## 35.2 Escopo

Os critérios definidos nesta seção aplicam-se exclusivamente à implementação oficial da linguagem Capi.

Eles não estabelecem requisitos obrigatórios para implementações independentes.

---

## 35.3 Definição de Versão Estável

Para os fins desta especificação, considera-se versão estável aquela que atende integralmente aos critérios estabelecidos nesta seção e cuja utilização é recomendada para ambientes de produção.

---

## 35.4 Princípio Geral

A declaração de uma versão estável deverá basear-se em evidências técnicas objetivas.

A estabilidade não deverá ser determinada exclusivamente pelo tempo de desenvolvimento ou pela quantidade de funcionalidades implementadas.

---

## 35.5 Conformidade com a Especificação

A implementação deverá demonstrar conformidade com todos os contratos definidos pela especificação da linguagem aplicáveis à versão declarada estável.

---

## 35.6 Estabilidade Funcional

As funcionalidades consideradas integrantes da versão estável deverão apresentar comportamento consistente, previsível e validado.

---

## 35.7 Arquitetura

A arquitetura da implementação deverá encontrar-se estabilizada, preservando os princípios definidos nesta especificação.

Alterações arquiteturais significativas deverão ocorrer preferencialmente entre ciclos de desenvolvimento, evitando comprometer versões estáveis.

---

## 35.8 Auto-hospedagem

A implementação oficial deverá estar integralmente auto-hospedada, conforme os critérios estabelecidos no Capítulo 27.

---

## 35.9 Testes

Toda a suíte oficial de testes deverá ser executada com sucesso antes da declaração de uma versão estável.

---

## 35.10 Testes Diferenciais

Quando existirem múltiplos backends oficialmente suportados, os testes diferenciais deverão demonstrar equivalência funcional entre eles.

---

## 35.11 Regressões

Não deverão existir regressões funcionais conhecidas que comprometam a utilização normal da implementação.

---

## 35.12 Qualidade

A implementação deverá demonstrar qualidade compatível com sua utilização em produção, considerando, entre outros aspectos:

* corretude;
* estabilidade;
* manutenibilidade;
* portabilidade;
* desempenho.

---

## 35.13 Desempenho

O desempenho da implementação deverá atender aos objetivos definidos para a versão correspondente.

Melhorias futuras não impedem a declaração de estabilidade quando os requisitos mínimos forem atendidos.

---

## 35.14 Portabilidade

A implementação deverá operar corretamente nas plataformas oficialmente suportadas para a versão declarada estável.

---

## 35.15 Runtime

O Runtime deverá apresentar comportamento estável e compatível com os contratos definidos por esta especificação.

---

## 35.16 Biblioteca Padrão

A biblioteca padrão deverá possuir nível de maturidade compatível com o conjunto de funcionalidades oficialmente suportadas pela versão estável.

---

## 35.17 Toolchain

As ferramentas integrantes da toolchain oficial deverão apresentar comportamento consistente, documentação adequada e integração compatível com a implementação oficial.

---

## 35.18 Documentação

Toda a documentação oficial correspondente à versão estável deverá permanecer completa, atualizada e consistente com a implementação disponibilizada.

---

## 35.19 Diagnósticos

Os diagnósticos produzidos pela implementação deverão apresentar qualidade compatível com uso em produção, observando os critérios estabelecidos no Capítulo 29.

---

## 35.20 Integração Contínua

A versão estável deverá ser resultado de um processo contínuo de validação automatizada conforme definido no Capítulo 30.

---

## 35.21 Segurança

Falhas críticas de segurança conhecidas deverão ser corrigidas antes da declaração de uma versão estável.

---

## 35.22 Compatibilidade

A versão estável deverá observar as políticas oficiais de compatibilidade adotadas pela implementação oficial.

---

## 35.23 Evidências Técnicas

A declaração de estabilidade deverá ser acompanhada de evidências técnicas suficientes para demonstrar o atendimento dos critérios definidos nesta seção.

Essas evidências poderão incluir, entre outras:

* resultados da suíte oficial de testes;
* resultados dos testes diferenciais;
* benchmarks;
* relatórios de validação;
* documentação técnica.

---

## 35.24 Transparência

Os critérios utilizados para declarar uma versão estável deverão permanecer publicamente documentados.

---

## 35.25 Governança

A decisão de declarar uma versão estável constitui responsabilidade da governança da implementação oficial.

---

## 35.26 Revisão

A condição de versão estável poderá ser revista caso sejam posteriormente identificadas falhas críticas incompatíveis com os critérios estabelecidos nesta seção.

---

## 35.27 Evolução

A existência de uma versão estável não impede a continuidade do desenvolvimento da implementação oficial.

Novas funcionalidades deverão ser desenvolvidas preservando a estabilidade das versões oficialmente suportadas.

---

## 35.28 Neutralidade Tecnológica

A condição de versão estável independe das tecnologias empregadas na implementação.

O requisito fundamental é o atendimento aos contratos definidos pela especificação da linguagem.

---

## 35.29 Independência da Especificação

A declaração de uma versão estável não modifica a especificação da linguagem Capi.

Ela representa exclusivamente um marco da implementação oficial.

---

## 35.30 Visão de Longo Prazo

Versões estáveis deverão constituir referências confiáveis para adoção da linguagem Capi em ambientes de produção, preservando equilíbrio entre inovação, estabilidade e sustentabilidade.

---

## 35.31 Invariantes

Durante todo o ciclo de vida das versões estáveis deverão permanecer verdadeiros os seguintes princípios:

* a implementação permanece integralmente conforme a especificação da linguagem;
* estabilidade resulta de validação técnica objetiva;
* toda versão estável possui documentação, testes e evidências compatíveis com seu nível de maturidade;
* a evolução contínua da implementação não compromete a confiabilidade das versões estáveis;
* a auto-hospedagem, a qualidade e a governança permanecem fundamentos permanentes da implementação oficial;
* a declaração de estabilidade constitui um marco da implementação, e não da especificação da linguagem.

---

## 35.32 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir critérios objetivos que:

* permitam declarar formalmente versões estáveis da implementação;
* assegurem maturidade suficiente para utilização em ambientes de produção;
* preservem a conformidade com a especificação da linguagem;
* garantam transparência, qualidade e rastreabilidade do processo de validação;
* estabeleçam uma referência confiável para a evolução contínua da implementação oficial da linguagem Capi.

---

# 36 Considerações Finais

## 36.1 Objetivo

Esta seção consolida os princípios estabelecidos ao longo deste documento, sintetizando a visão arquitetural da implementação oficial da linguagem Capi e estabelecendo os fundamentos permanentes que deverão orientar sua evolução.

---

## 36.2 Escopo

As disposições desta seção aplicam-se exclusivamente à implementação oficial da linguagem Capi.

Elas não alteram, complementam ou substituem a especificação da linguagem.

---

## 36.3 Implementação como Referência

A implementação oficial constitui a implementação de referência da linguagem Capi.

Sua finalidade é demonstrar, validar e materializar os contratos definidos pela especificação da linguagem.

---

## 36.4 Separação entre Linguagem e Implementação

A linguagem Capi permanece conceitualmente independente de qualquer implementação específica.

A implementação oficial representa apenas uma realização concreta da especificação, não sua definição.

---

## 36.5 Separação entre Garantias e Implementação

Durante toda a evolução da implementação oficial deverá permanecer preservado o Princípio da Separação entre Garantias e Implementação.

As garantias oferecidas pela linguagem são definidas exclusivamente por sua especificação.

As técnicas, algoritmos, estruturas de dados e tecnologias utilizadas para implementá-las constituem detalhes de implementação.

---

## 36.6 Arquitetura Permanente

A arquitetura definida ao longo deste documento constitui a base permanente para a evolução da implementação oficial.

Sua evolução deverá ocorrer de forma incremental, preservando seus contratos fundamentais.

---

## 36.7 Bootstrap

O processo de bootstrap representa o mecanismo utilizado para estabelecer a implementação inicial da linguagem.

Sua existência constitui etapa da evolução da implementação, não característica permanente da linguagem.

---

## 36.8 Auto-hospedagem

A auto-hospedagem representa um marco de maturidade da implementação oficial.

Após sua obtenção, o compilador passa a evoluir utilizando predominantemente a própria linguagem Capi, preservando integralmente sua arquitetura e seus contratos.

---

## 36.9 Evolução Contínua

A implementação oficial deverá permanecer em evolução contínua.

Novas funcionalidades, otimizações, tecnologias e melhorias poderão ser incorporadas desde que preservem os princípios definidos por esta especificação.

---

## 36.10 Neutralidade Tecnológica

Este documento não estabelece compromisso permanente com linguagens de implementação, bibliotecas, frameworks, compiladores, sistemas operacionais ou tecnologias específicas.

A implementação oficial poderá evoluir tecnologicamente sempre que essa evolução preservar os contratos definidos pela especificação da linguagem.

---

## 36.11 Modularidade

A modularidade constitui princípio permanente da implementação oficial.

A evolução dos diferentes componentes deverá buscar máximo desacoplamento e alta coesão, favorecendo manutenção, testes e evolução contínua.

---

## 36.12 Sustentabilidade

A implementação oficial deverá ser conduzida de forma sustentável, considerando continuamente aspectos técnicos, organizacionais e comunitários que assegurem sua continuidade de longo prazo.

---

## 36.13 Qualidade

A qualidade da implementação deverá ser continuamente perseguida por meio de:

* arquitetura consistente;
* testes automatizados;
* diagnósticos de alta qualidade;
* integração contínua;
* documentação técnica;
* revisão permanente do código.

---

## 36.14 Governança

A governança da implementação oficial deverá assegurar que toda evolução permaneça alinhada aos princípios estabelecidos pela especificação da linguagem e por este documento.

---

## 36.15 Implementações Independentes

A existência da implementação oficial não limita nem restringe o desenvolvimento de implementações independentes da linguagem Capi.

Toda implementação poderá adotar soluções arquiteturais próprias, desde que preserve conformidade com a especificação da linguagem.

---

## 36.16 Visão de Longo Prazo

A implementação oficial deverá buscar consolidar-se como uma referência técnica caracterizada por:

* estabilidade;
* corretude;
* simplicidade arquitetural;
* desempenho;
* sustentabilidade;
* portabilidade;
* facilidade de manutenção.

---

## 36.17 Filosofia da Implementação

Toda decisão relacionada à implementação oficial deverá buscar equilíbrio entre inovação tecnológica e estabilidade arquitetural.

A evolução contínua constitui parte integrante da identidade do projeto.

---

## 36.18 Documentação

A documentação da implementação oficial deverá evoluir continuamente em conjunto com o código-fonte, preservando consistência, clareza e rastreabilidade.

---

## 36.19 Transparência

Os processos de desenvolvimento, validação, evolução e governança da implementação oficial deverão permanecer, sempre que possível, transparentes e documentados.

---

## 36.20 Continuidade

A conclusão dos marcos definidos neste documento não representa o encerramento do desenvolvimento da implementação oficial.

Ela estabelece apenas uma base sólida para sua evolução permanente.

---

## 36.21 Independência da Especificação

Este documento descreve exclusivamente a implementação oficial da linguagem Capi.

A especificação da linguagem permanece o único documento normativo responsável por definir sua semântica, seus contratos e suas garantias.

---

## 36.22 Consolidação

Com a conclusão deste documento passam a estar formalmente definidos:

* a arquitetura da implementação oficial;
* o processo de bootstrap;
* o modelo de evolução incremental;
* a estratégia de auto-hospedagem;
* a arquitetura de backends;
* a política de testes;
* os mecanismos de qualidade;
* a integração contínua;
* a governança da implementação;
* o roadmap de evolução de longo prazo.

---

## 36.23 Síntese Normativa

Todos os capítulos deste documento deverão ser interpretados de forma integrada.

Na ocorrência de conflitos aparentes, prevalecem os princípios fundamentais estabelecidos pela especificação da linguagem e pelo Princípio da Separação entre Garantias e Implementação.

---

## 36.24 Invariantes

Durante toda a existência da implementação oficial deverão permanecer verdadeiros os seguintes princípios:

* a implementação permanece subordinada à especificação da linguagem;
* a arquitetura evolui de forma incremental e sustentável;
* bootstrap e auto-hospedagem constituem etapas da implementação, não da linguagem;
* a Backend API, a Intermediate Representation (IR), o Runtime e a ABI permanecem contratos permanentes da implementação;
* tecnologias específicas podem ser substituídas sem alterar os contratos definidos pela especificação;
* qualidade, testes, documentação, governança e integração contínua permanecem fundamentos permanentes da implementação oficial;
* implementações independentes da linguagem permanecem plenamente possíveis e compatíveis com esta especificação.

---

## 36.25 Encerramento

Este documento estabelece o plano normativo da implementação oficial da linguagem Capi, definindo sua arquitetura, seu processo de construção, sua evolução e seus mecanismos permanentes de qualidade.

Sua finalidade é servir como referência para o desenvolvimento, manutenção e evolução do compilador oficial, preservando continuamente os princípios arquiteturais da linguagem e assegurando que sua implementação permaneça tecnicamente consistente, sustentável e fiel à especificação.

---

## 36.26 Critérios de Conclusão

Esta seção será considerada concluída quando a implementação oficial possuir:

* uma arquitetura integralmente alinhada à especificação da linguagem;
* um processo completo de bootstrap e auto-hospedagem;
* mecanismos permanentes de validação, qualidade e governança;
* um roadmap sustentável para sua evolução de longo prazo;
* uma base técnica capaz de assegurar que a implementação oficial da linguagem Capi permaneça correta, estável, evolutiva e independente das tecnologias utilizadas para sua realização.
