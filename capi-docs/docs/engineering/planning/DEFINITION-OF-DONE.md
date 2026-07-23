# Definition of Done

**Projeto:** Linguagem Capi  
**Documento:** DEFINITION-OF-DONE  
**Status:** Proposto  
**Stage:** Stage 0 — Fundação do projeto  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o que significa uma entrega estar concluída na implementação oficial da linguagem Capi.

Seu objetivo é estabelecer critérios objetivos para aceitar:

- tarefas;
- documentos;
- crates;
- componentes;
- testes;
- milestones demonstráveis;
- stages completos.

Este documento não define ordem de implementação. A ordem pertence ao Documento 28 e aos documentos de planejamento derivados.

---

## 2. Princípio central

Uma entrega só está concluída quando é possível demonstrar, revisar e validar o resultado.

Código existente não basta.

Documento preenchido não basta.

Teste isolado passando não basta.

A conclusão exige coerência entre:

```text
especificação
documentação
implementação
testes
build
CI
rastreabilidade
```

---

## 3. Escopo

A Definition of Done se aplica a:

- alterações de documentação;
- implementação de crates;
- mudanças em APIs internas;
- mudanças de CLI;
- testes;
- infraestrutura de build;
- integração contínua;
- ADRs;
- encerramento de stages.

Ela deve ser usada por autores, revisores e mantenedores antes de considerar uma entrega pronta.

---

## 4. Estados de conclusão

O projeto deve distinguir os seguintes estados:

```text
Não iniciado
Planejado, mas sem trabalho efetivo.

Em andamento
Há trabalho iniciado, mas ainda sem validação completa.

Implementado
O código ou documento principal existe, mas ainda pode faltar revisão, teste ou integração.

Validado
Os comandos, testes e revisões aplicáveis foram executados.

Concluído
Todos os critérios deste documento foram atendidos.

Bloqueado
Existe impedimento conhecido que impede validação ou conclusão.
```

Somente `Concluído` representa entrega aceita.

---

## 5. Definition of Done para tarefas

Uma tarefa está concluída quando:

- o escopo solicitado foi atendido;
- não há trabalho essencial deixado para depois sem registro explícito;
- a mudança respeita os documentos normativos aplicáveis;
- a mudança respeita a arquitetura definida;
- os testes necessários foram adicionados ou atualizados;
- os comandos de validação aplicáveis passam;
- limitações conhecidas foram documentadas;
- decisões novas foram registradas no lugar correto;
- o resultado pode ser explicado e reproduzido por outra pessoa.

Tarefas parciais podem ser úteis, mas não devem ser marcadas como concluídas.

---

## 6. Definition of Done para documentação

Um documento de engenharia está concluído quando:

- possui finalidade clara;
- define seu escopo;
- declara o que não cobre quando isso evita ambiguidade;
- está coerente com a especificação;
- está coerente com os documentos de engenharia já aprovados;
- não introduz novas regras normativas de linguagem sem base na especificação;
- diferencia decisão permanente de limitação temporária;
- aponta critérios de aceitação ou conclusão quando aplicável;
- usa caminhos, nomes e responsabilidades compatíveis com a estrutura oficial;
- pode orientar implementação ou revisão sem depender de conhecimento implícito.

Documentos bloqueantes devem estar aprovados antes da implementação correspondente.

Documentos operacionais devem estar suficientemente definidos antes de serem usados como base de automação.

Documentos de consolidação podem evoluir durante o stage, mas precisam estar aprovados antes do encerramento formal do stage.

---

## 7. Definition of Done para implementação

Uma entrega de código está concluída quando:

- compila no workspace;
- preserva as fronteiras entre crates;
- usa APIs públicas adequadas entre componentes;
- não adiciona dependências proibidas;
- não mistura responsabilidades de frontend, middle-end, backend, runtime e toolchain;
- não transforma mecanismo temporário em semântica da linguagem;
- trata erros esperados com diagnóstico ou erro estruturado;
- evita panic como mecanismo normal de controle;
- possui testes compatíveis com o risco da mudança;
- não deixa código morto relevante sem justificativa;
- não introduz dívida técnica sem registro.

Se a mudança usa `unsafe`, ela só está concluída quando:

- a necessidade do `unsafe` está justificada;
- as invariantes estão documentadas;
- a superfície insegura está minimizada;
- há testes ou validações compatíveis com o risco.

---

## 8. Definition of Done para testes

Uma entrega de teste está concluída quando:

- valida comportamento especificado ou decisão registrada;
- falha pelo motivo correto antes da correção, quando for regressão;
- é determinística;
- pode rodar isoladamente;
- pode rodar em paralelo quando a suíte assim exigir;
- não depende de paths absolutos;
- não depende de estado externo não documentado;
- possui nome descritivo;
- usa fixtures ou snapshots de forma revisável;
- está classificada na suíte adequada.

Mudanças de snapshot só estão concluídas quando a nova saída foi revisada contra a especificação.

---

## 9. Definition of Done para CLI

Uma entrega de CLI está concluída quando:

- o comando documentado existe;
- flags e argumentos têm comportamento previsível;
- `--help` reflete o estado real do comando;
- `--version` retorna versão coerente com o build;
- erros de uso retornam código diferente de zero;
- erros esperados não causam panic não controlado;
- stdout e stderr são usados de forma consistente;
- há teste para o comportamento observável.

No Stage 0, a CLI mínima é considerada concluída apenas quando `capic --help`, `capic --version` e erro para arquivo inexistente estiverem implementados e testados.

---

## 10. Definition of Done para crates

Um crate do workspace está concluído para seu estágio quando:

- sua responsabilidade está definida em documentação;
- seu papel respeita `COMPONENT-RESPONSIBILITIES.md`;
- suas dependências respeitam `DEPENDENCY-RULES.md`;
- sua API pública inicial é suficiente para o estágio;
- seus tipos públicos não expõem detalhes desnecessários de implementação;
- possui testes locais quando houver comportamento próprio;
- compila com `cargo build --workspace`;
- participa de `cargo test --workspace`;
- não exige configuração local oculta.

Crates podem existir como fundação mínima no Stage 0, desde que seu escopo limitado esteja claro.

---

## 11. Definition of Done para build e CI

A infraestrutura de build e CI está concluída quando:

- o workspace Cargo existe;
- `Cargo.lock` está versionado;
- artefatos gerados não são versionados;
- os comandos canônicos funcionam localmente;
- a CI executa comandos equivalentes;
- falhas são visíveis;
- dependências são resolvidas de forma reprodutível;
- scripts documentam os comandos que encapsulam;
- a validação local e a validação de CI não divergem sem justificativa.

Comandos mínimos:

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

## 12. Definition of Done para diagnósticos

Uma entrega de diagnóstico está concluída quando:

- diferencia erro do usuário de erro interno do compilador;
- possui severidade definida;
- aponta arquivo, linha, coluna e span quando aplicável;
- preserva dados estruturados para ferramentas;
- renderiza mensagem compreensível para humanos;
- possui teste para o caso principal;
- não depende de formatação instável do ambiente.

Diagnóstico público não deve mudar acidentalmente.

---

## 13. Definition of Done para ADRs

Um ADR está concluído quando:

- descreve o contexto da decisão;
- explicita a decisão tomada;
- registra alternativas consideradas;
- registra consequências;
- tem status claro;
- é referenciado por documentos ou mudanças que dependem dele;
- não contradiz a especificação.

Decisões arquiteturais relevantes não devem ficar apenas em comentários de código, issues ou conversas.

---

## 14. Definition of Done para milestones demonstráveis

Um milestone demonstrável está concluído quando:

- existe um comando ou procedimento reproduzível;
- o resultado esperado está documentado;
- há teste automatizado quando viável;
- limitações conhecidas estão registradas;
- outra pessoa consegue executar a demonstração no ambiente suportado.

Exemplos:

- `capic --help`;
- `capic --version`;
- dump determinístico de AST, HIR ou MIR;
- diagnóstico estruturado para entrada inválida;
- primeiro programa Capi compilado;
- primeiro programa Capi executado;
- comparação entre backends;
- etapa de bootstrap reproduzível.

---

## 15. Definition of Done para stages

Um stage está concluído quando:

- todos os documentos obrigatórios do stage estão aprovados;
- todos os ADRs obrigatórios estão registrados;
- todas as entregas de infraestrutura estão operacionais;
- todas as entregas de implementação estão concluídas;
- todos os testes obrigatórios passam;
- a CI está verde;
- não há erro bloqueador conhecido;
- limitações conhecidas estão documentadas;
- o resultado demonstrável do stage funciona;
- existe registro de progresso do stage;
- a documentação está coerente com o código entregue.

Nenhum stage deve ser concluído apenas pela existência de documentos ou apenas pela existência de código.

---

## 16. Checklist específico do Stage 0

O Stage 0 está concluído quando todos os itens abaixo forem verdadeiros.

### 16.1 Documentação bloqueante

- `ENGINEERING-PRINCIPLES.md` aprovado;
- `PROJECT-STRUCTURE.md` aprovado;
- `COMPILER-ARCHITECTURE.md` aprovado;
- `WORKSPACE-ARCHITECTURE.md` aprovado;
- `COMPONENT-RESPONSIBILITIES.md` aprovado;
- `DEPENDENCY-RULES.md` aprovado;
- `DEVELOPMENT-SETUP.md` aprovado;
- `BUILD-SYSTEM.md` aprovado;
- `TEST-STRATEGY.md` aprovado;
- `DEFINITION-OF-DONE.md` aprovado.

### 16.2 Documentação de consolidação

- `ENGINEERING-GLOSSARY.md` aprovado;
- `COMPILATION-PIPELINE.md` aprovado;
- `BUILDING-FROM-SOURCE.md` aprovado;
- `CODING-STANDARDS.md` aprovado;
- `RUST-STYLE-GUIDE.md` aprovado.

### 16.3 ADRs obrigatórios

- ADR de Rust como linguagem da implementação oficial aprovado;
- ADR de organização física do repositório aprovado;
- ADR de organização da implementação em workspace Cargo aprovado;
- ADR de separação entre frontend, middle-end e backend aprovado;
- ADR de política de dependências externas aprovado;
- ADR de estratégia inicial de testes aprovado.

### 16.4 Infraestrutura

- workspace Cargo criado em `capi-lang`;
- estrutura inicial de crates criada;
- `cargo fmt` configurado;
- `cargo clippy` configurado;
- execução de testes configurada;
- CI configurada;
- documentação Rust configurada;
- scripts de desenvolvimento definidos quando necessários;
- política inicial de dependências aplicada;
- versões mínimas de ferramentas definidas.

### 16.5 Implementação

- executável `capic` criado;
- crate do driver criado;
- crate de sessão de compilação criado;
- crate de diagnósticos criado;
- crate de infraestrutura de fontes criado;
- `capic --help` implementado;
- `capic --version` implementado;
- leitura inicial de argumentos implementada;
- inicialização de sessão implementada;
- tratamento básico de erros internos implementado.

### 16.6 Validação

- `cargo build --workspace` passa;
- `cargo fmt --all --check` passa;
- `cargo clippy --workspace --all-targets` passa;
- `cargo test --workspace` passa;
- teste de `capic --help` passa;
- teste de `capic --version` passa;
- teste de arquivo inexistente passa;
- CI passa.

### 16.7 Resultado demonstrável

Os comandos abaixo devem funcionar no ambiente definido:

```bash
capic --help
capic --version
cargo test --workspace
```

O resultado esperado do Stage 0 é:

```text
O projeto de implementação existe, compila e pode evoluir com segurança.
```

---

## 17. Bloqueadores

Uma entrega não pode ser marcada como concluída se existir:

- falha de build;
- teste obrigatório falhando;
- CI vermelha;
- dependência proibida;
- documento bloqueante ausente;
- ADR obrigatório ausente;
- divergência conhecida com a especificação;
- panic conhecido em erro esperado de usuário;
- limitação relevante não documentada;
- mudança de contrato público sem revisão.

Bloqueadores devem ser registrados no mecanismo de acompanhamento do projeto.

---

## 18. Exceções

Exceções à Definition of Done só são aceitáveis quando:

- o motivo está registrado;
- o impacto é conhecido;
- há plano de correção;
- a exceção não viola a especificação;
- a exceção não mascara falha de segurança, corretude ou integridade do build;
- a exceção foi aprovada por mantenedores.

Exceções não devem ser usadas para encerrar stages com critérios essenciais pendentes.

---

## 19. Critérios de aceitação deste documento

Este documento é considerado preenchido quando:

- define conclusão para tarefas, documentos, código, testes, CLI, crates, CI, ADRs e stages;
- inclui checklist específico do Stage 0;
- incorpora os comandos oficiais de validação;
- preserva a precedência da especificação;
- não redefine a ordem dos stages;
- não contradiz `TEST-STRATEGY.md`, `BUILD-SYSTEM.md` ou o Documento 28.

---

## 20. Síntese

Na Capi, "done" significa entregue, validado, revisável, rastreável e demonstrável.

Essa definição existe para impedir avanço aparente sem fundação real. Cada stage deve terminar com evidência objetiva de que a implementação, a documentação e os testes continuam alinhados.
