# Capi

**Capi** é uma linguagem de programação orientada a objetos projetada em torno de ownership explícito de regiões de memória, identidade estável de objetos, gerenciamento determinístico de recursos e concorrência segura.

A linguagem não tenta adaptar o modelo de ownership do Rust à programação orientada a objetos tradicional. Em vez disso, Capi define seu próprio modelo de objetos:

- objetos representam identidade lógica e comportamento;
- armazenamento físico mutável pertence a unidades maiores chamadas **Domains**;
- referências a objetos são representadas por valores estáveis de **ObjectId**;
- mutação é controlada pelo sistema de tipos, por capacidades e por regras de domínio;
- segurança de memória deve ser garantida estaticamente sempre que possível, sem exigir um coletor de lixo obrigatório ou um modelo baseado em contagem de referências.

## Estado do Repositório

Capi está atualmente na fase de especificação e planejamento da implementação.

O repositório contém a especificação da linguagem, a especificação de implementação, decisões arquiteturais e a documentação inicial de engenharia necessária para iniciar a implementação oficial.

Ainda não existe um compilador funcional neste repositório. O diretório `capi-lang/` está reservado para a implementação oficial em Rust, mas o workspace ainda não foi criado.

## Fase Atual

O projeto está preparando o **Stage 0 - Fundação do Projeto**.

O Stage 0 é a etapa de fundação da implementação oficial. Seu objetivo é criar:

- o workspace Cargo inicial em `capi-lang/`;
- os primeiros crates do compilador;
- o executável mínimo `capic`;
- infraestrutura de build, formatação, lint, testes e CI;
- as regras iniciais de engenharia que orientam o desenvolvimento.

Os documentos de engenharia bloqueantes exigidos antes da criação do workspace e dos crates fundamentais foram redigidos:

- princípios de engenharia;
- estrutura do projeto;
- arquitetura do compilador;
- arquitetura do workspace;
- responsabilidades dos componentes;
- regras de dependência;
- setup de desenvolvimento;
- sistema de build;
- estratégia de testes;
- definition of done.

Os próximos itens de trabalho são os documentos operacionais e de consolidação restantes do Stage 0, seguidos pelas ADRs obrigatórias do Stage 0 e então pela criação efetiva do workspace `capi-lang`.

## Estrutura do Repositório

```text
capi/
├── capi-docs/
│   └── documentação do projeto, especificações, ADRs, RFCs e documentos de engenharia
│
└── capi-lang/
    └── reservado para o compilador oficial, runtime, biblioteca padrão e toolchain
```

## Mapa da Documentação

A documentação principal fica em [`capi-docs/docs`](capi-docs/docs/).

Pontos de entrada importantes:

- [`Especificação`](capi-docs/docs/specification/README.md) - especificação da linguagem e da implementação.
- [`Especificação da linguagem`](capi-docs/docs/specification/language/) - documentos `00` a `12`.
- [`Especificação de implementação`](capi-docs/docs/specification/implementation/) - documentos `13` a `28`.
- [`Documentação de engenharia`](capi-docs/docs/engineering/) - arquitetura, build, testes, desenvolvimento, planejamento, runtime, toolchain, release e segurança.
- [`ADRs`](capi-docs/docs/adr/) - registros de decisões arquiteturais.
- [`RFCs`](capi-docs/docs/rfc/) - propostas futuras de mudança na linguagem e no projeto.
- [`Governança`](capi-docs/docs/governance/) - processo de decisão e papéis do projeto.

O documento mais recente de planejamento da implementação é:

- [`28 - Plano de Desenvolvimento da Implementação Oficial`](capi-docs/docs/specification/implementation/28%20%E2%80%94%20Plano%20de%20Desenvolvimento%20da%20Implementa%C3%A7%C3%A3o%20Oficial.md)

## Modelo de Desenvolvimento

A implementação oficial é planejada como um compilador e uma toolchain baseados em Rust, inicialmente construídos com um workspace Cargo.

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
- ADRs iniciais para decisões centrais de implementação;
- os documentos de engenharia bloqueantes do Stage 0 redigidos;
- um plano documentado para construir a implementação oficial.

## O Que Ainda Não Está Pronto

O repositório ainda não fornece:

- um compilador utilizável;
- um gerenciador de pacotes;
- uma implementação da biblioteca padrão;
- uma implementação do runtime;
- uma toolchain instalável;
- garantias de compatibilidade para programas de usuário.

## Roadmap Imediato

O plano imediato é:

1. Finalizar os documentos operacionais e de consolidação do Stage 0.
2. Aprovar as ADRs obrigatórias do Stage 0.
3. Criar o workspace Cargo em `capi-lang`.
4. Criar os crates fundamentais do compilador.
5. Implementar os comandos mínimos `capic --help` e `capic --version`.
6. Configurar build, formatação, lint, testes e CI.
7. Validar a Definition of Done do Stage 0.

## Nota do Projeto

Capi ainda está em fase inicial. O repositório é intencionalmente documentation-first para que a implementação possa ser rastreável à especificação da linguagem, às decisões arquiteturais e às restrições de engenharia desde o começo.
