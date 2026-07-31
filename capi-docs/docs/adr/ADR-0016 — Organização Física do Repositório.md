# ADR-0016 — Organização Física do Repositório

**Status:** Aprovado  
**Data:** 2026-07-30  
**Stage:** Stage 0 — Fundação do projeto  
**Decisão:** Organizar o repositório raiz em `capi-docs/` e `capi-lang/`.

---

## Contexto

O projeto Capi precisa manter separação clara entre:

- especificação da linguagem;
- documentação de implementação;
- documentos de engenharia;
- ADRs;
- RFCs;
- governança;
- código da implementação oficial;
- testes;
- runtime;
- biblioteca padrão;
- toolchain.

A especificação é fonte normativa.

A implementação oficial é uma materialização concreta dessa especificação.

Misturar documentação normativa e código de implementação no mesmo espaço físico aumentaria o risco de confundir garantias da linguagem com mecanismos temporários da implementação.

---

## Decisão

O repositório raiz será organizado em dois diretórios principais:

```text
capi/
├── capi-docs/
│   └── documentação oficial do projeto
│
└── capi-lang/
    └── implementação oficial da linguagem
```

`capi-docs/` contém:

- especificação da linguagem;
- especificação de implementação;
- documentação de engenharia;
- ADRs;
- RFCs;
- governança;
- templates;
- documentos de planejamento.

`capi-lang/` contém ou conterá:

- workspace Rust;
- compilador;
- runtime;
- biblioteca padrão;
- toolchain;
- testes;
- exemplos;
- scripts;
- arquivos de build.

Configurações de CI do repositório vivem em:

```text
.github/workflows/
```

Elas pertencem à infraestrutura do repositório e podem validar `capi-lang` sem mover a configuração de CI para dentro do workspace Rust.

---

## Justificativa

Essa separação torna explícita a diferença entre:

```text
capi-docs
Fonte documental, normativa, decisória e operacional.

capi-lang
Fonte executável da implementação oficial.
```

A organização facilita:

- rastreabilidade entre especificação, decisões e código;
- evolução documental antes da implementação;
- revisão de decisões arquiteturais;
- criação do workspace Rust sem mover documentação normativa;
- futura migração para auto-hospedagem sem reestruturar a base documental.

---

## Alternativas Consideradas

### Colocar documentação dentro de `capi-lang`

Rejeitada porque faria a documentação normativa parecer subordinada ao código da implementação.

### Colocar código junto da árvore documental

Rejeitada porque misturaria artefatos executáveis, build e testes com a especificação.

### Usar repositórios totalmente separados

Rejeitada para o estágio atual porque adicionaria coordenação desnecessária antes de existir uma implementação funcional.

---

## Consequências Positivas

- Separação clara entre especificação e implementação.
- O repositório permanece legível mesmo antes do compilador existir.
- O Stage 0 pode criar `capi-lang` sem reclassificar documentos.
- ADRs e documentos de engenharia permanecem próximos da especificação.
- A implementação pode evoluir sem transformar código em fonte normativa primária.

---

## Consequências Negativas

- Mudanças de código e documentação podem tocar árvores diferentes.
- Links e referências precisam ser mantidos com disciplina.
- É necessário evitar duplicação de decisões entre `capi-docs` e comentários de código.

---

## Restrições

- Documentos normativos não devem ser movidos para `capi-lang`.
- Código da implementação oficial não deve viver em `capi-docs`, exceto exemplos documentais e templates.
- `capi-lang` deve permanecer subordinado à especificação e às ADRs aprovadas.
- A estrutura física não define a semântica da linguagem.

---

## Critérios de Validação

Esta decisão será considerada operacional quando:

- `capi-docs/` contiver a documentação oficial;
- `capi-lang/` estiver disponível para o workspace da implementação;
- o README raiz explicar o estado dos dois diretórios;
- documentos de engenharia apontarem caminhos de forma consistente;
- a criação do workspace Cargo ocorrer dentro de `capi-lang/`.

## Implementação no Stage 0

A organização física foi materializada com:

```text
capi-docs/
capi-lang/
.github/workflows/capi-lang-ci.yml
```

`capi-lang` contém o workspace Cargo, crates fundamentais, scripts, política de dependências, política de toolchain e README operacional.

`.github/workflows/capi-lang-ci.yml` valida o workspace sem transformar a CI em parte da linguagem ou em contrato público da toolchain Capi.

---

## Referências

- Documento 27 — Bootstrap Plan e Arquitetura da Implementação Oficial
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial
- `PROJECT-STRUCTURE.md`
- `WORKSPACE-ARCHITECTURE.md`
- README raiz do repositório
