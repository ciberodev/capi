# Workspace Architecture

**Projeto:** Linguagem Capi
**Documento:** WORKSPACE-ARCHITECTURE
**Status:** Proposto
**Stage:** Stage 0 — Fundação do projeto
**Natureza:** Documento de engenharia bloqueante
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define a arquitetura inicial do workspace Rust da implementação oficial da Linguagem Capi.

Seu objetivo é orientar a criação de `capi-lang` como workspace Cargo, definindo a organização inicial de crates, regras de dependência, responsabilidades de alto nível e critérios de evolução.

Este documento complementa:

* `PROJECT-STRUCTURE.md`;
* `COMPILER-ARCHITECTURE.md`;
* `COMPONENT-RESPONSIBILITIES.md`;
* `DEPENDENCY-RULES.md`;
* `BUILD-SYSTEM.md`.

Ele não define APIs internas detalhadas nem substitui os documentos específicos de cada componente.

---

## 2. Escopo

Este documento cobre:

* estrutura inicial do workspace Cargo;
* crates fundamentais do Stage 0;
* convenções de nomes;
* separação entre crates binários e bibliotecas;
* dependências permitidas entre crates;
* integração com testes;
* evolução por stages;
* relação com runtime, biblioteca padrão, toolchain e backends.

Este documento não cobre:

* implementação de algoritmos;
* layout interno completo de módulos Rust;
* políticas detalhadas de dependências externas;
* comandos completos da CLI;
* estrutura final após auto-hospedagem;
* organização de pacotes Capi de usuário.

---

## 3. Princípios

O workspace deve seguir os princípios definidos em `ENGINEERING-PRINCIPLES.md`.

Regras centrais:

* o workspace deve refletir fronteiras arquiteturais;
* dependências devem formar um grafo acíclico;
* crates devem ter responsabilidade clara;
* crates de infraestrutura não devem depender de fases avançadas;
* backends devem permanecer isolados;
* crates binários devem ser finos e delegar lógica a crates de biblioteca;
* o workspace deve compilar e testar desde o Stage 0.

Cargo é mecanismo da implementação inicial em Rust.

Cargo não define a linguagem Capi nem a arquitetura permanente da futura implementação auto-hospedada.

---

## 4. Localização do workspace

O workspace Cargo da implementação oficial deve ser criado em:

```text
capi-lang/
```

Estrutura inicial esperada:

```text
capi-lang/
├── Cargo.toml
├── Cargo.lock
├── crates/
├── compiler/
├── runtime/
├── std/
├── toolchain/
├── tests/
├── examples/
├── bootstrap/
├── scripts/
├── tools/
└── docs/
```

O diretório `crates/` contém crates Rust compartilhados ou fundamentais.

Diretórios como `compiler/`, `runtime/`, `std/` e `toolchain/` podem conter crates próprios quando o componente amadurecer.

A estrutura pode evoluir por stage, desde que preserve as responsabilidades definidas por `PROJECT-STRUCTURE.md`.

---

## 5. Manifesto raiz

O arquivo `capi-lang/Cargo.toml` deve declarar o workspace.

Configuração conceitual:

```toml
[workspace]
members = [
  "crates/*",
]
resolver = "2"

[workspace.package]
edition = "2021"
license = "MIT OR Apache-2.0"
repository = "..."

[workspace.dependencies]
# Dependências compartilhadas aprovadas.
```

A edição Rust e a licença devem seguir as decisões aprovadas do projeto.

A versão exata da edição, MSRV e dependências compartilhadas deve ser consolidada em `BUILD-SYSTEM.md`, `DEPENDENCY-RULES.md` e documentos relacionados.

---

## 6. `Cargo.lock`

`Cargo.lock` deve ser versionado em `capi-lang`.

Justificativa:

* o workspace produz executáveis;
* o bootstrap exige reprodutibilidade;
* CI deve testar o mesmo grafo de dependências;
* alterações transitivas devem ser visíveis em revisão.

Atualizações de dependências devem ser revisáveis e rastreáveis.

---

## 7. Crates fundamentais do Stage 0

O Stage 0 deve criar apenas os crates necessários para estabelecer a base executável do compilador.

Crates mínimos:

```text
crates/
├── capi-cli/
├── capi-driver/
├── capi-session/
├── capi-diagnostics/
├── capi-source/
└── capi-common/
```

### 7.1 `capi-cli`

Crate binário responsável pelo executável `capic`.

Responsabilidades iniciais:

* parse mínimo de argumentos;
* `capic --help`;
* `capic --version`;
* inicialização do driver;
* conversão final de resultado em código de saída.

Não deve conter lógica estrutural do compilador.

### 7.2 `capi-driver`

Crate de coordenação do pipeline.

Responsabilidades iniciais:

* criar e executar uma sessão de compilação;
* coordenar fases disponíveis;
* selecionar ação solicitada;
* encaminhar diagnósticos;
* retornar resultado estruturado ao CLI.

No Stage 0, o driver ainda não precisa compilar código Capi.

### 7.3 `capi-session`

Crate da sessão de compilação.

Responsabilidades iniciais:

* representar opções de compilação;
* armazenar configuração da invocação;
* referenciar source map e diagnósticos;
* carregar informações de versão;
* fornecer contexto operacional para o driver.

Não deve se tornar estado global genérico.

### 7.4 `capi-diagnostics`

Crate da infraestrutura inicial de diagnósticos.

Responsabilidades iniciais:

* severidade;
* mensagens estruturadas;
* emissão textual inicial;
* coleta de diagnósticos;
* erros internos básicos.

Diagnósticos específicos de fases posteriores podem ser adicionados nos stages correspondentes.

### 7.5 `capi-source`

Crate de infraestrutura de fontes.

Responsabilidades iniciais:

* representação inicial de arquivo-fonte;
* identificadores de fonte;
* spans básicos;
* carregamento controlado de arquivos;
* erros de arquivo inexistente.

No Stage 1, esse crate será consolidado com `SourceMap`, localização, Unicode e regras detalhadas.

### 7.6 `capi-common`

Crate para tipos pequenos compartilhados que não pertencem claramente a uma fase específica.

Pode conter:

* tipos utilitários;
* identificadores comuns;
* resultados internos;
* helpers sem dependência arquitetural pesada.

Não deve virar depósito genérico.

Quando um tipo crescer em responsabilidade, ele deve migrar para crate próprio.

---

## 8. Grafo inicial de dependências

O grafo inicial deve ser acíclico.

Direção permitida:

```text
capi-cli
    ↓
capi-driver
    ↓
capi-session
    ↓
capi-source
    ↓
capi-common

capi-driver
    ↓
capi-diagnostics
    ↓
capi-common

capi-session
    ↓
capi-diagnostics
    ↓
capi-common
```

Regras:

* `capi-common` não depende dos demais crates do compilador;
* `capi-diagnostics` pode depender de `capi-source` apenas quando spans concretos forem necessários;
* `capi-source` não depende de `capi-driver`;
* `capi-session` não executa fases do pipeline;
* `capi-driver` coordena, mas não implementa fases específicas;
* `capi-cli` não deve ser dependência de nenhum crate de biblioteca.

Se uma dependência parecer necessária no sentido inverso, a responsabilidade provavelmente está no crate errado.

---

## 9. Crates por stages posteriores

Crates de fases posteriores devem ser introduzidos conforme o Documento 28.

Evolução esperada:

```text
Stage 1
capi-lexer

Stage 2
capi-parser
capi-ast

Stage 3
capi-hir
capi-resolve

Stage 4
capi-types
capi-typeck

Stages 5 a 7
capi-object
capi-memory
capi-domain

Stage 8
capi-mir

Stages 9 a 11
capi-runtime
capi-abi
capi-std
capi-backend
capi-backend-cranelift

Stage 17
capi-backend-llvm
```

Essa lista é orientação inicial, não obrigação de criar todos os crates antecipadamente.

Um crate novo deve ser criado quando houver responsabilidade estável o suficiente para justificar fronteira própria.

---

## 10. Organização de crates por domínio

Enquanto o workspace for pequeno, crates podem viver em `capi-lang/crates`.

Quando um domínio crescer, ele pode ser organizado sob diretório próprio:

```text
capi-lang/
├── compiler/
│   └── crates/
├── runtime/
│   └── crates/
├── std/
│   └── crates/
└── toolchain/
    └── crates/
```

Essa migração exige atualização do manifesto raiz e documentação correspondente.

O objetivo é evitar uma lista plana excessiva de crates quando a implementação amadurecer.

---

## 11. Convenções de nomes

Crates Rust devem usar prefixo `capi-`.

Exemplos:

```text
capi-driver
capi-session
capi-diagnostics
capi-source
capi-lexer
capi-parser
```

Nomes devem:

* indicar responsabilidade;
* evitar abreviações obscuras;
* ser estáveis o bastante para revisão;
* não codificar tecnologia temporária fora de crates específicos.

Backends podem usar nomes explícitos:

```text
capi-backend-cranelift
capi-backend-llvm
```

---

## 12. Crates binários

Crates binários devem ser finos.

No Stage 0, o binário principal é:

```text
capi-cli
```

Ele deve produzir o executável:

```text
capic
```

Regra:

* parse de CLI pode estar no crate binário;
* lógica do compilador deve estar em crates de biblioteca;
* código de saída deve refletir resultado estruturado do driver;
* testes de CLI devem validar comportamento observável.

O comando `capi` será introduzido posteriormente pela toolchain.

---

## 13. Dependências externas

Dependências externas devem ser minimizadas no Stage 0.

Permitido inicialmente:

* dependência para parsing de CLI, se aprovada;
* dependência para testes, se aprovada;
* dependência para diagnósticos, se justificada.

Regras:

* toda dependência deve ter finalidade explícita;
* versões devem ser controladas;
* licença deve ser compatível;
* dependências estruturais exigem ADR;
* dependências transitivas devem ser revisáveis.

A política detalhada pertence a `DEPENDENCY-RULES.md`.

---

## 14. Features Cargo

Features Cargo devem ser usadas com parcimônia.

Uso aceitável:

* habilitar backend opcional;
* separar integrações experimentais;
* controlar recursos de desenvolvimento.

Uso proibido:

* esconder divergência semântica;
* alterar comportamento da linguagem;
* criar combinações não testadas;
* substituir versionamento ou configuração explícita.

Cada feature pública do workspace deve ter documentação e teste correspondente quando afetar comportamento.

---

## 15. Testes no workspace

Cada crate deve possuir testes compatíveis com sua responsabilidade.

Organização esperada:

```text
crate/
├── src/
└── tests/
```

Testes integrados e de conformidade vivem em:

```text
capi-lang/tests/
```

Regras:

* testes unitários ficam próximos ao código;
* testes de CLI validam executáveis;
* fixtures compartilhadas ficam em `capi-lang/tests/fixtures`;
* snapshots devem ser determinísticos;
* testes não devem depender de ordem acidental de execução.

---

## 16. Documentação de crates

APIs internas públicas entre crates devem possuir documentação suficiente para uso correto.

Cada crate deve declarar:

* responsabilidade;
* não objetivos;
* principais tipos exportados;
* dependências arquiteturais;
* relação com os documentos de engenharia.

Essa documentação pode começar no `README.md` do crate ou em documentação Rust.

---

## 17. Código `unsafe`

O workspace deve compilar sem `unsafe` no Stage 0, salvo justificativa formal.

Quando `unsafe` for introduzido:

* deve estar localizado;
* deve declarar invariantes;
* deve possuir teste;
* deve ser revisado;
* deve seguir `UNSAFE-POLICY.md`;
* uso significativo deve ser registrado por ADR.

Crates de frontend e infraestrutura básica devem evitar `unsafe`.

---

## 18. Backends no workspace

Backends devem ser crates isolados.

Estrutura esperada:

```text
capi-backend
capi-backend-cranelift
capi-backend-llvm
```

`capi-backend` define a interface abstrata.

`capi-backend-cranelift` implementa Cranelift.

`capi-backend-llvm` implementa LLVM quando introduzido.

Regras:

* crates de frontend não dependem de crates de backend;
* `capi-mir` não depende de Cranelift ou LLVM;
* dependências Cranelift e LLVM não devem aparecer em crates compartilhados;
* divergências entre backends devem ser testáveis.

---

## 19. Runtime e biblioteca padrão no workspace

Runtime e biblioteca padrão devem permanecer separados do compilador.

Estrutura esperada:

```text
capi-runtime
capi-std
```

Durante o bootstrap, podem existir crates auxiliares em Rust para materializar runtime ou biblioteca mínima.

Esses crates devem permanecer identificados como mecanismos de bootstrap.

Eles não devem redefinir semântica da linguagem.

---

## 20. Toolchain no workspace

A toolchain pública será introduzida progressivamente.

Crates esperados:

```text
capi-toolchain
capi-fmt
capi-test-runner
capi-docgen
capi-lsp
```

Esses crates não devem duplicar regras internas do compilador.

Devem orquestrar componentes através de APIs documentadas.

---

## 21. Versionamento interno

Durante os stages iniciais, crates do workspace compartilham versão interna do projeto.

Publicação independente de crates não é objetivo inicial.

Regras:

* versionamento externo deve aguardar política de release;
* mudanças internas podem ser feitas em conjunto dentro do workspace;
* APIs públicas internas devem continuar documentadas;
* alterações incompatíveis entre crates devem ser revisadas como mudança arquitetural quando afetarem múltiplos componentes.

---

## 22. Comandos obrigatórios

O workspace deve suportar comandos básicos desde o Stage 0:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets
cargo test --workspace
cargo build --workspace
```

O executável `capic` deve suportar:

```bash
capic --help
capic --version
```

Scripts podem encapsular esses comandos, mas não devem substituir a capacidade de executá-los diretamente.

---

## 23. Critérios para criação de crate

Um novo crate deve ser criado quando:

* representar responsabilidade arquitetural distinta;
* possuir fronteira clara;
* reduzir acoplamento real;
* permitir teste independente;
* evitar dependência circular;
* estabilizar uma API interna entre componentes.

Um novo crate não deve ser criado apenas por preferência estética.

Se a responsabilidade ainda for instável, ela pode começar como módulo interno até justificar extração.

---

## 24. Critérios para fusão ou remoção de crate

Um crate pode ser fundido ou removido quando:

* sua responsabilidade não for distinta;
* introduzir acoplamento artificial;
* dificultar evolução do pipeline;
* duplicar outro componente;
* existir apenas por decisão temporária já superada.

Mudanças desse tipo devem atualizar:

* manifesto do workspace;
* documentação;
* testes;
* dependências afetadas;
* ADR, se a decisão for arquitetural.

---

## 25. Relação com o Stage 0

Para concluir o Stage 0, o workspace deve possuir:

* `Cargo.toml` raiz;
* `Cargo.lock` versionado;
* crates fundamentais criados;
* executável `capic`;
* build do workspace funcionando;
* `cargo fmt` configurado;
* `cargo clippy` configurado;
* testes iniciais executáveis;
* ausência de dependências circulares;
* política inicial de dependências respeitada.

Resultado demonstrável:

```bash
cargo test --workspace
capic --help
capic --version
```

---

## 26. Critérios de aceite deste documento

Este documento estará pronto para orientar o Stage 0 quando:

* definir a localização do workspace Cargo;
* definir os crates fundamentais;
* estabelecer o grafo inicial de dependências;
* separar crates binários de bibliotecas;
* preservar fronteiras entre compilador, runtime, std, toolchain e backends;
* não contradizer os Documentos 00 a 28;
* permitir a criação incremental do workspace em `capi-lang`;
* deixar explícito que Cargo é mecanismo de implementação, não contrato da linguagem.

---

## 27. Síntese

O workspace Rust da implementação oficial deve começar pequeno, acíclico e testável.

Sua arquitetura inicial deve permitir `capic --help`, `capic --version` e `cargo test --workspace`, preservando fronteiras que suportem a evolução posterior para lexer, parser, HIR, MIR, runtime, Cranelift, LLVM, bootstrap e auto-hospedagem.
