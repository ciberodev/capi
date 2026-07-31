# Capi Language Implementation

This directory contains the official implementation of the Capi language.

The implementation is currently in Stage 0: project foundation.

The Cargo workspace and the fundamental compiler crates have been created.

## Workspace

The workspace root is this directory:

```bash
cd capi-lang
```

Standard validation commands:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets
cargo test --workspace
cargo build --workspace
cargo doc --workspace --no-deps
```

Dependency policy is recorded in:

```text
DEPENDENCIES.md
```

Minimum tool versions are recorded in:

```text
TOOLCHAIN.md
```

The same validation sequence, including the Stage 0 `capic` smoke checks, can be run with:

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
