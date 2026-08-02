# Capi

**Capi** is an object-oriented programming language designed around explicit ownership of memory regions, stable object identity, deterministic resource management, and safe concurrency.

The language does not try to adapt Rust's ownership model to traditional object-oriented programming. Instead, Capi defines its own object model:

- objects represent logical identity and behavior;
- mutable physical storage belongs to larger units called **Domains**;
- object references are represented through stable **ObjectId** values;
- mutation is controlled through the type system, capabilities, and domain rules;
- memory safety should be enforced statically whenever possible, without requiring a mandatory garbage collector or reference counting model.

## Repository Status

Capi is currently in **Stage 1 - Source Infrastructure, Diagnostics, and Lexer** for the official implementation.

The repository contains the language specification, implementation specification, architecture decisions, engineering documentation, and the initial Rust workspace for the official implementation.

There is a `capic` executable in `capi-lang/`. It supports `--help`, `--version`, argument validation, source-file loading errors, session initialization, basic internal-error handling, and the Stage 1 token dump command:

```bash
capic --emit tokens arquivo.capi
```

It does not compile Capi programs yet.

The documentation package has been consolidated with top-level indexes, a changelog, approved Stage 0 and Stage 1 engineering documents, and the formal Stage 0 progress record.

## Current Phase

**Stage 0 - Project Foundation** has been completed locally according to the Stage 0 validation criteria.

Stage 0 created:

- the initial Cargo workspace in `capi-lang/`;
- the first compiler crates;
- the minimal `capic` executable;
- build, formatting, lint, test, and CI infrastructure;
- the initial engineering rules that guide development.

The required Stage 0 engineering documents and ADRs have been approved in the documentation set.

**Stage 1 - Source Infrastructure, Diagnostics, and Lexer** has delivered:

- source management with `SourceId`, `SourceFile`, `SourceMap`, `Span`, line and column lookup;
- structured diagnostics with codes, labels, notes, suggestions, and rendering;
- the initial token model;
- the initial lexer for identifiers, keywords, literals, operators, delimiters, comments, EOF, and lexical errors;
- token dump support through `capic --emit tokens`;
- lexer fixtures, snapshots, compile-fail lexical tests, diagnostic-position tests, and malformed-input robustness tests.

The required Stage 1 engineering documents have been approved in the documentation set.

The formal progress record is:

- [`FEATURE-STATUS.md`](capi-docs/docs/engineering/planning/FEATURE-STATUS.md)

The documentation changelog is:

- [`capi-docs/CHANGELOG.md`](capi-docs/CHANGELOG.md)

## Repository Layout

```text
capi/
├── capi-docs/
│   └── project documentation, specifications, ADRs, RFCs, templates, and engineering docs
│
├── .github/
│   └── CI workflows
│
└── capi-lang/
    └── official implementation workspace
```

## Documentation Map

The main documentation lives under [`capi-docs/docs`](capi-docs/docs/).

Important entry points:

- [`Capi Docs`](capi-docs/README.md) - documentation package overview.
- [`Documentation root`](capi-docs/docs/README.md) - map of the documentation tree.
- [`Specification`](capi-docs/docs/specification/README.md) - language and implementation specification.
- [`Language specification`](capi-docs/docs/specification/language/) - documents `00` to `12`.
- [`Implementation specification`](capi-docs/docs/specification/implementation/) - documents `13` to `28`.
- [`Engineering documentation`](capi-docs/docs/engineering/) - architecture, build, testing, development, planning, runtime, toolchain, release, and security docs.
- [`ADRs`](capi-docs/docs/adr/) - architecture decision records.
- [`RFCs`](capi-docs/docs/rfc/) - future language and project change proposals.
- [`Governance`](capi-docs/docs/governance/) - project decision process and roles.

The latest implementation-planning document is:

- [`28 - Plano de Desenvolvimento da Implementacao Oficial`](capi-docs/docs/specification/implementation/28%20%E2%80%94%20Plano%20de%20Desenvolvimento%20da%20Implementa%C3%A7%C3%A3o%20Oficial.md)

The currently active engineering indexes are:

- [`Architecture`](capi-docs/docs/engineering/architecture/README.md)
- [`Build and CI`](capi-docs/docs/engineering/build-and-ci/README.md)
- [`Compiler`](capi-docs/docs/engineering/compiler/README.md)
- [`Development`](capi-docs/docs/engineering/development/README.md)
- [`Testing`](capi-docs/docs/engineering/testing/README.md)
- [`Planning`](capi-docs/docs/engineering/planning/README.md)

## Development Model

The official implementation is planned as a Rust-based compiler and toolchain, initially built with a Cargo workspace.

The current workspace crates are:

- `capi-cli` - `capic` executable and argument parsing;
- `capi-driver` - compiler driver orchestration and token dump output;
- `capi-common` - shared foundational types and constants;
- `capi-session` - compilation session setup;
- `capi-source` - source-file loading, source maps, spans, line and column lookup;
- `capi-diagnostics` - structured diagnostic infrastructure;
- `capi-lexer` - token model and initial lexer.

The workspace currently has no external Rust crate dependencies. The dependency policy is recorded in [`capi-lang/DEPENDENCIES.md`](capi-lang/DEPENDENCIES.md).

The Rust toolchain policy is recorded in [`capi-lang/TOOLCHAIN.md`](capi-lang/TOOLCHAIN.md). The workspace pins Rust `1.88.0`.

The planned compiler architecture separates:

- source management;
- lexer;
- parser and AST;
- HIR;
- name resolution;
- type checking;
- ownership, region, and domain analysis;
- MIR;
- backend-independent optimizations;
- code generation;
- runtime;
- standard library;
- CLI and tooling.

Cranelift is planned as the initial backend. LLVM is planned as a later optimization and compatibility backend.

## Language and Documentation Language

The official project documentation is written in Brazilian Portuguese (PT-BR).

This is intentional. The specification, engineering documentation, ADRs, and planning documents will remain in PT-BR during language design and implementation.

English documentation such as user manuals, guides, tutorials, and public-facing learning material may be produced after the language and toolchain are mature enough for users.

## What Is Ready

At this point, the repository has:

- a complete language specification set from documents `00` to `12`;
- an implementation specification set from documents `13` to `28`;
- approved Stage 0 ADRs for core implementation decisions;
- approved Stage 0 and Stage 1 engineering documents;
- documentation indexes for `capi-docs`, `docs`, ADRs, engineering, compiler, architecture, build/CI, development, testing, and planning;
- a `capi-docs` changelog;
- a Rust Cargo workspace in `capi-lang`;
- the fundamental compiler crates plus the initial source, diagnostics, and lexer crates;
- a `capic` executable with Stage 1 token dump support;
- local validation scripts and CI workflow configuration;
- lexer fixtures and snapshot tests.

The current local validation set includes:

```bash
cd capi-lang
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked
cargo test --workspace --locked
cargo run -p capi-cli --locked -- --help
cargo run -p capi-cli --locked -- --version
cargo run -p capi-cli --locked -- --emit tokens tests/lexer/pass/basic.cap
scripts/ci-local.sh
```

The CI workflow is versioned at [`.github/workflows/capi-lang-ci.yml`](.github/workflows/capi-lang-ci.yml). Remote CI execution depends on a `push` or `pull_request` event, while the equivalent local validation has passed for Stage 1.

## What Is Not Ready Yet

The repository does not yet provide:

- a compiler capable of compiling Capi programs;
- a package manager;
- a standard library implementation;
- a runtime implementation;
- an installable toolchain;
- compatibility guarantees for user programs.

## Near-Term Roadmap

The immediate next step is to continue with the next frontend work defined by Document 28, building on the Stage 1 source, diagnostics, and lexer foundation.

## Project Note

Capi is still early. The repository is intentionally documentation-first so the implementation can be traceable to the language specification, architecture decisions, and engineering constraints from the beginning.
