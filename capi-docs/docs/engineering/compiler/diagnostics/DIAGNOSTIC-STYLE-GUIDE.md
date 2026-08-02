# Diagnostic Style Guide

**Projeto:** Linguagem Capi  
**Documento:** DIAGNOSTIC-STYLE-GUIDE  
**Status:** Aprovado  
**Stage:** Stage 1 — Fontes, diagnósticos e lexer  
**Natureza:** Documento de engenharia bloqueante  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define o estilo de escrita e apresentação dos diagnósticos da implementação oficial da Linguagem Capi.

Seu objetivo é garantir que diagnósticos sejam:

- claros para humanos;
- estruturados para ferramentas;
- consistentes entre fases;
- precisos em relação a spans;
- estáveis para snapshots e UI tests;
- distintos entre erro de usuário e erro interno.

Este documento complementa `DIAGNOSTIC-DATA-MODEL.md` e `DIAGNOSTIC-ARCHITECTURE.md`.

---

## 2. Escopo

Este documento cobre:

- texto da mensagem principal;
- labels;
- notas;
- sugestões;
- tom e vocabulário;
- uso de severidades;
- estilo para diagnósticos de fonte;
- estilo para diagnósticos léxicos;
- estabilidade de saída textual;
- requisitos de testes.

Este documento não cobre:

- estrutura interna de `Diagnostic`;
- arquitetura de coleta e renderização;
- catálogo definitivo de códigos;
- formato JSON;
- layout visual completo;
- integração LSP;
- crash reports.

Esses temas pertencem a:

- `DIAGNOSTIC-DATA-MODEL.md`;
- `DIAGNOSTIC-ARCHITECTURE.md`;
- `ERROR-CODE-POLICY.md`;
- `OUTPUT-FORMATS.md`;
- `INTERNAL-COMPILER-ERRORS.md`.

---

## 3. Princípios de Estilo

Diagnósticos devem:

- dizer o que está errado;
- apontar onde está errado;
- explicar o motivo quando não for óbvio;
- sugerir correção quando houver correção clara;
- evitar jargão interno;
- evitar mensagens vagas;
- manter tom direto e neutro;
- ser determinísticos.

Um diagnóstico bom deve permitir que o usuário tome a próxima ação sem conhecer a implementação interna do compilador.

---

## 4. Idioma

No Stage 1, as mensagens públicas da implementação oficial devem ser escritas em inglês técnico simples.

Motivos:

- integração mais direta com toolchains;
- menor variação em snapshots;
- consistência com nomes de APIs internas;
- compatibilidade futura com mensagens de ferramentas.

Documentação pode permanecer em português. O idioma das mensagens emitidas pelo compilador pode ser internacionalizado futuramente.

---

## 5. Mensagem Principal

A mensagem principal deve ser curta e específica.

Preferir:

```text
unterminated string literal
invalid character in source file
expected identifier after `module`
source file is not valid UTF-8
```

Evitar:

```text
error
invalid syntax
failed
something went wrong
lexer error
```

Regras:

- usar minúscula inicial, salvo nomes próprios;
- não terminar com ponto final quando for frase curta;
- não incluir caminho, linha ou coluna na mensagem;
- não incluir código de erro no texto da mensagem;
- não mencionar função, struct ou módulo interno;
- não culpar o usuário.

---

## 6. Severidades

Uso esperado:

| Severidade | Estilo |
| --- | --- |
| `Error` | Problema que impede conclusão normal. |
| `Warning` | Problema não bloqueador que pode indicar erro futuro. |
| `Note` | Contexto adicional. |
| `Help` | Orientação de correção. |
| `InternalError` | Falha do compilador, não do programa do usuário. |

Regras:

- erro léxico inválido usa `Error`;
- arquivo inexistente usa `Error`;
- UTF-8 inválido usa `Error`;
- violação de invariante usa `InternalError`;
- notes e help não devem ser usados como substitutos de mensagem principal.

---

## 7. Códigos

Quando houver código, ele deve aparecer de forma estável e separada da mensagem.

Formato textual provisório:

```text
error[LEX0001]: invalid character in source file
```

Regras:

- código não deve depender de ordem de execução;
- código não deve ser inventado por renderizador;
- mensagem não deve precisar repetir o código;
- snapshots devem incluir código quando ele estiver implementado.

O formato final pertence a `ERROR-CODE-POLICY.md`.

---

## 8. Labels

Labels devem explicar a relação entre mensagem e trecho apontado.

Preferir:

```text
invalid character
string starts here
file ends before this comment is closed
```

Evitar:

```text
here
problem
bad
wrong
```

Regras:

- label primária deve ser curta;
- label secundária deve adicionar contexto real;
- não repetir exatamente a mensagem principal;
- não usar label quando o span já for autoexplicativo e a renderização ficar mais ruidosa;
- labels devem apontar spans precisos.

---

## 9. Notas

Notas explicam contexto que não cabe na mensagem principal.

Uso adequado:

- explicar uma limitação do Stage 1;
- indicar que uma construção ainda não faz parte do subconjunto;
- explicar regra relevante da linguagem;
- adicionar contexto para erro interno.

Exemplos:

```text
note: Capi source files must be encoded as UTF-8
note: block comments are not nested in the Stage 1 lexer
```

Notas não devem introduzir uma segunda causa principal do erro.

---

## 10. Help e Sugestões

Use help quando houver ação clara.

Exemplos:

```text
help: close the string with a double quote
help: remove this character or replace it with a valid token
```

Sugestões estruturadas devem ser usadas quando houver substituição objetiva:

```text
replace `:` with `;`
insert `"` here
```

Regras:

- não sugerir correção incerta como se fosse definitiva;
- usar `MaybeIncorrect` quando a sugestão depender de intenção;
- não oferecer múltiplas sugestões contraditórias sem contexto;
- não usar help para repetir a mensagem principal.

---

## 11. Diagnósticos de Fonte

### 11.1 Arquivo Inexistente

Mensagem:

```text
source file not found
```

Nota opcional:

```text
note: path was provided by the command line
```

Regras:

- não exigir span;
- incluir caminho por campo estruturado ou renderização;
- não transformar em internal error.

### 11.2 Falha de Leitura

Mensagem:

```text
could not read source file
```

Nota opcional:

```text
note: operating system error: permission denied
```

Regras:

- preservar erro original como contexto;
- não expor backtrace por padrão;
- classificar como erro de usuário ou ambiente.

### 11.3 UTF-8 Inválido

Mensagem:

```text
source file is not valid UTF-8
```

Help:

```text
help: save the file as UTF-8 and try again
```

Regras:

- não substituir bytes inválidos silenciosamente;
- apontar offset ou região quando disponível;
- não executar lexer nesse arquivo.

---

## 12. Diagnósticos Léxicos

### 12.1 Caractere Inválido

Mensagem:

```text
invalid character in source file
```

Label:

```text
invalid character
```

Help opcional:

```text
help: remove this character or replace it with a valid token
```

### 12.2 String Não Terminada

Mensagem:

```text
unterminated string literal
```

Labels:

```text
string starts here
file ends before the string is closed
```

Help:

```text
help: close the string with a double quote
```

### 12.3 Char Inválido

Mensagens:

```text
empty character literal
character literal contains more than one character
unterminated character literal
```

Regras:

- apontar o literal completo quando possível;
- apontar a região interna quando isso for mais preciso;
- não confundir char inválido com erro de tipo.

### 12.4 Escape Inválido

Mensagem:

```text
invalid escape sequence
```

Label:

```text
unknown escape sequence
```

Help opcional:

```text
help: use one of `\\`, `\"`, `\n`, `\r`, or `\t`
```

### 12.5 Comentário de Bloco Não Terminado

Mensagem:

```text
unterminated block comment
```

Labels:

```text
block comment starts here
file ends before the comment is closed
```

Help:

```text
help: close the block comment with `*/`
```

---

## 13. Erros Internos

Erros internos devem ser explícitos.

Mensagem principal:

```text
internal compiler error
```

Nota obrigatória ou recomendada:

```text
note: lexer produced a token without a valid span
```

Regras:

- usar severidade `InternalError`;
- não culpar o programa do usuário;
- incluir contexto técnico suficiente para depuração;
- evitar detalhes excessivos no modo normal;
- backtrace ou dump interno pertencem a mecanismos próprios.

Entrada inválida comum nunca deve ser reportada como internal compiler error.

---

## 14. Tom

O tom deve ser direto, técnico e neutro.

Preferir:

```text
expected `;` after declaration
```

Evitar:

```text
you forgot a semicolon
obviously invalid declaration
the parser got confused
```

Regras:

- não usar humor;
- não usar linguagem acusatória;
- não usar exclamações;
- não sugerir que o compilador "não entendeu";
- explicar a regra quando necessário.

---

## 15. Pontuação e Formatação

Regras:

- mensagem principal curta não termina com ponto;
- notes e help podem ser frases completas;
- nomes de tokens e lexemas devem usar crase;
- caminhos devem ser renderizados pelo formatador;
- não embutir ANSI no modelo de dados;
- não depender de cor para transmitir significado.

Exemplos:

```text
expected `identifier`
unexpected token `}`
help: insert `;` after this expression
```

---

## 16. Lexemas e Tokens

Ao mencionar texto do código:

- usar o lexema exato entre crases;
- manter escapes legíveis;
- truncar lexemas longos em renderização, não no modelo;
- não normalizar Unicode para mensagem;
- evitar reproduzir strings muito longas na mensagem principal.

Exemplos:

```text
unexpected token `}`
invalid suffix on integer literal `123abc`
```

---

## 17. Caminhos e Localizações

Mensagens não devem incluir localização manualmente.

Preferir estrutura:

```text
error[LEX0001]: invalid character in source file
  --> main.cap:3:12
```

Evitar:

```text
invalid character in main.cap at line 3 column 12
```

Motivo: localização pertence ao renderizador e deve ser derivada do `SourceMap`.

---

## 18. Determinismo

Diagnósticos em testes devem ser estáveis.

Regras:

- normalizar caminhos;
- desabilitar cor em snapshots;
- não depender de ordem de `HashMap`;
- não incluir timestamps;
- não incluir paths temporários completos;
- não incluir mensagens do sistema operacional sem normalização quando forem parte de snapshot.

Quando for necessário preservar erro do sistema operacional, snapshots devem usar fixture controlada ou normalização.

---

## 19. Quantidade de Informação

Um diagnóstico deve ser suficiente, mas não verboso.

Estrutura preferida:

```text
message: causa principal
primary label: região do problema
note: regra relevante, se necessária
help: ação recomendada, se clara
```

Evitar:

- múltiplas notes redundantes;
- explicações longas de semântica em erro léxico;
- sugestões especulativas;
- repetir o mesmo texto na mensagem, label e note.

---

## 20. Responsabilidade por Fase

Cada fase deve emitir diagnósticos apenas sobre sua responsabilidade.

Lexer:

- caracteres inválidos;
- literais lexicalmente inválidos;
- comentários não terminados;
- escapes inválidos;
- tokens inválidos.

Parser:

- tokens inesperados;
- construções sintáticas incompletas;
- recuperação sintática.

Semântica:

- nomes não resolvidos;
- tipos incompatíveis;
- violações de ownership, regiões e Domains.

Uma fase não deve antecipar diagnóstico de fase posterior para produzir mensagem aparentemente melhor.

---

## 21. Exemplos de Saída Inicial

Formato humano conceitual:

```text
error[LEX0001]: invalid character in source file
  --> main.cap:1:5
   |
 1 | let @ = 1;
   |     ^ invalid character
   |
   = help: remove this character or replace it with a valid token
```

String não terminada:

```text
error[LEX0002]: unterminated string literal
  --> main.cap:1:12
   |
 1 | let name = "Gabriel
   |            ^ string starts here
   |
   = help: close the string with a double quote
```

Esses exemplos são orientativos. O formato final pertence a `OUTPUT-FORMATS.md`.

---

## 22. Limitações do Stage 1

No Stage 1, não é obrigatório implementar:

- mensagens internacionalizadas;
- sugestões automáticas para todos os diagnósticos;
- renderização rica com múltiplos arquivos;
- agrupamento por causa raiz;
- explicações longas estilo manual;
- links para documentação;
- formatação LSP;
- política final de cores.

Essas limitações não devem impedir diagnósticos léxicos claros, estruturados e testáveis.

---

## 23. Testes Obrigatórios

Testes de estilo devem cobrir:

- mensagem principal curta;
- severidade renderizada corretamente;
- código renderizado quando disponível;
- span primário com label;
- note;
- help;
- diagnóstico sem span;
- diagnóstico de arquivo inexistente;
- diagnóstico de UTF-8 inválido;
- caractere inválido;
- string não terminada;
- char inválido;
- escape inválido;
- comentário de bloco não terminado;
- internal compiler error distinto;
- snapshot sem cor;
- caminho normalizado.

Esses testes podem ser implementados como snapshots ou UI tests, conforme `LEXER-TESTS.md` e `OUTPUT-FORMATS.md`.

---

## 24. Critérios de Aprovação

Este documento pode ser considerado aprovado quando:

- o idioma inicial das mensagens estiver definido;
- regras de mensagem, labels, notes e help estiverem claras;
- diagnósticos de fonte e lexer do Stage 1 tiverem estilo definido;
- internal compiler errors tiverem tratamento distinto;
- regras de determinismo para snapshots estiverem explícitas;
- limitações do Stage 1 estiverem documentadas;
- os testes obrigatórios forem rastreáveis às entregas do Documento 28.

Após aprovação, diagnósticos emitidos por `capi-source`, `capi-lexer`, `capi-driver` e `capi-cli` devem seguir este guia durante o Stage 1.
