# Engineering Principles

**Projeto:** Linguagem Capi
**Documento:** ENGINEERING-PRINCIPLES
**Status:** Proposto
**Stage:** Stage 0 — Fundação do projeto
**Natureza:** Documento de engenharia bloqueante
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define os princípios de engenharia que orientam a implementação oficial da Linguagem Capi.

Seu objetivo é estabelecer critérios técnicos permanentes para a tomada de decisão durante o desenvolvimento do compilador, runtime, biblioteca padrão, toolchain, testes, bootstrap e auto-hospedagem.

Este documento não redefine a linguagem Capi.

Ele orienta como a implementação oficial deve ser construída para preservar os contratos definidos pela especificação.

---

## 2. Escopo

Este documento se aplica a:

* código da implementação oficial;
* organização dos crates;
* documentação de engenharia;
* testes;
* diagnósticos;
* runtime;
* biblioteca padrão;
* toolchain;
* backends;
* automação de build;
* bootstrap;
* contribuições assistidas por IA.

Este documento não substitui:

* a especificação da linguagem;
* os documentos de implementação;
* ADRs;
* RFCs;
* documentos especializados de cada componente.

Quando houver conflito, prevalecem os documentos normativos da especificação e os ADRs aprovados aplicáveis.

---

## 3. Princípio da separação entre garantias e mecanismos

A implementação oficial deve preservar uma separação rigorosa entre:

```text
garantias da linguagem
mecanismos da implementação
```

Garantias pertencem à linguagem Capi e são definidas pela especificação.

Mecanismos pertencem à implementação oficial e podem evoluir desde que preservem as garantias observáveis.

Consequências práticas:

* uma regra da Capi não deve ser derivada implicitamente de uma limitação do Rust;
* uma otimização não deve alterar comportamento observável;
* uma estrutura interna não deve ser tratada como contrato público sem aprovação;
* decisões temporárias de bootstrap não devem contaminar a especificação;
* Cranelift, LLVM, Cargo, Rust e bibliotecas auxiliares são mecanismos, não definições da linguagem.

---

## 4. Princípio da especificação como fonte de verdade

Toda implementação deve ser rastreável até uma definição aplicável.

Antes de implementar uma funcionalidade, deve estar claro:

* qual contrato da especificação está sendo materializado;
* qual documento de engenharia detalha a solução;
* quais testes validam o comportamento;
* quais limitações temporárias existem no subconjunto implementado;
* quais decisões arquiteturais exigem ADR.

Código não é fonte normativa primária.

Quando o código revelar uma lacuna, ambiguidade ou conflito na documentação, a decisão deve ser registrada antes de ser tratada como comportamento oficial.

---

## 5. Princípio da evolução incremental

A implementação oficial deve evoluir em incrementos pequenos, verificáveis e demonstráveis.

Cada incremento deve preservar a capacidade do projeto de:

* compilar;
* executar testes;
* emitir diagnósticos compreensíveis;
* reproduzir falhas;
* demonstrar o resultado esperado do stage em andamento.

O desenvolvimento deve favorecer cortes verticais controlados quando eles reduzirem risco técnico, desde que permaneçam rastreáveis e limitados ao escopo aprovado.

Nenhum stage deve ser considerado concluído apenas pela existência de código.

---

## 6. Princípio da arquitetura em camadas

A implementação deve preservar fronteiras claras entre:

```text
fonte
lexer
parser
AST
HIR
análises semânticas
MIR
backend
runtime
toolchain
```

Cada componente deve depender apenas dos contratos necessários ao seu papel.

Dependências entre camadas devem ser explícitas, justificadas e testáveis.

Regras obrigatórias:

* o frontend não deve depender de Cranelift ou LLVM;
* a MIR deve permanecer independente de backend;
* diagnósticos devem atravessar o pipeline por modelos estruturados;
* o runtime não deve redefinir semântica da linguagem;
* a toolchain deve orquestrar componentes sem absorver responsabilidades internas do compilador.

---

## 7. Princípio da representação explícita

Entidades internas relevantes devem possuir representação explícita, estável dentro de seu escopo e adequada para depuração.

Isso inclui, quando aplicável:

* IDs internos tipados;
* spans;
* símbolos;
* tipos;
* entidades HIR;
* blocos MIR;
* objetos;
* Domains;
* artefatos de build.

Representações internas devem evitar dependência acidental de ponteiros, referências Rust, ordem de alocação ou detalhes temporários do backend.

Quando uma identidade lógica for necessária, ela deve ser modelada diretamente.

---

## 8. Princípio da segurança por construção

A implementação deve favorecer estruturas que tornem estados inválidos difíceis ou impossíveis de representar.

Esse princípio se aplica especialmente a:

* ownership;
* borrowing;
* regiões;
* Domains;
* ObjectId;
* mutabilidade;
* inicialização;
* descarte;
* FFI;
* código `unsafe`.

Código `unsafe` deve ser localizado, documentado, revisado e testado.

Nenhum uso de `unsafe` deve ser aceito apenas por conveniência.

Todo `unsafe` deve declarar:

* qual invariante precisa preservar;
* por que a alternativa segura é insuficiente;
* como o comportamento é testado;
* quais limites impedem vazamento de invariantes para código seguro.

---

## 9. Princípio dos diagnósticos como contrato de experiência

Diagnósticos não são detalhe secundário da implementação.

Eles fazem parte da experiência de desenvolvimento e devem ser tratados como saída estruturada do compilador.

Diagnósticos devem ser:

* precisos;
* determinísticos;
* rastreáveis até spans;
* adequados para humanos;
* adequados para ferramentas;
* testáveis por snapshots, UI tests ou mecanismos equivalentes.

Falhas internas devem ser distinguíveis de erros do programa do usuário.

Entradas inválidas não devem causar pânico não controlado.

---

## 10. Princípio de testes desde o primeiro stage

Testes devem acompanhar a implementação desde o Stage 0.

Cada componente deve possuir testes compatíveis com seu risco e responsabilidade.

A estratégia mínima inclui:

* testes unitários para estruturas e algoritmos locais;
* testes de integração para fronteiras entre componentes;
* testes positivos para programas válidos;
* testes negativos para programas inválidos;
* testes de regressão para falhas corrigidas;
* testes de execução quando houver código gerado;
* testes diferenciais quando houver duas implementações comparáveis;
* testes de conformidade conforme a implementação amadurecer.

Nenhuma funcionalidade deve ser considerada concluída sem teste correspondente ou justificativa formal.

---

## 11. Princípio da reprodutibilidade

Resultados importantes devem ser reproduzíveis sempre que tecnicamente possível.

Isso inclui:

* builds;
* testes;
* snapshots;
* dumps de AST, HIR e MIR;
* diagnósticos;
* artefatos de bootstrap;
* comparações entre estágios;
* benchmarks.

Quando reprodução binária exata ainda não for viável, a reprodução funcional deve ser preservada.

Ferramentas, versões mínimas, dependências e ambiente suportado devem ser documentados.

---

## 12. Princípio do isolamento de dependências externas

Dependências externas devem ser adotadas com finalidade clara.

Uma dependência é aceitável quando:

* reduz risco real;
* evita reimplementar infraestrutura não essencial;
* possui licença compatível;
* é mantida de forma adequada;
* pode ser isolada atrás de uma interface interna;
* não redefine contratos da linguagem.

Dependências estruturais devem ser registradas por ADR.

Cranelift e LLVM devem permanecer isolados nos respectivos backends.

A troca, coexistência ou remoção de backend não deve exigir reescrita do frontend ou da MIR.

---

## 13. Princípio da estabilidade progressiva

Nem toda API interna nasce estável.

Durante os stages iniciais, contratos internos podem evoluir conforme a implementação revela necessidades reais.

Entretanto, essa evolução deve ser controlada:

* mudanças incompatíveis devem ser explícitas;
* consumidores internos devem ser atualizados juntos;
* testes devem acompanhar a mudança;
* documentos devem permanecer coerentes com o código;
* limitações temporárias devem ser registradas.

Estabilidade interna de stage não significa estabilidade pública permanente.

---

## 14. Princípio do bootstrap responsável

Rust é a linguagem hospedeira inicial da implementação oficial.

Essa decisão pertence ao processo de bootstrap e não altera a definição da linguagem Capi.

Durante o bootstrap:

* a implementação em Rust deve permanecer recuperável;
* componentes migrados para Capi devem ser comparáveis aos equivalentes em Rust quando aplicável;
* a cadeia de compilação deve ser documentada;
* artefatos relevantes devem possuir proveniência;
* regressões introduzidas pela migração devem bloquear avanço;
* a dependência operacional de Rust deve diminuir de forma progressiva.

Auto-hospedagem é um marco de maturidade da implementação oficial, não uma mudança semântica da linguagem.

---

## 15. Princípio de performance orientada por evidência

Desempenho importa, mas não deve justificar violação de contratos.

Durante o bootstrap inicial, corretude, clareza arquitetural e verificabilidade prevalecem sobre otimização prematura.

Otimizações devem:

* preservar comportamento observável;
* possuir teste ou benchmark quando relevante;
* ser mensuráveis;
* não acoplar fases anteriores a backends específicos;
* ser removíveis ou ajustáveis se se mostrarem prejudiciais.

Regressões relevantes devem ser registradas e tratadas conforme política própria.

---

## 16. Princípio da simplicidade operacional

O projeto deve ser simples de construir, testar e depurar no ambiente suportado.

Comandos essenciais devem ser documentados e automatizados quando possível.

O Stage 0 deve garantir, no mínimo, que seja possível:

```bash
cargo test
capic --help
capic --version
```

Scripts devem encapsular fluxos repetitivos sem esconder decisões importantes.

Falhas de build e teste devem produzir informação suficiente para reprodução.

---

## 17. Princípio de documentação viva

Documentação de engenharia deve acompanhar a implementação.

Um documento pode começar incompleto quando classificado como operacional ou de consolidação, mas não deve permanecer incoerente com o código.

Documentos bloqueantes devem estar aprovados antes da implementação correspondente.

Mudanças de comportamento, arquitetura ou processo devem atualizar:

* documentação aplicável;
* ADRs, quando necessário;
* testes;
* registros de progresso;
* limitações conhecidas.

---

## 18. Princípio de contribuições assistidas por IA

Ferramentas de inteligência artificial podem auxiliar na escrita de documentos, código, testes e revisões.

Entregas assistidas por IA devem seguir os mesmos critérios das demais contribuições.

Elas devem ser:

* revisadas por responsável humano;
* rastreáveis;
* compatíveis com licenciamento e dependências;
* testadas;
* coerentes com a especificação;
* tratadas como proposta até validação.

Nenhuma decisão deve ser considerada correta apenas por ter sido gerada automaticamente.

---

## 19. Critérios de aceite deste documento

Este documento estará pronto para orientar o Stage 0 quando:

* seus princípios estiverem coerentes com os Documentos 00 a 28;
* não redefinir contratos da linguagem;
* puder orientar `PROJECT-STRUCTURE.md`, `COMPILER-ARCHITECTURE.md`, `WORKSPACE-ARCHITECTURE.md`, `DEPENDENCY-RULES.md`, `TEST-STRATEGY.md` e `DEFINITION-OF-DONE.md`;
* estabelecer critérios claros para decisões iniciais de engenharia;
* for aprovado conforme o processo documental do projeto.

---

## 20. Síntese

A implementação oficial da Capi deve ser construída de forma incremental, rastreável, testável e fiel à especificação.

O objetivo central da engenharia do projeto é transformar os contratos da linguagem em software funcional sem confundir mecanismos temporários com garantias permanentes.
