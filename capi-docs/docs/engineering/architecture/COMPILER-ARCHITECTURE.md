# Compiler Architecture

**Projeto:** Linguagem Capi
**Documento:** COMPILER-ARCHITECTURE
**Status:** Proposto
**Stage:** Stage 0 — Fundação do projeto
**Natureza:** Documento de engenharia bloqueante
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define a arquitetura de engenharia do compilador oficial da Linguagem Capi.

Seu objetivo é transformar a arquitetura conceitual definida pela especificação em uma organização prática para a implementação inicial em Rust, preservando as fronteiras entre frontend, middle-end, backend, runtime e toolchain.

Este documento orienta a criação dos componentes fundamentais do compilador durante o Stage 0.

Ele não define algoritmos específicos de lexer, parser, inferência de tipos, borrow checking, lowering, otimização ou geração de código.

---

## 2. Escopo

Este documento cobre:

* camadas do compilador;
* responsabilidades de alto nível;
* artefatos produzidos por cada fase;
* infraestrutura compartilhada;
* fronteiras entre componentes;
* integração com runtime, biblioteca padrão e toolchain;
* critérios arquiteturais para o Stage 0;
* regras para evolução incremental.

Este documento não cobre:

* layout final dos crates;
* APIs detalhadas;
* sintaxe da linguagem;
* estrutura completa da AST, HIR ou MIR;
* ABI detalhada;
* design interno de Cranelift ou LLVM;
* comandos públicos da toolchain.

Esses temas pertencem aos documentos de engenharia específicos.

---

## 3. Princípios arquiteturais

A arquitetura do compilador deve seguir `ENGINEERING-PRINCIPLES.md`.

Em particular:

* a especificação é fonte de verdade;
* a implementação não redefine a linguagem;
* o frontend não depende de backends;
* a MIR permanece independente de Cranelift e LLVM;
* diagnósticos são estruturados;
* cada fase possui entrada e saída explícitas;
* artefatos intermediários devem ser determinísticos quando emitidos;
* a arquitetura deve permitir evolução incremental e bootstrap.

Cada componente deve possuir responsabilidade definida e interface explícita.

Dependências circulares entre componentes do compilador são proibidas.

---

## 4. Visão geral

O compilador oficial é organizado como um pipeline em camadas.

Fluxo conceitual:

```text
Source files
    ↓
Source map, spans and session
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
Type checking
    ↓
Semantic checks
    ↓
MIR lowering
    ↓
MIR
    ↓
MIR validation and passes
    ↓
Backend interface
    ↓
Backend implementation
    ↓
Object files or executable artifacts
```

Infraestrutura compartilhada acompanha todo o pipeline:

```text
Session
Diagnostics
Source map
Interners
IDs
Configuration
Error handling
Tracing and dumps
```

---

## 5. Camadas principais

O compilador é dividido em quatro camadas principais:

```text
Frontend
Middle-end
Backend
Shared infrastructure
```

### 5.1 Frontend

O frontend é responsável por transformar código-fonte Capi em uma representação semanticamente validada.

Inclui:

* infraestrutura de fonte, spans e localização;
* lexer;
* parser;
* AST;
* lowering de AST para HIR;
* HIR;
* resolução de nomes;
* inferência e verificação de tipos;
* verificação semântica;
* ownership, borrowing, regiões e Domains.

O frontend não deve depender de Cranelift, LLVM, linker, formato de objeto ou detalhes de plataforma.

### 5.2 Middle-end

O middle-end é responsável por transformar a representação semântica validada em MIR e aplicar validações ou passes independentes de backend.

Inclui:

* lowering para MIR;
* validação estrutural da MIR;
* dumps de MIR;
* passes independentes de plataforma;
* preparação de informações consumidas pela interface de backend.

O middle-end não deve produzir código específico de Cranelift, LLVM ou arquitetura física.

### 5.3 Backend

O backend é responsável por transformar MIR validada em artefatos executáveis ou intermediários de código nativo.

Inclui:

* interface abstrata de backend;
* backend Cranelift;
* backend LLVM, quando introduzido;
* geração de objetos;
* integração com linker;
* emissão de artefatos;
* tratamento de falhas de codegen.

Cada backend deve consumir a mesma representação de entrada definida pela interface de backend.

### 5.4 Infraestrutura compartilhada

Infraestrutura compartilhada contém serviços usados por múltiplas fases.

Inclui:

* sessão de compilação;
* gerenciamento de arquivos-fonte;
* source map;
* diagnósticos;
* IDs internos;
* interning;
* configuração;
* tratamento de erros internos;
* suporte a dumps;
* utilitários comuns.

Infraestrutura compartilhada não deve conter regras semânticas específicas de uma fase.

---

## 6. Artefatos do pipeline

Cada fase deve produzir artefatos explícitos.

Artefatos principais:

| Fase | Entrada | Saída |
| --- | --- | --- |
| Source loading | caminhos, configuração | `SourceFile`, `SourceMap` |
| Lexer | fonte e spans | tokens ou diagnósticos léxicos |
| Parser | tokens | AST ou diagnósticos sintáticos |
| AST lowering | AST | HIR |
| Name resolution | HIR | HIR resolvida, símbolos, diagnósticos |
| Type checking | HIR resolvida | tipos, coerções, diagnósticos |
| Semantic checks | HIR tipada | modelo semântico validado |
| MIR lowering | HIR validada | MIR |
| MIR validation | MIR | MIR validada ou erro interno estruturado |
| Backend | MIR validada | objetos, executáveis ou diagnósticos de codegen |

Uma fase não deve modificar diretamente o artefato principal produzido por fase anterior.

Quando uma transformação alterar o nível de abstração, uma nova representação deve ser produzida.

---

## 7. Sessão de compilação

A sessão de compilação é o contexto operacional de uma invocação do compilador.

Ela deve conter ou referenciar:

* opções de compilação;
* target selecionado;
* source map;
* coletor de diagnósticos;
* configuração de emissão;
* informações de versão;
* estado necessário para rastreabilidade da execução.

A sessão não deve se tornar um depósito global indiscriminado.

Dados pertencentes a fases específicas devem permanecer em estruturas específicas dessas fases.

---

## 8. Diagnósticos

Diagnósticos são saída estruturada do compilador.

Toda fase deve reportar erros por meio da infraestrutura de diagnósticos, não por impressão direta em `stdout` ou `stderr`.

Diagnósticos devem conter, quando aplicável:

* código ou categoria;
* severidade;
* mensagem;
* span primário;
* spans secundários;
* notas;
* sugestões;
* origem da fase.

Falhas internas devem ser distinguíveis de erros do programa do usuário.

Entradas inválidas devem produzir diagnósticos, não pânico não controlado.

---

## 9. Representações internas

Representações internas devem ter escopo claro.

### 9.1 Tokens

Tokens representam a análise léxica da fonte.

Devem preservar spans e categorias necessárias ao parser.

### 9.2 AST

AST representa a estrutura sintática do programa.

Ela deve preservar detalhes necessários para diagnósticos e recuperação sintática.

AST não deve carregar decisões semânticas que pertencem à HIR ou fases posteriores.

### 9.3 HIR

HIR representa a estrutura semântica inicial.

Ela deve reduzir dependência da forma textual original e fornecer base para resolução de nomes, tipagem e verificações semânticas.

HIR não deve depender diretamente da estrutura física da AST.

### 9.4 MIR

MIR representa fluxo de controle e operações em formato adequado para validação, passes e geração de código.

MIR deve permanecer independente de Cranelift e LLVM.

### 9.5 IDs internos

Entidades internas devem usar IDs tipados quando identidade estável for necessária.

IDs não devem depender de ponteiros, endereços de memória ou ordem acidental de alocação.

---

## 10. Fronteiras obrigatórias

As seguintes fronteiras devem ser preservadas:

```text
Lexer não conhece AST.
Parser não executa resolução semântica.
AST não depende de HIR.
HIR não depende de backend.
Type checker não gera código.
Borrow checker não depende de Cranelift ou LLVM.
MIR não depende de Cranelift ou LLVM.
Backend não acessa estruturas privadas do frontend.
Runtime não redefine semântica da linguagem.
Toolchain não implementa regras internas do compilador.
```

Qualquer exceção deve ser justificada e registrada.

Exceções arquiteturais relevantes exigem ADR.

---

## 11. Integração com runtime

O compilador pode depender dos contratos públicos documentados do runtime.

Essa dependência ocorre principalmente em:

* inicialização;
* Domains;
* ObjectId;
* layout aplicável;
* intrínsecos;
* chamadas geradas;
* integração com código nativo.

O compilador não deve depender de detalhes privados da implementação do runtime.

Durante stages iniciais, contratos entre compilador e runtime podem ser estáveis apenas para o subconjunto implementado, conforme o Documento 28.

---

## 12. Integração com biblioteca padrão

A biblioteca padrão é compilada ou integrada pela mesma arquitetura geral usada por programas Capi, respeitados os limites do bootstrap.

O compilador deve tratar a biblioteca padrão como componente do ecossistema, não como exceção semântica arbitrária.

Intrínsecos devem ser explícitos, documentados e testados.

APIs da biblioteca padrão não devem ser hardcoded no frontend sem contrato aprovado.

---

## 13. Integração com toolchain

`capic` é o executável especializado do compilador.

`capi` é a ferramenta de alto nível da toolchain.

A toolchain pode invocar o compilador, configurar projetos e organizar artefatos, mas não deve duplicar regras internas do pipeline.

Interface mínima esperada no Stage 0:

```bash
capic --help
capic --version
```

Interfaces de emissão progressiva devem ser adicionadas conforme os stages:

```bash
capic --emit tokens
capic --emit ast
capic --emit hir
capic --emit mir
capic check
capic build
```

---

## 14. Backends

O backend inicial oficial será Cranelift.

LLVM será introduzido posteriormente como backend adicional de otimização e ampliação de suporte.

Regras:

* Cranelift não substitui a MIR;
* LLVM IR não substitui a MIR;
* backends não devem alterar semântica observável;
* divergências entre backends devem ser diagnosticadas, testadas ou documentadas;
* backend específico não deve introduzir dependência nas fases anteriores.

A interface de backend deve ser definida antes de acoplamentos diretos com tecnologias específicas.

---

## 15. Dumps e observabilidade

O compilador deve permitir inspeção controlada de artefatos intermediários.

Dumps devem ser:

* determinísticos;
* adequados para testes;
* estáveis dentro do escopo declarado;
* livres de endereços de memória ou ordem acidental;
* vinculados à fase correspondente.

Emissões esperadas ao longo dos stages:

```text
tokens
ast
hir
mir
backend-ir, quando aplicável
```

Dumps são ferramentas de engenharia e teste.

Eles não constituem API pública estável salvo decisão explícita.

---

## 16. Erros internos

Erro interno do compilador representa violação de invariante da implementação.

Ele deve ser separado de erro de programa do usuário.

Exemplos:

* MIR estruturalmente inválida após validação;
* ID interno ausente;
* fase executada fora de ordem;
* backend recebendo artefato incompatível;
* inconsistência entre tabelas internas.

Erros internos devem produzir informação suficiente para reprodução.

Quando um erro interno revelar lacuna de validação, a fase responsável deve ser fortalecida.

---

## 17. Evolução por stages

A arquitetura deve suportar a ordem definida pelo Documento 28.

Relação com stages iniciais:

```text
Stage 0
Driver, sessão, diagnósticos básicos, fontes e CLI mínima.

Stage 1
Source model, spans, diagnósticos e lexer.

Stage 2
Parser e AST.

Stage 3
HIR e resolução de nomes.

Stage 4
Sistema de tipos.

Stages 5 a 7
Objetos, ownership, regiões e Domains.

Stage 8
MIR.

Stages 9 a 11
Runtime, ABI aplicável, biblioteca padrão mínima e Cranelift.
```

Cada stage deve preservar capacidade de build e testes.

---

## 18. Estrutura inicial no Stage 0

No Stage 0, a implementação ainda não precisa processar código Capi.

Entretanto, deve criar a base arquitetural para o pipeline.

Componentes mínimos:

* executável `capic`;
* driver;
* sessão de compilação;
* diagnósticos básicos;
* infraestrutura inicial de fontes;
* tratamento básico de erros internos;
* testes de CLI;
* organização inicial coerente com `PROJECT-STRUCTURE.md`.

Resultado demonstrável:

```bash
capic --help
capic --version
cargo test
```

---

## 19. Critérios de aceite deste documento

Este documento estará pronto para orientar o Stage 0 quando:

* definir as camadas principais do compilador;
* estabelecer as fronteiras obrigatórias entre fases;
* descrever artefatos principais do pipeline;
* orientar a criação de driver, sessão, diagnósticos e fontes;
* manter Cranelift e LLVM isolados nos backends;
* não contradizer os Documentos 00 a 28;
* não substituir documentos especializados de componentes;
* permitir a criação do workspace inicial conforme `PROJECT-STRUCTURE.md`.

---

## 20. Síntese

A arquitetura do compilador oficial da Capi é um pipeline em camadas, com artefatos explícitos, diagnósticos estruturados e isolamento entre frontend, middle-end, backend, runtime e toolchain.

Seu objetivo central é permitir uma implementação incremental, testável e preparada para bootstrap sem transformar mecanismos temporários em garantias da linguagem.
