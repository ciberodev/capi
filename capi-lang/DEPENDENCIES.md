# Dependency Policy

This file records the initial dependency policy for the official Capi implementation.

The authoritative engineering rules live in:

- `capi-docs/docs/engineering/architecture/DEPENDENCY-RULES.md`
- `capi-docs/docs/adr/ADR-0013 — Política de Dependências Externas.md`

## Stage 0 Policy

Stage 0 starts with no external Rust crate dependencies.

Allowed dependencies are only workspace path dependencies between the initial Capi crates:

```text
capi-cli -> capi-driver
capi-driver -> capi-common
capi-driver -> capi-diagnostics
capi-driver -> capi-session
capi-session -> capi-diagnostics
capi-session -> capi-source
capi-source -> capi-common
capi-diagnostics -> capi-common
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

The Stage 0 dependency check is:

```bash
scripts/deps.sh
```

It validates the Cargo metadata and rejects external dependency sources in `Cargo.lock`.

If an external dependency is approved in a future stage, this file and `scripts/deps.sh` must be updated in the same change.
