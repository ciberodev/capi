# Component Responsibilities

**Projeto:** Linguagem Capi
**Documento:** COMPONENT-RESPONSIBILITIES
**Status:** Proposto
**Stage:** Stage 0 — Fundação do projeto
**Natureza:** Documento de engenharia bloqueante
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define as responsabilidades dos componentes da implementação oficial da Linguagem Capi.

Seu objetivo é impedir sobreposição indevida de responsabilidades, dependências circulares e vazamento de detalhes internos entre fases do compilador, runtime, biblioteca padrão, toolchain e infraestrutura compartilhada.

Este documento complementa:

* `ENGINEERING-PRINCIPLES.md`;
* `PROJECT-STRUCTURE.md`;
* `COMPILER-ARCHITECTURE.md`;
* `WORKSPACE-ARCHITECTURE.md`;
* `DEPENDENCY-RULES.md`.

---

## 2. Escopo

Este documento cobre:

* responsabilidades dos componentes do Stage 0;
* responsabilidades de fases futuras do pipeline;
* entradas e saídas esperadas;
* não objetivos de cada componente;
* fronteiras entre compilador, runtime, std e toolchain;
* critérios para mover responsabilidades entre componentes.

Este documento não cobre:

* APIs detalhadas;
* algoritmos internos;
* layout final de módulos Rust;
* estruturas completas de AST, HIR ou MIR;
* formato detalhado de diagnósticos;
* comandos completos da toolchain.

---

## 3. Regras gerais de responsabilidade

Cada componente deve possuir uma responsabilidade principal.

Um componente deve:

* receber entradas explícitas;
* produzir saídas explícitas;
* ocultar seus detalhes internos;
* depender apenas de componentes necessários;
* expor apenas contratos compatíveis com seu papel;
* possuir testes proporcionais ao risco.

Um componente não deve:

* assumir responsabilidade de fase posterior;
* acessar estruturas privadas de outro componente;
* imprimir diagnósticos diretamente;
* criar dependência circular;
* depender de backend fora da camada de backend;
* redefinir regra da linguagem por conveniência de implementação.

---

## 4. Categorias de componentes

Os componentes são organizados em cinco categorias:

```text
CLI and driver
Shared infrastructure
Frontend
Middle-end
Backend
Runtime, std and toolchain
```

Responsabilidades transversais, como testes, build, observabilidade e bootstrap, possuem documentos próprios, mas interagem com esses componentes.

---

## 5. Componentes mínimos do Stage 0

O Stage 0 deve criar a base executável do compilador.

Componentes mínimos:

```text
capi-cli
capi-driver
capi-session
capi-diagnostics
capi-source
capi-common
```

Esses componentes não precisam compilar código Capi no Stage 0.

Eles devem permitir:

```bash
capic --help
capic --version
cargo test --workspace
```

---

## 6. `capi-cli`

### Responsabilidade

Fornecer o executável `capic`.

### Deve fazer

* ler argumentos de linha de comando;
* implementar `--help`;
* implementar `--version`;
* converter argumentos em uma solicitação ao driver;
* apresentar diagnósticos recebidos;
* retornar código de saída apropriado.

### Não deve fazer

* implementar fases do compilador;
* manipular AST, HIR ou MIR;
* acessar diretamente backends;
* conter regras semânticas da linguagem;
* substituir a futura ferramenta `capi`.

### Entradas

* argumentos do processo;
* ambiente necessário à execução;
* saída estruturada do driver.

### Saídas

* texto de ajuda;
* versão;
* diagnósticos formatados;
* código de saída.

---

## 7. `capi-driver`

### Responsabilidade

Coordenar a execução do pipeline disponível em uma sessão de compilação.

### Deve fazer

* criar ou receber uma sessão de compilação;
* selecionar a ação solicitada;
* executar fases na ordem correta;
* encaminhar diagnósticos;
* retornar resultado estruturado ao CLI ou toolchain;
* impedir execução de fase sem pré-condições atendidas.

### Não deve fazer

* implementar lexer, parser, type checker ou backend;
* armazenar estado global arbitrário;
* conhecer detalhes privados de componentes;
* redefinir diagnósticos de fases;
* conter lógica específica de Cranelift ou LLVM.

### Entradas

* opções de compilação;
* sessão;
* arquivos ou comandos solicitados.

### Saídas

* resultado de compilação;
* diagnósticos agregados;
* artefato solicitado, quando houver.

---

## 8. `capi-session`

### Responsabilidade

Representar o contexto operacional de uma invocação do compilador.

### Deve fazer

* armazenar opções de compilação;
* referenciar source map;
* referenciar coletor de diagnósticos;
* registrar target selecionado;
* carregar informações de versão;
* fornecer contexto para fases sem criar dependência direta entre elas.

### Não deve fazer

* executar fases do pipeline;
* conter lógica semântica;
* virar depósito genérico de dados;
* ocultar dependências reais entre componentes;
* substituir estruturas específicas de cada fase.

### Entradas

* configuração;
* argumentos normalizados;
* serviços compartilhados.

### Saídas

* contexto consultável;
* estado operacional da compilação;
* referências a infraestrutura compartilhada.

---

## 9. `capi-diagnostics`

### Responsabilidade

Fornecer infraestrutura estruturada para diagnósticos e erros internos.

### Deve fazer

* representar severidade;
* representar mensagens;
* associar spans quando disponíveis;
* coletar diagnósticos;
* formatar saída textual inicial;
* distinguir erros do usuário de erros internos;
* suportar testes de saída.

### Não deve fazer

* decidir regras semânticas;
* conhecer detalhes internos de todas as fases;
* imprimir diretamente sem mediação do chamador;
* depender de backend;
* depender do CLI.

### Entradas

* informações de erro;
* spans;
* mensagens;
* contexto de emissão.

### Saídas

* diagnósticos estruturados;
* renderização textual;
* indicação de erro bloqueador.

---

## 10. `capi-source`

### Responsabilidade

Representar e carregar fontes de entrada.

### Deve fazer

* representar `SourceId`;
* representar arquivo-fonte;
* representar spans básicos;
* carregar arquivo de forma controlada;
* reportar arquivo inexistente;
* preservar dados necessários para localização.

### Não deve fazer

* tokenizar;
* interpretar sintaxe;
* executar resolução de módulos;
* emitir diagnósticos por conta própria sem infraestrutura comum;
* depender de parser ou driver.

### Entradas

* caminho;
* conteúdo textual;
* configuração de leitura.

### Saídas

* fonte carregada;
* identificador de fonte;
* span;
* erro estruturado de carregamento.

---

## 11. `capi-common`

### Responsabilidade

Fornecer tipos pequenos e utilitários compartilhados que não pertencem a uma fase específica.

### Deve fazer

* hospedar tipos transversais mínimos;
* evitar duplicação entre crates fundamentais;
* permanecer estável e pequeno;
* permitir dependência por crates básicos.

### Não deve fazer

* acumular regras semânticas;
* virar módulo `misc`;
* conter dependências pesadas;
* depender de crates de pipeline;
* esconder responsabilidade que merece crate próprio.

Quando um item de `capi-common` ganhar semântica própria, deve ser movido para componente mais específico.

---

## 12. Lexer

### Responsabilidade

Converter fonte em tokens.

### Deve fazer

* reconhecer tokens do subconjunto implementado;
* preservar spans;
* reportar erros léxicos;
* aceitar entrada via infraestrutura de fonte;
* produzir dump determinístico quando solicitado.

### Não deve fazer

* construir AST;
* resolver ambiguidades sintáticas de alto nível;
* consultar tabela de símbolos;
* depender de parser;
* depender de type checker.

### Entrada

* `SourceFile`;
* configuração léxica aplicável.

### Saída

* sequência de tokens;
* diagnósticos léxicos.

---

## 13. Parser

### Responsabilidade

Converter tokens em AST.

### Deve fazer

* implementar gramática do subconjunto;
* preservar spans;
* produzir AST;
* recuperar de erros sintáticos quando possível;
* reportar diagnósticos sintáticos.

### Não deve fazer

* resolver nomes;
* inferir tipos;
* validar ownership;
* gerar HIR diretamente sem AST formal;
* depender de backend.

### Entrada

* tokens;
* sessão;
* infraestrutura de diagnósticos.

### Saída

* AST;
* diagnósticos sintáticos.

---

## 14. AST

### Responsabilidade

Representar a estrutura sintática do programa.

### Deve fazer

* preservar forma sintática relevante;
* preservar spans;
* permitir dumps determinísticos;
* servir de entrada para lowering para HIR.

### Não deve fazer

* armazenar tipo inferido;
* armazenar resolução final de símbolos;
* representar fluxo de controle de baixo nível;
* depender de HIR;
* depender de MIR.

---

## 15. HIR

### Responsabilidade

Representar a estrutura semântica inicial do programa.

### Deve fazer

* reduzir dependência da forma textual;
* representar entidades semanticamente relevantes;
* fornecer base para resolução de nomes;
* fornecer base para tipagem;
* preservar rastreabilidade com AST quando necessário.

### Não deve fazer

* depender fisicamente da AST como estrutura primária;
* conter detalhes de backend;
* substituir MIR;
* representar instruções de máquina.

---

## 16. Resolução de nomes

### Responsabilidade

Resolver identificadores, escopos, símbolos e imports.

### Deve fazer

* construir ou consultar tabelas de símbolos;
* detectar duplicidades;
* detectar nomes inexistentes;
* detectar ambiguidades;
* associar referências a definições internas estáveis.

### Não deve fazer

* inferir tipos completos;
* validar borrow;
* decidir layout de memória;
* gerar MIR;
* depender de backend.

### Entrada

* HIR;
* módulos disponíveis;
* contexto de sessão.

### Saída

* HIR resolvida ou estruturas associadas;
* símbolos;
* diagnósticos de resolução.

---

## 17. Sistema de tipos

### Responsabilidade

Inferir e verificar tipos conforme a especificação.

### Deve fazer

* representar tipos internos;
* verificar compatibilidade;
* aplicar subtipagem e coerções aprovadas;
* resolver chamadas;
* diagnosticar erros de tipo;
* preservar determinismo.

### Não deve fazer

* criar regras de compatibilidade não especificadas;
* validar regras de runtime;
* depender de layout físico;
* depender de backend;
* usar regras do Rust como substituto das regras da Capi.

### Entrada

* HIR resolvida;
* símbolos;
* restrições aplicáveis.

### Saída

* informações de tipo;
* coerções;
* diagnósticos de tipo.

---

## 18. Modelo de objetos

### Responsabilidade

Representar classes, identidade, herança, métodos e despacho dinâmico no nível semântico.

### Deve fazer

* validar hierarquias;
* validar override;
* preservar identidade lógica;
* preparar informação para layout posterior;
* manter coerência com subtipagem nominal.

### Não deve fazer

* assumir layout final de backend sem contrato;
* transformar `ObjectId` em referência Rust por semântica;
* implementar runtime;
* depender de Cranelift ou LLVM.

---

## 19. Ownership, borrowing e regiões

### Responsabilidade

Verificar garantias de segurança de memória da Capi.

### Deve fazer

* representar places e access paths;
* verificar moves;
* verificar borrows;
* detectar conflitos;
* verificar regiões;
* validar escape analysis;
* emitir diagnósticos próprios.

### Não deve fazer

* depender do borrow checker do Rust como semântica da Capi;
* decidir geração de código;
* depender de backend;
* mascarar escapes inválidos com runtime.

---

## 20. Domains

### Responsabilidade

Implementar e validar a semântica de Domains no compilador.

### Deve fazer

* representar associação de objetos a Domains;
* validar criação e descarte;
* validar restrições de escape;
* integrar-se ao ownership;
* preparar informação para runtime.

### Não deve fazer

* transformar Domain em mero lifetime Rust;
* depender de backend;
* ocultar regras de segurança em runtime;
* alterar semântica operacional definida.

---

## 21. MIR

### Responsabilidade

Representar fluxo de controle e operações independentes de backend.

### Deve fazer

* representar funções;
* representar blocos básicos;
* representar instruções e terminadores;
* preservar invariantes;
* permitir validação estrutural;
* permitir dumps determinísticos;
* servir como entrada canônica para backend.

### Não deve fazer

* depender de Cranelift;
* depender de LLVM;
* representar registradores físicos;
* codificar ABI de plataforma diretamente;
* acessar estruturas privadas do frontend.

---

## 22. Passes de MIR

### Responsabilidade

Executar validações e transformações independentes de backend sobre MIR.

### Deve fazer

* preservar semântica observável;
* manter invariantes;
* registrar diagnósticos ou erros internos;
* ser testável isoladamente;
* produzir resultado determinístico quando aplicável.

### Não deve fazer

* introduzir otimização específica de Cranelift;
* introduzir otimização específica de LLVM;
* alterar regras da linguagem;
* depender de layout físico final.

---

## 23. Interface de backend

### Responsabilidade

Definir o contrato entre MIR validada e backends concretos.

### Deve fazer

* abstrair seleção de backend;
* definir entrada comum;
* definir saída esperada;
* representar configuração de target;
* isolar falhas de backend;
* permitir comparação entre backends.

### Não deve fazer

* implementar Cranelift diretamente;
* implementar LLVM diretamente;
* depender de frontend;
* aceitar MIR inválida;
* alterar semântica.

---

## 24. Backend Cranelift

### Responsabilidade

Gerar código nativo usando Cranelift.

### Deve fazer

* traduzir MIR para Cranelift IR;
* gerar funções;
* gerar operações do subconjunto;
* produzir objetos ou executáveis;
* reportar falhas de codegen;
* permanecer isolado no crate de backend correspondente.

### Não deve fazer

* substituir MIR por Cranelift IR;
* impor dependências ao frontend;
* redefinir ABI;
* alterar semântica observável;
* bloquear introdução futura do LLVM.

---

## 25. Backend LLVM

### Responsabilidade

Gerar código otimizado usando LLVM quando introduzido.

### Deve fazer

* traduzir MIR para LLVM IR;
* aplicar otimizações LLVM compatíveis;
* produzir artefatos equivalentes ao backend Cranelift;
* participar de testes diferenciais;
* permanecer isolado no crate de backend correspondente.

### Não deve fazer

* substituir MIR por LLVM IR;
* forçar dependência LLVM em crates compartilhados;
* mudar a semântica da linguagem;
* remover Cranelift sem decisão aprovada.

---

## 26. Runtime

### Responsabilidade

Fornecer suporte mínimo de execução necessário ao código gerado.

### Deve fazer

* inicialização;
* finalização;
* suporte a Domains;
* suporte a ObjectId;
* suporte a objetos;
* alocação conforme contrato;
* integração com código gerado.

### Não deve fazer

* redefinir semântica da linguagem;
* aceitar programas que o compilador deveria rejeitar;
* expor detalhes privados ao frontend;
* substituir análises estáticas por checks arbitrários.

---

## 27. Biblioteca padrão

### Responsabilidade

Fornecer APIs oficiais da linguagem conforme especificação.

### Deve fazer

* implementar tipos fundamentais;
* implementar APIs mínimas de bootstrap;
* integrar-se ao runtime por contratos explícitos;
* evoluir de forma compatível;
* ser testada como parte do ecossistema.

### Não deve fazer

* criar semântica especial não documentada;
* exigir hardcoding arbitrário no frontend;
* depender de detalhes privados do compilador;
* ser confundida com runtime.

---

## 28. Toolchain

### Responsabilidade

Orquestrar fluxos de desenvolvimento da linguagem Capi.

### Deve fazer

* criar projetos;
* invocar compilador;
* executar build;
* executar testes;
* formatar código;
* gerar documentação;
* integrar ferramentas auxiliares.

### Não deve fazer

* duplicar regras internas do compilador;
* conter lógica semântica da linguagem;
* depender de detalhes privados de HIR ou MIR;
* substituir diagnósticos estruturados.

---

## 29. Testes

### Responsabilidade

Validar componentes, integração, conformidade e regressões.

### Deve fazer

* testar componentes isoladamente;
* testar pipeline completo;
* testar casos positivos;
* testar casos negativos;
* registrar regressões;
* validar dumps quando aplicável;
* permitir testes diferenciais.

### Não deve fazer

* depender de artefatos não reproduzíveis;
* mascarar comportamento indefinido;
* substituir documentação de contrato;
* acoplar conformidade a detalhes privados desnecessários.

---

## 30. Build e CI

### Responsabilidade

Garantir que o workspace compile, teste e valide de forma reproduzível.

### Deve fazer

* executar build;
* executar testes;
* executar formatação;
* executar lint;
* validar comandos obrigatórios;
* preservar rastreabilidade de falhas.

### Não deve fazer

* esconder falhas com scripts silenciosos;
* depender de estado local não documentado;
* aprovar alteração sem testes obrigatórios;
* modificar contratos do projeto.

---

## 31. Observabilidade

### Responsabilidade

Permitir inspeção técnica da implementação.

### Deve fazer

* oferecer dumps;
* oferecer logs internos controlados;
* permitir rastreamento de fases;
* ajudar reprodução de erros;
* preservar determinismo em saídas testáveis.

### Não deve fazer

* vazar dados irrelevantes em saída pública;
* tornar dumps API pública sem aprovação;
* depender de backend para observar frontend;
* alterar comportamento compilado.

---

## 32. Bootstrap

### Responsabilidade

Conduzir a migração progressiva da implementação inicial em Rust para Capi.

### Deve fazer

* preservar compilador de bootstrap enquanto necessário;
* comparar componentes Rust e Capi quando aplicável;
* registrar cadeia de confiança;
* validar reprodutibilidade;
* bloquear regressões funcionais;
* reduzir dependência operacional de Rust progressivamente.

### Não deve fazer

* alterar especificação da linguagem;
* remover implementação Rust antes de validação;
* aceitar divergência funcional sem documentação;
* misturar protótipos com implementação oficial.

---

## 33. Matriz resumida de responsabilidades

| Componente | Responsabilidade principal | Não objetivo principal |
| --- | --- | --- |
| `capi-cli` | Executável `capic` | Implementar pipeline |
| `capi-driver` | Coordenar fases | Implementar fases |
| `capi-session` | Contexto da invocação | Estado global genérico |
| `capi-diagnostics` | Diagnósticos estruturados | Definir semântica |
| `capi-source` | Fontes e spans | Tokenizar |
| Lexer | Fonte para tokens | Construir AST |
| Parser | Tokens para AST | Resolver nomes |
| AST | Sintaxe | Semântica |
| HIR | Semântica inicial | Backend |
| Resolver | Nomes e símbolos | Tipagem completa |
| Type checker | Tipos | Codegen |
| Memory checks | Ownership, borrow, regiões | Backend |
| Domains | Regras de Domain | Runtime arbitrário |
| MIR | IR independente de backend | Cranelift/LLVM |
| Backend interface | Contrato de backend | Backend concreto |
| Cranelift backend | Codegen Cranelift | Definir linguagem |
| LLVM backend | Codegen LLVM | Substituir MIR |
| Runtime | Suporte de execução | Análise semântica |
| Std | APIs oficiais | Runtime |
| Toolchain | Fluxos de projeto | Regras internas do compilador |

---

## 34. Critérios para mover responsabilidade

Uma responsabilidade deve ser movida quando:

* criar dependência circular;
* exigir acesso a dados privados de outro componente;
* crescer além do escopo original;
* duplicar lógica existente;
* impedir teste isolado;
* misturar fases do pipeline.

A mudança deve atualizar:

* documentação aplicável;
* testes;
* dependências do workspace;
* ADR, quando arquiteturalmente relevante.

---

## 35. Relação com o Stage 0

Para concluir o Stage 0, este documento deve orientar a criação dos componentes mínimos:

```text
capi-cli
capi-driver
capi-session
capi-diagnostics
capi-source
capi-common
```

Critérios mínimos:

* cada componente possui responsabilidade clara;
* dependências iniciais são acíclicas;
* `capic --help` funciona;
* `capic --version` funciona;
* arquivo inexistente produz erro controlado;
* testes iniciais validam comportamento observável.

---

## 36. Critérios de aceite deste documento

Este documento estará pronto para orientar o Stage 0 quando:

* definir responsabilidades dos componentes mínimos;
* definir responsabilidades das fases futuras em nível suficiente;
* explicitar não objetivos;
* preservar fronteiras entre fases;
* não contradizer os Documentos 00 a 28;
* complementar `COMPILER-ARCHITECTURE.md` e `WORKSPACE-ARCHITECTURE.md`;
* permitir criação incremental dos crates fundamentais.

---

## 37. Síntese

A implementação oficial da Capi deve evoluir por componentes com responsabilidades explícitas.

Cada componente deve fazer uma coisa bem definida, expor contratos claros e evitar assumir trabalho pertencente a outra fase.

Essa disciplina é necessária para manter o compilador testável, rastreável, preparado para múltiplos backends e apto ao bootstrap.
