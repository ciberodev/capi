# Capi

**Capi** é uma linguagem de programação orientada a objetos projetada em torno de ownership explícito de regiões de memória, identidade estável de objetos, gerenciamento determinístico de recursos e concorrência segura.

A linguagem não adapta o modelo de ownership do Rust à programação orientada a objetos tradicional. Em vez disso, Capi define seu próprio modelo de objetos:

- objetos representam identidade lógica e comportamento;
- armazenamento físico mutável pertence a unidades maiores chamadas **Domains**;
- referências a objetos são representadas por valores estáveis de **ObjectId**;
- mutação é controlada pelo sistema de tipos, por capacidades e por regras de domínio;
- segurança de memória deve ser garantida estaticamente sempre que possível, sem exigir um coletor de lixo obrigatório ou um modelo baseado em contagem de referências.

## Estado do Repositório

Capi está atualmente após o **Stage 3 - HIR e resolução de nomes** da implementação oficial.

O repositório contém a especificação da linguagem, a especificação da implementação, decisões arquiteturais, documentação de engenharia, registros de planejamento e o workspace Rust da implementação oficial.

Existe um executável `capic` em `capi-lang/`. Atualmente ele suporta:

- `--help`;
- `--version`;
- validação de argumentos;
- erros de carregamento de arquivo fonte;
- inicialização de sessão;
- tratamento básico de erros internos;
- dumps de tokens do Stage 1;
- dumps de AST do Stage 2;
- dumps de HIR resolvida do Stage 3.

Comandos demonstráveis atuais:

```bash
capic --emit tokens arquivo.capi
capic --emit ast arquivo.capi
capic --emit hir arquivo.capi
```

Ele ainda não compila programas Capi.

O próximo stage planejado é:

```text
Stage 4 - Sistema de tipos
```

## Fase Atual

O **Stage 0 - Fundação do Projeto** está concluído.

O Stage 0 entregou:

- o workspace Cargo inicial em `capi-lang/`;
- os primeiros crates do compilador;
- o executável mínimo `capic`;
- infraestrutura de build, formatação, lint, testes, documentação e CI;
- as regras iniciais de engenharia que orientam o desenvolvimento.

O **Stage 1 - Infraestrutura de Fontes, Diagnósticos e Lexer** está concluído.

O Stage 1 entregou:

- gerenciamento de fontes com `SourceId`, `SourceFile`, `SourceMap`, `Span` e consulta de linha e coluna;
- diagnósticos estruturados com códigos, labels, notas, sugestões e renderização;
- o modelo inicial de tokens;
- o lexer inicial para identificadores, keywords, literais, operadores, delimitadores, comentários, EOF e erros léxicos;
- suporte a dump de tokens por meio de `capic --emit tokens`;
- fixtures do lexer, snapshots, testes léxicos compile-fail, testes de posição de diagnósticos e testes de robustez contra entradas malformadas.

O **Stage 2 - Parser e AST** está concluído.

O Stage 2 entregou:

- o crate `capi-ast`;
- o crate `capi-parser`;
- nós de AST para unidades de compilação, módulos, imports, declarações, classes, funções, tipos, comandos, expressões, padrões e nós de erro;
- preservação de spans nos nós relevantes da AST;
- suporte de parser para o subconjunto sintático inicial;
- precedência e associatividade de operadores;
- diagnósticos sintáticos estruturados com códigos `PARSE`;
- recuperação após erros sintáticos recuperáveis;
- ASTs parciais com nós de erro explícitos;
- dumps determinísticos da AST;
- testes de snapshot golden para dumps da AST;
- suporte a dump de AST por meio de `capic --emit ast`.

O **Stage 3 - HIR e Resolução de Nomes** está concluído.

O Stage 3 entregou:

- o crate `capi-hir` como modelo HIR puro, sem dependência direta da AST;
- o crate `capi-lowering` como fronteira AST -> HIR;
- o crate `capi-sema` para escopos, símbolos e resolução de nomes;
- IDs HIR tipados e determinísticos;
- identidades determinísticas `ScopeId` e `SymbolId`;
- lowering de AST para HIR com preservação de fonte e spans;
- tabelas de símbolos e grafos de escopo para o subconjunto inicial;
- resolução de nomes para valores, tipos, módulos/imports e padrões do subconjunto inicial;
- diagnósticos semânticos estruturados para referências duplicadas, inexistentes e ambíguas;
- dumps determinísticos de HIR inicial e HIR resolvida;
- suporte a dump de HIR resolvida por meio de `capic --emit hir`.

O registro formal de progresso é:

- [`FEATURE-STATUS.md`](capi-docs/docs/engineering/planning/FEATURE-STATUS.md)

A ordem de implementação, milestones, roadmap, riscos e registros de dívida técnica são:

- [`IMPLEMENTATION-ORDER.md`](capi-docs/docs/engineering/planning/IMPLEMENTATION-ORDER.md)
- [`MILESTONES.md`](capi-docs/docs/engineering/planning/MILESTONES.md)
- [`ROADMAP.md`](capi-docs/docs/engineering/planning/ROADMAP.md)
- [`RISK-REGISTER.md`](capi-docs/docs/engineering/planning/RISK-REGISTER.md)
- [`TECHNICAL-DEBT.md`](capi-docs/docs/engineering/planning/TECHNICAL-DEBT.md)

O changelog da documentação é:

- [`capi-docs/CHANGELOG.md`](capi-docs/CHANGELOG.md)

## Estrutura do Repositório

```text
capi/
├── capi-docs/
│   └── documentação do projeto, especificações, ADRs, RFCs, templates e documentos de engenharia
│
├── .github/
│   └── workflows de CI
│
└── capi-lang/
    └── workspace da implementação oficial
```

## Mapa da Documentação

A documentação principal fica em [`capi-docs/docs`](capi-docs/docs/).

Pontos de entrada importantes:

- [`Capi Docs`](capi-docs/README.md) - visão geral do pacote de documentação.
- [`Raiz da documentação`](capi-docs/docs/README.md) - mapa da árvore de documentação.
- [`Especificação`](capi-docs/docs/specification/README.md) - especificação da linguagem e da implementação.
- [`Especificação da linguagem`](capi-docs/docs/specification/language/) - documentos `00` a `12`.
- [`Especificação de implementação`](capi-docs/docs/specification/implementation/) - documentos `13` a `28`.
- [`Documentação de engenharia`](capi-docs/docs/engineering/) - arquitetura, build, testes, desenvolvimento, planejamento e documentação do compilador.
- [`Engenharia do compilador`](capi-docs/docs/engineering/compiler/README.md) - fontes, diagnósticos, lexer, parser, AST, HIR e semântica inicial.
- [`Engenharia de testes`](capi-docs/docs/engineering/testing/README.md) - estratégia de testes, testes do lexer, testes do parser e testes semânticos.
- [`Planejamento`](capi-docs/docs/engineering/planning/README.md) - definição de pronto, status de features, ordem de implementação, milestones, roadmap, riscos e dívida técnica.
- [`ADRs`](capi-docs/docs/adr/) - registros de decisões arquiteturais.

O documento de planejamento da implementação é:

- [`28 - Plano de Desenvolvimento da Implementação Oficial`](capi-docs/docs/specification/implementation/28%20%E2%80%94%20Plano%20de%20Desenvolvimento%20da%20Implementa%C3%A7%C3%A3o%20Oficial.md)

## Modelo de Desenvolvimento

A implementação oficial é um compilador e futura toolchain baseados em Rust, inicialmente construídos com um workspace Cargo.

Os crates atuais do workspace são:

- `capi-cli` - executável `capic` e parsing de argumentos;
- `capi-driver` - orquestração do driver do compilador e saídas de dump;
- `capi-common` - tipos e constantes fundamentais compartilhados;
- `capi-session` - configuração da sessão de compilação;
- `capi-source` - carregamento de arquivos fonte, source maps, spans e consulta de linha e coluna;
- `capi-diagnostics` - infraestrutura de diagnósticos estruturados;
- `capi-lexer` - modelo de tokens e lexer inicial;
- `capi-ast` - modelo da árvore sintática abstrata e dump determinístico da AST;
- `capi-parser` - parser, diagnósticos sintáticos, recuperação e construção da AST;
- `capi-hir` - modelo de representação semântica de alto nível e dump determinístico da HIR;
- `capi-lowering` - lowering de AST para HIR;
- `capi-sema` - grafo de escopos, tabela de símbolos e resolução inicial de nomes.

O workspace atualmente não possui dependências externas de crates Rust. A política de dependências está registrada em [`capi-lang/DEPENDENCIES.md`](capi-lang/DEPENDENCIES.md).

A política da toolchain Rust está registrada em [`capi-lang/TOOLCHAIN.md`](capi-lang/TOOLCHAIN.md). O workspace fixa Rust `1.88.0`.

A arquitetura planejada do compilador separa:

- gerenciamento de fontes;
- lexer;
- parser e AST;
- HIR;
- resolução de nomes;
- checagem de tipos;
- análise de ownership, regiões e domains;
- MIR;
- otimizações independentes de backend;
- geração de código;
- runtime;
- biblioteca padrão;
- CLI e ferramentas.

Cranelift é planejado como o backend inicial. LLVM é planejado como backend posterior de otimização e compatibilidade.

## Idioma da Linguagem e da Documentação

A documentação oficial do projeto é escrita em português brasileiro (PT-BR).

Isso é intencional. A especificação, a documentação de engenharia, as ADRs e os documentos de planejamento permanecerão em PT-BR durante o design e a implementação da linguagem.

Documentação em inglês, como manuais de usuário, guias, tutoriais e materiais públicos de aprendizado, poderá ser produzida depois que a linguagem e a toolchain estiverem maduras o suficiente para usuários.

## O Que Está Pronto

Neste momento, o repositório possui:

- um conjunto completo de especificação da linguagem, dos documentos `00` a `12`;
- um conjunto de especificação de implementação, dos documentos `13` a `28`;
- ADRs aprovadas do Stage 0 para decisões centrais de implementação;
- documentos de engenharia aprovados para os Stages 0, 1, 2 e 3;
- índices de documentação para `capi-docs`, `docs`, ADRs, engenharia, compilador, arquitetura, build/CI, desenvolvimento, testes e planejamento;
- registros de planejamento para status de features, ordem de implementação, milestones, roadmap, riscos e dívida técnica;
- um changelog de `capi-docs`;
- um workspace Cargo Rust em `capi-lang`;
- os crates fundamentais do compilador mais os crates de fontes, diagnósticos, lexer, AST, parser, HIR, lowering e semântica;
- um executável `capic` com suporte a dump de tokens, AST e HIR resolvida;
- scripts de validação local e configuração de workflow de CI;
- fixtures do lexer e testes de snapshot;
- testes de integração do parser e snapshots golden de dump da AST;
- testes de lowering, testes de integração semântica, fixtures semânticas e snapshots de HIR.

O conjunto atual de validação local inclui:

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
cargo run -p capi-cli --locked -- --emit hir tests/semantic/pass/basic.cap
scripts/check.sh
```

O workflow de CI está versionado em [`.github/workflows/capi-lang-ci.yml`](.github/workflows/capi-lang-ci.yml). Ele valida formatação, versões da toolchain, política de dependências, Clippy, testes, build, documentação e smoke tests do `capic` para dumps de tokens, AST e HIR.

## O Que Ainda Não Está Pronto

O repositório ainda não fornece:

- um compilador capaz de compilar programas Capi;
- checagem de tipos;
- análise de ownership e domains;
- MIR;
- geração de código;
- um gerenciador de pacotes;
- uma implementação da biblioteca padrão;
- uma implementação do runtime;
- uma toolchain instalável;
- garantias de compatibilidade para programas de usuário.

## Roadmap Imediato

O próximo passo imediato é o **Stage 4 - Sistema de tipos**.

O Stage 4 deve começar pelos documentos obrigatórios do sistema de tipos e depois implementar:

- modelo interno de tipos;
- interning e canonicalização de tipos;
- inferência inicial de tipos;
- checagem de tipos para o subconjunto inicial;
- regras iniciais de subtipagem e coerções;
- diagnósticos estruturados de tipo.

O comando demonstrável mais recente é:

```bash
capic --emit hir arquivo.capi
```

## Nota do Projeto

Capi ainda está em fase inicial. O repositório é intencionalmente documentation-first para que a implementação possa ser rastreável à especificação da linguagem, às decisões arquiteturais e às restrições de engenharia desde o começo.
