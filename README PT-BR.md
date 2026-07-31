# Capi

**Capi** é uma linguagem de programação orientada a objetos projetada em torno de ownership explícito de regiões de memória, identidade estável de objetos, gerenciamento determinístico de recursos e concorrência segura.

A linguagem não tenta adaptar o modelo de ownership do Rust à programação orientada a objetos tradicional. Em vez disso, Capi define seu próprio modelo de objetos:

- objetos representam identidade lógica e comportamento;
- armazenamento físico mutável pertence a unidades maiores chamadas **Domains**;
- referências a objetos são representadas por valores estáveis de **ObjectId**;
- mutação é controlada pelo sistema de tipos, por capacidades e por regras de domínio;
- segurança de memória deve ser garantida estaticamente sempre que possível, sem exigir um coletor de lixo obrigatório ou um modelo baseado em contagem de referências.

## Estado do Repositório

Capi está atualmente no encerramento do **Stage 0 - Fundação do Projeto** da implementação oficial.

O repositório contém a especificação da linguagem, a especificação de implementação, decisões arquiteturais, documentação de engenharia e o workspace Rust inicial da implementação oficial.

Existe um executável mínimo `capic` em `capi-lang/`. Ele suporta os comandos de Stage 0 `--help`, `--version`, validação de argumentos, erros de carregamento de arquivo fonte, inicialização de sessão e tratamento básico de erros internos. Ele ainda não compila programas Capi.

O pacote de documentação também foi consolidado com índices de topo, changelog, documentos de engenharia aprovados do Stage 0 e o registro formal de progresso do Stage 0.

## Fase Atual

O **Stage 0 - Fundação do Projeto** foi concluído localmente conforme os critérios de validação do Stage 0.

O Stage 0 criou:

- o workspace Cargo inicial em `capi-lang/`;
- os primeiros crates do compilador;
- o executável mínimo `capic`;
- infraestrutura de build, formatação, lint, testes e CI;
- as regras iniciais de engenharia que orientam o desenvolvimento.

Os documentos de engenharia e ADRs exigidos para o Stage 0 foram aprovados na documentação do projeto.

O registro formal de progresso é:

- [`FEATURE-STATUS.md`](capi-docs/docs/engineering/planning/FEATURE-STATUS.md)

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
- [`Documentação de engenharia`](capi-docs/docs/engineering/) - arquitetura, build, testes, desenvolvimento, planejamento, runtime, toolchain, release e segurança.
- [`ADRs`](capi-docs/docs/adr/) - registros de decisões arquiteturais.
- [`RFCs`](capi-docs/docs/rfc/) - propostas futuras de mudança na linguagem e no projeto.
- [`Governança`](capi-docs/docs/governance/) - processo de decisão e papéis do projeto.

O documento mais recente de planejamento da implementação é:

- [`28 - Plano de Desenvolvimento da Implementação Oficial`](capi-docs/docs/specification/implementation/28%20%E2%80%94%20Plano%20de%20Desenvolvimento%20da%20Implementa%C3%A7%C3%A3o%20Oficial.md)

Os índices de engenharia atualmente ativos são:

- [`Arquitetura`](capi-docs/docs/engineering/architecture/README.md)
- [`Build e CI`](capi-docs/docs/engineering/build-and-ci/README.md)
- [`Desenvolvimento`](capi-docs/docs/engineering/development/README.md)
- [`Testes`](capi-docs/docs/engineering/testing/README.md)
- [`Planejamento`](capi-docs/docs/engineering/planning/README.md)

## Modelo de Desenvolvimento

A implementação oficial é planejada como um compilador e uma toolchain baseados em Rust, inicialmente construídos com um workspace Cargo.

Os crates atuais do workspace no Stage 0 são:

- `capi-cli` - executável mínimo `capic`;
- `capi-driver` - driver inicial do compilador;
- `capi-session` - configuração da sessão de compilação;
- `capi-diagnostics` - infraestrutura básica de diagnósticos;
- `capi-source` - infraestrutura de carregamento de arquivos fonte;
- `capi-common` - tipos fundamentais compartilhados.

O workspace atualmente não possui dependências externas de crates Rust. A política de dependências está registrada em [`capi-lang/DEPENDENCIES.md`](capi-lang/DEPENDENCIES.md).

A política inicial da toolchain Rust está registrada em [`capi-lang/TOOLCHAIN.md`](capi-lang/TOOLCHAIN.md). O Stage 0 fixa Rust `1.88.0`.

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
- documentos de engenharia do Stage 0 aprovados;
- índices de documentação para `capi-docs`, `docs`, ADRs, engenharia, arquitetura, build/CI, desenvolvimento, testes e planejamento;
- um changelog de `capi-docs`;
- um workspace Cargo Rust em `capi-lang`;
- os crates fundamentais do Stage 0;
- um executável mínimo `capic`;
- scripts de validação local e configuração de workflow de CI.

O conjunto de validação local do Stage 0 inclui:

```bash
cd capi-lang
cargo build --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked
cargo test --workspace --locked
cargo run -p capi-cli --locked -- --help
cargo run -p capi-cli --locked -- --version
scripts/ci-local.sh
```

O workflow de CI está versionado em [`.github/workflows/capi-lang-ci.yml`](.github/workflows/capi-lang-ci.yml). A execução remota da CI depende de um evento de `push` ou `pull_request`, enquanto a validação local equivalente passou para o Stage 0.

## O Que Ainda Não Está Pronto

O repositório ainda não fornece:

- um compilador capaz de compilar programas Capi;
- um gerenciador de pacotes;
- uma implementação da biblioteca padrão;
- uma implementação do runtime;
- uma toolchain instalável;
- garantias de compatibilidade para programas de usuário.

## Roadmap Imediato

O próximo passo imediato é iniciar o stage seguinte definido pelo Documento 28, usando a fundação criada no Stage 0.

## Nota do Projeto

Capi ainda está em fase inicial. O repositório é intencionalmente documentation-first para que a implementação possa ser rastreável à especificação da linguagem, às decisões arquiteturais e às restrições de engenharia desde o começo.
