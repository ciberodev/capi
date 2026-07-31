# Coding Standards

**Projeto:** Linguagem Capi  
**Documento:** CODING-STANDARDS  
**Status:** Aprovado  
**Stage:** Stage 0 — Fundação do projeto  
**Natureza:** Documento de consolidação  
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define os padrões gerais de código para a implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer critérios práticos para escrever, revisar e manter código do compilador, runtime, biblioteca padrão, toolchain, testes e automações.

Este documento não substitui:

- `ENGINEERING-PRINCIPLES.md`;
- `DEPENDENCY-RULES.md`;
- `RUST-STYLE-GUIDE.md`;
- `UNSAFE-POLICY.md`;
- documentos específicos de cada componente.

Quando houver conflito, prevalecem a especificação, ADRs aprovados e documentos especializados aplicáveis.

---

## 2. Escopo

Este documento cobre:

- clareza de código;
- organização de módulos;
- fronteiras entre componentes;
- APIs internas;
- tratamento de erros;
- diagnósticos;
- testes;
- documentação no código;
- dependências;
- uso de `unsafe`;
- critérios de revisão.

Este documento não cobre:

- sintaxe da linguagem Capi;
- algoritmos detalhados de compilação;
- formato completo de AST, HIR ou MIR;
- convenções completas de estilo Rust;
- política completa de contribuição ou commits.

---

## 3. Princípios Gerais

Código da implementação oficial deve ser:

- rastreável até especificação, documento de engenharia ou ADR;
- simples antes de genérico;
- explícito em entradas, saídas e invariantes;
- testável;
- determinístico quando produzir saída observável;
- organizado por responsabilidade;
- resistente a entradas inválidas;
- compatível com evolução incremental.

Código não deve:

- redefinir semântica da linguagem;
- misturar responsabilidades de camadas diferentes;
- criar dependências circulares;
- esconder decisões arquiteturais em detalhes locais;
- depender de comportamento acidental de crates externos;
- transformar workaround de bootstrap em contrato permanente.

---

## 4. Fonte de Verdade

A ordem de autoridade é:

```text
1. Especificação da linguagem e da implementação
2. ADRs aprovados
3. Documentos de engenharia
4. Testes de conformidade e regressão
5. Código
```

Código pode revelar lacunas na documentação, mas não deve resolver sozinho uma ambiguidade normativa.

Quando a implementação exigir decisão nova:

- registre a decisão no documento apropriado;
- abra ADR quando a decisão afetar arquitetura, dependências, segurança, bootstrap ou fronteiras de componentes;
- adicione teste quando houver comportamento observável.

---

## 5. Organização do Código

O código deve refletir a arquitetura documentada.

Regras:

- cada crate deve ter responsabilidade clara;
- cada módulo deve ter propósito local identificável;
- módulos não devem virar depósitos genéricos;
- estruturas compartilhadas devem viver no crate mais fundamental adequado;
- crates binários devem ser finos;
- lógica de compilação deve viver em crates de biblioteca;
- backend não deve vazar para frontend;
- runtime não deve redefinir regra semântica.

Se dois componentes precisam compartilhar lógica, a solução preferida é extrair contrato comum, não criar dependência reversa.

---

## 6. APIs Internas

APIs internas devem ser desenhadas para expressar contratos reais.

Uma API deve:

- declarar tipos de entrada e saída claros;
- evitar estado global implícito;
- distinguir erro de usuário de erro interno;
- preservar spans e rastreabilidade quando aplicável;
- evitar expor detalhes privados desnecessários;
- tornar estados inválidos difíceis de representar.

Uma API não deve:

- retornar strings soltas para erros estruturados;
- exigir ordem oculta de chamadas sem documentação;
- aceitar parâmetros genéricos demais por conveniência;
- depender de `panic` para fluxo esperado;
- expor estruturas mutáveis amplas sem necessidade.

---

## 7. Nomes

Nomes devem privilegiar precisão.

Regras:

- use nomes do glossário quando existirem;
- preserve siglas oficiais como AST, HIR, MIR, ABI e FFI;
- preserve entidades da linguagem como `Domain` e `ObjectId`;
- nomes de crates devem seguir a responsabilidade do componente;
- nomes de funções devem descrever ação ou consulta;
- nomes de tipos devem descrever entidade ou contrato.

Evite:

- abreviações locais sem valor;
- nomes genéricos como `Data`, `Info`, `Manager` ou `Helper` sem qualificação forte;
- nomes que confundam mecanismo de implementação com garantia da linguagem.

---

## 8. Representações Internas

Representações internas relevantes devem ser explícitas.

Preferir:

- IDs tipados;
- enums para estados fechados;
- structs pequenas com responsabilidade clara;
- tipos novos para conceitos distintos;
- invariantes preservadas por construtores;
- coleções com ordem determinística quando a saída for observável.

Evitar:

- uso de `usize` cru para identidades lógicas;
- strings como substituto para símbolos estruturados;
- mapas com ordem não determinística em dumps ou snapshots;
- ponteiros ou referências como identidade lógica;
- estados parcialmente inicializados expostos.

---

## 9. Tratamento de Erros

Erros devem ser classificados corretamente.

Categorias:

```text
Erro do usuário
Entrada, programa ou uso inválido da ferramenta.

Erro interno do compilador
Violação de invariante interna ou bug da implementação.

Erro de ambiente
Falha de filesystem, toolchain externa, linker ou plataforma.
```

Regras:

- erro esperado de usuário deve gerar diagnóstico;
- erro interno deve ser distinguível e rastreável;
- `panic` não deve ser usado para erro esperado;
- mensagens devem carregar contexto suficiente;
- perda de span deve ser evitada quando o erro pertence ao código-fonte;
- falhas de IO devem preservar causa.

---

## 10. Diagnósticos

Diagnósticos são contrato de experiência.

Código que cria diagnósticos deve:

- usar a infraestrutura de diagnósticos;
- preservar severidade;
- associar spans quando disponíveis;
- evitar imprimir diretamente em `stdout` ou `stderr`;
- manter mensagens determinísticas;
- ser testável por snapshots ou UI tests quando aplicável.

O CLI pode renderizar diagnósticos.

Fases internas devem produzir dados estruturados.

---

## 11. Estado e Mutabilidade

Estado mutável deve ser localizado e justificado.

Preferir:

- estruturas imutáveis após construção;
- builders quando montagem gradual for necessária;
- mutabilidade confinada a uma fase;
- passagem explícita de contexto;
- retorno de nova representação após transformação.

Evitar:

- estado global mutável;
- caches invisíveis que alterem comportamento observável;
- mutação compartilhada entre fases;
- dependência de ordem de execução não documentada.

---

## 12. Determinismo

Saídas observáveis devem ser determinísticas.

Isso inclui:

- diagnósticos;
- dumps;
- snapshots;
- ordem de testes;
- artefatos de build quando viável;
- listagens de símbolos;
- serializações internas.

Quando uma estrutura não preservar ordem, a saída deve ordenar explicitamente antes de renderizar ou comparar.

---

## 13. Dependências

Toda dependência deve ter finalidade clara.

Antes de adicionar dependência:

- verifique se ela respeita `DEPENDENCY-RULES.md`;
- confirme licença compatível;
- avalie dependências transitivas;
- avalie impacto em MSRV;
- confirme que ela não cruza fronteiras arquiteturais;
- registre a justificativa quando a dependência for estrutural.

Dependência externa não deve substituir entendimento do problema central do compilador.

---

## 14. `unsafe`

Código `unsafe` deve ser raro, localizado e documentado.

Todo uso de `unsafe` deve declarar:

- qual invariante preserva;
- por que código seguro não é suficiente;
- qual superfície fica exposta;
- como o comportamento é testado;
- por que a escolha não vaza para código seguro.

`unsafe` não deve ser usado por conveniência, micro-otimização prematura ou para contornar o type checker de Rust sem justificativa.

---

## 15. Comentários

Comentários devem explicar intenção, invariante ou contexto.

Use comentários para:

- invariantes não óbvias;
- decisões locais com trade-off;
- relação com especificação ou ADR;
- motivo de ordem de execução;
- justificativa de `unsafe`;
- normalizações necessárias para determinismo.

Evite comentários que apenas repitam o código.

Código claro é preferível a comentário compensando nome ruim.

---

## 16. Documentação no Código

APIs públicas de crates internos devem ter documentação suficiente para revisão e uso correto.

Documentação deve explicar:

- responsabilidade do tipo ou função;
- invariantes relevantes;
- significado de erros;
- relação com spans ou diagnósticos;
- pré-condições quando existirem.

Documentação interna não deve prometer estabilidade pública além do que o projeto aprovou.

---

## 17. Testes

Toda mudança de comportamento deve ter teste correspondente ou justificativa registrada.

Critério prático:

- comportamento local: teste unitário;
- contrato entre crates: teste de integração;
- saída de CLI ou diagnóstico: UI test ou snapshot;
- programa aceito: `compile-pass`;
- programa rejeitado: `compile-fail`;
- programa executado: `run-pass`;
- bug corrigido: teste de regressão.

Testes devem ser determinísticos e independentes de estado global.

---

## 18. Código Temporário

Código temporário é permitido apenas quando:

- desbloqueia avanço incremental;
- tem escopo limitado;
- não altera contrato público;
- está documentado;
- possui caminho claro de remoção ou substituição.

Marcadores como `TODO` devem indicar motivo e etapa esperada, não apenas intenção vaga.

Exemplo aceitável:

```text
TODO(stage-2): substituir parser provisório pelo parser real definido em PARSER-IMPLEMENTATION.md.
```

Exemplo ruim:

```text
TODO: melhorar isso.
```

---

## 19. Performance

Performance importa, mas não deve antecipar complexidade sem evidência.

Antes de otimizar:

- preserve corretude;
- adicione teste ou benchmark quando aplicável;
- identifique o gargalo;
- mantenha a otimização local;
- documente trade-offs relevantes.

Otimização não pode alterar semântica observável.

---

## 20. Segurança

Código deve tratar entradas externas como não confiáveis.

Inclui:

- arquivos de usuário;
- argumentos de CLI;
- manifests futuros;
- dependências externas;
- artefatos de build;
- dados vindos de FFI.

Entradas inválidas devem produzir erro controlado.

O compilador não deve assumir que programas Capi inválidos são raros ou benignos.

---

## 21. Revisão de Código

Uma revisão deve verificar:

- aderência à especificação;
- aderência à arquitetura;
- dependências novas ou alteradas;
- testes;
- tratamento de erros;
- determinismo;
- clareza de APIs;
- preservação de fronteiras entre crates;
- ausência de `unsafe` injustificado;
- ausência de comportamento temporário sem registro.

Revisão não deve se limitar a estilo.

---

## 22. Comandos Obrigatórios

Antes de considerar uma mudança pronta, execute quando o workspace existir:

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

Se o workspace ainda não existir, esses comandos não são aplicáveis e a validação é documental.

---

## 23. Critérios de Aceitação do Documento

Este documento é considerado preenchido quando:

- define padrões gerais para código da implementação oficial;
- preserva a separação entre garantias e mecanismos;
- cobre organização, APIs, erros, diagnósticos, testes, dependências e `unsafe`;
- deixa detalhes finos de Rust para `RUST-STYLE-GUIDE.md`;
- não contradiz `ENGINEERING-PRINCIPLES.md`, `DEPENDENCY-RULES.md`, `TEST-STRATEGY.md` ou `BUILD-SYSTEM.md`.

---

## 24. Síntese

Código da Capi deve ser claro, rastreável, testável e fiel à arquitetura.

O objetivo não é apenas fazer o compilador funcionar, mas garantir que cada incremento preserve a especificação, as fronteiras entre componentes e a capacidade de evolução até bootstrap e auto-hospedagem.
