# ADR-0003 — Separação entre Frontend, Middle-end e Backend

**Status:** Aprovado  
**Data:** 2026-07-30  
**Stage:** Stage 0 — Fundação do projeto  
**Decisão:** Separar a implementação do compilador em frontend, middle-end e backend, com MIR independente de backend.

---

## Contexto

A specification da Capi define uma linguagem com sistema de tipos, modelo de memória, `Domain`, `ObjectId`, runtime, ABI e IR próprios.

A implementação oficial precisa preservar essas garantias enquanto evolui de forma incremental.

O Documento 27 determina que Cranelift será o backend inicial e LLVM será introduzido posteriormente. Para isso ser possível sem reescrever o compilador, o frontend e o middle-end não podem depender diretamente de APIs de backend.

O Documento 28 também separa os stages de frontend, HIR, tipagem, semântica, MIR, runtime, backend Cranelift, conformidade, LLVM e auto-hospedagem.

---

## Decisão

O compilador oficial será organizado em camadas:

```text
Frontend
Middle-end
Backend
Runtime
Toolchain
Shared infrastructure
```

O frontend será responsável por:

- source loading;
- spans;
- lexer;
- parser;
- AST;
- lowering para HIR;
- resolução de nomes;
- checagem de tipos;
- verificações semânticas;
- ownership, regiões, `Domain` e regras de mutabilidade.

O middle-end será responsável por:

- lowering para MIR;
- validação da MIR;
- passes independentes de backend;
- dumps e inspeção de MIR;
- preparação da interface de backend.

O backend será responsável por:

- consumir MIR validada;
- implementar uma interface comum de backend;
- gerar objetos, executáveis ou artefatos intermediários;
- isolar Cranelift, LLVM e backends futuros.

A MIR deve permanecer independente de Cranelift, LLVM e detalhes de plataforma.

---

## Justificativa

Essa separação permite:

- preservar a semântica da Capi antes da geração de código;
- testar frontend sem backend;
- testar backends diferencialmente;
- introduzir LLVM depois de Cranelift;
- manter a linguagem independente dos mecanismos de geração de código;
- evoluir middle-end e otimizações sem afetar parsing ou tipagem;
- permitir futura auto-hospedagem por componentes.

---

## Alternativas Consideradas

### Acoplar frontend diretamente ao Cranelift

Rejeitada porque transformaria o backend inicial em dependência estrutural da análise da linguagem.

### Usar LLVM IR como IR principal da Capi

Rejeitada porque a IR da Capi precisa preservar conceitos semânticos próprios antes da tradução para backends concretos.

### Implementar um pipeline monolítico

Rejeitada porque dificultaria testes, diagnósticos, evolução incremental, substituição de backends e bootstrap.

---

## Consequências Positivas

- Frontend, middle-end e backend podem evoluir separadamente.
- Backends oficiais devem preservar comportamento observável equivalente.
- A MIR se torna fronteira testável e inspecionável.
- A implementação fica preparada para Cranelift e LLVM.
- A arquitetura reduz risco de dependências circulares.

---

## Consequências Negativas

- Exige modelagem explícita de contratos entre fases.
- Pode gerar mais crates e tipos internos no Stage 0.
- A interface de backend precisa ser cuidadosamente mantida.
- Dumps, validações e testes de representações intermediárias passam a ser obrigatórios conforme o compilador amadurece.

---

## Restrições

- O frontend não pode depender de Cranelift, LLVM, linker ou formato de objeto.
- A MIR não pode depender de backend concreto.
- Backends não podem redefinir semântica da linguagem.
- Fases devem produzir entradas e saídas explícitas.
- Diagnósticos devem circular por infraestrutura estruturada.
- Erros internos devem ser separados de erros do usuário.

---

## Critérios de Validação

Esta decisão será considerada operacional quando:

- os crates fundamentais preservarem as fronteiras iniciais;
- fases futuras forem adicionadas sem dependência reversa;
- a MIR puder ser validada sem backend;
- Cranelift consumir a interface de backend, não estruturas privadas do frontend;
- LLVM puder ser introduzido posteriormente sem reescrever frontend ou HIR.

---

## Referências

- Documento 06 — Arquitetura do Compilador
- Documento 07 — Intermediate Representation — IR
- Documentos 13 a 22 — Especificação de implementação do compilador
- Documento 27 — Bootstrap Plan e Arquitetura da Implementação Oficial
- Documento 28 — Plano de Desenvolvimento da Implementação Oficial
- `COMPILER-ARCHITECTURE.md`
- `COMPILATION-PIPELINE.md`
- `COMPONENT-RESPONSIBILITIES.md`
- `DEPENDENCY-RULES.md`
