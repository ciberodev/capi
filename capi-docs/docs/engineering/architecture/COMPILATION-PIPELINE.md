# Compilation Pipeline

**Projeto:** Linguagem Capi  
**Documento:** COMPILATION-PIPELINE  
**Status:** Aprovado  
**Stage:** Stage 0 — Fundação do projeto  
**Natureza:** Documento de consolidação  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o pipeline de compilação da implementação oficial da Linguagem Capi em nível de engenharia.

Seu objetivo é consolidar:

- a ordem das fases;
- as entradas e saídas de cada fase;
- as fronteiras entre frontend, middle-end e backend;
- o papel do driver e da sessão;
- a circulação de diagnósticos;
- os pontos de validação;
- a evolução incremental do pipeline por stage.

Este documento não define algoritmos internos de lexer, parser, análise semântica, MIR, otimização ou codegen.

---

## 2. Escopo

Este documento cobre o fluxo conceitual e operacional do compilador oficial.

Inclui:

- carregamento de fontes;
- criação da sessão;
- source map e spans;
- lexer;
- parser;
- AST;
- lowering para HIR;
- resolução de nomes;
- checagem de tipos;
- verificações semânticas;
- lowering para MIR;
- validação e passes de MIR;
- interface de backend;
- geração de artefatos;
- diagnósticos;
- comandos mínimos do Stage 0.

Não inclui:

- gramática completa da linguagem;
- estrutura detalhada de AST, HIR ou MIR;
- layout de memória;
- ABI detalhada;
- implementação de Cranelift ou LLVM;
- desenho completo da CLI;
- formato completo dos dumps.

Esses temas pertencem aos documentos específicos de cada área.

---

## 3. Princípios

O pipeline deve seguir os princípios definidos em:

- `ENGINEERING-PRINCIPLES.md`;
- `COMPILER-ARCHITECTURE.md`;
- `COMPONENT-RESPONSIBILITIES.md`;
- `DEPENDENCY-RULES.md`;
- `TEST-STRATEGY.md`.

Regras centrais:

- cada fase possui entrada e saída explícitas;
- fases não devem acessar detalhes privados de fases não adjacentes;
- frontend não depende de backend;
- MIR permanece independente de Cranelift e LLVM;
- diagnósticos circulam por infraestrutura estruturada;
- a sessão coordena contexto, mas não substitui dependências explícitas;
- artefatos intermediários devem ser determinísticos quando emitidos;
- falhas esperadas de usuário geram diagnósticos, não panic;
- erros internos devem ser distinguíveis de erros do usuário.

---

## 4. Visão Geral

Fluxo conceitual completo:

```text
CLI / toolchain request
    ↓
Driver
    ↓
Compilation session
    ↓
Source loading
    ↓
SourceMap + SourceFile + spans
    ↓
Lexer
    ↓
Tokens
    ↓
Parser
    ↓
AST
    ↓
AST lowering
    ↓
HIR
    ↓
Name resolution
    ↓
Resolved HIR + symbols
    ↓
Type checking
    ↓
Typed HIR
    ↓
Semantic checks
    ↓
Validated semantic model
    ↓
MIR lowering
    ↓
MIR
    ↓
MIR validation and passes
    ↓
Validated MIR
    ↓
Backend interface
    ↓
Backend implementation
    ↓
Object files / executable artifacts
```

Infraestrutura transversal:

```text
Diagnostics
SourceMap
Session
Configuration
Interners
Typed IDs
Error handling
Tracing
Dumps
Test harnesses
```

---

## 5. Papel do Driver

O driver é o coordenador do pipeline.

Ele deve:

- receber solicitação normalizada da CLI ou toolchain;
- criar ou receber uma sessão de compilação;
- selecionar o modo de execução;
- chamar fases na ordem correta;
- impedir execução de fases sem pré-condições;
- agregar diagnósticos;
- retornar resultado estruturado;
- preservar rastreabilidade da execução.

Ele não deve:

- implementar lexer, parser, type checker ou backend;
- acessar estruturas privadas de fases;
- converter erro de usuário em panic;
- imprimir diretamente diagnósticos;
- conter lógica específica de Cranelift ou LLVM;
- redefinir semântica da linguagem.

---

## 6. Papel da Sessão de Compilação

A sessão representa o contexto operacional de uma invocação do compilador.

Ela deve conter ou referenciar:

- opções de compilação;
- modo de execução;
- target selecionado;
- source map;
- coletor de diagnósticos;
- configuração de emissão;
- informações de versão;
- serviços compartilhados autorizados;
- estado de rastreabilidade.

A sessão não deve virar depósito global arbitrário.

Dados próprios de uma fase devem permanecer nas estruturas dessa fase.

---

## 7. Modos de Execução

O pipeline deve permitir modos parciais conforme os stages avançarem.

Modos planejados:

| Modo | Para em | Objetivo |
| --- | --- | --- |
| `help` | CLI | Exibir ajuda. |
| `version` | CLI | Exibir versão. |
| `check-source` | source loading | Validar existência e leitura de fontes. |
| `lex` | tokens | Executar lexer e opcionalmente emitir tokens. |
| `parse` | AST | Executar parser e opcionalmente emitir AST. |
| `check` | HIR validada | Executar análises sem gerar código. |
| `emit-mir` | MIR | Emitir MIR para inspeção. |
| `build` | artefato | Gerar objeto ou executável. |
| `run` | execução | Compilar e executar, quando suportado. |

No Stage 0, apenas `help`, `version` e tratamento básico de entrada inválida são obrigatórios.

---

## 8. Fases do Pipeline

### 8.1 CLI Request

Entrada:

- argumentos do processo;
- ambiente necessário;
- configuração futura de projeto, quando existir.

Saída:

- solicitação normalizada ao driver;
- ou diagnóstico de uso inválido;
- ou saída direta de `--help` e `--version`.

Responsável inicial:

- `capi-cli`.

---

### 8.2 Driver Initialization

Entrada:

- solicitação normalizada;
- configuração inicial;
- informações de versão.

Saída:

- sessão de compilação;
- seleção de modo de execução;
- resultado antecipado quando o modo não exige pipeline completo.

Responsáveis iniciais:

- `capi-driver`;
- `capi-session`.

---

### 8.3 Source Loading

Entrada:

- caminhos de entrada;
- opções de leitura;
- sessão.

Saída:

- `SourceFile`;
- `SourceMap`;
- spans base;
- diagnósticos de arquivo, encoding ou leitura.

Responsável planejado:

- `capi-source`.

Falha de arquivo inexistente é erro esperado de usuário.

---

### 8.4 Lexer

Entrada:

- `SourceFile`;
- spans;
- configuração léxica.

Saída:

- tokens;
- diagnósticos léxicos;
- spans associados a tokens.

O lexer não deve:

- resolver nomes;
- interpretar tipos;
- acessar backend;
- decidir semântica além da classificação léxica.

---

### 8.5 Parser

Entrada:

- tokens;
- source map;
- coletor de diagnósticos.

Saída:

- AST;
- diagnósticos sintáticos;
- estrutura recuperável quando a recuperação for possível.

O parser não deve:

- inferir tipos;
- resolver símbolos globais;
- aplicar regras de ownership;
- depender de backend.

---

### 8.6 AST

AST é a representação estrutural próxima da sintaxe do usuário.

Responsabilidades:

- preservar forma sintática relevante;
- manter spans;
- permitir diagnósticos sintáticos e lowering;
- evitar carregar decisões semânticas que pertencem à HIR.

AST não é representação semântica final.

---

### 8.7 AST Lowering

Entrada:

- AST;
- sessão;
- infraestrutura de símbolos quando aplicável.

Saída:

- HIR inicial;
- diagnósticos de lowering;
- mapeamentos de rastreabilidade entre AST e HIR.

O lowering deve reduzir detalhes puramente sintáticos e preparar a representação para análise semântica.

---

### 8.8 HIR

HIR é a representação semântica de alto nível.

Responsabilidades:

- representar entidades semânticas;
- carregar IDs internos estáveis dentro do escopo da compilação;
- preservar ligação com spans;
- servir de base para resolução de nomes, tipos e verificações semânticas.

HIR não deve depender de MIR ou backend.

---

### 8.9 Name Resolution

Entrada:

- HIR inicial;
- escopos;
- módulos;
- símbolos disponíveis.

Saída:

- HIR resolvida;
- tabela de símbolos;
- diagnósticos de nomes;
- vínculos entre usos e definições.

Essa fase não deve decidir layout físico, codegen ou ABI.

---

### 8.10 Type Checking

Entrada:

- HIR resolvida;
- símbolos;
- modelo de tipos.

Saída:

- HIR tipada;
- informações de tipos;
- coerções;
- diagnósticos de tipo.

Essa fase valida o sistema de tipos conforme a especificação.

---

### 8.11 Semantic Checks

Entrada:

- HIR tipada;
- informações de tipos;
- modelo de escopos e símbolos.

Saída:

- modelo semântico validado;
- diagnósticos semânticos;
- informações para lowering posterior.

Inclui, conforme os stages avançarem:

- ownership;
- borrowing;
- regiões;
- `Domain`;
- effects;
- mutabilidade;
- regras de concorrência;
- validações de objeto.

---

### 8.12 MIR Lowering

Entrada:

- HIR semanticamente validada;
- informações de tipos;
- informações de controle de fluxo.

Saída:

- MIR;
- mapeamento de origem para diagnósticos e dumps;
- erro interno estruturado se invariantes esperadas forem violadas.

MIR lowering transforma semântica validada em representação adequada para análise de fluxo e backend.

---

### 8.13 MIR Validation

Entrada:

- MIR.

Saída:

- MIR validada;
- erro interno estruturado quando invariantes internas forem quebradas.

Essa fase protege o backend contra entrada inválida produzida pelo próprio compilador.

Falha de validação de MIR normalmente indica bug do compilador, não erro do usuário.

---

### 8.14 MIR Passes

Entrada:

- MIR validada.

Saída:

- MIR transformada e revalidada;
- dumps opcionais;
- métricas opcionais.

Passes de MIR devem preservar semântica e permanecer independentes de backend.

---

### 8.15 Backend Interface

Entrada:

- MIR validada;
- target;
- configuração de codegen;
- metadados necessários.

Saída:

- solicitação de backend;
- artefatos intermediários;
- diagnósticos de codegen.

A interface de backend é a fronteira entre middle-end e backends concretos.

---

### 8.16 Backend Implementation

Entrada:

- representação consumível pelo backend;
- target;
- configuração de emissão.

Saída:

- objeto;
- executável;
- biblioteca;
- artefatos intermediários;
- diagnósticos ou falhas estruturadas.

Backends concretos não devem alterar semântica da linguagem.

---

### 8.17 Linking and Artifact Emission

Entrada:

- objetos;
- runtime;
- bibliotecas;
- configuração de link.

Saída:

- executável;
- biblioteca;
- metadados de artefato;
- diagnósticos de link.

Essa fase será detalhada em documentos de backend, ABI e release.

---

## 9. Diagnósticos no Pipeline

Toda fase deve reportar problemas por meio da infraestrutura de diagnósticos.

Regras:

- diagnósticos devem preservar severidade;
- diagnósticos devem conter span quando aplicável;
- diagnósticos devem ser agregados pela sessão ou driver;
- CLI é responsável pela apresentação final;
- fases não devem imprimir diretamente em `stdout` ou `stderr`;
- erro esperado de usuário não deve causar panic;
- erro interno deve ser identificado como erro interno.

Exemplos:

| Situação | Classificação |
| --- | --- |
| arquivo inexistente | erro do usuário |
| token inválido | erro léxico |
| sintaxe inválida | erro sintático |
| nome desconhecido | erro semântico |
| tipo incompatível | erro de tipo |
| MIR inválida após lowering | erro interno do compilador |
| falha do linker | erro de ferramenta ou ambiente |

---

## 10. Invariantes entre Fases

O pipeline deve preservar as invariantes abaixo.

### 10.1 Fonte e spans

- todo token deve ser rastreável a um span;
- todo diagnóstico ligado a código deve apontar para localização reproduzível;
- paths absolutos não devem vazar para testes ou snapshots sem normalização.

### 10.2 Representações

- AST não deve carregar decisões de backend;
- HIR não deve depender de MIR;
- MIR não deve depender de Cranelift ou LLVM;
- artefatos de backend não devem retroalimentar frontend.

### 10.3 Erros

- fases podem produzir diagnósticos e interromper fases posteriores;
- fases posteriores não devem executar se pré-condições falharam;
- erro interno deve ser separado de erro de usuário.

### 10.4 Determinismo

- dumps devem ser estáveis;
- ordenações observáveis devem ser determinísticas;
- IDs internos devem ser adequados a reprodução e depuração;
- testes não devem depender de ordem acidental de mapas ou filesystem.

---

## 11. Execução Parcial

O pipeline deve permitir execução parcial para testes, depuração e desenvolvimento incremental.

Exemplos futuros:

```bash
capic --dump-tokens arquivo.capi
capic --dump-ast arquivo.capi
capic --dump-hir arquivo.capi
capic --dump-mir arquivo.capi
capic check arquivo.capi
capic build arquivo.capi
```

Os nomes finais de comandos e flags pertencem aos documentos da toolchain.

Este documento apenas estabelece que o pipeline deve ser estruturado para permitir esses cortes.

---

## 12. Dumps e Observabilidade

Dumps são saídas de inspeção usadas por desenvolvedores e testes.

Podem existir para:

- tokens;
- AST;
- HIR;
- símbolos;
- tipos;
- MIR;
- passes;
- backend intermediário.

Regras:

- dumps não são especificação pública da linguagem, salvo quando documento próprio declarar;
- dumps usados em testes devem ser determinísticos;
- dumps devem normalizar paths e IDs instáveis;
- dumps não devem exigir backend quando a fase inspecionada pertence ao frontend.

---

## 13. Integração com Testes

Cada fase deve possuir estratégia de teste proporcional ao risco.

Relação inicial:

| Fase | Testes esperados |
| --- | --- |
| CLI | testes de `--help`, `--version`, argumentos inválidos |
| Source loading | arquivo inexistente, encoding, spans |
| Lexer | tokens válidos e inválidos |
| Parser | sintaxe válida, sintaxe inválida, recuperação |
| AST lowering | estrutura HIR esperada |
| Name resolution | nomes válidos e inválidos |
| Type checking | tipos aceitos e rejeitados |
| Semantic checks | ownership, regions, domains, effects |
| MIR | lowering, validação, dumps |
| Backend | codegen e run-pass |

No Stage 0, testes obrigatórios se limitam ao ciclo mínimo definido em `TEST-STRATEGY.md`.

---

## 14. Pipeline Mínimo do Stage 0

No Stage 0, o compilador ainda não precisa processar código Capi.

Pipeline mínimo:

```text
process args
    ↓
identify mode
    ↓
print help/version
    or
initialize session
    ↓
report basic input errors
```

Entregas mínimas:

- `capic --help`;
- `capic --version`;
- leitura inicial de argumentos;
- inicialização de sessão;
- infraestrutura básica de diagnósticos;
- erro para arquivo inexistente;
- testes mínimos da CLI e do driver.

O pipeline deve nascer pequeno, mas já com a separação correta entre CLI, driver, sessão, fonte e diagnósticos.

---

## 15. Evolução por Stages

Evolução esperada:

| Stage | Evolução do pipeline |
| --- | --- |
| Stage 0 | CLI mínima, driver, sessão, diagnósticos e fonte básica. |
| Stage 1 | Source model, spans, diagnósticos iniciais e lexer. |
| Stage 2 | Parser, AST e recuperação sintática. |
| Stage 3 | Lowering para HIR e resolução de nomes. |
| Stage 4 | Inferência e verificação de tipos. |
| Stages 5 a 7 | Ownership, regiões, objetos, domains, effects e concorrência. |
| Stage 8 | MIR, validação e dumps. |
| Stage 9 | Runtime mínimo integrado ao subconjunto. |
| Stage 10 | Biblioteca padrão mínima. |
| Stage 11 | Backend Cranelift e primeiros artefatos nativos. |
| Stage 15 | Suíte de conformidade e validação ampla. |
| Stages 16 a 18 | Bootstrap, LLVM e auto-hospedagem. |

---

## 16. Relação com Crates

Crates iniciais do Stage 0 e relação com o pipeline:

| Crate | Papel no pipeline |
| --- | --- |
| `capi-cli` | Entrada de processo e apresentação final. |
| `capi-driver` | Coordenação de fases e resultado estruturado. |
| `capi-session` | Contexto operacional da compilação. |
| `capi-diagnostics` | Diagnósticos estruturados e renderização inicial. |
| `capi-source` | Arquivos-fonte, source map, spans e localização. |
| `capi-common` | Tipos compartilhados mínimos e utilidades justificadas. |

Crates futuros devem ser adicionados apenas quando uma nova fase exigir fronteira própria.

---

## 17. Critérios de Aceitação do Documento

Este documento é considerado preenchido quando:

- define a ordem conceitual do pipeline;
- descreve entrada e saída das fases;
- preserva fronteiras entre frontend, middle-end e backend;
- define papel do driver e da sessão;
- estabelece regras de diagnósticos;
- descreve o pipeline mínimo do Stage 0;
- explica evolução por stages;
- não contradiz `COMPILER-ARCHITECTURE.md`, `COMPONENT-RESPONSIBILITIES.md` ou a especificação.

---

## 18. Síntese

O pipeline de compilação da Capi deve evoluir de forma incremental, mas sempre com fronteiras claras.

No Stage 0, ele existe apenas como fundação operacional da CLI, driver, sessão, fontes e diagnósticos. Nos stages seguintes, cada fase deve ser adicionada como transformação explícita, testável e rastreável até a especificação.
