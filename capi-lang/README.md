# Capi Language Implementation

This directory contains the official implementation of the Capi language.

The implementation is currently in Stage 1: source infrastructure,
diagnostics, and the initial lexer.

The Cargo workspace, fundamental compiler crates, source handling,
structured diagnostics, token model, and lexer token dump are available.

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

The Stage 1 lexer demonstration is:

```bash
cargo run -p capi-cli -- --emit tokens tests/lexer/pass/basic.cap
```

The user-facing form is:

```bash
capic --emit tokens arquivo.capi
```

## Crates

Current workspace crates:

| Crate | Responsibility |
| --- | --- |
| `capi-cli` | `capic` executable and argument parsing. |
| `capi-driver` | Compiler driver orchestration and token dump output. |
| `capi-common` | Shared primitive project constants and status types. |
| `capi-session` | Compilation session state. |
| `capi-source` | `SourceId`, `SourceFile`, `SourceMap`, `Span`, line and column lookup. |
| `capi-diagnostics` | Structured diagnostics, labels, notes, suggestions and rendering. |
| `capi-lexer` | Token model and lexer for the initial frontend subset. |

## Lexer tests

Stage 1 lexical fixtures live in:

```text
tests/lexer/pass/
tests/lexer/fail/
tests/lexer/snapshots/
```

They validate source loading, spans, Unicode, token recognition, invalid inputs,
diagnostic positions, structured diagnostics and non-panic behavior for
malformed input.

Dependency policy is recorded in:

```text
DEPENDENCIES.md
```

Minimum tool versions are recorded in:

```text
TOOLCHAIN.md
```

The same validation sequence, including `capic` smoke checks, can be run with:

```bash
scripts/check.sh
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
