# Feature Status

**Projeto:** Linguagem Capi  
**Documento:** FEATURE-STATUS  
**Status:** Aprovado  
**Stage:** Stages 0-2 — Fundação, frontend léxico e frontend sintático inicial  
**Natureza:** Registro de progresso  
**Base normativa:** Documento 28 — Plano de Desenvolvimento da Implementação Oficial

---

## Stage 0 — Fundação do projeto

```text
Stage: Stage 0 — Fundação do projeto
Responsável: Projeto Capi
Data de início: 2026-07-30
Data de conclusão: 2026-07-30
Status: Concluído
```

### Documentos concluídos

Documentos bloqueantes aprovados:

- `ENGINEERING-PRINCIPLES.md`
- `PROJECT-STRUCTURE.md`
- `COMPILER-ARCHITECTURE.md`
- `WORKSPACE-ARCHITECTURE.md`
- `COMPONENT-RESPONSIBILITIES.md`
- `DEPENDENCY-RULES.md`
- `DEVELOPMENT-SETUP.md`
- `BUILD-SYSTEM.md`
- `TEST-STRATEGY.md`
- `DEFINITION-OF-DONE.md`

Documentos operacionais e de consolidação aprovados:

- `ENGINEERING-GLOSSARY.md`
- `COMPILATION-PIPELINE.md`
- `BUILDING-FROM-SOURCE.md`
- `CODING-STANDARDS.md`
- `RUST-STYLE-GUIDE.md`

Documento operacional de referência aprovado:

- `28 — Plano de Desenvolvimento da Implementação Oficial.md`

### ADRs criados e aprovados

- `ADR-0001 — Rust como Linguagem da Implementação Oficial.md`
- `ADR-0002 — Organização da Implementação em Workspace Cargo.md`
- `ADR-0003 — Separação entre Frontend, Middle-end e Backend.md`
- `ADR-0013 — Política de Dependências Externas.md`
- `ADR-0015 — Estratégia Inicial de Testes.md`
- `ADR-0016 — Organização Física do Repositório.md`

### Infraestrutura concluída

- workspace Cargo criado em `capi-lang`;
- estrutura inicial de crates criada;
- `cargo fmt` configurado;
- `cargo clippy` configurado;
- execução de testes configurada;
- integração contínua configurada em `.github/workflows/capi-lang-ci.yml`;
- documentação Rust configurada;
- scripts de desenvolvimento configurados;
- política inicial de dependências definida;
- versões mínimas das ferramentas definidas.

### Implementações concluídas

- executável `capic` criado;
- crate `capi-driver` criado;
- crate `capi-session` criado;
- crate `capi-diagnostics` criado;
- crate `capi-source` criado;
- `capic --help` implementado;
- `capic --version` implementado;
- leitura inicial de argumentos implementada;
- inicialização da sessão implementada;
- tratamento básico de erros internos implementado.

### Testes concluídos

Validação executada em `capi-lang`:

```bash
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked
cargo test --workspace --locked
cargo run -p capi-cli --locked -- --help
cargo run -p capi-cli --locked -- --version
cargo run -p capi-cli --locked -- does-not-exist.capi
scripts/ci-local.sh
```

Resultado:

- build completo aprovado;
- formatação aprovada;
- Clippy aprovado;
- suíte inicial aprovada;
- `capic --help` aprovado;
- `capic --version` aprovado;
- arquivo inexistente retorna erro controlado;
- validação local equivalente à CI aprovada.

### Pendências

Nenhuma pendência bloqueante do Stage 0.

A execução remota do workflow no provedor de CI depende de `push` ou `pull_request`, mas o workflow está versionado e a validação local equivalente foi aprovada.

### Riscos

- As ADRs e documentos aprovados devem ser mantidos sincronizados com mudanças posteriores no workspace.
- Dependências externas futuras exigirão revisão explícita e possível ADR.
- A MSRV `1.88.0` deve ser preservada até decisão formal de atualização.

### Resultado da validação

Stage 0 concluído conforme os critérios do Documento 28.

O workspace Rust existe, compila, testa, gera documentação Rust, valida dependências, valida toolchain, executa `capic --help`, executa `capic --version`, reporta arquivo inexistente de forma controlada e possui CI configurada.

---

## Stage 1 — Fontes, diagnósticos e lexer

```text
Stage: Stage 1 — Fontes, diagnósticos e lexer
Responsável: Projeto Capi
Data de início: 2026-08-02
Data de conclusão: 2026-08-02
Status: Concluído
```

### Documentos concluídos

Documentos de fontes aprovados:

- `compiler/source/SOURCE-MODEL.md`
- `compiler/source/SOURCE-MAP.md`
- `compiler/source/SPANS-AND-LOCATIONS.md`
- `compiler/source/UNICODE-AND-ENCODING.md`

Documentos de frontend léxico aprovados:

- `compiler/frontend/TOKEN-MODEL.md`
- `compiler/frontend/LEXER-IMPLEMENTATION.md`

Documentos de diagnósticos aprovados:

- `compiler/diagnostics/DIAGNOSTIC-DATA-MODEL.md`
- `compiler/diagnostics/DIAGNOSTIC-ARCHITECTURE.md`
- `compiler/diagnostics/DIAGNOSTIC-STYLE-GUIDE.md`

Documento de testes aprovado:

- `testing/LEXER-TESTS.md`

READMEs e registros atualizados:

- `docs/README.md`
- `docs/engineering/README.md`
- `docs/engineering/compiler/README.md`
- `docs/engineering/testing/README.md`
- `docs/engineering/build-and-ci/README.md`
- `capi-docs/README.md`
- `capi-docs/CHANGELOG.md`
- `capi-lang/README.md`
- `README.md`
- `README PT-BR.md`

### ADRs criados e aprovados

Nenhuma ADR nova foi criada no Stage 1.

As decisões do Stage 1 foram implementadas dentro das regras já aprovadas no
Stage 0, especialmente:

- `ADR-0001 — Rust como Linguagem da Implementação Oficial.md`
- `ADR-0002 — Organização da Implementação em Workspace Cargo.md`
- `ADR-0003 — Separação entre Frontend, Middle-end e Backend.md`
- `ADR-0013 — Política de Dependências Externas.md`
- `ADR-0015 — Estratégia Inicial de Testes.md`
- `ADR-0016 — Organização Física do Repositório.md`

### Infraestrutura concluída

- crate `capi-source` consolidado;
- crate `capi-diagnostics` consolidado;
- crate `capi-lexer` criado;
- diretório `capi-lang/tests/lexer/pass/` criado;
- diretório `capi-lang/tests/lexer/fail/` criado;
- diretório `capi-lang/tests/lexer/snapshots/` criado;
- infraestrutura de testes de integração do lexer criada;
- snapshot inicial de dump de tokens criado;
- workflow de CI atualizado para validar `capic --emit tokens`;
- `scripts/check.sh` atualizado para validar o smoke test do Stage 1;
- configuração `.vscode/settings.json` adicionada para apontar o rust-analyzer para `capi-lang/Cargo.toml`.

### Implementações concluídas

- `SourceId` implementado;
- `SourceFile` implementado;
- `SourceMap` implementado;
- `Span` implementado;
- offsets, linha e coluna implementados;
- leitura de arquivo UTF-8 válido implementada;
- rejeição de UTF-8 inválido sem panic implementada;
- modelo de diagnósticos estruturados implementado;
- renderização inicial de diagnósticos implementada;
- modelo de tokens implementado;
- lexer implementado;
- identificadores implementados;
- palavras-chave implementadas;
- literais implementados;
- operadores implementados;
- delimitadores implementados;
- comentários de linha e bloco implementados;
- erros léxicos estruturados implementados;
- recuperação léxica básica implementada;
- EOF implementado;
- dump de tokens implementado via `capic --emit tokens arquivo.capi`.

### Testes concluídos

Validação executada em `capi-lang`:

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets
cargo run -p capi-cli -- --emit tokens tests/lexer/pass/basic.cap
scripts/check.sh
```

Cobertura obrigatória concluída:

- testes de `SourceMap`;
- testes de spans;
- testes de Unicode;
- testes de identificadores;
- testes de palavras-chave;
- testes de literais;
- testes de operadores;
- testes de delimitadores;
- testes de comentários;
- testes de tokens inválidos;
- testes de posição dos diagnósticos;
- testes de diagnósticos estruturados;
- testes de entradas malformadas sem panic;
- testes compile-fail léxicos;
- testes de CLI para `capic --emit tokens`;
- snapshot de dump de tokens.

Critérios de conclusão verificados:

- arquivos válidos são lidos corretamente;
- posições de erro são precisas;
- todos os tokens do subconjunto inicial são reconhecidos;
- entradas inválidas produzem diagnósticos estruturados;
- não há pânico em entradas malformadas;
- todos os testes obrigatórios passam.

### Resultado demonstrável

O resultado demonstrável do Stage 1 é:

```bash
capic --emit tokens arquivo.capi
```

Durante desenvolvimento local:

```bash
cd capi-lang
cargo run -p capi-cli -- --emit tokens tests/lexer/pass/basic.cap
```

Saída validada:

```text
0    Keyword(Let) tests/lexer/pass/basic.cap:1:1..1:4 "let"
1    Identifier tests/lexer/pass/basic.cap:1:5..1:10 "value"
2    Operator(Equal) tests/lexer/pass/basic.cap:1:11..1:12 "="
3    Literal(Integer) tests/lexer/pass/basic.cap:1:13..1:14 "1"
4    Delimiter(Semicolon) tests/lexer/pass/basic.cap:1:14..1:15 ";"
5    Eof tests/lexer/pass/basic.cap:2:1..2:1
```

### Pendências

Nenhuma pendência bloqueante do Stage 1.

Itens reservados para stages posteriores:

- HIR;
- resolução de nomes;
- checagem de tipos;
- análise de ownership;
- MIR;
- backend;
- runtime;
- biblioteca padrão.

### Riscos

- A lista de tokens inicial deve permanecer sincronizada com `TOKEN-MODEL.md` e com a especificação.
- A política Unicode de identificadores ainda é conservadora e deve ser revisitada quando nomes Unicode forem normativamente definidos.
- O formato de dump de tokens é inicial e deve permanecer determinístico enquanto for usado por snapshots.
- Novas dependências externas continuam proibidas sem revisão explícita conforme `DEPENDENCIES.md` e `ADR-0013`.
- A MSRV `1.88.0` deve ser preservada até decisão formal de atualização.

### Resultado da validação

Stage 1 concluído conforme os critérios do Documento 28.

O workspace Rust lê fontes válidas, rejeita UTF-8 inválido sem panic, resolve
spans e localizações, produz diagnósticos estruturados, reconhece todos os tokens
do subconjunto inicial, recupera erros léxicos básicos, emite dump de tokens por
CLI e passa todos os testes obrigatórios definidos para o stage.

---

## Stage 2 — Parser e AST

```text
Stage: Stage 2 — Parser e AST
Responsável: Projeto Capi
Data de início: 2026-08-02
Data de conclusão: 2026-08-02
Status: Concluído
```

### Documentos concluídos

Documentos de frontend sintático aprovados:

- `compiler/frontend/AST-MODEL.md`
- `compiler/frontend/PARSER-IMPLEMENTATION.md`
- `compiler/frontend/PARSER-RECOVERY.md`
- `compiler/frontend/AST-LOWERING.md`

Documento de testes aprovado:

- `testing/PARSER-TESTS.md`

READMEs e registros atualizados:

- `docs/engineering/compiler/README.md`
- `docs/engineering/testing/README.md`
- `docs/engineering/planning/FEATURE-STATUS.md`

### ADRs criados e aprovados

Nenhuma ADR nova foi criada no Stage 2.

As decisões do Stage 2 foram implementadas dentro das regras já aprovadas nos
stages anteriores, especialmente:

- `ADR-0001 — Rust como Linguagem da Implementação Oficial.md`
- `ADR-0002 — Organização da Implementação em Workspace Cargo.md`
- `ADR-0003 — Separação entre Frontend, Middle-end e Backend.md`
- `ADR-0013 — Política de Dependências Externas.md`
- `ADR-0015 — Estratégia Inicial de Testes.md`
- `ADR-0016 — Organização Física do Repositório.md`

### Infraestrutura concluída

- crate `capi-ast` criado;
- crate `capi-parser` criado;
- integração de `capi-ast` e `capi-parser` ao workspace Cargo;
- integração do parser ao `capi-driver`;
- integração de `--emit ast` à CLI `capic`;
- infraestrutura de testes de integração do parser criada;
- fixtures de dump de AST criadas em `capi-parser/tests/fixtures/ast_dump/`;
- snapshots golden de dump de AST criados;
- testes de CLI para `capic --emit ast` adicionados.

### Implementações concluídas

- modelo inicial de AST implementado;
- unidade de compilação implementada;
- nós de módulo e imports implementados;
- nós de declarações implementados;
- nós de classes, interfaces, traits e membros implementados;
- nós de funções, construtores, parâmetros e generics implementados;
- nós de tipos sintáticos implementados;
- nós de blocos e comandos implementados;
- nós de expressões implementados;
- nós de padrões implementados;
- nós de erro sintático implementados;
- preservação de spans em nós sintáticos relevantes implementada;
- parser de módulos implementado;
- parser de imports implementado;
- parser de declarações implementado;
- parser de classes, interfaces e traits implementado;
- parser de funções, construtores e parâmetros implementado;
- parser de tipos implementado;
- parser de expressões implementado;
- parser de comandos implementado;
- precedência e associatividade de operadores implementadas;
- diagnósticos sintáticos estruturados com códigos `PARSE` implementados;
- recuperação de erros recuperáveis implementada;
- AST parcial com nós de erro implementada;
- dump determinístico da AST implementado;
- emissão de AST via `capic --emit ast arquivo.capi` implementada.

### Testes concluídos

Validação executada em `capi-lang`:

```bash
cargo fmt --all --check
cargo test -p capi-parser parses_declarations
cargo test -p capi-parser parses_expressions
cargo test -p capi-parser parses_operator_precedence
cargo test -p capi-parser parses_types
cargo test -p capi-parser parses_classes
cargo test -p capi-parser reports_syntax_errors
cargo test -p capi-parser recovers_after_syntax_errors
cargo test -p capi-parser builds_ast_with_spans_and_dump
cargo test --workspace
cargo clippy --workspace --all-targets
```

Cobertura obrigatória concluída:

- testes de declarações;
- testes de expressões;
- testes de precedência;
- testes de tipos;
- testes de classes;
- testes de erros sintáticos;
- testes de recuperação;
- testes de AST;
- testes de spans por nó específico;
- testes de nós de erro;
- testes golden de dump determinístico da AST;
- testes de CLI para `capic --emit ast`.

Critérios de conclusão verificados:

- o subconjunto sintático inicial é aceito;
- entradas inválidas produzem diagnósticos sintáticos adequados;
- o parser continua após erros recuperáveis;
- a AST preserva spans em nós relevantes;
- o dump da AST é determinístico;
- o resultado esperado pode ser obtido por `capic --emit ast arquivo.capi`;
- todos os testes obrigatórios passam.

### Resultado demonstrável

O resultado demonstrável do Stage 2 é:

```bash
capic --emit ast arquivo.capi
```

Durante desenvolvimento local:

```bash
cd capi-lang
cargo run -p capi-cli --bin capic -- --emit ast crates/capi-parser/tests/fixtures/ast_dump/basic.cap
```

Saída validada:

```text
CompilationUnit span=0..363
  ModuleDecl path=banco.contas span=0..20
  ImportDecl path=banco.Cliente span=21..42
  ClassDecl name=Cliente span=51..252
    Modifier Public span=44..50
    Extends Path Pessoa span=76..82
    Implements Path Autenticavel span=94..106
    Uses Path Logavel span=112..119
```

A saída completa é validada por snapshot golden em:

```text
capi-lang/crates/capi-parser/tests/fixtures/ast_dump/basic.ast
capi-lang/crates/capi-parser/tests/fixtures/ast_dump/recovery.ast
```

### Pendências

Nenhuma pendência bloqueante do Stage 2.

Itens reservados para stages posteriores:

- HIR;
- lowering efetivo de AST para HIR;
- resolução de nomes;
- tabelas de símbolos;
- escopos;
- checagem de tipos;
- análise de ownership;
- MIR;
- backend;
- runtime;
- biblioteca padrão;
- recuperação sofisticada de IDE/LSP.

### Riscos

- O subconjunto sintático inicial deve permanecer sincronizado com os documentos
  de especificação e com `PARSER-IMPLEMENTATION.md`.
- O formato de dump da AST é contrato observado e deve ser alterado apenas com
  atualização intencional dos snapshots golden.
- A recuperação sintática atual é suficiente para o Stage 2, mas recursos de
  IDE/LSP exigirão estratégia incremental em stage futuro.
- A AST preserva spans sintáticos, mas o consumo desses spans por HIR,
  resolução de nomes e diagnósticos semânticos ainda depende dos próximos
  stages.
- Novas dependências externas continuam proibidas sem revisão explícita conforme
  `DEPENDENCIES.md` e `ADR-0013`.
- A MSRV `1.88.0` deve ser preservada até decisão formal de atualização.

### Resultado da validação

Stage 2 concluído conforme os critérios do Documento 28.

O workspace Rust reconhece o subconjunto sintático inicial, constrói AST com
spans, emite diagnósticos sintáticos estruturados, recupera erros recuperáveis,
produz AST parcial com nós de erro, emite dump determinístico de AST por CLI e
passa todos os testes obrigatórios definidos para o stage.
