# Planning

Esta pasta reúne a documentação de planejamento, acompanhamento e critérios de
conclusão da implementação oficial da Linguagem Capi.

Ela não define semântica da linguagem nem arquitetura técnica primária. Sua
função é transformar planos, stages, entregas, validações e riscos em registros
objetivos que possam ser revisados.

---

## Documentos aprovados

| Documento | Status | Natureza | Finalidade |
| --- | --- | --- | --- |
| `DEFINITION-OF-DONE.md` | Aprovado | Documento de engenharia bloqueante | Define o que significa uma entrega estar concluída: documentação, código, testes, infraestrutura, ADRs e encerramento de stages. |
| `FEATURE-STATUS.md` | Aprovado | Registro de progresso | Registra o progresso formal dos stages, incluindo documentos aprovados, ADRs, infraestrutura, implementação, validações, pendências e riscos. |
| `IMPLEMENTATION-ORDER.md` | Aprovado | Documento de planejamento derivado | Consolida a ordem operacional de implementação dos Stages 0 a 19 e aponta o próximo stage planejado. |
| `MILESTONES.md` | Aprovado | Registro de milestones | Registra marcos demonstráveis, evidências e próximos resultados esperados por stage. |
| `RISK-REGISTER.md` | Aprovado | Registro de riscos | Mantém riscos técnicos, operacionais e de escopo com probabilidade, impacto, status e mitigação. |
| `ROADMAP.md` | Aprovado | Roadmap operacional | Consolida a evolução em horizontes de entrega sem substituir o Documento 28. |
| `TECHNICAL-DEBT.md` | Aprovado | Registro de dívida técnica | Registra dívidas técnicas aceitas, itens monitorados, impacto e condição de remoção. |

---

## Documentos reservados

No estado atual, não há documentos reservados vazios nesta pasta.

Novos documentos de planejamento só devem ser adicionados quando houver
necessidade operacional clara que não seja coberta por `DEFINITION-OF-DONE.md`,
`FEATURE-STATUS.md`, `IMPLEMENTATION-ORDER.md`, `MILESTONES.md`,
`RISK-REGISTER.md`, `ROADMAP.md` ou `TECHNICAL-DEBT.md`.

---

## Estado atual

O progresso formal dos stages está registrado em:

```text
FEATURE-STATUS.md
```

Estado atual:

```text
Stage 0 — Fundação do projeto: Concluído
Stage 1 — Fontes, diagnósticos e lexer: Concluído
Stage 2 — Parser e AST: Concluído
Stage 3 — HIR e resolução de nomes: Concluído
Stage 4 — Sistema de tipos: Concluído
Próximo stage: Stage 5 — Modelo de objetos
```

O registro declara, por stage:

* documentos bloqueantes aprovados;
* documentos operacionais e de consolidação aprovados;
* ADRs obrigatórias aprovadas;
* infraestrutura inicial concluída;
* implementação mínima concluída;
* validações executadas;
* pendências não bloqueantes;
* riscos a acompanhar nos próximos stages.

Esse documento é a referência formal para responder o que foi concluído.

Resultados demonstráveis já registrados:

```text
Stage 0 — capic --help, capic --version
Stage 1 — capic --emit tokens arquivo.capi
Stage 2 — capic --emit ast arquivo.capi
Stage 3 — capic --emit hir arquivo.capi
Stage 4 — capic check arquivo.capi
```

---

## Critério de conclusão

O critério geral de aceite para qualquer entrega fica em:

```text
DEFINITION-OF-DONE.md
```

Uma entrega só deve ser considerada concluída quando puder ser:

* demonstrada;
* revisada;
* validada;
* rastreada até especificação, ADR ou documento de engenharia aplicável;
* reproduzida por outra pessoa.

Para os stages concluídos, isso inclui build, formatação, lint, testes, smoke
tests do `capic`, documentação, dependências, CI e resultado demonstrável do
stage.

---

## Relação com o Documento 28

O Documento 28 define o plano de desenvolvimento da implementação oficial e a
ordem de execução por stage.

Esta pasta registra a aplicação desse plano:

* `DEFINITION-OF-DONE.md` define quando uma entrega está pronta;
* `FEATURE-STATUS.md` registra o que foi concluído;
* `IMPLEMENTATION-ORDER.md` consolida a ordem operacional de execução;
* `MILESTONES.md` registra marcos demonstráveis;
* `RISK-REGISTER.md` registra riscos e mitigação;
* `ROADMAP.md` organiza a evolução por horizontes;
* `TECHNICAL-DEBT.md` registra dívidas aceitas e itens monitorados.

O planejamento nesta pasta não deve substituir o Documento 28. Quando houver
conflito, o plano aprovado da especificação e as ADRs aplicáveis prevalecem.

---

## Uso prático

Antes de marcar uma tarefa como concluída:

1. verifique os critérios aplicáveis em `DEFINITION-OF-DONE.md`;
2. execute as validações necessárias;
3. atualize ADRs ou documentos quando a entrega introduzir decisão nova;
4. registre progresso em `FEATURE-STATUS.md` quando a entrega afetar o status do
   stage;
5. atualize `MILESTONES.md` quando houver novo resultado demonstrável;
6. registre riscos em `RISK-REGISTER.md`;
7. registre dívidas técnicas em `TECHNICAL-DEBT.md` quando houver limitação
   aceita.

---

## Leitura recomendada

Para entender o planejamento do projeto, leia nesta ordem:

1. `DEFINITION-OF-DONE.md`
2. `FEATURE-STATUS.md`
3. `IMPLEMENTATION-ORDER.md`
4. `MILESTONES.md`
5. `ROADMAP.md`
6. `RISK-REGISTER.md`
7. `TECHNICAL-DEBT.md`
8. `../../specification/implementation/28 — Plano de Desenvolvimento da Implementação Oficial.md`
9. `../../adr/README.md`
10. `../build-and-ci/README.md`
11. `../testing/TEST-STRATEGY.md`

Essa ordem parte dos critérios de aceite, passa pelo progresso registrado e
conecta o planejamento a milestones, roadmap, riscos, dívidas, decisões
arquiteturais, build e testes.

---

## Critério de atualização

Atualize este README quando:

* um documento desta pasta for criado, preenchido ou aprovado;
* um stage mudar de estado;
* novos registros formais de progresso forem adicionados;
* os critérios de conclusão forem alterados;
* a relação entre planejamento, ADRs, build ou testes mudar.
