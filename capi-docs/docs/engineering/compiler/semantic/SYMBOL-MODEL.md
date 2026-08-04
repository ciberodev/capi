# Symbol Model

**Projeto:** Linguagem Capi  
**Documento:** SYMBOL-MODEL  
**Status:** Aprovado  
**Stage:** Stage 3 — HIR e resolução de nomes  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o modelo de engenharia de símbolos da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- o que é um símbolo semântico;
- quais declarações introduzem símbolos;
- como símbolos se relacionam com HIR, escopos e nomes textuais;
- quais IDs e chaves devem ser estáveis durante a compilação;
- como tabelas de símbolos registram declarações;
- como conflitos, duplicidades e ambiguidades são representados;
- quais dados pertencem ao Stage 3;
- quais dados pertencem a fases posteriores;
- quais invariantes devem ser preservadas;
- quais testes validam o modelo.

Neste documento, "símbolo" significa uma entidade semântica declarada no programa, identificada por `SymbolId` ou mecanismo equivalente. Isso não deve ser confundido com texto internado de identificadores, que representa apenas o nome escrito pelo usuário.

---

## 2. Escopo

Este documento cobre:

- identidade de símbolos;
- nomes de símbolos;
- categorias de símbolos;
- declarações que introduzem símbolos;
- relação entre símbolos e elementos HIR;
- tabelas de símbolos;
- registro de declarações;
- conflitos no mesmo escopo;
- shadowing entre escopos;
- símbolos importados;
- símbolos de módulos;
- símbolos locais, parâmetros e membros;
- símbolos de erro;
- determinismo e dumps;
- interface com resolução de nomes, escopos e tipagem.

Este documento não cobre:

- algoritmo completo de resolução de nomes;
- árvore ou grafo completo de escopos;
- regras finais de visibilidade;
- inferência ou verificação de tipos;
- overload e seleção de chamadas;
- coerções;
- ownership;
- Domains;
- layout de objetos;
- geração de símbolos de objeto, linker ou ABI.

Esses temas pertencem a:

- `SCOPE-MODEL.md`;
- `NAME-RESOLUTION.md`;
- `TYPE-MODEL.md`;
- `TYPE-INFERENCE.md`;
- `SUBTYPING-AND-COERCIONS.md`;
- `OWNERSHIP-MODEL.md`;
- `DOMAIN-IMPLEMENTATION.md`;
- `OBJECT-LAYOUT.md`;
- `NAME-MANGLING.md`;
- `ABI-IMPLEMENTATION.md`.

---

## 3. Posição no Stage 3

No Stage 3, o modelo de símbolos sustenta:

- criação de identidades internas para declarações;
- construção de tabelas de símbolos;
- associação entre escopos e declarações;
- detecção de símbolos duplicados;
- base para resolver referências inexistentes ou ambíguas;
- enriquecimento da HIR com `SymbolId` e referências resolvidas;
- dump HIR com informações de resolução quando aplicável.

Fluxo conceitual:

```text
HIR inicial
    ↓
Construção de escopos
    ↓
Registro de declarações
    ↓
Criação de SymbolId
    ↓
Tabelas de símbolos por escopo
    ↓
Resolução de referências
    ↓
HIR com símbolos resolvidos
```

O modelo de símbolos não substitui a HIR. Ele é uma estrutura semântica auxiliar usada para enriquecer a HIR com identidades resolvidas.

---

## 4. Princípios

O modelo de símbolos deve seguir estes princípios:

- símbolo representa declaração, não ocorrência textual;
- texto internado não é símbolo semântico;
- cada declaração válida que introduz nome deve possuir identidade própria;
- símbolos devem ser opacos e tipados;
- símbolos devem ser estáveis durante uma sessão de compilação;
- a criação de símbolos deve ser determinística;
- símbolos devem apontar para origem HIR e span;
- tabelas de símbolos devem preservar conflitos de forma diagnóstica;
- shadowing permitido não altera identidade de símbolos externos;
- referências ambíguas não devem escolher símbolo arbitrário;
- símbolos não devem carregar tipos inferidos na fase inicial;
- símbolos não devem depender de MIR, backend, ABI ou linker.

---

## 5. Terminologia

| Termo | Significado |
| --- | --- |
| Nome textual | Texto escrito pelo usuário, normalmente internado como `SymbolName` ou `Symbol`. |
| Símbolo semântico | Identidade interna de uma declaração, representada por `SymbolId`. |
| Declaração | Elemento HIR que introduz um nome em algum escopo. |
| Referência | Uso de nome que deve apontar para uma declaração. |
| Entrada de símbolo | Registro contendo nome, categoria, origem, escopo e alvo HIR. |
| Tabela de símbolos | Estrutura que associa nomes textuais a entradas de símbolo em um escopo. |
| Espaço de nomes | Partição lógica usada quando categorias diferentes não conflitam entre si. |
| Binding | Associação entre uma referência HIR e um símbolo resolvido. |

Regras:

- `SymbolName` ou `Symbol` pode ser usado para texto internado;
- `SymbolId` deve ser usado para identidade semântica;
- documentação e APIs devem evitar chamar texto internado de "símbolo" sem qualificação.

---

## 6. Entidades Conceituais

| Entidade | Responsabilidade |
| --- | --- |
| `SymbolId` | Identidade opaca de uma declaração registrada. |
| `SymbolName` | Nome textual internado ou equivalente. |
| `SymbolTable` | Coleção de símbolos organizados por escopo. |
| `SymbolEntry` | Registro de uma declaração nomeada. |
| `SymbolKind` | Categoria semântica do símbolo. |
| `Namespace` | Espaço lógico usado para separar categorias quando a linguagem permitir. |
| `DeclarationSite` | Origem declarativa do símbolo na HIR. |
| `SymbolVisibility` | Informação de visibilidade sintática ou semântica inicial. |
| `SymbolState` | Estado de registro, exposição ou recuperação do símbolo. |
| `ScopeSymbols` | Conjunto de entradas de símbolo pertencentes a um `ScopeId`. |
| `ImportBinding` | Entrada que representa símbolo disponibilizado por import. |
| `DuplicateSymbol` | Registro de conflito entre declarações no mesmo escopo. |

Os nomes concretos podem variar na implementação. O contrato obrigatório é a presença dessas responsabilidades.

---

## 7. Identidade de Símbolos

Cada símbolo semântico deve possuir um identificador próprio.

Contrato conceitual:

```rust
pub struct SymbolId(u32);
```

Regras:

- `SymbolId` deve ser único dentro da sessão ou unidade semântica definida pela implementação;
- `SymbolId` deve ser opaco;
- `SymbolId` não deve codificar nome, tipo, escopo ou posição;
- `SymbolId` não deve depender de endereço de memória;
- `SymbolId` deve permanecer estável durante todas as fases que usam a mesma HIR;
- símbolos diferentes podem ter o mesmo nome textual em escopos diferentes;
- símbolos diferentes podem ter o mesmo nome textual em namespaces diferentes quando a linguagem permitir;
- uma declaração válida não deve receber mais de um `SymbolId` para a mesma entidade.

IDs podem ser alocados por arena, índice sequencial ou mecanismo equivalente, desde que a ordem seja determinística.

---

## 8. Nome Textual e Interning

O nome textual representa o identificador escrito no código.

Contrato conceitual:

```rust
pub struct SymbolName(u32);

pub struct SymbolText {
    pub interned: SymbolName,
    pub display: String,
}
```

Regras:

- o texto pode ser internado para comparação eficiente;
- a origem autoritativa do lexema continua sendo `SourceMap` e `Span`;
- interning não cria declaração;
- dois usos do mesmo texto podem apontar para símbolos diferentes;
- comparação por texto não substitui resolução de nomes;
- normalização Unicode, quando aplicável, deve seguir os documentos de fonte, lexer e sintaxe.

Se a implementação já usar `Symbol` como nome internado, `SymbolId` deve continuar sendo distinguível do tipo usado para texto.

---

## 9. Categorias de Símbolos

Símbolos devem registrar a categoria semântica da declaração.

Contrato conceitual:

```rust
pub enum SymbolKind {
    Module,
    Import,
    Class,
    Interface,
    Trait,
    TypeAlias,
    Function,
    Method,
    Constructor,
    Field,
    Const,
    Param,
    Local,
    PatternBinding,
    GenericParam,
    Error,
}
```

Regras:

- categorias devem ser específicas o suficiente para resolução e diagnósticos;
- `Constructor` deve ser distinguível de `Function`;
- `Method` deve ser distinguível de função livre quando isso afetar resolução;
- `Param`, `Local` e `PatternBinding` devem ser distinguíveis para diagnósticos e fases posteriores;
- `Error` representa recuperação controlada, não declaração válida.

Categorias podem ser refinadas em fases futuras sem quebrar este contrato, desde que preservem equivalência funcional.

---

## 10. Declarações que Introduzem Símbolos

Devem introduzir símbolos, quando presentes no subconjunto implementado:

- módulos explícitos;
- imports nomeados ou aliases;
- classes;
- interfaces;
- traits;
- aliases de tipo;
- funções livres;
- métodos;
- construtores;
- campos;
- constantes;
- parâmetros;
- declarações locais;
- bindings de padrões;
- parâmetros genéricos.

Regras:

- elementos puramente sintáticos não introduzem símbolo;
- literais não introduzem símbolo;
- referências a nomes não introduzem símbolo;
- wildcard import não introduz símbolo próprio para cada nome durante o lowering;
- símbolos importados podem ser materializados de forma tardia durante resolução de imports;
- declarações inválidas podem produzir `SymbolKind::Error` somente para recuperação controlada.

---

## 11. Entrada de Símbolo

Cada símbolo registrado deve possuir uma entrada com metadados mínimos.

Contrato conceitual:

```rust
pub struct SymbolEntry {
    pub id: SymbolId,
    pub name: Option<SymbolName>,
    pub kind: SymbolKind,
    pub namespace: Namespace,
    pub declaring_scope: ScopeId,
    pub declaration: DeclarationSite,
    pub visibility: SymbolVisibility,
    pub state: SymbolState,
    pub origin: HirOrigin,
}
```

Regras:

- `name` pode ser ausente para símbolos implícitos ou construtores sem identificador textual próprio;
- `declaring_scope` deve apontar para escopo existente;
- `declaration` deve apontar para elemento HIR que introduziu o símbolo;
- `origin` deve ser suficiente para diagnosticar duplicidade ou uso inválido;
- `visibility` no Stage 3 pode refletir apenas modificadores preservados;
- validade semântica final de visibilidade pertence a fases posteriores quando depender de tipos ou herança.

---

## 12. Local de Declaração

O local declarativo conecta o símbolo ao elemento HIR correspondente.

Contrato conceitual:

```rust
pub enum DeclarationSite {
    Unit(HirUnitId),
    Item(HirItemId),
    Import(HirImportId),
    Member(HirMemberId),
    Param(HirParamId),
    Local(HirLocalId),
    Pattern(HirPatternId),
    TypeRef(HirTypeRefId),
    Synthetic,
    Error(HirErrorId),
}
```

Regras:

- símbolos de item devem apontar para `HirItemId`;
- símbolos de membro devem apontar para `HirMemberId`;
- símbolos de parâmetro devem apontar para `HirParamId`;
- símbolos locais devem apontar para `HirLocalId`;
- parâmetros genéricos podem apontar para representação própria ou `TypeRef`, conforme a HIR concreta;
- símbolos sintéticos devem ser raros e possuir origem rastreável;
- `Error` não deve ser usado para esconder bugs internos.

---

## 13. Namespaces

A implementação deve representar namespaces quando a linguagem distinguir categorias que podem compartilhar o mesmo nome.

Contrato conceitual:

```rust
pub enum Namespace {
    Value,
    Type,
    Module,
    Member,
    Label,
    Macro,
}
```

Regras:

- `Value` cobre variáveis, parâmetros, constantes e funções quando a linguagem tratá-los no mesmo espaço;
- `Type` cobre classes, interfaces, traits, aliases e parâmetros genéricos;
- `Module` cobre módulos e imports de módulo;
- `Member` cobre campos, métodos e construtores quando resolvidos em contexto de tipo;
- namespaces não utilizados pelo subconjunto inicial podem existir sem serem populados;
- conflito só ocorre entre símbolos no mesmo escopo e namespace, salvo regra específica da linguagem.

Se a linguagem consolidar namespaces em uma fase futura, a implementação pode reduzir essa enumeração, preservando diagnósticos equivalentes.

---

## 14. Visibilidade

O símbolo deve preservar informação de visibilidade suficiente para resolução e diagnósticos.

Contrato conceitual:

```rust
pub enum SymbolVisibility {
    Public,
    Private,
    Protected,
    Internal,
    Pending,
}
```

Regras:

- visibilidade ausente ou ainda não validada deve permanecer `Pending`;
- modificadores duplicados ou incompatíveis devem ser diagnosticados pela fase apropriada;
- o modelo de símbolos pode armazenar flags sintáticas, mas não deve decidir regras de acesso baseadas em tipos;
- acesso efetivo a membros pode depender de fases posteriores.

---

## 15. Estado do Símbolo

Símbolos devem carregar estado suficiente para distinguir registro válido, exposição por import, conflito e recuperação.

Contrato conceitual:

```rust
pub enum SymbolState {
    Declared,
    Imported,
    Duplicate { primary: SymbolId },
    Ambiguous,
    Error,
}
```

Regras:

- `Declared` indica símbolo introduzido diretamente no escopo;
- `Imported` indica símbolo disponibilizado por import resolvido;
- `Duplicate` preserva identidade do símbolo conflitante principal;
- `Ambiguous` pode marcar exposição importada ou registrada de forma ambígua, mas não deve ser usado como resolução bem-sucedida de referência;
- `Error` permite continuidade sem representar declaração válida;
- estados de tipo, inicialização ou ownership não pertencem a `SymbolState`.

---

## 16. Tabela de Símbolos

A tabela de símbolos registra símbolos por escopo, namespace e nome.

Contrato conceitual:

```rust
pub struct SymbolTable {
    pub symbols: SymbolArena,
    pub scopes: OrderedMap<ScopeId, ScopeSymbols>,
}

pub struct ScopeSymbols {
    pub scope: ScopeId,
    pub entries: OrderedMap<(Namespace, SymbolName), SymbolSet>,
}

pub struct SymbolSet {
    pub primary: Option<SymbolId>,
    pub conflicts: Vec<SymbolId>,
}
```

Regras:

- a tabela deve permitir consulta por `ScopeId`, `Namespace` e `SymbolName`;
- a tabela deve preservar ordem determinística para dumps e diagnósticos;
- conflitos devem ser armazenados, não descartados;
- um escopo sem declarações deve ser representável;
- todo `ScopeId` presente em `SymbolTable.scopes` deve existir no `ScopeGraph`;
- todo escopo do `ScopeGraph` pode possuir uma entrada `ScopeSymbols`, mesmo vazia;
- consulta não deve modificar a tabela;
- registro de símbolo deve ser separado de resolução de referência.

`OrderedMap` é conceitual. A implementação pode usar `BTreeMap`, vetor ordenado ou `HashMap` com ordenação explícita antes de dumps.

---

## 17. Registro de Declarações

O registro de declarações cria símbolos a partir da HIR.

Entrada conceitual:

```rust
pub struct SymbolRegistrationInput<'a> {
    pub hir: &'a Hir,
    pub scopes: &'a ScopeGraph,
}
```

Saída conceitual:

```rust
pub struct SymbolRegistrationOutput {
    pub table: SymbolTable,
    pub diagnostics: Vec<Diagnostic>,
}
```

Regras:

- registro deve percorrer a HIR em ordem determinística;
- todo elemento declarativo válido deve produzir exatamente uma entrada semântica por entidade declarada;
- o `ScopeGraph` deve estar construído antes do registro de símbolos;
- símbolos devem ser registrados no escopo que contém a declaração;
- parâmetros devem ser registrados no escopo de assinatura ou corpo conforme `SCOPE-MODEL.md`;
- membros devem ser registrados no namespace de membros do tipo proprietário;
- se o escopo de registro de uma declaração não puder ser determinado, isso é erro interno ou erro HIR estrutural explícito;
- parâmetros genéricos devem ser registrados antes do uso em bounds quando o subconjunto suportar generics;
- o registro não deve resolver usos de nomes em expressões;
- o registro não deve inferir tipos.

---

## 18. Duplicidade e Conflitos

Quando duas declarações incompatíveis introduzem o mesmo nome no mesmo escopo e namespace, a tabela deve registrar conflito.

Contrato conceitual:

```rust
pub struct DuplicateSymbol {
    pub name: SymbolName,
    pub namespace: Namespace,
    pub scope: ScopeId,
    pub primary: SymbolId,
    pub duplicate: SymbolId,
}
```

Regras:

- duplicidade no mesmo escopo deve produzir diagnóstico;
- o diagnóstico deve apontar para a declaração duplicada e a declaração anterior;
- nenhuma escolha arbitrária deve ocultar o conflito;
- a tabela pode manter um símbolo primário apenas para continuidade controlada;
- referências ao nome conflitante podem produzir diagnóstico de ambiguidade ou duplicidade conforme `NAME-RESOLUTION.md`;
- overload permitido pela linguagem não deve ser tratado como duplicidade, mas o Stage 3 não deve selecionar overload.

---

## 19. Shadowing

Shadowing ocorre quando declarações com o mesmo nome aparecem em escopos diferentes.

Regras:

- shadowing permitido não é duplicidade;
- símbolo interno possui prioridade sobre símbolo externo durante busca hierárquica;
- símbolo externo preserva identidade e continua disponível onde for visível;
- shadowing proibido pela linguagem deve produzir diagnóstico;
- o modelo de símbolos deve preservar informação suficiente para diagnosticar shadowing proibido;
- regras detalhadas pertencem a `SCOPE-MODEL.md` e `NAME-RESOLUTION.md`.

---

## 20. Imports e Símbolos Externos

Imports disponibilizam símbolos de outros módulos ou pacotes.

Contrato conceitual:

```rust
pub struct ImportBinding {
    pub import: HirImportId,
    pub imported_symbol: Option<SymbolId>,
    pub alias: Option<SymbolName>,
    pub state: ImportBindingState,
}

pub enum ImportBindingState {
    Pending,
    Resolved,
    NotFound,
    Ambiguous,
    Error,
}
```

Regras:

- import permanece pendente até resolução de módulos/imports;
- alias, quando presente, define o nome exposto no escopo importador;
- wildcard import pode disponibilizar múltiplos símbolos, mas essa expansão deve ser determinística;
- conflitos entre imports devem ser diagnosticáveis;
- símbolos externos devem preservar origem e módulo de definição;
- o Stage 3 pode limitar imports ao subconjunto inicial, desde que a limitação esteja documentada em testes e diagnósticos.

---

## 21. Referências Resolvidas

A resolução de nomes conecta usos na HIR a símbolos. O formato autoritativo de queries, candidatos e bindings pertence a `NAME-RESOLUTION.md`.

Regras:

- referência resolvida com sucesso deve apontar para `SymbolId`;
- referência ambígua deve preservar candidatos relevantes;
- referência inexistente deve produzir diagnóstico e permanecer representável;
- `NotFound` não deve ser usado como símbolo sentinela;
- referências inválidas não devem permitir que fases posteriores assumam sucesso;
- HIR enriquecida deve expor resolução por `ResolvedBinding` ou forma equivalente definida por `NAME-RESOLUTION.md`;
- `SymbolTable` não deve decidir sozinha o resultado final de uma referência, porque a decisão depende de `ScopeGraph`, contexto HIR e regras de resolução.

---

## 22. Relação com HIR

A HIR fornece os locais de declaração e os locais de referência. O modelo de símbolos fornece identidade semântica para declarações.

Regras:

- `HirName` preserva nome textual e span;
- `SymbolEntry` preserva identidade semântica de declaração;
- campos `symbol: Option<SymbolId>` na HIR devem permanecer vazios antes do registro/resolução;
- após registro, declarações válidas devem apontar para seus símbolos;
- após resolução, referências válidas devem apontar para símbolos ou entidades resolvidas equivalentes;
- o modelo de símbolos não deve reestruturar a HIR;
- a HIR não deve depender da tabela para traversal estrutural.

---

## 23. Relação com Escopos

Símbolos existem em escopos.

Regras:

- todo símbolo deve possuir `declaring_scope`;
- todo `declaring_scope` deve existir no grafo ou árvore de escopos;
- o escopo define região inicial de visibilidade;
- a tabela de símbolos deve permitir listar símbolos por escopo;
- a resolução deve consultar escopos na ordem definida por `SCOPE-MODEL.md`;
- símbolos não devem criar escopos por si próprios, embora algumas declarações com símbolos também introduzam escopos.

Exemplos:

| Declaração | Símbolo | Escopo onde registra | Pode introduzir novo escopo |
| --- | --- | --- | --- |
| módulo | `Module` | escopo pai ou global | sim |
| classe | `Class` | escopo atual | sim |
| função | `Function` | escopo atual | sim |
| parâmetro | `Param` | escopo da função | não |
| `let` local | `Local` | escopo do bloco | não |
| campo | `Field` | escopo de membros da classe | não |

---

## 24. Relação com Tipos

Símbolos não são tipos.

Regras:

- símbolo de classe, interface, trait ou alias pode declarar uma entidade que será resolvida para tipo;
- `TypeId` pertence ao modelo de tipos;
- `SymbolId` não deve ser usado como substituto de `TypeId`;
- durante Stage 3, referências de tipo podem ser resolvidas para símbolos de categoria compatível;
- transformação de símbolo de tipo em tipo interno pertence ao Stage 4;
- generics podem introduzir símbolos de tipo, mas bounds e substituições pertencem à tipagem.

---

## 25. Diagnósticos

O modelo de símbolos deve fornecer dados suficientes para diagnósticos de resolução.

Categorias iniciais:

- símbolo duplicado;
- declaração conflitante no mesmo namespace;
- shadowing proibido;
- import conflitante;
- referência inexistente;
- referência ambígua;
- categoria incompatível para o contexto de resolução;
- uso de símbolo inválido criado para recuperação.

Regras:

- diagnósticos devem usar `HirOrigin` e spans das declarações/referências;
- duplicidade deve apontar declaração atual e declaração anterior;
- ambiguidade deve listar candidatos relevantes quando possível;
- entrada inválida do usuário não deve virar erro interno;
- erro interno deve ser reservado para violação de invariante da tabela, IDs ausentes ou corrupção de estado.

---

## 26. Dump de Símbolos

Quando houver dump de HIR resolvida ou dump específico de símbolos, a saída deve ser determinística.

Requisitos:

- listar escopos em ordem estável;
- listar símbolos por `SymbolId` ou ordem declarativa estável;
- exibir nome textual, categoria, namespace e escopo;
- exibir local HIR de declaração;
- exibir origem ou span quando habilitado;
- exibir conflitos explicitamente;
- exibir imports pendentes, resolvidos e ambíguos;
- não exibir endereços de memória;
- não depender de ordem de hash map.

Exemplo conceitual:

```text
symbols
  sym0 kind=Function name=main ns=Value scope=scope0 decl=item0 span=0..24
  sym1 kind=Param name=args ns=Value scope=scope1 decl=param0 span=14..18
```

O formato final pertence à implementação, desde que preserve estabilidade e utilidade para testes.

---

## 27. Determinismo

Para a mesma HIR inicial, mesma versão do compilador e mesmas opções:

- a mesma declaração deve receber símbolo equivalente;
- a ordem de alocação deve ser estável;
- conflitos devem ser reportados na mesma ordem;
- imports wildcard devem expandir em ordem estável;
- dumps devem ser determinísticos;
- diagnósticos devem ser determinísticos.

A implementação não deve depender de:

- endereços de memória;
- ordem aleatória de hash;
- paralelismo sem ordenação explícita;
- ordem de leitura não normalizada de diretórios;
- caminhos absolutos não normalizados.

---

## 28. Invariantes

Uma tabela de símbolos válida deve obedecer:

- todo `SymbolId` referenciado existe;
- todo símbolo possui `SymbolKind`;
- todo símbolo possui `declaring_scope` válido;
- todo `ScopeSymbols.scope` aponta para escopo existente no `ScopeGraph`;
- todo símbolo possui `DeclarationSite` válido ou erro explícito;
- todo símbolo nomeado possui `SymbolName`;
- nenhum símbolo válido aponta para elemento HIR inexistente;
- conflitos preservam todos os participantes relevantes;
- shadowing permitido não remove símbolo sombreado;
- resolução bem-sucedida nunca aponta para símbolo em estado `Error`;
- símbolos não carregam tipos inferidos antes da fase de tipos;
- símbolos não dependem de MIR, backend ou ABI.

Violação dessas invariantes por bug da implementação deve produzir erro interno estruturado quando houver caminho de recuperação.

---

## 29. Testes Obrigatórios

Testes do modelo de símbolos no Stage 3 devem cobrir:

- criação de símbolo para função livre;
- criação de símbolo para classe;
- criação de símbolo para interface;
- criação de símbolo para trait;
- criação de símbolo para campo;
- criação de símbolo para método;
- criação de símbolo para construtor;
- criação de símbolo para constante;
- criação de símbolo para parâmetro;
- criação de símbolo para declaração local;
- criação de símbolo para binding de padrão;
- criação de símbolo para parâmetro genérico quando suportado;
- nomes iguais em escopos diferentes;
- duplicidade no mesmo escopo;
- namespaces distintos quando aplicável;
- shadowing permitido;
- shadowing proibido quando aplicável ao subconjunto;
- import com alias;
- import wildcard;
- conflito entre imports;
- referência resolvida para símbolo local;
- referência resolvida para parâmetro;
- referência resolvida para item global;
- referência inexistente;
- referência ambígua;
- preservação de origem e span;
- estabilidade de `SymbolId` dentro de uma execução;
- dump determinístico da tabela de símbolos ou HIR resolvida.

Testes de símbolos não devem exigir inferência de tipos, ownership, Domains, MIR, backend ou ABI.

---

## 30. Critérios de Aceite

Este documento é considerado aprovado para orientar a implementação do Stage 3 quando:

- distingue nome textual internado de símbolo semântico;
- define identidade e categorias de símbolos;
- define entrada de símbolo e local de declaração;
- define relação com HIR e escopos;
- define registro de declarações;
- define tratamento de duplicidade, shadowing, imports e referências;
- define invariantes da tabela de símbolos;
- define requisitos de dump e determinismo;
- define testes obrigatórios.

A implementação correspondente será considerada concluída quando:

- declarações do subconjunto inicial criarem símbolos;
- símbolos possuírem `SymbolId` estável durante a compilação;
- tabelas de símbolos forem construídas por escopo;
- duplicidades forem diagnosticadas;
- referências válidas puderem apontar para símbolos;
- referências inexistentes e ambíguas forem representadas sem associação arbitrária;
- HIR resolvida puder carregar ou consultar os símbolos resultantes;
- dumps relevantes forem determinísticos;
- todos os testes obrigatórios do Stage 3 passarem.

---

## 31. Relações Normativas

Este documento depende diretamente de:

- Documento 04 — Sintaxe da Linguagem;
- Documento 06 — Arquitetura do Compilador;
- Documento 16 — HIR;
- Documento 17 — Resolução de Nomes;
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial;
- `HIR-MODEL.md`;
- `SCOPE-MODEL.md`;
- `AST-LOWERING.md`;
- `SPANS-AND-LOCATIONS.md`;
- `DIAGNOSTIC-DATA-MODEL.md`.

Este documento orienta diretamente:

- `NAME-RESOLUTION.md`;
- `TYPE-MODEL.md`;
- `TYPE-INFERENCE.md`;
- `SEMANTIC-TESTS.md`;
- implementação das tabelas de símbolos;
- implementação de diagnósticos de símbolos duplicados, inexistentes e ambíguos.
