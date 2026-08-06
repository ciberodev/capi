# Type Model

**Projeto:** Linguagem Capi  
**Documento:** TYPE-MODEL  
**Status:** Aprovado  
**Stage:** Stage 4 — Sistema de tipos  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o modelo de engenharia de tipos da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- o que é um tipo interno do compilador;
- quais categorias de tipos devem ser representadas;
- como tipos se relacionam com símbolos, HIR e referências de tipo;
- quais IDs e handles devem ser usados para identidade de tipos;
- quais propriedades semânticas mínimas cada tipo deve expor;
- como tipos incompletos, desconhecidos e inválidos são representados;
- quais dados pertencem ao Stage 4;
- quais dados pertencem a documentos posteriores;
- quais invariantes devem ser preservadas;
- quais testes validam o modelo.

Neste documento, "tipo" significa uma entidade semântica determinada pelo sistema de tipos da linguagem e representada internamente por `TypeId`, `TyId`, `TypeRef` ou mecanismo equivalente. Isso não deve ser confundido com uma referência sintática de tipo na HIR, que pode ainda estar pendente ou não resolvida.

---

## 2. Escopo

Este documento cobre:

- identidade de tipos;
- categorias fundamentais de tipos;
- tipos primitivos;
- `Unit`;
- tipos por valor;
- tipos por identidade;
- tipos de Domain;
- `ObjectId<T>`;
- tipos opcionais e de resultado como construções genéricas do subconjunto inicial;
- tipos de função, método, construtor e assinatura;
- parâmetros de tipo e aplicações genéricas;
- tipos desconhecidos, de erro e parcialmente determinados;
- propriedades semânticas de tipos;
- associação entre tipos e símbolos;
- associação entre tipos e elementos HIR;
- determinismo de representação e dumps;
- interface com interning, inferência, verificação, subtipagem e coerções.

Este documento não cobre:

- algoritmo de inferência;
- pipeline completo de verificação de tipos;
- implementação de interning;
- regras detalhadas de subtipagem e coerção;
- seleção final de overload;
- implementação completa de generics;
- ownership e borrow checking;
- análise de regiões;
- validação operacional de Domains;
- layout de objetos;
- vtables e despacho dinâmico;
- lowering para MIR;
- ABI e geração de código.

Esses temas pertencem a:

- `TYPE-INTERNING.md`;
- `TYPE-INFERENCE.md`;
- `TYPE-CHECKING-PIPELINE.md`;
- `SUBTYPING-AND-COERCIONS.md`;
- `GENERICS-IMPLEMENTATION.md`;
- `OWNERSHIP-MODEL.md`;
- `REGION-ANALYSIS.md`;
- `DOMAIN-IMPLEMENTATION.md`;
- `OBJECT-MODEL.md`;
- `OBJECT-LAYOUT.md`;
- `MIR-LOWERING.md`;
- `ABI-IMPLEMENTATION.md`.

---

## 3. Posição no Stage 4

No Stage 4, o modelo de tipos sustenta:

- representação interna dos tipos da linguagem;
- interning e comparação canônica de tipos;
- conversão de referências de tipo resolvidas para entidades tipadas;
- enriquecimento da HIR com tipos;
- inferência de tipos para elementos sem anotação explícita;
- verificação de compatibilidade;
- subtipagem e coerções aplicáveis ao subconjunto inicial;
- resolução de chamadas baseada em assinaturas tipadas;
- diagnósticos de inconsistência de tipos;
- execução de `capic check arquivo.capi`.

Fluxo conceitual:

```text
HIR com nomes resolvidos
    ↓
Coleta de declarações de tipo
    ↓
Construção de TypeKind
    ↓
Interning e criação de TypeId
    ↓
Inferência e restrições
    ↓
Verificação de compatibilidade
    ↓
HIR tipada
```

O modelo de tipos não substitui HIR, símbolos ou escopos. Ele define as entidades tipadas que essas estruturas passam a referenciar durante a análise semântica.

---

## 4. Princípios

O modelo de tipos deve seguir estes princípios:

- todo elemento tipável válido deve possuir exatamente um tipo estático ao fim do Stage 4;
- referências sintáticas de tipo devem ser distinguidas de tipos semânticos;
- tipos devem possuir identidade opaca e determinística;
- tipos equivalentes devem poder ser comparados sem depender de texto fonte;
- tipos não devem depender de endereço de memória;
- tipos não devem codificar layout físico, ABI ou detalhes de backend;
- tipos devem preservar rastreabilidade para símbolos e HIR quando aplicável;
- tipos por identidade devem preservar a distinção entre classe e `ObjectId<T>`;
- `ObjectId<T>` não deve ser modelado como ponteiro, endereço ou ownership;
- `Unit` deve ser modelado como tipo válido;
- ausência de valor deve ser modelada por `Optional<T>` ou construção equivalente, não por `null`;
- erro de tipo deve ser representável para permitir recuperação controlada;
- inferência não deve resolver nomes novamente;
- verificação de tipos não deve antecipar ownership, regiões, layout físico ou geração de código.

---

## 5. Terminologia

| Termo | Significado |
| --- | --- |
| Tipo semântico | Entidade interna que representa um tipo já conhecido pelo compilador. |
| Referência de tipo | Elemento HIR que aponta para um nome de tipo escrito no programa. |
| `TypeId` | Identidade opaca de um tipo interno. |
| `TypeKind` | Forma estrutural ou categoria principal de um tipo. |
| Tipo nominal | Tipo cuja identidade depende de declaração nomeada, como classe, interface, trait ou Domain. |
| Tipo construído | Tipo formado a partir de outro tipo, como `ObjectId<T>` ou aplicação genérica. |
| Tipo desconhecido | Marcador temporário para tipo ainda não determinado. |
| Tipo de erro | Marcador usado após diagnóstico para manter a análise em andamento. |
| Propriedade de tipo | Atributo semântico como copiável, identidade, herdável ou persistente. |
| Assinatura | Tipo ou estrutura tipada associada a função, método ou construtor. |

Regras:

- `HirTypeRefId` representa referência de tipo na HIR;
- `SymbolId` representa declaração resolvida;
- `TypeId` representa tipo semântico;
- APIs devem evitar usar o mesmo nome para texto internado, símbolo e tipo semântico.

---

## 6. Entidades Conceituais

| Entidade | Responsabilidade |
| --- | --- |
| `TypeId` | Identidade opaca de um tipo semântico. |
| `TypeKind` | Categoria e estrutura de um tipo. |
| `TypeInterner` | Autoridade de canonicalização e reutilização de tipos. |
| `TypeTable` | Tabela de tipos associados a símbolos e elementos HIR. |
| `TypeInfo` | Registro canônico de um tipo interno. |
| `TypeOrigin` | Origem normativa, declarativa ou inferida do tipo. |
| `TypeProperties` | Propriedades semânticas consultáveis. |
| `TypeCategory` | Categoria fundamental definida pelo Documento 02. |
| `TypeRefBinding` | Associação entre `HirTypeRefId`, `SymbolId` e `TypeId`. |
| `TypedHirMap` | Mapeamento entre elementos HIR tipáveis e `TypeId`. |
| `FunctionSignature` | Tipos de parâmetros, retorno e propriedades de chamada. |
| `GenericParamId` | Identidade de parâmetro genérico. |
| `TypeArgument` | Argumento usado em aplicação genérica. |
| `TypeError` | Representação estruturada de erro de tipo recuperável. |

Os nomes concretos podem variar na implementação. O contrato obrigatório é a presença dessas responsabilidades.

---

## 7. Identidade de Tipos

Cada tipo semântico deve possuir identificador próprio.

Contrato conceitual:

```rust
pub struct TypeId(u32);
```

Regras:

- `TypeId` deve ser opaco;
- `TypeId` não deve codificar nome, categoria, escopo, layout ou posição;
- `TypeId` não deve depender de endereço de memória;
- `TypeId` deve permanecer estável durante a sessão semântica que usa a mesma HIR;
- a atribuição de IDs deve ser determinística;
- tipos canonicamente iguais devem apontar para o mesmo `TypeId` quando interning estiver ativo;
- tipos semanticamente distintos não devem compartilhar `TypeId`;
- APIs internas devem preferir `TypeId` a índices crus.

IDs podem ser alocados por arena, interner, índice sequencial ou mecanismo equivalente, desde que as invariantes acima sejam preservadas.

---

## 8. Forma dos Tipos

A forma interna de um tipo deve representar sua categoria semântica sem misturar responsabilidades de fases posteriores.

Contrato conceitual:

```rust
pub enum TypeKind {
    Primitive(PrimitiveType),
    Unit,
    Value(ValueType),
    Identity(IdentityType),
    Domain(DomainType),
    ObjectId(TypeId),
    Function(FunctionSignature),
    GenericParam(GenericParamId),
    GenericInstance(GenericInstance),
    Unknown(UnknownType),
    Error(TypeError),
}
```

Regras:

- `TypeKind` deve ser suficiente para decisões de tipagem do Stage 4;
- layout físico não deve fazer parte de `TypeKind`;
- ABI, alinhamento, offsets e vtables pertencem a documentos de objeto, MIR, ABI e codegen;
- `Unknown` deve ser temporário ou restrito a recuperação;
- `Error` deve impedir cascatas de diagnósticos redundantes;
- tipos compostos devem referenciar outros tipos por `TypeId`;
- tipos nominais devem manter vínculo com o `SymbolId` da declaração.

---

## 9. Categorias Fundamentais

Todo tipo final válido deve pertencer exatamente a uma categoria fundamental do Documento 02:

| Categoria | Responsabilidade |
| --- | --- |
| Primitivo | Valor elementar fornecido pela linguagem. |
| Por valor | Agregação copiável sem identidade própria. |
| Por identidade | Tipo de objeto com identidade, herança e polimorfismo. |
| Domain | Unidade de gerenciamento de memória e lifetime. |

Regras:

- um tipo não deve pertencer simultaneamente a mais de uma categoria fundamental;
- `Unit` deve ser representado como tipo especial válido;
- `ObjectId<T>` deve ser representado como tipo construído associado a tipo por identidade;
- interfaces e traits participam do sistema de tipos como contratos, sem definir estado persistente;
- parâmetros genéricos não são categoria fundamental final, mas placeholders usados para formar tipos válidos.

---

## 10. Tipos Primitivos e Unit

Tipos primitivos representam valores elementares da linguagem.

Contrato conceitual:

```rust
pub enum PrimitiveType {
    Bool,
    Char,
    Int,
    UInt,
    Float,
    Double,
}
```

Regras:

- tipos primitivos devem possuir tamanho conhecido em tempo de compilação conceitual;
- tipos primitivos não possuem identidade;
- tipos primitivos não pertencem a Domains;
- tipos primitivos não participam de hierarquias de classes;
- cópia de primitivos deve ser tratada como cópia por valor;
- `Unit` deve ser tipo próprio, não ausência de tipo;
- funções sem retorno explícito devem ter retorno `Unit` quando a linguagem permitir.

Detalhes de tamanho físico, alinhamento e representação binária pertencem a `DATA-LAYOUT.md`, `ABI-IMPLEMENTATION.md` e documentos de backend.

---

## 11. Tipos por Valor

Tipos por valor representam agregações copiáveis sem identidade própria.

Contrato conceitual:

```rust
pub enum ValueType {
    Struct(SymbolId),
    Tuple(Vec<TypeId>),
    Enum(SymbolId),
}
```

Regras:

- tipos por valor não possuem `ObjectId`;
- tipos por valor não pertencem diretamente a Domains;
- cópia deve preservar a semântica de cada campo;
- campos tipados como `ObjectId<T>` copiam identidade, não objeto;
- tipos por valor não participam da hierarquia nominal de classes;
- seus campos devem possuir tipos determinados antes da HIR tipada ser considerada completa.

Layout, ordem física de campos e representação de enum pertencem às fases de layout e geração de código.

---

## 12. Tipos por Identidade

Tipos por identidade representam classes e demais entidades orientadas a objetos que possuem identidade.

Contrato conceitual:

```rust
pub struct IdentityType {
    pub symbol: SymbolId,
    pub kind: IdentityTypeKind,
}

pub enum IdentityTypeKind {
    Class,
    Interface,
    Trait,
}
```

Regras:

- classes definem tipos por identidade;
- instâncias de classes possuem exatamente um `ObjectId`;
- interfaces representam contratos sem estado persistente;
- traits representam reutilização de comportamento sem estado persistente;
- herança simples de classes deve ser representável;
- implementação de interfaces e traits deve ser consultável por fases de subtipagem;
- identidade de objeto não deve depender de layout, endereço ou backend;
- o tipo nominal deve apontar para o símbolo declarativo correspondente.

O modelo de tipos registra relações necessárias para validação. O mecanismo de despacho dinâmico, vtables e layout pertencem aos documentos de object model e codegen.

---

## 13. ObjectId

`ObjectId<T>` representa a identidade lógica de um objeto do tipo `T`.

Contrato conceitual:

```rust
pub struct ObjectIdType {
    pub object_type: TypeId,
}
```

Regras:

- `object_type` deve apontar para tipo por identidade compatível;
- `ObjectId<T>` não deve ser representado como ponteiro;
- `ObjectId<T>` não deve conceder ownership sobre a memória do objeto;
- `ObjectId<T>` deve ser copiável;
- `ObjectId<T>` deve preservar Domain, lifetime e identidade;
- upcast de `ObjectId<Sub>` para `ObjectId<Super>` deve ser representável como coerção segura;
- downcast somente deve ser permitido quando as regras semânticas aplicáveis forem satisfeitas;
- conversões não devem alterar Domain nem identidade.

As garantias operacionais de validade, lifetime e acesso pertencem aos documentos de ownership, regiões e Domains.

---

## 14. Tipos de Domain

Tipos de Domain representam a unidade física de gerenciamento de memória da linguagem.

Contrato conceitual:

```rust
pub struct DomainType {
    pub symbol: SymbolId,
}
```

Regras:

- Domain não participa da hierarquia de objetos;
- Domain não possui identidade orientada a objetos;
- todo objeto deve estar associado a exatamente um Domain;
- a associação entre `ObjectId<T>` e Domain deve ser representável para fases posteriores;
- regras de escrita e isolamento não devem ser implementadas pelo modelo básico de tipos;
- o modelo de tipos deve expor informação suficiente para `DOMAIN-IMPLEMENTATION.md` validar as regras aplicáveis.

Alocação, desalocação, sincronização e lifetime operacional pertencem aos documentos de memória e runtime.

---

## 15. Tipos Genéricos

O modelo de tipos deve representar parâmetros e aplicações genéricas do subconjunto inicial.

Contrato conceitual:

```rust
pub struct GenericParam {
    pub id: GenericParamId,
    pub owner: SymbolId,
    pub constraints: Vec<TypeConstraint>,
}

pub struct GenericInstance {
    pub base: SymbolId,
    pub args: Vec<TypeArgument>,
}
```

Regras:

- parâmetros genéricos devem possuir identidade própria;
- aplicações genéricas devem ser determinísticas;
- `Optional<T>` deve representar ausência de valor;
- `Result<T, E>` deve representar sucesso ou falha esperada quando presente no subconjunto;
- tipos genéricos são invariantes por padrão, salvo exceção especificada;
- restrições de parâmetros devem ser representáveis sem executar a verificação completa neste documento;
- generics não devem introduzir conversões implícitas não previstas pela especificação.

Detalhes de substituição, checagem de bounds, monomorfização ou estratégia equivalente pertencem a `GENERICS-IMPLEMENTATION.md`.

---

## 16. Tipos de Assinatura

Funções, métodos e construtores devem possuir assinatura tipada.

Contrato conceitual:

```rust
pub struct FunctionSignature {
    pub params: Vec<TypeId>,
    pub return_type: TypeId,
    pub receiver: Option<ReceiverType>,
}

pub struct ReceiverType {
    pub self_type: TypeId,
    pub access: ReceiverAccess,
}
```

Regras:

- todos os parâmetros devem possuir tipo determinado;
- toda assinatura deve possuir tipo de retorno;
- ausência de retorno significativo deve ser representada por `Unit`;
- métodos devem preservar o tipo do receiver quando aplicável;
- construtores devem possuir contrato tipado explícito na representação interna;
- mutabilidade ou capacidade de escrita do receiver pode ser registrada como propriedade de tipo ou metadado de assinatura;
- resolução de chamadas deve consultar assinaturas tipadas, mas sua política detalhada pertence ao pipeline de type checking.

---

## 17. Propriedades de Tipos

Cada tipo deve expor propriedades semânticas necessárias às verificações do Stage 4 e fases posteriores.

Contrato conceitual:

```rust
pub struct TypeProperties {
    pub category: TypeCategory,
    pub has_identity: bool,
    pub copyable: bool,
    pub inheritable: bool,
    pub polymorphic: bool,
    pub persistent: bool,
}
```

Regras:

- propriedades devem ser derivadas da categoria e da declaração do tipo;
- propriedades não devem contradizer o Documento 02;
- tipos primitivos e por valor não possuem identidade;
- tipos por identidade possuem identidade por meio de objetos;
- Domain possui responsabilidades de memória, mas não identidade orientada a objetos;
- propriedades devem ser consultáveis sem reexecutar resolução de nomes;
- propriedades devem ser determinísticas.

---

## 18. Origem e Rastreabilidade

Tipos devem preservar origem suficiente para diagnósticos e ferramentas.

Contrato conceitual:

```rust
pub enum TypeOrigin {
    Builtin,
    Declared(SymbolId),
    Inferred(HirId),
    Constructed,
    ErrorRecovery,
}
```

Regras:

- tipos built-in devem ter origem distinguível;
- tipos declarados devem apontar para o símbolo correspondente;
- tipos inferidos devem poder apontar para elemento HIR relevante;
- tipos construídos devem preservar seus argumentos;
- diagnósticos devem conseguir recuperar spans através de HIR, símbolo ou `SourceMap`;
- origem não deve ser usada como identidade semântica.

---

## 19. Associação com HIR

O Stage 4 deve enriquecer a HIR com tipos sem modificar sua estrutura conceitual.

Contrato conceitual:

```rust
pub struct TypedHirMap {
    pub items: TypeMap<HirItemId>,
    pub members: TypeMap<HirMemberId>,
    pub params: TypeMap<HirParamId>,
    pub locals: TypeMap<HirLocalId>,
    pub exprs: TypeMap<HirExprId>,
    pub patterns: TypeMap<HirPatternId>,
    pub type_refs: TypeMap<HirTypeRefId>,
}
```

Regras:

- elementos tipáveis válidos devem possuir entrada no mapa tipado;
- referências de tipo resolvidas devem mapear para `TypeId`;
- expressões válidas devem possuir tipo resultante;
- parâmetros e locais devem possuir tipo declarado ou inferido;
- padrões devem registrar o tipo esperado ou produzido quando aplicável;
- ausência de entrada para elemento tipável válido é erro de implementação;
- elementos inválidos devem possuir `Error` ou diagnóstico associado;
- o mapa pode estar embutido na HIR ou existir como tabela auxiliar.

---

## 20. Tipos Desconhecidos e de Erro

O modelo deve representar estados intermediários e erros de forma explícita.

Contrato conceitual:

```rust
pub enum UnknownType {
    InferenceVar(u32),
    Deferred,
}

pub struct TypeError {
    pub reason: TypeErrorReason,
}
```

Regras:

- `Unknown` pode existir durante inferência;
- `Unknown` não deve permanecer em HIR final tipada sem diagnóstico ou estado parcial explícito;
- `Error` deve ser usado quando um diagnóstico já foi emitido ou será emitido pela fase;
- `Error` deve permitir continuidade controlada;
- o uso de `Error` não deve mascarar erros independentes;
- diagnósticos em cascata devem ser evitados quando derivarem do mesmo erro original.

---

## 21. Relação com Símbolos e Escopos

Tipos nominais dependem da resolução de nomes já concluída.

Regras:

- declarações de classe, interface, trait, struct, enum e Domain devem possuir `SymbolId`;
- tipos nominais devem apontar para o `SymbolId` da declaração;
- resolução de uma referência de tipo deve ocorrer antes de construir seu tipo semântico final;
- o type checker não deve criar símbolos para nomes não resolvidos;
- escopos podem ser consultados indiretamente para contexto, mas não devem ser reconstruídos pelo modelo de tipos;
- ambiguidades de nomes devem bloquear a criação de tipo nominal definitivo para aquela referência;
- símbolos duplicados diagnosticados no Stage 3 não devem ser reinterpretados como overload de tipos no Stage 4.

---

## 22. Compatibilidade e Subtipagem

O modelo de tipos deve fornecer dados suficientes para verificar compatibilidade e subtipagem.

Regras:

- subtipagem nominal de classes deve ser representável;
- implementação de interfaces deve ser representável;
- uso compatível de traits deve ser representável quando aplicável;
- upcast deve preservar identidade, Domain e lifetime;
- conversões proibidas devem poder ser diagnosticadas;
- coerções implícitas devem ser restritas às regras seguras da especificação;
- detalhes algorítmicos pertencem a `SUBTYPING-AND-COERCIONS.md`.

---

## 23. Determinismo e Dumps

A representação de tipos deve produzir resultados determinísticos.

Regras:

- ordem de alocação de tipos deve ser estável para uma mesma entrada;
- dumps devem ordenar mapas por IDs ou ordem semântica definida;
- dumps não devem conter endereços de memória;
- dumps não devem depender de iteração instável de hash maps;
- tipos internados devem ser impressos em forma canônica;
- tipos de erro devem ser exibidos de forma explícita e estável.

Formato conceitual:

```text
type #0 = Builtin Int
type #1 = Builtin Unit
type #2 = Identity class Conta symbol #4
type #3 = ObjectId<#2>
expr #12 : #3
```

O formato final pode variar, desde que preserve determinismo e rastreabilidade.

---

## 24. Diagnósticos

O modelo de tipos deve permitir diagnósticos precisos, mas não define a política completa de emissão.

Situações que devem ser representáveis:

- tipo inexistente;
- tipo ambíguo;
- tipo usado em categoria inválida;
- aplicação genérica com número incorreto de argumentos;
- restrição genérica não satisfeita;
- tipo inferido incompatível com tipo esperado;
- atribuição incompatível;
- retorno incompatível;
- argumento incompatível com parâmetro;
- operador aplicado a tipos inválidos;
- conversão proibida;
- uso de `ObjectId<T>` com `T` não identificável;
- tentativa de usar `null` ou ausência implícita de valor, quando detectável.

Regras:

- diagnósticos devem apontar para spans relevantes;
- tipos esperado e encontrado devem ser informados quando possível;
- erros derivados de `Error` devem evitar ruído;
- diagnóstico de tipo não deve substituir diagnóstico de resolução de nomes.

---

## 25. Invariantes

As seguintes invariantes são obrigatórias:

- todo `TypeId` válido aponta para exatamente um `TypeInfo`;
- todo `TypeInfo` possui `TypeKind`;
- todo tipo nominal válido aponta para `SymbolId`;
- todo `ObjectId<T>` válido aponta para tipo por identidade;
- `Unit` é tipo válido;
- ausência de valor não é representada por `null`;
- todo elemento HIR tipável válido possui tipo ao fim do Stage 4;
- tipos equivalentes internados possuem identidade canônica;
- tipos inválidos possuem diagnóstico ou marcador de erro;
- `Unknown` não aparece em saída final sem estado parcial explícito;
- HIR tipada não depende de AST;
- o modelo de tipos não depende de MIR, backend, ABI ou layout físico.

Violação dessas invariantes deve ser tratada como erro de implementação ou internal compiler error, conforme a política de diagnósticos internos.

---

## 26. Testes Obrigatórios

O Stage 4 deve conter testes que validem pelo menos:

- criação determinística de tipos primitivos e `Unit`;
- resolução de referências de tipo para `TypeId`;
- associação de tipos a parâmetros, locais, membros e expressões;
- rejeição de tipo inexistente;
- rejeição de aplicação genérica inválida;
- inferência básica de locais e expressões;
- incompatibilidade em atribuições;
- incompatibilidade em retornos;
- incompatibilidade em chamadas;
- upcast válido entre tipos por identidade;
- rejeição de conversão entre hierarquias incompatíveis;
- `ObjectId<T>` aceitando apenas tipo por identidade;
- uso de `Optional<T>` para ausência de valor no subconjunto inicial;
- ausência de `Unknown` não diagnosticado na HIR tipada;
- dumps tipados determinísticos.

Esses testes devem integrar a suíte semântica e contribuir para o critério do Documento 28: `capic check arquivo.capi`.

---

## 27. Critérios de Conclusão

Este documento é considerado atendido quando:

- a implementação possui representação interna de tipos compatível com este modelo;
- tipos internos são opacos, determinísticos e rastreáveis;
- HIR resolvida pode ser enriquecida com `TypeId` ou equivalente;
- tipos primitivos, `Unit`, tipos por valor, tipos por identidade, Domain e `ObjectId<T>` são representáveis;
- tipos genéricos do subconjunto inicial são representáveis;
- erros e desconhecidos são representados de forma recuperável;
- invariantes são validadas por testes;
- os demais documentos do Stage 4 conseguem referenciar este modelo sem redefinir suas entidades.
