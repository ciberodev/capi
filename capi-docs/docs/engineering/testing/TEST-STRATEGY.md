# Test Strategy

**Projeto:** Linguagem Capi  
**Documento:** TEST-STRATEGY  
**Status:** Proposto  
**Stage:** Stage 0 — Fundação do projeto  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define a estratégia oficial de testes da implementação da linguagem Capi.

Seu objetivo é estabelecer:

- quais tipos de teste devem existir;
- onde os testes devem viver;
- quais comandos validam o workspace;
- como regressões devem ser registradas;
- quais critérios mínimos bloqueiam integração;
- como a suíte evolui entre os estágios da implementação oficial.

Este documento não redefine semântica da linguagem. Ele traduz a especificação e a arquitetura do compilador em práticas verificáveis de engenharia.

---

## 2. Escopo

A estratégia cobre testes para:

- crates internos do compilador;
- CLI `capic`;
- diagnósticos;
- parsing e análise semântica;
- checagem de tipos;
- modelos de memória e concorrência;
- MIR;
- geração de código;
- execução de programas Capi;
- biblioteca padrão;
- bootstrap;
- conformidade com a especificação.

Durante o Stage 0, a suíte é propositalmente pequena. Ela deve validar que a fundação do workspace está correta antes que funcionalidades reais da linguagem sejam implementadas.

---

## 3. Princípios

Os testes da implementação oficial devem seguir os princípios abaixo.

### 3.1 Testes fazem parte da especificação executável

Sempre que uma regra da linguagem for implementada, deve existir pelo menos um teste que demonstre o comportamento esperado.

Quando uma decisão semântica for ambígua, a especificação normativa deve ser ajustada antes do teste ser tratado como fonte de verdade.

### 3.2 Falhas devem ser locais e compreensíveis

Um teste deve falhar apontando uma causa específica.

Falhas amplas, que exigem leitura extensa do compilador para descobrir o problema, devem ser evitadas quando puderem ser divididas em testes menores.

### 3.3 Testes devem ser determinísticos

A suíte não deve depender de:

- ordem instável de arquivos;
- relógio de parede;
- aleatoriedade sem seed fixa;
- localização do sistema;
- mensagens externas ao workspace;
- estado global persistente entre execuções.

Quando a plataforma afetar a saída, o teste deve normalizar caminhos, separadores e detalhes ambientais.

### 3.4 Regressões devem permanecer na suíte

Todo bug corrigido deve ganhar um teste de regressão.

O teste deve ser adicionado antes ou junto da correção, e deve continuar existindo enquanto a regra coberta permanecer válida.

### 3.5 Testes não substituem arquitetura

Testes devem validar contratos, mas não compensar acoplamento indevido entre crates.

Se um teste exige atravessar camadas internas demais para observar um comportamento, isso é indício de que a API pública da camada deve ser revista.

---

## 4. Comandos Oficiais

A validação padrão do workspace é:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets
cargo test --workspace
cargo build --workspace
```

Quando o crate `capi-cli` e o binário `capic` existirem, a validação mínima da CLI inclui:

```bash
cargo run -p capi-cli -- --help
cargo run -p capi-cli -- --version
```

O CI deve executar os mesmos comandos. Nenhuma mudança deve ser considerada pronta se quebrar formatação, lint, build ou testes.

---

## 5. Camadas de Teste

A suíte oficial é organizada em camadas. Cada camada cobre um nível diferente de contrato.

---

### 5.1 Testes Unitários

Testes unitários validam comportamento local de uma crate ou módulo.

Devem ser usados para:

- estruturas de dados internas;
- funções puras;
- normalização de caminhos;
- montagem de spans;
- conversões entre representações internas;
- pequenas regras de validação.

Eles devem viver preferencialmente junto ao código testado, em módulos `#[cfg(test)]`.

Testes unitários não devem depender de detalhes de CLI, filesystem real ou ordem global de execução.

---

### 5.2 Testes de Integração entre Crates

Testes de integração validam contratos públicos entre crates do workspace.

Devem ser usados para:

- fluxo `source -> lexer -> parser`;
- fluxo `AST -> HIR`;
- fluxo `HIR -> type checking`;
- preparação de MIR;
- contratos entre `capi-driver` e crates internas.

Eles devem viver em diretórios `tests/` das crates responsáveis por expor o contrato testado.

---

### 5.3 Testes de CLI

Testes de CLI validam o comportamento observável do binário `capic`.

Devem cobrir:

- `capic --help`;
- `capic --version`;
- arquivo inexistente;
- argumentos inválidos;
- códigos de saída;
- formato básico de erro;
- comandos e flags oficiais conforme forem criados.

Esses testes não devem depender da linguagem estar completa. Desde o Stage 0, a CLI precisa ter comportamento previsível mesmo para entradas inválidas.

---

### 5.4 Testes de Diagnóstico

Testes de diagnóstico validam erros e mensagens emitidas pelo compilador.

Devem cobrir:

- código de erro, quando existir;
- severidade;
- arquivo;
- linha e coluna;
- span principal;
- labels auxiliares;
- sugestões, quando houver;
- estabilidade da mensagem pública.

Mensagens de diagnóstico são parte da experiência oficial da linguagem. Mudanças nelas devem ser intencionais.

---

### 5.5 Testes `compile-pass`

Testes `compile-pass` validam programas Capi que devem ser aceitos pelo compilador.

Eles devem ser usados quando o compilador já tiver suporte suficiente para aceitar uma unidade de código completa ou parcial.

Esses testes não precisam executar o programa. Seu contrato é apenas: o compilador aceita o código.

---

### 5.6 Testes `compile-fail`

Testes `compile-fail` validam programas Capi que devem ser rejeitados.

Devem cobrir:

- erros léxicos;
- erros sintáticos;
- violações semânticas;
- erros de tipo;
- violações de ownership, lifetimes, effects, domínios e concorrência;
- uso incorreto de recursos da biblioteca padrão.

Cada teste deve declarar claramente qual erro principal espera.

---

### 5.7 Testes `run-pass`

Testes `run-pass` validam programas Capi que devem compilar e executar com sucesso.

Eles entram quando a implementação possuir backend e runtime suficientes.

Devem cobrir:

- valor de saída;
- stdout e stderr quando relevante;
- código de saída;
- comportamento observável do runtime;
- interações com a biblioteca padrão.

---

### 5.8 Testes de Snapshot

Snapshots devem ser usados para saídas textuais extensas ou estruturadas, especialmente:

- diagnósticos;
- dumps de AST;
- dumps de HIR;
- dumps de MIR;
- saída de comandos internos de inspeção;
- golden files de codegen textual, quando aplicável.

Snapshots não devem ser atualizados automaticamente sem revisão humana.

Uma mudança em snapshot deve responder a uma pergunta simples: a saída nova está correta segundo a especificação?

---

### 5.9 Testes de Codegen

Testes de codegen validam que a representação final gerada preserva a semântica do programa.

Eles devem entrar quando houver backend suficiente.

Podem cobrir:

- validade do artefato gerado;
- padrões essenciais de IR;
- chamadas corretas ao runtime;
- layout de dados;
- ABI interna;
- integração com bibliotecas externas permitidas.

Testes de codegen não devem depender de detalhes acidentais de otimização, salvo quando o contrato testado for explicitamente sobre otimização.

---

### 5.10 Testes de Conformidade

Testes de conformidade validam a implementação oficial contra a especificação da linguagem.

Eles devem ser organizados por área da especificação:

- léxico;
- sintaxe;
- semântica;
- tipos;
- memória;
- concorrência;
- módulos;
- FFI;
- runtime;
- biblioteca padrão.

Esses testes serão a base para validar futuras implementações alternativas da linguagem.

---

### 5.11 Testes Diferenciais

Testes diferenciais comparam duas rotas de implementação do mesmo comportamento.

Exemplos futuros:

- backend próprio versus backend LLVM;
- interpretador de MIR versus binário gerado;
- compilador bootstrap versus compilador anterior;
- execução com e sem otimizações.

Eles devem ser introduzidos apenas quando existirem duas rotas suficientemente maduras para comparação.

---

### 5.12 Fuzzing

Fuzzing deve ser usado para encontrar falhas de robustez em fronteiras críticas:

- lexer;
- parser;
- lowering para HIR;
- serialização de estruturas internas;
- validadores de MIR;
- fronteiras de FFI, quando aplicável.

O contrato mínimo de fuzzing é: entrada inválida pode gerar diagnóstico, mas não pode causar panic não controlado, corrupção interna ou comportamento indefinido no compilador.

---

### 5.13 Testes de Performance

Testes de performance não fazem parte da validação obrigatória de todo commit no Stage 0.

Eles serão introduzidos quando houver massa crítica de código compilável.

Devem medir:

- tempo de compilação;
- uso de memória do compilador;
- tempo de execução de programas gerados;
- impacto de otimizações;
- regressões em workloads representativos.

Benchmarks devem ser estáveis, versionados e separados da suíte funcional padrão.

---

## 6. Organização Física

A organização inicial recomendada para testes é:

```text
capi-lang/
├── crates/
│   ├── capi-cli/
│   │   └── tests/
│   ├── capi-driver/
│   │   └── tests/
│   ├── capi-diagnostics/
│   │   └── tests/
│   ├── capi-lexer/
│   │   └── tests/
│   ├── capi-parser/
│   │   └── tests/
│   └── ...
└── tests/
    ├── cli/
    ├── compile-pass/
    ├── compile-fail/
    ├── run-pass/
    ├── ui/
    ├── conformance/
    ├── differential/
    ├── performance/
    └── fixtures/
```

Regras:

- testes próximos ao código validam comportamento local;
- testes na raiz validam comportamento do sistema;
- fixtures compartilhadas devem ficar em `capi-lang/tests/fixtures`;
- snapshots devem ficar próximos da suíte que os consome;
- arquivos temporários devem ser criados em diretórios temporários por teste.

---

## 7. Suíte Mínima do Stage 0

No Stage 0, a suíte oficial deve validar a fundação do projeto.

Ela deve incluir:

- build completo do workspace;
- formatação obrigatória;
- lint obrigatório;
- execução de `cargo test --workspace`;
- teste de `capic --help`;
- teste de `capic --version`;
- teste de erro para arquivo inexistente;
- teste de inicialização do driver;
- teste básico de emissão de diagnóstico;
- teste básico de localização de arquivo e span;
- validação de que crates públicas compilam sem dependências proibidas.

Essa suíte não deve tentar validar a linguagem inteira. Seu papel é garantir que o projeto nasce com ciclo de feedback confiável.

---

## 8. Convenções de Nome

Nomes de testes devem descrever comportamento, não implementação.

Exemplos aceitáveis:

```rust
accepts_empty_source_file
reports_missing_input_file
emits_span_for_unterminated_string
rejects_duplicate_module_name
```

Exemplos a evitar:

```rust
test_lexer_1
parser_hack_case
works
bugfix
```

Arquivos de teste devem usar nomes estáveis e descritivos:

```text
missing-input-file.rs
unterminated-string.capi
duplicate-module-name.capi
basic-function-call.capi
```

---

## 9. Fixtures

Fixtures são entradas versionadas usadas por testes.

Devem seguir estas regras:

- cada fixture deve existir por uma razão clara;
- fixtures grandes devem ter comentário ou nome que explique sua finalidade;
- uma fixture não deve ser reutilizada para muitos testes sem relação;
- fixtures inválidas devem indicar no nome o tipo de erro esperado;
- paths absolutos não devem aparecer em fixtures ou snapshots.

Fixtures não são exemplos de documentação. Elas podem ser mínimas e artificiais se isso tornar o teste mais preciso.

---

## 10. Snapshots e Golden Files

Snapshots e golden files devem ser tratados como saída pública testada.

Ao alterar um snapshot, o autor da mudança deve verificar:

- se a nova saída corresponde à especificação;
- se a mudança não remove informação útil;
- se spans e labels continuam corretos;
- se a saída não inclui detalhes instáveis do ambiente;
- se o teste ainda cobre a intenção original.

Snapshots devem ser revisados como código.

---

## 11. Diagnósticos

Diagnósticos exigem atenção especial porque são o principal contrato de erro do compilador.

Testes de diagnóstico devem preferir validar estrutura e conteúdo essencial em vez de depender de detalhes cosméticos.

O teste deve ser estrito para:

- erro principal;
- severidade;
- posição;
- range;
- relação com a regra violada.

O teste pode ser menos estrito para:

- cor;
- largura terminal;
- detalhes de renderização não normativos;
- ordenação de notas auxiliares quando a ordem não for semanticamente relevante.

---

## 12. Códigos de Saída

Testes de CLI devem validar códigos de saída.

Contrato inicial:

- sucesso deve retornar `0`;
- erro de uso deve retornar código diferente de `0`;
- erro de arquivo inexistente deve retornar código diferente de `0`;
- panic do compilador nunca deve ser usado como mecanismo normal de erro.

Os códigos específicos podem ser refinados em documento próprio quando a CLI amadurecer.

---

## 13. Isolamento

Cada teste deve poder rodar isoladamente e em paralelo.

Testes não devem:

- depender da ordem de execução;
- escrever em diretórios compartilhados sem isolamento;
- modificar variáveis globais sem restauração;
- depender de arquivos gerados por outro teste;
- assumir estado prévio do workspace.

Quando necessário, testes devem criar diretórios temporários próprios.

---

## 14. Política de Flaky Tests

Teste instável é falha do projeto, não ruído aceitável.

Um teste identificado como instável deve ser:

- corrigido;
- isolado;
- reescrito;
- ou removido temporariamente com justificativa explícita.

Não é aceitável manter teste instável como requisito bloqueante sem investigação.

---

## 15. Regressões

Todo bug confirmado deve gerar um teste de regressão.

O teste deve:

- falhar antes da correção;
- passar depois da correção;
- ter nome ligado ao comportamento, não ao número da issue;
- incluir referência à issue apenas como comentário auxiliar, quando útil.

Regressões em regras da linguagem devem ser classificadas na suíte correspondente, por exemplo `compile-fail`, `compile-pass` ou `run-pass`.

---

## 16. Evolução por Estágio

A suíte deve crescer junto com os estágios da implementação.

### Stage 0 — Fundação

Foco:

- workspace;
- CI;
- CLI mínima;
- diagnósticos básicos;
- estrutura de testes.

### Stage 1 — Lexer

Foco:

- tokens válidos;
- tokens inválidos;
- comentários;
- strings;
- números;
- spans;
- recuperação de erro léxico.

### Stage 2 — Parser

Foco:

- sintaxe válida;
- sintaxe inválida;
- precedência;
- associatividade;
- recuperação de erro;
- snapshots de AST quando úteis.

### Stage 3 — Semântica Inicial

Foco:

- resolução de nomes;
- módulos;
- escopos;
- lowering para HIR;
- diagnósticos semânticos básicos.

### Stage 4 — Tipos

Foco:

- inferência;
- unificação;
- generics;
- traits ou contratos equivalentes;
- erros de tipo;
- mensagens de tipo estáveis.

### Stages 5 a 7 — Memória, Effects e Concorrência

Foco:

- ownership;
- borrowing;
- lifetimes;
- regions;
- effects;
- handlers;
- isolamento por domínio;
- regras de concorrência.

### Stage 8 — MIR

Foco:

- construção de MIR;
- validação de MIR;
- invariantes internas;
- dumps estáveis;
- interpretação ou execução intermediária quando existir.

### Stages 9 a 11 — Runtime, Biblioteca Padrão e Backend

Foco:

- execução de programas;
- integração com runtime;
- biblioteca padrão mínima;
- codegen;
- `run-pass`;
- erros de execução definidos.

### Stage 15 — Conformidade

Foco:

- suíte normativa;
- cobertura ampla da especificação;
- fuzzing;
- regressões históricas;
- validação de compatibilidade.

### Stages 16 a 18 — Bootstrap e Backend Alternativo

Foco:

- compilador compilando partes de si mesmo;
- comparação entre compiladores;
- comparação entre backends;
- testes diferenciais;
- estabilidade de artefatos.

---

## 17. Cobertura

Cobertura é métrica auxiliar, não objetivo primário.

O projeto deve priorizar:

- regras críticas cobertas;
- diagnósticos importantes cobertos;
- regressões preservadas;
- contratos públicos testados;
- invariantes internas verificadas.

Uma porcentagem alta de cobertura não compensa ausência de testes para regras centrais da linguagem.

---

## 18. Integração com CI

O CI deve bloquear mudanças que quebrem:

- formatação;
- clippy;
- build;
- testes;
- validação mínima da CLI;
- regras de dependência automatizadas quando existirem.

O CI deve executar comandos equivalentes aos comandos oficiais deste documento.

Suítes caras, como fuzzing prolongado e benchmarks, podem rodar em pipelines separados.

---

## 19. Adição de Novos Testes

Ao adicionar funcionalidade, a mudança deve incluir testes no menor nível capaz de validar o contrato.

Critério prático:

- regra local: teste unitário;
- contrato entre crates: teste de integração;
- comportamento visível ao usuário: teste de CLI ou UI;
- programa aceito: `compile-pass`;
- programa rejeitado: `compile-fail`;
- programa executável: `run-pass`;
- bug corrigido: teste de regressão.

Se uma funcionalidade não puder ser testada sem acoplamento excessivo, a API de teste ou a fronteira arquitetural deve ser revista.

---

## 20. Remoção de Testes

Um teste só deve ser removido quando:

- a regra testada deixou de existir na especificação;
- o teste era duplicado sem valor adicional;
- o teste estava incorreto;
- a suíte foi reorganizada e a cobertura foi preservada em outro lugar.

Remover teste por conveniência não é aceitável.

---

## 21. Critérios de Aceitação do Documento

Este documento é considerado preenchido quando:

- define as camadas oficiais de teste;
- define a suíte mínima do Stage 0;
- estabelece comandos de validação;
- descreve organização física;
- define política para fixtures, snapshots e regressões;
- explica como a suíte evolui por estágio;
- não contradiz a especificação nem os documentos de arquitetura.

---

## 22. Síntese

A estratégia de testes da Capi deve transformar a especificação em comportamento verificável.

No Stage 0, o objetivo é criar uma fundação pequena, rápida e confiável. Nos estágios seguintes, a suíte deve crescer junto com o compilador até se tornar uma suíte de conformidade capaz de proteger a linguagem, a implementação oficial e futuras implementações alternativas.
