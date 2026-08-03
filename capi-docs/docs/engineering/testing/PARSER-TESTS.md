# Parser Tests

**Projeto:** Linguagem Capi  
**Documento:** PARSER-TESTS  
**Status:** Aprovado  
**Stage:** Stage 2 — Parser e AST  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define a estratégia de testes para o parser e a AST da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- quais comportamentos sintáticos devem ser testados;
- onde os testes de parser devem viver;
- quais camadas de teste são obrigatórias;
- como validar AST, spans, precedência e diagnósticos;
- como testar recuperação sintática;
- como testar o resultado demonstrável `capic --emit ast`;
- quais critérios precisam ser atendidos para concluir o Stage 2.

---

## 2. Escopo

Este documento cobre:

- testes unitários do parser;
- testes de integração entre source, lexer, parser e diagnostics;
- testes de AST;
- testes de precedência e associatividade;
- testes de recovery;
- testes negativos de erros sintáticos;
- testes de spans em nós AST;
- snapshots ou UI tests para dump de AST;
- testes de CLI para `capic --emit ast`;
- regressões de bugs sintáticos.

Este documento não cobre:

- testes completos de lexer;
- testes de lowering AST para HIR;
- resolução de nomes;
- checagem de tipos;
- ownership;
- MIR;
- codegen;
- runtime;
- execução de programas Capi;
- conformidade final da linguagem.

Esses temas pertencem a:

- `LEXER-TESTS.md`;
- `SEMANTIC-TESTS.md`;
- `TYPE-CHECKING-PIPELINE.md`;
- `OWNERSHIP-TESTS.md`;
- `MIR-TESTS.md`;
- `CODEGEN-TESTS.md`;
- `RUN-PASS-TESTS.md`;
- `CONFORMANCE-SUITE.md`.

---

## 3. Princípios

Os testes do parser devem seguir:

- determinismo;
- fixtures pequenas e legíveis;
- uma causa principal por teste;
- cobertura de casos válidos e inválidos;
- validação explícita de AST quando a estrutura for o contrato;
- validação explícita de spans;
- validação de diagnósticos estruturados;
- validação de recuperação sem loop infinito;
- ausência de dependência de caminhos absolutos;
- preservação de regressões corrigidas.

Um teste de parser deve falhar de forma local: ao quebrar, deve ficar claro se o problema é reconhecimento sintático, construção da AST, precedência, span, diagnóstico, recovery ou dump.

---

## 4. Camadas de Teste

O Stage 2 deve usar cinco camadas principais.

| Camada | Objetivo |
| --- | --- |
| Unitários | Validar rotinas locais de parsing e precedência. |
| Integração | Validar `SourceMap -> Lexer -> Parser -> AST -> Diagnostics`. |
| Snapshot/UI | Validar dumps de AST e diagnósticos sintáticos. |
| Compile-fail sintático | Validar rejeição estruturada de entradas inválidas. |
| CLI | Validar `capic --emit ast arquivo.capi`. |

Testes unitários devem cobrir a maior parte das regras locais. Snapshots devem ser usados quando a saída textual da AST ou dos diagnósticos for contrato observado.

---

## 5. Organização Esperada

Organização sugerida no workspace:

```text
capi-lang/
├── crates/
│   ├── capi-parser/
│   │   ├── src/
│   │   └── tests/
│   ├── capi-ast/
│   ├── capi-lexer/
│   ├── capi-source/
│   └── capi-diagnostics/
└── tests/
    └── parser/
        ├── pass/
        ├── fail/
        ├── recovery/
        ├── ast/
        └── snapshots/
```

Enquanto a estrutura final não existir, os testes podem viver junto ao crate que expõe a API pública do parser.

Fixtures compartilhadas devem ser pequenas e nomeadas pelo comportamento sintático principal.

---

## 6. Convenções de Nome

Nomes de testes devem indicar comportamento.

Preferir:

```text
parses_module_decl
parses_function_with_return_type
parses_class_with_constructor
groups_multiplication_before_addition
recovers_missing_closing_paren_before_block
emits_error_for_missing_expression_after_equals
```

Evitar:

```text
test1
parser_works
bad_syntax
full_program
misc_ast
```

Fixtures devem usar nomes como:

```text
valid_function_simple.cap
valid_class_members.cap
valid_expr_precedence.cap
invalid_missing_paren.cap
recovery_missing_semicolon.cap
ast_dump_function.snapshot
```

---

## 7. API de Teste

Testes unitários podem usar helper que receba fonte em memória e retorne AST e diagnósticos.

Contrato conceitual:

```rust
fn parse_source(text: &str) -> ParseTestOutput {
    // SourceMap -> Lexer -> Parser
}

struct ParseTestOutput {
    ast: Ast,
    diagnostics: Vec<Diagnostic>,
}
```

Helpers de teste devem:

- criar `SourceMap` determinístico;
- executar lexer real, salvo testes unitários isolados do parser;
- falhar se houver diagnóstico léxico inesperado;
- permitir consultar AST de forma estável;
- permitir validar spans por offsets ou trechos de texto;
- normalizar caminhos e nomes de arquivo em snapshots.

---

## 8. Testes Unitários Obrigatórios

Testes unitários do parser devem cobrir:

- arquivo vazio;
- EOF explícito;
- declaração de módulo;
- import simples;
- import com wildcard;
- função sem parâmetros;
- função com parâmetros;
- função com retorno explícito;
- função com retorno omitido;
- função com corpo vazio;
- classe vazia;
- classe com campo;
- classe com método;
- classe com construtor;
- interface vazia;
- interface com assinatura de método;
- trait vazio;
- trait com método padrão;
- modificadores;
- atributos;
- tipos nomeados;
- tipos qualificados;
- tipos genéricos;
- arrays;
- tuplas;
- blocos;
- comandos simples;
- expressões primárias;
- chamadas;
- acesso a membro;
- indexação;
- criação com `new`;
- operadores prefixados;
- operadores binários;
- agrupamentos.

Cada teste deve validar pelo menos a categoria principal do nó produzido e ausência de diagnóstico sintático inesperado.

---

## 9. Testes de Unidade de Compilação

Devem existir testes para:

- arquivo vazio quando permitido pelo subconjunto;
- arquivo apenas com módulo;
- arquivo com módulo e imports;
- arquivo com múltiplas declarações;
- imports antes de declarações;
- declaração após imports;
- EOF após declaração completa;
- token extra após declaração malformada.

Requisitos:

- `CompilationUnit` deve preservar `SourceId`;
- módulo deve ser opcional quando a gramática permitir;
- imports devem preservar ordem;
- declarações devem preservar ordem;
- span da unidade deve ser verificável.

---

## 10. Testes de Declarações

Declarações de alto nível devem cobrir:

- `function`;
- `class`;
- `interface`;
- `trait`;
- `const`;
- `let` global, se habilitado;
- declarações com atributos;
- declarações com múltiplos modificadores;
- declarações com parâmetros genéricos;
- declarações em sequência.

Casos negativos:

- declaração sem nome;
- modificador sem declaração;
- atributo sem declaração;
- palavra-chave inesperada em top-level;
- declaração iniciada e não concluída.

Validações:

- categoria AST;
- nome;
- modificadores preservados;
- atributos preservados;
- span total;
- diagnóstico estruturado nos casos inválidos.

---

## 11. Testes de Classes, Interfaces e Traits

Classes devem cobrir:

- classe vazia;
- classe com `extends`;
- classe com `implements`;
- classe com `uses`;
- classe com campo;
- classe com método;
- classe com construtor;
- classe com membros em ordem;
- classe com generics;
- classe com modificadores.

Interfaces devem cobrir:

- interface vazia;
- interface com assinatura;
- interface com múltiplos membros.

Traits devem cobrir:

- trait vazio;
- trait com assinatura;
- trait com método com corpo.

Casos negativos:

- corpo sem `}`;
- membro inválido;
- construtor incompleto;
- campo sem `;`;
- método sem nome;

Testes devem validar que o parser apenas preserva estrutura sintática, sem checar existência de tipos, validade de herança, layout ou regras semânticas.

---

## 12. Testes de Funções e Parâmetros

Devem cobrir:

- função sem parâmetros;
- função com um parâmetro;
- função com múltiplos parâmetros;
- parâmetros com tipos nomeados;
- parâmetros com tipos genéricos;
- retorno explícito;
- retorno omitido;
- corpo vazio;
- corpo com comandos;
- assinatura sem corpo quando o contexto permitir;
- generics em função.

Casos negativos:

- `function` sem nome;
- lista de parâmetros sem `)`;
- parâmetro sem nome;
- parâmetro sem tipo quando obrigatório;
- vírgula ausente;
- vírgula duplicada;
- retorno com tipo ausente;
- corpo ausente quando obrigatório.

---

## 13. Testes de Tipos

Tipos sintáticos devem cobrir:

- tipo simples;
- tipo qualificado;
- tipo primitivo escrito como nome;
- tipo genérico com um argumento;
- tipo genérico com múltiplos argumentos;
- tipo genérico aninhado;
- array com tamanho;
- array sem tamanho quando permitido;
- tupla;
- tipo funcional, se habilitado no subconjunto.

Casos negativos:

- tipo ausente após `:`;
- `<` sem `>`;
- argumento genérico ausente;
- separador ausente em genérico;
- `]` ausente em array;
- tupla incompleta.

Validações:

- estrutura de `TypeSyntax`;
- spans;
- diagnósticos;
- ausência de resolução de tipo.

---

## 14. Testes de Comandos

Comandos devem cobrir:

- bloco vazio;
- bloco com múltiplos comandos;
- declaração local `let`;
- constante local `const`;
- comando de expressão;
- `return;`;
- `return expr;`;
- `break;`;
- `continue;`;
- `if`;
- `if else`;
- `else if`;
- `switch`;
- `while`;
- `for`;
- `foreach`, se habilitado no token model;
- `match`;
- bloco `unsafe`.

Casos negativos:

- comando simples sem `;`;
- bloco sem `}`;
- `if` sem `)`;
- `return` incompleto;
- `for` com componentes malformados;
- token inesperado dentro de bloco.

Testes não devem validar alcançabilidade, tipo da condição, escopo ou retorno obrigatório.

---

## 15. Testes de Expressões

Expressões devem cobrir:

- literal inteiro;
- literal float;
- literal string;
- literal char;
- literal booleano;
- nome;
- `this`;
- agrupamento;
- tupla;
- array literal;
- chamada sem argumentos;
- chamada com argumentos;
- chamada encadeada;
- acesso a membro;
- indexação;
- criação com `new`;
- operadores prefixados;
- operadores binários;
- atribuição;
- conversão por forma de chamada, como `valor.as<Int32>()`;
- condicional ternário se habilitado.

Casos negativos:

- expressão ausente após `=`;
- operador binário sem lado direito;
- chamada sem `)`;
- argumento ausente;
- indexação sem `]`;
- agrupamento sem `)`;
- token inesperado em expressão.

---

## 16. Testes de Precedência e Associatividade

Devem existir testes específicos para cada nível da tabela inicial de precedência:

```text
postfix
prefix
* / %
+ -
< <= > >=
== != ===
&&
||
=
```

Casos obrigatórios:

- multiplicação agrupa antes de adição;
- parênteses sobrescrevem precedência;
- operadores de mesmo nível associam corretamente;
- atribuição associa à direita;
- pós-fixos associam à esquerda;
- prefixos aplicam ao operando correto;
- chamada tem precedência maior que binário;
- acesso a membro encadeia corretamente;
- indexação encadeia corretamente.

Os testes devem validar a forma da AST, não o valor calculado.

Exemplo de contrato:

```text
a + b * c
```

Deve produzir `Binary(+, a, Binary(*, b, c))`.

```text
(a + b) * c
```

Deve produzir `Binary(*, Group(Binary(+, a, b)), c)` ou forma equivalente que preserve o agrupamento conforme `AST-MODEL.md`.

---

## 17. Testes de AST

Testes de AST devem validar:

- raiz `CompilationUnit`;
- categorias de nós;
- ordem de filhos;
- spans de nós folha;
- spans de nós compostos;
- ausência de dados semânticos;
- presença de modificadores e atributos;
- presença de nós de erro quando esperado;
- determinismo de IDs quando expostos;
- dump determinístico.

Esses testes devem evitar depender da representação física interna quando houver API pública mais estável.

Quando a estrutura interna ainda estiver evoluindo, snapshots de dump de AST podem servir como contrato de transição, desde que mudanças sejam revisadas intencionalmente.

---

## 18. Testes de Spans

Spans devem ser testados em:

- módulo;
- import;
- função completa;
- nome de função;
- parâmetro;
- tipo;
- bloco;
- comando;
- expressão binária;
- literal;
- chamada;
- acesso a membro;
- nó de erro.

Regras:

- spans devem usar offsets half-open `[start, end)`;
- lexema recuperado pelo span deve corresponder ao trecho esperado;
- EOF usa span vazio no fim da fonte;
- token ou nó sintético usa span vazio no ponto de inserção;
- nós compostos incluem delimitadores relevantes.

Testes com Unicode devem existir quando spans cruzarem texto multibyte próximo a construções sintáticas.

---

## 19. Testes de Diagnósticos Sintáticos

Diagnósticos sintáticos devem validar:

- severidade;
- código ou categoria estruturada;
- span primário;
- labels secundárias quando houver;
- mensagem principal quando estável;
- ausência de panic;
- quantidade de diagnósticos quando recovery for esperado.

Categorias obrigatórias:

- token inesperado;
- token esperado ausente;
- delimitador não fechado;
- separador ausente;
- declaração incompleta;
- tipo incompleto;
- expressão incompleta;
- muitos erros sintáticos.

Mensagens podem ser testadas por snapshot quando a política de diagnóstico já estiver estável. Caso contrário, testes devem priorizar categoria e spans.

---

## 20. Testes de Recovery

Recovery deve cobrir:

- token inesperado em top-level;
- declaração sem nome;
- modificador sem declaração;
- função sem `)`;
- função sem corpo;
- parâmetro sem tipo;
- vírgula ausente entre parâmetros;
- vírgula duplicada;
- classe sem `}`;
- membro inválido dentro de classe;
- campo sem `;`;
- tipo genérico sem `>`;
- tipo ausente após `:`;
- bloco sem `}`;
- `if` sem `)`;
- `return` incompleto;
- `let x = ;`;
- operador binário sem lado direito;
- chamada com argumento ausente;
- array ou indexação sem `]`;
- `switch` com `case` malformado;
- `match` com padrão ausente;
- EOF inesperado;
- sequência longa de tokens inválidos;
- limite de diagnósticos;
- ausência de loop infinito.

Cada teste de recovery deve validar:

- diagnóstico principal;
- ponto em que o parser retomou;
- AST parcial produzida;
- presença de `AstErrorNode` quando aplicável;
- continuidade para construções posteriores válidas.

---

## 21. Testes de Dump de AST

O Stage 2 exige o resultado demonstrável:

```bash
capic --emit ast arquivo.capi
```

Snapshots de dump devem cobrir:

- unidade mínima;
- módulo e imports;
- função simples;
- classe com membros;
- expressão com precedência;
- comando de controle;
- AST parcial com nó de erro.

Requisitos:

- saída determinística;
- spans em formato estável;
- ausência de endereços de memória;
- ausência de IDs não determinísticos;
- ausência de dados semânticos;
- normalização de caminhos de arquivo quando exibidos.

Mudança em snapshot de AST deve ser revisada como mudança de contrato observável do parser.

---

## 22. Testes de CLI

Testes de CLI devem validar:

```bash
capic --emit ast arquivo.capi
```

Casos obrigatórios:

- arquivo válido produz dump de AST;
- arquivo com erro sintático produz diagnóstico e, se suportado, AST parcial;
- arquivo inexistente continua sendo tratado pela infraestrutura de source/driver;
- flag inválida não chama parser;
- código de saída diferencia sucesso e erro.

Regras:

- testes de CLI devem usar fixtures temporárias com caminhos normalizados;
- saída textual deve ser validada por snapshot quando estável;
- stderr e stdout devem ter contrato definido pelo driver;
- testes de CLI não substituem testes unitários do parser.

---

## 23. Testes de Regressão

Todo bug corrigido no parser deve ganhar teste de regressão.

O teste deve:

- reproduzir o menor caso possível;
- declarar o comportamento esperado;
- ficar na pasta apropriada (`pass`, `fail`, `recovery` ou `ast`);
- referenciar issue ou comentário curto quando houver contexto útil;
- evitar depender de detalhes semânticos posteriores.

Regressões de recovery devem validar que o parser não entra em loop e que diagnósticos não crescem sem limite.

---

## 24. Critérios de Cobertura do Stage 2

Antes de concluir o Stage 2, a suíte deve demonstrar:

- parsing de módulos;
- parsing de declarações;
- parsing de classes;
- parsing de funções;
- parsing de tipos;
- parsing de expressões;
- parsing de comandos;
- precedência de operadores;
- recuperação de erros;
- dump da AST;
- preservação de spans;
- diagnósticos sintáticos estruturados.

A cobertura deve acompanhar o subconjunto sintático implementado. Construções especificadas mas ainda não habilitadas devem ter testes `ignored`, registro explícito de pendência, ou não devem ser incluídas como obrigatórias até entrarem no subconjunto.

---

## 25. Critérios de Aceite

Para este documento ser considerado implementado:

- testes unitários do parser existem e passam;
- testes de integração source/lexer/parser existem e passam;
- testes negativos de sintaxe existem e passam;
- testes de recovery existem e passam;
- testes de precedência existem e passam;
- testes de AST validam spans e estrutura;
- snapshots de `--emit ast` são determinísticos;
- CLI `capic --emit ast` é testada;
- regressões conhecidas possuem testes;
- `cargo test --workspace` passa;
- validação de Stage 2 passa na CI.

---

## 26. Relações Normativas

Este documento depende diretamente de:

- Documento 04 — Sintaxe da Linguagem;
- Documento 15 — Parser e AST;
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial;
- `TEST-STRATEGY.md`;
- `AST-MODEL.md`;
- `PARSER-IMPLEMENTATION.md`;
- `PARSER-RECOVERY.md`;
- `AST-LOWERING.md`;
- `TOKEN-MODEL.md`;
- `SPANS-AND-LOCATIONS.md`;
- `DIAGNOSTIC-DATA-MODEL.md`.

Este documento orienta diretamente:

- testes do crate `capi-parser`;
- testes de integração do frontend;
- testes de CLI para `capic --emit ast`;
- snapshots de AST;
- critérios de conclusão do Stage 2.
