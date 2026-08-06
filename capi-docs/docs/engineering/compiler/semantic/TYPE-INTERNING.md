# Type Interning

**Projeto:** Linguagem Capi  
**Documento:** TYPE-INTERNING  
**Status:** Aprovado  
**Stage:** Stage 4 — Sistema de tipos  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o contrato de engenharia para interning de tipos na implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- como tipos semânticos são canonicalizados;
- como `TypeId` é criado, reutilizado e consultado;
- quais dados entram na chave canônica de um tipo;
- quais dados não podem influenciar identidade de tipo;
- como tipos built-in são registrados;
- como tipos nominais, construídos, genéricos e de assinatura são internados;
- como tipos desconhecidos e de erro participam do processo;
- como o interner preserva determinismo;
- quais invariantes e testes validam a implementação.

Interning de tipos é a técnica pela qual duas representações semanticamente iguais de um tipo passam a compartilhar a mesma identidade interna. Essa identidade é representada por `TypeId`, `TyId` ou mecanismo equivalente definido em `TYPE-MODEL.md`.

---

## 2. Escopo

Este documento cobre:

- autoridade de canonicalização de tipos;
- estrutura conceitual de `TypeInterner`;
- criação e consulta de `TypeId`;
- chaves canônicas de tipos;
- interning de tipos primitivos e `Unit`;
- interning de tipos nominais;
- interning de `ObjectId<T>`;
- interning de tipos por valor;
- interning de aplicações genéricas;
- interning de assinaturas;
- tratamento de tipos desconhecidos e de erro;
- relação com `TypeTable`, HIR e símbolos;
- determinismo, dumps e testes.

Este documento não cobre:

- modelo completo de tipos;
- algoritmo de inferência;
- resolução de restrições;
- regras detalhadas de subtipagem;
- coerções;
- seleção de overload;
- implementação completa de generics;
- ownership;
- regiões;
- Domains em nível operacional;
- layout de objetos;
- ABI;
- geração de código.

Esses temas pertencem a:

- `TYPE-MODEL.md`;
- `TYPE-INFERENCE.md`;
- `TYPE-CHECKING-PIPELINE.md`;
- `SUBTYPING-AND-COERCIONS.md`;
- `GENERICS-IMPLEMENTATION.md`;
- `OWNERSHIP-MODEL.md`;
- `REGION-ANALYSIS.md`;
- `DOMAIN-IMPLEMENTATION.md`;
- `OBJECT-LAYOUT.md`;
- `ABI-IMPLEMENTATION.md`.

---

## 3. Posição no Stage 4

No Stage 4, o interning de tipos sustenta:

- comparação eficiente entre tipos;
- armazenamento canônico de tipos construídos;
- reuso de tipos built-in;
- estabilidade de IDs durante a sessão semântica;
- associação consistente entre HIR e tipos;
- inferência baseada em handles opacos;
- verificação de compatibilidade sem comparar estruturas repetidamente;
- dumps determinísticos da HIR tipada e das tabelas de tipos.

Fluxo conceitual:

```text
TypeKind bruto
    ↓
Normalização canônica
    ↓
TypeKey
    ↓
Consulta no TypeInterner
    ↓
TypeId existente ou novo
    ↓
TypeTable / TypedHirMap
```

O interner não decide se um tipo é permitido em determinado contexto. Ele apenas garante identidade canônica para tipos que a fase semântica solicitou registrar.

---

## 4. Princípios

O interning de tipos deve seguir estes princípios:

- tipos semanticamente iguais devem possuir o mesmo `TypeId`;
- tipos semanticamente distintos não devem compartilhar `TypeId`;
- `TypeId` deve ser opaco;
- chaves canônicas não devem depender de endereço de memória;
- chaves canônicas não devem depender de spans, texto fonte ou ordem instável de mapas;
- origem diagnóstica não deve alterar identidade semântica do tipo;
- propriedades derivadas não devem duplicar a identidade do tipo;
- interning deve ser determinístico para a mesma entrada;
- built-ins devem ser registrados em ordem fixa;
- tipos compostos devem usar `TypeId` dos componentes;
- tipos nominais devem usar `SymbolId` resolvido, não texto do nome;
- o interner não deve resolver nomes;
- o interner não deve executar inferência;
- o interner não deve aplicar coerções;
- o interner não deve depender de MIR, backend, ABI ou layout físico.

---

## 5. Terminologia

| Termo | Significado |
| --- | --- |
| Interning | Processo de armazenar uma única instância canônica para valores semanticamente iguais. |
| `TypeId` | Identidade opaca retornada pelo interner. |
| `TypeKind` | Forma semântica do tipo conforme `TYPE-MODEL.md`. |
| `TypeKey` | Chave canônica usada para busca e deduplicação. |
| `TypeInfo` | Registro armazenado para um `TypeId`. |
| Tipo canônico | Tipo já registrado e consultável por `TypeId`. |
| Tipo bruto | Estrutura solicitada ao interner antes da canonicalização. |
| Interning estrutural | Deduplicação baseada na forma do tipo e em seus componentes. |
| Interning nominal | Deduplicação baseada no símbolo declarativo do tipo. |

Regras:

- `TypeId` identifica tipo semântico;
- `TypeKey` identifica forma canônica para busca interna;
- `TypeInfo` contém a representação consultável do tipo;
- nenhuma API externa ao módulo de tipos deve depender do formato físico de `TypeKey`.

---

## 6. Entidades Conceituais

| Entidade | Responsabilidade |
| --- | --- |
| `TypeInterner` | Autoridade que cria e reutiliza `TypeId`. |
| `TypeKey` | Chave canônica hashable e comparável. |
| `TypeInfo` | Dados armazenados para cada tipo internado. |
| `TypeArena` | Armazenamento indexado por `TypeId`. |
| `TypeIndex` | Mapa de `TypeKey` para `TypeId`. |
| `BuiltinTypes` | Conjunto fixo de `TypeId` para tipos built-in. |
| `InternedType` | Resultado de uma operação de interning. |
| `TypeInternerSnapshot` | Estado observável para dumps ou testes. |
| `TypeInterningError` | Erro interno quando invariantes do interner são violadas. |

Os nomes concretos podem variar na implementação. O contrato obrigatório é a presença dessas responsabilidades.

---

## 7. Interface Conceitual

O interner deve oferecer operações pequenas e explícitas.

Contrato conceitual:

```rust
pub struct TypeInterner {
    builtins: BuiltinTypes,
    arena: TypeArena,
    index: TypeIndex,
}

impl TypeInterner {
    pub fn intern(&mut self, kind: TypeKind) -> TypeId;
    pub fn get(&self, id: TypeId) -> &TypeInfo;
    pub fn key_of(&self, id: TypeId) -> &TypeKey;
    pub fn builtins(&self) -> BuiltinTypes;
}
```

Regras:

- `intern` deve retornar `TypeId` existente quando a chave já estiver registrada;
- `intern` deve criar novo `TypeId` apenas para chave ausente;
- `get` deve falhar de forma controlada ou gerar erro interno para `TypeId` inválido;
- `builtins` deve retornar IDs estáveis dentro da sessão;
- nenhuma operação deve expor índice cru como contrato público interno;
- consultas devem ser somente leitura;
- mutação deve ficar restrita ao registro de novos tipos.

---

## 8. TypeInfo

`TypeInfo` representa os dados canônicos associados a um `TypeId`.

Contrato conceitual:

```rust
pub struct TypeInfo {
    pub id: TypeId,
    pub kind: TypeKind,
    pub key: TypeKey,
    pub origin: TypeOrigin,
    pub properties: TypeProperties,
}
```

Regras:

- `id` deve corresponder ao índice ou handle usado para armazenar o registro;
- `kind` deve representar a forma semântica canônica;
- `key` deve ser derivável de `kind` e dos componentes semanticamente relevantes;
- `origin` deve existir para rastreabilidade, mas não deve definir igualdade semântica;
- `properties` devem ser derivadas ou validadas a partir do tipo;
- `TypeInfo` não deve armazenar layout físico, offsets, vtables, registradores ou ABI.

---

## 9. TypeKey

`TypeKey` é a chave usada pelo interner para deduplicar tipos.

Contrato conceitual:

```rust
pub enum TypeKey {
    Primitive(PrimitiveType),
    Unit,
    Value(ValueTypeKey),
    Identity(SymbolId),
    Domain(SymbolId),
    ObjectId(TypeId),
    Function(FunctionSignatureKey),
    GenericParam(GenericParamId),
    GenericInstance(GenericInstanceKey),
    Unknown(UnknownKey),
    Error(ErrorTypeKey),
}
```

Regras:

- `TypeKey` deve implementar igualdade estrutural determinística;
- `TypeKey` deve implementar hashing ou ordenação sem depender de endereços;
- tipos nominais devem usar `SymbolId`;
- tipos compostos devem usar `TypeId` dos componentes já internados;
- campos de origem, span, texto escrito e diagnóstico não devem participar da chave, salvo quando definirem identidade semântica;
- propriedades deriváveis não devem participar da chave;
- duas chaves iguais devem sempre apontar para o mesmo `TypeId`.

---

## 10. Dados que Não Participam da Identidade

Os seguintes dados não devem alterar a identidade canônica de um tipo:

- `Span`;
- arquivo fonte;
- nó AST de origem;
- elemento HIR que solicitou o tipo;
- texto original de um nome resolvido;
- ordem de descoberta não semântica;
- mensagens diagnósticas;
- severidade diagnóstica;
- propriedades deriváveis;
- layout físico;
- ABI;
- backend selecionado;
- endereço de memória.

Exemplo:

```text
let a: Int
let b: Int
```

As duas referências ao tipo `Int` devem apontar para o mesmo `TypeId`, ainda que tenham spans diferentes.

---

## 11. Registro de Built-ins

Tipos built-in devem ser registrados de forma fixa durante a inicialização semântica.

Ordem conceitual mínima:

```text
Bool
Char
Int
UInt
Float
Double
Unit
```

Regras:

- built-ins devem estar disponíveis antes da conversão de referências de tipo;
- IDs de built-ins devem ser determinísticos dentro de uma mesma versão da implementação;
- built-ins não devem depender de prelude textual para existir internamente;
- built-ins podem ter símbolos sintéticos ou entradas especiais, desde que a distinção seja documentada;
- `Unit` deve ser registrado como tipo válido;
- ausência de valor não deve criar tipo built-in `Null`.

Se a implementação optar por alterar a ordem interna, dumps e testes devem continuar determinísticos.

---

## 12. Interning de Tipos Nominais

Tipos nominais são internados a partir do símbolo da declaração.

Exemplos:

```rust
TypeKey::Identity(class_symbol)
TypeKey::Domain(domain_symbol)
TypeKey::Value(ValueTypeKey::Struct(struct_symbol))
```

Regras:

- a resolução de nomes deve ter produzido `SymbolId` antes do interning nominal;
- o nome textual não deve ser usado como identidade final;
- duas declarações distintas com mesmo texto em escopos diferentes devem produzir tipos distintos;
- declaração duplicada no mesmo escopo deve permanecer erro de resolução e não deve ser mesclada pelo interner;
- classes, interfaces, traits, structs, enums e Domains devem ser distinguíveis na chave;
- a categoria do símbolo deve ser validada antes ou durante o registro do tipo.

---

## 13. Interning de ObjectId

`ObjectId<T>` deve ser internado como tipo construído a partir do tipo do objeto.

Contrato conceitual:

```rust
pub fn object_id(&mut self, object_type: TypeId) -> TypeId {
    self.intern(TypeKind::ObjectId(object_type))
}
```

Regras:

- `object_type` deve ser um `TypeId` já internado;
- `ObjectId<T>` deve reutilizar o mesmo `TypeId` quando `T` for o mesmo;
- `ObjectId<T>` não deve usar layout, ponteiro ou endereço como chave;
- `ObjectId<Sub>` e `ObjectId<Super>` são tipos distintos, ainda que possam ser compatíveis por upcast;
- subtipagem entre `ObjectId<T>` pertence a `SUBTYPING-AND-COERCIONS.md`;
- validar se `T` é tipo por identidade pode ocorrer antes do interning ou durante type checking, mas falhas devem ser diagnosticáveis.

---

## 14. Interning de Tipos por Valor

Tipos por valor devem ser internados conforme sua natureza nominal ou estrutural.

Regras:

- structs e enums declarados devem usar chave nominal baseada em `SymbolId`;
- tuples devem usar chave estrutural baseada na sequência de `TypeId`;
- a ordem dos campos de tuple deve participar da chave;
- spans dos campos não devem participar da chave;
- campos de structs e enums nominais não precisam ser repetidos na chave quando o `SymbolId` já define a identidade;
- tipos por valor diferentes não devem ser mesclados por possuírem campos estruturalmente iguais quando forem nominais.

Exemplo:

```text
type #10 = Tuple<#1, #2>
type #11 = Tuple<#2, #1>
```

`#10` e `#11` devem ser tipos distintos quando a ordem dos componentes for diferente.

---

## 15. Interning de Genéricos

Aplicações genéricas devem ser internadas por base e argumentos canônicos.

Contrato conceitual:

```rust
pub struct GenericInstanceKey {
    pub base: SymbolId,
    pub args: Vec<TypeId>,
}
```

Regras:

- `base` deve apontar para declaração genérica resolvida;
- todos os argumentos devem estar internados antes da aplicação;
- ordem dos argumentos deve participar da chave;
- `Optional<Int>` e `Optional<UInt>` devem ser tipos distintos;
- duas ocorrências de `Optional<Int>` devem compartilhar `TypeId`;
- quantidade incorreta de argumentos deve ser diagnosticável;
- validação de bounds pertence a `GENERICS-IMPLEMENTATION.md` e ao pipeline de type checking.

Parâmetros genéricos devem usar identidade própria:

```rust
pub struct GenericParamId(u32);
```

Dois parâmetros chamados `T` em funções ou tipos diferentes não devem compartilhar identidade apenas por terem o mesmo nome textual.

---

## 16. Interning de Assinaturas

Tipos de função e assinaturas devem ser internáveis quando participarem da tipagem.

Contrato conceitual:

```rust
pub struct FunctionSignatureKey {
    pub receiver: Option<ReceiverTypeKey>,
    pub params: Vec<TypeId>,
    pub return_type: TypeId,
}
```

Regras:

- tipos dos parâmetros devem estar internados;
- tipo de retorno deve estar internado;
- ausência de retorno significativo deve usar `Unit`;
- ordem de parâmetros participa da chave;
- receiver participa da chave quando a assinatura representa método;
- nome da função não deve participar da chave estrutural de assinatura;
- símbolos de overload pertencem à resolução de chamadas e não devem ser confundidos com igualdade de assinatura.

---

## 17. Tipos Unknown e Error

Tipos desconhecidos e de erro exigem tratamento explícito para evitar falsa equivalência.

Regras para `Unknown`:

- variáveis de inferência distintas devem possuir identidade distinta;
- `Unknown` não deve ser mesclado apenas por categoria;
- `Unknown::Deferred` só pode ser compartilhado quando a implementação documentar que ele é marcador global sem identidade inferencial;
- `Unknown` não deve aparecer em saída final tipada sem diagnóstico ou estado parcial explícito.

Regras para `Error`:

- `Error` pode ser internado como marcador único quando sua finalidade for suprimir cascatas;
- `Error` pode possuir chave diferenciada quando a implementação precisar preservar categorias de recuperação;
- spans e mensagens não devem participar da chave de `Error`;
- presença de `Error` deve permanecer observável para validação e dumps.

---

## 18. Normalização

Antes de consultar o índice, o interner deve normalizar a representação bruta para chave canônica.

Regras:

- aliases, quando existirem, devem ser tratados conforme a fase responsável por expandi-los;
- referências de tipo devem ser convertidas para `TypeId` antes de compor tipos construídos;
- nomes qualificados resolvidos devem virar `SymbolId`;
- listas de argumentos devem preservar ordem semântica;
- propriedades derivadas devem ser recalculadas ou consultadas a partir do tipo canônico;
- normalização não deve aplicar coerções;
- normalização não deve escolher overload;
- normalização não deve resolver restrições genéricas.

Pipeline conceitual:

```text
TypeKind bruto
    ↓
validar componentes mínimos
    ↓
converter componentes para TypeId/SymbolId
    ↓
montar TypeKey
    ↓
consultar TypeIndex
```

---

## 19. TypeTable e HIR Tipada

O interner é a autoridade de identidade, mas `TypeTable` ou `TypedHirMap` é a autoridade de associação com elementos do programa.

Regras:

- `TypeInterner` armazena tipos;
- `TypedHirMap` associa elementos HIR a `TypeId`;
- `TypeRefBinding` associa referências de tipo a `TypeId`;
- `SymbolId` de declaração nominal pode possuir associação com `TypeId`;
- a mesma `TypeId` pode ser usada por vários elementos HIR;
- o interner não deve armazenar lista de todos os usos HIR de um tipo;
- remoção ou reconstrução de HIR exige invalidar ou reconstruir associações tipadas correspondentes.

---

## 20. Determinismo

O interner deve produzir resultados determinísticos.

Regras:

- built-ins devem ser registrados em ordem fixa;
- traversal que cria tipos deve seguir ordem semântica documentada;
- mapas internos devem produzir dumps ordenados por `TypeId` ou chave canônica;
- hashing aleatório não pode afetar ordem de alocação observável;
- concorrência, quando introduzida, não deve tornar IDs não determinísticos;
- chaves equivalentes devem ser detectadas independentemente da ordem de solicitação, exceto pelo número atribuído ao primeiro registro;
- testes devem evitar depender de ordem não especificada.

Quando a implementação paralelizar análise semântica, deve haver etapa de consolidação determinística para tipos compartilhados.

---

## 21. Concorrência e Sessão de Compilação

No subconjunto inicial, o interner pode ser local à sessão semântica.

Regras:

- `TypeId` deve ser válido apenas dentro da sessão ou banco semântico que o criou;
- `TypeId` não deve ser serializado como identidade global permanente;
- caches incrementais futuros devem versionar ou validar chaves canônicas;
- acesso concorrente deve preservar atomicidade entre consulta e inserção;
- leituras podem ser compartilhadas após congelamento do interner;
- mutação concorrente deve ser evitada até haver política explícita de paralelismo.

Este documento não exige compilação incremental, mas não deve bloquear sua introdução futura.

---

## 22. Erros Internos

As seguintes situações indicam erro de implementação:

- `TypeId` sem `TypeInfo`;
- `TypeInfo.id` diferente do ID usado na consulta;
- `TypeKey` armazenada incompatível com `TypeKind`;
- duas entradas com mesma `TypeKey` e `TypeId` diferente;
- `TypeId` apontando para índice fora da arena;
- tipo composto referenciando `TypeId` inexistente;
- tipo nominal referenciando `SymbolId` inexistente ou de categoria incompatível;
- dump observável contendo endereço de memória;
- built-in obrigatório ausente.

Esses casos devem seguir a política de internal compiler errors definida nos documentos de diagnósticos.

---

## 23. Dumps

O interner deve oferecer forma determinística de inspeção para testes e depuração.

Formato conceitual:

```text
types:
  #0 Builtin Bool
  #1 Builtin Char
  #2 Builtin Int
  #3 Builtin UInt
  #4 Builtin Float
  #5 Builtin Double
  #6 Builtin Unit
  #7 Identity class Conta symbol #12
  #8 ObjectId<#7>
  #9 GenericInstance Optional<#2>
```

Regras:

- dumps devem ordenar por `TypeId`;
- tipos compostos devem imprimir argumentos em forma canônica;
- símbolos devem aparecer por ID e, quando útil, por nome estável;
- tipos de erro e desconhecidos devem ser explícitos;
- dumps não devem depender de ponteiros, paths absolutos temporários ou ordem instável de hash map.

---

## 24. Testes Obrigatórios

O Stage 4 deve conter testes de interning que validem:

- built-ins criados uma única vez;
- `Unit` internado como tipo válido;
- duas solicitações de `Int` retornam o mesmo `TypeId`;
- tipos nominais distintos com mesmo nome textual em escopos diferentes não são mesclados;
- duas ocorrências de `ObjectId<Conta>` retornam o mesmo `TypeId`;
- `ObjectId<Conta>` e `ObjectId<Cliente>` retornam `TypeId` diferentes;
- tuples com mesmos componentes e mesma ordem são mescladas;
- tuples com ordem diferente não são mescladas;
- `Optional<Int>` é mesclado entre ocorrências equivalentes;
- `Optional<Int>` e `Optional<UInt>` são distintos;
- assinaturas com parâmetros iguais e mesma ordem são mescladas;
- assinaturas com ordem de parâmetros diferente são distintas;
- variáveis de inferência distintas não são mescladas indevidamente;
- tipo de erro é representável e aparece em dump;
- dump do interner é determinístico.

Esses testes podem ser unitários no crate semântico e também aparecer em testes de snapshot da HIR tipada.

---

## 25. Invariantes

As seguintes invariantes são obrigatórias:

- todo `TypeId` válido aponta para exatamente um `TypeInfo`;
- toda `TypeKey` registrada aponta para exatamente um `TypeId`;
- todo `TypeInfo` possui `TypeKey` compatível com seu `TypeKind`;
- tipos canonicamente iguais compartilham `TypeId`;
- tipos canonicamente distintos não compartilham `TypeId`;
- built-ins obrigatórios existem antes da tipagem;
- tipos nominais usam `SymbolId`, não texto;
- tipos compostos usam componentes internados;
- `Span` e origem diagnóstica não afetam identidade de tipo;
- `ObjectId<T>` não codifica ponteiro, endereço ou layout;
- dumps são determinísticos;
- o interner não executa resolução de nomes, inferência, coerções, MIR, ABI ou codegen.

Violação dessas invariantes deve ser tratada como erro de implementação.

---

## 26. Critérios de Conclusão

Este documento é considerado atendido quando:

- existe autoridade única para criação e consulta de `TypeId`;
- tipos built-in são registrados de forma determinística;
- chaves canônicas são definidas para todas as formas exigidas por `TYPE-MODEL.md`;
- tipos semanticamente iguais são deduplicados;
- tipos semanticamente distintos permanecem separados;
- HIR tipada e `TypeTable` usam `TypeId` internado;
- dumps e testes demonstram determinismo;
- os documentos de inferência, pipeline, generics, subtipagem e coerções podem usar este contrato sem redefinir interning.
