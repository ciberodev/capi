# ADR-0013 — Política de Dependências Externas

**Status:** Aprovado  
**Data:** 2026-07-30  
**Stage:** Stage 0 — Fundação do projeto  
**Decisão:** Adotar política restritiva e explícita para dependências externas da implementação oficial.

---

## Contexto

A implementação oficial da Capi usará Rust e Cargo no bootstrap.

Isso torna natural a adoção de crates externos, ferramentas de desenvolvimento e dependências estruturais como Cranelift e, futuramente, LLVM.

Ao mesmo tempo, dependências externas afetam:

- segurança;
- licenciamento;
- reprodutibilidade;
- tempo de build;
- manutenção;
- MSRV;
- CI;
- bootstrap;
- portabilidade;
- sustentabilidade de longo prazo.

O Documento 27 exige dependências externas reduzidas e encapsuladas. O Documento 28 exige política inicial de dependências no Stage 0.

---

## Decisão

Dependências externas só serão adotadas quando houver finalidade clara, benefício técnico suficiente e compatibilidade com a arquitetura.

Toda dependência deve ser avaliada quanto a:

- finalidade;
- licença;
- manutenção;
- estabilidade;
- segurança;
- dependências transitivas;
- impacto em build e CI;
- impacto em MSRV;
- possibilidade de substituição;
- aderência às fronteiras arquiteturais.

Dependências estruturais devem permanecer isoladas.

Em particular:

- Cranelift deve ficar isolado no backend Cranelift;
- LLVM deve ficar isolado no backend LLVM quando for introduzido;
- bibliotecas de CLI não podem definir o driver;
- bibliotecas de diagnóstico não podem definir o modelo semântico de diagnóstico;
- bibliotecas de parsing, se usadas, não podem definir a AST oficial.

`Cargo.lock` deve ser versionado no workspace da implementação.

No Stage 0, a implementação oficial começa sem dependências externas Rust.

Apenas dependências internas de workspace por `path` são permitidas neste momento.

---

## Justificativa

Dependências aceleram implementação, mas também criam custo permanente.

Uma política restritiva evita:

- acoplamento acidental;
- vazamento de mecanismos externos para a linguagem;
- aumento descontrolado do grafo de dependências;
- dificuldade de auditoria;
- regressões de build;
- incompatibilidade com bootstrap e auto-hospedagem.

Essa decisão preserva a neutralidade tecnológica da linguagem e mantém ferramentas externas como mecanismos substituíveis.

---

## Alternativas Consideradas

### Permitir dependências livremente

Rejeitada porque aumentaria rapidamente complexidade, risco de supply chain e acoplamento a APIs externas.

### Proibir dependências externas

Rejeitada porque inviabilizaria ou atrasaria desnecessariamente componentes deliberadamente escolhidos, como Cranelift, LLVM e ferramentas essenciais de desenvolvimento.

### Vendoring obrigatório desde o Stage 0

Rejeitada como regra inicial por aumentar custo operacional antes da existência do workspace funcional.

Pode ser reconsiderada para dependências críticas ou release.

---

## Consequências Positivas

- O grafo de dependências permanece revisável.
- Dependências críticas ficam encapsuladas.
- O bootstrap fica mais reprodutível.
- O risco de vazamento semântico de bibliotecas externas diminui.
- A manutenção de longo prazo fica mais previsível.

---

## Consequências Negativas

- Adicionar dependências exigirá justificativa explícita.
- Algumas implementações simples podem exigir mais código próprio.
- Revisões precisarão considerar licenças e dependências transitivas.
- O projeto precisará manter documentação atualizada sobre dependências aceitas.

---

## Restrições

- Dependências não podem redefinir regras da linguagem.
- Dependências de backend não podem vazar para frontend ou middle-end.
- Dependências transitivas devem ser revisáveis.
- Dependências com licença incompatível são proibidas.
- Dependências abandonadas ou instáveis devem ser evitadas.
- Mudanças relevantes em dependências devem passar por revisão.

---

## Critérios de Validação

Esta decisão será considerada operacional quando:

- `DEPENDENCY-RULES.md` estiver aprovado;
- o workspace versionar `Cargo.lock`;
- novas dependências forem justificadas em revisão;
- dependências estruturais estiverem isoladas nos componentes corretos;
- CI validar o workspace com o grafo de dependências resolvido.

## Implementação no Stage 0

A política inicial foi registrada em:

- `capi-lang/DEPENDENCIES.md`;
- `capi-lang/scripts/deps.sh`.

O script de validação rejeita dependências externas no Stage 0 ao verificar ausência de entradas `source =` em `Cargo.lock`.

O grafo atual contém apenas crates internos:

```text
capi-cli -> capi-driver
capi-driver -> capi-common
capi-driver -> capi-diagnostics
capi-driver -> capi-session
capi-session -> capi-diagnostics
capi-session -> capi-source
capi-source -> capi-common
capi-diagnostics -> capi-common
```

Qualquer dependência externa futura exige atualização da política operacional e, se for estrutural, ADR própria ou atualização de ADR aplicável.

---

## Referências

- Documento 12 — Ecossistema e Gerenciamento de Pacotes
- Documento 27 — Bootstrap Plan e Arquitetura da Implementação Oficial
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial
- `DEPENDENCY-RULES.md`
- `BUILD-SYSTEM.md`
- `RUST-STYLE-GUIDE.md`
- `SUPPLY-CHAIN-SECURITY.md`
