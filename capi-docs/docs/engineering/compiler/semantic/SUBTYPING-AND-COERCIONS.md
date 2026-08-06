# Subtyping and Coercions

**Projeto:** Linguagem Capi  
**Documento:** SUBTYPING-AND-COERCIONS  
**Status:** Aprovado  
**Stage:** Stage 4 — Sistema de tipos  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o contrato de engenharia para subtipagem e coerções da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- como a relação de subtipo é representada e consultada;
- quais formas de subtipagem pertencem ao Stage 4;
- quais coerções implícitas são permitidas;
- quais conversões exigem manifestação explícita do programador;
- quais conversões são proibidas;
- como coerções são registradas na HIR tipada;
- como subtipagem participa de atribuições, retornos, chamadas e overload;
- como preservar identidade, Domain e lifetime em `ObjectId<T>`;
- quais diagnósticos devem ser produzidos;
- quais invariantes e testes validam a implementação.

Subtipagem é uma relação semântica entre tipos. Coerção é uma transformação aceita pelo compilador entre um tipo produzido e um tipo esperado. Nem toda compatibilidade exige coerção observável, e nenhuma coerção implícita pode violar as garantias do sistema de tipos.

---

## 2. Escopo

Este documento cobre:

- subtipagem reflexiva;
- subtipagem nominal entre classes;
- implementação de interfaces;
- compatibilidade de traits quando aplicável ao subconjunto;
- subtipagem de `ObjectId<T>`;
- upcast implícito seguro;
- coerções explícitas e implícitas;
- conversões proibidas;
- interação com chamadas, argumentos, retornos e atribuições;
- registro de coerções;
- diagnósticos;
- determinismo e testes.

Este documento não cobre:

- modelo completo de tipos;
- implementação do interner;
- algoritmo de inferência;
- pipeline completo de type checking;
- implementação completa de generics;
- resolução completa de overload;
- downcast operacional completo;
- layout de objetos;
- vtables;
- despacho dinâmico;
- ownership;
- regiões;
- validação operacional de Domains;
- MIR;
- ABI;
- backend.

Esses temas pertencem a:

- `TYPE-MODEL.md`;
- `TYPE-INTERNING.md`;
- `TYPE-INFERENCE.md`;
- `TYPE-CHECKING-PIPELINE.md`;
- `GENERICS-IMPLEMENTATION.md`;
- `OBJECT-MODEL.md`;
- `OBJECT-LAYOUT.md`;
- `DYNAMIC-DISPATCH.md`;
- `OWNERSHIP-MODEL.md`;
- `REGION-ANALYSIS.md`;
- `DOMAIN-IMPLEMENTATION.md`;
- `MIR-LOWERING.md`;
- `ABI-IMPLEMENTATION.md`.

---

## 3. Posição no Stage 4

No Stage 4, subtipagem e coerções são usadas pelo pipeline de type checking para validar compatibilidade.

Fluxo conceitual:

```text
tipo produzido + tipo esperado
    ↓
comparação canônica
    ↓
consulta de subtipagem
    ↓
decisão de compatibilidade
    ↓
registro opcional de coerção
    ↓
HIR tipada / diagnóstico
```

Essa fase não altera a HIR estrutural. Ela registra decisões tipadas em tabelas auxiliares ou campos semânticos.

---

## 4. Princípios

Subtipagem e coerções devem seguir estes princípios:

- igualdade canônica de `TypeId` é sempre compatível;
- subtipagem deve ser determinística;
- subtipagem nominal deve usar símbolos declarativos, não texto;
- coerções implícitas só são permitidas quando seguras;
- upcast deve preservar identidade, Domain e lifetime;
- downcast não deve ser implícito;
- conversões entre hierarquias incompatíveis são proibidas;
- coerção aplicada deve ser registrada explicitamente;
- nenhuma coerção deve alterar ownership, alocação, layout ou ABI;
- coerção não deve resolver nomes;
- coerção não deve inferir tipos novos sem passar pelo pipeline;
- ausência de regra de coerção deve produzir incompatibilidade;
- ambiguidade não deve ser resolvida arbitrariamente.

---

## 5. Terminologia

| Termo | Significado |
| --- | --- |
| Subtipo | Tipo que pode ser usado onde um supertipo é esperado. |
| Supertipo | Tipo mais geral aceito por uma relação de subtipagem. |
| Compatibilidade | Aceitação de um tipo produzido em contexto de tipo esperado. |
| Coerção | Conversão registrada entre tipo produzido e tipo esperado. |
| Conversão implícita | Coerção aplicada automaticamente pelo compilador. |
| Conversão explícita | Conversão exigida por sintaxe ou intenção do programador. |
| Upcast | Conversão de subtipo para supertipo. |
| Downcast | Conversão de supertipo para subtipo. |
| Coerção identidade | Caso em que tipo produzido e esperado são o mesmo tipo. |
| Candidato de coerção | Transformação possível ainda não validada. |

Regras:

- compatibilidade pode existir por igualdade, subtipagem ou coerção permitida;
- coerção identidade não precisa produzir operação em runtime;
- coerções registradas são dados semânticos, não código gerado.

---

## 6. Entidades Conceituais

| Entidade | Responsabilidade |
| --- | --- |
| `SubtypeChecker` | Consulta relações de subtipo. |
| `CoercionChecker` | Decide se uma coerção é permitida. |
| `CompatibilityResult` | Resultado de compatibilidade entre tipos. |
| `Coercion` | Registro de coerção aplicada. |
| `CoercionKind` | Categoria da coerção. |
| `CoercionTable` | Mapa de coerções por elemento HIR. |
| `SubtypeGraph` | Relações nominais de classes, interfaces e traits. |
| `InheritanceChain` | Cadeia de superclasses de uma classe. |
| `InterfaceImplSet` | Interfaces implementadas por um tipo por identidade. |
| `CoercionDiagnostic` | Diagnóstico produzido por coerção inválida. |

Os nomes concretos podem variar na implementação. O contrato obrigatório é a presença dessas responsabilidades.

---

## 7. Relação de Subtipo

Contrato conceitual:

```rust
pub trait SubtypeChecker {
    fn is_subtype(&self, actual: TypeId, expected: TypeId) -> SubtypeResult;
}

pub enum SubtypeResult {
    Yes(SubtypeEvidence),
    No,
    Error,
}
```

Regras:

- `is_subtype(T, T)` deve retornar sucesso para todo tipo válido;
- tipos de erro devem produzir `Error` ou sucesso recuperável sem cascata;
- tipos desconhecidos não devem produzir sucesso final;
- a consulta deve ser pura e determinística;
- o resultado deve poder carregar evidência útil para coerção e diagnóstico;
- ausência de relação conhecida deve retornar `No`, não escolher conversão alternativa.

---

## 8. Evidência de Subtipagem

Evidência explica por que uma relação de subtipo é válida.

Contrato conceitual:

```rust
pub enum SubtypeEvidence {
    Identity,
    ClassExtends(Vec<SymbolId>),
    ImplementsInterface(SymbolId),
    CompatibleTrait(SymbolId),
    ObjectIdUpcast(Box<SubtypeEvidence>),
    GenericVariance(GenericVarianceEvidence),
}
```

Regras:

- evidência deve ser determinística;
- cadeias de herança devem ser ordenadas da classe concreta ao supertipo;
- evidência de interface deve indicar contrato implementado;
- evidência de `ObjectId<T>` deve preservar a evidência sobre `T`;
- evidência não deve depender de layout físico;
- evidência pode ser usada para mensagens de diagnóstico e dumps.

---

## 9. Subtipagem Reflexiva

Todo tipo válido é subtipo de si mesmo.

Regras:

- `T <: T` deve valer por identidade canônica de `TypeId`;
- reflexividade não deve registrar coerção observável obrigatória;
- `Error` pode ser tratado como compatível apenas para recuperação;
- `Unknown` não deve ser considerado subtipo final sem solução.

---

## 10. Classes

Classes usam subtipagem nominal baseada em herança simples.

Regras:

- se `Sub extends Super`, então `Sub <: Super`;
- relação transitiva deve ser reconhecida;
- uma classe possui no máximo uma superclasse direta;
- ciclos de herança devem ser diagnosticados;
- subtipagem deve usar `SymbolId` da classe;
- nomes textuais iguais em escopos diferentes não devem ser mesclados;
- herança múltipla de implementação é proibida;
- layout físico não participa da decisão de subtipagem.

Exemplo conceitual:

```text
Cachorro <: Animal
Animal <: SerVivo
logo Cachorro <: SerVivo
```

---

## 11. Interfaces

Classes podem ser subtipos de interfaces que implementam.

Regras:

- se classe `C` implementa interface `I`, então `C <: I` no contexto de observação de objeto;
- interfaces não possuem estado persistente;
- implementação de interface não altera hierarquia de classes;
- múltiplas interfaces são permitidas;
- conflito ou ausência de implementação exigida deve ser diagnosticável;
- detalhes de checagem de membros de interface podem ser coordenados com generics e object model.

Interfaces representam contratos. A coerção para interface não deve alterar identidade, Domain ou lifetime do objeto.

---

## 12. Traits

Traits participam de compatibilidade conforme permitido pela linguagem e pelo subconjunto implementado.

Regras:

- trait não representa herança múltipla de implementação;
- trait não adiciona estado persistente;
- uso compatível de trait deve ser baseado em declaração explícita ou regra normativa;
- conflitos entre traits devem ser diagnosticados;
- compatibilidade com trait não deve alterar layout físico;
- quando traits ainda não estiverem implementadas no subconjunto, consultas devem retornar `No` ou diagnóstico de recurso não suportado.

---

## 13. ObjectId

`ObjectId<T>` possui relação de subtipagem derivada da relação de `T`.

Regra principal:

```text
se Sub <: Super
então ObjectId<Sub> <: ObjectId<Super>
```

Regras:

- `T` deve ser tipo por identidade válido;
- upcast de `ObjectId<Sub>` para `ObjectId<Super>` deve ser seguro;
- upcast deve ter custo conceitual zero;
- upcast deve preservar `ObjectId`;
- upcast deve preservar Domain;
- upcast deve preservar lifetime;
- upcast não deve modificar memória;
- `ObjectId<T>` não deve ser tratado como ponteiro;
- `ObjectId<Super>` para `ObjectId<Sub>` é downcast e não deve ser implícito.

---

## 14. Tipos Primitivos e Unit

No subconjunto inicial, tipos primitivos e `Unit` não possuem subtipagem entre si.

Regras:

- `Bool <: Bool`;
- `Char <: Char`;
- `Int <: Int`;
- `UInt <: UInt`;
- `Float <: Float`;
- `Double <: Double`;
- `Unit <: Unit`;
- `Int` não deve ser implicitamente subtipo de `Float` sem regra explícita;
- conversões numéricas implícitas não devem ser inventadas pela implementação;
- `Unit` não deve ser convertido implicitamente para outro tipo.

Conversões numéricas futuras devem ser especificadas antes de implementação.

---

## 15. Tipos por Valor

Tipos por valor nominais não são subtipos entre si apenas por possuírem a mesma estrutura.

Regras:

- structs nominais distintos são incompatíveis por padrão;
- enums nominais distintos são incompatíveis por padrão;
- tuples podem ser compatíveis apenas quando possuírem mesma aridade e tipos de componentes compatíveis conforme regra explícita;
- cópia por valor não é coerção de subtipo;
- campos internos não definem subtipagem nominal;
- tipos por valor não participam de `ObjectId`.

---

## 16. Generics e Variância

Tipos genéricos são invariantes por padrão.

Regras:

- `G<A>` não deve ser subtipo de `G<B>` apenas porque `A <: B`;
- exceções de variância devem ser explícitas e comprovadamente seguras;
- `Optional<Sub>` não deve ser considerado subtipo de `Optional<Super>` sem regra formal;
- `Result<T, E>` segue invariância por padrão;
- validação de bounds e substituições pertence a `GENERICS-IMPLEMENTATION.md`;
- este documento define apenas a consulta de compatibilidade resultante.

---

## 17. Compatibilidade

Compatibilidade decide se um valor de tipo produzido pode ser usado em contexto de tipo esperado.

Contrato conceitual:

```rust
pub enum CompatibilityResult {
    Compatible,
    CompatibleWithCoercion(Coercion),
    Incompatible(TypeMismatch),
    Error,
}
```

Ordem conceitual:

1. Se tipos são iguais, compatível.
2. Se `actual <: expected`, compatível por subtipagem.
3. Se há coerção implícita segura, compatível com coerção.
4. Caso contrário, incompatível.

Regras:

- a ordem deve ser determinística;
- coerção não deve ser buscada quando tipos já são iguais;
- `Error` deve evitar cascatas;
- incompatibilidade deve preservar tipos esperado e encontrado para diagnóstico.

---

## 18. Coerções Implícitas

Coerções implícitas são aplicadas automaticamente pelo compilador quando seguras.

Permitidas no Stage 4:

- coerção identidade;
- upcast nominal seguro;
- upcast de `ObjectId<Sub>` para `ObjectId<Super>`;
- coerção para interface implementada quando representada pelo sistema de tipos;
- coerção de trait compatível quando habilitada pelo subconjunto.

Regras:

- toda coerção implícita não trivial deve ser registrada;
- coerção implícita não deve alterar identidade;
- coerção implícita não deve alterar Domain;
- coerção implícita não deve alterar lifetime;
- coerção implícita não deve introduzir alocação oculta no Stage 4;
- coerção implícita não deve depender de backend.

---

## 19. Conversões Explícitas

Conversões explícitas exigem sintaxe ou intenção declarada do programador.

Regras:

- downcast é conversão explícita ou operação verificada, não coerção implícita;
- conversão numérica com possível perda deve ser explícita quando existir;
- conversão entre representações distintas deve ser explícita;
- cast explícito ainda deve ser validado pelo sistema de tipos;
- cast explícito inválido deve produzir diagnóstico;
- regras operacionais de downcast pertencem à semântica operacional e fases posteriores.

Este documento não exige implementação completa de downcast no Stage 4, apenas impede sua aplicação implícita.

---

## 20. Conversões Proibidas

As seguintes conversões são proibidas:

- entre hierarquias de classes incompatíveis;
- entre `ObjectId<T>` e ponteiros;
- entre `ObjectId<T>` e inteiros;
- entre Domains distintos como se fossem objetos;
- que alterem Domain de um objeto;
- que alterem identidade de um objeto;
- que alterem lifetime de um objeto;
- que transformem `null` em objeto válido;
- que ignorem ausência representada por `Optional<T>`;
- que dependam de layout físico para serem aceitas no Stage 4.

Conversão proibida deve produzir diagnóstico quando solicitada ou necessária para compatibilidade.

---

## 21. Coercion

Coerções aplicadas devem ser representadas explicitamente.

Contrato conceitual:

```rust
pub struct Coercion {
    pub id: CoercionId,
    pub kind: CoercionKind,
    pub source: TypeId,
    pub target: TypeId,
    pub origin: CoercionOrigin,
}

pub enum CoercionKind {
    Identity,
    Upcast(SubtypeEvidence),
    InterfaceUpcast(SubtypeEvidence),
    TraitCoercion(SubtypeEvidence),
    ExplicitCast,
    Error,
}
```

Regras:

- `source` e `target` devem ser tipos internados;
- coerções não triviais devem registrar origem HIR;
- `Identity` pode ser omitida de tabelas se a ausência for interpretada como identidade;
- `Error` deve permitir recuperação;
- coerção aplicada deve ser consultável por lowering futuro;
- coerção não deve conter layout físico ou instrução de backend.

---

## 22. CoercionTable

`CoercionTable` registra coerções associadas a elementos HIR.

Contrato conceitual:

```rust
pub struct CoercionTable {
    pub exprs: Map<HirExprId, Coercion>,
    pub args: Map<(HirExprId, usize), Coercion>,
    pub returns: Map<HirStmtId, Coercion>,
}
```

Regras:

- tabela deve ser determinística;
- uma expressão deve ter no máximo uma coerção final por contexto;
- argumentos de chamada podem registrar coerções próprias;
- retornos podem registrar coerções próprias;
- coerções inválidas não devem ser registradas como válidas;
- elementos com `Error` podem registrar coerção de erro.

---

## 23. Atribuições

Em atribuições, tipo do valor deve ser compatível com tipo do destino.

Regras:

- igualdade canônica é aceita;
- subtipo pode ser aceito quando seguro;
- upcast permitido deve ser registrado;
- coerção proibida deve produzir diagnóstico;
- atribuição não deve alterar mutabilidade ou autoridade de escrita;
- regras de escrita e capacidade pertencem às fases de memória e Domains.

---

## 24. Retornos

Em retornos, tipo da expressão retornada deve ser compatível com tipo da assinatura.

Regras:

- `return expr` deve comparar tipo de `expr` com retorno esperado;
- `return` sem expressão deve produzir `Unit`;
- retorno esperado `Unit` não aceita valor significativo sem regra explícita;
- upcast permitido deve ser registrado;
- incompatibilidade deve apontar para expressão de retorno e assinatura quando possível.

---

## 25. Chamadas

Em chamadas, cada argumento deve ser compatível com o parâmetro correspondente.

Regras:

- coerções de argumentos participam da aplicabilidade de candidato;
- candidato que exige coerção proibida não é aplicável;
- candidato sem coerções proibidas pode ser aplicável;
- se múltiplos candidatos forem aplicáveis, o pipeline deve aplicar regra determinística ou diagnosticar ambiguidade;
- coerções de argumentos selecionados devem ser registradas;
- coerções não devem esconder aridade incorreta.

---

## 26. Overload

Subtipagem e coerções podem influenciar seleção de overload.

Regras:

- candidato com correspondência exata deve ser preferível a candidato que exige coerção, quando o pipeline definir preferência;
- preferência deve ser documentada e determinística;
- se não houver ordem normativa entre coerções, empate deve gerar ambiguidade;
- overload não deve introduzir coerção não permitida;
- overload não deve escolher candidato com diagnóstico bloqueante.

Se overload não estiver habilitado no subconjunto inicial, múltiplos candidatos aplicáveis devem ser diagnosticados conforme `TYPE-CHECKING-PIPELINE.md`.

---

## 27. Diagnósticos

Diagnósticos de subtipagem e coerção devem ser precisos.

Situações mínimas:

- tipos incompatíveis;
- hierarquias incompatíveis;
- upcast impossível;
- downcast implícito proibido;
- conversão explícita inválida;
- coerção numérica implícita não permitida;
- `ObjectId<T>` usado com `T` inválido;
- conversão que alteraria Domain;
- conversão que alteraria identidade;
- conversão que exigiria `null`;
- ambiguidade causada por coerções igualmente aplicáveis.

Regras:

- diagnóstico deve informar tipo esperado e encontrado quando possível;
- diagnóstico deve apontar para o uso que exige compatibilidade;
- evidência de subtipo pode ser usada para mensagens;
- erro derivado de `Error` deve evitar cascata;
- diagnósticos devem ser determinísticos.

---

## 28. Recuperação

Subtipagem e coerções devem permitir continuidade controlada.

Regras:

- tipo `Error` deve ser tratado como compatível para evitar cascatas, quando apropriado;
- coerção de erro pode ser registrada para preservar forma da HIR tipada;
- incompatibilidade real deve produzir diagnóstico antes de recuperação;
- recuperação não deve criar evidência de subtipo falsa;
- recuperação não deve registrar upcast válido quando a relação não existe;
- estado final do pipeline deve refletir diagnósticos emitidos.

---

## 29. Determinismo e Dumps

Resultados de subtipagem e coerção devem ser determinísticos.

Regras:

- cadeias de herança devem ser percorridas em ordem estável;
- interfaces e traits devem ser ordenados por `SymbolId` ou ordem declarativa estável;
- coerções devem receber IDs determinísticos quando IDs existirem;
- dumps devem usar `TypeId`, `SymbolId` e IDs HIR;
- dumps não devem conter endereços de memória;
- decisões de overload envolvendo coerções devem ser reproduzíveis.

Formato conceitual:

```text
subtype #7 <: #3 via class #7 -> #5 -> #3
coercion expr #12 #8 -> #9 ObjectIdUpcast
```

---

## 30. Invariantes

As seguintes invariantes são obrigatórias:

- subtipagem usa `TypeId` internado;
- subtipagem nominal usa `SymbolId`;
- `T <: T` para todo tipo válido;
- herança de classes é simples;
- ciclos de herança não são aceitos como relações válidas;
- `ObjectId<Sub> <: ObjectId<Super>` somente quando `Sub <: Super`;
- upcast preserva identidade, Domain e lifetime;
- downcast não é implícito;
- generics são invariantes por padrão;
- coerção aplicada é registrada;
- conversão proibida nunca é tratada como compatível;
- decisões são determinísticas;
- subtipagem e coerções não dependem de layout, MIR, ABI ou backend.

Violação dessas invariantes deve ser tratada como erro de implementação.

---

## 31. Testes Obrigatórios

O Stage 4 deve conter testes que validem:

- reflexividade de tipos;
- subclasse aceita como superclasse;
- transitividade de herança;
- hierarquias incompatíveis rejeitadas;
- classe aceita como interface implementada;
- classe rejeitada como interface não implementada;
- `ObjectId<Sub>` aceito como `ObjectId<Super>`;
- `ObjectId<Super>` rejeitado como `ObjectId<Sub>` implícito;
- upcast registrado em `CoercionTable`;
- downcast implícito diagnosticado;
- conversão entre `ObjectId<T>` e inteiro rejeitada;
- `Unit` incompatível com valor significativo;
- primitivos incompatíveis sem regra explícita;
- generics invariantes por padrão;
- coerções de argumentos em chamadas registradas;
- ambiguidade de overload por coerções diagnosticada quando aplicável;
- dumps de coerções determinísticos.

Esses testes devem integrar a suíte semântica e contribuir para o critério do Documento 28: subtipagem respeita a especificação.

---

## 32. Critérios de Conclusão

Este documento é considerado atendido quando:

- existe serviço de consulta de subtipagem usando `TypeId`;
- relações nominais de classes, interfaces e traits suportadas são representáveis;
- `ObjectId<T>` participa de upcast seguro;
- coerções implícitas permitidas são registradas;
- conversões explícitas e proibidas são diferenciadas;
- atribuições, retornos e chamadas usam compatibilidade definida aqui;
- diagnósticos cobrem incompatibilidades do subconjunto inicial;
- testes demonstram determinismo e preservação de identidade, Domain e lifetime.
