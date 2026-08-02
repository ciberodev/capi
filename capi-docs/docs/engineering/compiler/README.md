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

O Stage 1 do compilador está concentrado na infraestrutura inicial:

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

A implementação correspondente vive em:

```text
../../../../capi-lang/crates/capi-source/
../../../../capi-lang/crates/capi-diagnostics/
../../../../capi-lang/crates/capi-lexer/
../../../../capi-lang/crates/capi-driver/
../../../../capi-lang/crates/capi-cli/
```

---

## Documentos Ativos do Stage 1

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

### Diagnósticos

| Documento | Status | Finalidade |
| --- | --- | --- |
| `diagnostics/DIAGNOSTIC-DATA-MODEL.md` | Aprovado | Define severidade, código, span primário, labels, notas e sugestões. |
| `diagnostics/DIAGNOSTIC-ARCHITECTURE.md` | Aprovado | Define fluxo de produção, agregação e renderização de diagnósticos. |
| `diagnostics/DIAGNOSTIC-STYLE-GUIDE.md` | Aprovado | Define estilo de mensagens, labels e notas para diagnósticos. |

Esses documentos formam o contrato operacional mínimo do Stage 1.

---

## Documentos Reservados

### Frontend

| Documento | Finalidade esperada |
| --- | --- |
| `frontend/AST-MODEL.md` | Definir a árvore sintática abstrata quando o parser for implementado. |
| `frontend/PARSER-IMPLEMENTATION.md` | Definir estratégia de parsing, gramática operacional e contratos de saída. |
| `frontend/PARSER-RECOVERY.md` | Definir recuperação de erros sintáticos. |
| `frontend/AST-LOWERING.md` | Definir transformação de AST para representação semântica inicial. |

### Diagnósticos

| Documento | Finalidade esperada |
| --- | --- |
| `diagnostics/ERROR-CODE-POLICY.md` | Consolidar política completa de códigos de erro. |
| `diagnostics/OUTPUT-FORMATS.md` | Definir formatos humano, JSON e possíveis formatos de tooling. |
| `diagnostics/INTERNAL-COMPILER-ERRORS.md` | Definir política de ICEs, invariantes e mensagens internas. |

### Semântica

| Documento | Finalidade esperada |
| --- | --- |
| `semantic/SYMBOL-MODEL.md` | Definir símbolos, entidades nomeadas e tabelas semânticas. |
| `semantic/SCOPE-MODEL.md` | Definir escopos, blocos, módulos e visibilidade. |
| `semantic/NAME-RESOLUTION.md` | Definir resolução de nomes. |
| `semantic/TYPE-MODEL.md` | Definir representação de tipos. |
| `semantic/TYPE-INFERENCE.md` | Definir inferência de tipos. |
| `semantic/TYPE-CHECKING-PIPELINE.md` | Definir pipeline de checagem semântica. |
| `semantic/TYPE-INTERNING.md` | Definir interning/canonicalização de tipos. |
| `semantic/GENERICS-IMPLEMENTATION.md` | Definir implementação de generics. |
| `semantic/SUBTYPING-AND-COERCIONS.md` | Definir subtipagem e coerções. |
| `semantic/HIR-MODEL.md` | Definir representação semântica de alto nível. |

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

## Resultado Demonstrável do Stage 1

O resultado observável mínimo do frontend inicial é:

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

## Comandos Canônicos de Validação

Execute a partir de `capi-lang/`:

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets
cargo run -p capi-cli -- --emit tokens tests/lexer/pass/basic.cap
```

Esses comandos validam formatação, testes obrigatórios, lint e o resultado
demonstrável do lexer.

---

## Ordem de Leitura Recomendada

Para entender o Stage 1, leia nesta ordem:

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

Essa ordem começa pelo modelo de fontes, conecta localização e diagnósticos,
passa pelo contrato de tokens e termina na estratégia de testes léxicos.

---

## Relação com Outras Áreas

Documentos relacionados:

```text
../architecture/COMPILATION-PIPELINE.md
../architecture/COMPONENT-RESPONSIBILITIES.md
../architecture/DEPENDENCY-RULES.md
../testing/TEST-STRATEGY.md
../testing/LEXER-TESTS.md
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
