# Type Inference

**Projeto:** Linguagem Capi  
**Documento:** TYPE-INFERENCE  
**Status:** Aprovado  
**Stage:** Stage 4 — Sistema de tipos  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o contrato de engenharia da inferência de tipos da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- qual entrada a inferência consome;
- qual saída a inferência produz;
- quais elementos HIR são tipáveis;
- como tipos explícitos e inferidos são combinados;
- como variáveis de inferência e restrições são representadas;
- como a inferência usa `TypeInterner` e `TypeId`;
- como a HIR é enriquecida com tipos;
- como erros de inferência são recuperados;
- quais responsabilidades pertencem a documentos adjacentes;
- quais invariantes e testes validam a fase.

Inferência de tipos é o processo de determinar tipos ausentes ou parcialmente conhecidos a partir de anotações explícitas, símbolos resolvidos, assinaturas, expressões e restrições do sistema de tipos.

---

## 2. Escopo

Este documento cobre:

- entrada e saída da inferência;
- identificação de elementos tipáveis;
- origem de tipos explícitos;
- criação de variáveis de inferência;
- geração de restrições;
- propagação de tipo esperado;
- síntese de tipo produzido;
- unificação ou mecanismo equivalente;
- materialização de `TypeId` internado;
- inferência de declarações locais;
- inferência de expressões;
- inferência de chamadas;
- inferência de blocos;
- inferência de retornos;
- tratamento de `Unknown` e `Error`;
- enriquecimento da HIR tipada;
- diagnósticos de inferência;
- determinismo e testes.

Este documento não cobre:

- modelo completo de tipos;
- implementação de interning;
- pipeline completo de type checking;
- regras detalhadas de subtipagem e coerções;
- seleção completa de overload;
- implementação completa de generics;
- ownership e borrow checking;
- análise de regiões;
- regras operacionais de Domains;
- validação de layout de objetos;
- MIR;
- ABI;
- geração de código.

Esses temas pertencem a:

- `TYPE-MODEL.md`;
- `TYPE-INTERNING.md`;
- `TYPE-CHECKING-PIPELINE.md`;
- `SUBTYPING-AND-COERCIONS.md`;
- `GENERICS-IMPLEMENTATION.md`;
- `OWNERSHIP-MODEL.md`;
- `REGION-ANALYSIS.md`;
- `DOMAIN-IMPLEMENTATION.md`;
- `MIR-LOWERING.md`;
- `ABI-IMPLEMENTATION.md`.

---

## 3. Posição no Stage 4

No Stage 4, a inferência de tipos sucede a resolução de nomes e usa os tipos internos definidos por `TYPE-MODEL.md`.

Fluxo conceitual:

```text
HIR com nomes resolvidos
    ↓
TypeInterner com built-ins
    ↓
Coleta de anotações explícitas
    ↓
Criação de variáveis de inferência
    ↓
Geração de restrições
    ↓
Resolução das restrições
    ↓
Materialização de TypeId
    ↓
TypedHirMap / HIR tipada
```

A inferência não substitui a verificação de tipos. Ela determina tipos e registra restrições ou resultados que serão validados pelo pipeline de type checking.

---

## 4. Princípios

A inferência de tipos deve seguir estes princípios:

- inferir tipos, não resolver nomes;
- preservar a estrutura conceitual da HIR;
- usar `SymbolId` e bindings já resolvidos;
- usar `TypeId` internado como identidade de tipo final;
- distinguir tipo declarado de tipo inferido;
- nunca escolher solução arbitrária para restrições incompatíveis;
- produzir resultados determinísticos;
- preferir diagnósticos localizados a falhas globais;
- permitir continuidade controlada por tipos de erro;
- não aplicar coerções implícitas fora das regras do pipeline;
- não antecipar ownership, regiões, Domains operacionais, MIR, backend ou ABI;
- deixar verificações não tipadas para a etapa semântica apropriada.

---

## 5. Terminologia

| Termo | Significado |
| --- | --- |
| Tipo esperado | Tipo imposto pelo contexto sobre uma expressão ou declaração. |
| Tipo produzido | Tipo sintetizado por uma expressão ou construção. |
| Tipo declarado | Tipo escrito pelo programador e resolvido para `TypeId`. |
| Tipo inferido | Tipo determinado pelo compilador sem anotação explícita direta. |
| Variável de inferência | Placeholder temporário usado até a determinação do tipo final. |
| Restrição | Relação que deve ser satisfeita entre tipos ou variáveis de inferência. |
| Solução | Associação final entre variáveis de inferência e `TypeId`. |
| Materialização | Substituição de variáveis resolvidas por tipos internados finais. |
| Contexto de inferência | Estado corrente usado ao visitar HIR e acumular restrições. |

Regras:

- `Unknown` em `TYPE-MODEL.md` pode representar variável ou estado temporário de inferência;
- `TypeId` final deve vir do `TypeInterner`;
- inferência deve preservar origem para diagnósticos e ferramentas.

---

## 6. Entidades Conceituais

| Entidade | Responsabilidade |
| --- | --- |
| `TypeInferencer` | Coordena a inferência sobre HIR resolvida. |
| `InferenceInput` | Entrada composta por HIR, símbolos, tipos e opções. |
| `InferenceOutput` | Resultado com mapas tipados, solução e diagnósticos. |
| `InferenceContext` | Estado mutável da inferência. |
| `InferVarId` | Identidade de variável de inferência. |
| `InferenceVar` | Placeholder com origem, estado e restrições associadas. |
| `TypeConstraint` | Relação tipada a ser satisfeita. |
| `ConstraintSet` | Conjunto determinístico de restrições. |
| `ConstraintSolver` | Resolve ou reduz restrições. |
| `InferenceSolution` | Mapeia `InferVarId` para `TypeId` ou erro. |
| `ExpectedType` | Tipo imposto pelo contexto. |
| `TypedHirMap` | Associação final entre elementos HIR e `TypeId`. |
| `InferenceDiagnostic` | Diagnóstico produzido pela inferência. |

Os nomes concretos podem variar na implementação. O contrato obrigatório é a presença dessas responsabilidades.

---

## 7. Entradas

Entrada conceitual:

```rust
pub struct InferenceInput<'a> {
    pub hir: &'a Hir,
    pub symbols: &'a SymbolTable,
    pub bindings: &'a NameBindingTable,
    pub types: &'a mut TypeInterner,
    pub options: InferenceOptions,
}
```

Regras:

- a HIR deve ter sido construída por lowering válido;
- referências de nomes necessárias à tipagem devem estar resolvidas;
- símbolos duplicados ou ambíguos devem estar diagnosticados antes da inferência;
- built-ins devem estar registrados no `TypeInterner`;
- diagnósticos anteriores devem estar disponíveis para bloqueio quando necessário;
- a inferência não deve consultar tokens diretamente;
- a inferência não deve modificar a AST;
- a inferência não deve depender de MIR.

Se a implementação armazenar bindings diretamente na HIR, `NameBindingTable` pode ser substituída por acesso equivalente.

---

## 8. Saídas

Saída conceitual:

```rust
pub struct InferenceOutput {
    pub typed_hir: TypedHirMap,
    pub solution: InferenceSolution,
    pub constraints: ConstraintSet,
    pub diagnostics: Vec<Diagnostic>,
    pub state: InferenceState,
}
```

Estado conceitual:

```rust
pub enum InferenceState {
    Complete,
    CompleteWithErrors,
    Blocked,
}
```

Regras:

- elementos HIR tipáveis válidos devem possuir `TypeId`;
- variáveis de inferência resolvidas devem apontar para `TypeId` internado;
- variáveis não resolvidas em saída final exigem diagnóstico ou estado `Blocked`;
- erros recuperáveis devem produzir tipo `Error` ou marcador equivalente;
- a saída deve distinguir tipos declarados de tipos inferidos quando aplicável;
- ausência de diagnóstico para inferência impossível é erro de implementação.

---

## 9. Elementos Tipáveis

A inferência deve considerar todos os elementos HIR que produzem, recebem ou transportam tipos.

Elementos mínimos:

- itens de tipo;
- funções;
- métodos;
- construtores;
- parâmetros;
- campos;
- declarações locais;
- padrões;
- blocos;
- comandos de retorno;
- expressões literais;
- expressões de nome;
- expressões de chamada;
- expressões de acesso a membro;
- expressões de atribuição;
- expressões de controle de fluxo quando produzirem valor;
- referências sintáticas de tipo.

Regras:

- cada elemento tipável deve ter origem HIR estável;
- elementos inválidos devem receber tipo `Error` ou diagnóstico associado;
- elementos que não produzem valor significativo devem usar `Unit` quando forem expressões ou retornos tipados;
- a lista concreta pode crescer com a linguagem, mas não deve violar as invariantes deste documento.

---

## 10. Tipos Explícitos

Tipos explícitos são anotações escritas pelo programador e resolvidas para `TypeId`.

Fontes comuns:

- tipo de parâmetro;
- tipo de retorno;
- tipo de campo;
- tipo de variável local;
- argumento genérico explícito;
- assinatura de função, método ou construtor.

Regras:

- referência de tipo explícita deve estar resolvida para símbolo ou built-in antes de virar `TypeId`;
- tipo explícito inválido deve produzir diagnóstico e tipo `Error`;
- tipo explícito deve prevalecer como restrição sobre o valor associado;
- spans de anotações explícitas devem ser preservados para diagnóstico;
- inferência não deve alterar o tipo declarado para satisfazer expressão incompatível.

---

## 11. Variáveis de Inferência

Variáveis de inferência representam tipos ainda não determinados.

Contrato conceitual:

```rust
pub struct InferVarId(u32);

pub struct InferenceVar {
    pub id: InferVarId,
    pub origin: InferenceOrigin,
    pub kind: InferenceVarKind,
    pub state: InferenceVarState,
}
```

Regras:

- cada variável deve possuir identidade própria;
- variáveis devem ser alocadas em ordem determinística;
- variáveis não devem ser confundidas com `TypeId` final;
- variáveis podem ser representadas como `Unknown::InferenceVar`;
- variáveis distintas não devem ser internadas como o mesmo tipo final antes de solução;
- toda variável deve possuir origem HIR ou origem sintética rastreável;
- toda variável deve ser resolvida, diagnosticada ou marcada como bloqueada ao fim da inferência.

---

## 12. Restrições

Restrições expressam relações necessárias entre tipos, variáveis e contextos.

Contrato conceitual:

```rust
pub enum TypeConstraint {
    Equals(TypeTerm, TypeTerm),
    Assignable {
        expected: TypeTerm,
        actual: TypeTerm,
    },
    Callable {
        callee: HirExprId,
        args: Vec<HirExprId>,
    },
    MemberAccess {
        receiver: TypeTerm,
        member: SymbolId,
    },
    Return {
        expected: TypeTerm,
        actual: TypeTerm,
    },
}
```

Regras:

- restrições devem preservar origem para diagnósticos;
- restrições devem ser acumuladas em ordem determinística;
- igualdade exige mesmo tipo final;
- atribuição pode depender de compatibilidade, subtipagem ou coerção permitida pelo pipeline;
- restrições de chamada devem preservar argumentos e assinatura candidata;
- restrições incompatíveis não devem ser descartadas silenciosamente;
- detalhes de subtipagem e coerção pertencem a `SUBTYPING-AND-COERCIONS.md`.

---

## 13. Termos de Tipo

Restrições devem operar sobre termos que podem ser tipos finais ou variáveis.

Contrato conceitual:

```rust
pub enum TypeTerm {
    Known(TypeId),
    Var(InferVarId),
    Error,
}
```

Regras:

- `Known` sempre aponta para tipo internado;
- `Var` aponta para variável de inferência viva;
- `Error` representa recuperação;
- termos devem poder ser materializados para `TypeId` final ou erro;
- termos não devem carregar spans como parte de sua identidade.

---

## 14. Fluxo Bidirecional

A inferência pode usar propagação bidirecional: tipos esperados fluem do contexto para subexpressões, e tipos produzidos fluem das subexpressões para o contexto.

Contrato conceitual:

```text
infer_expr(expr, expected) -> TypeTerm
```

Regras:

- quando existir tipo esperado, ele deve gerar restrição sobre o tipo produzido;
- quando não existir tipo esperado, a expressão deve sintetizar seu tipo;
- anotações explícitas criam tipo esperado forte;
- blocos, retornos e chamadas devem propagar expectativas quando aplicável;
- a ausência de tipo esperado pode criar variável de inferência;
- a presença de tipo esperado não autoriza coerção fora das regras do pipeline.

---

## 15. Declarações Locais

Declarações locais podem possuir tipo explícito, inicializador ou ambos.

Regras:

- local com tipo explícito e inicializador deve restringir o inicializador ao tipo declarado;
- local com tipo explícito e sem inicializador recebe tipo declarado quando a linguagem permitir;
- local sem tipo explícito e com inicializador infere tipo a partir do inicializador;
- local sem tipo explícito e sem inicializador deve produzir diagnóstico, salvo construção permitida pela linguagem;
- o tipo final da declaração local deve ser registrado em `TypedHirMap`;
- shadowing já deve ter sido tratado pela resolução de nomes.

---

## 16. Parâmetros e Campos

Parâmetros e campos normalmente exigem tipo explícito no subconjunto inicial.

Regras:

- parâmetro com tipo explícito deve registrar esse `TypeId`;
- parâmetro sem tipo quando não permitido deve produzir diagnóstico;
- campo com tipo explícito deve registrar esse `TypeId`;
- campo sem tipo quando não permitido deve produzir diagnóstico;
- usos de parâmetro ou campo devem consultar o tipo registrado;
- inferência de corpo não deve alterar assinatura pública já declarada.

Se uma versão futura permitir inferência em parâmetros ou campos, a regra deve ser formalizada antes de implementação.

---

## 17. Funções, Métodos e Retornos

Funções e métodos devem possuir assinatura tipada antes da inferência completa do corpo.

Regras:

- tipos de parâmetros entram no contexto do corpo;
- tipo de retorno explícito cria expectativa para comandos de retorno;
- ausência de retorno explícito deve usar `Unit` quando a linguagem permitir;
- cada `return expr` deve gerar restrição entre retorno esperado e tipo de `expr`;
- `return` sem expressão deve produzir `Unit`;
- corpo sem `return` significativo deve ser compatível com `Unit`;
- métodos devem considerar tipo do receiver na inferência de acessos e chamadas.

Inferência não deve validar regras de controle de fluxo que não sejam necessárias para tipagem.

---

## 18. Expressões Literais

Literais possuem tipos conhecidos ou inferíveis a partir do contexto.

Regras:

- literal booleano deve produzir `Bool`;
- literal de caractere deve produzir `Char`;
- literal inteiro pode produzir tipo inteiro padrão ou variável restringida por contexto;
- literal de ponto flutuante pode produzir tipo flutuante padrão ou variável restringida por contexto;
- literal string, quando existir no subconjunto, deve mapear para tipo definido pela biblioteca ou produzir erro se ainda indisponível;
- ausência explícita de valor não deve produzir `Null`;
- `Unit` deve ser produzido por construções sem valor significativo.

Tipos padrão de literais devem ser definidos de forma determinística pela implementação ou por documento normativo aplicável.

---

## 19. Expressões de Nome

Expressões de nome usam bindings produzidos pela resolução de nomes.

Regras:

- nome resolvido para local deve produzir tipo do local;
- nome resolvido para parâmetro deve produzir tipo do parâmetro;
- nome resolvido para função pode produzir assinatura ou entidade chamável;
- nome resolvido para tipo em contexto de valor deve produzir diagnóstico, salvo construção permitida;
- nome não resolvido deve reutilizar diagnóstico de resolução e produzir `Error`;
- inferência não deve procurar símbolos por texto.

---

## 20. Chamadas

Chamadas combinam tipo do callee, argumentos e tipo esperado pelo contexto.

Regras:

- callee deve ser resolvido para função, método, construtor ou entidade chamável equivalente;
- argumentos devem ser inferidos em ordem determinística;
- cada argumento deve gerar restrição com o parâmetro correspondente;
- quantidade incorreta de argumentos deve produzir diagnóstico;
- retorno da chamada deve ser o tipo de retorno da assinatura selecionada;
- tipo esperado pelo contexto pode restringir seleção quando o pipeline permitir;
- seleção completa de overload pertence a `TYPE-CHECKING-PIPELINE.md`;
- coerções de argumento pertencem a `SUBTYPING-AND-COERCIONS.md`.

Quando a seleção de chamada estiver bloqueada, a expressão deve receber tipo `Error` ou variável bloqueada diagnosticada.

---

## 21. Acesso a Membros

Acesso a membro depende do tipo do receiver e do símbolo do membro.

Regras:

- receiver deve ser inferido antes de determinar o tipo do membro;
- acesso a campo produz tipo do campo;
- acesso a método produz assinatura ou entidade chamável;
- acesso inválido deve produzir diagnóstico;
- acesso por `ObjectId<T>` deve respeitar as regras de observação de `T`;
- validações de visibilidade, mutação e dispatch podem depender de fases posteriores ou do pipeline;
- inferência deve registrar tipo resultante quando o acesso for tipável.

---

## 22. Atribuições

Atribuições impõem compatibilidade entre destino e valor.

Regras:

- destino deve possuir tipo esperado;
- valor deve ser inferido com tipo esperado do destino quando aplicável;
- restrição `Assignable` deve ser gerada entre destino e valor;
- tipo produzido pela atribuição deve ser `Unit`, salvo regra da linguagem em sentido diferente;
- atribuição a destino não atribuível deve ser diagnosticável;
- regras de mutação e capacidade de escrita pertencem ao pipeline semântico e documentos de memória.

---

## 23. Blocos e Controle de Fluxo

Blocos e construções de controle devem possuir tipo quando participarem de expressão.

Regras:

- bloco sem valor final significativo deve produzir `Unit`;
- bloco com expressão final deve produzir o tipo dessa expressão;
- comandos internos podem gerar restrições próprias;
- ramos de controle que produzem valor devem gerar restrição para tipo comum ou esperado;
- incompatibilidade entre ramos deve produzir diagnóstico;
- análise exaustiva de controle de fluxo só pertence à inferência quando necessária para determinar tipo.

---

## 24. Padrões

Padrões recebem tipo esperado do valor que estão decompondo.

Regras:

- binding introduzido por padrão deve receber tipo derivado do padrão;
- padrão incompatível com tipo esperado deve produzir diagnóstico;
- padrões compostos devem propagar tipo esperado para subpadrões;
- duplicidade de bindings de padrão pertence à resolução de nomes, mas o tipo final pertence à inferência;
- padrões inválidos devem receber marcador `Error` para continuidade.

---

## 25. Generics

Inferência pode preencher argumentos genéricos quando o subconjunto permitir.

Regras:

- parâmetros genéricos explícitos devem virar `GenericParamId` ou `TypeId` conforme o modelo;
- argumentos genéricos explícitos devem ser internados antes do uso;
- argumentos genéricos omitidos podem criar variáveis de inferência;
- constraints de generics devem ser registradas como restrições;
- falha em determinar argumento genérico deve produzir diagnóstico;
- substituição, bounds, variância e estratégia de implementação pertencem a `GENERICS-IMPLEMENTATION.md`.

---

## 26. Resolução de Restrições

A implementação pode usar unificação, propagação, solver de restrições ou estratégia equivalente.

Regras:

- o algoritmo escolhido deve produzir resultado funcionalmente equivalente para programas válidos;
- resolução deve ser determinística;
- restrições devem ser processadas até ponto fixo ou estado conclusivo equivalente;
- conflito entre tipo conhecido e variável resolvida deve produzir diagnóstico;
- conflito entre tipos conhecidos deve produzir diagnóstico de incompatibilidade;
- variáveis sem solução devem produzir diagnóstico quando forem necessárias à HIR final;
- soluções parciais só podem permanecer em estado `Blocked` ou recuperação explícita.

---

## 27. Materialização

Após resolver restrições, a inferência deve materializar tipos finais.

Regras:

- cada variável resolvida deve apontar para `TypeId` internado;
- tipos compostos produzidos pela solução devem passar pelo `TypeInterner`;
- `Unknown` não deve permanecer em elementos tipáveis válidos;
- elementos com erro devem usar tipo `Error` internado ou marcador equivalente;
- `TypedHirMap` deve ser preenchido de forma completa para elementos tipáveis;
- origem `Declared` ou `Inferred` deve ser preservada quando aplicável.

---

## 28. Diagnósticos

Diagnósticos de inferência devem ser estruturados e rastreáveis.

Situações mínimas:

- tipo não pôde ser inferido;
- anotação explícita incompatível com inicializador;
- retorno incompatível com assinatura;
- argumento incompatível com parâmetro;
- ramos de controle com tipos incompatíveis;
- chamada sem assinatura aplicável;
- membro inexistente para tipo do receiver;
- argumento genérico não inferível;
- restrições genéricas incompatíveis;
- uso de tipo de erro proveniente de fase anterior, quando relevante.

Regras:

- diagnóstico deve apontar para span primário relevante;
- tipos esperado e encontrado devem ser exibidos quando possível;
- origem da restrição deve ser preservada;
- erros em cascata devem ser reduzidos por `Error`;
- diagnóstico de inferência não deve duplicar diagnóstico de resolução de nomes.

---

## 29. Recuperação

A inferência deve continuar após erros quando isso puder ser feito com segurança.

Regras:

- expressão com erro pode produzir `Error`;
- local com inicializador inválido pode receber `Error`;
- chamada inválida pode produzir `Error`;
- restrições envolvendo `Error` não devem gerar diagnósticos redundantes;
- recuperação não deve criar tipos válidos falsos;
- estado `CompleteWithErrors` deve ser usado quando a HIR tipada contém marcadores de erro;
- estado `Blocked` deve ser usado quando a análise não puder prosseguir com segurança.

---

## 30. Determinismo e Dumps

Inferência deve produzir resultados determinísticos.

Regras:

- traversal da HIR deve seguir ordem estável;
- variáveis de inferência devem ser alocadas em ordem estável;
- restrições devem ter ordenação estável;
- solução deve ser estável para a mesma entrada;
- dumps não devem conter endereços de memória;
- dumps devem imprimir tipos por `TypeId` canônico;
- variáveis não resolvidas em estado parcial devem ser impressas explicitamente.

Formato conceitual:

```text
inference:
  var ?0 origin local #3 = #2
  constraint #0 assign expected #2 actual ?0
  expr #8 : #2 inferred
  local #3 : #2 inferred
```

O formato final pode variar, desde que seja estável e útil para testes.

---

## 31. Invariantes

As seguintes invariantes são obrigatórias:

- inferência consome HIR com nomes resolvidos;
- inferência não cria `SymbolId`;
- inferência não modifica AST;
- todo `TypeId` final vem do `TypeInterner`;
- todo elemento HIR tipável válido possui tipo final;
- tipo declarado e tipo inferido permanecem distinguíveis;
- toda variável de inferência é resolvida, diagnosticada ou bloqueada;
- restrições incompatíveis possuem diagnóstico;
- `Unknown` não aparece na HIR final sem estado parcial explícito;
- `Error` é usado para recuperação controlada;
- resultados são determinísticos;
- inferência não depende de MIR, backend, ABI ou layout físico.

Violação dessas invariantes deve ser tratada como erro de implementação.

---

## 32. Testes Obrigatórios

O Stage 4 deve conter testes que validem:

- inferência de local a partir de literal;
- tipo explícito de local preservado;
- erro em inicializador incompatível com tipo explícito;
- inferência de expressão de nome a partir de parâmetro;
- inferência de chamada simples;
- erro de quantidade incorreta de argumentos;
- erro de argumento incompatível;
- inferência de retorno compatível com assinatura;
- erro de retorno incompatível;
- bloco sem expressão final produzindo `Unit`;
- bloco com expressão final produzindo tipo da expressão;
- ramos de controle incompatíveis diagnosticados;
- `ObjectId<T>` preservado como tipo construído;
- argumento genérico simples inferido quando permitido;
- falha de inferência genérica diagnosticada;
- ausência de `Unknown` não diagnosticado na HIR tipada;
- dump de inferência determinístico.

Esses testes devem integrar a suíte semântica e contribuir para o critério do Documento 28: `capic check arquivo.capi`.

---

## 33. Critérios de Conclusão

Este documento é considerado atendido quando:

- existe uma fase de inferência operando sobre HIR resolvida;
- tipos explícitos são convertidos para `TypeId`;
- variáveis de inferência e restrições são representadas;
- elementos HIR tipáveis recebem tipos finais ou erro recuperável;
- resultados são registrados em `TypedHirMap` ou estrutura equivalente;
- diagnósticos cobrem falhas de inferência do subconjunto inicial;
- dumps e testes demonstram determinismo;
- os documentos de pipeline, generics, subtipagem e coerções conseguem usar este contrato sem redefinir inferência.
