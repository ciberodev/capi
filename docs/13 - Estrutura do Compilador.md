# 13 — Estrutura do Compilador

## Especificação de Implementação da Linguagem Capi

**Versão:** 1.0 (Draft)

---

# 1. Introdução

## 1.1 Objetivo

Este documento define a **estrutura de implementação do compilador oficial da linguagem Capi**.

Enquanto o Documento 06 — Arquitetura do Compilador estabelece a arquitetura lógica do processo de compilação, este documento descreve como essa arquitetura é materializada em uma implementação concreta, organizando seus componentes, módulos, responsabilidades e fluxo interno de dados.

Seu objetivo é servir como referência para o desenvolvimento do compilador oficial da linguagem, estabelecendo uma organização consistente, modular e evolutiva.

---

## 1.2 Escopo

Este documento especifica:

* a organização estrutural do compilador;
* a divisão em módulos;
* as responsabilidades de cada componente;
* o pipeline interno de compilação;
* os artefatos produzidos durante cada fase;
* os componentes de infraestrutura compartilhados;
* os princípios arquiteturais da implementação.

Não fazem parte deste documento:

* os algoritmos específicos do Lexer;
* a implementação do Parser;
* a estrutura detalhada da AST;
* a definição da HIR;
* os algoritmos de inferência de tipos;
* a estrutura da IR;
* os passes de otimização;
* a geração de código para arquiteturas específicas.

Esses assuntos são tratados pelos documentos subsequentes desta especificação de implementação.

---

## 1.3 Relação com os Documentos Anteriores

Este documento complementa diretamente a especificação da linguagem.

Em especial, sua implementação deve preservar integralmente as garantias estabelecidas por:

| Documento    | Responsabilidade                 |
| ------------ | -------------------------------- |
| Documento 01 | Fundamentos da Linguagem         |
| Documento 02 | Sistema de Tipos                 |
| Documento 03 | Modelo de Memória                |
| Documento 04 | Sintaxe                          |
| Documento 05 | Semântica Operacional            |
| Documento 06 | Arquitetura do Compilador        |
| Documento 07 | Intermediate Representation (IR) |
| Documento 09 | Runtime Mínimo                   |
| Documento 10 | ABI e Interoperabilidade         |

A estrutura apresentada neste documento representa uma implementação compatível com esses documentos, sem alterar suas garantias ou comportamento observável.

---

## 1.4 Filosofia da Implementação

O compilador oficial da Capi adota uma arquitetura baseada em componentes independentes, cada um responsável por uma única etapa claramente definida do processo de compilação.

Essa organização busca:

* reduzir o acoplamento entre módulos;
* facilitar testes unitários;
* permitir evolução independente das fases do compilador;
* favorecer reutilização de componentes;
* simplificar a manutenção do código.

Cada módulo deve possuir responsabilidades claramente delimitadas e comunicar-se com os demais exclusivamente por meio de artefatos formais produzidos durante a compilação.

---

## 1.5 Separação entre Especificação e Implementação

Assim como nos documentos que definem a linguagem, este documento mantém o princípio da separação entre **garantias** e **implementação**.

A especificação da linguagem define **o comportamento que deve ser preservado**.

Este documento define **uma organização recomendada para implementar esse comportamento**.

Outras implementações poderão adotar arquiteturas diferentes, desde que preservem integralmente:

* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* a IR especificada oficialmente;
* a ABI oficial;
* todas as garantias observáveis da linguagem.

---

# 2. Objetivos da Arquitetura

A arquitetura do compilador foi concebida para servir como base de longo prazo para a evolução da linguagem Capi.

Seus princípios orientam todas as decisões relacionadas à organização interna da implementação e procuram equilibrar simplicidade, desempenho, extensibilidade e facilidade de manutenção.

---

## 2.1 Simplicidade

Cada componente do compilador deve possuir responsabilidades claramente definidas.

Sempre que possível, uma fase deve realizar apenas uma transformação conceitual, produzindo um artefato bem definido para a etapa seguinte.

Essa abordagem reduz a complexidade do sistema e facilita sua compreensão por novos colaboradores.

---

## 2.2 Modularidade

O compilador deve ser organizado em módulos independentes.

Cada módulo deve:

* possuir uma interface pública bem definida;
* ocultar seus detalhes internos de implementação;
* depender apenas dos artefatos produzidos pelas fases anteriores;
* evitar dependências circulares.

Essa organização favorece manutenção, reutilização e evolução independente dos componentes.

---

## 2.3 Testabilidade

Cada etapa do compilador deve poder ser testada isoladamente.

Isso implica que:

* cada módulo possa receber entradas bem definidas;
* os artefatos produzidos sejam determinísticos;
* erros possam ser reproduzidos de forma consistente;
* testes unitários não dependam da execução completa do compilador.

Além dos testes unitários, a arquitetura deve favorecer testes de integração, regressão e validação do pipeline completo.

---

## 2.4 Determinismo

O compilador deve produzir resultados determinísticos.

Dadas:

* a mesma versão do compilador;
* o mesmo código-fonte;
* as mesmas dependências;
* as mesmas opções de compilação;

os artefatos produzidos deverão ser semanticamente idênticos.

Esse princípio é fundamental para garantir reprodutibilidade, depuração e confiabilidade do processo de compilação.

---

## 2.5 Compilação Incremental

A arquitetura deve permitir a incorporação futura de mecanismos de compilação incremental.

Embora uma implementação inicial possa realizar compilações completas, sua organização interna deve facilitar:

* reutilização de artefatos intermediários;
* detecção de alterações;
* recompilação mínima;
* cache entre execuções.

A adoção efetiva desses mecanismos pertence à evolução do compilador e não altera sua arquitetura fundamental.

---

## 2.6 Evolução Contínua

A arquitetura deve permitir que novos recursos da linguagem sejam incorporados com impacto mínimo sobre os componentes existentes.

Novas fases, novos passes de otimização, novos backends e novas estratégias de compilação devem poder ser adicionados preservando a estabilidade dos módulos já existentes.

Esse princípio reduz o custo de evolução do compilador e favorece sua manutenção ao longo do tempo.

---

# 3. Visão Geral

## 3.1 Arquitetura em Camadas

O compilador da Capi é organizado em uma sequência de camadas especializadas, cada uma responsável por transformar o programa em um nível diferente de abstração.

Conceitualmente, o pipeline pode ser representado da seguinte forma:

```text
Código Fonte
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
Resolução de Nomes
      │
      ▼
Inferência e Verificação de Tipos
      │
      ▼
Verificação Semântica
      │
      ▼
Lowering
      │
      ▼
IR
      │
      ▼
Otimizações
      │
      ▼
Backend
      │
      ▼
Código Objeto / Executável
```

Cada etapa opera exclusivamente sobre o artefato produzido pela etapa imediatamente anterior.

---

## 3.2 Frontend

O Frontend é responsável por compreender o programa escrito pelo desenvolvedor e convertê-lo para uma representação semanticamente válida.

Compõem o Frontend:

* Lexer;
* Parser;
* AST;
* HIR;
* Resolução de Nomes;
* Inferência e Verificação de Tipos;
* Verificação Semântica.

Ao término do Frontend, o programa está completamente validado segundo as regras da linguagem.

---

## 3.3 Middle-end

O Middle-end inicia-se após a conclusão da Verificação Semântica e termina com a geração de uma Intermediate Representation (IR) otimizada e pronta para o Backend.

Sua responsabilidade é transformar a representação semântica validada produzida pelo Frontend em uma representação intermediária independente da plataforma de destino, aplicando todas as transformações e otimizações que não dependem da arquitetura de hardware.

Entre suas responsabilidades incluem-se:

* Lowering para IR;
* otimizações locais;
* otimizações globais;
* simplificação da IR;
* preparação da IR para geração de código.

Essa camada permanece completamente independente da arquitetura de destino, permitindo que uma mesma IR otimizada seja utilizada por diferentes implementações de Backend.

---

## 3.4 Backend

O Backend transforma a IR otimizada em código executável para uma plataforma específica.

Entre suas responsabilidades encontram-se:

* seleção de instruções;
* alocação de registradores;
* organização do layout de dados;
* emissão de código objeto;
* integração com a ABI e o Runtime.

Cada arquitetura suportada pode possuir uma implementação própria de Backend, compartilhando as mesmas fases anteriores do compilador.

O Backend deve respeitar integralmente a ABI definida no Documento 10, incluindo convenções de chamada, representação física dos tipos e organização de memória compatível com a plataforma de destino.

---

## 3.5 Infraestrutura Compartilhada

Além das fases do pipeline, o compilador possui um conjunto de componentes de infraestrutura utilizados por praticamente todos os módulos.

Esses componentes incluem, entre outros:

* gerenciamento de arquivos-fonte;
* gerenciamento de sessões de compilação;
* sistema de diagnósticos;
* internamento de strings;
* gerenciamento de memória temporária;
* configuração da compilação;
* cache de artefatos.

Esses componentes não participam diretamente da transformação do programa, mas fornecem os serviços necessários para que todas as fases do compilador operem de maneira consistente e eficiente.

---

## 3.6 Fluxo Geral de Compilação

A compilação ocorre como uma sequência ordenada de transformações, onde cada fase recebe um artefato, realiza uma operação específica e produz um novo artefato para a etapa seguinte.

Nenhuma fase deve modificar diretamente os artefatos produzidos por fases anteriores.

Sempre que necessário realizar transformações significativas, uma nova representação intermediária deve ser produzida, preservando a rastreabilidade entre os diferentes níveis de abstração do programa.

Essa organização favorece:

* previsibilidade;
* depuração;
* testes independentes;
* evolução modular;
* compilação incremental futura;
* reutilização de componentes.

---

# 4. Organização do Projeto

A implementação do compilador oficial da Capi é organizada em módulos independentes, refletindo diretamente as fases do pipeline de compilação.

Cada módulo possui uma responsabilidade claramente definida e comunica-se com os demais exclusivamente por meio de interfaces públicas e artefatos formais.

Essa organização reduz o acoplamento entre componentes, facilita testes independentes e permite evolução incremental da implementação.

---

## 4.1 Organização em Módulos

O compilador é dividido em módulos especializados, cada um responsável por uma etapa do processo de compilação ou por serviços compartilhados.

Conceitualmente, sua organização pode ser representada da seguinte forma:

```text
compiler/
│
├── driver/
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
│
├── diagnostics/
├── source/
├── session/
├── interner/
├── arena/
├── common/
│
└── runtime/
```

Essa organização representa uma divisão lógica da implementação e não impõe uma estrutura física específica de diretórios ou pacotes.

---

## 4.2 Componentes do Pipeline

Os módulos que compõem diretamente o pipeline de compilação são:

| Módulo       | Responsabilidade                                               |
| ------------ | -------------------------------------------------------------- |
| Driver       | Coordena a execução do pipeline e produz o artefato solicitado |
| Lexer        | Geração de tokens                                              |
| Parser       | Construção da AST                                              |
| AST          | Representação sintática                                        |
| HIR          | Representação semântica inicial                                |
| Resolver     | Resolução de nomes e escopos                                   |
| Type Checker | Inferência e verificação de tipos                              |
| Semantic     | Verificações semânticas                                        |
| Lowering     | Conversão da HIR para IR                                       |
| IR           | Representação intermediária                                    |
| Optimizer    | Passes de otimização                                           |
| Backend      | Geração de código                                              |

Cada módulo realiza apenas uma transformação conceitual do programa.

---

## 4.3 Componentes de Infraestrutura

Além do pipeline principal, o compilador utiliza diversos componentes de infraestrutura compartilhados.

Esses componentes não representam fases de compilação, mas serviços utilizados por praticamente todos os módulos.

Entre eles encontram-se:

| Módulo      | Responsabilidade                 |
| ----------- | -------------------------------- |
| Diagnostics | Emissão de diagnósticos          |
| Source      | Gerenciamento dos arquivos-fonte |
| Session     | Contexto global da compilação    |
| Interner    | Internamento de strings          |
| Arena       | Alocação temporária de memória   |
| Common      | Estruturas compartilhadas        |

A separação entre infraestrutura e pipeline reduz duplicação de código e melhora a coesão da implementação.

---

## 4.4 Backend Modular

O Backend poderá ser dividido internamente em implementações específicas para cada arquitetura suportada.

Exemplo conceitual:

```text
backend/
    llvm/
    x86_64/
    aarch64/
    wasm/
```

Todos compartilham a mesma IR produzida pelas fases anteriores.

Essa organização permite que novos alvos de compilação sejam adicionados sem modificar o restante do compilador.

---

## 4.5 Evolução da Estrutura

A organização apresentada neste documento representa uma arquitetura de referência.

Novos módulos poderão ser incorporados futuramente para suportar funcionalidades como:

* compilação incremental;
* cache persistente;
* análise estática;
* ferramentas de refatoração;
* geração de documentação;
* integração com IDEs.

Essas extensões deverão preservar a separação entre responsabilidades estabelecida nesta arquitetura.

---

# 5. Dependências entre Módulos

A organização do compilador segue uma arquitetura em camadas.

Cada módulo deve depender apenas dos componentes necessários para cumprir sua responsabilidade, evitando dependências circulares e reduzindo o acoplamento entre fases.

---

## 5.1 Fluxo de Dependências

Conceitualmente, as dependências entre os módulos do pipeline seguem a direção do fluxo de compilação.

```text
Driver
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
Semantic
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
```

Cada módulo conhece apenas os artefatos necessários produzidos pelas etapas anteriores.

---

## 5.2 Dependências Compartilhadas

Os componentes de infraestrutura podem ser utilizados por múltiplos módulos simultaneamente.

Conceitualmente:

```text
                 Diagnostics
                      ▲
                      │
Source ───────────────┼────────────── Session
                      │
                  Interner
                      │
                    Arena
```

Esses módulos não dependem do pipeline de compilação e fornecem serviços comuns para todas as fases.

---

## 5.3 Acoplamento Mínimo

Nenhum módulo deve acessar diretamente estruturas internas pertencentes a outro módulo.

A comunicação entre fases ocorre exclusivamente por meio de:

* interfaces públicas;
* artefatos formais;
* serviços compartilhados da infraestrutura.

Esse princípio favorece encapsulamento e reduz impactos durante a evolução da implementação.

---

## 5.4 Ausência de Dependências Circulares

Dependências circulares entre módulos são proibidas.

Em particular:

* o Lexer não depende do Parser;
* o Parser não depende do Resolver;
* a AST não depende da HIR;
* a HIR não depende da IR;
* o Backend não depende do Frontend.

Quando dois componentes necessitarem compartilhar informações, essas estruturas deverão ser movidas para um módulo comum da infraestrutura.

---

## 5.5 Estabilidade das Interfaces

As interfaces públicas entre módulos devem permanecer pequenas e estáveis.

Alterações internas em um componente não devem exigir modificações nas fases seguintes, desde que os contratos públicos permaneçam preservados.

Esse princípio reduz o custo de manutenção e facilita a substituição ou evolução de implementações específicas.

---

# 6. Infraestrutura Compartilhada

Os módulos de infraestrutura fornecem serviços reutilizáveis utilizados durante todo o processo de compilação.

Esses componentes não implementam fases do pipeline, mas oferecem abstrações comuns que evitam duplicação de código e promovem uniformidade na implementação.

---

## 6.1 Source Manager

O **Source Manager** é responsável pelo gerenciamento dos arquivos-fonte utilizados durante a compilação.

Entre suas responsabilidades incluem-se:

* carregamento de arquivos;
* identificação de módulos;
* mapeamento entre posições e arquivos;
* controle de codificação dos arquivos;
* acesso eficiente ao conteúdo do código-fonte.

Todas as referências a linhas, colunas e intervalos de texto utilizam o Source Manager como autoridade.

---

## 6.2 Session

A **Session** representa o contexto global de uma compilação.

Ela concentra informações compartilhadas por todas as fases, como:

* opções de compilação;
* plataforma de destino;
* módulos carregados;
* configurações do projeto;
* cache de artefatos;
* referências aos serviços de infraestrutura.

Cada compilação possui exatamente uma Session ativa.

---

## 6.3 Diagnostics

O sistema de **Diagnostics** centraliza toda a emissão de mensagens produzidas pelo compilador.

Entre elas:

* erros;
* avisos;
* observações;
* informações;
* sugestões de correção.

Todos os módulos utilizam esse componente, garantindo uniformidade na apresentação dos diagnósticos.

---

## 6.4 String Interner

O **String Interner** mantém uma representação única para cada cadeia de caracteres utilizada internamente pelo compilador.

Seu uso reduz:

* consumo de memória;
* comparações repetidas entre strings;
* duplicação de identificadores.

Os módulos passam a manipular identificadores internados em vez de cadeias de caracteres completas.

---

## 6.5 Arena Allocator

A implementação poderá utilizar um **Arena Allocator** para armazenar estruturas temporárias produzidas durante a compilação.

Esse mecanismo reduz significativamente o custo de alocação e liberação de memória para estruturas como:

* nós da AST;
* nós da HIR;
* tipos;
* símbolos;
* instruções intermediárias.

A estratégia de gerenciamento dessas arenas pertence à implementação.

---

## 6.6 Componentes Comuns

O módulo **Common** reúne estruturas reutilizadas por diferentes partes do compilador.

Entre elas podem estar:

* identificadores internos;
* tipos auxiliares;
* coleções especializadas;
* estruturas utilitárias;
* algoritmos compartilhados.

Esse módulo deve permanecer pequeno, coeso e independente das fases do pipeline, servindo como base comum para toda a implementação.

---

# 7. Pipeline de Compilação

O compilador da Capi organiza o processo de compilação como um pipeline composto por fases independentes e ordenadas.

Cada fase recebe um artefato formal produzido pela etapa anterior, realiza uma única transformação conceitual e produz um novo artefato para a etapa seguinte.

Essa organização favorece modularidade, testabilidade, compilação incremental e evolução independente dos componentes.

---

## 7.1 Visão Geral

O pipeline completo do compilador pode ser representado da seguinte forma:

```text
Código Fonte
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
Resolução de Nomes
      │
      ▼
Inferência e Verificação de Tipos
      │
      ▼
Verificação Semântica
      │
      ▼
Lowering
      │
      ▼
IR
      │
      ▼
Passes de Otimização
      │
      ▼
Backend
      │
      ▼
Código Objeto
      │
      ▼
Linkedição
      │
      ▼
Executável / Biblioteca
```

Embora o compilador possa realizar algumas otimizações internas de fluxo, o comportamento observável deverá ser equivalente ao pipeline apresentado.

---

## 7.2 Frontend

O Frontend compreende todas as fases responsáveis por transformar o código-fonte em uma representação completamente validada segundo a linguagem.

Suas etapas são:

* análise léxica;
* análise sintática;
* construção da AST;
* construção da HIR;
* resolução de nomes;
* inferência e verificação de tipos;
* verificação semântica.

Ao término do Frontend, qualquer programa restante é considerado semanticamente válido.

---

## 7.3 Middle-end

O Middle-end transforma a representação semântica produzida pelo Frontend em uma representação intermediária otimizada e independente da plataforma.

Suas responsabilidades incluem:

* Lowering para IR;
* otimizações locais;
* otimizações globais;
* simplificação da IR;
* preparação para geração de código.

Nenhuma dessas transformações pode alterar o comportamento observável do programa.

---

## 7.4 Backend

O Backend recebe a IR otimizada e produz código executável para uma arquitetura específica.

Entre suas responsabilidades encontram-se:

* seleção de instruções;
* alocação de registradores;
* organização do layout de dados;
* emissão de código objeto;
* integração com a ABI;
* preparação para a etapa de linkedição.

A arquitetura do Backend é independente do Frontend e do Middle-end.

---

## 7.5 Linkedição

Após a geração do código objeto, ocorre a etapa de linkedição.

Essa etapa é responsável por:

* resolver símbolos externos;
* integrar módulos compilados;
* incorporar a Biblioteca Padrão;
* integrar o Runtime Mínimo;
* produzir o artefato final.

Dependendo do alvo da compilação, o resultado poderá ser:

* executável;
* biblioteca estática;
* biblioteca dinâmica;
* módulo WebAssembly;
* outro formato suportado pela Toolchain.

---

## 7.6 Pipeline Determinístico

Cada fase do pipeline deve produzir resultados determinísticos.

Isso significa que:

* a mesma entrada produz a mesma saída;
* não existem dependências implícitas entre fases;
* nenhuma etapa depende da ordem física dos arquivos;
* a execução paralela não altera o resultado produzido.

Esse princípio constitui um requisito fundamental para a reprodutibilidade da compilação.

---

# 8. Artefatos Produzidos

Cada fase do compilador produz um artefato formal que representa o programa em um nível específico de abstração.

Esses artefatos constituem a interface pública entre as fases do compilador.

---

## 8.1 Transformações

O pipeline pode ser descrito como uma sequência de transformações entre artefatos:

| Entrada       | Fase               | Saída         |
| ------------- | ------------------ | ------------- |
| Código-fonte  | Lexer              | Tokens        |
| Tokens        | Parser             | AST           |
| AST           | Construção da HIR  | HIR           |
| HIR           | Resolução de Nomes | HIR Resolvida |
| HIR Resolvida | Type Checker       | HIR Tipada    |
| HIR Tipada    | Semantic Checker   | HIR Validada  |
| HIR Validada  | Lowering           | IR            |
| IR            | Optimizer          | IR Otimizada  |
| IR Otimizada  | Backend            | Código Objeto |
| Código Objeto | Linker             | Executável    |

Cada transformação possui responsabilidade única e claramente definida.

---

## 8.2 AST

A AST representa exclusivamente a estrutura sintática do programa.

Ela preserva:

* declarações;
* comandos;
* expressões;
* módulos;
* classes;
* funções.

A AST não contém informações semânticas completas.

---

## 8.3 HIR

A HIR (High-level Intermediate Representation) representa a estrutura semântica do programa.

Ela preserva:

* estrutura lógica da linguagem;
* escopos;
* símbolos;
* tipos;
* relações semânticas;
* informações necessárias para validação.

A HIR constitui a principal estrutura manipulada pelo Frontend.

Sua estrutura detalhada será definida no Documento 16 — Representação Semântica (HIR), onde são especificados seus nós, invariantes e relações internas. A HIR serve como base para a Resolução de Nomes, Verificação de Tipos e Verificação Semântica.

---

## 8.4 IR

A IR representa o programa em uma forma independente da sintaxe e da arquitetura de destino.

Ela constitui:

* entrada do Middle-end;
* base para otimizações;
* entrada do Backend.

Sua estrutura é definida pelo Documento 07 — Intermediate Representation.

---

## 8.5 Código Objeto

Após o Backend, o programa é representado por código objeto compatível com a plataforma de destino.

Esse artefato pode ser posteriormente combinado com outros módulos durante a linkedição.

---

## 8.6 Metadados

Além dos artefatos principais, diversas fases produzem metadados auxiliares.

Exemplos incluem:

* mapa entre HIR e AST;
* localização no código-fonte;
* informações de depuração;
* símbolos públicos;
* estatísticas internas;
* informações para compilação incremental.

Esses metadados não alteram a semântica do programa.

---

# 9. Contexto Compartilhado

Embora cada fase do compilador opere sobre artefatos específicos, diversas informações precisam permanecer disponíveis durante toda a compilação.

Essas informações são reunidas em um contexto compartilhado denominado **Session**.

A Session atua como ponto central de coordenação da compilação, evitando dependências diretas entre módulos.

---

## 9.1 Objetivo

A Session reúne todos os componentes compartilhados utilizados durante uma compilação.

Ela permite que diferentes fases utilizem serviços comuns sem depender diretamente umas das outras.

Essa abordagem reduz acoplamento e facilita testes independentes.

---

## 9.2 Informações Compartilhadas

Entre as informações normalmente armazenadas na Session encontram-se:

* opções de compilação;
* arquitetura de destino;
* plataforma;
* configuração do projeto;
* módulos carregados;
* cache de compilação;
* Source Manager;
* Diagnostics;
* String Interner;
* Arena Allocator.

Esses elementos permanecem disponíveis durante toda a execução do compilador.

---

## 9.3 Serviços Compartilhados

Além dos dados propriamente ditos, a Session disponibiliza serviços utilizados por diferentes fases.

Exemplos:

* carregamento de módulos;
* consulta de arquivos;
* emissão de diagnósticos;
* resolução de caminhos;
* acesso ao sistema de arquivos;
* gerenciamento de cache.

Esses serviços permanecem independentes do pipeline de compilação.

---

## 9.4 Ciclo de Vida

Uma Session é criada no início da compilação e permanece ativa até a geração do artefato final.

Ao término da compilação:

* todos os artefatos temporários são liberados;
* arenas são descartadas;
* caches persistentes podem ser atualizados;
* estatísticas podem ser registradas.

Cada compilação utiliza exatamente uma Session independente.

---

## 9.5 Compartilhamento Controlado

A Session não deve ser utilizada como mecanismo de comunicação arbitrária entre módulos.

Informações pertencentes ao pipeline devem continuar sendo transmitidas exclusivamente através dos artefatos formais produzidos por cada fase.

A Session deve concentrar apenas:

* configuração;
* infraestrutura;
* serviços compartilhados;
* estado global da compilação.

Essa separação preserva a modularidade da arquitetura e evita dependências implícitas entre as fases do compilador.

---

## 9.6 Rastreabilidade

Todos os artefatos produzidos durante a compilação devem manter referências suficientes para permitir a rastreabilidade entre os diferentes níveis de representação.

Isso inclui, quando aplicável:

* associação entre nós da AST e da HIR;
* correspondência entre elementos da HIR e da IR;
* localização original no código-fonte;
* informações utilizadas pelos diagnósticos;
* metadados para depuração.

Essa rastreabilidade facilita a emissão de diagnósticos precisos, a geração de informações de depuração e a análise do comportamento interno do compilador, sem comprometer a independência entre as fases do pipeline.

---

# 10. Estratégia de Execução

A implementação do compilador da Capi adota um modelo de execução baseado em fases sucessivas e independentes.

Cada fase inicia somente após a conclusão da anterior, recebendo como entrada apenas os artefatos produzidos pelo pipeline e os serviços disponibilizados pela infraestrutura compartilhada.

Essa estratégia favorece previsibilidade, isolamento entre componentes e facilita a depuração do processo de compilação.

---

## 10.1 Modelo de Execução

O compilador executa cada fase como uma transformação completa sobre um conjunto de artefatos.

Conceitualmente:

```text
Artefato A
     │
     ▼
  Fase N
     │
     ▼
Artefato B
```

Cada transformação deve ser determinística e produzir um novo artefato consistente antes que a próxima fase seja iniciada.

---

## 10.2 Execução Sequencial

A implementação de referência executa o pipeline de forma sequencial.

Essa abordagem simplifica:

* a implementação inicial;
* a depuração;
* a validação do compilador;
* o bootstrap da linguagem.

Embora implementações futuras possam explorar paralelismo, a semântica do pipeline permanece equivalente à execução sequencial.

---

## 10.3 Isolamento entre Fases

Cada fase deve operar exclusivamente sobre seus próprios dados de trabalho.

Não é permitido modificar diretamente:

* artefatos produzidos por fases anteriores;
* estruturas internas pertencentes a outros módulos;
* estados privados de componentes distintos.

Quando uma transformação significativa for necessária, deverá ser produzido um novo artefato.

---

## 10.4 Execução Sob Demanda

Nem todas as fases precisam ser executadas em todas as invocações do compilador.

Por exemplo:

* `check` pode interromper a execução após a verificação semântica;
* ferramentas de documentação podem utilizar apenas o Frontend;
* ferramentas de análise estática podem operar sobre a HIR;
* ferramentas de depuração podem solicitar a geração da IR.

A arquitetura deve permitir que diferentes ferramentas utilizem subconjuntos do pipeline.

---

## 10.5 Cancelamento da Compilação

Caso uma fase identifique erros fatais que impeçam a continuidade da compilação, o pipeline poderá ser interrompido.

Entretanto, sempre que possível, o compilador deve concluir a análise da fase corrente para produzir o maior número possível de diagnósticos antes do encerramento.

Essa estratégia reduz o número de ciclos de edição e compilação necessários durante o desenvolvimento.

---

## 10.6 Paralelização Futura

Embora a implementação inicial utilize execução sequencial, sua arquitetura deve permitir futura paralelização.

Exemplos incluem:

* compilação paralela de módulos independentes;
* execução concorrente de passes de otimização compatíveis;
* geração simultânea de código para múltiplos arquivos.

Essas otimizações não poderão alterar o comportamento determinístico do compilador.

---

# 11. Compilação Incremental

A arquitetura do compilador foi concebida para suportar compilação incremental, reduzindo o trabalho necessário quando apenas parte do código-fonte sofre alterações.

Embora esse recurso possa não estar presente nas primeiras versões da implementação, sua futura incorporação não deverá exigir alterações estruturais significativas na arquitetura.

---

## 11.1 Objetivo

A compilação incremental busca evitar a recompilação completa do projeto sempre que possível.

Seu objetivo é reutilizar artefatos previamente produzidos, recompilando apenas os componentes efetivamente afetados por uma alteração.

---

## 11.2 Unidades de Compilação

Cada módulo constitui uma unidade independente de compilação.

Sempre que um módulo for modificado, o compilador deverá identificar:

* quais artefatos tornaram-se inválidos;
* quais dependências foram afetadas;
* quais fases precisam ser reexecutadas.

Essa granularidade permite reduzir significativamente o tempo de compilação em projetos de grande porte.

---

## 11.3 Cache de Artefatos

A implementação poderá manter um cache contendo artefatos produzidos durante compilações anteriores.

Entre eles:

* tokens;
* AST;
* HIR;
* IR;
* metadados;
* símbolos públicos.

O formato físico desse cache pertence exclusivamente à implementação.

O cache deve ser invalidado sempre que um arquivo-fonte for modificado ou quando as opções de compilação forem alteradas. A chave do cache deve incluir, no mínimo, o conteúdo do arquivo (ou seu hash) e as opções de compilação relevantes.

---

## 11.4 Detecção de Alterações

Para determinar quais artefatos permanecem válidos, o compilador poderá utilizar mecanismos como:

* hashes de arquivos;
* hashes de módulos;
* versões de artefatos;
* árvores de dependências;
* assinaturas públicas.

A estratégia utilizada deverá garantir consistência entre o cache e o código-fonte.

---

## 11.5 Recompilação Parcial

Sempre que possível, apenas os módulos afetados deverão ser recompilados.

Essa decisão depende da análise das dependências entre módulos e da validade dos artefatos armazenados em cache.

A recompilação parcial não pode alterar o resultado final em relação a uma compilação completa.

---

## 11.6 Consistência

Caso o compilador detecte qualquer inconsistência entre o cache e o estado atual do projeto, deverá descartar os artefatos comprometidos e realizar sua reconstrução.

A corretude da compilação sempre possui prioridade sobre o reaproveitamento de artefatos.

---

# 12. Gerenciamento de Memória Interno

Durante a compilação, o compilador manipula milhões de objetos temporários, como tokens, nós da AST, símbolos, tipos e instruções intermediárias.

Para reduzir o custo de alocação e simplificar o gerenciamento desses objetos, a implementação poderá utilizar estratégias específicas de gerenciamento de memória.

Essas estratégias pertencem exclusivamente ao compilador e não possuem qualquer relação com o Modelo de Memória da linguagem Capi.

---

## 12.1 Objetivo

O gerenciamento interno de memória tem como objetivos:

* reduzir o número de alocações individuais;
* minimizar fragmentação;
* acelerar a construção das estruturas intermediárias;
* simplificar a liberação de memória ao término da compilação.

Esses mecanismos são completamente transparentes ao usuário da linguagem.

---

## 12.2 Arena Allocation

A implementação poderá utilizar **Arena Allocation** para armazenar estruturas temporárias produzidas durante cada fase.

Nessa estratégia, diversos objetos são alocados em uma única região contínua de memória.

Ao término de seu ciclo de vida, toda a arena é liberada de uma única vez.

Essa abordagem é especialmente adequada para estruturas como:

* nós da AST;
* nós da HIR;
* símbolos;
* tipos;
* instruções da IR.

O uso de Arena Allocation é recomendado, mas não obrigatório. Implementações alternativas podem utilizar outras estratégias de alocação, desde que mantenham o desempenho e a estabilidade esperados para um compilador da Capi.

---

## 12.3 Internamento de Objetos

Além do internamento de cadeias de caracteres, a implementação poderá reutilizar outras estruturas frequentemente repetidas.

Exemplos incluem:

* identificadores;
* tipos primitivos;
* assinaturas;
* qualificadores;
* operadores.

Essa estratégia reduz consumo de memória e acelera comparações por identidade.

---

## 12.4 Lifetime dos Artefatos

Cada artefato produzido pelo compilador possui um ciclo de vida claramente definido.

Por exemplo:

| Artefato      | Lifetime típico          |
| ------------- | ------------------------ |
| Tokens        | Até o término do Parsing |
| AST           | Até a geração da HIR     |
| HIR           | Até o Lowering           |
| IR            | Até a geração do código  |
| Código Objeto | Até a Linkedição         |

A implementação poderá antecipar a liberação de artefatos que não serão mais utilizados, reduzindo o consumo total de memória.

---

## 12.5 Independência da Linguagem

O gerenciamento de memória interno do compilador não faz parte da especificação da linguagem Capi.

Os mecanismos utilizados para armazenar AST, HIR, IR ou quaisquer estruturas auxiliares pertencem exclusivamente à implementação do compilador.

Alterações nesses mecanismos não afetam:

* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* a ABI;
* o comportamento observável dos programas compilados.

---

## 12.6 Evolução

Novas estratégias de gerenciamento de memória poderão ser incorporadas futuramente, incluindo:

* pools especializados;
* compactação de estruturas;
* arenas hierárquicas;
* alocadores paralelos;
* gerenciamento otimizado para compilação incremental.

Independentemente da estratégia adotada, a arquitetura deverá preservar os princípios de modularidade, determinismo e isolamento estabelecidos neste documento.

---

# 13. Tratamento de Erros

A arquitetura do compilador deve permitir a detecção, propagação e apresentação consistente de erros em todas as fases do pipeline.

Sempre que possível, o compilador deve continuar sua execução após encontrar erros, produzindo diagnósticos adicionais que auxiliem o desenvolvedor na correção do programa.

O tratamento de erros constitui parte integrante da arquitetura do compilador e complementa o sistema de diagnósticos definido pelo Documento 23 — Diagnósticos e Recuperação de Erros.

---

## 13.1 Objetivo

O sistema de tratamento de erros possui os seguintes objetivos:

* detectar inconsistências o mais cedo possível;
* preservar a estabilidade da compilação;
* permitir recuperação quando viável;
* acumular múltiplos diagnósticos em uma única execução;
* impedir que erros internos comprometam a confiabilidade do compilador.

---

## 13.2 Categorias de Erros

Durante a compilação podem ocorrer diferentes categorias de erros, incluindo:

* erros léxicos;
* erros sintáticos;
* erros de resolução de nomes;
* erros de tipos;
* erros semânticos;
* erros internos do compilador;
* falhas de infraestrutura.

Cada categoria deverá ser tratada pela fase responsável por sua detecção.

---

## 13.3 Recuperação Local

Sempre que possível, uma fase deverá recuperar-se de erros localizados para continuar processando o restante do programa.

Exemplos incluem:

* sincronização do Parser após erro de sintaxe;
* criação de símbolos inválidos para permitir continuidade da análise;
* utilização de tipos especiais para representar erros de tipagem.

A recuperação não deve comprometer a consistência das fases seguintes.

---

## 13.4 Erros Fatais

Algumas condições impedem a continuidade segura da compilação.

Exemplos:

* corrupção de estruturas internas;
* falha de leitura de arquivos essenciais;
* inconsistência irrecuperável da infraestrutura;
* erro interno do compilador.

Nessas situações, a compilação deverá ser interrompida de forma controlada, emitindo um diagnóstico apropriado.

---

## 13.5 Propagação de Erros

Erros detectados em uma fase não devem ser propagados como exceções entre módulos do pipeline.

Sempre que possível, eles devem ser representados por diagnósticos formais e por estruturas específicas que permitam à compilação prosseguir.

Esse modelo reduz o acoplamento entre módulos e facilita a implementação de estratégias de recuperação.

---

## 13.6 Robustez

O compilador deve considerar todo código-fonte como potencialmente inválido.

Nenhuma entrada produzida pelo usuário deve ser assumida como correta antes da fase responsável por sua validação.

Esse princípio aumenta a robustez da implementação e reduz a possibilidade de falhas inesperadas durante a compilação.

---

# 14. Instrumentação

Além da compilação propriamente dita, o compilador disponibiliza mecanismos destinados à observação de seu comportamento interno.

Esses mecanismos auxiliam o desenvolvimento do próprio compilador, a investigação de problemas, a análise de desempenho e a validação das transformações realizadas ao longo do pipeline.

A instrumentação não altera a semântica da compilação e pode ser habilitada ou desabilitada conforme a necessidade.

---

## 14.1 Objetivo

A instrumentação permite observar o funcionamento interno do compilador durante sua execução.

Entre seus objetivos encontram-se:

* facilitar depuração;
* medir desempenho;
* validar transformações;
* identificar gargalos;
* apoiar o desenvolvimento de novas funcionalidades.

---

## 14.2 Logging

A implementação poderá registrar eventos relevantes durante a compilação.

Entre eles:

* início e término das fases;
* carregamento de módulos;
* execução de passes;
* utilização de cache;
* estatísticas de compilação.

O nível de detalhamento do log pertence à implementação.

---

## 14.3 Tracing

A implementação poderá disponibilizar mecanismos de rastreamento detalhado das operações executadas pelo compilador.

O tracing poderá registrar, por exemplo:

* chamadas entre componentes;
* duração de operações;
* criação de artefatos;
* execução de otimizações.

Essas informações destinam-se principalmente ao desenvolvimento do compilador.

---

## 14.4 Estatísticas

O compilador poderá produzir estatísticas referentes à compilação.

Exemplos incluem:

* quantidade de arquivos processados;
* número de tokens gerados;
* quantidade de nós da AST;
* quantidade de símbolos;
* tempo gasto em cada fase;
* consumo aproximado de memória.

Essas informações auxiliam a evolução da implementação.

---

## 14.5 Dump dos Artefatos

Para fins de depuração e desenvolvimento, o compilador poderá permitir a visualização dos artefatos produzidos em cada fase.

Entre eles:

* lista de tokens;
* AST;
* HIR;
* tabela de símbolos;
* IR;
* IR otimizada.

Esses mecanismos são destinados exclusivamente ao desenvolvimento do compilador e não fazem parte da linguagem.

---

## 14.6 Independência

Os mecanismos de instrumentação não devem influenciar o comportamento observável da compilação.

Independentemente de estarem habilitados ou não, o compilador deve produzir exatamente os mesmos artefatos finais.

---

# 15. Testabilidade

A arquitetura do compilador foi concebida para favorecer a construção de testes automatizados em todos os níveis da implementação.

Cada módulo deve poder ser validado isoladamente, reduzindo a complexidade dos testes e aumentando a confiabilidade da evolução do compilador.

---

## 15.1 Objetivo

A estratégia de testes possui os seguintes objetivos:

* validar individualmente cada componente;
* detectar regressões;
* garantir estabilidade do pipeline;
* facilitar refatorações;
* preservar compatibilidade entre versões.

---

## 15.2 Testes Unitários

Cada módulo do compilador deverá possuir testes unitários próprios.

Esses testes verificam o comportamento isolado de componentes como:

* Lexer;
* Parser;
* Resolver;
* Type Checker;
* Lowering;
* Optimizer;
* Backend.

Os testes unitários não devem depender da execução completa do pipeline.

---

## 15.3 Testes de Integração

Além dos testes unitários, o compilador deverá possuir testes que validem a interação entre fases consecutivas.

Exemplos incluem:

* Lexer → Parser;
* Parser → AST;
* AST → HIR;
* HIR → Type Checker;
* IR → Backend.

Esses testes verificam a compatibilidade entre os artefatos produzidos.

---

## 15.4 Testes de Regressão

Toda correção de defeito deverá resultar na criação de um teste de regressão correspondente.

Esse conjunto de testes tem como objetivo impedir que problemas anteriormente corrigidos retornem em versões futuras do compilador.

---

## 15.5 Golden Tests

Sempre que apropriado, a implementação poderá utilizar *Golden Tests*.

Nessa abordagem, a saída produzida por uma fase é comparada com um artefato previamente validado.

Exemplos:

* sequência de tokens;
* AST serializada;
* HIR;
* IR;
* diagnósticos emitidos.

Essa estratégia facilita a validação de grandes quantidades de casos de teste.

---

## 15.6 Benchmarks

A implementação poderá incluir benchmarks destinados a acompanhar o desempenho do compilador ao longo de sua evolução.

Entre os aspectos monitorados encontram-se:

* tempo de compilação;
* consumo de memória;
* desempenho das otimizações;
* escalabilidade em projetos de grande porte.

Os resultados desses benchmarks auxiliam decisões de evolução da implementação, mas não alteram os contratos definidos por esta especificação.

---

# 16. Extensibilidade

A arquitetura do compilador da Capi foi concebida para evoluir continuamente sem exigir alterações estruturais significativas nos componentes existentes.

Novas funcionalidades devem ser incorporadas por meio da adição de novos módulos, fases ou passes, preservando os contratos públicos estabelecidos entre os componentes do compilador.

Esse princípio reduz o custo de manutenção e permite que o compilador acompanhe a evolução da linguagem ao longo do tempo.

---

## 16.1 Objetivo

A arquitetura deve permitir a incorporação de novas funcionalidades preservando:

* modularidade;
* baixo acoplamento;
* estabilidade das interfaces;
* compatibilidade com implementações existentes;
* previsibilidade do pipeline.

Toda evolução deve ocorrer de forma incremental, minimizando o impacto sobre os componentes já consolidados.

---

## 16.2 Inclusão de Novas Fases

O pipeline poderá receber novas fases sempre que uma transformação conceitualmente distinta justificar sua existência.

Novas fases deverão:

* possuir responsabilidade claramente definida;
* receber um artefato formal como entrada;
* produzir um artefato formal como saída;
* manter independência em relação às demais fases.

Sempre que possível, novas funcionalidades devem ser implementadas como uma fase adicional, em vez de ampliar excessivamente a responsabilidade de fases existentes.

---

## 16.3 Novos Passes de Otimização

O módulo de otimização deverá permitir a inclusão de novos passes de forma independente.

Cada passe deve:

* executar uma transformação específica;
* preservar a semântica do programa;
* operar sobre a IR;
* poder ser habilitado ou desabilitado individualmente.

Essa organização facilita experimentação e evolução contínua das estratégias de otimização.

---

## 16.4 Novos Backends

A arquitetura permite a incorporação de novos Backends sem necessidade de alterações no Frontend ou no Middle-end.

Cada Backend deverá implementar exclusivamente as transformações necessárias para gerar código para uma arquitetura específica.

Exemplos incluem:

* x86-64;
* AArch64;
* RISC-V;
* WebAssembly;
* LLVM;
* outras arquiteturas suportadas futuramente.

Todos compartilham exatamente a mesma IR produzida pelo compilador.

---

## 16.5 Evolução da Linguagem

Novos recursos da linguagem poderão exigir alterações no compilador.

Sempre que possível, essas alterações deverão permanecer restritas às fases diretamente relacionadas ao novo recurso.

Por exemplo:

* novos elementos sintáticos afetam Lexer e Parser;
* novos tipos afetam a verificação de tipos;
* novas otimizações afetam apenas o Middle-end;
* novos alvos de compilação afetam apenas o Backend.

Essa separação reduz o impacto das evoluções da linguagem sobre o restante do compilador.

---

## 16.6 Compatibilidade Arquitetural

A evolução da implementação não deve alterar os contratos públicos estabelecidos entre os módulos.

Quando modificações incompatíveis forem inevitáveis, elas deverão ocorrer de forma coordenada e preservar, sempre que possível, a organização geral do pipeline.

Esse princípio favorece estabilidade, manutenção e reutilização dos componentes do compilador.

---

# 17. Bootstrap

O desenvolvimento do compilador oficial da Capi ocorrerá de forma incremental por meio de um processo de **bootstrap**.

Inicialmente, o compilador será implementado utilizando uma linguagem hospedeira (*host language*). À medida que a própria linguagem Capi se tornar capaz de implementar seus componentes fundamentais, partes do compilador passarão a ser reescritas em Capi.

Essa estratégia permite validar continuamente a linguagem enquanto reduz gradualmente sua dependência de tecnologias externas.

---

## 17.1 Objetivo

O bootstrap possui os seguintes objetivos:

* permitir o desenvolvimento inicial do compilador;
* validar continuamente a linguagem;
* utilizar a própria Capi para implementar seus componentes;
* reduzir dependências externas ao longo do tempo;
* estabelecer um compilador autossustentável.

---

## 17.2 Fase Inicial

Na primeira etapa do projeto, o compilador será implementado integralmente em uma linguagem hospedeira.

Essa implementação deverá produzir compiladores compatíveis com todas as especificações da linguagem, servindo como base para as etapas seguintes do bootstrap.

A escolha da linguagem hospedeira pertence ao projeto de implementação e não faz parte desta especificação.

---

## 17.3 Migração Gradual

À medida que a linguagem evoluir, componentes do compilador poderão ser reimplementados em Capi.

Essa migração poderá ocorrer módulo a módulo.

Por exemplo:

* Biblioteca Padrão;
* Lexer;
* Parser;
* HIR;
* Resolver;
* Type Checker;
* Optimizer.

Durante esse período, componentes escritos em linguagens diferentes poderão coexistir temporariamente.

---

## 17.4 Compilador Autohospedado

O objetivo final do bootstrap é que o compilador oficial seja capaz de compilar a si próprio.

Nesse estágio:

* o compilador é escrito predominantemente em Capi;
* novas versões são produzidas utilizando a versão anterior do próprio compilador;
* a linguagem torna-se autossustentável.

Essa condição caracteriza um compilador **autohospedado** (*self-hosting compiler*).

---

## 17.5 Validação do Bootstrap

Cada etapa do bootstrap deverá preservar integralmente os contratos definidos pela especificação da linguagem.

A migração para componentes escritos em Capi não poderá alterar:

* o comportamento observável da linguagem;
* a ABI oficial;
* a IR oficial;
* o Runtime;
* os diagnósticos esperados.

As diferentes gerações do compilador devem produzir artefatos semanticamente equivalentes.

---

## 17.6 Evolução Contínua

Mesmo após atingir o estado de autohospedagem, o bootstrap permanece um processo contínuo.

Novos componentes poderão ser reescritos, refatorados ou substituídos para aproveitar recursos introduzidos em versões futuras da linguagem.

Essa evolução não altera a arquitetura fundamental do compilador.

---

# 18. Considerações Finais

A estrutura apresentada neste documento estabelece a organização de referência para a implementação do compilador oficial da linguagem Capi.

Sua arquitetura foi concebida para equilibrar simplicidade, modularidade e capacidade de evolução, permitindo que cada fase do processo de compilação seja desenvolvida, testada e mantida de forma independente.

---

## 18.1 Síntese da Arquitetura

O compilador é organizado como um pipeline de transformações sucessivas, composto por:

* Frontend;
* Middle-end;
* Backend;
* infraestrutura compartilhada.

Cada etapa recebe um artefato formal, realiza uma transformação específica e produz um novo artefato para a fase seguinte.

Essa organização preserva o isolamento entre componentes e favorece a evolução incremental da implementação.

---

## 18.2 Princípios Preservados

Toda implementação compatível com esta arquitetura deverá preservar os seguintes princípios:

* responsabilidade única por módulo;
* baixo acoplamento;
* interfaces estáveis;
* pipeline determinístico;
* testabilidade;
* extensibilidade;
* separação entre especificação e implementação.

Esses princípios orientam todas as decisões arquiteturais do compilador oficial.

---

## 18.3 Relação com os Documentos Seguintes

Este documento estabelece apenas a organização geral da implementação.

Os documentos subsequentes detalham individualmente cada componente do compilador:

* Documento 14 — Lexer e Tokens;
* Documento 15 — Parser e AST;
* Documento 16 — Representação Semântica (HIR);
* Documento 17 — Resolução de Nomes e Escopos;
* Documento 18 — Inferência e Verificação de Tipos;
* Documento 19 — Verificação Semântica;
* Documento 20 — Lowering para IR;
* Documento 21 — Passes de Otimização;
* Documento 22 — Backend e Geração de Código;
* Documento 23 — Diagnósticos e Recuperação de Erros;
* Documento 24 — Testes do Compilador;
* Documento 25 — Bootstrap e Subconjunto Inicial.

Cada um desses documentos aprofunda uma etapa específica do pipeline definido nesta arquitetura.

---

## 18.4 Arquitetura de Referência

A arquitetura descrita neste documento representa a implementação de referência do compilador oficial da Capi.

Outras implementações poderão adotar organizações internas diferentes, desde que preservem integralmente:

* os contratos públicos da linguagem;
* o comportamento definido pela Semântica Operacional;
* o Sistema de Tipos;
* o Modelo de Memória;
* a ABI oficial;
* a Intermediate Representation especificada;
* o Runtime oficial;
* os princípios fundamentais estabelecidos pela especificação da linguagem.

Dessa forma, este documento conclui a definição estrutural do compilador e estabelece a base sobre a qual os documentos seguintes detalham cada fase individual do processo de compilação.
