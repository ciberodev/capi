# Dependency Rules

**Projeto:** Linguagem Capi
**Documento:** DEPENDENCY-RULES
**Status:** Proposto
**Stage:** Stage 0 — Fundação do projeto
**Natureza:** Documento de engenharia bloqueante
**Base normativa:** Documentos de especificação 00 a 28

---

## 1. Finalidade

Este documento define as regras de dependência da implementação oficial da Linguagem Capi.

Seu objetivo é controlar dependências entre crates, componentes, fases do compilador e bibliotecas externas, preservando modularidade, reprodutibilidade, licenciamento, segurança e evolução incremental.

Este documento orienta a criação do workspace Rust no Stage 0.

---

## 2. Escopo

Este documento cobre:

* dependências internas entre crates;
* dependências entre componentes arquiteturais;
* dependências externas do ecossistema Rust;
* dependências estruturais, como Cranelift e LLVM;
* uso de `Cargo.lock`;
* compatibilidade com MSRV;
* critérios de aprovação, revisão e remoção de dependências;
* regras para features Cargo;
* regras para dependências de teste, build e desenvolvimento.

Este documento não cobre:

* resolução de pacotes Capi;
* lockfile da futura toolchain Capi;
* política completa de supply chain;
* auditoria de vulnerabilidades em detalhe;
* APIs internas de cada crate.

---

## 3. Princípios

Dependências devem seguir `ENGINEERING-PRINCIPLES.md`.

Regras centrais:

* dependência é custo permanente até prova em contrário;
* dependências devem ter finalidade explícita;
* dependências internas devem formar grafo acíclico;
* dependências externas devem possuir licença compatível;
* dependências transitivas devem ser revisáveis;
* dependências de backend devem permanecer isoladas;
* dependências não podem redefinir semântica da linguagem;
* dependências não podem substituir validações exigidas pela especificação.

Cargo, Rust, Cranelift e LLVM são mecanismos da implementação oficial.

Eles não fazem parte da definição da linguagem Capi.

---

## 4. Tipos de dependência

Este documento distingue:

```text
Dependências internas
Crates e módulos pertencentes ao workspace capi-lang.

Dependências externas
Crates, bibliotecas, ferramentas ou componentes externos ao repositório.

Dependências estruturais
Dependências externas que afetam arquitetura, backend, runtime, bootstrap ou build.

Dependências de desenvolvimento
Ferramentas ou crates usados apenas para testes, geração, lint, benchmark ou CI.

Dependências de plataforma
Bibliotecas ou ferramentas necessárias para targets específicos, linking ou execução.
```

Cada tipo possui regras próprias de aprovação e isolamento.

---

## 5. Regras para dependências internas

Dependências internas devem refletir a arquitetura documentada.

Regras obrigatórias:

* o grafo deve ser acíclico;
* dependências devem apontar para componentes mais fundamentais ou fases anteriores;
* crates binários podem depender de bibliotecas, mas bibliotecas não dependem de crates binários;
* infraestrutura compartilhada não depende de fases avançadas;
* frontend não depende de backend;
* MIR não depende de Cranelift ou LLVM;
* runtime não depende de detalhes privados do compilador;
* toolchain não depende de detalhes privados de AST, HIR ou MIR.

Quando dois componentes precisarem compartilhar uma estrutura, a estrutura deve ser movida para componente comum adequado.

Não é permitido criar dependência reversa apenas para reutilizar uma função local.

---

## 6. Grafo inicial do Stage 0

O Stage 0 deve iniciar com o menor grafo útil.

Crates mínimos:

```text
capi-cli
capi-driver
capi-session
capi-diagnostics
capi-source
capi-common
```

Direção permitida:

```text
capi-cli
    ↓
capi-driver
    ↓
capi-session
    ↓
capi-source
    ↓
capi-common

capi-driver
    ↓
capi-diagnostics
    ↓
capi-common

capi-session
    ↓
capi-diagnostics
    ↓
capi-common
```

Regras adicionais:

* `capi-common` não depende dos demais crates do compilador;
* `capi-cli` não é dependência de nenhum crate;
* `capi-driver` coordena, mas não implementa fases;
* `capi-session` não chama o pipeline;
* `capi-source` não depende de `capi-driver`;
* `capi-diagnostics` não depende de backend.

Qualquer desvio deve ser documentado e revisado.

---

## 7. Dependências por camada

### 7.1 CLI

Pode depender de:

* driver;
* biblioteca de parsing de CLI aprovada;
* diagnósticos via driver ou API documentada.

Não pode depender de:

* lexer;
* parser;
* HIR;
* MIR;
* backends concretos.

### 7.2 Driver

Pode depender de:

* sessão;
* diagnósticos;
* fases disponíveis do pipeline;
* interface de backend quando o stage exigir.

Não pode depender de:

* detalhes privados de fases;
* backends concretos antes da seleção por interface;
* runtime privado.

### 7.3 Infraestrutura compartilhada

Pode depender de:

* `capi-common`;
* dependências externas pequenas e aprovadas.

Não pode depender de:

* parser;
* type checker;
* MIR;
* backend;
* runtime.

### 7.4 Frontend

Pode depender de:

* fontes;
* diagnósticos;
* sessão;
* estruturas de fases anteriores;
* componentes semânticos necessários.

Não pode depender de:

* Cranelift;
* LLVM;
* linker;
* runtime privado;
* toolchain pública.

### 7.5 Middle-end

Pode depender de:

* HIR validada;
* diagnósticos;
* tipos e informações semânticas aprovadas;
* infraestrutura comum.

Não pode depender de:

* Cranelift;
* LLVM;
* registradores físicos;
* formato de objeto;
* linker.

### 7.6 Backend

Pode depender de:

* MIR validada;
* interface de backend;
* ABI aplicável;
* runtime por contratos documentados;
* dependência concreta do backend correspondente.

Não pode depender de:

* estruturas privadas do frontend;
* AST;
* HIR privada;
* detalhes da toolchain.

---

## 8. Dependências externas

Dependências externas devem ser minimizadas.

Uma dependência externa só deve ser adicionada quando:

* resolver problema real;
* reduzir risco técnico;
* tiver manutenção ativa ou estabilidade adequada;
* possuir licença compatível;
* ser compatível com MSRV;
* não introduzir dependências transitivas excessivas;
* puder ser isolada atrás de API interna quando relevante;
* não redefinir contratos da linguagem.

Antes de adicionar uma dependência, deve-se avaliar se a biblioteca padrão Rust ou código local simples é suficiente.

---

## 9. Dependências estruturais

Dependências estruturais exigem ADR.

Exemplos:

* Cranelift;
* LLVM;
* linker alternativo;
* framework de testes de snapshot;
* biblioteca de diagnósticos que modele a saída pública;
* biblioteca de parsing de CLI usada pela toolchain pública;
* infraestrutura de serialização usada em formatos estáveis.

Uma dependência estrutural deve declarar:

* finalidade;
* alternativas consideradas;
* impacto no workspace;
* licença;
* risco de manutenção;
* estratégia de isolamento;
* critério de substituição ou remoção.

---

## 10. Cranelift

Cranelift é dependência estrutural do backend inicial.

Regras:

* deve permanecer isolado no backend Cranelift;
* não pode aparecer em crates de frontend;
* não pode substituir MIR;
* não pode definir semântica da Capi;
* falhas de Cranelift devem ser convertidas em erro de backend ou erro interno estruturado;
* versões devem ser controladas.

O restante do compilador deve depender de uma interface de backend, não de APIs Cranelift.

---

## 11. LLVM

LLVM será introduzido posteriormente como dependência estrutural do backend LLVM.

Regras:

* não deve ser dependência do Stage 0;
* não deve ser introduzido antes dos critérios do Documento 28;
* deve permanecer isolado no backend LLVM;
* não pode substituir MIR;
* não pode alterar semântica observável;
* deve participar de testes diferenciais quando disponível;
* versões e ambiente devem ser documentados.

Ferramentas específicas do ecossistema LLVM devem permanecer encapsuladas pelo backend LLVM.

---

## 12. Dependências de desenvolvimento

Dependências de desenvolvimento incluem ferramentas e crates usados para:

* testes;
* snapshots;
* fuzzing;
* benchmarks;
* geração de fixtures;
* lint;
* cobertura;
* CI.

Regras:

* devem ficar em `dev-dependencies` quando forem crates Rust;
* não devem vazar para artefatos finais;
* devem ser reproduzíveis em CI;
* devem possuir licença compatível;
* devem ser removíveis sem alterar semântica da linguagem.

Dependências de fuzzing, benchmarking e cobertura podem ser introduzidas em stages posteriores.

---

## 13. Dependências de build

Dependências de build incluem `build.rs`, geradores e ferramentas executadas antes ou durante a compilação.

Regras:

* devem ser evitadas no Stage 0 salvo necessidade clara;
* devem ser determinísticas;
* não devem depender de estado local não documentado;
* não devem baixar código durante o build;
* devem registrar entradas e saídas relevantes;
* devem funcionar em CI.

Build scripts não devem alterar contratos da linguagem.

---

## 14. Dependências de plataforma

Dependências de plataforma devem ser isoladas.

Exemplos:

* linker;
* bibliotecas do sistema;
* ferramentas de inspeção de objetos;
* SDKs específicos;
* LLVM instalado pelo sistema, quando aplicável.

Regras:

* devem ser documentadas por target;
* devem ser verificadas no setup;
* não devem contaminar componentes independentes de plataforma;
* devem possuir fallback ou diagnóstico claro quando ausentes;
* devem ser refletidas em CI quando fizerem parte do suporte oficial.

No início, o target principal é Linux x86-64 conforme a implementação oficial.

---

## 15. MSRV e edição Rust

A implementação oficial deve declarar MSRV.

Regras:

* toda dependência deve ser compatível com a MSRV declarada;
* aumento de MSRV deve ser registrado;
* edição Rust deve ser declarada no workspace;
* dependências não devem forçar aumento de MSRV sem decisão explícita;
* CI deve validar a versão suportada quando a política estiver estabelecida.

A MSRV exata pertence a `BUILD-SYSTEM.md` ou documento operacional equivalente.

---

## 16. `Cargo.lock`

`Cargo.lock` deve ser versionado em `capi-lang`.

Regras:

* alterações em `Cargo.lock` devem ser revisadas;
* atualização de dependência deve indicar motivo;
* mudanças transitivas relevantes devem ser observadas;
* lockfile deve ser preservado para bootstrap;
* CI deve usar o lockfile versionado.

Remover `Cargo.lock` exige decisão formal.

---

## 17. `workspace.dependencies`

Dependências compartilhadas devem ser declaradas em `workspace.dependencies` quando usadas por múltiplos crates.

Benefícios:

* controle centralizado de versões;
* redução de divergência;
* revisão mais simples;
* reprodutibilidade.

Regras:

* nem toda dependência precisa ser compartilhada;
* dependências específicas de backend devem ficar no crate de backend;
* dependências experimentais não devem ser promovidas ao workspace sem necessidade;
* features devem ser controladas no local mais restrito possível.

---

## 18. Features Cargo

Features devem ser explícitas e testadas.

Permitido:

* habilitar backend opcional;
* controlar integração experimental;
* separar recursos de desenvolvimento;
* ativar suporte a target específico.

Proibido:

* alterar semântica da linguagem;
* esconder comportamento não testado;
* criar combinações incompatíveis;
* ativar dependência estrutural em crate compartilhado sem necessidade;
* usar feature como substituto de configuração de target.

Features públicas devem ter documentação.

---

## 19. Licenciamento

Toda dependência externa deve possuir licença compatível com o projeto.

Antes da adoção, devem ser verificados:

* licença declarada;
* licença de dependências transitivas relevantes;
* restrições de distribuição;
* compatibilidade com artefatos oficiais;
* compatibilidade com bootstrap e releases.

Dependências sem licença clara não devem ser adotadas.

Casos ambíguos devem ser bloqueados até revisão.

---

## 20. Segurança e supply chain

Dependências externas introduzem risco de supply chain.

Regras:

* preferir dependências maduras e mantidas;
* evitar crates abandonados;
* evitar dependências com grande superfície transitiva sem justificativa;
* revisar crates que executam código em build;
* registrar dependências estruturais;
* usar auditoria automatizada quando disponível.

Vulnerabilidade conhecida em dependência deve ser tratada conforme política de segurança.

---

## 21. Critérios para adicionar dependência externa

Uma proposta de dependência externa deve responder:

```text
Qual problema resolve?
Por que implementação local simples não basta?
Qual licença possui?
Qual MSRV exige?
Quantas dependências transitivas adiciona?
Ela afeta artefatos finais?
Ela afeta bootstrap?
Ela precisa de ADR?
Como será testada?
Como poderá ser removida?
```

Sem respostas adequadas, a dependência não deve ser adicionada.

---

## 22. Critérios para remover dependência

Uma dependência deve ser removida quando:

* não for mais usada;
* puder ser substituída por código simples;
* introduzir risco incompatível;
* bloquear MSRV sem justificativa;
* tiver licença incompatível;
* causar acoplamento indevido;
* duplicar funcionalidade já existente;
* impedir bootstrap ou reprodutibilidade.

Remoção deve atualizar:

* manifestos Cargo;
* lockfile;
* documentação;
* testes;
* ADR, quando aplicável.

---

## 23. Dependências proibidas

São proibidas, salvo decisão arquitetural explícita:

* dependência de frontend para backend concreto;
* dependência de MIR para Cranelift ou LLVM;
* dependência de runtime para AST ou HIR privadas;
* dependência de toolchain para estruturas internas instáveis do compilador;
* dependência externa sem licença clara;
* dependência que execute download de código durante build;
* dependência que altere semântica da linguagem;
* dependência circular entre crates.

---

## 24. Dependências experimentais

Dependências experimentais podem ser usadas em protótipos controlados.

Regras:

* devem ser identificadas como experimentais;
* não devem entrar no caminho oficial de build sem revisão;
* não devem ser exigidas por `cargo test --workspace` salvo decisão explícita;
* devem possuir critério de promoção ou descarte;
* não devem alterar contratos documentados.

Protótipo bem-sucedido não aprova automaticamente dependência.

---

## 25. Revisão de dependências

Mudanças de dependência devem ser revisadas com atenção proporcional ao impacto.

Exige revisão simples:

* dependência de teste pequena;
* atualização patch sem mudança relevante;
* remoção de dependência não usada.

Exige revisão arquitetural:

* nova dependência estrutural;
* mudança de backend;
* aumento de MSRV;
* dependência de build;
* dependência com grande árvore transitiva;
* dependência que afeta formatos, diagnósticos ou artefatos.

---

## 26. Rastreabilidade

Dependências devem ser rastreáveis.

Cada dependência relevante deve poder ser relacionada a:

* componente que a utiliza;
* motivo de uso;
* documento ou ADR aplicável;
* testes que cobrem seu uso;
* risco conhecido;
* plano de atualização ou remoção quando necessário.

Dependências estruturais devem possuir ADR.

---

## 27. Ferramentas de validação

O workspace deve permitir validação de dependências por ferramentas quando possível.

Exemplos:

```bash
cargo tree
cargo metadata
cargo deny
cargo audit
```

A adoção de ferramentas específicas depende de decisão operacional.

Quando adotadas, devem ser documentadas em `BUILD-SYSTEM.md`, `CI-CD.md` ou documento equivalente.

---

## 28. Relação com o Stage 0

Para concluir o Stage 0:

* o workspace deve possuir grafo interno acíclico;
* dependências externas devem ser mínimas;
* `Cargo.lock` deve estar versionado;
* MSRV ou política inicial equivalente deve estar definida;
* licença das dependências deve ser compatível;
* `cargo test --workspace` deve funcionar;
* `cargo clippy --workspace --all-targets` deve funcionar;
* dependências estruturais iniciais devem possuir ADR.

No Stage 0, Cranelift e LLVM não precisam estar no workspace.

Cranelift será introduzido no stage próprio do backend.

LLVM será introduzido posteriormente conforme o Documento 28.

---

## 29. Critérios de aceite deste documento

Este documento estará pronto para orientar o Stage 0 quando:

* definir regras para dependências internas;
* definir regras para dependências externas;
* estabelecer o grafo inicial permitido;
* exigir ausência de ciclos;
* preservar isolamento de backends;
* definir critérios de licenciamento e MSRV;
* explicar uso de `Cargo.lock`;
* não contradizer os Documentos 00 a 28;
* complementar `WORKSPACE-ARCHITECTURE.md` e `COMPONENT-RESPONSIBILITIES.md`.

---

## 30. Síntese

Dependências devem tornar a implementação mais simples, segura e verificável.

Elas não devem enfraquecer fronteiras arquiteturais, reduzir reprodutibilidade, ampliar risco sem justificativa ou transformar mecanismos da implementação em contratos da linguagem.
