# Especificação da Linguagem Capi

Este diretório reúne a especificação conceitual da linguagem Capi e a especificação de referência para sua implementação oficial. Os documentos estão organizados como uma sequência progressiva: primeiro definem as garantias observáveis da linguagem e depois descrevem uma arquitetura capaz de preservar essas garantias no compilador, runtime, biblioteca padrão, toolchain e processo de bootstrap.

A premissa central da especificação é a separação entre garantias e mecanismos: as garantias pertencem à linguagem e devem ser estáveis para o programador; os mecanismos pertencem à implementação e podem evoluir desde que preservem a semântica definida.

## Organização

- [`language/`](language/) contém a especificação da linguagem, dos seus modelos semânticos e das interfaces públicas do ecossistema.
- [`implementation/`](implementation/) contém a especificação de implementação do compilador oficial e o plano de bootstrap.

## Modelo Geral da Linguagem

Capi é uma linguagem orientada a objetos projetada em torno de uma separação explícita entre identidade, comportamento e armazenamento físico. Objetos representam identidade lógica e comportamento; memória mutável pertence a `Domains`; referências a objetos são expressas por `ObjectId`; e a capacidade de mutação é controlada pelo sistema de tipos e pela semântica operacional.

Essa organização busca preservar:

- segurança de memória sem coletor de lixo obrigatório;
- identidade estável para objetos;
- mutação controlada por domínio;
- concorrência previsível;
- determinismo semântico;
- custo de execução observável e compatível com implementação nativa.

## Pipeline de Referência

A arquitetura especificada descreve um compilador em fases, com fronteiras claras entre entrada textual, análise estrutural, análise semântica, representações intermediárias, otimizações e geração de código:

1. fonte, spans e tokens;
2. lexer;
3. parser e AST;
4. lowering para HIR;
5. resolução de nomes e escopos;
6. inferência e verificação de tipos;
7. verificação semântica, ownership, regiões e domains;
8. lowering para IR/MIR;
9. otimizações;
10. backend e geração de código;
11. runtime mínimo, biblioteca padrão e toolchain;
12. testes, validação, conformidade e bootstrap.

## Ordem de Leitura Recomendada

Para entender a linguagem, leia primeiro os documentos `00` a `05`.
Para entender como a linguagem é compilada e executada, continue com `06` a `12`.
Para implementar o compilador oficial ou uma implementação compatível, leia `13` a `27` na ordem.

## Documentos da Linguagem

| Documento | Tema | Papel |
| --- | --- | --- |
| [`00 — Linguagem Capi — Apresentação`](language/00%20%E2%80%94%20Linguagem%20Capi%20%E2%80%94%20Apresenta%C3%A7%C3%A3o.md) | Apresentação geral | Introduz a motivação, os princípios fundamentais, `Domain`, `ObjectId`, mutação, ciclos, herança, concorrência e escapes de segurança. |
| [`01 — Visão Geral e Fundamentos`](language/01%20%E2%80%94%20Vis%C3%A3o%20Geral%20e%20Fundamentos.md) | Fundamentos | Define os objetivos da linguagem, o modelo conceitual e a separação entre classe, objeto, domain e identidade. |
| [`02 — Sistema de Tipos`](language/02%20%E2%80%94%20Sistema%20de%20Tipos.md) | Tipos | Define tipos, nulabilidade, subtipagem, capacidades, mutabilidade e garantias estruturais, de memória e concorrência. |
| [`03 — Modelo de Memória`](language/03%20%E2%80%94%20Modelo%20de%20Mem%C3%B3ria.md) | Memória | Especifica domains, lifetime, alocação, identidade, regiões, escape analysis e garantias de organização física. |
| [`04 — Sintaxe da Linguagem`](language/04%20%E2%80%94%20Sintaxe%20da%20Linguagem.md) | Sintaxe | Define a forma textual de programas, módulos, imports, classes, métodos, expressões, blocos, erros e convenções. |
| [`05 — Semântica Operacional`](language/05%20%E2%80%94%20Sem%C3%A2ntica%20Operacional.md) | Execução | Define como programas são avaliados, como objetos e domains interagem, como erros ocorrem e quais garantias são observáveis. |
| [`06 — Arquitetura do Compilador`](language/06%20%E2%80%94%20Arquitetura%20do%20Compilador.md) | Compilador | Estabelece as fases conceituais do compilador e as garantias que cada etapa deve preservar. |
| [`07 — Intermediate Representation — IR`](language/07%20%E2%80%94%20Intermediate%20Representation%20%E2%80%94%20IR.md) | IR | Define o papel da representação intermediária como fronteira canônica entre frontend, middle-end e backend. |
| [`08 — Biblioteca Padrão`](language/08%20%E2%80%94%20Biblioteca%20Padr%C3%A3o.md) | Biblioteca padrão | Define o papel da biblioteca oficial, sua relação com o core, domains, APIs e evolução compatível. |
| [`09 — Runtime Mínimo`](language/09%20%E2%80%94%20Runtime%20M%C3%ADnimo.md) | Runtime | Define a infraestrutura mínima de execução necessária para preservar as garantias da linguagem. |
| [`10 — ABI e Interoperabilidade — FFI`](language/10%20%E2%80%94%20ABI%20e%20Interoperabilidade%20%E2%80%94%20FFI.md) | ABI e FFI | Define princípios de interoperabilidade, fronteiras `unsafe`, compatibilidade binária e responsabilidades da ABI. |
| [`11 — Toolchain`](language/11%20%E2%80%94%20Toolchain.md) | Ferramentas | Define `capi`, `capic`, formatter, testes, documentação, comandos e experiência de desenvolvimento. |
| [`12 — Ecossistema e Gerenciamento de Pacotes`](language/12%20%E2%80%94%20Ecossistema%20e%20Gerenciamento%20de%20Pacotes.md) | Ecossistema | Define pacotes, bibliotecas, registros, dependências, versionamento e evolução do ecossistema. |

## Documentos de Implementação

| Documento | Tema | Papel |
| --- | --- | --- |
| [`13 — Estrutura do Compilador`](implementation/13%20%E2%80%94%20Estrutura%20do%20Compilador.md) | Estrutura | Define a arquitetura modular do compilador oficial, suas camadas, contratos e evolução. |
| [`14 — Lexer e Tokens`](implementation/14%20%E2%80%94%20Lexer%20e%20Tokens.md) | Lexer | Especifica tokenização, spans, categorias léxicas, erros, recuperação e testes. |
| [`15 — Parser e AST`](implementation/15%20%E2%80%94%20Parser%20e%20AST.md) | Parser | Define o parser, a AST, recuperação sintática, invariantes estruturais e integração com diagnósticos. |
| [`16 — HIR`](implementation/16%20%E2%80%94%20HIR.md) | HIR | Define a representação semântica de alto nível, o lowering da AST e a base para análise semântica. |
| [`17 — Resolução de Nomes`](implementation/17%20%E2%80%94%20Resolu%C3%A7%C3%A3o%20de%20Nomes.md) | Nomes e escopos | Especifica escopos, símbolos, resolução, diagnósticos e contratos com fases posteriores. |
| [`18 — Inferência e Verificação de Tipos`](implementation/18%20%E2%80%94%20Infer%C3%AAncia%20e%20Verifica%C3%A7%C3%A3o%20de%20Tipos.md) | Tipagem | Define inferência, checagem, coerções, generics e preservação das garantias do sistema de tipos. |
| [`19 — Ownership e Borrow Checker`](implementation/19%20%E2%80%94%20Ownership%20e%20Borrow%20Checker.md) | Verificação semântica | Detalha a verificação semântica ligada a ownership, mutabilidade, acesso e consistência da HIR. |
| [`20 — Domains e Análise de Regiões`](implementation/20%20%E2%80%94%20Domains%20e%20An%C3%A1lise%20de%20Regi%C3%B5es.md) | Domains e regiões | Define lowering semântico, análise de regiões, constraints de domains e preparação para IR. |
| [`21 — MIR e Fluxo de Controle`](implementation/21%20%E2%80%94%20MIR%20e%20Fluxo%20de%20Controle.md) | MIR | Define a representação intermediária de controle de fluxo, invariantes e relação com backend. |
| [`22 — Geração de Código`](implementation/22%20%E2%80%94%20Gera%C3%A7%C3%A3o%20de%20C%C3%B3digo.md) | Otimizações | Especifica passes sobre a representação intermediária e preservação semântica antes do backend. |
| [`23 — Runtime e Biblioteca Padrão Inicial`](implementation/23%20%E2%80%94%20Runtime%20e%20Biblioteca%20Padr%C3%A3o%20Inicial.md) | Diagnósticos | Define arquitetura de diagnósticos, recuperação de erros e integração transversal com o pipeline. |
| [`24 — Toolchain e Experiência de Desenvolvimento`](implementation/24%20%E2%80%94%20Toolchain%20e%20Experi%C3%AAncia%20de%20Desenvolvimento.md) | Backend | Define backend, objetos gerados, ABI, linking, portabilidade e relação com a plataforma alvo. |
| [`25 — Testes, Validação e Conformidade`](implementation/25%20%E2%80%94%20Testes%2C%20Valida%C3%A7%C3%A3o%20e%20Conformidade.md) | Testes | Define estratégia de testes, golden tests, integração, conformidade, regressão e validação contínua. |
| [`26 — Bootstrap e Subconjunto Inicial`](implementation/26%20%E2%80%94%20Bootstrap%20e%20Subconjunto%20Inicial.md) | Bootstrap | Define o subconjunto inicial, fases de evolução e critérios para avançar rumo à auto-hospedagem. |
| [`27 — Bootstrap Plan e Arquitetura da Implementação Oficial`](implementation/27%20%E2%80%94%20Bootstrap%20Plan%20e%20Arquitetura%20da%20Implementa%C3%A7%C3%A3o%20Oficial.md) | Plano oficial | Consolida o plano de engenharia, decisões arquiteturais, backend inicial, milestones e critérios de conclusão. |

## Invariantes da Especificação

- Objetos não são donos diretos da memória física mutável.
- `Domain` é a unidade física de organização e controle de memória.
- `ObjectId` representa identidade lógica estável.
- Escrita e mutação são mediadas por capacidades e por regras de domínio.
- O sistema de tipos deve impedir usos inválidos antes da execução sempre que possível.
- A semântica operacional define o comportamento observável, não a estrutura interna de uma implementação específica.
- O compilador deve preservar as garantias da linguagem em todas as representações intermediárias.
- Interoperabilidade e `unsafe` são fronteiras explícitas, localizadas e responsáveis por não invalidar as garantias do código seguro.

## Estado dos Documentos

Os documentos desta pasta estão em formato de especificação evolutiva. Mudanças devem manter rastreabilidade entre linguagem, implementação, testes e decisões arquiteturais. Alterações normativas devem preservar a separação entre:

- o que a linguagem garante;
- o que a implementação oficial escolhe como mecanismo;
- o que outras implementações compatíveis podem decidir de forma diferente.
