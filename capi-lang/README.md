# Capi Language Implementation

This directory contains the official Rust implementation workspace for the Capi
language.

The implementation is currently past Stage 3: HIR and name resolution. It does
not compile Capi programs yet, but the frontend can emit tokens, AST, and
resolved HIR dumps.

## Workspace

The workspace root is this directory:

```bash
cd capi-lang
```

Standard validation commands:

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets
cargo build --workspace
cargo doc --workspace --no-deps
```

The consolidated local validation, including `capic` smoke checks, is:

```bash
./scripts/check.sh
```

## Demonstrable Commands

Current user-facing compiler dumps:

```bash
capic --emit tokens arquivo.capi
capic --emit ast arquivo.capi
capic --emit hir arquivo.capi
```

Equivalent local development commands:

```bash
cargo run -p capi-cli -- --emit tokens tests/lexer/pass/basic.cap
cargo run -p capi-cli --bin capic -- --emit ast crates/capi-parser/tests/fixtures/ast_dump/basic.cap
cargo run -p capi-cli -- --emit hir tests/semantic/pass/basic.cap
```

## Crates

Current workspace crates:

| Crate | Responsibility |
| --- | --- |
| `capi-cli` | `capic` executable and argument parsing. |
| `capi-driver` | Compiler driver orchestration and dump outputs. |
| `capi-common` | Shared primitive project constants and status types. |
| `capi-session` | Compilation session state. |
| `capi-source` | `SourceId`, `SourceFile`, `SourceMap`, `Span`, line and column lookup. |
| `capi-diagnostics` | Structured diagnostics, labels, notes, suggestions and rendering. |
| `capi-lexer` | Token model and lexer for the initial frontend subset. |
| `capi-ast` | Abstract syntax tree model and deterministic AST dump. |
| `capi-parser` | Parser, syntax diagnostics, recovery, and AST construction. |
| `capi-hir` | High-level intermediate representation model and deterministic HIR dump. |
| `capi-lowering` | AST-to-HIR lowering. |
| `capi-sema` | Scope graph, symbol table, and initial name resolution. |

## Tests and Fixtures

Lexer fixtures live in:

```text
tests/lexer/pass/
tests/lexer/fail/
tests/lexer/snapshots/
```

Parser AST dump fixtures live in:

```text
crates/capi-parser/tests/fixtures/ast_dump/
```

Semantic fixtures and snapshots live in:

```text
tests/semantic/pass/
tests/semantic/fail/
tests/semantic/snapshots/
```

They validate source loading, spans, Unicode, token recognition, parser
recovery, AST dumps, AST-to-HIR lowering, scopes, symbols, name resolution,
semantic diagnostics, deterministic dumps, and non-panic behavior for malformed
input.

## Policies

Dependency policy is recorded in:

```text
DEPENDENCIES.md
```

Minimum tool versions are recorded in:

```text
TOOLCHAIN.md
```

Rust API documentation can be generated directly with:

```bash
scripts/doc.sh
```

Development scripts:

```bash
scripts/fmt.sh
scripts/fmt-check.sh
scripts/tools.sh
scripts/deps.sh
scripts/lint.sh
scripts/test.sh
scripts/build.sh
scripts/doc.sh
scripts/check.sh
scripts/ci-local.sh
```
