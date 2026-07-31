# Engineering Glossary

**Projeto:** Linguagem Capi  
**Documento:** ENGINEERING-GLOSSARY  
**Status:** Aprovado  
**Stage:** Stage 0 — Fundação do projeto  
**Natureza:** Documento de consolidação  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento consolida o vocabulário de engenharia usado na implementação oficial da Linguagem Capi.

Seu objetivo é reduzir ambiguidade entre especificação, documentos de engenharia, ADRs, código, testes e revisão.

Este glossário não cria novas regras da linguagem. Quando uma definição normativa for necessária, prevalecem os documentos de especificação correspondentes.

---

## 2. Escopo

Este documento cobre termos relacionados a:

- linguagem;
- compilador;
- runtime;
- workspace;
- build;
- testes;
- planejamento;
- documentação;
- decisões arquiteturais.

Termos específicos de uma fase futura podem ser detalhados em documentos próprios e apenas resumidos aqui.

---

## 3. Convenções de Uso

Regras gerais:

- termos técnicos centrais podem permanecer em inglês quando forem nomes consolidados no compilador, como `frontend`, `backend`, `lowering`, `snapshot` e `bootstrap`;
- nomes de representações internas devem manter a sigla oficial, como AST, HIR e MIR;
- nomes de entidades da linguagem devem preservar a forma usada na especificação, como `Domain`, `ObjectId` e `capic`;
- documentos podem usar português ao redor do termo técnico sem traduzir o identificador.

Exemplo recomendado:

```text
O parser produz AST.
O lowering transforma AST em HIR.
O backend consome MIR validada.
```

Exemplo a evitar:

```text
O analisador transforma a árvore abstrata em representação intermediária alta e depois em representação média.
```

---

## 4. Termos Fundamentais da Linguagem

### ABI

Application Binary Interface.

Contrato binário entre código gerado, runtime, bibliotecas, plataforma e FFI.

### Capability

Permissão ou capacidade associada a um valor, referência, operação ou contexto.

Na Capi, capacidades ajudam a controlar mutabilidade, acesso e efeitos permitidos.

### Classe

Unidade de definição de comportamento, interface e estrutura lógica de objetos.

Classe não deve ser confundida com alocação física direta de cada objeto.

### Concorrência segura

Propriedade esperada da linguagem em que operações concorrentes preservam as garantias de memória, mutabilidade e domínio.

### Determinismo semântico

Propriedade em que o comportamento observável da linguagem não depende de detalhes acidentais da implementação.

### Domain

Unidade física de organização e controle de memória mutável.

Objetos não são donos diretos da memória física mutável. Essa responsabilidade pertence a `Domain`.

### FFI

Foreign Function Interface.

Fronteira explícita entre código Capi e código externo. FFI pode exigir regras especiais de ABI, segurança e `unsafe`.

### Identidade lógica

Identidade estável de uma entidade do programa independentemente de detalhes físicos de armazenamento.

Em Capi, objetos possuem identidade lógica representada por `ObjectId`.

### Mutabilidade controlada

Modelo em que escritas e alterações de estado são mediadas por tipos, capacidades e regras de domínio.

### ObjectId

Identificador estável de objeto.

Representa identidade lógica, não ponteiro bruto, endereço físico ou propriedade direta de memória.

### Objeto

Entidade da linguagem que representa identidade lógica e comportamento.

Objeto não deve ser tratado como sinônimo de bloco de memória diretamente possuído por si mesmo.

### Ownership

Conjunto de regras que define posse, acesso, movimentação e descarte de valores ou recursos.

Na implementação da Capi, ownership deve ser tratado como garantia da linguagem, não como simples reaproveitamento do ownership de Rust.

### Região

Unidade lógica usada em análises de vida, escopo, acesso e organização de valores.

Regiões ajudam a conectar regras de memória, domains e verificação semântica.

### Runtime

Infraestrutura de execução necessária para preservar garantias da linguagem em tempo de execução.

Runtime não redefine semântica; ele implementa mecanismos necessários para sustentá-la.

### Semântica operacional

Definição de como programas Capi são avaliados e quais comportamentos são observáveis.

---

## 5. Termos do Compilador

### AST

Abstract Syntax Tree.

Representação estrutural produzida pelo parser, ainda próxima da sintaxe escrita pelo usuário.

### Backend

Camada responsável por transformar MIR validada em artefatos de código nativo ou intermediários específicos de backend.

Backends planejados incluem Cranelift inicialmente e LLVM posteriormente.

### Borrow checking

Verificação de empréstimos, acessos e restrições de uso associadas a ownership, mutabilidade e regiões.

### Codegen

Geração de código.

Fase que transforma representações internas em artefatos consumidos por backend, linker ou plataforma.

### Compilador oficial

Implementação de referência mantida pelo projeto Capi.

O compilador oficial é mecanismo da implementação e não substitui a especificação como fonte normativa.

### Compilation session

Contexto operacional de uma invocação do compilador.

Inclui opções, configuração, source map, diagnósticos, target e serviços compartilhados.

### Cranelift

Backend inicial planejado para geração de código nativo.

Cranelift é mecanismo de implementação, não definição da linguagem.

### Diagnóstico

Mensagem estruturada emitida pelo compilador para erro, aviso, nota ou falha interna.

Diagnósticos devem estar ligados a spans quando aplicável e devem ser testáveis.

### Driver

Componente que coordena a execução do pipeline do compilador.

O driver seleciona ações, prepara sessões e chama fases, mas não implementa lexer, parser, type checker ou backend.

### Frontend

Camada que transforma fonte Capi em representações analisadas semanticamente.

Inclui source loading, lexer, parser, AST, HIR, resolução de nomes, checagem de tipos e verificações semânticas.

### HIR

High-level Intermediate Representation.

Representação semântica de alto nível, derivada da AST e preparada para análises semânticas.

### Interning

Técnica para armazenar valores repetidos, como símbolos ou tipos, uma única vez e referenciá-los por identificadores internos.

### Lexer

Fase que transforma texto-fonte em tokens.

### LLVM

Backend planejado para estágio posterior, voltado a otimização, compatibilidade e validação diferencial.

LLVM é mecanismo de implementação, não definição da linguagem.

### Lowering

Transformação de uma representação para outra mais adequada à próxima fase.

Exemplos: AST para HIR, HIR para MIR.

### Middle-end

Camada entre frontend e backend.

Responsável por MIR, validações, passes e transformações independentes de backend.

### MIR

Mid-level Intermediate Representation.

Representação intermediária usada como fronteira central entre análises semânticas e backend.

MIR deve permanecer independente de Cranelift e LLVM.

### Parser

Fase que transforma tokens em AST.

### Pipeline

Sequência de fases do compilador, desde carregamento de fontes até geração de artefatos.

### Source map

Estrutura que relaciona arquivos-fonte, posições, spans e localizações apresentáveis em diagnósticos.

### Span

Intervalo em um arquivo-fonte.

Spans permitem associar tokens, nós de AST, entidades semânticas e diagnósticos a posições no código.

### Symbol

Identificador interno associado a nomes do programa.

Pode ser representado por interning para evitar duplicação e permitir comparação eficiente.

### Token

Unidade léxica produzida pelo lexer e consumida pelo parser.

### Type checking

Fase que verifica se expressões, declarações e operações respeitam o sistema de tipos.

---

## 6. Termos de Workspace e Crates

### `capi-lang`

Diretório reservado para a implementação oficial da linguagem.

Deve conter workspace Rust, compilador, runtime, biblioteca padrão, toolchain, testes e scripts.

### `capi-docs`

Diretório da documentação oficial do projeto.

Contém especificação, engenharia, ADRs, RFCs, governança e templates.

### Cargo

Sistema de build e gerenciamento de pacotes usado pela implementação inicial em Rust.

Cargo é mecanismo de implementação.

### Cargo workspace

Workspace Rust que agrupa os crates da implementação oficial.

No Stage 0, deve ser criado em `capi-lang`.

### Crate

Unidade de compilação e empacotamento Rust.

Na implementação oficial, crates devem refletir fronteiras arquiteturais e responsabilidades claras.

### Crate binário

Crate que produz executável.

Exemplo planejado: `capi-cli`, responsável pelo binário `capic`.

### Crate de biblioteca

Crate que expõe funcionalidade para outros crates.

Exemplos planejados: `capi-driver`, `capi-session`, `capi-diagnostics`, `capi-source`.

### Grafo de dependências

Conjunto de dependências entre crates.

Deve ser acíclico e respeitar as regras de dependência do projeto.

### Workspace root

Raiz do workspace Cargo.

Na implementação oficial, corresponde a `capi-lang/` após sua criação.

---

## 7. Crates Fundamentais do Stage 0

### `capi-cli`

Crate binário responsável pelo executável `capic`.

Deve ser fino e delegar coordenação ao driver.

### `capi-common`

Crate para tipos e utilidades realmente compartilhados.

Não deve virar depósito genérico de lógica sem responsabilidade clara.

### `capi-diagnostics`

Crate responsável por diagnósticos estruturados, severidade, mensagens, renderização inicial e erros internos básicos.

### `capi-driver`

Crate responsável por coordenar o pipeline disponível em uma sessão de compilação.

### `capi-session`

Crate responsável pelo contexto operacional de uma invocação do compilador.

### `capi-source`

Crate responsável por arquivos-fonte, source map, spans e localização.

---

## 8. Termos de Build e CI

### Artefato

Resultado produzido por build, teste, codegen ou release.

Exemplos: binário, arquivo objeto, relatório, documentação gerada.

### Build

Processo de compilação do workspace ou de parte dele.

No Stage 0, o comando mínimo é `cargo build --workspace`.

### CI

Continuous Integration.

Automação que executa validações do projeto, como build, formatação, lint e testes.

### Clippy

Ferramenta de lint para Rust.

No Stage 0, deve ser executada com `cargo clippy --workspace --all-targets`.

### Formatação

Aplicação de estilo automático de código.

No Stage 0, deve ser validada com `cargo fmt --all --check`.

### Lockfile

Arquivo que fixa versões resolvidas de dependências.

No workspace Rust, corresponde a `Cargo.lock` e deve ser versionado.

### Reprodutibilidade

Capacidade de obter resultado equivalente em ambientes equivalentes.

É requisito para build, testes, CI e bootstrap.

### Rustfmt

Ferramenta oficial de formatação de código Rust.

### Target

Plataforma ou alvo de compilação.

Pode incluir arquitetura, sistema operacional, ABI e convenções de geração de código.

---

## 9. Termos de Testes

### `compile-fail`

Teste que valida programa Capi que deve ser rejeitado pelo compilador.

### `compile-pass`

Teste que valida programa Capi que deve ser aceito pelo compilador, sem necessariamente executá-lo.

### Conformance suite

Suíte de conformidade que valida a implementação contra a especificação da linguagem.

### Differential test

Teste que compara duas rotas de implementação para o mesmo comportamento.

Exemplos futuros: backend Cranelift versus LLVM, compilador bootstrap versus anterior.

### Fixture

Arquivo ou dado de entrada usado por testes.

Fixtures devem ser versionadas, estáveis e ter finalidade clara.

### Fuzzing

Técnica de teste com geração ampla de entradas para encontrar falhas de robustez.

### Golden file

Arquivo esperado usado para comparar saída produzida por testes.

### Regression test

Teste adicionado para garantir que um bug corrigido não retorne.

### `run-pass`

Teste que valida programa Capi que deve compilar e executar com sucesso.

### Snapshot

Registro versionado de uma saída esperada, como diagnóstico ou dump de representação interna.

Snapshots devem ser revisados como código.

### UI test

Teste voltado ao comportamento observável pelo usuário, especialmente diagnósticos e saída de CLI.

### Unit test

Teste local de uma função, módulo ou unidade pequena de comportamento.

---

## 10. Termos de Planejamento e Processo

### ADR

Architecture Decision Record.

Documento que registra uma decisão arquitetural, contexto, alternativas e consequências.

### Bloqueante

Documento, decisão ou entrega que precisa existir antes de iniciar o trabalho que depende dele.

### Definition of Done

Conjunto de critérios objetivos para considerar uma entrega concluída.

### Documento de consolidação

Documento que organiza, resume ou estabiliza decisões e vocabulário.

Pode amadurecer durante o stage, mas deve estar aprovado antes da conclusão formal quando for obrigatório.

### Documento operacional

Documento que define procedimentos, comandos, rotinas ou critérios práticos de execução.

### Entrega

Resultado esperado de um stage ou tarefa.

Pode ser documental, infraestrutural, de implementação, de teste ou validação.

### Milestone

Marco técnico ou operacional relevante do projeto.

### RFC

Request for Comments.

Processo usado para propor mudanças relevantes na linguagem, especificação ou projeto.

### Stage

Unidade de planejamento e validação do desenvolvimento.

Cada stage possui objetivo, dependências, entregas e critérios de conclusão.

### Stage 0

Stage de fundação do projeto.

Seu objetivo é criar a base documental, arquitetural, de workspace, build, testes e CLI mínima para iniciar a implementação oficial.

---

## 11. Termos de Documentação e Normatividade

### Documento normativo

Documento que define regras ou contratos que devem ser obedecidos.

### Documento de engenharia

Documento que transforma especificação e decisões em orientação prática para implementação, revisão, teste e manutenção.

### Especificação

Fonte primária de verdade da linguagem e dos contratos de implementação.

Código não substitui especificação.

### Garantia da linguagem

Propriedade prometida pela linguagem ao programador.

Exemplos: segurança de memória, identidade estável, mutação controlada.

### Mecanismo da implementação

Escolha técnica usada para implementar uma garantia.

Exemplos: Cargo, Rust, Cranelift, LLVM, estrutura interna de crates.

### Rastreabilidade

Capacidade de conectar uma implementação, teste ou decisão ao documento que a justifica.

### Subconjunto inicial

Parte limitada da linguagem implementada durante bootstrap ou stages iniciais.

Subconjuntos não devem redefinir a linguagem completa.

---

## 12. Termos de Segurança e Erros

### Erro do usuário

Erro causado por programa, entrada ou uso inválido da ferramenta.

Deve ser reportado como diagnóstico, não como falha interna.

### Erro interno do compilador

Falha causada por violação de invariante interna ou bug da implementação.

Deve ser distinguível de erro do usuário.

### Panic

Falha abrupta no programa Rust.

Não deve ser usado como mecanismo normal para reportar erro esperado de usuário.

### Segurança por construção

Princípio de modelar estruturas e APIs para tornar estados inválidos difíceis ou impossíveis de representar.

### `unsafe`

Código Rust que exige manutenção manual de invariantes de segurança.

Na implementação da Capi, deve ser minimizado, documentado, revisado e testado.

---

## 13. Termos Preferenciais

Use os termos abaixo de forma consistente:

| Preferir | Evitar quando ambíguo |
| --- | --- |
| `Domain` | domínio de memória sem qualificação |
| `ObjectId` | ponteiro de objeto |
| `frontend` | analisador inteiro |
| `middle-end` | otimizador genérico |
| `backend` | gerador final genérico sem contexto |
| AST | árvore sintática abstrata em nomes técnicos |
| HIR | IR alto em nomes técnicos |
| MIR | IR médio em nomes técnicos |
| `lowering` | conversão genérica sem fase |
| diagnóstico | mensagem de erro quando incluir warnings/notas |
| erro interno do compilador | erro do usuário |
| stage | fase, quando se referir ao plano do Documento 28 |
| crate | pacote Rust, quando o contexto for Cargo |
| workspace Cargo | repositório inteiro |
| repositório | workspace, quando se referir ao projeto inteiro |

---

## 14. Critérios de Aceitação do Documento

Este documento é considerado preenchido quando:

- consolida os termos centrais da linguagem e da implementação;
- preserva a separação entre garantias e mecanismos;
- define vocabulário comum para Stage 0;
- inclui termos de compilador, workspace, build, testes e planejamento;
- não introduz regras normativas novas;
- permanece coerente com os documentos de especificação e engenharia existentes.

---

## 15. Síntese

Este glossário existe para manter a implementação da Capi linguisticamente consistente.

Termos consistentes reduzem ambiguidade, facilitam revisão e ajudam a preservar rastreabilidade entre documentação, código, testes e decisões arquiteturais.
