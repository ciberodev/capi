# Semantic Tests

**Projeto:** Linguagem Capi  
**Documento:** SEMANTIC-TESTS  
**Status:** Aprovado  
**Stage:** Stage 3 — HIR e resolução de nomes  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define a estratégia de testes para a primeira etapa semântica da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- quais comportamentos semânticos do Stage 3 devem ser testados;
- onde os testes devem viver;
- quais camadas de teste são obrigatórias;
- como validar lowering de AST para HIR;
- como validar HIR inicial;
- como validar `ScopeGraph`;
- como validar `SymbolTable`;
- como validar resolução de nomes;
- como testar diagnósticos semânticos iniciais;
- como testar o resultado demonstrável `capic --emit hir`;
- quais critérios precisam ser atendidos para concluir o Stage 3.

---

## 2. Escopo

Este documento cobre:

- testes de lowering AST-HIR;
- testes do modelo de HIR inicial;
- testes de IDs HIR determinísticos;
- testes de rastreabilidade AST-HIR;
- testes de `ScopeGraph`;
- testes de mapeamento HIR-escopo;
- testes de `SymbolTable`;
- testes de registro de símbolos;
- testes de duplicidade de símbolos;
- testes de shadowing;
- testes de imports no subconjunto inicial;
- testes de resolução de nomes simples e qualificados;
- testes de referências inexistentes;
- testes de ambiguidades;
- testes de diagnósticos semânticos iniciais;
- testes de dump de HIR inicial e HIR resolvida;
- testes de CLI para `capic --emit hir`.

Este documento não cobre:

- inferência ou verificação de tipos;
- subtipagem;
- coerções;
- seleção de overload;
- checagem de ownership;
- borrow checker;
- análise de regiões;
- Domains;
- MIR;
- codegen;
- runtime;
- execução de programas Capi;
- conformidade final da linguagem.

Esses temas pertencem a:

- `TYPE-INFERENCE.md`;
- `TYPE-CHECKING-PIPELINE.md`;
- `OWNERSHIP-TESTS.md`;
- `MIR-TESTS.md`;
- `CODEGEN-TESTS.md`;
- `RUN-PASS-TESTS.md`;
- `CONFORMANCE-SUITE.md`.

---

## 3. Princípios

Os testes semânticos do Stage 3 devem seguir:

- determinismo;
- fixtures pequenas e legíveis;
- uma causa principal por teste;
- separação entre HIR inicial e HIR resolvida;
- validação explícita de spans e origem;
- validação explícita de IDs quando forem parte do dump;
- validação de diagnósticos estruturados;
- ausência de dependência de caminhos absolutos;
- ausência de dependência de ordem instável de mapas;
- preservação de regressões corrigidas;
- não antecipação de tipagem.

Um teste semântico deve falhar de forma local: ao quebrar, deve ficar claro se o problema é lowering, HIR, escopo, símbolo, import, resolução, diagnóstico ou dump.

---

## 4. Camadas de Teste

O Stage 3 deve usar seis camadas principais.

| Camada | Objetivo |
| --- | --- |
| Unitários | Validar builders, arenas, IDs, mapas e funções locais. |
| Integração semântica | Validar `SourceMap -> Lexer -> Parser -> AST -> HIR -> Scopes -> Symbols -> Resolution`. |
| Snapshot/UI | Validar dumps de HIR, escopos, símbolos e diagnósticos. |
| Compile-pass semântico | Validar programas que passam por lowering e resolução. |
| Compile-fail semântico | Validar duplicidade, inexistência, ambiguidade e erros de resolução. |
| CLI | Validar `capic --emit hir arquivo.capi`. |

Testes unitários devem cobrir invariantes locais. Snapshots devem ser usados quando a saída textual de HIR, símbolos, escopos ou diagnósticos for contrato observado.

---

## 5. Organização Esperada

Organização sugerida no workspace:

```text
capi-lang/
├── crates/
│   ├── capi-hir/
│   │   ├── src/
│   ├── capi-lowering/
│   │   ├── src/
│   │   └── tests/
│   ├── capi-sema/
│   │   ├── src/
│   │   └── tests/
│   ├── capi-parser/
│   ├── capi-ast/
│   ├── capi-source/
│   └── capi-diagnostics/
└── tests/
    └── semantic/
        ├── lowering/
        ├── hir/
        ├── scopes/
        ├── symbols/
        ├── resolution/
        ├── pass/
        ├── fail/
        └── snapshots/
```

Enquanto a estrutura final não existir, os testes podem viver junto ao crate que expõe a API pública da HIR ou da análise semântica.

Fixtures compartilhadas devem ser pequenas e nomeadas pelo comportamento semântico principal.

---

## 6. Convenções de Nome

Nomes de testes devem indicar comportamento.

Preferir:

```text
lowers_function_to_hir_item
preserves_hir_origin_for_local_decl
builds_scope_for_nested_block
registers_param_symbol_in_function_scope
resolves_local_before_global
reports_duplicate_symbol_in_same_scope
reports_unresolved_name
reports_ambiguous_wildcard_import
```

Evitar:

```text
semantic_works
test_resolution
bad_name
hir_misc
full_program
```

Fixtures devem usar nomes como:

```text
lowering_function_simple.cap
hir_class_members.cap
scope_nested_blocks.cap
symbol_duplicate_local.cap
resolve_local_shadowing.cap
resolve_missing_name.cap
resolve_ambiguous_import.cap
hir_dump_function.snapshot
```

---

## 7. API de Teste

Testes semânticos podem usar helper que receba fonte em memória e execute o pipeline até o ponto desejado.

O helper de lowering deve usar `capi-lowering`; testes semânticos devem consumir HIR por `capi-hir` e não depender da estrutura da AST após a transformação.

Contrato conceitual:

```rust
fn lower_source(text: &str) -> LoweringTestOutput;
fn resolve_source(text: &str) -> NameResolutionTestOutput;

struct LoweringTestOutput {
    hir: Option<Hir>,
    ast_to_hir: AstToHirMap,
    diagnostics: Vec<Diagnostic>,
}

struct NameResolutionTestOutput {
    hir: Option<Hir>,
    scopes: Option<ScopeGraph>,
    symbols: Option<SymbolTable>,
    bindings: Option<NameBindingTable>,
    diagnostics: Vec<Diagnostic>,
}
```

Helpers devem:

- criar `SourceMap` determinístico;
- executar lexer e parser reais por padrão;
- falhar se houver diagnóstico léxico ou sintático inesperado;
- permitir executar apenas até HIR inicial;
- permitir executar até resolução de nomes;
- normalizar nomes de arquivos em snapshots;
- oferecer APIs para consultar spans por trecho de texto;
- não depender de paths absolutos;
- não executar inferência de tipos.

---

## 8. Testes de Lowering Obrigatórios

Testes de lowering AST-HIR devem cobrir:

- unidade mínima;
- módulo implícito;
- módulo explícito;
- import simples;
- import wildcard;
- função livre;
- função com retorno omitido;
- função sem corpo quando suportada;
- parâmetros;
- classe vazia;
- classe com campo;
- classe com método;
- classe com construtor;
- interface;
- trait;
- modificadores;
- atributos;
- tipos nomeados;
- tipos genéricos;
- arrays;
- tuplas;
- tipo função;
- declaração local `let`;
- declaração local `const`;
- bloco vazio;
- bloco com múltiplos comandos;
- `if`;
- `else if` normalizado;
- `while`;
- `for`;
- `switch`;
- `match`;
- patterns básicos;
- literais;
- caminhos não resolvidos;
- chamadas;
- acesso a membro;
- indexação;
- `new`;
- operadores unários;
- operadores binários;
- atribuição;
- agrupamento redundante;
- spans preservados;
- `SourceId` preservado em `HirOrigin`;
- origem AST preservada;
- `AstToHirMap` determinístico;
- AST com erro bloqueando lowering;
- HIR parcial marcada como inválida, se suportada.

Esses testes não devem exigir `ScopeGraph`, `SymbolTable`, `NameBindingTable` nem inferência de tipos.

---

## 9. Testes de HIR Obrigatórios

Testes do modelo de HIR devem cobrir:

- criação de IDs HIR tipados;
- unicidade de IDs por categoria;
- estabilidade de IDs dentro da mesma execução;
- arena ou armazenamento equivalente sem referências inválidas;
- traversal completo da HIR sem AST;
- listas em ordem textual;
- elementos de erro HIR;
- validade parcial quando suportada;
- separação entre nomes pendentes e bindings resolvidos;
- ausência de `SymbolId` na HIR inicial;
- ausência de `ScopeId` final na HIR inicial, salvo campos opcionais vazios;
- ausência de `TypeId` na HIR inicial;
- dump determinístico de HIR inicial.

Snapshots de HIR inicial devem exibir caminhos e tipos como pendentes.

---

## 10. Testes de Escopos Obrigatórios

Testes de `ScopeGraph` devem cobrir:

- criação de escopo global;
- escopo de módulo implícito;
- escopo de módulo explícito;
- imports associados ao escopo correto;
- função livre com escopo interno;
- método com escopo interno;
- construtor com escopo interno;
- parâmetros visíveis no corpo;
- bloco vazio;
- bloco aninhado;
- declaração local visível no bloco correto;
- declaração local fora de escopo;
- classe com escopo de tipo;
- classe com escopo de membros;
- interface com escopo próprio;
- trait com escopo próprio;
- campo registrado no escopo de membros;
- `if` com blocos separados;
- `while` com corpo;
- `for` com variável local ou inicializador;
- `match` com bindings por braço;
- patterns com bindings;
- parâmetros genéricos quando suportados;
- mapeamento HIR-escopo;
- determinismo de `ScopeId`;
- dump determinístico de escopos;
- escopo de erro para HIR parcial, se suportado.

Testes de escopo não devem validar tipos de condições, exaustividade ou ownership.

---

## 11. Testes de Símbolos Obrigatórios

Testes de `SymbolTable` devem cobrir:

- símbolo de módulo;
- símbolo de função livre;
- símbolo de classe;
- símbolo de interface;
- símbolo de trait;
- símbolo de campo;
- símbolo de método;
- símbolo de construtor;
- símbolo de constante;
- símbolo de parâmetro;
- símbolo de declaração local;
- símbolo de binding de pattern;
- símbolo de parâmetro genérico quando suportado;
- `SymbolId` único;
- `SymbolName` distinto de `SymbolId`;
- registro por `ScopeId`;
- namespaces distintos quando aplicável;
- duplicidade no mesmo escopo;
- nomes iguais em escopos diferentes;
- shadowing permitido;
- shadowing proibido quando aplicável;
- import com alias;
- import wildcard;
- conflito entre imports;
- preservação de origem e span;
- dump determinístico da tabela de símbolos.

Testes de símbolos não devem resolver chamadas, inferir tipos ou selecionar overload.

---

## 12. Testes de Resolução Obrigatórios

Testes de resolução de nomes devem cobrir:

- referência a função livre;
- referência a parâmetro;
- referência a declaração local;
- local sombreando global;
- bloco interno sombreando bloco externo quando permitido;
- erro de shadowing proibido quando aplicável;
- nome inexistente em expressão;
- tipo inexistente;
- símbolo duplicado no mesmo escopo;
- referência ambígua;
- tipo nomeado resolvido;
- classe usada como tipo;
- interface usada como tipo;
- trait usada como tipo;
- parâmetro genérico usado como tipo quando suportado;
- import simples;
- import com alias;
- import wildcard;
- conflito entre imports;
- módulo explícito;
- módulo implícito;
- nome qualificado de módulo;
- segmento intermediário inexistente em nome qualificado;
- chamada com callee resolvido, sem validar assinatura;
- acesso a membro resolvido somente quando não depender de inferência;
- acesso a membro pendente quando depender de tipo;
- `this` quando suportado;
- binding de pattern;
- duplicidade de binding em pattern;
- continuidade após erro independente.

Testes de resolução não devem exigir inferência de tipo do receiver, compatibilidade de argumentos ou dispatch dinâmico.

---

## 13. Testes de Diagnóstico

Diagnósticos semânticos do Stage 3 devem ser testados para:

- símbolo duplicado;
- referência inexistente;
- referência ambígua;
- import inexistente;
- import ambíguo;
- namespace incompatível;
- shadowing proibido;
- binding duplicado em pattern;
- construção HIR inválida para resolução;
- erro interno estruturado em invariante quebrada quando houver caminho de recuperação.

Cada teste de diagnóstico deve validar:

- categoria do diagnóstico;
- severidade;
- span principal;
- labels auxiliares quando houver;
- ausência de panic;
- ausência de diagnóstico de tipo quando o erro é apenas de resolução;
- ordem determinística quando múltiplos diagnósticos forem emitidos.

Mensagens textuais podem ser validadas por snapshot quando o estilo já estiver estável. Antes disso, testes devem priorizar dados estruturados.

---

## 14. Testes de Dumps

O Stage 3 deve validar dumps relacionados a:

- HIR inicial;
- HIR resolvida;
- escopos, se houver dump dedicado;
- símbolos, se houver dump dedicado;
- diagnósticos de resolução em formato textual ou estruturado;
- CLI `capic --emit hir`.

Requisitos:

- saída determinística;
- ausência de endereços de memória;
- ausência de paths absolutos não normalizados;
- IDs impressos em ordem estável;
- caminhos não resolvidos marcados como pendentes na HIR inicial;
- bindings exibidos apenas na HIR resolvida;
- erros e HIR parcial exibidos explicitamente.

---

## 15. Compile-Pass Semântico

Casos compile-pass semânticos devem validar que o programa chega até HIR resolvida sem diagnósticos bloqueadores.

Cobertura mínima:

- arquivo mínimo válido;
- módulo com função;
- imports válidos no subconjunto inicial;
- função com parâmetro e local;
- classe com campo e método;
- interface;
- trait;
- bloco aninhado;
- shadowing permitido;
- tipo nomeado existente;
- match com binding quando suportado.

Compile-pass semântico não significa que o programa está tipado ou executável.

---

## 16. Compile-Fail Semântico

Casos compile-fail semânticos devem validar que entradas semânticas inválidas produzem diagnósticos estruturados sem panic.

Cobertura mínima:

- função duplicada no mesmo escopo;
- classe duplicada no mesmo namespace;
- parâmetro duplicado;
- local duplicado quando proibido;
- campo duplicado;
- método duplicado quando overload não for suportado;
- import inexistente;
- import ambíguo;
- nome inexistente;
- tipo inexistente;
- nome qualificado com segmento inexistente;
- referência ambígua por imports;
- shadowing proibido quando aplicável;
- binding duplicado em pattern.

Cada caso deve declarar a causa principal e evitar misturar muitos erros independentes.

---

## 17. Testes de Regressão

Todo bug corrigido em lowering, HIR, escopos, símbolos ou resolução deve gerar teste de regressão.

Regras:

- o teste deve reproduzir a entrada mínima;
- o nome deve indicar o comportamento corrigido;
- regressões de diagnóstico devem validar código/categoria e span;
- regressões de dump devem usar snapshot estável;
- regressões de panic devem validar execução controlada com diagnóstico ou erro interno estruturado.

---

## 18. Determinismo

Testes semânticos devem verificar determinismo quando aplicável.

Devem ser determinísticos:

- IDs HIR;
- `AstToHirMap`;
- `ScopeId`;
- `SymbolId`;
- ordem de símbolos em dumps;
- ordem de escopos em dumps;
- ordem de candidatos ambíguos;
- ordem de diagnósticos;
- saída de `capic --emit hir`.

Nenhum teste deve depender de:

- ordem de `HashMap`;
- endereço de memória;
- paths absolutos;
- locale;
- ordem de arquivos não normalizada;
- paralelismo sem junção ordenada.

---

## 19. Integração com CLI

Testes de CLI devem cobrir:

- `capic --emit hir arquivo.capi` para arquivo válido;
- `capic --emit hir arquivo.capi` com HIR inicial determinística;
- erro semântico reportado por `capic --emit hir` quando a resolução fizer parte do comando;
- arquivo com erro sintático bloqueando HIR normal;
- normalização de caminhos em saída;
- código de saída de sucesso para caso válido;
- código de saída de falha controlada para erro de usuário.

Se o comando possuir modo explícito para HIR inicial versus HIR resolvida, ambos devem ter testes separados.

---

## 20. Dados de Teste

Fixtures devem:

- ser pequenas;
- conter apenas o necessário para o comportamento testado;
- evitar dependência de biblioteca padrão não implementada;
- evitar comportamento de tipos ainda não implementado;
- usar nomes claros;
- preservar comentários apenas quando relevantes;
- declarar expectativas por snapshot ou arquivo auxiliar quando necessário.

Fixtures não devem ser usadas como exemplos de linguagem completa quando o objetivo for uma regra pequena de resolução.

---

## 21. Critérios de Aceite

Este documento é considerado aprovado para orientar o Stage 3 quando:

- define camadas de teste semântico;
- define organização esperada;
- define testes obrigatórios para lowering;
- define testes obrigatórios para HIR;
- define testes obrigatórios para escopos;
- define testes obrigatórios para símbolos;
- define testes obrigatórios para resolução;
- define testes de diagnósticos, dumps, pass/fail, CLI e regressão;
- define critérios de determinismo.

A suíte semântica do Stage 3 será considerada suficiente quando:

- AST válida produzir HIR inicial testada;
- HIR inicial preservar spans, `SourceId` e origem AST;
- `ScopeGraph` for testado para construções do subconjunto inicial;
- `SymbolTable` for testada para declarações do subconjunto inicial;
- resolução de nomes cobrir valores, tipos, módulos/imports e ambiguidades do subconjunto inicial;
- erros de resolução produzirem diagnósticos estruturados;
- `capic --emit hir` possuir teste demonstrável;
- todos os testes obrigatórios passarem.

---

## 22. Relações Normativas

Este documento depende diretamente de:

- Documento 16 — HIR;
- Documento 17 — Resolução de Nomes;
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial;
- `AST-MODEL.md`;
- `AST-LOWERING.md`;
- `HIR-MODEL.md`;
- `SCOPE-MODEL.md`;
- `SYMBOL-MODEL.md`;
- `NAME-RESOLUTION.md`;
- `DIAGNOSTIC-DATA-MODEL.md`;
- `DIAGNOSTIC-ARCHITECTURE.md`;
- `TEST-STRATEGY.md`;
- `PARSER-TESTS.md`;
- `LEXER-TESTS.md`.

Este documento orienta diretamente:

- implementação dos testes do Stage 3;
- testes de `capic --emit hir`;
- testes de regressão semântica;
- critérios de conclusão do Stage 3.
