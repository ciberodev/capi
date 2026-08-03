# Toolchain Policy

This file records the minimum tool versions for the official Capi implementation.

The current policy remains intentionally narrow: the Rust workspace must build,
test, lint, format, and generate Rust API documentation with the pinned Rust
toolchain. Stages 1 and 2 did not change the MSRV or pinned tool versions.

## Minimum Versions

| Tool | Minimum version | Required |
| --- | --- | --- |
| `rustc` | `1.88.0` | yes |
| `cargo` | `1.88.0` | yes |
| `rustfmt` | `1.8.0-stable` | yes |
| `clippy` | `0.1.88` | yes |

The active toolchain is pinned in:

```text
capi-lang/rust-toolchain.toml
```

The workspace MSRV is also declared through `workspace.package.rust-version`.

## Updating Versions

Changing the MSRV or the pinned toolchain requires:

- updating `rust-toolchain.toml`;
- updating `Cargo.toml`;
- updating `clippy.toml`;
- updating this file;
- running `scripts/tools.sh`;
- running `scripts/check.sh`;
- recording the reason in the relevant engineering document or ADR when the change is architectural.

Dependencies added in future stages must remain compatible with the declared MSRV unless a toolchain update is approved in the same change.
