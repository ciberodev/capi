# Generics Implementation

**Projeto:** Linguagem Capi  
**Documento:** GENERICS-IMPLEMENTATION  
**Status:** Aprovado  
**Stage:** Stage 4 — Sistema de tipos  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o contrato de engenharia para implementação de generics no subconjunto inicial da Linguagem Capi.

Seu objetivo é estabelecer:

- como parâmetros genéricos são representados;
- como argumentos genéricos são coletados, inferidos e validados;
- como tipos genéricos são internados;
- como bounds e constraints participam do type checking;
- como substituições genéricas são aplicadas;
- como `Optional<T>` e `Result<T, E>` participam do sistema de tipos;
- como variância é tratada no Stage 4;
- como generics interagem com chamadas, overload, subtipagem e coerções;
- quais diagnósticos devem ser produzidos;
- quais invariantes e testes validam a implementação.

Generics permitem definir tipos e assinaturas parametrizadas preservando segurança estática. No Stage 4, a implementação deve fornecer o subconjunto necessário para representar, verificar e usar tipos parametrizados sem definir ainda uma estratégia final de monomorfização, layout ou codegen.

---

## 2. Escopo

Este documento cobre:

- parâmetros genéricos;
- argumentos genéricos explícitos;
- argumentos genéricos inferidos;
- aplicações genéricas;
- bounds e constraints;
- substituição de parâmetros por argumentos;
- contexto genérico;
- generics em tipos;
- generics em funções, métodos e construtores;
- `Optional<T>`;
- `Result<T, E>`;
- variância inicial;
- diagnósticos;
- determinismo e testes.

Este documento não cobre:

- modelo completo de tipos;
- implementação do interner;
- algoritmo completo de inferência;
- pipeline completo de type checking;
- regras completas de subtipagem e coerção;
- monomorfização final;
- compartilhamento de código genérico;
- layout de instâncias genéricas;
- ABI de tipos genéricos;
- vtables genéricas;
- especialização otimizada;
- ownership;
- regiões;
- Domains em nível operacional;
- MIR;
- backend.

Esses temas pertencem a:

- `TYPE-MODEL.md`;
- `TYPE-INTERNING.md`;
- `TYPE-INFERENCE.md`;
- `TYPE-CHECKING-PIPELINE.md`;
- `SUBTYPING-AND-COERCIONS.md`;
- `OBJECT-LAYOUT.md`;
- `MIR-LOWERING.md`;
- `ABI-IMPLEMENTATION.md`;
- `OWNERSHIP-MODEL.md`;
- `REGION-ANALYSIS.md`;
- `DOMAIN-IMPLEMENTATION.md`.

---

## 3. Posição no Stage 4

Generics participam do Stage 4 como parte do sistema de tipos.

Fluxo conceitual:

```text
Declaração genérica
    ↓
Registro de GenericParamId
    ↓
Coleta de bounds
    ↓
Uso com argumentos explícitos ou inferidos
    ↓
Validação de aridade e constraints
    ↓
Substituição
    ↓
Interning de GenericInstance
    ↓
HIR tipada
```

Generics do Stage 4 devem ser suficientes para `capic check arquivo.capi` validar programas tipados que usam o subconjunto inicial.

---

## 4. Princípios

A implementação de generics deve seguir estes princípios:

- parâmetros genéricos possuem identidade própria;
- nome textual de parâmetro não é identidade semântica;
- argumentos genéricos finais devem ser `TypeId` internados;
- aplicações genéricas devem ser determinísticas;
- bounds devem ser verificados antes de considerar a aplicação válida;
- generics são invariantes por padrão;
- exceções de variância exigem regra explícita;
- `Optional<T>` representa ausência de valor;
- ausência de valor não deve ser representada por `null`;
- `Result<T, E>` representa sucesso ou falha esperada;
- substituição não deve modificar declaração original;
- generics não devem depender de layout, MIR, ABI ou backend no Stage 4;
- erros devem ser recuperáveis sem criar tipos válidos falsos.

---

## 5. Terminologia

| Termo | Significado |
| --- | --- |
| Declaração genérica | Declaração que introduz parâmetros de tipo. |
| Parâmetro genérico | Placeholder declarado, como `T`. |
| Argumento genérico | Tipo fornecido ou inferido para um parâmetro. |
| Aplicação genérica | Uso de declaração genérica com argumentos. |
| Bound | Restrição declarada sobre parâmetro genérico. |
| Constraint | Relação que deve ser satisfeita durante type checking. |
| Substituição | Mapeamento de parâmetro genérico para argumento concreto. |
| Instância genérica | Tipo ou assinatura resultante de uma aplicação. |
| Variância | Relação entre subtipagem de argumentos e subtipagem de instâncias. |
| Aridade genérica | Quantidade de parâmetros ou argumentos genéricos. |

Regras:

- `GenericParamId` identifica parâmetro;
- `TypeArgument` representa argumento;
- `GenericInstance` representa aplicação;
- `TypeId` identifica tipo final internado.

---

## 6. Entidades Conceituais

| Entidade | Responsabilidade |
| --- | --- |
| `GenericParamId` | Identidade opaca de parâmetro genérico. |
| `GenericParam` | Dados de um parâmetro genérico. |
| `GenericParamList` | Lista ordenada de parâmetros de uma declaração. |
| `GenericArgument` | Argumento explícito, inferido ou de erro. |
| `GenericArgumentList` | Lista ordenada de argumentos em uma aplicação. |
| `GenericBound` | Restrição declarada sobre parâmetro. |
| `GenericConstraint` | Restrição a ser validada pelo pipeline. |
| `GenericContext` | Escopo semântico de parâmetros genéricos disponíveis. |
| `GenericSubstitution` | Mapeamento de parâmetros para argumentos finais. |
| `GenericInstance` | Aplicação genérica internável. |
| `GenericChecker` | Valida aridade, bounds e substituições. |
| `GenericDiagnostic` | Diagnóstico relacionado a generics. |

Os nomes concretos podem variar na implementação. O contrato obrigatório é a presença dessas responsabilidades.

---

## 7. Parâmetros Genéricos

Parâmetros genéricos devem possuir identidade própria.

Contrato conceitual:

```rust
pub struct GenericParamId(u32);

pub struct GenericParam {
    pub id: GenericParamId,
    pub owner: SymbolId,
    pub name: SymbolName,
    pub bounds: Vec<GenericBound>,
    pub default: Option<TypeId>,
}
```

Regras:

- `GenericParamId` deve ser opaco;
- parâmetros devem ser alocados em ordem declarativa;
- dois parâmetros chamados `T` em owners diferentes devem ter IDs diferentes;
- `owner` deve apontar para declaração genérica;
- `name` é usado para diagnóstico, não para identidade global;
- bounds devem preservar origem para diagnóstico;
- defaults só devem existir se a linguagem permitir no subconjunto.

---

## 8. Owners Genéricos

Um owner genérico é a declaração que introduz parâmetros genéricos.

Owners possíveis:

- tipo nominal;
- função;
- método;
- construtor;
- interface;
- trait;
- alias de tipo, quando existir.

Regras:

- cada owner deve possuir lista de parâmetros ordenada;
- parâmetros de owner externo podem estar disponíveis para membros internos conforme regras de escopo;
- parâmetros de função não devem vazar para fora da assinatura ou corpo;
- parâmetros de tipo podem estar disponíveis em membros do tipo;
- conflitos de nomes de parâmetros no mesmo owner devem ser diagnosticados.

---

## 9. Contexto Genérico

`GenericContext` define quais parâmetros estão disponíveis em determinado ponto da HIR.

Contrato conceitual:

```rust
pub struct GenericContext {
    pub owner_stack: Vec<SymbolId>,
    pub params: Vec<GenericParamId>,
}
```

Regras:

- contexto deve ser derivado de HIR e símbolos resolvidos;
- ordem dos parâmetros deve ser determinística;
- lookup de parâmetro genérico deve respeitar escopos;
- shadowing de parâmetro genérico só deve ser permitido se a linguagem permitir explicitamente;
- contexto genérico não deve substituir `ScopeGraph`.

---

## 10. Bounds

Bounds restringem os argumentos aceitos por parâmetros genéricos.

Contrato conceitual:

```rust
pub enum GenericBound {
    Implements(TypeId),
    SubtypeOf(TypeId),
    Trait(TypeId),
}
```

Regras:

- bound deve referenciar tipo ou contrato válido;
- bound inválido deve produzir diagnóstico;
- bound deve ser convertido para constraint no type checking;
- múltiplos bounds devem ser todos satisfeitos;
- bound não satisfeito deve rejeitar aplicação genérica;
- bounds não devem ser ignorados por inferência;
- detalhes de sintaxe pertencem à especificação de sintaxe.

---

## 11. Argumentos Genéricos

Argumentos genéricos podem ser explícitos ou inferidos.

Contrato conceitual:

```rust
pub enum GenericArgument {
    Explicit(TypeId),
    Inferred(InferVarId),
    Error,
}
```

Regras:

- argumento explícito deve ser convertido para `TypeId`;
- argumento inferido deve ser resolvido antes da instância final;
- argumento de erro deve permitir recuperação;
- ordem dos argumentos deve corresponder à ordem dos parâmetros;
- argumentos excedentes devem produzir diagnóstico;
- argumentos ausentes sem inferência ou default devem produzir diagnóstico.

---

## 12. Aridade

A aridade genérica deve ser validada antes da checagem de bounds.

Regras:

- número de argumentos explícitos não pode exceder número de parâmetros;
- argumentos ausentes podem ser inferidos quando o contexto permitir;
- argumentos ausentes podem usar default quando suportado;
- se não houver inferência nem default, a aplicação é inválida;
- diagnóstico de aridade deve indicar esperado e recebido;
- aridade inválida deve produzir tipo `Error` ou estado recuperável.

---

## 13. Aplicações Genéricas

Aplicações genéricas produzem tipos ou assinaturas parametrizadas por argumentos.

Contrato conceitual:

```rust
pub struct GenericInstance {
    pub base: SymbolId,
    pub args: Vec<TypeId>,
}
```

Regras:

- `base` deve apontar para declaração genérica válida;
- argumentos finais devem estar internados;
- aplicação deve ser internada por `TypeInterner`;
- `base` e ordem dos argumentos definem identidade da aplicação;
- duas aplicações equivalentes devem compartilhar `TypeId`;
- aplicação inválida deve produzir tipo `Error`.

---

## 14. Substituição

Substituição aplica argumentos finais no lugar dos parâmetros.

Contrato conceitual:

```rust
pub struct GenericSubstitution {
    pub owner: SymbolId,
    pub entries: Vec<(GenericParamId, TypeId)>,
}
```

Regras:

- cada parâmetro do owner deve ter no máximo uma entrada;
- entradas devem seguir ordem declarativa;
- substituição não deve modificar a declaração original;
- substituição deve ser aplicada recursivamente em tipos compostos;
- substituição deve preservar `TypeId` já internados quando não houver parâmetro;
- resultado de substituição que formar tipo novo deve passar pelo interner;
- ciclo de substituição deve ser diagnosticado ou tratado como erro interno conforme origem.

---

## 15. Inferência de Argumentos Genéricos

O pipeline pode inferir argumentos genéricos a partir de parâmetros, argumentos de chamada, receiver e tipo esperado.

Fontes:

- tipo de argumento de chamada;
- tipo de parâmetro da assinatura genérica;
- receiver de método;
- tipo de retorno esperado;
- anotações explícitas parciais;
- constraints de bounds.

Regras:

- inferência deve criar variáveis para argumentos omitidos;
- solução deve ser determinística;
- conflito entre fontes deve produzir diagnóstico;
- ausência de informação suficiente deve produzir diagnóstico;
- argumento inferido final deve ser `TypeId` internado;
- inferência não deve escolher tipo arbitrário para satisfazer bound;
- defaults, quando existirem, devem ser aplicados em ordem definida.

---

## 16. Funções e Métodos Genéricos

Funções e métodos genéricos possuem parâmetros próprios e podem usar parâmetros do owner externo.

Regras:

- assinatura deve ser construída com contexto genérico ativo;
- parâmetros genéricos da função devem ser distintos dos parâmetros do tipo owner;
- chamadas podem fornecer argumentos explícitos;
- chamadas podem inferir argumentos omitidos;
- resultado da chamada deve aplicar substituição na assinatura;
- overload deve considerar aplicação genérica somente após aridade e bounds mínimos;
- falha de inferência genérica torna candidato inaplicável ou produz diagnóstico conforme contexto.

---

## 17. Tipos Genéricos Nominais

Tipos nominais podem ser parametrizados.

Regras:

- declaração genérica nominal deve registrar seus parâmetros;
- uso do tipo deve fornecer, inferir ou defaultar argumentos conforme permitido;
- instância nominal deve ser internada como `GenericInstance`;
- membros do tipo devem ser vistos sob substituição da instância;
- `List<Int>` e `List<UInt>` são tipos distintos;
- `List<T>` dentro do owner deve preservar parâmetro até substituição.

---

## 18. Optional

`Optional<T>` representa ausência explícita de valor.

Regras:

- `Optional` deve ser modelado como tipo genérico ou construção equivalente;
- `Optional<T>` deve possuir exatamente um argumento;
- `T` deve ser tipo válido;
- ausência de valor deve usar `Optional<T>`, não `null`;
- acesso ao valor interno exige comprovação ou verificação explícita conforme regras semânticas;
- `Optional<Sub>` não é subtipo de `Optional<Super>` por padrão;
- `Optional<T>` deve ser internado de forma canônica.

---

## 19. Result

`Result<T, E>` representa sucesso ou falha esperada.

Regras:

- `Result` deve possuir dois argumentos quando suportado;
- `T` representa tipo de sucesso;
- `E` representa tipo de falha;
- ambos devem ser tipos válidos;
- casos de sucesso e falha devem permanecer explícitos no sistema de tipos;
- `Result<T, E>` é invariante por padrão;
- tratamento de exaustividade pertence à verificação semântica ou fases posteriores quando aplicável.

---

## 20. Variância

Generics são invariantes por padrão.

Regras:

- `G<A>` não é subtipo de `G<B>` apenas porque `A <: B`;
- covariância, contravariância e invariância explícitas exigem regra normativa;
- nenhuma exceção deve ser implementada sem documento aplicável;
- subtipagem de instâncias genéricas deve consultar política de variância;
- política padrão deve ser determinística e segura.

No Stage 4, a implementação deve assumir invariância salvo caso explicitamente aprovado por especificação ou RFC.

---

## 21. Constraints e Type Checking

Constraints genéricas devem ser integradas ao pipeline de type checking.

Contrato conceitual:

```rust
pub enum GenericConstraint {
    SatisfiesBound {
        param: GenericParamId,
        arg: TypeId,
        bound: GenericBound,
    },
}
```

Regras:

- cada bound gera constraint;
- constraints devem ser avaliadas após argumentos finais estarem disponíveis;
- constraint satisfeita permite a aplicação;
- constraint não satisfeita produz diagnóstico;
- constraint com `Error` deve permitir recuperação sem cascata;
- constraints devem ser determinísticas.

---

## 22. Integração com Subtipagem

Bounds podem depender de subtipagem e implementação de interfaces ou traits.

Regras:

- `SubtypeOf(B)` exige que argumento seja subtipo de `B`;
- `Implements(I)` exige implementação de interface aplicável;
- `Trait(T)` exige compatibilidade com trait quando suportada;
- consulta deve usar `SUBTYPING-AND-COERCIONS.md`;
- coerções não devem ser usadas para satisfazer bound salvo regra explícita;
- falha de bound deve preservar evidência ou motivo para diagnóstico.

---

## 23. Integração com Interning

Tipos genéricos finais devem ser internados.

Regras:

- `GenericParam` pode ter representação própria;
- `GenericInstance` deve usar `SymbolId` base e `TypeId` dos argumentos;
- substituições que produzirem tipos compostos devem chamar `TypeInterner`;
- aplicações equivalentes devem compartilhar `TypeId`;
- ordem de argumentos participa da chave;
- spans e origem diagnóstica não participam da identidade.

---

## 24. Integração com HIR

A HIR deve preservar informações suficientes para rastrear generics.

Regras:

- declarações genéricas devem apontar para lista de parâmetros;
- usos genéricos explícitos devem apontar para argumentos HIR;
- argumentos inferidos devem aparecer na saída tipada ou tabela auxiliar;
- `TypedHirMap` deve conter tipo final da aplicação;
- diagnósticos devem apontar para owner, argumento ou bound relevante;
- a HIR estrutural não deve ser reescrita por substituição.

---

## 25. Diagnósticos

Diagnósticos de generics devem ser estruturados.

Situações mínimas:

- parâmetro genérico duplicado;
- argumento genérico excedente;
- argumento genérico ausente;
- argumento genérico não inferível;
- bound inválido;
- bound não satisfeito;
- uso de tipo não genérico com argumentos;
- uso de tipo genérico sem argumentos quando não inferível;
- ciclo de alias ou substituição genérica;
- variância não suportada;
- aplicação de `Optional` com aridade incorreta;
- aplicação de `Result` com aridade incorreta.

Regras:

- diagnóstico deve informar aridade esperada e recebida quando aplicável;
- diagnóstico de bound deve informar argumento e restrição;
- diagnóstico deve preservar span primário;
- erro derivado de `Error` deve evitar cascata;
- diagnósticos devem ser determinísticos.

---

## 26. Recuperação

Generics devem permitir continuidade controlada após erro.

Regras:

- aplicação inválida pode produzir tipo `Error`;
- argumento inválido pode virar `GenericArgument::Error`;
- bound inválido pode bloquear apenas a aplicação afetada;
- aridade inválida não deve criar instância válida falsa;
- inferência genérica falha não deve escolher argumento arbitrário;
- recuperação deve preservar diagnósticos emitidos;
- pipeline final deve refletir erro em `CheckedWithErrors` ou `Blocked`.

---

## 27. Determinismo e Dumps

Generics devem produzir resultados determinísticos.

Regras:

- parâmetros devem ser listados em ordem declarativa;
- argumentos devem ser listados em ordem de parâmetro;
- substitutions devem ser serializadas em ordem de parâmetro;
- instâncias devem imprimir base e argumentos canônicos;
- constraints devem ser ordenadas de forma estável;
- dumps não devem conter endereços de memória.

Formato conceitual:

```text
generic params function #7: <T#0>
substitution call #12: T#0 -> type #2
instance Optional<#2> -> type #9
constraint T#0 implements #5 ok
```

---

## 28. Invariantes

As seguintes invariantes são obrigatórias:

- todo parâmetro genérico possui `GenericParamId`;
- todo owner genérico possui lista determinística de parâmetros;
- todo argumento final é `TypeId` ou `Error`;
- toda aplicação válida satisfaz aridade;
- toda aplicação válida satisfaz bounds;
- substituição não modifica declaração original;
- instância genérica válida é internada;
- generics são invariantes por padrão;
- `Optional<T>` não representa `null`;
- `Result<T, E>` mantém sucesso e falha explícitos;
- falha de inferência genérica possui diagnóstico;
- generics não dependem de layout, MIR, ABI ou backend.

Violação dessas invariantes deve ser tratada como erro de implementação.

---

## 29. Testes Obrigatórios

O Stage 4 deve conter testes que validem:

- declaração genérica com parâmetro único;
- rejeição de parâmetro genérico duplicado;
- aplicação genérica com aridade correta;
- rejeição de argumento excedente;
- rejeição de argumento ausente não inferível;
- inferência simples de argumento genérico em chamada;
- conflito de inferência genérica diagnosticado;
- bound satisfeito;
- bound não satisfeito diagnosticado;
- substituição em tipo de retorno;
- substituição em tipo de parâmetro;
- `Optional<Int>` internado de forma canônica;
- `Optional` com aridade incorreta diagnosticado;
- `Result<Int, ErrorType>` internado quando suportado;
- `Result` com aridade incorreta diagnosticado;
- invariância de `Optional<Sub>` e `Optional<Super>`;
- dumps de generics determinísticos.

Esses testes devem integrar a suíte semântica e contribuir para o critério do Documento 28: generics do subconjunto inicial implementados e todos os testes semânticos passam.

---

## 30. Critérios de Conclusão

Este documento é considerado atendido quando:

- parâmetros genéricos possuem identidade interna;
- owners genéricos registram parâmetros e bounds;
- argumentos explícitos e inferidos são validados;
- aridade e constraints são diagnosticadas;
- substituições são aplicadas de forma determinística;
- instâncias genéricas são internadas;
- `Optional<T>` e `Result<T, E>` do subconjunto são representáveis;
- variância padrão é invariância;
- HIR tipada preserva aplicações e argumentos finais;
- diagnósticos e dumps são determinísticos.
