# AST Lowering

**Projeto:** Linguagem Capi  
**Documento:** AST-LOWERING  
**Status:** Aprovado  
**Stage:** Stage 2 — Parser e AST / Stage 3 — HIR e resolução de nomes  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o contrato de engenharia para a transformação da AST em HIR na implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- quando o lowering de AST para HIR pode ser executado;
- quais informações são preservadas;
- quais informações são normalizadas;
- quais informações não podem ser introduzidas nessa etapa;
- como diagnósticos e nós de erro da AST afetam o lowering;
- como manter rastreabilidade entre código-fonte, AST e HIR;
- quais formas canônicas iniciais devem ser produzidas;
- quais testes validam a transformação.

---

## 2. Escopo

Este documento cobre:

- lowering inicial de AST para HIR;
- contrato do componente de lowering;
- entrada e saída da transformação;
- criação de elementos HIR a partir de nós AST;
- preservação de spans;
- preservação da origem AST;
- normalizações estruturais permitidas;
- tratamento de AST parcial com erro;
- limites entre lowering, resolução de nomes e tipos;
- dump inicial de HIR quando aplicável ao Stage 3;
- testes obrigatórios de lowering.

Este documento não cobre:

- modelo completo da HIR;
- resolução de nomes;
- construção de tabelas de símbolos;
- inferência ou verificação de tipos;
- checagem de ownership;
- análise de Domains;
- lowering de HIR para MIR;
- geração de código;
- layout de objetos;
- ABI.

Esses temas pertencem a:

- `HIR-MODEL.md`;
- `SYMBOL-MODEL.md`;
- `SCOPE-MODEL.md`;
- `NAME-RESOLUTION.md`;
- `TYPE-MODEL.md`;
- `TYPE-INFERENCE.md`;
- `OWNERSHIP-MODEL.md`;
- `DOMAIN-IMPLEMENTATION.md`;
- `MIR-LOWERING.md`;
- `OBJECT-LAYOUT.md`;
- `ABI-IMPLEMENTATION.md`.

---

## 3. Posição nos Stages

`AST-LOWERING.md` é documento obrigatório do Stage 2 porque o modelo da AST precisa ser definido já considerando sua consumidora imediata.

A implementação efetiva do lowering pertence ao Stage 3, onde também são implementados:

- HIR;
- IDs internos;
- símbolos;
- escopos;
- resolução de nomes;
- dump da HIR.

Critério prático:

```text
Stage 2
Define o contrato e garante que AST seja abaixável.

Stage 3
Implementa o lowering e valida a HIR inicial.
```

No Stage 2, o parser não precisa produzir HIR. Ele precisa produzir uma AST que obedeça aos contratos necessários para este documento.

---

## 4. Princípios

O lowering de AST para HIR deve seguir estes princípios:

- a AST não deve ser modificada;
- a HIR inicial deve ser independente da estrutura física da AST;
- toda HIR deve preservar rastreabilidade com AST e spans;
- normalizações devem preservar significado;
- a transformação deve ser determinística;
- dados semânticos resolvidos não devem ser inventados;
- nós de erro na AST devem bloquear ou produzir HIR marcada como inválida;
- fases posteriores devem operar sobre HIR, não sobre AST;
- lowering não deve depender da sequência original de tokens;
- lowering não deve depender de backend, MIR ou ABI.

---

## 5. Papel no Pipeline

Fluxo conceitual:

```text
SourceFile
    ↓
Lexer
    ↓
TokenStream
    ↓
Parser
    ↓
AST
    ↓
AST Lowering
    ↓
HIR inicial
    ↓
Resolução de nomes
    ↓
Inferência e verificação de tipos
    ↓
Verificação semântica
    ↓
Lowering para MIR
```

O lowering de AST para HIR é a primeira transformação que sai da representação puramente sintática e cria a representação semântica inicial usada pelo restante do frontend.

---

## 6. Entradas

Entrada conceitual:

```rust
pub struct AstLoweringInput<'a> {
    pub ast: &'a Ast,
    pub source_map: &'a SourceMap,
}
```

Também pode receber:

- sessão de compilação;
- interner de símbolos;
- coletor de diagnósticos;
- alocador de HIR;
- configuração de dump.

Regras:

- a AST deve estar completa o suficiente para traversal;
- a AST pode conter nós de erro;
- diagnósticos do parser devem estar disponíveis;
- o lowering não deve consumir tokens diretamente;
- o lowering não deve modificar a AST.

---

## 7. Saídas

Saída conceitual:

```rust
pub struct AstLoweringOutput {
    pub hir: Option<Hir>,
    pub diagnostics: Vec<Diagnostic>,
    pub blocked: bool,
}
```

Formas equivalentes são permitidas, desde que expressem:

- HIR inicial construída;
- diagnósticos de lowering;
- indicação de bloqueio quando a AST não permite HIR útil.

Regras:

- AST sintaticamente válida deve produzir HIR inicial;
- AST com erros recuperados pode produzir HIR parcial somente se isso for explicitamente suportado;
- se houver `AstErrorNode` em posição semanticamente obrigatória, o lowering pode bloquear;
- bloqueio de lowering não deve apagar diagnósticos do parser.

---

## 8. Organização do Componente

O lowering pode viver em:

```text
capi-hir
capi-sema
capi-frontend
```

ou crate equivalente, conforme a organização aprovada do workspace.

Responsabilidades do componente:

- percorrer a AST;
- criar elementos HIR;
- atribuir IDs internos da HIR;
- preservar spans;
- preservar origem AST;
- aplicar normalizações estruturais permitidas;
- emitir diagnósticos de inconsistência estrutural;
- bloquear fases posteriores quando a AST estiver inválida.

Dependências permitidas:

- `capi-source`;
- `capi-diagnostics`;
- `capi-ast`;
- `capi-hir`;
- `capi-common`;
- biblioteca padrão Rust.

O componente não deve depender de:

- resolver de nomes como pré-condição;
- type checker;
- borrow checker;
- MIR;
- backends.

---

## 9. Estado Interno do Lowerer

Estado conceitual:

```rust
struct AstLowerer<'ast> {
    ast: &'ast Ast,
    hir: HirBuilder,
    diagnostics: DiagnosticSink,
    ast_to_hir: AstToHirMap,
}
```

Componentes:

| Componente | Responsabilidade |
| --- | --- |
| `AstLowerer` | Coordena traversal e transformação. |
| `HirBuilder` | Cria elementos HIR e IDs determinísticos. |
| `AstToHirMap` | Preserva relação entre nós AST e elementos HIR. |
| `DiagnosticSink` | Registra problemas estruturais encontrados no lowering. |
| `LoweringContext` | Mantém contexto atual de módulo, declaração, função ou bloco. |

O lowerer deve ser determinístico e não deve guardar referências para dados temporários inválidos após a transformação.

---

## 10. IDs e Rastreabilidade

Todo elemento HIR relevante deve possuir:

- ID interno próprio;
- span de origem;
- referência à origem AST quando aplicável.

Contrato conceitual:

```rust
pub struct HirOrigin {
    pub ast_node: Option<AstNodeId>,
    pub span: Span,
}
```

Regras:

- `HirId` e `AstNodeId` são identidades distintas;
- HIR não deve reutilizar IDs da AST como identidade própria;
- um nó AST pode gerar zero, um ou múltiplos elementos HIR;
- um elemento HIR pode apontar para o nó AST principal que o originou;
- quando uma normalização combinar múltiplos nós AST, a HIR deve preservar span representativo e, se necessário, origens auxiliares;
- spans devem permanecer válidos para diagnósticos posteriores.

---

## 11. Relação AST-HIR

Nem todo nó AST vira elemento HIR.

Categorias comuns:

| AST | HIR |
| --- | --- |
| `CompilationUnit` | módulo/unidade HIR |
| `ModuleDecl` | caminho de módulo inicial |
| `ImportDecl` | import sem resolução |
| `ClassDecl` | item de tipo/classe |
| `FunctionDecl` | item de função/método |
| `Param` | parâmetro HIR |
| `TypeSyntax` | referência de tipo não resolvida |
| `Block` | bloco HIR |
| `Stmt` | comando HIR |
| `Expr` | expressão HIR |
| `Modifier` | flags ou lista de modificadores pendentes |
| `Attribute` | atributo pendente |
| delimitador | normalmente descartado |
| agrupamento sintático | descartado ou preservado como origem |
| `AstErrorNode` | bloqueio ou erro HIR explícito |

Elementos puramente sintáticos podem ser descartados se a rastreabilidade continuar suficiente.

---

## 12. Ordem de Traversal

Ordem recomendada:

```text
1. unidade de compilação
2. módulo
3. imports
4. declarações de alto nível
5. membros
6. assinaturas
7. tipos sintáticos
8. corpos
9. comandos
10. expressões
```

Regras:

- a ordem textual deve ser preservada em listas HIR quando ela puder afetar diagnósticos, documentação ou resolução posterior;
- IDs devem ser atribuídos em ordem determinística;
- traversal não deve depender de mapas sem ordenação estável;
- declarações podem ser registradas antes de corpos se isso simplificar resolução posterior, desde que a HIR resultante seja consistente.

---

## 13. Unidade, Módulos e Imports

### 13.1 Unidade de Compilação

`CompilationUnit` deve produzir uma unidade HIR ou módulo inicial.

Regras:

- `SourceId` deve ser preservado;
- span da unidade deve ser preservado;
- declarações devem ser associadas à unidade;
- ausência de declaração explícita de módulo deve ser representada de forma explícita ou por módulo implícito definido pela sessão.

### 13.2 Módulo

`ModuleDecl` deve ser convertido em caminho de módulo pendente.

Regras:

- o caminho permanece não resolvido;
- o lowerer não verifica correspondência com diretórios;
- erro de módulo duplicado ou ausente pertence à fase apropriada de resolução/configuração, salvo se a AST violar contrato estrutural.

### 13.3 Imports

`ImportDecl` deve produzir entradas de import na HIR.

Regras:

- imports preservam ordem textual;
- wildcard deve permanecer distinguível;
- nenhum módulo importado deve ser resolvido no lowering;
- diagnósticos de import inexistente pertencem à resolução de nomes.

---

## 14. Declarações

Declarações AST devem virar itens HIR.

Contrato conceitual:

```rust
fn lower_decl(&mut self, decl: &Decl) -> Option<HirItemId>;
```

Regras:

- cada declaração válida deve receber identidade HIR;
- nome deve ser preservado como nome não resolvido ou símbolo textual;
- modificadores devem ser preservados como dados pendentes;
- atributos devem ser preservados como dados pendentes;
- corpo pode ser abaixado imediatamente ou de forma atrasada, desde que a HIR fique consistente;
- duplicidade de nomes não deve ser diagnosticada no lowering.

### 14.1 Modificadores

Modificadores podem ser convertidos para flags sintáticas pendentes.

Exemplos:

```text
public
private
static
override
abstract
sealed
final
unsafe
```

Regras:

- significado e validade de combinação pertencem à análise semântica;
- o lowering pode preservar ordem original para diagnósticos;
- duplicatas podem permanecer para diagnóstico posterior, salvo se o parser já as tratou.

### 14.2 Atributos

Atributos devem ser preservados como metadados pendentes.

Regras:

- nome do atributo permanece não resolvido;
- argumentos são abaixados como expressões ou valores sintáticos pendentes;
- efeitos como `@domain` não são aplicados no lowering inicial.

---

## 15. Classes, Interfaces e Traits

### 15.1 Classes

`ClassDecl` deve produzir item HIR de classe.

Preservar:

- nome;
- parâmetros genéricos;
- modificadores;
- atributos;
- superclasse sintática pendente;
- interfaces implementadas pendentes;
- traits usadas pendentes;
- membros;
- span e origem AST.

Não verificar:

- existência da superclasse;
- herança simples;
- implementação de interface;
- validade de traits;
- layout de objeto;
- construtores obrigatórios.

### 15.2 Interfaces

`InterfaceDecl` deve produzir item HIR de interface.

Preservar:

- nome;
- parâmetros genéricos;
- membros;
- modificadores e atributos;
- spans.

Não verificar se membros são semanticamente permitidos em interface nesta etapa.

### 15.3 Traits

`TraitDecl` deve produzir item HIR de trait.

Preservar:

- nome;
- parâmetros genéricos;
- membros;
- modificadores e atributos;
- spans.

Não verificar regras de estado persistente ou composição nesta etapa.

### 15.4 Membros

Membros devem ser associados ao item de tipo que os contém.

Regras:

- métodos viram funções/métodos HIR;
- campos viram campos HIR com tipo e inicializador pendentes;
- construtores viram construtores HIR próprios;
- ordem textual dos membros deve ser preservada;
- acesso a `this`, campos e métodos não deve ser resolvido no lowering.

---

## 16. Funções, Métodos e Construtores

### 16.1 Funções e Métodos

Preservar:

- nome;
- parâmetros;
- retorno explícito ou ausência de retorno;
- parâmetros genéricos;
- modificadores e atributos;
- corpo;
- span.

Normalização permitida:

- retorno omitido pode ser representado como retorno implícito pendente de `Unit`;
- função sem corpo pode ser representada como assinatura;
- método e função livre podem compartilhar forma HIR com contexto de pertencimento.

Não realizar:

- resolução de overload;
- checagem de retorno obrigatório;
- inferência de tipos de parâmetros;
- validação de `override`;
- validação de `static`.

### 16.2 Construtores

Construtores devem permanecer distintos de funções comuns ou receber marcador HIR explícito de construtor.

Regras:

- nome implícito da classe não deve ser inventado como identificador de usuário;
- parâmetros e corpo devem ser preservados;
- chamadas a outros construtores ou superclasse não são validadas;
- regras de inicialização pertencem a fases posteriores.

### 16.3 Parâmetros

Parâmetros devem produzir elementos HIR próprios ou entradas de assinatura.

Preservar:

- nome;
- tipo anotado pendente;
- valor padrão, se a gramática permitir;
- span;
- ordem.

Não registrar símbolo resolvido nesta etapa.

---

## 17. Tipos Sintáticos para Tipos Pendentes

`TypeSyntax` deve ser convertido para representação HIR de tipo pendente ou referência de tipo não resolvida.

Contrato conceitual:

```rust
pub enum HirTypeRef {
    Path(UnresolvedPath),
    Generic { base: UnresolvedPath, args: Vec<HirTypeRef> },
    Array { element: Box<HirTypeRef>, size: Option<HirExprId> },
    Tuple(Vec<HirTypeRef>),
    Function { params: Vec<HirTypeRef>, ret: Box<HirTypeRef> },
    Error(HirErrorId),
}
```

Regras:

- nomes como `Int32`, `Bool`, `String` e `Unit` permanecem não resolvidos ou marcados como nomes primitivos pendentes, conforme `HIR-MODEL.md`;
- argumentos genéricos preservam ordem;
- bounds genéricos permanecem pendentes;
- tipos ausentes ou com erro bloqueiam ou geram `HirTypeRef::Error`;
- subtipagem, alias, variância e existência de tipos não são verificados.

---

## 18. Blocos e Comandos

### 18.1 Blocos

`Block` deve produzir bloco HIR.

Regras:

- comandos preservam ordem;
- span do bloco é preservado;
- escopo lexical pode ser registrado como estrutura pendente se necessário para Stage 3, mas criação/validação de escopos pertence a `SCOPE-MODEL.md`;
- bloco vazio produz lista vazia.

### 18.2 Declarações Locais

`let` e `const` locais devem virar declarações locais HIR.

Preservar:

- nome;
- tipo anotado pendente;
- inicializador;
- mutabilidade ou imutabilidade sintática;
- span.

Não verificar:

- inicialização obrigatória;
- tipo do inicializador;
- shadowing;
- uso antes de definição.

### 18.3 Controle de Fluxo

Comandos como `if`, `while`, `for`, `switch` e `match` devem preservar sua estrutura semântica básica.

Normalizações permitidas:

- `else if` pode virar `else` contendo outro `if`;
- comando de corpo único pode virar bloco implícito se a HIR exigir blocos uniformes;
- `for` clássico pode ser representado por forma HIR própria ou normalizado para componentes de laço, desde que origem e spans sejam preservados.

Não verificar:

- condição booleana;
- alcançabilidade;
- exaustividade;
- destino de `break`/`continue`;
- retorno obrigatório.

---

## 19. Expressões

Expressões AST devem virar expressões HIR.

Contrato conceitual:

```rust
fn lower_expr(&mut self, expr: &Expr) -> HirExprId;
```

Regras:

- literais preservam categoria, valor normalizado quando disponível e span;
- nomes viram caminhos ou referências não resolvidas;
- chamadas preservam callee e argumentos;
- acesso a membro preserva base e nome;
- indexação preserva base e índice;
- `new` preserva tipo e argumentos;
- operadores preservam operador sintático;
- atribuição preserva lado esquerdo e direito;
- agrupamento pode ser eliminado se a árvore já preserva precedência;
- expressão com erro vira erro HIR ou bloqueia.

Não realizar:

- resolução de variável, campo, função ou método;
- overload de operadores;
- conversões implícitas;
- avaliação constante;
- inferência de tipo.

---

## 20. Patterns, `switch` e `match`

### 20.1 Patterns

Padrões AST devem virar padrões HIR pendentes.

Preservar:

- forma do padrão;
- caminho não resolvido;
- subpadrões;
- literais;
- wildcard;
- span.

Não verificar:

- exaustividade;
- compatibilidade com tipo analisado;
- classes seladas;
- variantes de `Optional` ou `Result`.

### 20.2 `switch`

`switch` deve preservar expressão de seleção e casos.

Regras:

- `case` e `default` preservam ordem;
- duplicidade ou cobertura não é verificada;
- corpos são abaixados como blocos ou listas de comandos HIR.

### 20.3 `match`

`match` deve preservar expressão, braços e padrões.

Normalização permitida:

- corpo de braço com expressão pode virar bloco/retorno implícito se a HIR definir forma única;
- formas sintáticas equivalentes de padrão podem virar forma canônica.

Exaustividade pertence a fases semânticas.

---

## 21. Normalizações Permitidas

Normalizações permitidas no lowering inicial:

- remover agrupamentos sintáticos redundantes;
- transformar retorno omitido em retorno pendente de `Unit`;
- representar função sem corpo como assinatura;
- transformar corpo único em bloco implícito quando HIR exigir;
- transformar paths sintáticos em caminhos não resolvidos canônicos;
- transformar modificadores em flags pendentes;
- transformar atributos em metadados pendentes;
- separar declaração e inicializador em campos HIR distintos;
- normalizar `else if` como aninhamento explícito.

Toda normalização deve preservar:

- span útil;
- origem AST;
- ordem observável;
- significado do programa;
- capacidade de emitir diagnóstico preciso.

---

## 22. Normalizações Proibidas

O lowering inicial não deve:

- resolver nomes;
- criar símbolos definitivos;
- inferir tipos;
- aplicar conversões implícitas;
- selecionar overload;
- reordenar declarações com significado observável para diagnósticos;
- eliminar declaração inválida sem diagnóstico;
- expandir chamadas de biblioteca padrão;
- aplicar regras de ownership;
- decidir layout de objeto;
- introduzir MIR;
- transformar `new` em alocação concreta;
- executar avaliação constante;
- aplicar efeitos de atributos como `@domain`.

Essas transformações pertencem a fases posteriores.

---

## 23. AST com Erros

AST com `AstErrorNode` deve ser tratada de forma explícita.

Política inicial:

```text
Se houver erro sintático bloqueador, não produzir HIR utilizável.
Se houver erro localizado e a HIR parcial for útil para ferramentas, produzir HIR marcada como inválida.
```

Regras:

- lowering não deve ocultar diagnósticos do parser;
- todo erro AST convertido deve gerar marcador HIR de erro ou bloqueio;
- fases semânticas não devem executar como se a HIR fosse válida;
- dump de HIR parcial deve indicar nós de erro;
- a decisão de seguir com HIR parcial deve ser configurável por modo de compilação ou ferramenta.

Para compilação normal, a presença de erro sintático deve bloquear resolução semântica completa.

---

## 24. Diagnósticos do Lowering

O lowering deve emitir diagnósticos apenas para problemas da transformação estrutural.

Categorias iniciais:

- AST contém nó de erro em posição obrigatória;
- AST viola invariante declarada por `AST-MODEL.md`;
- span inválido ou ausente em nó obrigatório;
- construção sintática ainda não suportada pelo lowering;
- erro interno de mapeamento AST-HIR;
- limite estrutural excedido.

O lowering não deve emitir:

- nome não encontrado;
- tipo incompatível;
- símbolo duplicado;
- expressão não booleana;
- retorno ausente;
- membro inacessível;
- violação de ownership.

Esses diagnósticos pertencem às fases semânticas.

---

## 25. Determinismo

Para a mesma AST, mesma versão do compilador e mesmas opções:

- HIR inicial deve ser funcionalmente equivalente;
- IDs HIR devem ser atribuídos em ordem determinística;
- mapeamento AST-HIR deve ser determinístico;
- diagnósticos de lowering devem ser determinísticos;
- dump de HIR inicial deve ser determinístico.

O lowering não deve depender de:

- ordem de hash maps sem ordenação;
- endereços de memória;
- paralelismo sem ordenação explícita;
- locale do sistema;
- caminhos absolutos não normalizados.

---

## 26. Interface com Resolução de Nomes

A HIR produzida pelo lowering deve estar pronta para resolução de nomes.

Ela deve fornecer:

- itens declarados com nomes preservados;
- estrutura de módulos e imports;
- corpos de funções e blocos;
- caminhos não resolvidos;
- referências de tipo não resolvidas;
- expressões de nome não resolvidas;
- spans e origens AST;
- ordem textual suficiente para diagnósticos.

Ela não deve fornecer:

- símbolos resolvidos;
- escopos finais;
- binding de identificadores;
- tipos finais;
- relações de herança validadas.

---

## 27. Interface com Dumps

Quando o Stage 3 implementar:

```bash
capic --emit hir arquivo.capi
```

o lowering deve fornecer HIR inicial percorrível.

Requisitos de dump:

- exibir unidade/módulo;
- exibir itens;
- exibir assinaturas;
- exibir blocos, comandos e expressões;
- exibir caminhos não resolvidos como não resolvidos;
- exibir spans;
- exibir origens AST quando habilitado;
- não exibir dados de resolução inexistentes;
- não exibir endereços ou IDs não determinísticos.

---

## 28. Testes Obrigatórios

Testes de lowering devem cobrir:

- unidade mínima;
- módulo explícito;
- imports simples e wildcard;
- função livre;
- função com retorno omitido;
- parâmetros;
- classe vazia;
- classe com campos, métodos e construtor;
- interface;
- trait;
- modificadores;
- atributos;
- tipos nomeados;
- tipos genéricos;
- arrays e tuplas;
- declaração local `let`;
- constante;
- blocos;
- `if` e `else if`;
- `while`;
- `for`;
- `switch`;
- `match`;
- literais;
- nomes não resolvidos;
- chamadas;
- acesso a membro;
- indexação;
- `new`;
- operadores;
- agrupamento redundante;
- spans preservados;
- origem AST preservada;
- AST com erro bloqueando lowering;
- HIR parcial marcada como inválida, se suportada;
- dump determinístico.

Testes de lowering não devem exigir resolução de nomes nem inferência de tipos.

---

## 29. Critérios de Aceite

Para este documento ser considerado implementado no Stage 3:

- existe API de lowering de AST para HIR;
- AST válida produz HIR inicial;
- HIR preserva spans e origem AST;
- HIR não depende diretamente da estrutura física da AST;
- nomes e tipos permanecem não resolvidos;
- normalizações permitidas estão documentadas e testadas;
- AST com erro é bloqueada ou marcada explicitamente;
- diagnósticos de lowering são estruturados;
- testes obrigatórios passam;
- dump de HIR inicial é determinístico quando a flag existir.

Para o Stage 2, este documento é considerado aprovado quando seus contratos forem compatíveis com `AST-MODEL.md`, `PARSER-IMPLEMENTATION.md` e `PARSER-RECOVERY.md`.

---

## 30. Relações Normativas

Este documento depende diretamente de:

- Documento 15 — Parser e AST;
- Documento 16 — Representação Semântica (HIR);
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial;
- `AST-MODEL.md`;
- `PARSER-IMPLEMENTATION.md`;
- `PARSER-RECOVERY.md`;
- `SPANS-AND-LOCATIONS.md`;
- `DIAGNOSTIC-DATA-MODEL.md`.

Este documento orienta diretamente:

- `HIR-MODEL.md`;
- `SYMBOL-MODEL.md`;
- `SCOPE-MODEL.md`;
- `NAME-RESOLUTION.md`;
- `SEMANTIC-TESTS.md`;
- implementação de `capic --emit hir`.
