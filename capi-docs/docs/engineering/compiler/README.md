# Compiler

Esta pasta reúne a documentação de engenharia específica do compilador oficial
da Linguagem Capi.

Ela transforma a especificação da linguagem em contratos implementáveis para
fontes, diagnósticos, frontend, análise semântica, memória, IR intermediário,
modelo de objetos e geração de código. Documentos nesta área não redefinem a
semântica normativa da linguagem; eles descrevem como a implementação oficial
deve materializar essa semântica.

Quando houver conflito, prevalecem a especificação normativa, as ADRs aprovadas
e os documentos de engenharia bloqueantes do stage atual.

---

## Estado Atual

O Stage 3 do compilador está concentrado na HIR, no lowering da AST para HIR e
na primeira etapa semântica de escopos, símbolos e resolução de nomes.

O Stage 1 entregou a infraestrutura de fontes, diagnósticos, tokens e lexer:

* leitura e armazenamento de arquivos fonte;
* `SourceId`, `SourceFile`, `SourceMap`, `Span`, linha e coluna;
* modelo de diagnósticos estruturados;
* modelo de tokens;
* lexer do subconjunto inicial;
* comentários, literais, operadores, delimitadores, identificadores e keywords;
* erros léxicos estruturados;
* dump de tokens via `capic --emit tokens arquivo.capi`;
* testes obrigatórios de source, spans, Unicode, lexer, diagnósticos e entradas
  malformadas.

O Stage 2 ampliou essa base com parser e AST:

* crate de AST com nós para unidade de compilação, módulos, imports,
  declarações, classes, funções, tipos, comandos, expressões, padrões e nós de
  erro;
* preservação de spans em nós sintáticos relevantes;
* parser do subconjunto sintático inicial;
* parsing de módulos, imports, declarações, classes, funções, tipos, comandos e
  expressões;
* precedência e associatividade de operadores;
* diagnósticos sintáticos estruturados com códigos `PARSE`;
* recuperação de erros recuperáveis com AST parcial;
* dump determinístico da AST via `capic --emit ast arquivo.capi`;
* testes obrigatórios de declarações, expressões, precedência, tipos, classes,
  erros sintáticos, recuperação, spans e dump da AST.

O Stage 3 amplia o frontend com HIR e análise semântica inicial:

* `capi-hir` como modelo HIR puro, sem dependência direta da AST;
* `capi-lowering` como fronteira AST -> HIR;
* IDs HIR tipados e determinísticos;
* preservação de `SourceId`, spans e mapeamento AST-HIR no lowering;
* tabelas de símbolos com `SymbolId` interno;
* grafo de escopos com `ScopeId` interno;
* registro de módulos, imports, itens, membros, parâmetros, locais e patterns;
* resolução de nomes para o subconjunto inicial;
* diagnóstico de símbolos duplicados, referências inexistentes e ambiguidades;
* dump determinístico de HIR resolvida via `capic --emit hir arquivo.capi`;
* testes obrigatórios de lowering, HIR, escopos, símbolos, resolução,
  diagnósticos semânticos e CLI.

A implementação correspondente vive em:

```text
../../../../capi-lang/crates/capi-source/
../../../../capi-lang/crates/capi-diagnostics/
../../../../capi-lang/crates/capi-lexer/
../../../../capi-lang/crates/capi-ast/
../../../../capi-lang/crates/capi-parser/
../../../../capi-lang/crates/capi-hir/
../../../../capi-lang/crates/capi-lowering/
../../../../capi-lang/crates/capi-sema/
../../../../capi-lang/crates/capi-driver/
../../../../capi-lang/crates/capi-cli/
```

---

## Documentos Ativos dos Stages 1, 2 e 3

### Fontes

| Documento | Status | Finalidade |
| --- | --- | --- |
| `source/SOURCE-MODEL.md` | Aprovado | Define o modelo de arquivo fonte, identidade de fonte e texto carregado. |
| `source/SOURCE-MAP.md` | Aprovado | Define o mapa de fontes, armazenamento, consulta e resolução de spans. |
| `source/SPANS-AND-LOCATIONS.md` | Aprovado | Define offsets, spans, linha, coluna e regras de localização. |
| `source/UNICODE-AND-ENCODING.md` | Aprovado | Define política inicial de UTF-8, Unicode, BOM e limites de slice. |

### Frontend

| Documento | Status | Finalidade |
| --- | --- | --- |
| `frontend/TOKEN-MODEL.md` | Aprovado | Define o contrato de tokens, keywords, literais, operadores e delimitadores. |
| `frontend/LEXER-IMPLEMENTATION.md` | Aprovado | Define a implementação do lexer, recuperação e dump de tokens. |
| `frontend/AST-MODEL.md` | Aprovado | Define o modelo da AST, nós sintáticos, spans, nós de erro e dump determinístico. |
| `frontend/PARSER-IMPLEMENTATION.md` | Aprovado | Define estratégia de parsing, contratos de entrada e saída, precedência e integração com AST. |
| `frontend/PARSER-RECOVERY.md` | Aprovado | Define recuperação sintática, sincronização, diagnósticos e AST parcial. |
| `frontend/AST-LOWERING.md` | Aprovado | Define o contrato de lowering da AST para HIR e seus limites para o Stage 3. |

### Semântica Inicial

| Documento | Status | Finalidade |
| --- | --- | --- |
| `semantic/HIR-MODEL.md` | Aprovado | Define a representação semântica de alto nível, IDs HIR, origem, estrutura e dump. |
| `semantic/SYMBOL-MODEL.md` | Aprovado | Define símbolos, namespaces, identidade interna e tabela de símbolos. |
| `semantic/SCOPE-MODEL.md` | Aprovado | Define escopos, owners, hierarquia e relação HIR-escopo. |
| `semantic/NAME-RESOLUTION.md` | Aprovado | Define resolução de nomes, bindings e diagnósticos de resolução. |

### Diagnósticos

| Documento | Status | Finalidade |
| --- | --- | --- |
| `diagnostics/DIAGNOSTIC-DATA-MODEL.md` | Aprovado | Define severidade, código, span primário, labels, notas e sugestões. |
| `diagnostics/DIAGNOSTIC-ARCHITECTURE.md` | Aprovado | Define fluxo de produção, agregação e renderização de diagnósticos. |
| `diagnostics/DIAGNOSTIC-STYLE-GUIDE.md` | Aprovado | Define estilo de mensagens, labels e notas para diagnósticos. |

Esses documentos formam o contrato operacional dos Stages 1, 2 e 3.

---

## Documentos Reservados

### Diagnósticos

| Documento | Finalidade esperada |
| --- | --- |
| `diagnostics/ERROR-CODE-POLICY.md` | Consolidar política completa de códigos de erro. |
| `diagnostics/OUTPUT-FORMATS.md` | Definir formatos humano, JSON e possíveis formatos de tooling. |
| `diagnostics/INTERNAL-COMPILER-ERRORS.md` | Definir política de ICEs, invariantes e mensagens internas. |

### Semântica

| Documento | Finalidade esperada |
| --- | --- |
| `semantic/TYPE-MODEL.md` | Definir representação de tipos. |
| `semantic/TYPE-INFERENCE.md` | Definir inferência de tipos. |
| `semantic/TYPE-CHECKING-PIPELINE.md` | Definir pipeline de checagem semântica. |
| `semantic/TYPE-INTERNING.md` | Definir interning/canonicalização de tipos. |
| `semantic/GENERICS-IMPLEMENTATION.md` | Definir implementação de generics. |
| `semantic/SUBTYPING-AND-COERCIONS.md` | Definir subtipagem e coerções. |

### Memória

| Documento | Finalidade esperada |
| --- | --- |
| `memory/OWNERSHIP-MODEL.md` | Definir modelo operacional de ownership. |
| `memory/BORROW-CHECKER.md` | Definir verificador de empréstimos. |
| `memory/REGION-ANALYSIS.md` | Definir análise de regiões. |
| `memory/ESCAPE-ANALYSIS.md` | Definir análise de escape. |
| `memory/DROP-SEMANTICS.md` | Definir destruição e ordem de drop. |
| `memory/PLACE-AND-ACCESS-PATHS.md` | Definir places, paths de acesso e projeções. |
| `memory/DOMAIN-IMPLEMENTATION.md` | Definir implementação de domínios explícitos da linguagem. |

### MIR

| Documento | Finalidade esperada |
| --- | --- |
| `mir/MIR-MODEL.md` | Definir representação intermediária de nível médio. |
| `mir/MIR-LOWERING.md` | Definir lowering para MIR. |
| `mir/MIR-PASSES.md` | Definir passes de transformação e análise. |
| `mir/MIR-INVARIANTS.md` | Definir invariantes obrigatórios. |
| `mir/MIR-VALIDATION.md` | Definir validação estrutural da MIR. |
| `mir/MIR-DUMP-FORMAT.md` | Definir formato textual de dump da MIR. |

### Modelo de Objetos

| Documento | Finalidade esperada |
| --- | --- |
| `object-model/OBJECT-MODEL.md` | Definir classes, traits, interfaces e entidades de objeto. |
| `object-model/OBJECT-LAYOUT.md` | Definir layout de objetos. |
| `object-model/OBJECT-IDENTITY.md` | Definir identidade, referência e comparação de objetos. |
| `object-model/INHERITANCE-IMPLEMENTATION.md` | Definir implementação de herança. |
| `object-model/VTABLES.md` | Definir tabelas virtuais. |
| `object-model/DYNAMIC-DISPATCH.md` | Definir despacho dinâmico. |

### Codegen

| Documento | Finalidade esperada |
| --- | --- |
| `codegen/CODEGEN-ARCHITECTURE.md` | Definir arquitetura de geração de código. |
| `codegen/BACKEND-INTERFACE.md` | Definir interface comum para backends. |
| `codegen/CRANELIFT-BACKEND.md` | Definir backend Cranelift. |
| `codegen/LLVM-BACKEND.md` | Definir backend LLVM. |
| `codegen/TARGETS.md` | Definir targets suportados. |
| `codegen/OBJECT-FILES.md` | Definir geração de objetos. |
| `codegen/LINKING.md` | Definir linking. |
| `codegen/BACKEND-COMPATIBILITY.md` | Definir compatibilidade e divergências entre backends. |

Enquanto esses documentos estiverem vazios ou não aprovados, eles não introduzem
obrigações próprias para a implementação.

---

## Resultado Demonstrável dos Stages 1, 2 e 3

O resultado observável mínimo do lexer é:

```bash
capic --emit tokens arquivo.capi
```

Durante desenvolvimento local, execute a partir de `capi-lang/`:

```bash
cargo run -p capi-cli -- --emit tokens tests/lexer/pass/basic.cap
```

O comando deve imprimir tokens com:

* índice;
* categoria;
* arquivo;
* linha e coluna inicial;
* linha e coluna final;
* lexema preservado quando aplicável.

Entradas inválidas devem produzir diagnóstico estruturado, sem panic.

O resultado observável mínimo do parser e da AST é:

```bash
capic --emit ast arquivo.capi
```

Durante desenvolvimento local, execute a partir de `capi-lang/`:

```bash
cargo run -p capi-cli --bin capic -- --emit ast crates/capi-parser/tests/fixtures/ast_dump/basic.cap
```

O comando deve imprimir um dump determinístico da AST com:

* unidade de compilação;
* módulos e imports;
* declarações e membros;
* tipos, comandos e expressões;
* nós de erro quando a entrada for recuperável;
* spans em formato `inicio..fim`.

Para entradas sintaticamente inválidas mas recuperáveis, o driver pode retornar
falha e ainda assim emitir a AST parcial junto dos diagnósticos.

O resultado observável mínimo da HIR e da resolução inicial de nomes é:

```bash
capic --emit hir arquivo.capi
```

Durante desenvolvimento local, execute a partir de `capi-lang/`:

```bash
cargo run -p capi-cli -- --emit hir tests/semantic/pass/basic.cap
```

O comando deve imprimir um dump determinístico com:

* unidade HIR;
* módulos e imports;
* itens, blocos, comandos, expressões e tipos pendentes;
* escopos;
* símbolos;
* bindings de nomes resolvidos.

Entradas com erro semântico de resolução devem produzir diagnóstico estruturado
e retornar falha controlada. O dump pode exibir bindings como `not_found` ou
`ambiguous` quando a HIR resolvida parcial for útil para depuração.

---

## Critérios de Conclusão do Stage 1

O Stage 1 é considerado concluído quando:

* arquivos válidos são lidos corretamente;
* posições de erro são precisas;
* todos os tokens do subconjunto inicial são reconhecidos;
* entradas inválidas produzem diagnósticos estruturados;
* não há pânico em entradas malformadas;
* todos os testes obrigatórios passam.

Esses critérios foram cobertos por testes nos crates `capi-source`,
`capi-diagnostics`, `capi-lexer`, `capi-driver` e `capi-cli`.

---

## Critérios de Conclusão do Stage 2

O Stage 2 é considerado concluído quando:

* o subconjunto sintático inicial é aceito;
* entradas inválidas produzem diagnósticos sintáticos adequados;
* o parser continua após erros recuperáveis;
* a AST preserva spans em nós relevantes;
* o dump da AST é determinístico;
* o resultado esperado pode ser obtido por `capic --emit ast arquivo.capi`;
* todos os testes obrigatórios de parser e AST passam.

Esses critérios são cobertos por testes nos crates `capi-ast`, `capi-parser`,
`capi-driver` e `capi-cli`, incluindo testes de declarações, expressões,
precedência, tipos, classes, erros sintáticos, recuperação, spans e snapshots
golden do dump da AST.

---

## Critérios de Conclusão do Stage 3

O Stage 3 é considerado concluído quando:

* lowering de AST para HIR existe em `capi-lowering`;
* `capi-hir` permanece como modelo HIR puro, sem dependência direta da AST;
* IDs HIR, `ScopeId` e `SymbolId` são internos, tipados e determinísticos dentro
  da análise;
* símbolos do subconjunto inicial são registrados em tabela de símbolos;
* escopos do subconjunto inicial são construídos em grafo determinístico;
* módulos e imports do subconjunto inicial são representados;
* todos os nomes resolvíveis do subconjunto inicial são resolvidos;
* símbolos duplicados, referências inexistentes e ambiguidades produzem
  diagnósticos estruturados;
* HIR resolvida pode ser emitida por `capic --emit hir arquivo.capi`;
* todos os testes obrigatórios passam.

Esses critérios são cobertos por testes nos crates `capi-hir`,
`capi-lowering`, `capi-sema`, `capi-driver` e `capi-cli`, incluindo testes de
lowering, IDs, símbolos, escopos, resolução, diagnósticos semânticos, snapshots
e CLI.

---

## Comandos Canônicos de Validação

Execute a partir de `capi-lang/`:

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets
cargo run -p capi-cli -- --emit tokens tests/lexer/pass/basic.cap
cargo run -p capi-cli --bin capic -- --emit ast crates/capi-parser/tests/fixtures/ast_dump/basic.cap
cargo run -p capi-cli -- --emit hir tests/semantic/pass/basic.cap
```

Esses comandos validam formatação, testes obrigatórios, lint, o resultado
demonstrável do lexer, o resultado demonstrável do parser/AST e o resultado
demonstrável da HIR resolvida.

A validação consolidada do workspace é:

```bash
./scripts/check.sh
```

---

## Ordem de Leitura Recomendada

Para entender os Stages 1, 2 e 3, leia nesta ordem:

1. `source/SOURCE-MODEL.md`
2. `source/SOURCE-MAP.md`
3. `source/SPANS-AND-LOCATIONS.md`
4. `source/UNICODE-AND-ENCODING.md`
5. `diagnostics/DIAGNOSTIC-DATA-MODEL.md`
6. `diagnostics/DIAGNOSTIC-ARCHITECTURE.md`
7. `diagnostics/DIAGNOSTIC-STYLE-GUIDE.md`
8. `frontend/TOKEN-MODEL.md`
9. `frontend/LEXER-IMPLEMENTATION.md`
10. `../testing/LEXER-TESTS.md`
11. `frontend/AST-MODEL.md`
12. `frontend/PARSER-IMPLEMENTATION.md`
13. `frontend/PARSER-RECOVERY.md`
14. `frontend/AST-LOWERING.md`
15. `../testing/PARSER-TESTS.md`
16. `semantic/HIR-MODEL.md`
17. `semantic/SCOPE-MODEL.md`
18. `semantic/SYMBOL-MODEL.md`
19. `semantic/NAME-RESOLUTION.md`
20. `../testing/SEMANTIC-TESTS.md`

Essa ordem começa pelo modelo de fontes, conecta localização e diagnósticos,
passa pelo contrato de tokens, lexer, AST, parser, recuperação, lowering e
termina na HIR, escopos, símbolos, resolução de nomes e estratégia de testes
semânticos.

---

## Relação com Outras Áreas

Documentos relacionados:

```text
../architecture/COMPILATION-PIPELINE.md
../architecture/COMPONENT-RESPONSIBILITIES.md
../architecture/DEPENDENCY-RULES.md
../testing/TEST-STRATEGY.md
../testing/LEXER-TESTS.md
../testing/PARSER-TESTS.md
../testing/SEMANTIC-TESTS.md
../planning/DEFINITION-OF-DONE.md
../planning/FEATURE-STATUS.md
../../specification/README.md
../../adr/README.md
```

Mudanças no pipeline, no workspace, nos critérios de aceite ou na especificação
devem ser refletidas aqui quando alterarem responsabilidades do compilador.

---

## Critério de Atualização

Atualize este README quando:

* um documento desta pasta for preenchido ou aprovado;
* um novo stage do compilador começar ou terminar;
* uma fase nova do compilador for implementada;
* os crates do compilador mudarem de responsabilidade;
* o comando demonstrável do stage mudar;
* os critérios de conclusão forem alterados;
* a ordem recomendada de leitura deixar de representar o fluxo real.
