# Diagnostic Architecture

**Projeto:** Linguagem Capi  
**Documento:** DIAGNOSTIC-ARCHITECTURE  
**Status:** Aprovado  
**Stage:** Stage 1 — Fontes, diagnósticos e lexer  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define a arquitetura da infraestrutura de diagnósticos da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- como fases do compilador produzem diagnósticos;
- como diagnósticos são coletados;
- como spans são resolvidos para localizações;
- como o driver decide continuidade do pipeline;
- como o CLI apresenta diagnósticos;
- quais responsabilidades pertencem ao crate `capi-diagnostics`;
- quais responsabilidades pertencem a fases como lexer, parser e source loading;
- quais limites valem para o Stage 1.

---

## 2. Escopo

Este documento cobre:

- fluxo arquitetural de diagnósticos;
- fronteiras entre produtor, coletor e renderizador;
- integração com `SourceMap`;
- integração com sessão e driver;
- erro de usuário versus erro interno;
- diagnósticos do lexer e de carregamento de fonte;
- determinismo e testes;
- evolução para formatos externos.

Este documento não cobre:

- campos detalhados do modelo de dados;
- texto final de cada mensagem;
- catálogo completo de códigos;
- layout visual detalhado;
- saída JSON final;
- protocolo LSP;
- crash reporting;
- telemetria.

Esses temas pertencem a:

- `DIAGNOSTIC-DATA-MODEL.md`;
- `DIAGNOSTIC-STYLE-GUIDE.md`;
- `ERROR-CODE-POLICY.md`;
- `OUTPUT-FORMATS.md`;
- `INTERNAL-COMPILER-ERRORS.md`;
- `CRASH-REPORTING.md`;
- `TELEMETRY-POLICY.md`.

---

## 3. Princípios

A arquitetura de diagnósticos deve seguir estes princípios:

- fases produzem diagnósticos estruturados;
- fases não imprimem diretamente em `stdout` ou `stderr`;
- o CLI é responsável pela apresentação final ao usuário;
- a sessão ou driver agrega diagnósticos;
- diagnósticos ligados a código carregam spans quando possível;
- renderização depende de `SourceMap`, não de cópias de fonte dentro do diagnóstico;
- erro esperado de usuário não causa panic;
- erro interno é distinguível por severidade e categoria;
- saída de diagnóstico deve ser determinística e testável.

---

## 4. Visão Geral

Fluxo conceitual:

```text
Source loading / Lexer / Parser / fases posteriores
    ↓
Diagnostic
    ↓
DiagnosticBag ou collector
    ↓
Driver / Session
    ↓
Diagnostic renderer
    ↓
CLI / ferramenta / teste
```

O dado diagnóstico deve poder circular sem estar preso a um formato textual específico.

---

## 5. Componentes

| Componente | Responsabilidade |
| --- | --- |
| `capi-diagnostics` | Define dados, coletores, helpers e renderização inicial. |
| `capi-source` | Fornece `SourceMap`, spans, linha, coluna e trechos. |
| Fases do compilador | Produzem diagnósticos específicos da fase. |
| `capi-session` | Mantém ou referencia coletor compartilhado. |
| `capi-driver` | Coordena fases, agrega resultado e decide continuidade. |
| `capi-cli` | Renderiza ou encaminha diagnósticos ao usuário. |

`capi-diagnostics` não deve virar depósito de regras específicas de lexer, parser, type checker ou backend.

---

## 6. Produtores de Diagnóstico

Produtores são componentes que detectam problemas.

No Stage 1, produtores principais:

- carregamento de fonte;
- validação de UTF-8;
- lexer;
- driver para uso inválido ou arquivo inexistente.

Produtores futuros:

- parser;
- AST lowering;
- resolução de nomes;
- inferência e verificação de tipos;
- ownership e borrow checker;
- MIR validation;
- backends;
- linker/toolchain.

Cada produtor deve criar diagnósticos usando o modelo comum e não deve renderizá-los diretamente.

---

## 7. Coleta

Diagnósticos devem ser agregados em `DiagnosticBag` ou collector equivalente.

Responsabilidades do coletor:

- preservar ordem de emissão;
- permitir inserção por fases;
- permitir consulta por severidade;
- indicar erro bloqueador;
- indicar erro interno;
- permitir iteração determinística;
- permanecer independente de UI.

O coletor pode viver:

- dentro da saída de uma fase;
- dentro da sessão;
- como estrutura temporária agregada pelo driver.

No Stage 1, é aceitável que `LexOutput` carregue `Vec<Diagnostic>` ou `DiagnosticBag`, desde que o driver consiga agregá-los.

---

## 8. Integração com `SourceMap`

Diagnósticos não devem armazenar linha, coluna e trecho como fonte de verdade.

Para renderização, o renderizador deve receber:

- `Diagnostic`;
- `SourceMap`;
- opções de formato.

O renderizador resolve:

- caminho ou nome exibido;
- linha;
- coluna;
- trecho;
- marcação de span.

Essa separação garante que spans continuem compactos e que o diagnóstico permaneça útil para ferramentas.

---

## 9. Spans

Todo diagnóstico associado a código-fonte deve carregar span primário quando houver origem disponível.

Regras:

- lexer deve emitir spans para erros léxicos;
- parser deve usar spans dos tokens recebidos;
- fases semânticas devem preservar spans vindos da AST/HIR;
- diagnósticos de arquivo inexistente podem não ter span;
- diagnósticos de UTF-8 inválido podem usar localização parcial;
- erro interno pode ter span quando relacionado a código específico.

Spans sintéticos devem ser renderizados sem tentativa de recuperar trecho real.

---

## 10. Continuidade do Pipeline

O driver decide se fases posteriores podem executar.

Regras iniciais:

- erro de carregamento de fonte impede lexer;
- UTF-8 inválido impede lexer naquela fonte;
- erro léxico recuperável permite concluir lexing, mas impede sucesso final;
- erro léxico pode impedir parser, salvo modo explícito de recuperação futura;
- `InternalError` interrompe avanço seguro;
- warnings e notes não bloqueiam.

O critério de bloqueio deve ser consultável por `DiagnosticBag::has_errors()`.

---

## 11. Erro de Usuário e Erro Interno

Erro de usuário:

- representa entrada, configuração ou uso inválido;
- deve ser mostrado de forma compreensível;
- usa severidade `Error`;
- não deve expor detalhes internos desnecessários.

Erro interno:

- representa bug ou violação de invariante do compilador;
- usa severidade `InternalError`;
- deve ser distinguível para testes e relatórios;
- pode ser renderizado com orientação própria;
- não deve ser usado para entrada malformada comum.

Essa distinção é obrigatória desde o Stage 1.

---

## 12. Renderização

Renderização é a transformação de dados de diagnóstico em saída textual ou formato de ferramenta.

Responsabilidades do renderizador:

- ordenar ou preservar ordem conforme política;
- resolver spans via `SourceMap`;
- formatar severidade, código e mensagem;
- exibir labels, notes e sugestões;
- controlar uso de cor;
- normalizar caminhos para testes.

Responsabilidades que não pertencem ao renderizador:

- decidir se um erro existe;
- criar regra semântica;
- alterar diagnóstico;
- recuperar lexing ou parsing.

No Stage 1, a renderização textual inicial pode ser simples, mas deve consumir o modelo estruturado.

---

## 13. CLI

O CLI é responsável por apresentar diagnósticos ao usuário.

Ele deve:

- receber diagnósticos do driver;
- selecionar formato de saída;
- escrever em `stderr` ou destino apropriado;
- retornar código de saída adequado;
- evitar reconstruir diagnóstico a partir de strings soltas.

O CLI não deve:

- decidir regras de lexer;
- resolver spans manualmente sem infraestrutura comum;
- reclassificar erro interno como erro de usuário;
- imprimir diagnósticos diretamente de fases internas.

---

## 14. Driver e Sessão

O driver coordena fases e diagnósticos.

Responsabilidades:

- criar ou receber sessão;
- executar fases;
- agregar diagnósticos;
- interromper pipeline quando houver erro bloqueador;
- entregar diagnóstico ao CLI ou ferramenta;
- preservar ordem e rastreabilidade.

A sessão pode conter ou referenciar:

- `SourceMap`;
- `DiagnosticBag`;
- opções de emissão;
- configuração de formato.

A sessão não deve substituir estruturas específicas de fase nem esconder dependências reais.

---

## 15. Diagnósticos do Stage 1

O Stage 1 deve suportar, no mínimo:

| Origem | Diagnóstico |
| --- | --- |
| source loading | arquivo inexistente |
| source loading | falha de leitura |
| source loading | UTF-8 inválido |
| lexer | caractere inválido |
| lexer | literal numérico inválido |
| lexer | string não terminada |
| lexer | char inválido ou não terminado |
| lexer | escape inválido |
| lexer | comentário de bloco não terminado |
| lexer | erro interno de invariante, se ocorrer |

Diagnósticos léxicos devem carregar span primário sempre que a fonte for válida.

---

## 16. Códigos

A arquitetura deve permitir códigos estáveis.

No Stage 1:

- códigos podem ser opcionais durante a primeira implementação;
- categorias internas devem existir para não depender apenas de mensagem;
- antes da conclusão formal do stage, diagnósticos de lexer e fonte devem possuir código ou categoria estruturada.

O formato final e numeração pertencem a `ERROR-CODE-POLICY.md`.

---

## 17. Formatos de Saída

A arquitetura deve permitir múltiplos formatos:

- texto humano;
- snapshots de teste;
- JSON futuro;
- integração futura com editores.

No Stage 1, somente texto humano e formato estável para testes são obrigatórios.

O modelo de dados não deve depender de ANSI, terminal, largura visual ou locale.

---

## 18. Determinismo

Diagnósticos devem ser reproduzíveis.

Regras:

- mesma entrada gera mesma sequência de diagnósticos;
- ordem segue descoberta determinística;
- mensagens não dependem de ordem de hash map;
- caminhos em testes devem ser normalizados;
- renderização deve ter modo sem cor para snapshots;
- spans e localizações devem ser derivados do `SourceMap`.

---

## 19. Testabilidade

A infraestrutura deve ser testável em camadas:

- teste do modelo de dados sem renderização;
- teste do coletor;
- teste de renderização com `SourceMap` controlado;
- teste de diagnóstico léxico;
- teste de CLI ou driver para códigos de saída;
- snapshots ou UI tests para saída textual.

Testes não devem depender de caminhos absolutos, idioma do ambiente ou cor de terminal.

---

## 20. Dependências

Dependências permitidas para `capi-diagnostics`:

- `capi-source`, quando spans concretos e resolução de localização forem necessários;
- `capi-common`, para tipos pequenos compartilhados;
- biblioteca padrão Rust.

Dependências proibidas:

- `capi-cli`;
- `capi-driver`;
- lexer;
- parser;
- AST/HIR/MIR;
- backends;
- linker.

Se a dependência de `capi-source` criar ciclo, a arquitetura deve mover tipos mínimos compartilhados para um crate apropriado ou separar renderização de dados.

---

## 21. API Arquitetural Inicial

APIs conceituais:

```rust
pub struct DiagnosticBag { ... }
pub struct DiagnosticRenderer { ... }

impl DiagnosticBag {
    pub fn push(&mut self, diagnostic: Diagnostic);
    pub fn has_errors(&self) -> bool;
    pub fn has_internal_errors(&self) -> bool;
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic>;
}

impl DiagnosticRenderer {
    pub fn render(&self, diagnostic: &Diagnostic, sources: &SourceMap) -> RenderedDiagnostic;
}
```

No Stage 1, `RenderedDiagnostic` pode ser `String` ou escrita em `fmt::Write`/`io::Write`, desde que a arquitetura não prenda o modelo de dados ao CLI.

---

## 22. Fluxo de `capic --emit tokens`

Fluxo esperado para o resultado demonstrável do Stage 1:

```text
1. CLI normaliza argumentos.
2. Driver cria sessão.
3. SourceMap carrega arquivo.
4. Falhas de fonte viram diagnósticos.
5. Lexer produz tokens e diagnósticos.
6. Driver agrega diagnósticos.
7. CLI renderiza diagnósticos.
8. Se solicitado e possível, CLI ou driver emite dump de tokens.
9. Código de saída reflete presença de erro bloqueador.
```

Esse fluxo deve permitir testar separadamente diagnóstico e dump.

---

## 23. Limitações do Stage 1

No Stage 1, não é obrigatório implementar:

- saída JSON completa;
- LSP diagnostics;
- catálogo definitivo de códigos;
- sugestões automáticas em todos os casos;
- deduplicação global;
- agrupamento por arquivo;
- limites finais de diagnóstico;
- localização visual perfeita para Unicode complexo;
- crash reporting integrado.

Essas limitações não devem impedir diagnósticos estruturados, spans precisos e renderização textual inicial.

---

## 24. Testes Obrigatórios

Os testes de arquitetura de diagnósticos devem cobrir:

- fase produz diagnóstico sem imprimir;
- `DiagnosticBag` preserva ordem;
- `has_errors` bloqueia pipeline;
- `InternalError` é distinguível;
- diagnóstico com span é resolvido via `SourceMap`;
- diagnóstico sem span renderiza sem trecho;
- diagnóstico léxico aponta linha e coluna;
- erro de arquivo inexistente não exige span;
- lexer recuperável pode produzir tokens e erro;
- driver interrompe após erro de fonte;
- renderização textual é determinística;
- modo sem cor para snapshots;
- caminhos de teste são normalizados.

Testes específicos de estilo e formatos pertencem aos documentos próprios, mas devem usar esta arquitetura.

---

## 25. Critérios de Aprovação

Este documento pode ser considerado aprovado quando:

- produtores, coletores e renderizadores estiverem separados;
- integração com `SourceMap`, driver, sessão, CLI e lexer estiver definida;
- regras de continuidade do pipeline estiverem claras;
- erro de usuário e erro interno estiverem separados;
- dependências permitidas e proibidas estiverem explícitas;
- limitações do Stage 1 estiverem documentadas;
- os testes obrigatórios forem rastreáveis às entregas do Documento 28.

Após aprovação, `capi-diagnostics`, `capi-source`, `capi-driver`, `capi-cli` e o lexer devem ser integrados conforme esta arquitetura durante o Stage 1.
