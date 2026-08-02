# Spans and Locations

**Projeto:** Linguagem Capi  
**Documento:** SPANS-AND-LOCATIONS  
**Status:** Aprovado  
**Stage:** Stage 1 — Fontes, diagnósticos e lexer  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o contrato de engenharia para spans, offsets e localizações de código-fonte na implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- como intervalos de texto são representados;
- como tokens, AST, HIR e diagnósticos preservam origem;
- como offsets internos se relacionam com linha e coluna;
- como spans sintéticos são representados;
- como spans podem ser validados, combinados e exibidos;
- quais invariantes devem ser preservados no Stage 1.

---

## 2. Escopo

Este documento cobre:

- `Span`;
- offsets de início e fim;
- intervalo vazio;
- relação com `SourceId`;
- localização apresentável;
- conversão de offset para linha e coluna;
- spans sintéticos;
- combinação de spans;
- recuperação de trechos por span;
- testes obrigatórios.

Este documento não cobre:

- estrutura detalhada de `SourceFile`;
- armazenamento completo de `SourceMap`;
- política completa de Unicode;
- normalização de quebras de linha;
- largura visual definitiva de caracteres;
- modelo completo de diagnósticos;
- estrutura de tokens;
- AST, HIR ou MIR.

Esses temas pertencem a:

- `SOURCE-MODEL.md`;
- `SOURCE-MAP.md`;
- `UNICODE-AND-ENCODING.md`;
- `TOKEN-MODEL.md`;
- `DIAGNOSTIC-DATA-MODEL.md`;
- `AST-MODEL.md`;
- `HIR-MODEL.md`.

---

## 3. Princípios

Spans e localizações devem seguir estes princípios:

- `Span` é a referência oficial de origem no código-fonte;
- offsets internos devem ser precisos e baratos de copiar;
- linha e coluna são derivadas do texto armazenado no `SourceMap`;
- spans não armazenam texto;
- spans não devem depender de endereços de memória;
- spans devem ser preservados ao longo do frontend sempre que aplicável;
- entrada inválida deve produzir diagnóstico com localização útil quando possível;
- APIs de consulta devem retornar ausência ou erro estruturado para spans inválidos, não panic.

---

## 4. Conceitos

| Conceito | Responsabilidade |
| --- | --- |
| `ByteOffset` | Posição absoluta em bytes dentro de uma fonte UTF-8 armazenada. |
| `Span` | Intervalo dentro de uma fonte. |
| `SourceLocation` | Linha e coluna derivadas de um offset. |
| `ResolvedSpan` | Span resolvido para localizações e trecho exibível. |
| Span sintético | Origem sem intervalo direto no arquivo do usuário. |

Os nomes finais podem variar na implementação. O contrato obrigatório é a separação entre offset interno, span e localização apresentável.

---

## 5. `ByteOffset`

`ByteOffset` representa uma posição absoluta em bytes no texto armazenado de uma `SourceFile`.

Contrato conceitual:

```rust
pub struct ByteOffset(u32);
```

A representação física pode ser `u32`, `usize` ou novo tipo equivalente, desde que:

- seja opaca nas APIs públicas relevantes;
- não seja confundida com linha, coluna ou índice de caractere;
- permita comparação e ordenação;
- seja barata de copiar;
- suporte arquivos aceitos pelos limites do compilador.

No Stage 1, offsets são baseados no texto UTF-8 armazenado após a política de leitura aprovada.

---

## 6. `Span`

### 6.1 Definição

`Span` representa um intervalo em uma fonte.

Contrato conceitual:

```rust
pub struct Span {
    source: SourceId,
    start: ByteOffset,
    end: ByteOffset,
}
```

O intervalo é half-open:

```text
[start, end)
```

Isso significa que `start` é incluído no intervalo e `end` aponta para a primeira posição após o intervalo.

### 6.2 Invariantes

Um `Span` real deve obedecer:

- `start <= end`;
- `source` referencia uma fonte registrada no `SourceMap` correspondente;
- `start` e `end` estão dentro do texto da fonte;
- `start` e `end` estão em fronteiras válidas de caractere UTF-8 quando o span for usado para recuperar texto;
- o span não armazena cópia do texto.

Um `Span` vazio, com `start == end`, é válido para representar posições entre tokens, EOF ou pontos de inserção de diagnóstico.

### 6.3 Tamanho

O tamanho de um span em bytes é:

```text
end - start
```

Esse tamanho não representa número de caracteres Unicode nem largura visual.

---

## 7. Localizações

`SourceLocation` representa uma posição apresentável ao usuário.

Contrato conceitual:

```rust
pub struct SourceLocation {
    pub source: SourceId,
    pub line: u32,
    pub column: u32,
}
```

Para exibição em diagnósticos:

- linha deve ser 1-based;
- coluna deve ser 1-based;
- arquivo deve ser obtido pelo `SourceMap`;
- valores devem ser derivados de offsets, não armazenados como autoridade primária no token.

APIs internas podem usar índices 0-based se os tipos deixarem isso explícito, por exemplo `LineIndex` ou `ColumnIndex`.

---

## 8. Linha e Coluna

Linha e coluna são derivadas pelo `SourceMap` a partir do texto da fonte.

Regras iniciais:

- a primeira linha é a linha 1;
- a primeira coluna exibível é a coluna 1;
- offset `0` resolve para linha 1, coluna 1;
- offset no fim do arquivo é válido como posição;
- arquivo vazio possui uma posição inicial válida;
- quebras de linha determinam o início da próxima linha conforme a política de encoding aprovada.

A coluna inicial do Stage 1 pode ser baseada em distância escalar a partir do início da linha, desde que o documento `UNICODE-AND-ENCODING.md` defina a política final para tabs, caracteres combinantes e largura variável.

Nenhuma fase deve assumir que coluna visual é igual a offset em bytes.

---

## 9. Resolução de Span

Resolver um `Span` significa consultar o `SourceMap` para obter informações apresentáveis.

Contrato conceitual:

```rust
pub struct ResolvedSpan<'a> {
    pub span: Span,
    pub start: SourceLocation,
    pub end: SourceLocation,
    pub text: Option<&'a str>,
}
```

Uma resolução deve falhar de forma controlada quando:

- o `SourceId` não existir;
- os offsets estiverem fora do arquivo;
- `start > end`;
- os limites não estiverem em fronteiras válidas de UTF-8 para extração de texto.

Diagnósticos podem ser emitidos mesmo quando o trecho textual não puder ser recuperado, desde que a origem disponível seja suficiente.

---

## 10. Spans Sintéticos

Nem todo elemento interno possui origem direta no código do usuário.

Exemplos:

- nó inserido por recuperação de parser;
- token EOF;
- construção implícita introduzida pelo compilador;
- elemento gerado por expansão futura;
- diagnóstico associado ao arquivo como um todo.

A implementação deve prever uma forma de representar origem sintética.

Modelos aceitáveis:

```rust
pub enum Span {
    Real { source: SourceId, start: ByteOffset, end: ByteOffset },
    Synthetic,
}
```

ou:

```rust
pub struct Span {
    source: Option<SourceId>,
    start: ByteOffset,
    end: ByteOffset,
}
```

O modelo final deve deixar claro quando um span não pode ser resolvido para trecho de arquivo.

No Stage 1, é aceitável iniciar com spans reais e uma constante sintética mínima, desde que diagnósticos não tentem renderizar trecho inexistente como se fosse fonte real.

---

## 11. Combinação de Spans

Fases posteriores precisam formar spans maiores a partir de tokens ou nós.

Exemplos:

- span de expressão binária a partir do operando esquerdo até o direito;
- span de declaração a partir da palavra-chave até o fim do inicializador;
- span de bloco a partir de `{` até `}`.

Combinar spans deve obedecer:

- spans reais só podem ser combinados automaticamente quando pertencem ao mesmo `SourceId`;
- o resultado usa o menor `start` e o maior `end`;
- combinação com span sintético deve seguir política explícita;
- combinação de fontes diferentes deve retornar ausência ou erro, não inventar intervalo.

Contrato conceitual:

```rust
pub fn merge(self, other: Span) -> Option<Span>;
```

---

## 12. Spans em Tokens

Todo token produzido pelo lexer deve possuir span.

Para tokens reais:

- `source` identifica a fonte lexada;
- `start` aponta para o início do lexema;
- `end` aponta para a primeira posição após o lexema;
- o lexema deve poder ser recuperado pelo `SourceMap` quando o token representa texto válido.

Para EOF:

- o span pode ser vazio;
- `start == end`;
- o offset deve apontar para o fim da fonte.

Erros léxicos devem possuir span que cubra a região problemática sempre que possível.

---

## 13. Spans em AST e HIR

Cada nó de AST deve preservar o span correspondente à construção sintática que representa, quando aplicável.

Cada elemento de HIR deve preservar span de origem suficiente para diagnósticos semânticos.

Regras:

- AST deve preferir spans formados diretamente a partir dos tokens;
- HIR deve preservar spans vindos da AST;
- spans não devem ser descartados durante lowering sem justificativa;
- elementos implícitos devem usar span sintético ou span herdado claramente definido;
- diagnósticos posteriores devem apontar para a origem mais específica disponível.

---

## 14. Spans em Diagnósticos

Todo diagnóstico associado ao código do usuário deve carregar pelo menos um span primário quando houver origem disponível.

Diagnósticos podem possuir:

- span primário;
- spans secundários;
- labels associadas a spans;
- notas sem span;
- sugestões com span de substituição.

No Stage 1, diagnósticos léxicos devem apontar para:

- caractere inválido;
- literal não terminado;
- sequência de escape inválida;
- região recuperada após erro, quando aplicável.

A renderização final pertence aos documentos de diagnósticos.

---

## 15. Intervalos Inválidos

Um intervalo é inválido quando:

- `start > end`;
- `source` não existe no `SourceMap`;
- `start` está fora do arquivo;
- `end` está fora do arquivo;
- uma API de texto recebe offset que não é fronteira válida de UTF-8.

Construção pública de spans deve reduzir a chance de criar valores inválidos.

Estratégias aceitáveis:

- construtor validado pelo `SourceMap`;
- tipo `Span` com construtor privado;
- validação explícita ao resolver o span.

O Stage 1 pode usar construtores simples enquanto os testes cobrirem rejeição ou tratamento controlado dos casos inválidos.

---

## 16. API Pública Inicial

API conceitual mínima:

```rust
pub struct ByteOffset(...);
pub struct Span { ... }
pub struct SourceLocation { ... }

impl Span {
    pub fn new(source: SourceId, start: ByteOffset, end: ByteOffset) -> Option<Self>;
    pub fn source(self) -> SourceId;
    pub fn start(self) -> ByteOffset;
    pub fn end(self) -> ByteOffset;
    pub fn is_empty(self) -> bool;
    pub fn merge(self, other: Span) -> Option<Span>;
}
```

Integração esperada com `SourceMap`:

```rust
impl SourceMap {
    pub fn location(&self, source: SourceId, offset: ByteOffset) -> Option<SourceLocation>;
    pub fn resolve_span(&self, span: Span) -> Option<ResolvedSpan<'_>>;
    pub fn span_text(&self, span: Span) -> Option<&str>;
}
```

Os nomes finais podem variar. O contrato obrigatório é a existência das capacidades descritas.

---

## 17. Invariantes de Robustez

A implementação deve garantir:

- spans válidos não causam panic ao serem resolvidos;
- spans inválidos são tratados de forma controlada;
- spans vazios são suportados;
- EOF pode ser representado;
- offsets não são confundidos com colunas;
- spans não mantêm referência direta ao texto;
- combinação de spans de fontes diferentes não produz intervalo falso;
- diagnósticos não tentam renderizar spans sintéticos como trechos reais.

---

## 18. Performance

Spans devem ser compactos e baratos de copiar.

Diretrizes:

- `Span` deve ser `Copy` quando a representação permitir;
- `ByteOffset` deve ser barato de comparar;
- conversão offset -> linha deve usar tabela de linhas;
- recuperação de trecho deve evitar cópia;
- spans devem poder ser armazenados em tokens, AST e HIR sem custo excessivo.

O Stage 1 não exige otimização avançada, mas exige que o modelo não inviabilize arquivos grandes ou muitos tokens.

---

## 19. Limitações do Stage 1

No Stage 1, não é obrigatório implementar:

- mapeamento de código gerado para código original;
- spans de macro;
- múltiplas origens para um único span;
- coluna visual perfeita para todos os casos Unicode;
- integração com LSP;
- serialização de spans;
- cache incremental de localizações.

Essas limitações não devem impedir tokens e diagnósticos léxicos precisos para um arquivo Capi UTF-8 válido.

---

## 20. Testes Obrigatórios

Os testes de spans e localizações devem cobrir:

- criação de span válido;
- rejeição ou tratamento de `start > end`;
- span vazio;
- span no início do arquivo;
- span no fim do arquivo;
- span cobrindo uma linha inteira;
- span atravessando múltiplas linhas;
- resolução de offset para linha e coluna;
- resolução de EOF;
- consulta de texto por span;
- span com `SourceId` inválido;
- offset fora do arquivo;
- fronteira UTF-8 inválida para extração de texto;
- combinação de spans na mesma fonte;
- rejeição de combinação de fontes diferentes;
- span sintético sem tentativa de trecho real;
- diagnóstico léxico com span primário.

Testes detalhados de Unicode, BOM, tabs e quebras de linha específicas devem ser compartilhados com `UNICODE-AND-ENCODING.md`.

---

## 21. Critérios de Aprovação

Este documento pode ser considerado aprovado quando:

- o formato conceitual de `Span` estiver aceito;
- a política de intervalo half-open estiver aceita;
- a relação entre offset, linha e coluna estiver clara;
- spans sintéticos possuírem estratégia definida;
- a integração com `SourceMap`, lexer, AST, HIR e diagnósticos estiver definida;
- os testes obrigatórios forem rastreáveis às entregas do Documento 28.

Após aprovação, `capi-source` deve implementar `Span` e as consultas de localização necessárias antes da expansão completa do lexer.
