# Type Checking Pipeline

**Projeto:** Linguagem Capi  
**Documento:** TYPE-CHECKING-PIPELINE  
**Status:** Aprovado  
**Stage:** Stage 4 — Sistema de tipos  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o pipeline de engenharia da inferência e verificação de tipos da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- a ordem das etapas de tipagem;
- quais artefatos cada etapa consome e produz;
- como `TYPE-MODEL.md`, `TYPE-INTERNING.md` e `TYPE-INFERENCE.md` são coordenados;
- onde subtipagem, coerções, generics, chamadas e overload participam;
- como a HIR é enriquecida;
- como diagnósticos de tipo são emitidos;
- quando a fase deve prosseguir, recuperar ou bloquear;
- quais invariantes e testes validam o pipeline.

O pipeline de type checking é a fase do frontend que transforma HIR com nomes resolvidos em HIR tipada e verificada quanto às regras do sistema de tipos aplicáveis ao Stage 4.

---

## 2. Escopo

Este documento cobre:

- pré-condições do type checking;
- sequência de etapas do Stage 4;
- inicialização de tipos built-in;
- coleta de declarações de tipo;
- conversão de referências de tipo;
- construção de assinaturas;
- inferência;
- resolução de restrições;
- verificação de compatibilidade;
- aplicação registrada de coerções;
- resolução de chamadas e overload aplicável;
- verificação de generics do subconjunto inicial;
- diagnósticos;
- enriquecimento final da HIR;
- critérios para `capic check arquivo.capi`.

Este documento não cobre:

- modelo interno completo de tipos;
- implementação do interner;
- detalhes algorítmicos da inferência;
- regras completas de subtipagem e coerção;
- implementação completa de generics;
- ownership e borrow checker;
- análise de regiões;
- validação operacional de Domains;
- layout de objetos;
- MIR;
- ABI;
- backend;
- runtime.

Esses temas pertencem a:

- `TYPE-MODEL.md`;
- `TYPE-INTERNING.md`;
- `TYPE-INFERENCE.md`;
- `SUBTYPING-AND-COERCIONS.md`;
- `GENERICS-IMPLEMENTATION.md`;
- `OWNERSHIP-MODEL.md`;
- `REGION-ANALYSIS.md`;
- `DOMAIN-IMPLEMENTATION.md`;
- `OBJECT-LAYOUT.md`;
- `MIR-MODEL.md`;
- `ABI-IMPLEMENTATION.md`.

---

## 3. Posição no Stage 4

O pipeline de type checking sucede a resolução de nomes do Stage 3.

Fluxo conceitual:

```text
HIR com nomes resolvidos
    ↓
Inicialização do TypeInterner
    ↓
Coleta de tipos declarados
    ↓
Conversão de TypeRefs
    ↓
Construção de assinaturas
    ↓
Inferência de tipos
    ↓
Verificação de restrições
    ↓
Resolução de chamadas, coerções e overload aplicável
    ↓
Validação de generics do subconjunto
    ↓
HIR tipada + diagnósticos
```

Resultado demonstrável do Stage 4:

```bash
capic check arquivo.capi
```

---

## 4. Princípios

O pipeline deve seguir estes princípios:

- operar sobre HIR com nomes resolvidos;
- preservar a estrutura conceitual da HIR;
- usar `TypeId` internado para tipos finais;
- separar coleta, inferência, verificação e materialização;
- produzir diagnósticos determinísticos;
- não escolher candidatos ambíguos arbitrariamente;
- registrar coerções aplicadas de forma explícita;
- distinguir tipo declarado, inferido e de erro;
- não repetir resolução de nomes;
- não antecipar borrow checking, regiões, layout, MIR, ABI ou codegen;
- permitir recuperação controlada após erros de usuário;
- bloquear apenas quando a análise não puder continuar com segurança.

---

## 5. Terminologia

| Termo | Significado |
| --- | --- |
| Pipeline de type checking | Sequência coordenada de etapas que produz HIR tipada. |
| HIR resolvida | HIR enriquecida por resolução de nomes do Stage 3. |
| HIR tipada | HIR ou tabela auxiliar com tipos finais associados a elementos tipáveis. |
| Coleta de tipos | Registro inicial de tipos declarados e built-ins. |
| Conversão de type refs | Transformação de `HirTypeRefId` resolvido em `TypeId`. |
| Assinatura tipada | Representação dos tipos de parâmetros, receiver e retorno. |
| Restrição | Relação gerada durante inferência e verificação. |
| Coerção | Conversão permitida registrada entre tipo produzido e tipo esperado. |
| Candidato de chamada | Função, método ou construtor compatível em análise. |
| Resultado de type checking | HIR tipada, diagnósticos, coerções e estado final. |

---

## 6. Entidades Conceituais

| Entidade | Responsabilidade |
| --- | --- |
| `TypeChecker` | Coordena o pipeline de tipagem. |
| `TypeCheckInput` | Entrada composta por HIR resolvida, símbolos, escopos e opções. |
| `TypeCheckOutput` | Resultado final do pipeline. |
| `TypeCheckSession` | Estado compartilhado da fase. |
| `TypeInterner` | Autoridade de tipos internados. |
| `TypeTable` | Tabela de tipos por símbolo e declaração. |
| `TypedHirMap` | Mapa final de elementos HIR para `TypeId`. |
| `InferenceOutput` | Resultado da inferência usado pelo pipeline. |
| `CoercionTable` | Registro de coerções aplicadas. |
| `CallResolutionTable` | Registro de chamadas resolvidas. |
| `TypeDiagnosticSink` | Coletor de diagnósticos de tipo. |
| `TypeCheckState` | Estado final ou intermediário do pipeline. |

Os nomes concretos podem variar na implementação. O contrato obrigatório é a presença dessas responsabilidades.

---

## 7. Entrada

Entrada conceitual:

```rust
pub struct TypeCheckInput<'a> {
    pub hir: &'a Hir,
    pub scopes: &'a ScopeGraph,
    pub symbols: &'a SymbolTable,
    pub bindings: &'a NameBindingTable,
    pub previous_diagnostics: &'a [Diagnostic],
    pub options: TypeCheckOptions,
}
```

Regras:

- HIR deve estar estruturalmente válida ou marcada como parcial;
- nomes necessários à tipagem devem estar resolvidos;
- duplicidades e ambiguidades de nomes devem estar diagnosticadas;
- escopos e símbolos devem ser consistentes;
- diagnósticos anteriores podem bloquear partes da tipagem;
- pipeline não deve consultar tokens como fonte autoritativa;
- pipeline não deve modificar AST;
- pipeline não deve exigir MIR.

---

## 8. Saída

Saída conceitual:

```rust
pub struct TypeCheckOutput {
    pub typed_hir: TypedHirMap,
    pub type_table: TypeTable,
    pub coercions: CoercionTable,
    pub calls: CallResolutionTable,
    pub diagnostics: Vec<Diagnostic>,
    pub state: TypeCheckState,
}
```

Estado conceitual:

```rust
pub enum TypeCheckState {
    Checked,
    CheckedWithErrors,
    Blocked,
}
```

Regras:

- `Checked` exige ausência de diagnósticos bloqueantes de tipo;
- `CheckedWithErrors` permite HIR tipada com marcadores `Error`;
- `Blocked` deve ser usado quando entrada ou dependência impede análise segura;
- elementos tipáveis válidos devem possuir `TypeId`;
- chamadas válidas devem possuir alvo selecionado;
- coerções aplicadas devem estar registradas;
- nenhum `Unknown` não diagnosticado deve permanecer na saída final.

---

## 9. Etapas Obrigatórias

O pipeline do Stage 4 deve possuir as seguintes etapas conceituais:

1. Validar pré-condições.
2. Inicializar `TypeInterner`.
3. Registrar tipos built-in.
4. Coletar declarações de tipo.
5. Converter referências de tipo explícitas.
6. Construir assinaturas tipadas.
7. Executar inferência.
8. Resolver e validar restrições.
9. Resolver chamadas e overload aplicável.
10. Registrar coerções permitidas.
11. Validar generics do subconjunto inicial.
12. Materializar HIR tipada.
13. Emitir diagnósticos.
14. Validar invariantes finais.

A implementação pode fundir etapas internamente, desde que preserve as fronteiras conceituais e os artefatos observáveis.

---

## 10. Validação de Pré-condições

Antes de iniciar a tipagem, o pipeline deve verificar se a entrada permite análise segura.

Regras:

- erro estrutural grave de HIR deve bloquear a fase;
- ausência de `SymbolTable` consistente deve bloquear a fase;
- ausência de bindings para referências necessárias deve bloquear ou recuperar com `Error`;
- diagnósticos de resolução de nomes não devem ser duplicados;
- símbolos inválidos devem impedir criação de tipos nominais definitivos;
- estado parcial deve ser explícito.

---

## 11. Inicialização de Tipos

O pipeline deve inicializar a infraestrutura de tipos antes de analisar declarações.

Regras:

- `TypeInterner` deve existir antes da conversão de referências;
- built-ins obrigatórios devem ser registrados;
- `Unit` deve ser registrado como tipo válido;
- tipo `Null` não deve ser criado;
- IDs de built-ins devem ser determinísticos;
- estruturas auxiliares como `TypeTable`, `TypedHirMap`, `CoercionTable` e `CallResolutionTable` devem ser inicializadas vazias.

---

## 12. Coleta de Declarações de Tipo

Declarações que introduzem tipos devem ser coletadas antes da verificação de corpos.

Declarações mínimas:

- classes;
- interfaces;
- traits;
- structs;
- enums;
- Domains;
- parâmetros genéricos;
- aliases de tipo, quando existirem.

Regras:

- cada declaração nominal válida deve possuir `SymbolId`;
- cada tipo nominal deve ser registrado em `TypeTable`;
- tipos nominais devem ser internados por `SymbolId`;
- duplicidades já diagnosticadas não devem ser mescladas;
- relações declarativas necessárias à subtipagem podem ser registradas para uso posterior;
- coleta não deve validar corpo de funções ou métodos.

---

## 13. Conversão de Referências de Tipo

Referências explícitas de tipo na HIR devem ser convertidas para `TypeId`.

Regras:

- `HirTypeRefId` resolvido deve apontar para tipo interno;
- referência para símbolo de categoria inválida deve produzir diagnóstico;
- aplicação genérica explícita deve internar argumentos antes do tipo aplicado;
- número incorreto de argumentos genéricos deve produzir diagnóstico;
- referência inválida deve receber tipo `Error`;
- conversão não deve executar inferência de expressões;
- conversão não deve aplicar coerções.

---

## 14. Construção de Assinaturas

Funções, métodos e construtores devem possuir assinatura tipada antes da análise de corpos e chamadas.

Regras:

- parâmetros devem ter `TypeId`;
- retorno deve ter `TypeId`;
- ausência de retorno significativo deve usar `Unit`;
- receiver de método deve possuir tipo definido;
- parâmetros genéricos devem estar disponíveis no contexto da assinatura;
- assinatura inválida deve produzir diagnóstico e marcador `Error`;
- assinaturas devem ser consultáveis por resolução de chamadas.

---

## 15. Execução da Inferência

O pipeline deve executar a inferência definida em `TYPE-INFERENCE.md`.

Regras:

- inferência recebe HIR resolvida, tipos coletados, assinaturas e interner;
- inferência pode criar variáveis e restrições;
- inferência deve preencher tipos de expressões, locais, padrões e blocos;
- inferência deve preservar origem declarada ou inferida;
- inferência deve produzir solução determinística;
- falhas de inferência devem gerar diagnósticos;
- resultado da inferência deve ser consumido pela verificação de tipos.

---

## 16. Verificação de Restrições

Após inferência, o pipeline deve validar as restrições geradas.

Regras:

- igualdade entre tipos deve exigir equivalência canônica;
- atribuição deve exigir compatibilidade conforme o sistema de tipos;
- retorno deve ser compatível com assinatura;
- argumento deve ser compatível com parâmetro;
- operador deve aceitar os tipos de operandos;
- ramos que produzem valor devem ter tipo comum ou esperado;
- restrições incompatíveis devem produzir diagnóstico;
- restrições envolvendo `Error` devem evitar cascata redundante.

---

## 17. Subtipagem e Coerções

O pipeline deve consultar regras de subtipagem e coerção quando uma restrição admitir compatibilidade não idêntica.

Regras:

- subtipagem nominal deve ser consultada por serviço ou módulo dedicado;
- coerções implícitas só podem ser aplicadas quando permitidas;
- upcast válido deve ser registrado como coerção;
- coerção proibida deve produzir diagnóstico;
- downcast não deve ser assumido implicitamente;
- coerção aplicada deve preservar identidade, Domain e lifetime quando envolver `ObjectId<T>`;
- detalhes das regras pertencem a `SUBTYPING-AND-COERCIONS.md`.

Contrato conceitual:

```rust
pub struct CoercionTable {
    pub expr_coercions: Map<HirExprId, Coercion>,
}
```

---

## 18. Resolução de Chamadas

Chamadas devem ser resolvidas para função, método, construtor ou entidade chamável equivalente.

Regras:

- callee deve ter binding resolvido;
- candidatos devem ser coletados de forma determinística;
- aridade deve ser verificada antes de compatibilidade detalhada;
- argumentos devem ser comparados com parâmetros;
- coerções aplicáveis podem participar da seleção;
- retorno da chamada deve ser registrado no `TypedHirMap`;
- ausência de candidato aplicável deve produzir diagnóstico;
- múltiplos candidatos igualmente aplicáveis devem produzir ambiguidade;
- candidato não deve ser escolhido arbitrariamente.

Contrato conceitual:

```rust
pub struct CallResolution {
    pub call: HirExprId,
    pub target: SymbolId,
    pub signature: FunctionSignature,
    pub applied_coercions: Vec<Coercion>,
}
```

---

## 19. Overload Aplicável

Quando a linguagem ou o subconjunto permitir múltiplas assinaturas para o mesmo nome, o pipeline deve selecionar candidato aplicável.

Regras:

- overload deve operar apenas sobre candidatos já resolvidos por nome;
- overload não deve recuperar símbolos inexistentes;
- candidatos duplicados por erro anterior devem permanecer inválidos;
- seleção deve considerar aridade, tipos de argumentos e coerções permitidas;
- seleção deve ser determinística;
- empate sem regra de desempate deve produzir diagnóstico de ambiguidade;
- resultado deve ser registrado em `CallResolutionTable`.

Se overload não estiver habilitado no subconjunto inicial, múltiplos candidatos devem ser diagnosticados como ambíguos ou não suportados.

---

## 20. Generics do Subconjunto Inicial

O pipeline deve validar generics no nível suportado pelo Stage 4.

Regras:

- parâmetros genéricos devem ter identidade própria;
- argumentos explícitos devem ser convertidos para `TypeId`;
- argumentos inferidos devem vir da solução de inferência;
- quantidade de argumentos deve ser validada;
- constraints devem ser verificadas;
- substituições devem ser aplicadas de forma determinística;
- tipos genéricos resultantes devem ser internados;
- detalhes pertencem a `GENERICS-IMPLEMENTATION.md`.

---

## 21. Capacidade de Mutação

O pipeline pode registrar propriedades tipadas necessárias para validações de mutação, mas não substitui ownership, regiões ou Domains.

Regras:

- operações de escrita devem ser reconhecidas como tipadas;
- tipo do destino de escrita deve ser conhecido;
- incompatibilidade diretamente expressa no sistema de tipos deve ser diagnosticada;
- autoridade operacional de escrita pertence a Domains e fases de memória;
- ausência de capacidade de mutação quando exigida deve ser diagnosticável pela fase apropriada;
- o pipeline não deve validar lifetime ou exclusividade concorrente fora do escopo do Stage 4.

---

## 22. HIR Tipada

Ao final do pipeline, a HIR deve estar enriquecida com informações de tipo.

Regras:

- elementos tipáveis válidos devem apontar para `TypeId`;
- referências de tipo explícitas devem apontar para `TypeId`;
- chamadas válidas devem apontar para alvo selecionado;
- coerções aplicadas devem ser consultáveis;
- elementos inválidos devem possuir `Error` ou diagnóstico;
- tipo declarado e inferido devem permanecer distinguíveis;
- HIR tipada não deve depender da AST;
- fases posteriores não devem precisar repetir inferência.

---

## 23. Diagnósticos

O pipeline deve consolidar diagnósticos de tipo sem duplicar erros anteriores.

Situações mínimas:

- tipo inexistente ou inválido em contexto de tipo;
- tipo usado em categoria incorreta;
- inferência impossível;
- atribuição incompatível;
- retorno incompatível;
- chamada sem candidato aplicável;
- chamada ambígua;
- argumento incompatível;
- operador aplicado a tipos inválidos;
- coerção proibida;
- aplicação genérica inválida;
- constraint genérica não satisfeita;
- `ObjectId<T>` com `T` incompatível;
- `Unknown` restante em saída final.

Regras:

- cada diagnóstico deve possuir span primário;
- tipos esperado e encontrado devem ser informados quando possível;
- erros derivados de `Error` devem evitar ruído;
- diagnósticos devem ser emitidos em ordem determinística;
- erro de resolução de nomes não deve ser reemitido como erro de tipo equivalente.

---

## 24. Recuperação

O pipeline deve continuar após erros de usuário quando houver representação segura.

Regras:

- tipo `Error` deve ser usado para elementos inválidos;
- restrições envolvendo `Error` devem ser consideradas satisfeitas para evitar cascata, salvo quando ocultarem erro independente;
- chamada inválida pode receber alvo ausente e tipo `Error`;
- aplicação genérica inválida pode produzir tipo `Error`;
- tipo desconhecido não resolvido deve virar diagnóstico ou estado `Blocked`;
- recuperação não deve criar coerções falsas;
- recuperação não deve marcar programa inválido como `Checked`.

---

## 25. Estados do Pipeline

O pipeline deve manter estado explícito.

Estados:

| Estado | Significado |
| --- | --- |
| `Checked` | HIR tipada sem erros de tipo bloqueantes. |
| `CheckedWithErrors` | HIR tipada parcial ou completa com diagnósticos recuperáveis. |
| `Blocked` | Tipagem não pôde prosseguir de forma segura. |

Regras:

- `Checked` permite prosseguir para fases posteriores;
- `CheckedWithErrors` permite dumps e múltiplos diagnósticos, mas não codegen final;
- `Blocked` deve impedir fases dependentes da HIR tipada;
- estado final deve ser derivado de diagnósticos e invariantes, não de flag manual arbitrária.

---

## 26. Determinismo e Dumps

O pipeline deve produzir resultados determinísticos.

Regras:

- etapas devem percorrer HIR em ordem estável;
- candidatos de chamada devem ser ordenados determinísticamente;
- diagnósticos devem ter ordem estável;
- coerções devem ser registradas em ordem estável;
- dumps não devem conter endereços de memória;
- dumps devem usar `TypeId`, `SymbolId` e IDs HIR estáveis;
- mapas internos devem ser serializados por ordem de ID ou chave canônica.

Formato conceitual:

```text
type-check:
  state: checked
  expr #12 : type #2
  call #18 -> function #7
  coercion expr #21 ObjectId<#5> -> ObjectId<#3> upcast
```

O formato final pode variar, desde que seja determinístico.

---

## 27. Integração com `capic check`

O comando `capic check arquivo.capi` deve executar o pipeline completo necessário para validar o programa sem gerar código.

Fluxo mínimo:

```text
source
    ↓
lexer
    ↓
parser
    ↓
AST lowering
    ↓
HIR
    ↓
name resolution
    ↓
type checking pipeline
    ↓
diagnostics / success
```

Regras:

- sucesso exige ausência de diagnósticos bloqueantes;
- falha deve retornar código de saída apropriado;
- diagnósticos devem ser emitidos em formato compatível com a política do compilador;
- `check` não deve executar backend;
- `check` não deve produzir objeto, executável ou MIR obrigatória.

---

## 28. Invariantes

As seguintes invariantes são obrigatórias:

- pipeline consome HIR com nomes resolvidos;
- pipeline não cria símbolos para nomes textuais;
- built-ins existem antes da tipagem;
- todo tipo final é internado;
- todo elemento tipável válido possui `TypeId`;
- toda chamada válida possui alvo selecionado;
- toda coerção aplicada está registrada;
- toda restrição incompatível possui diagnóstico;
- nenhuma ambiguidade é resolvida arbitrariamente;
- nenhum `Unknown` não diagnosticado permanece na saída final;
- HIR tipada preserva rastreabilidade;
- resultados são determinísticos;
- pipeline não depende de MIR, ABI, backend ou layout físico.

Violação dessas invariantes deve ser tratada como erro de implementação.

---

## 29. Testes Obrigatórios

O Stage 4 deve conter testes que validem:

- `capic check` aceita programa válido básico;
- `capic check` rejeita programa com tipo inexistente;
- tipo explícito incompatível com inicializador;
- inferência de local válida;
- retorno compatível;
- retorno incompatível diagnosticado;
- chamada simples resolvida;
- chamada com aridade incorreta diagnosticada;
- chamada com argumento incompatível diagnosticada;
- chamada ambígua diagnosticada quando aplicável;
- upcast válido registrado como coerção;
- coerção proibida diagnosticada;
- aplicação genérica válida;
- aplicação genérica com aridade inválida;
- constraint genérica inválida diagnosticada quando suportada;
- `ObjectId<T>` rejeita `T` incompatível;
- HIR tipada não contém `Unknown` sem diagnóstico;
- dumps e diagnósticos são determinísticos.

Esses testes devem integrar a suíte semântica e contribuir para o critério do Documento 28: todos os testes semânticos passam.

---

## 30. Critérios de Conclusão

Este documento é considerado atendido quando:

- existe pipeline de type checking executável após resolução de nomes;
- o pipeline inicializa interning e tipos built-in;
- declarações e referências de tipo são convertidas para `TypeId`;
- assinaturas tipadas são construídas;
- inferência é executada e materializada;
- compatibilidade, chamadas, coerções e generics do subconjunto são validados;
- HIR tipada e tabelas auxiliares são produzidas;
- diagnósticos de tipo são estruturados e determinísticos;
- `capic check arquivo.capi` demonstra o Stage 4 sem acionar backend.
