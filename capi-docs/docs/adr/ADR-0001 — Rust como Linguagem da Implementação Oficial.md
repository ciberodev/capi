# ADR-0001 — Rust como Linguagem da Implementação Oficial

**Status:** Aprovado  
**Data:** 2026-07-30  
**Stage:** Stage 0 — Fundação do projeto  
**Decisão:** Usar Rust como linguagem hospedeira inicial da implementação oficial da Capi.

---

## Contexto

A Capi possui uma especificação própria de linguagem, sistema de tipos, modelo de memória, runtime, ABI, IR, toolchain e processo de bootstrap.

A implementação oficial precisa começar em uma linguagem hospedeira já existente, capaz de:

- construir um compilador nativo;
- modelar estruturas internas complexas;
- preservar segurança de memória na implementação;
- permitir evolução incremental;
- integrar backends como Cranelift e, posteriormente, LLVM;
- oferecer build, testes, lint, formatação e documentação desde o Stage 0;
- servir como compilador de bootstrap antes da auto-hospedagem em Capi.

O Documento 27 define que a implementação inicial será desenvolvida em Rust e que essa escolha pertence à implementação oficial, não à definição da linguagem.

O Documento 28 exige, no Stage 0, ambiente Rust definido, workspace Cargo, crates fundamentais, build, testes, formatação, Clippy e executável mínimo `capic`.

---

## Decisão

A implementação oficial inicial da linguagem Capi será escrita em Rust.

Rust será usado para implementar o compilador de bootstrap, incluindo inicialmente:

- CLI `capic`;
- driver;
- sessão de compilação;
- infraestrutura de fontes;
- diagnósticos;
- lexer;
- parser;
- AST;
- HIR;
- resolução de nomes;
- checagem de tipos;
- verificações semânticas;
- MIR;
- integração com backend Cranelift;
- runtime inicial;
- ferramentas auxiliares necessárias ao bootstrap.

Rust será tratado exclusivamente como mecanismo da implementação oficial.

Rust não define:

- a sintaxe da Capi;
- a semântica da Capi;
- o modelo de memória;
- `Domain`;
- `ObjectId`;
- a ABI pública;
- a IR oficial;
- o sistema de tipos;
- a futura implementação auto-hospedada.

As regras da Capi devem ser modeladas explicitamente pelo compilador, mesmo quando Rust já impediria determinada classe de erro internamente.

---

## Justificativa

Rust é adequado para o Stage 0 porque oferece:

- segurança de memória sem garbage collector obrigatório;
- controle explícito sobre layout e alocação;
- sistema de tipos forte para representar invariantes internas;
- ecossistema maduro para compiladores;
- integração viável com Cranelift;
- possibilidade de integração posterior com LLVM;
- Cargo para workspace, build, testes, lockfile e documentação;
- `rustfmt` e Clippy para padronização e análise;
- bom equilíbrio entre baixo nível e segurança.

Essa escolha reduz o risco inicial de implementação sem transferir a semântica de Rust para a Capi.

---

## Alternativas Consideradas

### Implementar diretamente em Capi

Rejeitada para o Stage 0 porque ainda não existe compilador Capi capaz de compilar a própria implementação.

Será o objetivo posterior do processo de bootstrap e auto-hospedagem.

### C ou C++

Rejeitadas como primeira opção por aumentarem o risco de erros de memória na implementação do compilador e do runtime inicial.

### Go, Zig, OCaml ou outras linguagens

Consideradas viáveis em tese, mas menos alinhadas ao plano já definido nos Documentos 27 e 28, especialmente quanto ao uso inicial de Cargo, Cranelift e ferramentas Rust.

---

## Consequências Positivas

- O projeto pode iniciar com uma base segura e testável.
- O Stage 0 pode usar Cargo como sistema de build inicial.
- Crates podem refletir fronteiras arquiteturais do compilador.
- A implementação ganha suporte imediato a testes, lint e formatação.
- A integração com Cranelift fica tecnicamente simples.
- A implementação em Rust pode servir como referência durante a migração para Capi.

---

## Consequências Negativas

- A implementação oficial passa a depender temporariamente da toolchain Rust.
- A equipe precisa controlar MSRV, dependências Cargo e compatibilidade do ecossistema Rust.
- Existe risco de confundir garantias da Capi com mecanismos de Rust.
- A remoção da dependência operacional de Rust exigirá processo posterior de bootstrap e testes diferenciais.

---

## Restrições

- Nenhuma regra da linguagem pode ser justificada apenas por limitação ou capacidade de Rust.
- `unsafe` deve ser minimizado, documentado, revisado e testado.
- Dependências Rust devem seguir a política de dependências externas.
- A implementação em Rust deve permanecer substituível progressivamente por Capi.
- A arquitetura do compilador deve permanecer preservada durante a migração.

---

## Critérios de Validação

Esta decisão será considerada operacional no Stage 0 quando:

- o ambiente Rust estiver definido;
- o workspace Cargo existir em `capi-lang`;
- os crates fundamentais compilarem;
- `cargo fmt`, `cargo clippy`, `cargo test` e `cargo build` funcionarem;
- `capic --help` e `capic --version` funcionarem;
- a documentação deixar claro que Rust é mecanismo de implementação, não definição da linguagem.

## Implementação no Stage 0

A decisão foi materializada no workspace `capi-lang`.

A MSRV inicial foi definida como Rust `1.88.0`.

Arquivos operacionais relacionados:

- `capi-lang/rust-toolchain.toml`;
- `capi-lang/TOOLCHAIN.md`;
- `capi-lang/clippy.toml`;
- `capi-lang/rustfmt.toml`;
- `capi-lang/scripts/tools.sh`.

A validação local confirma `rustc 1.88.0`, `cargo 1.88.0`, `rustfmt 1.8.0-stable` e `clippy 0.1.88`.

---

## Referências

- Documento 27 — Bootstrap Plan e Arquitetura da Implementação Oficial
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial
- `ENGINEERING-PRINCIPLES.md`
- `DEVELOPMENT-SETUP.md`
- `BUILD-SYSTEM.md`
- `RUST-STYLE-GUIDE.md`
