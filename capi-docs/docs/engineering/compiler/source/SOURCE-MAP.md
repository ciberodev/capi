# Source Map

**Projeto:** Linguagem Capi  
**Documento:** SOURCE-MAP  
**Status:** Aprovado  
**Stage:** Stage 1 — Fontes, diagnósticos e lexer  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o contrato de engenharia do `SourceMap` da implementação oficial da Linguagem Capi.

O `SourceMap` é a estrutura responsável por manter as fontes carregadas durante uma compilação e por servir como autoridade para consultas de localização necessárias ao lexer, aos spans e aos diagnósticos.

Seu objetivo é definir:

- posse e registro de `SourceFile`;
- atribuição de `SourceId`;
- consulta segura de fontes;
- acesso controlado ao texto;
- base para cálculo de linha e coluna;
- recuperação de trechos de código;
- integração com spans, diagnósticos e sessão de compilação;
- limites do Stage 1.

---

## 2. Escopo

Este documento cobre:

- estrutura conceitual do `SourceMap`;
- invariantes de armazenamento;
- regras de inserção e consulta;
- carregamento de arquivos;
- fontes em memória;
- tabelas de início de linha;
- consulta por offset;
- recuperação de texto por intervalo;
- tratamento de IDs inválidos;
- testes obrigatórios de `SourceMap`.

Este documento não cobre:

- definição detalhada de `SourceId` e `SourceFile`;
- definição final de `Span`;
- política completa de Unicode;
- cálculo final de coluna visual;
- formatação de diagnósticos;
- resolução de módulos;
- virtual file system;
- cache incremental;
- serialização persistente.

Esses temas pertencem a:

- `SOURCE-MODEL.md`;
- `SPANS-AND-LOCATIONS.md`;
- `UNICODE-AND-ENCODING.md`;
- `DIAGNOSTIC-DATA-MODEL.md`;
- `DIAGNOSTIC-ARCHITECTURE.md`.

---

## 3. Papel no Pipeline

No pipeline de compilação, o `SourceMap` pertence à fase de carregamento de fontes e é infraestrutura compartilhada pelas fases posteriores.

Fluxo conceitual:

```text
Entrada do driver
    ↓
Carregamento de fontes
    ↓
SourceMap
    ↓
SourceFile + SourceId
    ↓
Lexer
    ↓
Tokens com spans
    ↓
Diagnósticos com localizações
```

O `SourceMap` deve existir antes da execução do lexer. O lexer não deve carregar arquivos diretamente nem criar identidades de fonte por conta própria.

---

## 4. Responsabilidades

O `SourceMap` deve:

- armazenar todas as fontes registradas em uma compilação;
- atribuir `SourceId` estável para cada fonte;
- permitir consulta de `SourceFile` por `SourceId`;
- preservar o texto original disponível ao lexer;
- calcular ou armazenar dados auxiliares para linha e coluna;
- validar intervalos antes de retornar trechos;
- fornecer dados para renderização de diagnósticos;
- reportar ausência ou erro de consulta sem panic;
- manter comportamento determinístico.

O `SourceMap` não deve:

- tokenizar;
- interpretar sintaxe;
- resolver nomes ou módulos;
- emitir diagnósticos diretamente;
- imprimir em `stdout` ou `stderr`;
- depender do parser, da AST, da HIR, da MIR ou de backends;
- transformar caminho físico em identidade sem `SourceId`.

---

## 5. Entidades Relacionadas

| Entidade | Relação com `SourceMap` |
| --- | --- |
| `SourceId` | Chave opaca atribuída pelo `SourceMap`. |
| `SourceFile` | Valor armazenado e consultado pelo `SourceMap`. |
| `Span` | Intervalo que referencia uma fonte por `SourceId`. |
| `Location` | Linha e coluna derivadas de um offset em uma fonte. |
| `LineIndex` | Tabela auxiliar para consulta eficiente de linhas. |

`Location` e `LineIndex` são conceitos de engenharia. Seus nomes finais podem mudar na implementação, desde que o contrato deste documento seja preservado.

---

## 6. Modelo Conceitual

Contrato conceitual:

```rust
pub struct SourceMap {
    files: Vec<SourceFile>,
    line_indexes: Vec<LineIndex>,
}
```

A implementação inicial pode começar apenas com `files: Vec<SourceFile>`, desde que a evolução para consultas de linha, coluna e trechos esteja prevista antes da conclusão do Stage 1.

`SourceMap` é o proprietário operacional das fontes. Fases posteriores devem receber referências ou IDs, não cópias independentes do conteúdo completo sem necessidade.

---

## 7. Inserção de Fontes

### 7.1 Fonte em Memória

O `SourceMap` deve permitir registrar fonte a partir de texto já disponível em memória.

Uso esperado:

- testes unitários;
- testes de lexer;
- snippets;
- futuras integrações com ferramentas;
- fontes sintéticas.

Contrato mínimo:

```rust
pub fn add_file(&mut self, path: impl Into<PathBuf>, text: impl Into<String>) -> SourceId;
```

O nome `path` pode representar caminho físico ou nome de exibição. Fontes em memória não devem exigir existência no sistema de arquivos.

### 7.2 Fonte do Disco

O `SourceMap` deve permitir carregar fonte a partir do sistema de arquivos.

Contrato mínimo:

```rust
pub fn load_file(&mut self, path: impl Into<PathBuf>) -> Result<SourceId, SourceError>;
```

O carregamento deve:

- ler o conteúdo;
- validar que o texto armazenado é UTF-8 válido;
- registrar a fonte;
- retornar o `SourceId`;
- preservar erro de leitura de forma estruturada.

### 7.3 Ordem de Inserção

A implementação inicial pode atribuir IDs em ordem de inserção.

Essa ordem deve ser determinística dentro de uma mesma execução e não deve ser usada como regra semântica.

---

## 8. Invariantes de Armazenamento

O `SourceMap` deve garantir:

- cada fonte registrada possui exatamente um `SourceId`;
- cada `SourceId` válido resolve para exatamente uma `SourceFile`;
- `SourceId`s não são reutilizados dentro do mesmo `SourceMap`;
- uma fonte registrada não é removida durante o Stage 1;
- o texto de uma fonte registrada é tratado como imutável;
- consultas inválidas retornam ausência ou erro estruturado;
- o tamanho lógico do `SourceMap` corresponde ao número de fontes registradas.

Arquivos vazios são fontes válidas.

Múltiplas fontes com mesmo caminho ou nome de exibição podem existir em testes ou cenários sintéticos; a identidade continua sendo o `SourceId`.

---

## 9. Consulta de Fontes

### 9.1 Consulta por ID

Consulta por `SourceId` deve retornar referência à fonte ou ausência controlada.

Contrato mínimo:

```rust
pub fn get(&self, id: SourceId) -> Option<&SourceFile>;
```

Essa API não deve causar panic quando receber um ID inexistente.

### 9.2 Tamanho e Vazio

O `SourceMap` deve expor consultas simples:

```rust
pub fn len(&self) -> usize;
pub fn is_empty(&self) -> bool;
```

Essas APIs existem para testes, validação de sessão e verificações de pré-condição no driver.

### 9.3 Iteração

A implementação pode expor iteração sobre fontes registradas quando houver necessidade real.

Se exposta, a iteração deve preservar ordem determinística de inserção e não permitir mutação arbitrária do conteúdo.

---

## 10. Offsets

Offsets são posições absolutas dentro do texto armazenado de uma `SourceFile`.

No Stage 1:

- offsets devem ser baseados em bytes do texto UTF-8 armazenado;
- offset `0` representa o início do arquivo;
- offset igual ao tamanho em bytes do texto representa o fim do arquivo;
- offsets maiores que o tamanho do texto são inválidos;
- APIs devem distinguir offset válido de fronteira inválida de caractere UTF-8 quando necessário.

O uso de offset em bytes é uma decisão de engenharia para eficiência e compatibilidade com slicing interno em Rust. Isso não autoriza tratar byte, caractere Unicode e coluna visual como equivalentes.

---

## 11. Tabela de Linhas

Para permitir diagnósticos precisos, o `SourceMap` deve manter ou calcular uma tabela de início de linhas para cada fonte.

Contrato conceitual:

```rust
pub struct LineIndex {
    line_starts: Vec<ByteOffset>,
}
```

Regras:

- a primeira linha começa no offset `0`;
- cada quebra de linha registrada adiciona o início da linha seguinte;
- arquivos vazios possuem uma linha lógica inicial;
- a tabela deve ser derivada do texto armazenado;
- a tabela deve ser determinística;
- a tabela não deve alterar o texto da fonte.

O tratamento detalhado de `\n`, `\r\n`, BOM e outras políticas de codificação pertence a `UNICODE-AND-ENCODING.md`. O `SourceMap` deve apenas aplicar a política aprovada.

---

## 12. Linha e Coluna

O `SourceMap` deve fornecer base para converter offset em posição apresentável.

Contrato conceitual:

```rust
pub struct SourceLocation {
    pub source: SourceId,
    pub line: u32,
    pub column: u32,
}
```

Regras iniciais:

- linha e coluna exibidas para usuários devem ser 1-based;
- APIs internas podem usar índices 0-based se isso estiver documentado no tipo;
- linha deve ser derivada da tabela de linhas;
- coluna deve ser calculada a partir do início da linha;
- cálculo de coluna não deve assumir que cada byte corresponde a uma coluna visual.

A definição final de coluna em presença de Unicode, tabs e caracteres de largura variável pertence a `SPANS-AND-LOCATIONS.md` e `UNICODE-AND-ENCODING.md`.

---

## 13. Trechos de Código

O `SourceMap` deve permitir recuperação controlada de trechos para diagnósticos, dumps e testes.

Contrato conceitual:

```rust
pub fn slice(&self, source: SourceId, start: ByteOffset, end: ByteOffset) -> Option<&str>;
```

Uma consulta de trecho deve retornar ausência controlada quando:

- o `SourceId` não existir;
- `start > end`;
- `end` exceder o tamanho do texto;
- `start` ou `end` não estiverem em fronteira válida de caractere UTF-8.

Trechos devem ser views do texto armazenado sempre que possível, sem cópia desnecessária.

---

## 14. Relação com `Span`

`Span` deve referenciar intervalos em fontes armazenadas no `SourceMap`.

Contrato conceitual:

```rust
pub struct Span {
    source: SourceId,
    start: ByteOffset,
    end: ByteOffset,
}
```

O `SourceMap` deve conseguir validar e resolver um `Span` para:

- fonte;
- linha inicial;
- coluna inicial;
- linha final;
- coluna final;
- trecho correspondente, quando válido.

A representação final de `Span`, incluindo spans sintéticos e combinação de spans, pertence a `SPANS-AND-LOCATIONS.md`.

---

## 15. Relação com Diagnósticos

Diagnósticos dependem do `SourceMap` para renderizar localizações.

O `SourceMap` deve fornecer dados suficientes para:

- exibir nome de arquivo ou nome de exibição;
- calcular linha e coluna;
- mostrar trecho relevante;
- destacar intervalo primário;
- destacar intervalos secundários em versões futuras.

O `SourceMap` não decide:

- severidade;
- código do diagnóstico;
- texto da mensagem;
- sugestões de correção;
- formato de saída.

Essas decisões pertencem à infraestrutura de diagnósticos.

---

## 16. Relação com a Sessão

Cada sessão de compilação deve possuir ou referenciar um `SourceMap`.

No Stage 1:

- uma invocação do compilador deve usar um `SourceMap` principal;
- o driver deve carregar a entrada antes de executar o lexer;
- fases posteriores devem receber o contexto necessário sem criar novo `SourceMap` paralelo;
- testes podem construir `SourceMap` diretamente para isolamento.

Um `SourceMap` não deve depender da sessão para operar consultas básicas. Essa independência mantém o crate `capi-source` testável isoladamente.

---

## 17. Erros

O `SourceMap` deve distinguir:

| Situação | Tratamento |
| --- | --- |
| Falha de leitura | `SourceError` ou erro estruturado equivalente. |
| Fonte inexistente por ID | ausência controlada, como `Option::None`. |
| Offset inválido | ausência ou erro de consulta. |
| Intervalo inválido | ausência ou erro de consulta. |
| UTF-8 inválido em arquivo | erro estruturado de carregamento. |

No Stage 1, é aceitável iniciar com erros simples de IO para `load_file`, desde que a evolução para categorias próprias esteja alinhada aos documentos de Unicode e diagnósticos.

Panic não deve ser usado para erros causados por entrada do usuário.

---

## 18. API Pública Inicial

A API pública inicial esperada é:

```rust
pub struct SourceMap { ... }

impl SourceMap {
    pub fn add_file(&mut self, path: impl Into<PathBuf>, text: impl Into<String>) -> SourceId;
    pub fn load_file(&mut self, path: impl Into<PathBuf>) -> Result<SourceId, SourceError>;
    pub fn get(&self, id: SourceId) -> Option<&SourceFile>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
```

Extensões previstas durante o Stage 1:

```rust
impl SourceMap {
    pub fn location(&self, source: SourceId, offset: ByteOffset) -> Option<SourceLocation>;
    pub fn slice(&self, source: SourceId, start: ByteOffset, end: ByteOffset) -> Option<&str>;
    pub fn line_text(&self, source: SourceId, line: u32) -> Option<&str>;
}
```

Os nomes finais podem variar. O contrato obrigatório é a capacidade, não a grafia exata da API.

---

## 19. Performance

O `SourceMap` deve ser simples no Stage 1, mas não deve introduzir escolhas que inviabilizem arquivos grandes.

Diretrizes:

- registro de fonte deve ser linear no tamanho do texto;
- consulta por `SourceId` deve ser O(1) na implementação baseada em vetor;
- consulta de linha por offset deve usar busca eficiente na tabela de linhas;
- recuperação de trecho não deve copiar texto sem necessidade;
- estruturas auxiliares devem ser construídas uma vez por fonte ou sob demanda com cache explícito.

O Stage 1 não exige otimização avançada, mas exige comportamento previsível e testável.

---

## 20. Limitações do Stage 1

No Stage 1, não é obrigatório implementar:

- remoção de fontes;
- atualização incremental de texto;
- compactação de memória;
- cache persistente entre compilações;
- remapeamento de caminhos;
- source map entre código gerado e código original;
- resolução de módulos por importação;
- integração com LSP;
- suporte completo a arquivos virtuais.

Essas limitações não devem impedir que o lexer produza tokens com spans corretos para um arquivo Capi carregado.

---

## 21. Testes Obrigatórios

Os testes de `SourceMap` devem cobrir:

- criação vazia;
- `len` e `is_empty`;
- registro de uma fonte em memória;
- registro de múltiplas fontes com IDs distintos;
- estabilidade de `SourceId`;
- consulta por ID válido;
- consulta por ID inválido;
- preservação exata do texto;
- preservação do caminho ou nome de exibição;
- carregamento de arquivo existente;
- erro ao carregar arquivo inexistente;
- arquivo vazio;
- arquivo com múltiplas linhas;
- cálculo de início de linhas;
- offset no início do arquivo;
- offset no fim do arquivo;
- offset fora do arquivo;
- recuperação de trecho válido;
- rejeição de intervalo inválido;
- rejeição de fronteira UTF-8 inválida, quando a API de trecho for implementada.

Testes de coluna visual, tabs, BOM e Unicode detalhado devem ser cobertos em conjunto com `UNICODE-AND-ENCODING.md` e `SPANS-AND-LOCATIONS.md`.

---

## 22. Critérios de Aprovação

Este documento pode ser considerado aprovado quando:

- a responsabilidade do `SourceMap` estiver separada de `SourceFile`, `Span`, lexer e diagnósticos;
- o contrato de registro e consulta estiver aceito;
- a política inicial de offsets estiver definida;
- a necessidade de tabela de linhas estiver definida;
- as APIs mínimas do Stage 1 estiverem claras;
- as limitações estiverem explícitas;
- os testes obrigatórios estiverem rastreáveis às entregas do Documento 28.

Após aprovação, `capi-source` deve ser ajustado para implementar o contrato mínimo e preparar as extensões necessárias para spans, linha, coluna e diagnósticos léxicos.
