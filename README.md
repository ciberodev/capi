# Capi

**Capi** is an object-oriented programming language designed around explicit ownership of memory regions, stable object identity, deterministic resource management, and safe concurrency.

The language does not try to adapt Rust's ownership model to traditional object-oriented programming. Instead, Capi defines its own object model:

- objects represent logical identity and behavior;
- mutable physical storage belongs to larger units called **Domains**;
- object references are represented through stable **ObjectId** values;
- mutation is controlled through the type system, capabilities, and domain rules;
- memory safety should be enforced statically whenever possible, without requiring a mandatory garbage collector or reference counting model.

## Repository Status

Capi is currently in the specification and implementation-planning phase.

The repository contains the language specification, implementation specification, architecture decisions, and the initial engineering documentation required to start the official implementation.

There is not yet a working compiler in this repository. The `capi-lang/` directory is reserved for the official Rust implementation, but the workspace has not been created yet.

## Current Phase

The project is preparing **Stage 0 - Project Foundation**.

Stage 0 is the foundation stage for the official implementation. Its purpose is to create:

- the initial Cargo workspace in `capi-lang/`;
- the first compiler crates;
- the minimal `capic` executable;
- build, formatting, lint, test, and CI infrastructure;
- the initial engineering rules that guide development.

The blocking engineering documents required before creating the workspace and fundamental crates have been drafted:

- engineering principles;
- project structure;
- compiler architecture;
- workspace architecture;
- component responsibilities;
- dependency rules;
- development setup;
- build system;
- test strategy;
- definition of done.

The next work items are the remaining Stage 0 operational and consolidation documents, followed by the required Stage 0 ADRs and then the actual `capi-lang` workspace creation.

## Repository Layout

```text
capi/
├── capi-docs/
│   └── project documentation, specifications, ADRs, RFCs, and engineering docs
│
└── capi-lang/
    └── reserved for the official compiler, runtime, standard library, and toolchain
```

## Documentation Map

The main documentation lives under [`capi-docs/docs`](capi-docs/docs/).

Important entry points:

- [`Specification`](capi-docs/docs/specification/README.md) - language and implementation specification.
- [`Language specification`](capi-docs/docs/specification/language/) - documents `00` to `12`.
- [`Implementation specification`](capi-docs/docs/specification/implementation/) - documents `13` to `28`.
- [`Engineering documentation`](capi-docs/docs/engineering/) - architecture, build, testing, development, planning, runtime, toolchain, release, and security docs.
- [`ADRs`](capi-docs/docs/adr/) - architecture decision records.
- [`RFCs`](capi-docs/docs/rfc/) - future language and project change proposals.
- [`Governance`](capi-docs/docs/governance/) - project decision process and roles.

The latest implementation-planning document is:

- [`28 - Plano de Desenvolvimento da Implementacao Oficial`](capi-docs/docs/specification/implementation/28%20%E2%80%94%20Plano%20de%20Desenvolvimento%20da%20Implementa%C3%A7%C3%A3o%20Oficial.md)

## Development Model

The official implementation is planned as a Rust-based compiler and toolchain, initially built with a Cargo workspace.

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
- initial ADRs for core implementation decisions;
- the Stage 0 blocking engineering documents drafted;
- a documented plan for building the official implementation.

## What Is Not Ready Yet

The repository does not yet provide:

- a usable compiler;
- a package manager;
- a standard library implementation;
- a runtime implementation;
- an installable toolchain;
- compatibility guarantees for user programs.

## Near-Term Roadmap

The immediate plan is:

1. Finish the Stage 0 operational and consolidation documents.
2. Approve the required Stage 0 ADRs.
3. Create the `capi-lang` Cargo workspace.
4. Create the fundamental compiler crates.
5. Implement the minimal `capic --help` and `capic --version` commands.
6. Configure build, formatting, linting, tests, and CI.
7. Validate the Stage 0 Definition of Done.

## Project Note

Capi is still early. The repository is intentionally documentation-first so the implementation can be traceable to the language specification, architecture decisions, and engineering constraints from the beginning.
