# Planning

Esta pasta reúne a documentação de planejamento, acompanhamento e critérios de
conclusão da implementação oficial da Linguagem Capi.

Ela não define semântica da linguagem nem arquitetura técnica primária. Sua
função é transformar planos, stages, entregas, validações e riscos em registros
objetivos que possam ser revisados.

---

## Documentos aprovados no Stage 0

| Documento | Status | Natureza | Finalidade |
| --- | --- | --- | --- |
| `DEFINITION-OF-DONE.md` | Aprovado | Documento de engenharia bloqueante | Define o que significa uma entrega estar concluída: documentação, código, testes, infraestrutura, ADRs e encerramento de stages. |
| `FEATURE-STATUS.md` | Aprovado | Registro de progresso | Registra o progresso formal do Stage 0, incluindo documentos aprovados, ADRs, infraestrutura, implementação, validações, pendências e riscos. |

---

## Documentos reservados

| Documento | Finalidade esperada |
| --- | --- |
| `IMPLEMENTATION-ORDER.md` | Consolidar a ordem operacional de implementação quando a execução precisar ser detalhada além do Documento 28. |
| `MILESTONES.md` | Registrar marcos demonstráveis, entregas intermediárias e evidências esperadas por milestone. |
| `RISK-REGISTER.md` | Manter riscos técnicos, operacionais e de escopo com mitigação e status. |
| `ROADMAP.md` | Consolidar visão de evolução entre stages sem substituir a especificação. |
| `TECHNICAL-DEBT.md` | Registrar dívidas técnicas aceitas, motivo, impacto, prazo esperado e critério de remoção. |

Enquanto esses documentos estiverem vazios, eles não introduzem obrigações
próprias. As regras aplicáveis vêm de `DEFINITION-OF-DONE.md`,
`FEATURE-STATUS.md`, do Documento 28 e das ADRs aprovadas.

---

## Estado do Stage 0

O Stage 0 está registrado como concluído em:

```text
FEATURE-STATUS.md
```

O registro declara:

* documentos bloqueantes aprovados;
* documentos operacionais e de consolidação aprovados;
* ADRs obrigatórias aprovadas;
* infraestrutura inicial concluída;
* implementação mínima concluída;
* validações executadas;
* pendências não bloqueantes;
* riscos a acompanhar nos próximos stages.

Esse documento é a referência formal para responder se o Stage 0 possui registro
de progresso.

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

Para o Stage 0, isso inclui build, formatação, lint, testes, smoke tests do
`capic`, documentação, dependências e CI.

---

## Relação com o Documento 28

O Documento 28 define o plano de desenvolvimento da implementação oficial e a
ordem de execução por stage.

Esta pasta registra a aplicação desse plano:

* `DEFINITION-OF-DONE.md` define quando uma entrega está pronta;
* `FEATURE-STATUS.md` registra o que foi concluído;
* documentos reservados poderão detalhar roadmap, milestones, riscos e dívidas
  técnicas quando isso passar a ser necessário.

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
5. registre riscos ou dívidas técnicas quando houver limitação aceita.

---

## Leitura recomendada

Para entender o planejamento do projeto, leia nesta ordem:

1. `DEFINITION-OF-DONE.md`
2. `FEATURE-STATUS.md`
3. `../../specification/implementation/28 — Plano de Desenvolvimento da Implementação Oficial.md`
4. `../../adr/README.md`
5. `../build-and-ci/README.md`
6. `../testing/TEST-STRATEGY.md`

Essa ordem parte dos critérios de aceite, passa pelo progresso registrado e
conecta o planejamento às decisões arquiteturais, build e testes.

---

## Critério de atualização

Atualize este README quando:

* um documento reservado desta pasta for preenchido;
* um stage mudar de estado;
* novos registros formais de progresso forem adicionados;
* os critérios de conclusão forem alterados;
* a relação entre planejamento, ADRs, build ou testes mudar.
