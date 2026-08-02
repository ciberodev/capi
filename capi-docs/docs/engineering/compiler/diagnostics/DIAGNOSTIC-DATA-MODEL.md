# Diagnostic Data Model

**Projeto:** Linguagem Capi  
**Documento:** DIAGNOSTIC-DATA-MODEL  
**Status:** Aprovado  
**Stage:** Stage 1 — Fontes, diagnósticos e lexer  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o modelo de dados dos diagnósticos estruturados da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- a estrutura conceitual de `Diagnostic`;
- severidades;
- códigos de diagnóstico;
- mensagens;
- spans primários e secundários;
- labels;
- notas;
- sugestões;
- coleta de diagnósticos;
- distinção entre erro de usuário e erro interno;
- requisitos mínimos para diagnósticos léxicos do Stage 1.

Este documento define dados. Renderização textual, estilo de mensagens, cores, formatos externos e integração com ferramentas pertencem a documentos próprios.

---

## 2. Escopo

Este documento cobre:

- diagnósticos emitidos por fases do compilador;
- estrutura comum usada por lexer, parser e fases posteriores;
- campos obrigatórios e opcionais;
- invariantes de dados;
- agregação em `DiagnosticBag`;
- relação com `Span` e `SourceMap`;
- critérios para erro bloqueador;
- testes obrigatórios do modelo.

Este documento não cobre:

- layout visual de mensagens;
- saída JSON completa;
- códigos específicos de todas as fases;
- texto final de cada erro;
- recuperação do parser;
- política completa de warnings;
- telemetria;
- crash reports.

Esses temas pertencem a:

- `DIAGNOSTIC-ARCHITECTURE.md`;
- `DIAGNOSTIC-STYLE-GUIDE.md`;
- `ERROR-CODE-POLICY.md`;
- `OUTPUT-FORMATS.md`;
- `INTERNAL-COMPILER-ERRORS.md`;
- `LEXER-IMPLEMENTATION.md`;
- `PARSER-RECOVERY.md`.

---

## 3. Princípios

O modelo de diagnósticos deve seguir estes princípios:

- diagnósticos são dados estruturados, não strings soltas;
- toda fase reporta problemas por meio da infraestrutura comum;
- erro esperado de usuário não deve causar panic;
- erro interno deve ser distinguível de erro de usuário;
- diagnósticos associados a código devem carregar span quando possível;
- a mensagem humana não deve ser a única fonte de classificação;
- o modelo deve ser estável o suficiente para testes;
- renderização deve ser separada da construção dos dados.

---

## 4. Entidades

| Entidade | Responsabilidade |
| --- | --- |
| `Diagnostic` | Unidade estruturada de problema ou informação. |
| `DiagnosticSeverity` | Gravidade do diagnóstico. |
| `DiagnosticCode` | Identificador estável da categoria. |
| `DiagnosticMessage` | Mensagem principal para humanos. |
| `DiagnosticLabel` | Associação entre span e texto contextual. |
| `DiagnosticNote` | Informação adicional sem necessariamente apontar span. |
| `DiagnosticSuggestion` | Alteração sugerida, quando aplicável. |
| `DiagnosticBag` | Coleção ordenada de diagnósticos. |

Os nomes finais podem variar. O contrato obrigatório é a presença dessas responsabilidades.

---

## 5. `Diagnostic`

Contrato conceitual:

```rust
pub struct Diagnostic {
    severity: DiagnosticSeverity,
    code: Option<DiagnosticCode>,
    message: DiagnosticMessage,
    primary_span: Option<Span>,
    labels: Vec<DiagnosticLabel>,
    notes: Vec<DiagnosticNote>,
    suggestions: Vec<DiagnosticSuggestion>,
}
```

No Stage 1, a implementação existente pode começar com severidade e mensagem. Antes da conclusão formal do stage, diagnósticos léxicos e de fonte devem suportar span e código ou categoria estruturada.

---

## 6. Severidade

`DiagnosticSeverity` classifica o impacto do diagnóstico.

Contrato inicial:

```rust
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Note,
    Help,
    InternalError,
}
```

Significado:

| Severidade | Uso |
| --- | --- |
| `Error` | Erro de usuário ou entrada inválida que impede conclusão normal. |
| `Warning` | Problema não bloqueador. |
| `Note` | Informação contextual adicional. |
| `Help` | Orientação ou sugestão textual. |
| `InternalError` | Violação de invariante interna do compilador. |

O Stage 1 precisa suportar pelo menos `Error`, `Warning`, `Note` e `InternalError`. `Help` pode ser modelado como nota ou severidade própria, desde que sugestões permaneçam estruturáveis.

---

## 7. Erro de Usuário e Erro Interno

Erro de usuário é causado por entrada, configuração ou uso inválido.

Exemplos:

- arquivo inexistente;
- UTF-8 inválido;
- caractere léxico inválido;
- string não terminada;
- comentário de bloco não terminado.

Erro interno é causado por violação de invariante da implementação.

Exemplos:

- token emitido sem span real ou sintético válido;
- span inválido criado por bug interno;
- MIR inválida após fase que deveria validá-la;
- estado impossível em uma fase.

Regras:

- erro de usuário usa `Error`;
- erro interno usa `InternalError`;
- erro interno deve ser rastreável e não deve ser misturado com erro de sintaxe comum;
- panic não controlado não substitui diagnóstico interno estruturado quando houver caminho de recuperação.

---

## 8. Código de Diagnóstico

`DiagnosticCode` identifica uma categoria estável.

Contrato conceitual:

```rust
pub struct DiagnosticCode {
    namespace: DiagnosticNamespace,
    number: u32,
}
```

Exemplos de namespaces:

| Namespace | Fase |
| --- | --- |
| `source` | leitura, encoding, source map |
| `lex` | lexer |
| `parse` | parser |
| `name` | resolução de nomes |
| `type` | tipos |
| `internal` | erros internos |

Formato textual sugerido:

```text
E0001
LEX0001
SRC0001
ICE0001
```

O formato final pertence a `ERROR-CODE-POLICY.md`. No Stage 1, códigos podem começar como enum interno por categoria, desde que não dependam apenas da mensagem.

---

## 9. Mensagem

`DiagnosticMessage` é a mensagem principal para humanos.

Regras:

- deve ser clara e curta;
- não deve carregar localização embutida;
- não deve ser usada como identificador estável;
- deve evitar detalhes de implementação quando o problema for erro de usuário;
- pode ser construída por helpers para manter consistência.

Contrato conceitual:

```rust
pub struct DiagnosticMessage(String);
```

No Stage 1, `String` direta é aceitável se o restante do modelo não depender dela para classificação.

---

## 10. Spans

Diagnóstico associado a código-fonte deve carregar `primary_span` quando houver origem disponível.

Regras:

- `primary_span` aponta a região principal do problema;
- labels secundárias podem apontar contexto adicional;
- diagnósticos de arquivo inexistente podem não ter span;
- diagnósticos de UTF-8 inválido podem ter arquivo e offset aproximado, mesmo sem span completo;
- diagnóstico interno pode ter span quando a falha estiver ligada a código específico.

Spans são definidos por `SPANS-AND-LOCATIONS.md` e resolvidos por `SourceMap`.

---

## 11. Labels

`DiagnosticLabel` associa texto contextual a um span.

Contrato conceitual:

```rust
pub struct DiagnosticLabel {
    span: Span,
    message: Option<DiagnosticMessage>,
    style: LabelStyle,
}

pub enum LabelStyle {
    Primary,
    Secondary,
}
```

Regras:

- deve haver no máximo uma label primária para o mesmo papel principal;
- labels secundárias explicam contexto;
- labels não substituem a mensagem principal;
- labels devem referenciar spans válidos ou sintéticos resolvíveis conforme política.

No Stage 1, é aceitável modelar `primary_span` separadamente e adiar múltiplas labels, desde que a estrutura permita evolução.

---

## 12. Notas

`DiagnosticNote` carrega informação adicional.

Contrato conceitual:

```rust
pub struct DiagnosticNote {
    message: DiagnosticMessage,
}
```

Notas são úteis para:

- explicar causa;
- apontar limitação do estágio atual;
- informar consequência;
- sugerir onde procurar documentação.

Notas não precisam ter span.

---

## 13. Sugestões

`DiagnosticSuggestion` representa uma correção sugerida.

Contrato conceitual:

```rust
pub struct DiagnosticSuggestion {
    span: Span,
    replacement: String,
    applicability: Applicability,
    message: DiagnosticMessage,
}

pub enum Applicability {
    MachineApplicable,
    MaybeIncorrect,
    HasPlaceholders,
    Unspecified,
}
```

No Stage 1, sugestões podem ser opcionais. O modelo deve evitar que sugestões sejam representadas apenas como texto livre quando houver substituição clara.

---

## 14. Diagnósticos de Fonte

Falhas de fonte devem ser representáveis pelo modelo.

Categorias iniciais:

| Categoria | Span | Exemplo |
| --- | --- | --- |
| arquivo inexistente | não | caminho informado não existe |
| permissão negada | não | leitura negada pelo SO |
| caminho inválido | não | entrada não resolvível |
| UTF-8 inválido | parcial/opcional | bytes inválidos |
| limite excedido | não ou arquivo inteiro | arquivo grande demais |

Esses diagnósticos devem preservar caminho ou nome de exibição em dado estruturado ou mensagem, conforme a arquitetura final.

---

## 15. Diagnósticos Léxicos

O Stage 1 deve suportar diagnósticos léxicos.

Categorias iniciais:

| Categoria | Span esperado |
| --- | --- |
| caractere inválido | caractere completo |
| literal numérico inválido | sequência problemática |
| string não terminada | início da string até ponto de recuperação |
| char não terminado | início do char até ponto de recuperação |
| char vazio | delimitadores ou região vazia |
| char com múltiplos caracteres | conteúdo inválido |
| escape inválido | sequência de escape |
| comentário de bloco não terminado | início do comentário até EOF |

Todo diagnóstico léxico deve possuir span primário sempre que o lexer recebeu uma fonte válida.

---

## 16. `DiagnosticBag`

`DiagnosticBag` é uma coleção ordenada de diagnósticos.

Contrato conceitual:

```rust
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
}
```

Responsabilidades:

- preservar ordem de emissão;
- permitir inserção;
- permitir iteração;
- indicar se há erros bloqueadores;
- indicar se há erro interno;
- permitir extensão por fase sem acoplamento ao CLI.

APIs mínimas:

```rust
impl DiagnosticBag {
    pub fn push(&mut self, diagnostic: Diagnostic);
    pub fn has_errors(&self) -> bool;
    pub fn has_internal_errors(&self) -> bool;
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
```

---

## 17. Erro Bloqueador

Um diagnóstico é bloqueador quando impede avanço seguro da fase atual ou de fases posteriores.

No Stage 1:

- `Error` é bloqueador para conclusão bem-sucedida da compilação;
- `InternalError` é sempre bloqueador;
- `Warning`, `Note` e `Help` não são bloqueadores;
- o lexer pode continuar após `Error` recuperável, mas o resultado final da invocação deve indicar falha.

`DiagnosticBag::has_errors()` deve considerar `Error` e `InternalError`.

---

## 18. Ordem e Determinismo

Diagnósticos devem ser determinísticos.

Regras:

- preservar ordem de descoberta quando uma fase percorre o arquivo sequencialmente;
- evitar dependência de ordem não determinística de mapas hash;
- manter mensagens e códigos estáveis em testes;
- não incluir caminhos absolutos em snapshots salvo quando explicitamente normalizados;
- não depender de idioma ou configuração de ambiente para estrutura do diagnóstico.

---

## 19. Relação com Renderização

Este modelo não define a saída visual.

O renderizador deve receber:

- `Diagnostic`;
- `SourceMap`;
- opções de formato;
- destino de saída.

O renderizador pode derivar:

- caminho exibido;
- linha e coluna;
- trecho de fonte;
- marcação visual.

O `Diagnostic` não deve armazenar texto renderizado final como sua única representação.

---

## 20. Relação com Ferramentas

O modelo deve permitir consumo por ferramentas futuras.

Requisitos:

- campos estruturados para severidade e código;
- spans preservados;
- sugestões modeláveis;
- ausência de dependência obrigatória de ANSI/color;
- possibilidade futura de saída JSON.

O Stage 1 não precisa implementar formato externo completo, mas não deve impedir essa evolução.

---

## 21. API Pública Inicial

API mínima compatível com o estado atual:

```rust
pub enum Severity {
    Error,
    InternalError,
    Warning,
    Note,
}

pub struct Diagnostic { ... }

impl Diagnostic {
    pub fn new(severity: Severity, message: impl Into<String>) -> Self;
    pub fn error(message: impl Into<String>) -> Self;
    pub fn internal_error(message: impl Into<String>) -> Self;
    pub fn severity(&self) -> Severity;
    pub fn message(&self) -> &str;
}
```

Extensões necessárias para o Stage 1:

```rust
impl Diagnostic {
    pub fn with_code(self, code: DiagnosticCode) -> Self;
    pub fn with_primary_span(self, span: Span) -> Self;
    pub fn with_label(self, label: DiagnosticLabel) -> Self;
    pub fn with_note(self, note: impl Into<String>) -> Self;
}
```

Os nomes finais podem variar. O contrato obrigatório é a capacidade de carregar severidade, mensagem, código e span.

---

## 22. Builders

Builders são permitidos quando a construção direta ficar verbosa.

Contrato conceitual:

```rust
Diagnostic::error("unterminated string literal")
    .with_code(DiagnosticCode::lex(1))
    .with_primary_span(span)
    .with_note("string literals must be closed with a double quote");
```

Builders não devem esconder campos obrigatórios nem criar diagnósticos sem severidade ou mensagem.

---

## 23. Invariantes

A implementação deve garantir:

- todo diagnóstico possui severidade;
- todo diagnóstico possui mensagem principal;
- diagnósticos com origem de código possuem span quando possível;
- `InternalError` é distinguível por dado estruturado;
- `DiagnosticBag` preserva ordem;
- `has_errors` inclui erros internos;
- labels referenciam spans válidos ou sintéticos permitidos;
- sugestões com substituição referenciam span;
- renderização não é necessária para testar a estrutura.

---

## 24. Limitações do Stage 1

No Stage 1, não é obrigatório implementar:

- catálogo completo de códigos;
- internacionalização de mensagens;
- sugestões machine-applicable em todos os erros;
- saída JSON;
- labels secundárias múltiplas;
- agrupamento avançado de diagnósticos;
- deduplicação global;
- limites finais de quantidade de diagnósticos;
- integração LSP completa.

Essas limitações não devem impedir diagnósticos estruturados para leitura de fonte, UTF-8, spans e lexer.

---

## 25. Testes Obrigatórios

Os testes do modelo de diagnósticos devem cobrir:

- criação de erro;
- criação de warning;
- criação de note;
- criação de erro interno;
- `has_errors` com erro de usuário;
- `has_errors` com erro interno;
- `has_internal_errors`;
- preservação da mensagem;
- preservação da severidade;
- preservação de código, quando implementado;
- preservação de span primário;
- diagnóstico sem span para arquivo inexistente;
- label primária;
- nota;
- sugestão, quando implementada;
- ordem de inserção no `DiagnosticBag`;
- iteração determinística;
- diagnóstico léxico com span;
- diagnóstico de UTF-8 inválido como erro estruturado.

Testes de renderização pertencem a `OUTPUT-FORMATS.md` ou `DIAGNOSTIC-STYLE-GUIDE.md`, mas devem consumir este modelo.

---

## 26. Critérios de Aprovação

Este documento pode ser considerado aprovado quando:

- a estrutura conceitual de `Diagnostic` estiver aceita;
- severidades estiverem definidas;
- erro de usuário e erro interno estiverem separados;
- código, mensagem, spans, labels, notas e sugestões tiverem contrato claro;
- `DiagnosticBag` estiver definido;
- os diagnósticos léxicos mínimos do Stage 1 estiverem cobertos;
- as limitações estiverem explícitas;
- os testes obrigatórios forem rastreáveis às entregas do Documento 28.

Após aprovação, `capi-diagnostics` deve evoluir do modelo mínimo atual para suportar códigos e spans antes da conclusão formal do Stage 1.
