# Rust Style Guide

**Projeto:** Linguagem Capi  
**Documento:** RUST-STYLE-GUIDE  
**Status:** Aprovado  
**Stage:** Stage 0 — Fundação do projeto  
**Natureza:** Documento de consolidação  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o estilo Rust usado na implementação oficial da Linguagem Capi.

Seu objetivo é complementar `CODING-STANDARDS.md` com regras específicas para código Rust no workspace `capi-lang`.

Este documento não redefine arquitetura, dependências, política de `unsafe` ou semântica da linguagem.

---

## 2. Escopo

Este documento cobre:

- edição Rust;
- formatação;
- organização de módulos;
- nomes;
- visibilidade;
- APIs;
- erros;
- `Option` e `Result`;
- traits e generics;
- derives;
- macros;
- testes;
- features Cargo;
- Clippy;
- documentação Rust;
- uso de `unsafe`.

Este documento não cobre:

- algoritmos internos do compilador;
- estrutura completa de AST, HIR ou MIR;
- política completa de dependências;
- política completa de segurança;
- comandos públicos da toolchain Capi.

---

## 3. Edição Rust e MSRV

O workspace deve declarar uma edição Rust explícita.

Configuração conceitual inicial:

```toml
[workspace.package]
edition = "2021"
```

A MSRV exata deve ser definida em documento próprio ou em política aprovada do workspace.

Enquanto a MSRV não estiver formalizada:

- não adotar recursos recém-estabilizados sem necessidade clara;
- preferir compatibilidade com Rust estável;
- evitar dependências que forcem atualização frequente de toolchain;
- respeitar `rust-toolchain.toml` quando existir.

---

## 4. Formatação

Formatação deve ser feita por `rustfmt`.

Comando obrigatório:

```bash
cargo fmt --all --check
```

Correção local:

```bash
cargo fmt --all
```

Regras:

- não alinhar manualmente código contra o `rustfmt`;
- não discutir estilo que o `rustfmt` já decide;
- manter `rustfmt.toml` simples no Stage 0;
- alterações de formatação não devem ser misturadas com refactors grandes quando isso dificultar revisão.

---

## 5. Clippy

Clippy deve ser executado no workspace.

Comando obrigatório:

```bash
cargo clippy --workspace --all-targets
```

Regras:

- avisos relevantes devem ser corrigidos;
- `#[allow(...)]` deve ser local e justificado;
- não usar `allow` em crate inteira sem aprovação;
- não silenciar lint que revele erro de modelagem;
- preferir código claro a contorções para satisfazer lint menor.

Exemplo aceitável:

```rust
#[allow(clippy::too_many_arguments)]
fn build_diagnostic(/* campos estruturais obrigatórios */) {
    // Justificativa: builder será introduzido quando o modelo de diagnóstico estabilizar.
}
```

---

## 6. Organização de Crates

Crates devem refletir responsabilidades arquiteturais.

Regras:

- crate binário deve ser fino;
- lógica reutilizável deve ficar em crate de biblioteca;
- `capi-cli` não deve conter lógica de compilação;
- `capi-common` não deve virar depósito genérico;
- crates de frontend não devem depender de backend;
- crates de backend não devem vazar para fases anteriores;
- dependências devem seguir `DEPENDENCY-RULES.md`.

Cada crate deve ter um `lib.rs` ou `main.rs` simples e orientado a módulos claros.

---

## 7. Organização de Módulos

Módulos devem ser pequenos e ter responsabilidade local.

Preferir:

```text
src/
├── lib.rs
├── error.rs
├── span.rs
├── source_file.rs
└── source_map.rs
```

Evitar:

```text
src/
├── lib.rs
├── misc.rs
├── helpers.rs
└── everything.rs
```

Regras:

- nomes de módulos devem indicar domínio;
- módulos internos devem esconder detalhes privados;
- reexports devem formar API pública intencional;
- arquivos grandes devem ser divididos quando acumularem responsabilidades diferentes.

---

## 8. Visibilidade

Use a menor visibilidade suficiente.

Preferir:

```rust
pub(crate) struct SessionState {
    // ...
}
```

em vez de:

```rust
pub struct SessionState {
    // ...
}
```

Regras:

- `pub` significa contrato entre crates;
- `pub(crate)` significa contrato interno da crate;
- campos públicos devem ser raros;
- construtores devem preservar invariantes;
- não expor tipo apenas para facilitar teste.

Quando testes precisarem observar estado interno, prefira API de inspeção controlada ou teste no módulo correspondente.

---

## 9. Nomes Rust

Siga convenções Rust:

- tipos e traits em `PascalCase`;
- funções, métodos, módulos e variáveis em `snake_case`;
- constantes em `SCREAMING_SNAKE_CASE`;
- crates em `kebab-case` no Cargo e `snake_case` no código importado.

Preserve siglas oficiais:

```rust
AstNode
HirId
MirBlock
AbiInfo
FfiBoundary
ObjectId
DomainId
```

Evite nomes vagos:

```rust
Data
Info
Manager
Helper
Util
Thing
```

Quando um nome genérico parecer necessário, qualifique:

```rust
DiagnosticBuilder
SourceMap
SessionOptions
BackendTarget
```

---

## 10. Tipos Novos

Use tipos novos para conceitos distintos.

Preferir:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SourceId(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpanId(u32);
```

Evitar:

```rust
type SourceId = usize;
type SpanId = usize;
```

Tipos novos ajudam a impedir troca acidental de identificadores.

---

## 11. Structs

Structs devem representar entidades com responsabilidade clara.

Regras:

- prefira campos privados;
- use construtores para validar invariantes;
- evite structs com muitos campos opcionais sem motivo;
- considere builder quando a construção exigir muitos parâmetros;
- derive traits apenas quando o contrato fizer sentido.

Exemplo:

```rust
pub struct SourceFile {
    id: SourceId,
    path: SourcePath,
    text: SourceText,
}
```

---

## 12. Enums

Use enums para estados fechados.

Exemplo:

```rust
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Note,
}
```

Regras:

- enums devem representar alternativas reais;
- variantes devem ter nomes específicos;
- evite variante `Other` sem dados estruturados;
- evite strings para representar estados fechados.

---

## 13. Traits

Traits devem representar contrato real, não apenas conveniência.

Use traits quando:

- houver múltiplas implementações reais;
- a fronteira arquitetural exigir abstração;
- testes precisarem substituir dependência externa;
- backend interface precisar isolar implementação concreta.

Evite traits quando:

- só existe uma implementação e nenhuma variação prevista;
- a trait apenas repete métodos de uma struct;
- a abstração esconde dependência importante.

---

## 14. Generics

Generics devem ser usados quando removem duplicação real ou expressam contrato de tipo.

Regras:

- prefira tipos concretos em APIs de fase quando isso aumentar clareza;
- evite generics profundos em fronteiras centrais sem necessidade;
- coloque bounds próximos da função quando forem locais;
- use `where` quando os bounds ficarem longos.

Exemplo:

```rust
fn render<W>(writer: &mut W, diagnostic: &Diagnostic) -> std::io::Result<()>
where
    W: std::io::Write,
{
    // ...
}
```

---

## 15. `Option`

Use `Option` para ausência legítima de valor.

Regras:

- não use `Option` para esconder erro;
- documente quando `None` tiver significado não óbvio;
- prefira métodos explícitos a cadeias difíceis de depurar;
- evite `unwrap()` em código de produção.

Aceitável em teste:

```rust
let span = source_map.lookup(id).expect("span should exist in test fixture");
```

---

## 16. `Result`

Use `Result` para operações que podem falhar.

Regras:

- preserve a causa do erro;
- diferencie erro do usuário, erro interno e erro de ambiente;
- não converta erro estruturado em string cedo demais;
- use `?` quando o contexto permanecer claro;
- adicione contexto quando a origem puder ficar ambígua.

Evite:

```rust
return Err("failed".into());
```

Prefira tipos de erro estruturados.

---

## 17. `panic`, `unwrap` e `expect`

`panic` não deve ser fluxo normal de erro.

`unwrap()` é proibido em código de produção quando a falha puder ocorrer por entrada do usuário, ambiente ou estado externo.

`expect()` pode ser usado quando:

- a invariante é realmente interna;
- a mensagem explica a invariante;
- a falha indica bug do compilador.

Exemplo aceitável:

```rust
let current = stack.last().expect("scope stack must contain root scope");
```

Em testes, `expect()` é permitido para tornar falhas legíveis.

---

## 18. Derives

Derive apenas traits que fazem sentido para o tipo.

Comuns:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
```

Regras:

- `Copy` deve ser reservado para tipos pequenos e sem semântica de posse complexa;
- `Debug` é recomendado para tipos internos;
- `PartialOrd` e `Ord` exigem ordem semântica real ou uso determinístico documentado;
- derives de serialização só devem ser adicionados quando houver dependência aprovada.

---

## 19. Macros

Macros devem ser usadas com parcimônia.

Use macros quando:

- reduzirem repetição estrutural real;
- preservarem mensagens de erro compreensíveis;
- forem testadas;
- tiverem escopo limitado.

Evite macros para:

- esconder lógica de compilador;
- criar DSL interna prematura;
- contornar sistema de tipos;
- reduzir poucas linhas de código comum.

---

## 20. Lifetimes

Lifetimes devem expressar relação real entre referências.

Regras:

- prefira ownership quando simplificar o modelo;
- evite lifetimes nomeados quando elision for clara;
- não exponha lifetimes complexos em API pública sem necessidade;
- use arenas ou interning apenas quando a arquitetura justificar.

Complexidade de lifetime em API pública é custo de manutenção.

---

## 21. Collections

Escolha coleções pelo contrato necessário.

Regras:

- use `Vec` quando ordem for relevante;
- use mapas apenas quando busca por chave for necessária;
- ordene explicitamente antes de produzir saída observável;
- evite depender da ordem de iteração de mapas hash;
- considere índices tipados em vez de referências longas para grafos internos.

Dumps, snapshots e diagnósticos devem ser determinísticos.

---

## 22. Strings e Texto-fonte

Texto-fonte deve ser tratado com cuidado.

Regras:

- não confundir byte offset com coluna visual;
- spans devem usar modelo definido por `capi-source`;
- strings de diagnóstico devem passar pela infraestrutura de diagnósticos;
- caminhos devem ser normalizados em testes;
- não use `String` solta para representar símbolo quando houver interning.

---

## 23. Caminhos e Filesystem

Operações de filesystem devem preservar contexto.

Regras:

- erros de IO devem carregar path afetado;
- testes devem usar diretórios temporários;
- paths absolutos não devem aparecer em snapshots sem normalização;
- código interno deve distinguir path de entrada, path canônico e path apresentado.

---

## 24. Módulos de Teste

Testes unitários podem ficar no mesmo arquivo:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_missing_input_file() {
        // ...
    }
}
```

Regras:

- nomes de teste devem descrever comportamento;
- fixtures devem ser mínimas;
- `expect()` em testes deve explicar a falha;
- testes não devem depender de ordem global;
- testes de integração entre crates devem viver em `tests/`.

---

## 25. Features Cargo

Features devem ser poucas e explícitas.

Regras:

- não criar feature sem caso de uso aprovado;
- evitar default features amplas;
- features de backend devem permanecer isoladas;
- features não devem alterar semântica da linguagem;
- combinações relevantes devem ser testadas quando existirem.

Features são parte do contrato de build.

---

## 26. Reexports

Reexports devem formar API intencional.

Exemplo:

```rust
pub use crate::span::{Span, SpanId};
pub use crate::source_file::{SourceFile, SourceId};
```

Evite:

```rust
pub use crate::internal::*;
```

Reexports amplos dificultam controle de API e dependências.

---

## 27. Imports

Imports devem ser claros e locais.

Regras:

- evite imports globais desnecessários;
- prefira importar tipos usados repetidamente;
- use caminho qualificado quando isso aumentar clareza;
- não use glob imports fora de testes salvo justificativa local.

Em testes, `use super::*;` é aceitável.

---

## 28. Documentação Rust

Itens públicos entre crates devem ter documentação.

Comentários de documentação devem explicar:

- responsabilidade;
- invariantes;
- erros possíveis;
- relação com spans, diagnósticos ou sessão;
- limitações temporárias.

Exemplo:

```rust
/// Identifies a source file loaded in the current compilation session.
///
/// A `SourceId` is stable only within the session that created it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SourceId(u32);
```

---

## 29. Código Assíncrono

Código assíncrono não deve ser introduzido no compilador sem necessidade clara.

No Stage 0, o pipeline do compilador deve permanecer síncrono.

Uso futuro de async pode ser considerado para:

- LSP;
- servidor de toolchain;
- tarefas de IO concorrente;
- integração com registry.

Mesmo nesses casos, async deve ficar isolado da lógica central do compilador.

---

## 30. Concorrência

Concorrência interna deve ser introduzida apenas quando houver benefício claro.

Regras:

- preservar determinismo de saída;
- evitar estado compartilhado mutável;
- isolar paralelismo em camadas apropriadas;
- não paralelizar antes de estabilizar corretude;
- testar comportamento com ordem determinística.

---

## 31. `unsafe`

Este documento não define a política completa de `unsafe`.

Regra de estilo:

- mantenha blocos `unsafe` pequenos;
- coloque comentário de segurança próximo ao bloco;
- não esconda `unsafe` em abstrações com nome inocente;
- escreva API segura ao redor apenas quando as invariantes forem preservadas.

Formato recomendado:

```rust
// SAFETY: explanation of the invariant maintained here.
let value = unsafe { operation() };
```

---

## 32. Exemplos de Estilo

Preferir:

```rust
pub struct Diagnostic {
    severity: DiagnosticSeverity,
    message: DiagnosticMessage,
    primary_span: Option<Span>,
}

impl Diagnostic {
    pub fn new(severity: DiagnosticSeverity, message: DiagnosticMessage) -> Self {
        Self {
            severity,
            message,
            primary_span: None,
        }
    }
}
```

Evitar:

```rust
pub struct Diagnostic {
    pub kind: String,
    pub msg: String,
    pub pos: usize,
}
```

O primeiro exemplo modela contratos. O segundo mistura estado livre, strings soltas e posição sem tipo.

---

## 33. Comandos Obrigatórios

Quando o workspace existir, valide estilo Rust com:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets
cargo test --workspace
cargo build --workspace
```

Quando `capic` existir:

```bash
cargo run -p capi-cli -- --help
cargo run -p capi-cli -- --version
```

---

## 34. Critérios de Aceitação do Documento

Este documento é considerado preenchido quando:

- define estilo Rust para o workspace;
- cobre formatação, Clippy, módulos, visibilidade, tipos, erros, testes e documentação;
- preserva os padrões gerais de `CODING-STANDARDS.md`;
- não contradiz `BUILD-SYSTEM.md`, `DEPENDENCY-RULES.md` ou `UNSAFE-POLICY.md`;
- deixa claro que a validação Cargo só se aplica após a criação do workspace.

---

## 35. Síntese

Rust deve ser usado na Capi como ferramenta de implementação rigorosa, não como fonte implícita da semântica da linguagem.

O estilo do código deve favorecer tipos explícitos, APIs pequenas, erros estruturados, determinismo e fronteiras claras entre crates.
