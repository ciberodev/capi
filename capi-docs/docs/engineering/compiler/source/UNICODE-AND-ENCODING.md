# Unicode and Encoding

**Projeto:** Linguagem Capi  
**Documento:** UNICODE-AND-ENCODING  
**Status:** Aprovado  
**Stage:** Stage 1 — Fontes, diagnósticos e lexer  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define a política de engenharia para codificação, Unicode, quebras de linha e colunas na infraestrutura de fontes da implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer:

- a codificação aceita para arquivos Capi;
- o tratamento de UTF-8 inválido;
- a política de Byte Order Mark;
- a preservação do texto original;
- a política de quebras de linha;
- a relação entre bytes, caracteres Unicode e colunas;
- os requisitos para lexer, spans, source map e diagnósticos;
- os testes obrigatórios do Stage 1.

---

## 2. Escopo

Este documento cobre:

- leitura de arquivos como UTF-8;
- validação de bytes de entrada;
- representação textual em `SourceFile`;
- offsets em bytes;
- fronteiras de caracteres UTF-8;
- cálculo inicial de linha e coluna;
- tratamento de `\n`, `\r\n` e `\r`;
- tratamento de BOM;
- caracteres Unicode em comentários, strings, chars e identificadores;
- recuperação de erro em entrada inválida.

Este documento não cobre:

- regras completas de formação de identificadores;
- tabela final de classes Unicode aceitas pela linguagem;
- normalização semântica de identificadores;
- comparação de strings em tempo de execução;
- internacionalização da biblioteca padrão;
- formatação final de diagnósticos;
- protocolo LSP;
- codificações alternativas como entrada nativa.

Esses temas pertencem a:

- Documento 04 — Sintaxe da Linguagem;
- `SOURCE-MODEL.md`;
- `SOURCE-MAP.md`;
- `SPANS-AND-LOCATIONS.md`;
- `TOKEN-MODEL.md`;
- `LEXER-IMPLEMENTATION.md`;
- `DIAGNOSTIC-DATA-MODEL.md`.

---

## 3. Princípios

A política de Unicode e encoding deve seguir estes princípios:

- a codificação oficial de arquivos-fonte Capi é UTF-8;
- o texto armazenado em `SourceFile` deve ser UTF-8 válido;
- offsets internos são offsets de byte no texto armazenado;
- byte, caractere Unicode e coluna visual são conceitos distintos;
- diagnósticos devem apontar posições estáveis e reproduzíveis;
- o lexer deve operar de forma determinística para a mesma entrada;
- entrada inválida deve gerar erro estruturado, não panic;
- suporte a codificações adicionais só pode existir como conversão prévia explícita para UTF-8.

---

## 4. Codificação de Entrada

Todo arquivo Capi deve ser interpretado como UTF-8.

No Stage 1, a implementação oficial deve aceitar como fonte válida:

- arquivo vazio;
- sequência de bytes UTF-8 válida;
- UTF-8 válido com BOM inicial, conforme a política deste documento.

Arquivos em outras codificações não são entrada nativa da implementação oficial. Uma ferramenta futura pode converter outra codificação para UTF-8 antes de registrar a fonte, mas essa conversão deve acontecer fora do contrato básico de `SourceFile`.

---

## 5. Validação de UTF-8

O carregamento de fonte deve validar os bytes antes de expor texto ao lexer.

Regras:

- bytes que não formam UTF-8 válido devem produzir erro estruturado;
- `SourceFile::text` não deve conter texto inválido;
- o lexer não deve receber `&str` inválido;
- falha de UTF-8 é erro esperado de usuário;
- a implementação não deve substituir bytes inválidos silenciosamente por caractere de reposição.

No Stage 1, quando a leitura usa APIs que já validam UTF-8, como leitura para `String`, o erro deve ser preservado ou convertido em categoria própria antes da conclusão formal do stage.

---

## 6. Byte Order Mark

Um BOM UTF-8 inicial (`U+FEFF`) pode aparecer no início de um arquivo.

Política da implementação oficial:

- BOM inicial é aceito;
- BOM inicial não deve produzir token;
- BOM inicial não deve alterar o comportamento léxico do programa;
- offsets continuam referenciando o texto armazenado;
- diagnósticos não devem apontar o BOM como parte de um token real;
- `U+FEFF` fora do início do arquivo deve ser tratado como caractere normal ou inválido conforme a regra léxica aplicável ao contexto.

A implementação pode escolher armazenar o texto com o BOM preservado e ajustar o início léxico, ou armazenar uma forma sem BOM inicial. A escolha deve ser única, documentada no código e refletida nos testes.

Para reduzir ambiguidade no Stage 1, a política preferida é preservar o texto carregado e fazer o lexer ignorar o BOM somente quando ele estiver no offset inicial da fonte.

---

## 7. Preservação do Texto

O texto armazenado em `SourceFile` é a autoridade para spans.

Regras:

- o conteúdo não deve ser normalizado de forma silenciosa;
- quebras de linha não devem ser reescritas durante o carregamento;
- caracteres Unicode válidos devem ser preservados;
- o lexer deve preservar lexemas exatamente como aparecem no texto armazenado;
- spans devem referenciar offsets no texto armazenado.

Qualquer normalização futura deve ser uma decisão explícita, coberta por ADR ou atualização documental quando afetar spans, diagnósticos ou comportamento observável.

---

## 8. Offsets em Bytes

Offsets internos usados por `Span`, `SourceMap` e lexer são offsets de byte no texto UTF-8 armazenado.

Regras:

- offset `0` representa o início da fonte;
- offset igual ao tamanho em bytes representa o fim da fonte;
- offsets maiores que o tamanho em bytes são inválidos;
- extração de `&str` exige que início e fim estejam em fronteiras válidas de caractere UTF-8;
- offsets não representam número de caracteres Unicode;
- offsets não representam coluna visual.

Essa política permite slicing eficiente em Rust sem perder precisão para Unicode.

---

## 9. Fronteiras de Caracteres

Uma fronteira de caractere UTF-8 é uma posição de byte onde um valor Unicode escalar começa ou termina.

APIs que retornam texto por intervalo devem rejeitar ou retornar ausência quando:

- `start` não é fronteira válida;
- `end` não é fronteira válida;
- `start > end`;
- `end` excede o tamanho do texto.

O lexer deve avançar por caracteres Unicode válidos, preservando os offsets de byte de início e fim de cada lexema.

---

## 10. Quebras de Linha

O `SourceMap` deve reconhecer quebras de linha para construir tabela de linhas e calcular localizações.

Política inicial:

| Sequência | Tratamento |
| --- | --- |
| `\n` | Quebra de linha. |
| `\r\n` | Uma única quebra de linha. |
| `\r` | Quebra de linha aceita para localização. |

O texto armazenado não deve ser reescrito. A tabela de linhas deve interpretar essas sequências de forma determinística.

Quando `\r\n` aparece, o início da próxima linha é o offset após os dois bytes da sequência. A sequência não deve contar como duas quebras.

---

## 11. Tabela de Linhas

A tabela de linhas deve ser derivada do texto armazenado.

Regras:

- a primeira linha começa no offset `0`;
- arquivo vazio possui linha 1;
- cada quebra de linha adiciona o início da linha seguinte;
- uma quebra de linha no fim do arquivo cria uma linha lógica vazia ao final;
- a tabela deve armazenar offsets de byte;
- a tabela não deve alterar o texto.

Exemplo:

```text
a\nb
```

Tabela conceitual:

```text
line 1 starts at byte 0
line 2 starts at byte 2
```

---

## 12. Colunas

Colunas exibidas em diagnósticos devem ser 1-based.

No Stage 1, a coluna deve representar posição textual dentro da linha, calculada a partir de caracteres Unicode escalares desde o início da linha até o offset consultado.

Regras:

- coluna 1 é o início da linha;
- bytes de continuação UTF-8 não incrementam coluna separadamente;
- uma letra ASCII ocupa uma coluna textual;
- um valor Unicode escalar não ASCII ocupa uma coluna textual inicial no Stage 1;
- tabs contam como uma coluna textual no Stage 1;
- caracteres combinantes contam como uma coluna textual no Stage 1.

Essa política é suficiente para diagnósticos determinísticos iniciais. Coluna visual exata de terminal ou editor, incluindo largura de tabs, caracteres combinantes, emoji e East Asian Width, fica fora do Stage 1.

---

## 13. Caracteres Unicode no Lexer

O lexer deve processar entrada como sequência de valores Unicode escalares válidos.

Ele deve preservar:

- lexema original;
- offsets de byte;
- span do token;
- distinção entre maiúsculas e minúsculas;
- caracteres Unicode em strings, chars e comentários.

Classificação de caracteres em identificadores, dígitos, operadores ou elementos inválidos pertence ao modelo de tokens e à implementação do lexer, respeitando o Documento 04.

Este documento não autoriza o lexer a aceitar qualquer caractere Unicode em qualquer posição. Ele apenas define como caracteres válidos de entrada são decodificados e localizados.

---

## 14. Identificadores

A linguagem diferencia maiúsculas de minúsculas.

No Stage 1:

- identificadores devem preservar o texto original;
- o lexer não deve normalizar identificadores;
- comparações futuras de nomes devem ocorrer sobre a forma preservada ou internada definida pela análise semântica;
- caracteres Unicode aceitos em identificadores devem seguir a política definida em `TOKEN-MODEL.md` e `LEXER-IMPLEMENTATION.md`.

Normalização Unicode, como NFC ou NFD, não deve ser aplicada silenciosamente no Stage 1.

---

## 15. Strings e Chars

Strings e chars podem conter caracteres Unicode válidos conforme a sintaxe da linguagem.

O lexer deve:

- preservar o lexema original;
- registrar span completo incluindo delimitadores;
- validar sequências de escape conforme o contrato léxico;
- reportar strings ou chars não terminados com span útil;
- não alterar normalização Unicode do conteúdo.

A representação normalizada do valor de string ou char pode ser produzida pelo lexer ou por fase posterior, mas não substitui o lexema original.

---

## 16. Comentários

Comentários podem conter texto Unicode válido.

O lexer deve:

- reconhecer delimitadores de comentário por seus bytes/caracteres definidos pela sintaxe;
- preservar localização correta ao atravessar comentários;
- atualizar tabela de linha indiretamente por meio do `SourceMap`;
- reportar comentário de bloco não terminado com span útil, quando essa categoria existir na linguagem.

Comentários não influenciam semântica, mas influenciam localizações porque podem conter quebras de linha.

---

## 17. Caracteres Inválidos

Há duas categorias distintas de erro:

| Categoria | Exemplo | Responsável inicial |
| --- | --- | --- |
| UTF-8 inválido | bytes sem decodificação válida | carregamento de fonte |
| Unicode válido mas léxico inválido | caractere não permitido em contexto léxico | lexer |

UTF-8 inválido deve ser detectado antes de criar `SourceFile::text`.

Unicode válido mas léxico inválido deve produzir diagnóstico léxico com span apontando o caractere ou região problemática.

---

## 18. Recuperação de Erro

Quando possível, o compilador deve continuar após erro de Unicode ou léxico para produzir diagnósticos adicionais.

No Stage 1:

- erro de leitura ou UTF-8 inválido pode impedir o lexer de executar naquele arquivo;
- erro léxico em caractere Unicode válido deve permitir avanço para o próximo ponto seguro;
- ponto seguro significa próxima fronteira de caractere ou próxima estrutura reconhecível;
- recuperação não deve produzir tokens que ocultem o erro original.

---

## 19. Relação com Diagnósticos

Diagnósticos devem usar linha e coluna derivadas conforme este documento.

Para entrada Unicode:

- o span primário deve cobrir bytes completos de caracteres;
- o trecho exibido deve vir do texto preservado;
- colunas devem ser determinísticas;
- mensagens não devem depender da largura visual do terminal para estarem corretas;
- UTF-8 inválido deve informar arquivo e, quando possível, offset aproximado.

A renderização final de labels, carets e sugestões pertence aos documentos de diagnósticos.

---

## 20. API Esperada

O contrato deste documento exige capacidades, não nomes exatos.

APIs conceituais:

```rust
pub fn validate_utf8(bytes: &[u8]) -> Result<&str, EncodingError>;
pub fn has_initial_bom(text: &str) -> bool;
pub fn lexical_start_offset(text: &str) -> ByteOffset;
pub fn build_line_index(text: &str) -> LineIndex;
pub fn is_char_boundary(text: &str, offset: ByteOffset) -> bool;
```

Integração esperada:

```rust
impl SourceMap {
    pub fn location(&self, source: SourceId, offset: ByteOffset) -> Option<SourceLocation>;
    pub fn line_text(&self, source: SourceId, line: u32) -> Option<&str>;
}
```

Funções auxiliares podem ser privadas se os comportamentos estiverem cobertos por testes públicos.

---

## 21. Limitações do Stage 1

No Stage 1, não é obrigatório implementar:

- suporte nativo a UTF-16, UTF-32 ou Latin-1;
- detecção automática de encoding;
- normalização Unicode;
- política final de confusables;
- cálculo de coluna visual perfeito;
- largura customizada de tabs;
- segmentação por grapheme cluster;
- integração LSP completa;
- recuperação avançada de UTF-8 inválido dentro de um mesmo arquivo.

Essas limitações não devem impedir leitura correta de arquivos UTF-8 válidos, spans precisos e diagnósticos léxicos estruturados.

---

## 22. Testes Obrigatórios

Os testes de Unicode e encoding devem cobrir:

- arquivo vazio;
- arquivo ASCII;
- arquivo UTF-8 com caractere não ASCII;
- arquivo com emoji ou caractere multibyte;
- preservação exata do texto;
- offset de byte em caractere multibyte;
- rejeição de fronteira UTF-8 inválida em slice;
- `\n` como quebra de linha;
- `\r\n` como uma quebra de linha;
- `\r` como quebra de linha para localização;
- quebra de linha no fim do arquivo;
- BOM inicial aceito;
- BOM inicial sem token real;
- `U+FEFF` fora do início tratado por regra léxica normal;
- UTF-8 inválido rejeitado no carregamento;
- string com Unicode;
- char Unicode válido;
- comentário com Unicode e quebra de linha;
- diagnóstico léxico com coluna determinística em linha com Unicode;
- tab contabilizado conforme política do Stage 1.

Testes de classificação de identificadores devem ser definidos em conjunto com `TOKEN-MODEL.md` e `LEXER-IMPLEMENTATION.md`.

---

## 23. Critérios de Aprovação

Este documento pode ser considerado aprovado quando:

- UTF-8 estiver definido como codificação oficial da implementação;
- a política de BOM estiver aceita;
- a política de preservação do texto estiver clara;
- a política de quebras de linha estiver definida;
- a relação entre offset de byte, caractere Unicode e coluna estiver clara;
- as limitações do Stage 1 estiverem explícitas;
- os testes obrigatórios forem rastreáveis às entregas do Documento 28.

Após aprovação, `capi-source` e o lexer devem implementar a política aqui definida antes da conclusão formal do Stage 1.
