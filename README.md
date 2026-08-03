# Capi

**Capi** is an object-oriented programming language designed around explicit ownership of memory regions, stable object identity, deterministic resource management, and safe concurrency.

The language does not adapt Rust's ownership model to traditional object-oriented programming. Instead, Capi defines its own object model:

- objects represent logical identity and behavior;
- mutable physical storage belongs to larger units called **Domains**;
- object references are represented through stable **ObjectId** values;
- mutation is controlled through the type system, capabilities, and domain rules;
- memory safety should be enforced statically whenever possible, without requiring a mandatory garbage collector or reference counting model.

## Repository Status

Capi is currently past **Stage 2 - Parser and AST** for the official implementation.

The repository contains the language specification, implementation specification, architecture decisions, engineering documentation, planning records, and the Rust workspace for the official implementation.

There is a `capic` executable in `capi-lang/`. It currently supports:

- `--help`;
- `--version`;
- argument validation;
- source-file loading errors;
- session initialization;
- basic internal-error handling;
- Stage 1 token dumps;
- Stage 2 AST dumps.

Current demonstrable commands:

```bash
capic --emit tokens arquivo.capi
capic --emit ast arquivo.capi
```

It does not compile Capi programs yet.

The next planned stage is:

```text
Stage 3 - HIR and name resolution
```

## Current Phase

**Stage 0 - Project Foundation** is complete.

Stage 0 delivered:

- the initial Cargo workspace in `capi-lang/`;
- the first compiler crates;
- the minimal `capic` executable;
- build, formatting, lint, test, documentation, and CI infrastructure;
- the initial engineering rules that guide development.

**Stage 1 - Source Infrastructure, Diagnostics, and Lexer** is complete.

Stage 1 delivered:

- source management with `SourceId`, `SourceFile`, `SourceMap`, `Span`, line and column lookup;
- structured diagnostics with codes, labels, notes, suggestions, and rendering;
- the initial token model;
- the initial lexer for identifiers, keywords, literals, operators, delimiters, comments, EOF, and lexical errors;
- token dump support through `capic --emit tokens`;
- lexer fixtures, snapshots, compile-fail lexical tests, diagnostic-position tests, and malformed-input robustness tests.

**Stage 2 - Parser and AST** is complete.

Stage 2 delivered:

- the `capi-ast` crate;
- the `capi-parser` crate;
- AST nodes for compilation units, modules, imports, declarations, classes, functions, types, statements, expressions, patterns, and error nodes;
- span preservation for relevant AST nodes;
- parser support for the initial syntactic subset;
- operator precedence and associativity;
- structured syntax diagnostics with `PARSE` codes;
- recovery after recoverable syntax errors;
- partial ASTs with explicit error nodes;
- deterministic AST dumps;
- golden snapshot tests for AST dumps;
- AST dump support through `capic --emit ast`.

The formal progress record is:

- [`FEATURE-STATUS.md`](capi-docs/docs/engineering/planning/FEATURE-STATUS.md)

The implementation order, milestones, roadmap, risks, and technical debt records are:

- [`IMPLEMENTATION-ORDER.md`](capi-docs/docs/engineering/planning/IMPLEMENTATION-ORDER.md)
- [`MILESTONES.md`](capi-docs/docs/engineering/planning/MILESTONES.md)
- [`ROADMAP.md`](capi-docs/docs/engineering/planning/ROADMAP.md)
- [`RISK-REGISTER.md`](capi-docs/docs/engineering/planning/RISK-REGISTER.md)
- [`TECHNICAL-DEBT.md`](capi-docs/docs/engineering/planning/TECHNICAL-DEBT.md)

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
- [`Engineering documentation`](capi-docs/docs/engineering/) - architecture, build, testing, development, planning, and compiler docs.
- [`Compiler engineering`](capi-docs/docs/engineering/compiler/README.md) - compiler source, diagnostics, lexer, parser, AST, and future phases.
- [`Testing engineering`](capi-docs/docs/engineering/testing/README.md) - testing strategy, lexer tests, and parser tests.
- [`Planning`](capi-docs/docs/engineering/planning/README.md) - definition of done, feature status, implementation order, milestones, roadmap, risks, and technical debt.
- [`ADRs`](capi-docs/docs/adr/) - architecture decision records.

The implementation-planning document is:

- [`28 - Plano de Desenvolvimento da Implementacao Oficial`](capi-docs/docs/specification/implementation/28%20%E2%80%94%20Plano%20de%20Desenvolvimento%20da%20Implementa%C3%A7%C3%A3o%20Oficial.md)

## Development Model

The official implementation is a Rust-based compiler and future toolchain, initially built with a Cargo workspace.

The current workspace crates are:

- `capi-cli` - `capic` executable and argument parsing;
- `capi-driver` - compiler driver orchestration and dump outputs;
- `capi-common` - shared foundational types and constants;
- `capi-session` - compilation session setup;
- `capi-source` - source-file loading, source maps, spans, line and column lookup;
- `capi-diagnostics` - structured diagnostic infrastructure;
- `capi-lexer` - token model and initial lexer;
- `capi-ast` - abstract syntax tree model and deterministic AST dump;
- `capi-parser` - parser, syntax diagnostics, recovery, and AST construction.

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
- approved engineering documents for Stages 0, 1, and 2;
- documentation indexes for `capi-docs`, `docs`, ADRs, engineering, compiler, architecture, build/CI, development, testing, and planning;
- planning records for feature status, implementation order, milestones, roadmap, risks, and technical debt;
- a `capi-docs` changelog;
- a Rust Cargo workspace in `capi-lang`;
- the fundamental compiler crates plus source, diagnostics, lexer, AST, and parser crates;
- a `capic` executable with token dump and AST dump support;
- local validation scripts and CI workflow configuration;
- lexer fixtures and snapshot tests;
- parser integration tests and AST dump golden snapshots.

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
cargo run -p capi-cli --bin capic --locked -- --emit ast crates/capi-parser/tests/fixtures/ast_dump/basic.cap
scripts/check.sh
```

The CI workflow is versioned at [`.github/workflows/capi-lang-ci.yml`](.github/workflows/capi-lang-ci.yml). It validates formatting, toolchain versions, dependency policy, Clippy, tests, build, documentation, and `capic` smoke tests for token and AST dumps.

## What Is Not Ready Yet

The repository does not yet provide:

- a compiler capable of compiling Capi programs;
- HIR or name resolution;
- type checking;
- ownership and domain analysis;
- MIR;
- code generation;
- a package manager;
- a standard library implementation;
- a runtime implementation;
- an installable toolchain;
- compatibility guarantees for user programs.

## Near-Term Roadmap

The immediate next step is **Stage 3 - HIR and name resolution**.

Stage 3 should start with the required semantic documents and then implement:

- AST to HIR lowering;
- internal IDs;
- symbol tables;
- scopes;
- modules and imports;
- name resolution;
- duplicate, missing, and ambiguous symbol diagnostics;
- deterministic HIR dump.

Expected demonstrable command:

```bash
capic --emit hir arquivo.capi
```

## Project Note

Capi is still early. The repository is intentionally documentation-first so the implementation can be traceable to the language specification, architecture decisions, and engineering constraints from the beginning.
