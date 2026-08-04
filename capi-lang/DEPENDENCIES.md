# Dependency Policy

This file records the dependency policy for the official Capi implementation.

The authoritative engineering rules live in:

- `capi-docs/docs/engineering/architecture/DEPENDENCY-RULES.md`
- `capi-docs/docs/adr/ADR-0013 — Política de Dependências Externas.md`

## Current Policy

Stage 3 continues with no external Rust crate dependencies.

Allowed dependencies are only workspace path dependencies between Capi crates:

```text
capi-ast -> capi-lexer
capi-ast -> capi-source
capi-cli -> capi-driver
capi-driver -> capi-ast
capi-driver -> capi-common
capi-driver -> capi-diagnostics
capi-driver -> capi-hir
capi-driver -> capi-lexer
capi-driver -> capi-lowering
capi-driver -> capi-parser
capi-driver -> capi-sema
capi-driver -> capi-session
capi-driver -> capi-source
capi-hir -> capi-lexer
capi-hir -> capi-source
capi-lowering -> capi-ast
capi-lowering -> capi-diagnostics
capi-lowering -> capi-hir
capi-lowering -> capi-source
capi-lowering [dev] -> capi-lexer
capi-lowering [dev] -> capi-parser
capi-parser -> capi-ast
capi-parser -> capi-diagnostics
capi-parser -> capi-lexer
capi-parser -> capi-source
capi-sema -> capi-diagnostics
capi-sema -> capi-hir
capi-sema -> capi-source
capi-sema [dev] -> capi-lexer
capi-sema [dev] -> capi-lowering
capi-sema [dev] -> capi-parser
capi-session -> capi-diagnostics
capi-session -> capi-source
capi-source -> capi-common
capi-diagnostics -> capi-common
capi-diagnostics -> capi-source
capi-lexer -> capi-diagnostics
capi-lexer -> capi-source
```

The lockfile must remain versioned:

```text
capi-lang/Cargo.lock
```

## Adding Dependencies

External dependencies must not be added casually.

Before adding one, the proposal must answer:

- what problem it solves;
- why local code or the Rust standard library is not enough;
- its license;
- its MSRV impact;
- its transitive dependency count;
- whether it affects final artifacts;
- whether it affects bootstrap;
- whether it is structural and requires an ADR;
- how it is tested;
- how it can be removed.

Structural dependencies require an ADR before adoption.

Examples include compiler backends, snapshot frameworks that define public output, stable serialization infrastructure, and public CLI parsing libraries.

## Validation

The dependency check is:

```bash
scripts/deps.sh
```

It validates the Cargo metadata and rejects external dependency sources in `Cargo.lock`.

If an external dependency is approved in a future stage, this file and `scripts/deps.sh` must be updated in the same change.
