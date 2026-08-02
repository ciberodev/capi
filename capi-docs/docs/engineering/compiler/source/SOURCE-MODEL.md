# Source Model

**Projeto:** Linguagem Capi  
**Documento:** SOURCE-MODEL  
**Status:** Aprovado  
**Stage:** Stage 1 — Fontes, diagnósticos e lexer  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o modelo de engenharia para representação de arquivos-fonte na implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer o contrato inicial de:

- `SourceId`;
- `SourceFile`;
- relação entre `SourceFile` e `SourceMap`;
- conteúdo textual preservado;
- caminhos físicos e nomes exibidos;
- limites de responsabilidade do crate `capi-source`;
- integração com spans, lexer, diagnósticos e sessão de compilação.

Este documento deve estar aprovado antes da implementação formal de `SourceId`, `SourceFile` e das APIs públicas correspondentes no Stage 1.

---

## 2. Escopo

Este documento cobre:

- identidade de arquivos-fonte dentro de uma compilação;
- armazenamento do texto original;
- representação mínima de caminho;
- invariantes de criação e consulta;
- erros de carregamento;
- relação com UTF-8;
- requisitos para testes de fonte.

Este documento não cobre:

- estrutura detalhada de `SourceMap`;
- cálculo completo de linha e coluna;
- representação final de `Span`;
- política completa de Unicode e normalização;
- resolução de módulos;
- sistema de pacotes;
- lexer ou categorias de tokens;
- formatação de diagnósticos.

Esses temas pertencem aos documentos específicos:

- `SOURCE-MAP.md`;
- `SPANS-AND-LOCATIONS.md`;
- `UNICODE-AND-ENCODING.md`;
- `TOKEN-MODEL.md`;
- `DIAGNOSTIC-DATA-MODEL.md`;
- `DIAGNOSTIC-ARCHITECTURE.md`.

---

## 3. Princípios

O modelo de fontes deve seguir estes princípios:

- a fonte carregada é a autoridade textual para spans e diagnósticos;
- identificadores internos devem ser opacos e tipados;
- caminhos e conteúdo não devem ser confundidos com identidade;
- o lexer recebe texto e localização, mas não controla o armazenamento de fontes;
- diagnósticos referenciam spans ou fontes, não caminhos crus isolados;
- falhas esperadas de usuário devem produzir erro estruturado, não panic;
- o modelo inicial deve suportar um arquivo, sem impedir múltiplos arquivos no futuro.

O crate `capi-source` é infraestrutura compartilhada. Ele não deve conter regras sintáticas, semânticas, de tokenização ou de resolução de módulos.

---

## 4. Entidades

O modelo inicial possui três entidades principais:

| Entidade | Responsabilidade |
| --- | --- |
| `SourceId` | Identificar uma fonte carregada dentro de uma sessão ou source map. |
| `SourceFile` | Armazenar metadados e texto de uma fonte carregada. |
| `SourceMap` | Manter o conjunto de fontes carregadas e resolver consultas por `SourceId`. |

Este documento define `SourceId` e `SourceFile` diretamente. `SourceMap` é mencionado apenas como proprietário operacional das fontes; seu contrato detalhado pertence a `SOURCE-MAP.md`.

---

## 5. `SourceId`

### 5.1 Definição

`SourceId` é um identificador opaco atribuído a cada fonte registrada em um `SourceMap`.

Contrato conceitual:

```rust
pub struct SourceId(u32);
```

A representação física pode evoluir, mas o contrato externo deve preservar:

- opacidade;
- cópia barata;
- comparação por igualdade;
- uso como chave em mapas;
- ausência de significado fora do `SourceMap` que o criou.

### 5.2 Invariantes

Um `SourceId` deve:

- identificar no máximo uma fonte dentro de um `SourceMap`;
- permanecer estável durante a vida do `SourceMap`;
- não depender de endereço de memória;
- não expor índice cru como API principal de navegação;
- não ser reutilizado para outra fonte na mesma sessão.

O valor bruto pode existir para testes, dumps e serialização futura, mas não deve ser usado por fases do compilador para inferir ordem, caminho ou significado sem consultar o `SourceMap`.

### 5.3 Escopo de Validade

`SourceId` é válido apenas no contexto que o criou.

Comparar `SourceId`s de sessões ou `SourceMap`s diferentes não possui significado sem um contrato explícito de serialização ou cache incremental. O Stage 1 não define esse contrato.

### 5.4 Ordem

A implementação inicial pode atribuir `SourceId`s em ordem de inserção.

Essa ordem é detalhe operacional do `SourceMap` e não deve ser usada para resolver dependências, módulos ou prioridade de diagnóstico.

---

## 6. `SourceFile`

### 6.1 Definição

`SourceFile` representa uma fonte carregada e registrada.

Contrato conceitual:

```rust
pub struct SourceFile {
    id: SourceId,
    path: SourcePath,
    text: SourceText,
}
```

A implementação inicial pode usar `PathBuf` e `String` diretamente, desde que preserve os invariantes deste documento.

### 6.2 Campos Conceituais

| Campo | Finalidade |
| --- | --- |
| `id` | Identidade interna da fonte. |
| `path` | Caminho físico ou nome de exibição associado à fonte. |
| `text` | Conteúdo textual preservado após leitura e validação de codificação. |

Campos adicionais podem ser adicionados por `SOURCE-MAP.md`, `SPANS-AND-LOCATIONS.md` ou `UNICODE-AND-ENCODING.md`, como tabela de linhas, informação de BOM, nome lógico de módulo ou origem sintética.

### 6.3 Texto Original

`SourceFile` deve preservar o texto da fonte como entrada para o lexer.

No Stage 1:

- o texto armazenado deve ser UTF-8 válido;
- offsets usados por spans devem referenciar posições nesse texto armazenado;
- o lexer deve conseguir recuperar lexemas a partir dos offsets definidos;
- quebras de linha devem permanecer representáveis para cálculo posterior de linha e coluna.

Se houver normalização de quebras de linha ou tratamento de BOM, a política deve ser definida em `UNICODE-AND-ENCODING.md` e refletida nos testes.

### 6.4 Caminho

O caminho associado a uma fonte serve para:

- mensagens de diagnóstico;
- dumps de tokens;
- rastreabilidade;
- futuras integrações com módulos e ferramentas.

O caminho não é a identidade interna da fonte. Duas fontes podem ter o mesmo nome de exibição em cenários sintéticos ou de teste, e a distinção formal continua sendo o `SourceId`.

### 6.5 Imutabilidade

Depois de registrada no `SourceMap`, uma `SourceFile` deve ser tratada como imutável pelas fases do compilador.

Fases como lexer, parser e AST lowering podem referenciar trechos da fonte, mas não devem alterar seu conteúdo. Caso uma ferramenta futura precise trabalhar com edição incremental, ela deve registrar uma nova versão lógica da fonte por meio de contrato próprio.

---

## 7. Fontes Sintéticas

O modelo deve permitir evolução para fontes sintéticas, mesmo que a implementação inicial não exponha uma API completa para isso.

Exemplos de fontes sintéticas:

- entrada passada diretamente por testes;
- código gerado para testes internos;
- snippets produzidos por ferramentas;
- expansões futuras de macros, se existirem.

Uma fonte sintética ainda deve receber `SourceId`, texto e nome de exibição. Ela não deve exigir caminho físico existente no sistema de arquivos.

---

## 8. Carregamento

### 8.1 Entrada

O carregamento de fonte pode receber:

- caminho físico;
- conteúdo textual explícito;
- nome de exibição para testes ou fontes sintéticas.

### 8.2 Saída

O carregamento bem-sucedido deve produzir:

- `SourceId`;
- `SourceFile` armazenado no `SourceMap`;
- conteúdo disponível para consulta;
- metadados suficientes para diagnósticos.

### 8.3 Erros

Falhas de carregamento devem ser representadas por erro estruturado.

Categorias iniciais:

| Categoria | Exemplo |
| --- | --- |
| Arquivo inexistente | caminho informado não existe. |
| Permissão negada | sistema operacional rejeita leitura. |
| Diretório em vez de arquivo | caminho aponta para diretório. |
| Codificação inválida | bytes não formam UTF-8 válido. |
| Limite excedido | arquivo ultrapassa limite configurado. |

No Stage 1, a implementação pode começar com erro de IO preservando o erro original. A expansão para categorias próprias deve ser alinhada com `DIAGNOSTIC-DATA-MODEL.md` e `UNICODE-AND-ENCODING.md`.

O crate `capi-source` não deve imprimir mensagens diretamente. Ele retorna erros para a camada chamadora, que decide como convertê-los em diagnósticos.

---

## 9. Relação com `SourceMap`

`SourceMap` é o proprietário do conjunto de `SourceFile`s de uma compilação.

Do ponto de vista deste documento, ele deve:

- atribuir `SourceId`;
- armazenar `SourceFile`;
- consultar fonte por `SourceId`;
- preservar a estabilidade dos IDs;
- fornecer acesso controlado ao texto;
- servir como autoridade para localização usada por spans e diagnósticos.

O Stage 1 deve manter a API suficiente para:

- registrar fonte em memória;
- carregar fonte do disco;
- consultar fonte existente;
- identificar fonte ausente sem panic;
- executar testes de estabilidade de ID.

Detalhes como tabela de linhas, busca por offset, cache de nomes e cálculo de coluna pertencem a `SOURCE-MAP.md` e `SPANS-AND-LOCATIONS.md`.

---

## 10. Relação com Spans e Localizações

`SourceFile` não substitui `Span`.

O papel de cada conceito é:

| Conceito | Papel |
| --- | --- |
| `SourceId` | Identifica o arquivo. |
| `SourceFile` | Armazena texto e metadados do arquivo. |
| `Span` | Identifica um intervalo dentro de uma fonte. |
| Linha e coluna | Forma humana de exibir uma posição. |

Um `Span` deve referenciar uma fonte por `SourceId` e posições dentro do texto armazenado. As regras de offset, linha e coluna são definidas nos documentos próprios.

Representações posteriores, como tokens, AST, HIR e diagnósticos, devem preservar spans quando aplicável.

---

## 11. Relação com Unicode e Codificação

A linguagem Capi usa UTF-8 como codificação oficial de entrada.

No modelo de fonte:

- `SourceFile::text` deve conter texto UTF-8 válido;
- offsets devem ser compatíveis com índices de byte do texto armazenado;
- APIs não devem assumir que byte, caractere Unicode e coluna visual são equivalentes;
- cálculo de coluna deve ser definido fora de `SourceFile`;
- erros de UTF-8 inválido devem ser reportáveis sem panic.

Políticas detalhadas de BOM, normalização de quebras de linha, caracteres permitidos e coluna visual pertencem a `UNICODE-AND-ENCODING.md` e `SPANS-AND-LOCATIONS.md`.

---

## 12. Relação com Lexer

O lexer consome `SourceFile` ou uma visão controlada da fonte.

Ele pode:

- ler o texto da fonte;
- produzir tokens com spans associados ao `SourceId`;
- recuperar lexemas a partir de intervalos válidos;
- emitir diagnósticos léxicos associados a spans.

Ele não deve:

- carregar arquivos diretamente;
- criar `SourceId`;
- alterar o texto da fonte;
- resolver módulos;
- tratar caminho físico como identidade sem consultar o `SourceMap`.

O resultado demonstrável do Stage 1, `capic --emit tokens arquivo.capi`, depende desse contrato.

---

## 13. Relação com Diagnósticos

Diagnósticos devem usar o modelo de fonte para apontar origem.

Um diagnóstico associado a código-fonte deve conseguir referenciar:

- arquivo;
- span ou posição;
- linha e coluna derivadas;
- trecho de texto relevante, quando disponível.

`capi-source` fornece dados para diagnósticos, mas não decide severidade, código de erro, estilo de mensagem ou renderização.

Falhas de carregamento podem ser convertidas em diagnósticos por `capi-driver`, `capi-session` ou infraestrutura de diagnósticos, conforme definido nos documentos próprios.

---

## 14. API Pública Inicial

A API pública inicial do crate `capi-source` deve ser pequena e estável o suficiente para o Stage 1.

Contrato mínimo esperado:

```rust
pub struct SourceId(...);

impl SourceId {
    pub const fn from_raw(raw: u32) -> Self;
    pub const fn raw(self) -> u32;
}

pub struct SourceFile { ... }

impl SourceFile {
    pub fn new(id: SourceId, path: impl Into<PathBuf>, text: impl Into<String>) -> Self;
    pub const fn id(&self) -> SourceId;
    pub fn path(&self) -> &Path;
    pub fn text(&self) -> &str;
}
```

Essa API reflete o estado inicial já necessário para Stage 1. Evoluções previstas:

- substituir `PathBuf` direto por tipo específico, se necessário;
- substituir `String` direto por tipo específico, se necessário;
- adicionar origem sintética;
- adicionar limites de tamanho;
- adicionar informações de codificação;
- adicionar tabela de linhas no `SourceMap`.

Mudanças incompatíveis devem ser registradas antes de afetar lexer, parser ou diagnósticos.

---

## 15. Invariantes de Segurança e Robustez

A implementação deve garantir:

- consulta por `SourceId` inválido retorna ausência controlada;
- carregamento de arquivo inválido não causa panic;
- arquivos vazios são fontes válidas;
- texto UTF-8 válido é preservado;
- IDs não colidem dentro do mesmo `SourceMap`;
- APIs públicas não expõem mutação arbitrária de fontes registradas;
- testes não dependem de caminhos absolutos da máquina local.

Entradas malformadas devem ser tratadas como erro de usuário sempre que possível.

Panic deve ser reservado para violações internas de invariantes que não possam ser causadas por entrada normal.

---

## 16. Limitações do Stage 1

No Stage 1, não é obrigatório implementar:

- resolução de módulos;
- múltiplos pacotes;
- cache incremental;
- remapeamento de caminhos;
- virtual file system completo;
- edição incremental;
- fontes geradas por macro;
- serialização persistente de `SourceId`;
- política final de limites de tamanho.

O desenho, porém, não deve impedir essas evoluções.

---

## 17. Testes Obrigatórios

Os testes de `SOURCE-MODEL.md` devem cobrir:

- criação de `SourceId` estável;
- registro de fonte vazia;
- registro de fonte com conteúdo;
- preservação do caminho informado;
- preservação exata do texto UTF-8;
- consulta por ID válido;
- consulta por ID inexistente;
- múltiplas fontes com IDs distintos;
- carregamento de arquivo existente;
- erro ao carregar arquivo inexistente;
- ausência de panic em entradas esperadas.

Testes de linha, coluna, Unicode detalhado e spans completos pertencem aos documentos específicos, mas devem ser compatíveis com este modelo.

---

## 18. Critérios de Aprovação

Este documento pode ser considerado aprovado quando:

- o contrato de `SourceId` estiver aceito;
- o contrato de `SourceFile` estiver aceito;
- a fronteira entre `SourceFile`, `SourceMap` e `Span` estiver clara;
- a relação com lexer e diagnósticos estiver definida;
- as limitações do Stage 1 estiverem explícitas;
- os testes obrigatórios forem rastreáveis às entregas do Documento 28.

Após aprovação, a implementação de `capi-source` deve ser ajustada para refletir este contrato antes da implementação completa do lexer.
