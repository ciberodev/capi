# Name Resolution

**Projeto:** Linguagem Capi  
**Documento:** NAME-RESOLUTION  
**Status:** Aprovado  
**Stage:** Stage 3 — HIR e resolução de nomes  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o contrato de engenharia da resolução de nomes da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- a entrada e a saída da resolução de nomes;
- a sequência obrigatória entre HIR, escopos, símbolos e referências;
- como declarações são registradas;
- como referências simples e qualificadas são resolvidas;
- como imports participam da busca;
- como duplicidades, inexistência e ambiguidades são diagnosticadas;
- como a HIR é enriquecida com resultados de resolução;
- quais responsabilidades não pertencem ao Stage 3;
- quais invariantes e testes validam a fase.

A resolução de nomes é a primeira fase ativa da análise semântica. Ela transforma nomes pendentes da HIR em vínculos semânticos explícitos com declarações, sem inferir tipos, validar ownership, aplicar Domains ou gerar MIR.

---

## 2. Escopo

Este documento cobre:

- pipeline da resolução de nomes;
- construção ou consumo do `ScopeGraph`;
- registro de símbolos;
- resolução de referências em expressões;
- resolução de referências de tipos para símbolos declarativos;
- resolução de módulos e imports no subconjunto inicial;
- resolução de nomes qualificados;
- resolução de membros em nível compatível com Stage 3;
- shadowing;
- duplicidade;
- referência inexistente;
- ambiguidade;
- recuperação controlada;
- enriquecimento da HIR;
- dumps e testes de resolução.

Este documento não cobre:

- modelo completo de HIR;
- modelo completo de símbolos;
- modelo completo de escopos;
- inferência ou verificação de tipos;
- seleção de overload;
- coerções;
- avaliação constante;
- validação de visibilidade dependente de tipo ou herança;
- ownership e borrow checking;
- Domains;
- lowering para MIR;
- codegen;
- ABI.

Esses temas pertencem a:

- `HIR-MODEL.md`;
- `SYMBOL-MODEL.md`;
- `SCOPE-MODEL.md`;
- `TYPE-MODEL.md`;
- `TYPE-INFERENCE.md`;
- `TYPE-CHECKING-PIPELINE.md`;
- `SUBTYPING-AND-COERCIONS.md`;
- `OWNERSHIP-MODEL.md`;
- `DOMAIN-IMPLEMENTATION.md`;
- `MIR-LOWERING.md`;
- `ABI-IMPLEMENTATION.md`.

---

## 3. Posição no Stage 3

No Stage 3, a resolução de nomes deve implementar:

- construção do `ScopeGraph`, se ainda não tiver sido executada como etapa separada;
- registro de símbolos em `SymbolTable`;
- associação de declarações HIR a `SymbolId`;
- resolução de referências de valor, tipo, módulo e membros no subconjunto inicial;
- detecção de símbolos duplicados;
- detecção de referências inexistentes;
- detecção de ambiguidades;
- diagnósticos estruturados;
- dump HIR com dados resolvidos quando `capic --emit hir` for executado após essa fase.

Fluxo conceitual:

```text
HIR inicial
    ↓
ScopeGraph
    ↓
SymbolTable
    ↓
NameResolver
    ↓
NameResolutionOutput
    ↓
HIR com nomes resolvidos
```

---

## 4. Princípios

A resolução de nomes deve seguir estes princípios:

- resolver nomes, não tipos;
- usar `ScopeGraph` como autoridade de hierarquia lexical;
- usar `SymbolTable` como autoridade de declarações registradas;
- preservar identidade da HIR;
- preencher resultados de resolução de forma explícita;
- não escolher arbitrariamente entre candidatos ambíguos;
- não criar símbolos para referências;
- não esconder duplicidades;
- produzir diagnósticos determinísticos;
- permitir continuidade controlada após erros de usuário;
- manter distinção entre erro de usuário e erro interno;
- permanecer independente de MIR, backend, ABI e layout de objetos.

---

## 5. Entidades Conceituais

| Entidade | Responsabilidade |
| --- | --- |
| `NameResolver` | Coordena a fase de resolução. |
| `NameResolutionInput` | Entrada composta por HIR, escopos, símbolos e opções. |
| `NameResolutionOutput` | Resultado com HIR enriquecida ou tabelas auxiliares e diagnósticos. |
| `ResolutionContext` | Estado atual de escopo, namespace e item durante traversal. |
| `NameRefId` | Identidade de uma ocorrência de referência a nome. |
| `NameRefKind` | Categoria de referência que está sendo resolvida. |
| `ResolutionQuery` | Consulta de um nome ou caminho a partir de um escopo. |
| `ResolutionCandidate` | Candidato encontrado na tabela de símbolos. |
| `ResolutionResult` | Resultado final de uma consulta. |
| `ResolvedBinding` | Binding gravado na HIR ou tabela auxiliar. |
| `NameBindingTable` | Tabela auxiliar de bindings quando a HIR não armazena resolução diretamente. |
| `ResolutionError` | Erro estruturado de resolução. |

Os nomes concretos podem variar na implementação. O contrato obrigatório é a presença dessas responsabilidades.

---

## 6. Entradas

Entrada conceitual:

```rust
pub struct NameResolutionOptions {
    pub allow_partial_hir: bool,
    pub resolve_imports: bool,
    pub include_prelude: bool,
}

pub struct NameResolutionInput<'a> {
    pub hir: &'a mut Hir,
    pub scopes: &'a ScopeGraph,
    pub symbols: &'a mut SymbolTable,
    pub options: NameResolutionOptions,
}
```

Regras:

- HIR deve ter sido construída por lowering válido ou estar marcada como parcial;
- `ScopeGraph` deve estar consistente;
- `SymbolTable` deve conter declarações registradas ou ser preenchida por etapa coordenada do resolver;
- diagnósticos anteriores devem estar disponíveis para bloqueio quando necessário;
- resolução não deve consultar tokens;
- resolução não deve modificar a AST;
- resolução não deve exigir type checker.

Se a implementação combinar construção de escopos, registro de símbolos e resolução em uma única API, ela ainda deve preservar as fronteiras conceituais entre essas responsabilidades.

---

## 7. Saídas

Saída conceitual:

```rust
pub struct NameResolutionOutput {
    pub diagnostics: Vec<Diagnostic>,
    pub state: NameResolutionState,
    pub bindings: Option<NameBindingTable>,
    pub unresolved: Vec<NameRefId>,
}
```

Estado conceitual:

```rust
pub enum NameResolutionState {
    Resolved,
    ResolvedWithErrors,
    Blocked,
}
```

Regras:

- HIR deve conter ou conseguir consultar bindings resolvidos;
- referências válidas devem apontar para `SymbolId` ou entidade resolvida equivalente;
- referências inválidas devem possuir diagnóstico;
- ambiguidades devem preservar candidatos relevantes;
- estado `Blocked` deve ser usado quando HIR, escopos ou símbolos estiverem inválidos para análise segura;
- ausência de diagnóstico para referência não resolvida é erro de implementação.

---

## 8. Referências de Nome

Uma referência de nome é uma ocorrência HIR que precisa ser associada a declaração.

Contrato conceitual:

```rust
pub struct NameRefId(u32);

pub struct NameRef {
    pub id: NameRefId,
    pub kind: NameRefKind,
    pub path: UnresolvedPath,
    pub scope: ScopeId,
    pub origin: HirOrigin,
}

pub enum NameRefKind {
    Value,
    Type,
    Module,
    Import,
    Member,
    Constructor,
    PatternConstructor,
    Attribute,
}
```

Regras:

- cada referência deve ter escopo inicial determinável;
- cada referência deve ter namespace esperado ou conjunto de namespaces candidatos;
- paths vazios devem ser erro estrutural;
- referências de erro HIR não devem ser resolvidas como sucesso;
- atributos podem permanecer pendentes se o Stage 3 não implementar seus efeitos.

---

## 9. Queries de Resolução

Toda resolução deve ser expressa como consulta a partir de um escopo.

Contrato conceitual:

```rust
pub struct ResolutionQuery {
    pub path: UnresolvedPath,
    pub start_scope: ScopeId,
    pub namespaces: Vec<Namespace>,
    pub kind: NameRefKind,
}
```

Regras:

- `start_scope` deve existir no `ScopeGraph`;
- `namespaces` deve ser derivado do contexto HIR;
- consulta de valor não deve retornar tipo como sucesso;
- consulta de tipo não deve retornar variável local como sucesso;
- consulta de membro pode ser adiada quando exigir tipo do receiver;
- consultas devem ser puras em relação à tabela de símbolos, salvo caches explicitamente derivados.

---

## 10. Resultados de Resolução

Contrato conceitual:

```rust
pub enum ResolutionResult {
    Resolved(ResolutionCandidate),
    Ambiguous(Vec<ResolutionCandidate>),
    NotFound,
    Blocked(ResolutionError),
}

pub struct ResolutionError {
    pub kind: ResolutionErrorKind,
    pub origin: HirOrigin,
}

pub enum ResolutionErrorKind {
    InvalidScope,
    InvalidSymbol,
    InvalidPath,
    BlockedByHirError,
    UnsupportedInStage3,
}

pub struct ResolutionCandidate {
    pub symbol: SymbolId,
    pub scope: ScopeId,
    pub source: ResolutionSource,
}

pub enum ResolutionSource {
    LocalScope,
    AncestorScope,
    Import(HirImportId),
    Prelude,
    MemberScope,
    Synthetic,
}
```

Regras:

- `Resolved` deve conter exatamente um candidato válido;
- `Ambiguous` deve conter candidatos relevantes em ordem determinística;
- `NotFound` deve gerar diagnóstico;
- `Blocked` representa inconsistência estrutural ou erro anterior que impede análise segura;
- candidato com `SymbolState::Error` não deve ser sucesso;
- candidato duplicado deve produzir diagnóstico apropriado antes de ser usado para continuidade.

---

## 11. Bindings Resolvidos

Bindings são anexados à HIR ou armazenados em tabela auxiliar indexada por `HirId`/`NameRefId`.

Contrato conceitual:

```rust
pub struct NameBindingTable {
    pub by_ref: OrderedMap<NameRefId, ResolvedBinding>,
    pub by_hir: OrderedMap<HirId, ResolvedBinding>,
}

pub enum ResolvedBinding {
    Symbol(SymbolId),
    Module(SymbolId),
    TypeSymbol(SymbolId),
    Member(SymbolId),
    Import(HirImportId, SymbolId),
    Ambiguous(Vec<SymbolId>),
    NotFound,
    Error(HirErrorId),
}
```

Regras:

- binding de sucesso deve usar identidade semântica, não texto;
- `Ambiguous` e `NotFound` não são sucesso;
- `Error` deve referenciar erro HIR ou erro de resolução estruturado;
- fases posteriores devem conseguir distinguir sucesso de recuperação;
- HIR não deve receber símbolo sentinela para nome inexistente.

---

## 12. Sequência da Fase

Sequência obrigatória conceitual:

```text
1. Validar HIR inicial.
2. Construir ou receber ScopeGraph.
3. Registrar declarações na SymbolTable.
4. Diagnosticar duplicidades locais.
5. Resolver módulos e imports suportados.
6. Resolver referências de tipo.
7. Resolver referências de valor.
8. Resolver referências qualificadas.
9. Resolver referências de membro suportadas.
10. Anexar bindings à HIR.
11. Validar invariantes de saída.
```

Regras:

- etapas podem ser implementadas com passes combinados se preservarem o mesmo contrato;
- diagnósticos devem ser emitidos na fase que possui melhor contexto;
- falha em import não deve impedir resolução de referências locais independentes;
- referências dependentes de nome já inválido devem evitar cascata excessiva;
- type checking não deve ser executado dentro dessa sequência.

---

## 13. Registro de Declarações

O registro de declarações cria `SymbolId` para elementos declarativos.

Regras:

- deve usar `ScopeGraph` para determinar escopo de registro;
- deve usar `SYMBOL-MODEL.md` para categorias e namespaces;
- deve registrar símbolos antes de resolver usos;
- deve preservar conflitos em `SymbolSet`;
- deve diagnosticar duplicidade no mesmo escopo e namespace;
- deve permitir shadowing em escopos diferentes quando a linguagem permitir;
- não deve criar símbolo para path usado como referência;
- não deve inferir tipo de declaração.

Símbolos de declarações que também introduzem escopo devem obedecer:

```text
símbolo da declaração → escopo externo
escopo introduzido → corpo/região interna da declaração
```

---

## 14. Resolução de Referência Simples

Referência simples possui um único segmento de nome.

Ordem conceitual:

```text
1. consultar SymbolTable no escopo atual;
2. se não houver candidato, subir para escopo pai;
3. repetir até escopo global ou fronteira aplicável;
4. consultar imports visíveis;
5. consultar prelúdio, se aplicável;
6. produzir NotFound.
```

Regras:

- escopo mais interno tem prioridade;
- candidatos em namespace incompatível devem ser ignorados para sucesso;
- múltiplos candidatos equivalentes no mesmo nível produzem ambiguidade;
- duplicidade no mesmo escopo deve ser diagnosticada antes de resolução como sucesso;
- ordem de imports deve ser determinística;
- lookup não deve depender de ordem de hash map.

---

## 15. Resolução de Nomes Qualificados

Nomes qualificados possuem múltiplos segmentos.

Regras:

- primeiro segmento deve ser resolvido conforme contexto;
- cada segmento posterior deve ser resolvido dentro do namespace ou escopo exposto pelo segmento anterior;
- módulo qualificado deve avançar por escopos de módulo;
- tipo qualificado pode acessar escopo de membros ou namespace associado somente quando permitido;
- falha em segmento intermediário deve bloquear segmentos seguintes para evitar cascata;
- diagnóstico deve apontar o segmento que falhou;
- resolução qualificada não deve depender de type checking, exceto quando o acesso a membro exigir tipo do receiver; nesse caso deve ficar pendente ou produzir limitação explícita do Stage 3.

---

## 16. Resolução de Tipos

Referências de tipo na HIR são resolvidas para símbolos compatíveis.

Regras:

- `HirTypeRef::Path` deve procurar namespace `Type` e, quando aplicável, `Module`;
- classes, interfaces, traits, aliases e parâmetros genéricos são candidatos de tipo;
- tipos primitivos pendentes podem ser tratados como símbolos predefinidos ou nomes especiais conforme `TYPE-MODEL.md`;
- argumentos genéricos devem ter suas referências de tipo resolvidas, mas não verificadas;
- bounds genéricos podem ser resolvidos como referências de tipo;
- existência do símbolo de tipo é verificada no Stage 3;
- formação de `TypeId`, substituição genérica e checagem de bounds pertencem ao Stage 4.

---

## 17. Resolução de Valores

Referências de valor incluem variáveis locais, parâmetros, constantes, funções e bindings de padrões.

Regras:

- `Namespace::Value` deve ser consultado para expressões de nome;
- parâmetros e locals em escopos internos têm prioridade;
- funções e constantes globais podem ser encontradas após busca em ancestrais;
- uso antes da declaração deve obedecer regra da linguagem;
- se o Stage 3 ainda não modelar disponibilidade por posição, a limitação deve ser documentada e testada;
- seleção de overload de função não pertence ao Stage 3;
- chamada resolve o callee, mas não valida assinatura.

---

## 18. Resolução de Módulos e Imports

Imports disponibilizam símbolos externos ao escopo importador.

Regras:

- import nomeado deve tentar resolver path de módulo ou item importado;
- alias define nome exposto localmente;
- wildcard import expõe conjunto de símbolos em ordem determinística;
- import inexistente deve produzir diagnóstico;
- import ambíguo deve preservar candidatos;
- conflito entre import e declaração local deve seguir regra de prioridade da linguagem;
- resolução de pacotes externos pode ser limitada no Stage 3;
- imports não devem criar tipos, valores ou módulos fictícios sem diagnóstico.

O resolver pode representar imports pendentes em `ImportBinding` até que o sistema de módulos esteja completo.

---

## 19. Resolução de Membros

Membros incluem campos, métodos e construtores.

Regras:

- declaração de membro deve estar registrada em escopo de membros ou namespace `Member`;
- acesso qualificado por tipo pode resolver membro estático quando o subconjunto suportar;
- acesso por receiver de expressão pode exigir tipo do receiver e, portanto, pode ficar pendente para Stage 4;
- `this` pode resolver para símbolo sintético ou referência especial conforme contrato local;
- campo inexistente deve produzir diagnóstico somente quando a fase tiver informação suficiente;
- overload de método e dispatch não pertencem ao Stage 3.

O Stage 3 deve resolver apenas membros cujo contexto não dependa de inferência de tipos.

---

## 20. Patterns

Bindings de padrões introduzem símbolos e referências dentro de padrões podem apontar para construtores, variantes ou constantes.

Regras:

- binding de pattern deve registrar `SymbolKind::PatternBinding`;
- construtor em pattern deve ser resolvido no namespace apropriado;
- literal em pattern não exige resolução de nome;
- wildcard não introduz símbolo;
- duplicidade de binding no mesmo padrão deve ser diagnosticável;
- exaustividade e compatibilidade de tipo não pertencem ao Stage 3.

---

## 21. Shadowing

Shadowing é interpretado usando a hierarquia do `ScopeGraph`.

Regras:

- declaração em escopo interno pode sombrear símbolo externo quando permitido;
- duplicidade no mesmo escopo e namespace não é shadowing;
- shadowing deve considerar namespace;
- shadowing proibido deve produzir diagnóstico;
- shadowing permitido não deve alterar `SymbolId` sombreado;
- resolução em escopo interno deve escolher o símbolo mais próximo quando não houver ambiguidade local.

---

## 22. Ambiguidade

Uma referência é ambígua quando múltiplos candidatos válidos permanecem após aplicação das regras de prioridade.

Regras:

- ambiguidade deve produzir diagnóstico;
- diagnóstico deve apontar referência e candidatos relevantes;
- nenhum candidato deve ser escolhido arbitrariamente;
- binding deve ser `Ambiguous`;
- análise pode continuar com erro associado;
- candidatos devem ser listados em ordem determinística.

Fontes comuns:

- imports wildcard conflitantes;
- múltiplos imports com mesmo nome;
- duplicidade preservada em `SymbolSet`;
- namespaces mal definidos para o contexto;
- qualificação que encontra múltiplos módulos equivalentes.

---

## 23. Referência Inexistente

Uma referência é inexistente quando nenhuma declaração compatível é encontrada.

Regras:

- deve produzir diagnóstico;
- diagnóstico deve apontar o nome ou segmento que falhou;
- binding deve ser `NotFound` ou erro equivalente;
- não deve criar símbolo sentinela;
- sugestões podem ser produzidas quando houver infraestrutura;
- continuidade deve evitar cascatas a partir da mesma raiz inválida.

---

## 24. Duplicidade

Duplicidade ocorre quando declarações incompatíveis aparecem no mesmo escopo e namespace.

Regras:

- deve ser detectada durante registro de símbolos;
- deve produzir diagnóstico apontando declaração duplicada e anterior;
- `SymbolTable` deve preservar participantes;
- referência a nome duplicado não deve resolver como sucesso silencioso;
- overload permitido pela linguagem deve ser distinguido de duplicidade, mas seleção de overload não pertence ao Stage 3.

---

## 25. Enriquecimento da HIR

A resolução deve enriquecer a HIR sem alterar sua estrutura conceitual.

Regras:

- declarações devem receber `SymbolId` quando válidas;
- referências devem receber `ResolvedBinding` ou resultado equivalente;
- `HirBlock.scope` pode ser preenchido com `ScopeId`;
- erros devem ser representados explicitamente;
- spans e origens não devem ser apagados;
- IDs HIR não devem mudar;
- AST não deve ser modificada.

Implementações podem armazenar bindings em tabelas auxiliares, desde que fases posteriores consigam consultá-los de forma determinística.

---

## 26. Diagnósticos

Categorias iniciais:

- símbolo duplicado;
- shadowing proibido;
- nome inexistente;
- nome ambíguo;
- import inexistente;
- import ambíguo;
- namespace incompatível;
- membro inexistente quando resolvível no Stage 3;
- escopo inicial ausente para referência;
- símbolo em estado inválido usado como candidato.

Regras:

- erros de usuário devem ser diagnósticos semânticos;
- bugs de invariantes devem ser erros internos estruturados;
- diagnósticos devem ser determinísticos;
- diagnóstico de duplicidade deve apontar duas declarações;
- diagnóstico de inexistência deve apontar a referência;
- diagnóstico de ambiguidade deve listar candidatos quando possível;
- resolução não deve emitir diagnóstico de tipo incompatível.

---

## 27. Recuperação

A resolução deve continuar após erros quando for seguro.

Regras:

- referência inexistente pode gerar binding `NotFound`;
- ambiguidade pode gerar binding `Ambiguous`;
- erro em segmento qualificado bloqueia segmentos filhos;
- erro HIR estrutural pode bloquear a fase;
- duplicidade pode manter símbolo primário apenas para continuidade controlada;
- fases posteriores devem conseguir detectar bindings inválidos;
- recuperação não deve transformar programa inválido em sucesso.

---

## 28. Interface com Tipagem

Ao final da resolução, a tipagem deve poder assumir:

- declarações válidas possuem símbolos;
- referências válidas possuem bindings;
- referências de tipo existentes apontam para símbolos de tipo;
- referências de valor existentes apontam para símbolos de valor;
- erros de resolução estão representados;
- ambiguidades não foram escolhidas arbitrariamente.

A tipagem não deve precisar repetir busca lexical de nomes para referências já resolvidas.

---

## 29. Interface com Dumps

`capic --emit hir arquivo.capi` pode exibir HIR inicial ou HIR resolvida conforme ponto do pipeline escolhido.

Quando o dump incluir resolução, deve:

- exibir `ScopeId` quando útil;
- exibir `SymbolId` de declarações;
- exibir bindings de referências;
- exibir `NotFound` e `Ambiguous` explicitamente;
- exibir imports pendentes/resolvidos;
- manter ordem estável;
- não exibir endereços de memória;
- não depender de ordem de hash map.

Exemplo conceitual:

```text
item fn main id=item0 symbol=sym0 scope=scope1
  local x symbol=sym1 scope=scope2
  expr path x binding=sym1
```

---

## 30. Determinismo

Para a mesma HIR inicial, mesma versão do compilador e mesmas opções:

- escopos devem ser equivalentes;
- símbolos devem ser equivalentes;
- cada referência deve produzir o mesmo resultado;
- diagnósticos devem aparecer em ordem estável;
- candidatos ambíguos devem aparecer em ordem estável;
- dumps devem ser determinísticos.

A implementação não deve depender de:

- endereço de memória;
- ordem aleatória de hash;
- paralelismo sem ordenação explícita;
- ordem de diretórios não normalizada;
- locale do sistema.

---

## 31. Invariantes

Após resolução de nomes:

- todo `ScopeId` consultado existe no `ScopeGraph`;
- todo `SymbolId` consultado existe na `SymbolTable`;
- toda declaração válida que introduz nome possui símbolo;
- toda referência válida resolvida aponta para símbolo existente;
- toda referência inexistente possui diagnóstico;
- toda referência ambígua preserva candidatos;
- duplicidades não são descartadas;
- shadowing permitido preserva símbolos externos;
- HIR preserva IDs e origens;
- resolução não adiciona tipos inferidos;
- resolução não cria MIR;
- resolução não depende de backend ou ABI.

Violação causada por bug da implementação deve ser erro interno estruturado quando recuperável.

---

## 32. Testes Obrigatórios

Testes de resolução de nomes no Stage 3 devem cobrir:

- função livre resolvida por chamada simples;
- classe registrada como tipo;
- interface registrada como tipo;
- trait registrada como tipo;
- campo registrado em escopo de membros;
- método registrado em escopo de membros;
- construtor registrado distintamente;
- parâmetro resolvido no corpo;
- declaração local resolvida no bloco;
- shadowing permitido;
- duplicidade no mesmo escopo;
- referência inexistente;
- referência ambígua;
- tipo nomeado resolvido;
- parâmetro genérico resolvido quando suportado;
- import simples;
- import com alias;
- import wildcard;
- conflito entre imports;
- módulo explícito;
- módulo implícito;
- nome qualificado de módulo;
- nome qualificado inexistente em segmento intermediário;
- pattern binding;
- duplicidade de binding em pattern;
- referência em match quando suportada;
- `this` quando suportado;
- membro resolvido sem depender de tipo quando suportado;
- membro pendente quando depender de inferência;
- preservação de spans em diagnósticos;
- dump determinístico de HIR resolvida;
- continuidade após erro independente.

Testes de resolução não devem exigir inferência de tipos, ownership, Domains, MIR, backend ou ABI.

---

## 33. Critérios de Aceite

Este documento é considerado aprovado para orientar a implementação do Stage 3 quando:

- define entrada e saída da resolução;
- define queries, candidatos, resultados e bindings;
- define sequência entre escopos, símbolos e resolução;
- define resolução simples, qualificada, de tipos, valores, imports, membros e patterns;
- define tratamento de duplicidade, shadowing, inexistência e ambiguidade;
- define enriquecimento da HIR;
- define diagnósticos, recuperação, invariantes e testes obrigatórios.

A implementação correspondente será considerada concluída quando:

- todos os nomes do subconjunto inicial forem resolvidos;
- erros de resolução forem diagnosticados;
- HIR resolvida preservar identidade e origem;
- símbolos possuírem identidade interna estável;
- ambiguidades não forem resolvidas arbitrariamente;
- referências inexistentes não criarem símbolos fictícios;
- dump de HIR resolvida for determinístico;
- todos os testes obrigatórios do Stage 3 passarem.

---

## 34. Relações Normativas

Este documento depende diretamente de:

- Documento 04 — Sintaxe da Linguagem;
- Documento 06 — Arquitetura do Compilador;
- Documento 16 — HIR;
- Documento 17 — Resolução de Nomes;
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial;
- `HIR-MODEL.md`;
- `SYMBOL-MODEL.md`;
- `SCOPE-MODEL.md`;
- `AST-LOWERING.md`;
- `SPANS-AND-LOCATIONS.md`;
- `DIAGNOSTIC-DATA-MODEL.md`;
- `DIAGNOSTIC-ARCHITECTURE.md`.

Este documento orienta diretamente:

- `TYPE-MODEL.md`;
- `TYPE-INFERENCE.md`;
- `TYPE-CHECKING-PIPELINE.md`;
- `SEMANTIC-TESTS.md`;
- implementação do resolver de nomes;
- implementação de diagnósticos de símbolos duplicados, inexistentes e ambíguos;
- implementação de `capic --emit hir` com HIR resolvida.
