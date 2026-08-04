# Scope Model

**Projeto:** Linguagem Capi  
**Documento:** SCOPE-MODEL  
**Status:** Aprovado  
**Stage:** Stage 3 — HIR e resolução de nomes  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o modelo de engenharia de escopos da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- o que é um escopo semântico;
- quais construções HIR introduzem escopos;
- como escopos se organizam em hierarquia;
- como escopos se relacionam com símbolos;
- como referências encontram o escopo inicial de resolução;
- como fronteiras de visibilidade são preservadas;
- quais dados pertencem ao Stage 3;
- quais dados pertencem a fases posteriores;
- quais invariantes devem ser preservadas;
- quais testes validam o modelo.

Escopos são estruturas auxiliares da análise semântica. Eles não substituem a HIR e não representam tipos, ownership, regiões de memória ou Domains.

---

## 2. Escopo

Este documento cobre:

- identidade de escopos;
- grafo ou árvore de escopos;
- escopo global;
- escopos de módulo;
- escopos de tipo;
- escopos de membros;
- escopos de função, método e construtor;
- escopos de parâmetros;
- escopos de bloco;
- escopos de controle de fluxo;
- escopos de padrões;
- relação entre escopos, HIR e símbolos;
- ordem de busca;
- shadowing;
- imports e visibilidade externa;
- determinismo e dumps;
- testes obrigatórios do Stage 3.

Este documento não cobre:

- algoritmo completo de resolução de nomes;
- tabela de símbolos em detalhe;
- representação final de símbolos;
- inferência ou verificação de tipos;
- overload e seleção de chamadas;
- ownership e borrow checking;
- regiões de memória;
- Domains;
- MIR;
- backend;
- ABI.

Esses temas pertencem a:

- `SYMBOL-MODEL.md`;
- `NAME-RESOLUTION.md`;
- `TYPE-MODEL.md`;
- `TYPE-INFERENCE.md`;
- `OWNERSHIP-MODEL.md`;
- `REGION-ANALYSIS.md`;
- `DOMAIN-IMPLEMENTATION.md`;
- `MIR-MODEL.md`;
- `ABI-IMPLEMENTATION.md`.

---

## 3. Posição no Stage 3

No Stage 3, o modelo de escopos sustenta:

- construção da hierarquia semântica usada pela resolução de nomes;
- definição do escopo onde cada declaração registra seu símbolo;
- definição do escopo inicial de cada referência;
- aplicação de shadowing;
- separação entre escopos globais, locais, de módulos, tipos e membros;
- diagnósticos de duplicidade, referências inexistentes e ambiguidades;
- dump determinístico de HIR resolvida ou de estruturas semânticas auxiliares.

Fluxo conceitual:

```text
HIR inicial
    ↓
Construção do ScopeGraph
    ↓
Associação HIR → ScopeId
    ↓
Registro de símbolos por escopo
    ↓
Busca de nomes a partir de escopos
    ↓
HIR com referências resolvidas
```

---

## 4. Princípios

O modelo de escopos deve seguir estes princípios:

- escopo representa região de visibilidade, não bloco textual genérico;
- todo escopo deve possuir identidade própria;
- todo escopo, exceto o global, deve possuir pai semântico;
- a hierarquia deve ser determinística;
- escopos devem apontar para origem HIR quando aplicável;
- símbolos são registrados em escopos, mas escopos não são símbolos;
- uma construção pode introduzir símbolo, escopo ou ambos;
- shadowing permitido não remove símbolos de escopos externos;
- busca de nomes deve respeitar fronteiras de escopo;
- escopos não devem inferir tipos;
- escopos não devem depender de MIR, backend, ABI ou layout de objetos.

---

## 5. Terminologia

| Termo | Significado |
| --- | --- |
| Escopo | Região semântica em que declarações podem estar visíveis. |
| `ScopeId` | Identidade opaca de um escopo. |
| Escopo pai | Escopo imediatamente externo na hierarquia. |
| Escopo filho | Escopo diretamente contido por outro escopo. |
| Proprietário de escopo | Elemento HIR que introduz ou representa o escopo. |
| Escopo léxico | Escopo determinado pela estrutura textual/HIR do programa. |
| Escopo de membros | Escopo que contém campos, métodos, construtores e membros de tipo. |
| Escopo de resolução | Escopo inicial usado para resolver uma referência específica. |
| Fronteira de escopo | Limite que controla quais símbolos externos permanecem visíveis. |

---

## 6. Entidades Conceituais

| Entidade | Responsabilidade |
| --- | --- |
| `ScopeId` | Identidade opaca de escopo. |
| `ScopeGraph` | Estrutura que armazena escopos e relações pai-filho. |
| `ScopeData` | Registro de um escopo. |
| `ScopeKind` | Categoria semântica do escopo. |
| `ScopeOwner` | Elemento HIR que originou ou representa o escopo. |
| `ScopeRegion` | Região de origem e validade do escopo. |
| `ScopeLookupPolicy` | Política de busca aplicável a partir do escopo. |
| `ScopeFlags` | Propriedades auxiliares do escopo. |
| `HirScopeMap` | Mapeamento entre elementos HIR e escopos. |

Os nomes concretos podem variar na implementação. O contrato obrigatório é a presença dessas responsabilidades.

---

## 7. Identidade de Escopos

Cada escopo deve possuir identificador próprio.

Contrato conceitual:

```rust
pub struct ScopeId(u32);
```

Regras:

- `ScopeId` deve ser único dentro do `ScopeGraph`;
- `ScopeId` deve ser opaco;
- `ScopeId` não deve codificar tipo de escopo, posição, nome ou profundidade;
- `ScopeId` não deve depender de endereço de memória;
- `ScopeId` deve permanecer estável durante as fases que usam o mesmo `ScopeGraph`;
- IDs devem ser atribuídos em ordem determinística;
- APIs internas devem usar `ScopeId` em vez de índices crus.

---

## 8. Grafo de Escopos

O `ScopeGraph` representa a hierarquia de escopos.

Contrato conceitual:

```rust
pub struct ScopeGraph {
    pub root: ScopeId,
    pub scopes: ScopeArena,
    pub hir_map: HirScopeMap,
}

pub struct ScopeData {
    pub id: ScopeId,
    pub kind: ScopeKind,
    pub owner: ScopeOwner,
    pub parent: Option<ScopeId>,
    pub children: Vec<ScopeId>,
    pub region: ScopeRegion,
    pub lookup: ScopeLookupPolicy,
    pub flags: ScopeFlags,
}
```

Regras:

- `root` deve apontar para o escopo global;
- apenas o escopo global deve ter `parent: None`;
- todo filho deve apontar de volta para seu pai;
- a lista de filhos deve ser determinística;
- o grafo deve ser acíclico;
- traversal completo deve ser possível sem consultar a AST;
- a HIR pode ser consultada para origem e estrutura, mas não deve ser modificada pela construção do grafo.

Embora o termo usado seja "grafo", a estrutura inicial deve formar uma árvore de escopos léxicos. A implementação pode adicionar arestas auxiliares para imports, prelúdio ou relações de membros, desde que elas sejam distinguíveis das arestas pai-filho.

---

## 9. Categorias de Escopo

Contrato conceitual:

```rust
pub enum ScopeKind {
    Global,
    Module,
    Import,
    Type,
    Member,
    Function,
    Constructor,
    Parameter,
    Block,
    ControlFlow,
    Pattern,
    GenericParam,
    Error,
}
```

Regras:

- `Global` representa a raiz da compilação ou pacote analisado;
- `Module` representa região de nomes de módulo;
- `Import` é opcional e pode representar visibilidade introduzida por import agrupado;
- `Type` representa corpo semântico de classe, interface ou trait;
- `Member` representa namespace de campos, métodos e construtores;
- `Function` representa corpo de função ou método;
- `Constructor` representa corpo de construtor quando a implementação quiser distingui-lo;
- `Parameter` representa região de parâmetros quando separada do corpo;
- `Block` representa bloco lexical;
- `ControlFlow` representa escopo introduzido por `for`, `match`, `switch` ou construção equivalente;
- `Pattern` representa bindings introduzidos por padrões;
- `GenericParam` representa escopo de parâmetros genéricos quando necessário;
- `Error` permite recuperação controlada.

Categorias podem ser refinadas, desde que a ordem de busca e os diagnósticos permaneçam equivalentes.

---

## 10. Proprietário de Escopo

Cada escopo deve indicar o elemento HIR que o originou ou uma origem sintética explícita.

Contrato conceitual:

```rust
pub enum ScopeOwner {
    Global,
    Unit(HirUnitId),
    Item(HirItemId),
    Import(HirImportId),
    Member(HirMemberId),
    Function(HirItemId),
    Constructor(HirMemberId),
    Param(HirParamId),
    Local(HirLocalId),
    Block(HirBlockId),
    Stmt(HirStmtId),
    Expr(HirExprId),
    Pattern(HirPatternId),
    Synthetic,
    Error(HirErrorId),
}
```

Regras:

- escopos de módulo devem apontar para unidade ou item de módulo correspondente;
- escopos de tipo devem apontar para item de classe, interface ou trait;
- escopos de membros devem apontar para o tipo proprietário;
- escopos de função livre devem apontar para item de função;
- escopos de método devem apontar para membro de método;
- escopos de bloco devem apontar para `HirBlockId`;
- escopos sintéticos devem possuir região e origem rastreável;
- `Error` não deve ocultar violação interna de invariantes.

---

## 11. Região de Escopo

A região de escopo preserva rastreabilidade para diagnósticos e ferramentas.

Contrato conceitual:

```rust
pub struct ScopeRegion {
    pub origin: HirOrigin,
    pub span: Span,
}
```

Regras:

- `span` deve cobrir a região principal de visibilidade quando aplicável;
- escopos sintéticos devem usar span representativo;
- escopo global pode usar span sintético ou span cobrindo a unidade principal;
- região de escopo não substitui as regras formais de visibilidade;
- diagnósticos devem preferir spans das declarações/referências quando forem mais precisos.

---

## 12. Escopo Global

O escopo global é a raiz da hierarquia.

Regras:

- deve existir exatamente um escopo global por `ScopeGraph`;
- deve possuir `ScopeKind::Global`;
- deve ter `parent: None`;
- deve conter módulos e itens globais conforme o modelo de compilação;
- pode expor prelúdio ou biblioteca padrão por arestas auxiliares, não por filhos léxicos indistintos;
- não deve depender de ordem não determinística de arquivos.

Quando a compilação envolver múltiplas unidades, a implementação deve definir uma ordem estável para registrar unidades no escopo global.

---

## 13. Escopos de Módulo e Imports

Módulos introduzem regiões de nomes.

Regras:

- módulo explícito deve possuir escopo de módulo ou associação equivalente;
- módulo implícito deve ser representado de forma distinguível;
- itens do módulo são registrados no escopo do módulo;
- imports são visíveis no escopo definido pela linguagem;
- imports não devem ser resolvidos durante construção inicial de escopos;
- wildcard import não deve criar filhos léxicos para cada símbolo importado nessa etapa;
- conflitos de imports pertencem à resolução de nomes, mas o escopo deve preservar local de exposição.

Escopos de import são opcionais. A implementação pode representar imports como entradas associadas ao escopo importador, desde que a busca e os diagnósticos sejam equivalentes.

---

## 14. Escopos de Tipos e Membros

Classes, interfaces e traits podem introduzir escopos distintos para o corpo do tipo e para membros.

Regras:

- declaração de tipo registra símbolo no escopo externo;
- corpo do tipo deve possuir escopo próprio ou estrutura equivalente;
- membros devem ser registrados em escopo de membros ou namespace `Member` associado ao tipo;
- parâmetros genéricos do tipo devem ficar visíveis nas assinaturas e membros conforme a linguagem;
- herança, implementação de interfaces e composição de traits não são resolvidas pelo modelo de escopos inicial;
- acesso a membros por expressão pertence à resolução de nomes e tipagem posterior.

Separar `Type` e `Member` é recomendado quando simplificar diagnóstico e lookup. A implementação pode usar um único escopo com namespaces distintos se preservar equivalência funcional.

---

## 15. Escopos de Funções, Métodos e Construtores

Funções, métodos e construtores introduzem escopos para parâmetros e corpo.

Regras:

- a declaração da função/método/construtor é registrada no escopo externo apropriado;
- parâmetros devem ser visíveis no corpo;
- parâmetros podem estar em escopo próprio `Parameter` ou diretamente no escopo da função;
- o corpo deve possuir escopo de função, construtor ou bloco associado;
- declarações locais do corpo são registradas nos blocos onde aparecem;
- retorno, `break` e `continue` não introduzem símbolos;
- `this`, quando suportado, pode ser símbolo sintético ou referência especial, conforme `NAME-RESOLUTION.md`.

O contrato com `SYMBOL-MODEL.md` é:

```text
símbolo da função → escopo externo
símbolos de parâmetros → escopo de função/parâmetros
símbolos locais → escopo de bloco
```

---

## 16. Escopos de Bloco

Blocos HIR introduzem escopos lexicais quando a linguagem define visibilidade local por bloco.

Regras:

- cada bloco que introduz região lexical deve possuir `ScopeId`;
- blocos que não introduzem novo escopo devem mapear para escopo herdado de forma explícita;
- declarações locais são registradas no escopo do bloco correspondente;
- blocos aninhados devem ter pai igual ao escopo lexical imediatamente externo;
- bloco implícito criado por normalização deve preservar origem representativa;
- ordem de comandos deve ser preservada para diagnósticos e regras de disponibilidade.

Se a linguagem permitir uso antes da declaração em certos contextos, essa regra deve ser modelada pela resolução de nomes, não por reordenação acidental do escopo.

---

## 17. Escopos de Controle de Fluxo

Construções de controle podem introduzir escopos próprios.

Regras:

- `if` pode introduzir escopos para blocos de `then` e `else`;
- `while` pode introduzir escopo para corpo;
- `for` pode introduzir escopo para inicializador, variável de iteração e corpo;
- `switch` pode introduzir escopo para casos, conforme a linguagem;
- `match` pode introduzir escopo por braço para bindings de padrão;
- bindings de pattern devem ser visíveis apenas na região definida pela linguagem;
- escopos de controle não devem validar condição booleana, exaustividade ou alcançabilidade.

Detalhes de tipos de condições, exaustividade de `match` e fluxo pertencem a fases posteriores.

---

## 18. Escopos de Padrões

Padrões podem introduzir bindings.

Regras:

- binding de pattern deve produzir símbolo `PatternBinding`;
- o símbolo deve ser registrado no escopo onde a linguagem torna o binding visível;
- múltiplos bindings no mesmo padrão devem preservar ordem determinística;
- duplicidades dentro do mesmo padrão devem ser diagnosticáveis;
- a compatibilidade entre padrão e tipo analisado não pertence ao Stage 3;
- exaustividade não pertence ao Stage 3.

Escopo de padrão separado é opcional quando o escopo do braço de `match` ou construção equivalente for suficiente.

---

## 19. Parâmetros Genéricos

Parâmetros genéricos introduzem nomes no namespace de tipos.

Regras:

- parâmetro genérico deve possuir símbolo quando suportado pelo subconjunto;
- deve ser registrado em escopo visível para bounds, assinatura e corpo aplicável;
- conflitos entre parâmetros genéricos no mesmo escopo devem ser diagnosticáveis;
- parâmetros genéricos não devem virar `TypeId` no Stage 3;
- substituição, variância e constraints pertencem ao Stage 4.

Escopo `GenericParam` é opcional. A implementação pode registrar parâmetros genéricos no escopo do item proprietário se a visibilidade resultante for equivalente.

---

## 20. Mapeamento HIR-Escopo

A construção de escopos deve produzir mapeamento entre HIR e escopos.

Contrato conceitual:

```rust
pub struct HirScopeMap {
    pub owner_to_scope: OrderedMap<ScopeOwner, ScopeId>,
    pub hir_to_enclosing_scope: OrderedMap<HirId, ScopeId>,
}
```

Regras:

- todo elemento HIR que introduz escopo deve aparecer em `owner_to_scope`;
- toda declaração que introduz símbolo deve ter escopo de registro determinável;
- toda referência que exige resolução deve ter escopo inicial determinável;
- elementos que não introduzem escopo podem mapear para escopo envolvente;
- mapas devem ser determinísticos para dumps e testes;
- ausência de escopo para elemento que exige resolução é erro interno.

---

## 21. Política de Lookup

Cada escopo pode declarar política de busca para resolução.

Contrato conceitual:

```rust
pub struct ScopeLookupPolicy {
    pub allow_parent_lookup: bool,
    pub include_imports: bool,
    pub include_prelude: bool,
    pub lookup_members_through_receiver: bool,
}
```

Regras:

- busca local deve ocorrer antes de busca em ancestrais;
- imports devem ser consultados em ponto definido pela linguagem;
- prelúdio, se existir, deve ser distinguível de declarações do usuário;
- membros de tipo não devem ser confundidos com variáveis locais;
- lookup de membro por receiver pertence a resolução de nomes/tipagem, mas o escopo de membros deve estar disponível;
- `ScopeLookupPolicy` descreve capacidades e limites do escopo; `ResolutionQuery`, candidatos e bindings pertencem a `NAME-RESOLUTION.md`;
- política não deve depender de tipo inferido no Stage 3.

---

## 22. Flags de Escopo

Flags preservam propriedades auxiliares do escopo sem substituir `ScopeKind`.

Contrato conceitual:

```rust
pub struct ScopeFlags {
    pub synthetic: bool,
    pub contains_error: bool,
    pub allows_shadowing: bool,
    pub accepts_imports: bool,
}
```

Regras:

- `synthetic` deve marcar escopos criados por normalização ou conveniência interna;
- `contains_error` deve marcar escopos parciais ou derivados de HIR inválida;
- `allows_shadowing` pode registrar regra local quando ela for conhecida sem tipagem;
- `accepts_imports` indica que imports podem expor nomes nesse escopo;
- flags não devem codificar resultado de inferência de tipos;
- flags não substituem diagnósticos nem regras de `NAME-RESOLUTION.md`.

---

## 23. Ordem de Busca

Ordem conceitual para referência simples:

```text
1. escopo atual
2. escopos ancestrais, do mais interno ao mais externo
3. imports visíveis, na ordem definida pela linguagem
4. prelúdio ou biblioteca padrão automática, se aplicável
5. erro de referência inexistente
```

Regras:

- a busca deve ser determinística;
- símbolo encontrado em escopo mais interno tem prioridade sobre símbolos externos quando shadowing for permitido;
- múltiplos candidatos no mesmo nível lógico podem produzir ambiguidade;
- nenhuma escolha arbitrária deve ocorrer quando houver ambiguidade;
- nomes qualificados podem alterar a ordem de busca conforme `NAME-RESOLUTION.md`.

---

## 24. Shadowing

O `ScopeGraph` deve permitir identificar shadowing.

Regras:

- declaração em escopo interno pode sombrear declaração externa quando a linguagem permitir;
- shadowing permitido não altera nem remove símbolo externo;
- shadowing proibido deve produzir diagnóstico;
- duplicidade no mesmo escopo não é shadowing;
- shadowing deve considerar namespace;
- diagnósticos devem apontar declaração sombreante e declaração sombreada quando útil.

O modelo de escopos fornece a hierarquia; a decisão final de permitido/proibido pertence à resolução de nomes conforme regras da linguagem.

---

## 25. Fronteiras de Escopo

Fronteiras controlam quais símbolos são visíveis ao atravessar a hierarquia.

Regras:

- função cria fronteira para declarações locais internas;
- bloco cria fronteira lexical quando a linguagem define visibilidade por bloco;
- tipo cria fronteira entre membros e nomes locais;
- módulo cria fronteira de organização externa;
- imports atravessam fronteiras apenas pelas regras de visibilidade externa;
- membros privados/protegidos não devem ser expostos por lookup simples fora de contexto autorizado.

Validação completa de acesso a membros pode depender de tipo e herança, portanto não precisa ser concluída no Stage 3.

---

## 26. Construção do ScopeGraph

Entrada conceitual:

```rust
pub struct ScopeBuildInput<'a> {
    pub hir: &'a Hir,
}
```

Saída conceitual:

```rust
pub struct ScopeBuildOutput {
    pub graph: ScopeGraph,
    pub diagnostics: Vec<Diagnostic>,
}
```

Regras:

- construção deve percorrer a HIR em ordem determinística;
- construção não deve registrar símbolos como efeito obrigatório;
- construção não deve resolver referências;
- construção não deve inferir tipos;
- HIR inválida pode bloquear construção completa ou produzir escopos marcados como erro;
- diagnósticos desta etapa devem se limitar a inconsistências estruturais de escopo;
- ausência de escopo esperado por bug interno deve ser erro interno estruturado quando recuperável.

---

## 27. Relação com SYMBOL-MODEL

O contrato entre escopos e símbolos é:

- `ScopeGraph` define `ScopeId`;
- `SymbolEntry.declaring_scope` deve apontar para `ScopeId` existente;
- `SymbolTable` organiza símbolos por `ScopeId`, `Namespace` e nome textual;
- duplicidade é verificada dentro do mesmo `ScopeId` e `Namespace`;
- shadowing é verificado entre `ScopeId` ancestral e descendente;
- símbolos não criam escopos automaticamente;
- escopos não criam símbolos automaticamente.

Uma construção HIR pode gerar ambos. Exemplo: uma função introduz um símbolo no escopo externo e introduz escopo próprio para seus parâmetros e corpo.

---

## 28. Relação com HIR-MODEL

O contrato entre escopos e HIR é:

- HIR fornece a estrutura a ser percorrida;
- HIR preserva origem e spans usados pelo escopo;
- escopos podem ser associados a blocos por `HirBlock.scope: Option<ScopeId>`;
- declarações HIR precisam ter escopo de registro determinável;
- referências HIR precisam ter escopo inicial determinável;
- HIR inicial não deve conter escopos finais antes da construção do `ScopeGraph`;
- enriquecimento com escopos não deve modificar a identidade dos elementos HIR.

---

## 29. Relação com NAME-RESOLUTION

A resolução de nomes usa o `ScopeGraph` para:

- localizar escopo inicial de cada referência;
- buscar símbolos locais;
- caminhar por ancestrais;
- aplicar shadowing;
- consultar imports;
- detectar referência inexistente;
- detectar ambiguidade;
- anexar resultado de resolução à HIR.

`SCOPE-MODEL.md` não define o algoritmo completo de resolução. Ele define a estrutura e as invariantes que o algoritmo deve usar.

---

## 30. Diagnósticos

O modelo de escopos deve fornecer dados para diagnósticos como:

- declaração fora de região permitida;
- duplicidade decorrente de escopo incorreto;
- shadowing proibido;
- referência fora de escopo;
- import visível de forma ambígua;
- construção HIR que exigia escopo mas não recebeu um;
- ciclo ou pai ausente no `ScopeGraph`.

Regras:

- erros causados por programa inválido devem ser diagnósticos de usuário;
- falhas de invariantes internas devem ser erros internos estruturados;
- diagnósticos devem preferir spans de declarações e referências;
- diagnóstico de escopo deve preservar contexto suficiente para correção.

---

## 31. Dump de Escopos

Quando houver dump de HIR resolvida ou dump específico de escopos, a saída deve ser determinística.

Requisitos:

- listar `ScopeId`, `ScopeKind`, pai e proprietário;
- listar filhos em ordem estável;
- listar região ou span quando habilitado;
- listar símbolos associados quando integrado ao dump de símbolos;
- distinguir arestas pai-filho de arestas auxiliares de import/prelúdio;
- não exibir endereços de memória;
- não depender de ordem de hash map.

Exemplo conceitual:

```text
scopes
  scope0 kind=Global parent=<none> owner=global
    child scope1
  scope1 kind=Module parent=scope0 owner=unit0 span=0..24
    child scope2
  scope2 kind=Function parent=scope1 owner=item0 span=0..24
```

O formato final pertence à implementação.

---

## 32. Determinismo

Para a mesma HIR inicial, mesma versão do compilador e mesmas opções:

- os mesmos escopos devem ser criados;
- a hierarquia deve ser equivalente;
- IDs devem ser atribuídos em ordem determinística;
- filhos devem ser listados em ordem estável;
- mapeamento HIR-escopo deve ser determinístico;
- diagnósticos devem aparecer em ordem estável;
- dumps devem ser determinísticos.

A implementação não deve depender de:

- endereços de memória;
- ordem aleatória de hash;
- paralelismo sem ordenação explícita;
- ordem não normalizada de arquivos;
- locale do sistema.

---

## 33. Invariantes

Um `ScopeGraph` válido deve obedecer:

- existe exatamente um escopo global;
- `root` aponta para escopo global;
- escopo global não possui pai;
- todo outro escopo possui pai válido;
- toda relação pai-filho é recíproca;
- não existem ciclos na hierarquia lexical;
- todo `ScopeId` referenciado existe;
- todo `ScopeData` possui `ScopeKind`;
- todo escopo possui `ScopeOwner`;
- todo escopo possui região rastreável;
- todo elemento HIR que exige escopo possui mapeamento;
- todo elemento HIR que exige resolução possui escopo inicial determinável;
- símbolos registrados em escopos apontam para escopos existentes;
- arestas auxiliares não são confundidas com pai lexical;
- escopos não carregam tipos inferidos, ownership, regiões ou MIR.

Violação dessas invariantes por bug da implementação deve produzir erro interno estruturado quando houver caminho de recuperação.

---

## 34. Testes Obrigatórios

Testes do modelo de escopos no Stage 3 devem cobrir:

- criação de escopo global;
- unidade com módulo implícito;
- unidade com módulo explícito;
- imports associados ao escopo correto;
- função livre criando símbolo externo e escopo interno;
- parâmetros visíveis no corpo de função;
- bloco vazio;
- bloco aninhado;
- declaração local visível no bloco correto;
- declaração local fora de escopo;
- shadowing permitido;
- duplicidade no mesmo escopo;
- classe com escopo de tipo;
- classe com escopo de membros;
- método com escopo próprio;
- construtor com escopo próprio;
- campo registrado no escopo de membros;
- interface;
- trait;
- parâmetros genéricos quando suportados;
- `if` com blocos separados;
- `while` com corpo;
- `for` com variável local ou inicializador;
- `match` com bindings por braço;
- padrões com bindings;
- escopo de import wildcard quando suportado;
- mapeamento HIR-escopo;
- determinismo de `ScopeId`;
- dump determinístico de escopos;
- HIR parcial com escopo de erro, se suportada.

Testes de escopos não devem exigir inferência de tipos, ownership, Domains, MIR, backend ou ABI.

---

## 35. Critérios de Aceite

Este documento é considerado aprovado para orientar a implementação do Stage 3 quando:

- define `ScopeId` e `ScopeGraph`;
- define categorias e proprietários de escopo;
- define construção de escopos a partir da HIR;
- define mapeamento HIR-escopo;
- define relação com `SYMBOL-MODEL.md`;
- define relação com `NAME-RESOLUTION.md`;
- define ordem conceitual de busca;
- define shadowing, fronteiras e imports no nível de escopos;
- define invariantes, dumps e testes obrigatórios.

A implementação correspondente será considerada concluída quando:

- HIR inicial produzir `ScopeGraph`;
- todo elemento declarativo tiver escopo de registro;
- toda referência resolvível tiver escopo inicial;
- símbolos puderem ser registrados por `ScopeId`;
- shadowing e duplicidade puderem ser diagnosticados;
- dumps de escopos forem determinísticos;
- todos os testes obrigatórios do Stage 3 passarem.

---

## 36. Relações Normativas

Este documento depende diretamente de:

- Documento 04 — Sintaxe da Linguagem;
- Documento 06 — Arquitetura do Compilador;
- Documento 16 — HIR;
- Documento 17 — Resolução de Nomes;
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial;
- `HIR-MODEL.md`;
- `SYMBOL-MODEL.md`;
- `AST-LOWERING.md`;
- `SPANS-AND-LOCATIONS.md`;
- `DIAGNOSTIC-DATA-MODEL.md`.

Este documento orienta diretamente:

- `NAME-RESOLUTION.md`;
- `SEMANTIC-TESTS.md`;
- implementação do `ScopeGraph`;
- implementação do mapeamento HIR-escopo;
- implementação do registro de símbolos por escopo;
- implementação de diagnósticos de duplicidade, shadowing, referência inexistente e ambiguidade.
