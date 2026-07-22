# 08 — Biblioteca Padrão

## Especificação da Linguagem Capi

Versão: 1.0 (Draft)

---

# 1. Introdução

A Biblioteca Padrão da Capi reúne os componentes fundamentais disponibilizados para todos os programas escritos na linguagem.

Ela fornece abstrações reutilizáveis, estruturas de dados, algoritmos e serviços de uso geral que complementam os recursos oferecidos diretamente pela linguagem, reduzindo a necessidade de implementações repetitivas e promovendo consistência entre aplicações.

A Biblioteca Padrão faz parte da distribuição oficial da Capi e constitui um elemento essencial de seu ecossistema.

---

## 1.1 Objetivo

Este documento especifica os princípios, organização e responsabilidades da Biblioteca Padrão da Capi.

São objetivos deste documento:

* definir o papel da Biblioteca Padrão no ecossistema da linguagem;
* estabelecer sua organização modular;
* especificar os princípios arquiteturais que orientam seu desenvolvimento;
* definir os serviços mínimos que toda implementação oficial deverá fornecer;
* estabelecer os limites entre Biblioteca Padrão, Runtime Mínimo e Compilador.

A implementação física da Biblioteca Padrão não faz parte desta especificação.

---

## 1.2 Escopo

Este documento descreve exclusivamente os componentes pertencentes à Biblioteca Padrão da Capi.

Não fazem parte deste documento:

* a sintaxe da linguagem (Documento 04);
* o Sistema de Tipos (Documento 02);
* o Modelo de Memória (Documento 03);
* a Semântica Operacional (Documento 05);
* a Arquitetura do Compilador (Documento 06);
* a Intermediate Representation (Documento 07);
* o Runtime Mínimo (Documento 09);
* a ABI e a Interoperabilidade (Documento 10).

---

## 1.3 Relação com os Documentos Anteriores

A Biblioteca Padrão complementa os recursos definidos pelos documentos anteriores.

Enquanto:

* o Sistema de Tipos define os tipos fundamentais da linguagem;
* o Modelo de Memória define como objetos existem;
* a Semântica Operacional define como programas executam;
* o Compilador transforma código em executáveis;

a Biblioteca Padrão oferece implementações reutilizáveis construídas sobre essas garantias.

Sempre que possível, os componentes da Biblioteca Padrão utilizam exclusivamente recursos públicos da linguagem, sem depender de privilégios especiais do compilador.

---

# 2. Papel da Biblioteca Padrão

A Biblioteca Padrão constitui o conjunto oficial de componentes reutilizáveis disponibilizados pela linguagem.

Ela fornece abstrações de alto nível que simplificam o desenvolvimento de aplicações, mantendo coerência com os princípios arquiteturais da Capi.

---

## 2.1 Complemento da Linguagem

A Biblioteca Padrão complementa a linguagem, mas não faz parte dela.

A ausência de um componente da Biblioteca Padrão não altera a sintaxe nem a semântica da linguagem.

Da mesma forma, novas versões da Biblioteca Padrão poderão introduzir funcionalidades adicionais sem exigir alterações na especificação da linguagem.

---

## 2.2 Implementação em Capi

A implementação de referência da Biblioteca Padrão deverá ser escrita predominantemente na própria linguagem Capi.

Essa decisão possui diversos objetivos:

* validar continuamente a linguagem;
* demonstrar a expressividade da Capi;
* reduzir funcionalidades privilegiadas do compilador;
* facilitar auditoria e evolução da biblioteca.

Componentes que dependam diretamente de recursos específicos da plataforma poderão utilizar mecanismos definidos pelo Runtime Mínimo e pela ABI, respeitando os limites estabelecidos por esses documentos.

---

## 2.3 Mesmo Pipeline de Compilação

Toda a Biblioteca Padrão deverá utilizar exatamente o mesmo processo de compilação disponível para qualquer aplicação desenvolvida em Capi.

Isso significa que seus componentes passam pelas mesmas etapas de:

* análise léxica;
* análise sintática;
* análise semântica;
* geração da Intermediate Representation (IR);
* otimizações;
* geração de código.

Não existem regras especiais de compilação para a Biblioteca Padrão.

---

## 2.4 Distribuição Oficial

A distribuição oficial da linguagem deverá incluir uma implementação completa da Biblioteca Padrão.

Implementações alternativas poderão fornecer bibliotecas equivalentes, desde que preservem a compatibilidade semântica definida por esta especificação.

A existência de implementações distintas não altera os contratos públicos definidos para cada componente.

---

## 2.5 Evolução

A Biblioteca Padrão poderá evoluir continuamente.

Novos módulos, novos algoritmos e novos componentes poderão ser incorporados sem alterar as garantias da linguagem.

Essa evolução deverá preservar compatibilidade com as interfaces públicas já estabelecidas ou seguir as políticas oficiais de versionamento do ecossistema da Capi.

---

# 3. Princípios da Biblioteca Padrão

A Biblioteca Padrão da Capi foi concebida para permanecer simples, consistente e fortemente integrada aos princípios definidos pela linguagem.

Todos os componentes deverão respeitar os princípios apresentados neste capítulo.

---

## 3.1 Uso Exclusivo de Recursos Públicos

Sempre que possível, os componentes da Biblioteca Padrão deverão utilizar exclusivamente recursos públicos disponibilizados pela linguagem.

Funcionalidades internas do compilador não deverão ser utilizadas para implementar abstrações que possam ser escritas em Capi.

---

## 3.2 Modularidade

A Biblioteca Padrão deverá ser organizada em módulos independentes.

Cada módulo deverá possuir responsabilidades claramente definidas, reduzindo acoplamento e facilitando manutenção, evolução e reutilização.

Dependências circulares entre módulos deverão ser evitadas.

---

## 3.3 Portabilidade

Os componentes da Biblioteca Padrão deverão apresentar comportamento consistente em todas as plataformas oficialmente suportadas.

Diferenças específicas de sistemas operacionais ou arquiteturas deverão ser encapsuladas em camadas apropriadas de abstração.

---

## 3.4 Consistência Semântica

Todos os componentes deverão respeitar integralmente:

* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* a arquitetura baseada em Domains;
* as regras relacionadas à Capacidade de Mutação.

Nenhuma biblioteca poderá enfraquecer ou contornar as garantias estabelecidas pela linguagem.

---

## 3.5 Biblioteca Mínima

A Biblioteca Padrão deverá permanecer pequena e coesa.

Seu objetivo é fornecer apenas os componentes considerados fundamentais para o desenvolvimento de aplicações.

Funcionalidades especializadas, dependentes de domínios específicos ou de terceiros deverão ser disponibilizadas preferencialmente através do ecossistema de pacotes, preservando a simplicidade da distribuição oficial da linguagem.

---

# 4. Organização Geral da Biblioteca

A Biblioteca Padrão da Capi é organizada em módulos independentes, cada um responsável por um conjunto específico de funcionalidades.

Essa organização busca manter baixo acoplamento, facilitar a evolução da biblioteca e permitir que aplicações utilizem apenas os módulos necessários.

Conceitualmente, a biblioteca é organizada da seguinte forma:

```text
std
 ├── core
 ├── collections
 ├── text
 ├── io
 ├── math
 ├── time
 ├── filesystem
 ├── net
 ├── concurrent
 └── reflection
```

Novos módulos poderão ser incorporados em versões futuras sem alterar a organização dos módulos já existentes.

---

## 4.1 Independência dos Módulos

Cada módulo deverá possuir responsabilidades claramente definidas.

Sempre que possível:

* dependências entre módulos deverão ser unidirecionais;
* dependências circulares deverão ser evitadas;
* módulos de alto nível não deverão ser utilizados por módulos fundamentais.

Essa organização reduz o acoplamento e facilita a manutenção da biblioteca.

---

## 4.2 Hierarquia

Os módulos fundamentais constituem a base para os demais.

Por exemplo:

* `core` não depende de nenhum outro módulo;
* `collections` depende apenas de `core`;
* `text` pode depender de `core`;
* módulos especializados podem depender dos módulos fundamentais.

Essa hierarquia favorece implementações enxutas e reduz o custo de utilização da Biblioteca Padrão.

---

## 4.3 Evolução

A organização modular da biblioteca poderá evoluir ao longo do tempo.

Novos módulos poderão ser adicionados, desde que:

* não comprometam a estabilidade dos módulos existentes;
* preservem compatibilidade com as interfaces públicas;
* respeitem os princípios estabelecidos neste documento.

---

# 5. Convenções da Biblioteca

Todos os componentes da Biblioteca Padrão deverão seguir convenções uniformes de organização e nomenclatura.

Essas convenções promovem consistência, previsibilidade e facilidade de utilização.

---

## 5.1 Namespaces

Cada módulo deverá possuir seu próprio namespace.

Exemplos:

```text
std.core
std.collections
std.text
std.io
```

A estrutura física dos arquivos não precisa refletir exatamente essa organização, desde que a organização lógica permaneça preservada.

---

## 5.2 Convenções de Nomes

A Biblioteca Padrão deverá adotar convenções consistentes para:

* módulos;
* classes;
* interfaces;
* traits;
* funções;
* constantes.

A nomenclatura deverá privilegiar clareza, evitando abreviações desnecessárias.

---

## 5.3 Versionamento

A evolução da Biblioteca Padrão deverá preservar compatibilidade com as interfaces públicas sempre que possível.

Mudanças incompatíveis deverão seguir a política oficial de versionamento do ecossistema da Capi.

---

## 5.4 Estabilidade da API

Interfaces públicas pertencentes à Biblioteca Padrão deverão possuir contratos estáveis.

Uma vez disponibilizadas oficialmente, alterações incompatíveis deverão ocorrer apenas mediante processo formal de evolução da linguagem ou da biblioteca.

---

## 5.5 Documentação

Todos os componentes públicos da Biblioteca Padrão deverão possuir documentação oficial.

Essa documentação deverá descrever:

* finalidade;
* comportamento;
* restrições;
* complexidade quando aplicável;
* exemplos de utilização.

A documentação faz parte integrante da qualidade da Biblioteca Padrão.

---

# 6. Módulo Core

O módulo **Core** reúne as abstrações fundamentais utilizadas por praticamente todos os demais módulos da Biblioteca Padrão.

Ele não define novos recursos da linguagem nem modifica o Sistema de Tipos. Sua responsabilidade consiste em fornecer operações, interfaces e abstrações construídas sobre os conceitos definidos pelos documentos anteriores.

---

## 6.1 Papel do Core

O módulo Core constitui a base da Biblioteca Padrão.

Seu objetivo é disponibilizar componentes reutilizáveis que representem comportamentos fundamentais da linguagem sem introduzir dependências desnecessárias.

Sempre que possível, os demais módulos deverão depender apenas do Core.

---

## 6.2 Operações sobre Tipos Fundamentais

Os tipos fundamentais definidos pelo Sistema de Tipos permanecem especificados no Documento 02.

O módulo Core fornece as operações e abstrações associadas a esses tipos.

Entre elas:

* operações auxiliares para `Optional<T>`;
* operações auxiliares para `Result<T,E>`;
* operações auxiliares para `Unit`;
* funcionalidades relacionadas ao tipo `String`;
* operações comuns sobre coleções iteráveis;
* conversões e utilidades gerais.

O módulo Core também disponibiliza operações auxiliares relacionadas ao tipo Unit, permitindo sua utilização em composições funcionais, pipelines e demais abstrações que envolvam operações sem valor de retorno significativo.

A definição dos tipos pertence à linguagem; sua utilização prática pertence à Biblioteca Padrão.

---

## 6.3 Interfaces Fundamentais

O módulo Core deverá disponibilizar as principais interfaces utilizadas pelos demais módulos da biblioteca.

Exemplos incluem:

* `Iterable`;
* `Iterator`;
* `Comparable`;
* `Equatable`;
* `Hashable`;
* `Cloneable`;
* `Disposable`.

A lista definitiva dessas interfaces poderá evoluir ao longo do desenvolvimento da linguagem.

---

## 6.4 Traits Fundamentais

Além das interfaces, o Core poderá fornecer traits reutilizáveis que implementem comportamentos comuns.

Essas traits deverão utilizar exclusivamente mecanismos públicos da linguagem e não poderão depender de funcionalidades privilegiadas do compilador.

---

## 6.5 Dependências

O módulo Core deverá permanecer o mais independente possível.

Ele não deverá depender de módulos especializados, preservando sua função como alicerce de toda a Biblioteca Padrão.

Essa independência reduz o acoplamento, facilita a reutilização e contribui para implementações mais simples e portáveis.

---

# 7. Módulo Collections

O módulo **Collections** reúne as estruturas de dados fundamentais disponibilizadas pela Biblioteca Padrão da Capi.

Seu objetivo é fornecer implementações eficientes, consistentes e semanticamente compatíveis com o Sistema de Tipos e o Modelo de Memória da linguagem.

Todas as coleções deverão respeitar integralmente as garantias relacionadas a Domains, `ObjectId`, Lifetime e Capacidade de Mutação.

---

## 7.1 Objetivos

O módulo Collections deverá oferecer:

* estruturas de dados de uso geral;
* algoritmos de manipulação de coleções;
* interfaces comuns para iteração;
* implementações eficientes e previsíveis.

A especificação define o comportamento esperado das coleções, não sua implementação interna.

---

## 7.2 Coleções Fundamentais

A Biblioteca Padrão deverá fornecer, no mínimo, implementações para:

* `List<T>`;
* `Vector<T>`;
* `Set<T>`;
* `Map<K, V>`;
* `Queue<T>`;
* `Stack<T>`.

Implementações adicionais poderão ser incorporadas em versões futuras.

---

## 7.3 Relação com Domains

Toda coleção pertence ao Domain onde foi criada.

Coleções parametrizadas por tipos de identidade armazenam referências aos respectivos `ObjectId`s. Coleções parametrizadas por tipos por valor armazenam diretamente seus respectivos valores.

A associação entre coleções e Domains segue integralmente as regras estabelecidas pelo Modelo de Memória (Documento 03).

---

## 7.4 Iteração

Todas as coleções deverão implementar as interfaces de iteração definidas pelo módulo Core.

Isso garante comportamento uniforme para:

* laços de repetição;
* algoritmos genéricos;
* processamento funcional;
* composição de operações.

---

## 7.5 Compatibilidade com o Sistema de Tipos

Coleções deverão preservar integralmente as regras estabelecidas pelo Sistema de Tipos.

Isso inclui:

* generics;
* variância;
* igualdade;
* identidade;
* restrições de tipos.

A Biblioteca Padrão não poderá introduzir regras próprias que contrariem a especificação da linguagem.

---

## 7.6 Independência da Implementação

Este documento não especifica:

* algoritmos internos;
* estruturas físicas;
* balanceamento de árvores;
* tabelas hash;
* estratégias de crescimento.

Cada implementação poderá utilizar a estratégia mais adequada, desde que preserve os contratos públicos definidos para cada coleção.

---

# 8. Módulo Text

O módulo **Text** reúne os componentes responsáveis pelo processamento de texto e caracteres.

Seu objetivo é fornecer uma API consistente para manipulação de cadeias de caracteres, preservando compatibilidade com Unicode e com diferentes plataformas.

---

## 8.1 Objetivos

O módulo Text deverá oferecer recursos para:

* manipulação de texto;
* comparação;
* pesquisa;
* transformação;
* formatação;
* codificação.

---

## 8.2 O Tipo String

O tipo `String` é definido pelo Sistema de Tipos da linguagem (Documento 02).

O módulo Text fornece as operações disponíveis sobre esse tipo.

Entre elas:

* concatenação;
* divisão;
* substituição;
* pesquisa;
* comparação;
* conversão;
* formatação.

A implementação interna de `String` pertence exclusivamente à Biblioteca Padrão e ao Runtime Mínimo, quando necessário.

---

## 8.3 Unicode

A Biblioteca Padrão deverá tratar texto como dados Unicode.

Operações envolvendo caracteres deverão respeitar os princípios definidos pelo padrão Unicode, evitando pressupostos específicos de codificações locais.

A estratégia de armazenamento utilizada para representar texto pertence à implementação.

---

## 8.4 Conversões

O módulo deverá disponibilizar operações de conversão entre texto e outros tipos fundamentais.

Exemplos incluem:

* números;
* datas;
* valores booleanos;
* tipos definidos pelo usuário.

As regras de conversão deverão ser consistentes em todas as plataformas oficialmente suportadas.

---

## 8.5 Formatação

O módulo deverá fornecer mecanismos padronizados para formatação de texto.

Esses mecanismos poderão incluir:

* interpolação;
* alinhamento;
* preenchimento;
* representação textual de objetos.

A especificação detalhada dessas funcionalidades poderá evoluir em versões futuras.

---

# 9. Módulo Math

O módulo **Math** reúne operações matemáticas de uso geral.

Seu objetivo é fornecer uma base uniforme para cálculos numéricos, preservando comportamento consistente entre plataformas.

---

## 9.1 Objetivos

O módulo Math deverá disponibilizar:

* constantes matemáticas;
* operações elementares;
* funções trigonométricas;
* exponenciação;
* logaritmos;
* arredondamentos;
* operações estatísticas básicas.

---

## 9.2 Compatibilidade Numérica

As funções matemáticas deverão respeitar os tipos numéricos definidos pelo Sistema de Tipos.

Sempre que possível, seus resultados deverão ser consistentes entre plataformas.

Diferenças decorrentes da arquitetura de hardware deverão permanecer dentro dos limites aceitos pelos padrões adotados pela linguagem.

---

## 9.3 Aleatoriedade

A Biblioteca Padrão poderá fornecer mecanismos para geração de números pseudoaleatórios.

A especificação define apenas os contratos públicos dessas APIs.

Os algoritmos utilizados pertencem exclusivamente à implementação.

---

## 9.4 Precisão

Este documento não estabelece algoritmos específicos para operações matemáticas.

Cada implementação poderá utilizar bibliotecas especializadas ou algoritmos próprios, desde que preserve a precisão e o comportamento especificados para cada operação.

---

## 9.5 Extensibilidade

Novas funções matemáticas poderão ser incorporadas em versões futuras da Biblioteca Padrão.

A inclusão de novos recursos deverá preservar compatibilidade com as interfaces públicas já estabelecidas e manter a coerência com o restante do módulo.

---

# 10. Módulo IO

O módulo **IO** reúne as abstrações fundamentais para operações de entrada e saída de dados.

Seu objetivo é fornecer uma API uniforme e independente da plataforma para leitura, escrita e manipulação de fluxos de dados.

As implementações concretas poderão utilizar os mecanismos disponibilizados pelo Runtime Mínimo e pela plataforma de execução.

---

## 10.1 Objetivos

O módulo IO deverá fornecer recursos para:

* leitura de dados;
* escrita de dados;
* manipulação de fluxos (*streams*);
* operações bufferizadas;
* abstrações independentes da origem ou destino dos dados.

---

## 10.2 Streams

A Biblioteca Padrão deverá disponibilizar abstrações para fluxos de entrada e saída.

Essas abstrações deverão permitir o tratamento uniforme de diferentes origens de dados, como:

* memória;
* arquivos;
* dispositivos;
* conexões de rede;
* outros recursos disponibilizados pelo Runtime.

A forma como esses fluxos são implementados pertence exclusivamente à implementação.

---

## 10.3 Buffers

O módulo poderá disponibilizar estruturas para armazenamento temporário de dados durante operações de entrada e saída.

A estratégia de gerenciamento desses buffers não faz parte desta especificação.

---

## 10.4 Tratamento de Erros

Falhas durante operações de entrada e saída deverão ser representadas utilizando os mecanismos de tratamento de erros definidos pela linguagem, preferencialmente através de `Result<T,E>`.

A Biblioteca Padrão não deverá utilizar mecanismos alternativos que contrariem a Semântica Operacional.

---

## 10.5 Independência da Plataforma

As APIs públicas do módulo IO deverão permanecer consistentes entre todas as plataformas oficialmente suportadas.

Diferenças específicas dos sistemas operacionais deverão permanecer encapsuladas pela implementação.

---

# 11. Módulo Time

O módulo **Time** reúne os componentes relacionados à representação e manipulação de tempo.

Seu objetivo é oferecer abstrações portáveis para datas, horários, durações e medições temporais.

---

## 11.1 Objetivos

O módulo Time deverá fornecer recursos para:

* representação de instantes;
* representação de durações;
* medição de intervalos;
* obtenção da hora atual;
* cálculos envolvendo tempo.

---

## 11.2 Tipos Fundamentais

A Biblioteca Padrão deverá disponibilizar, no mínimo, abstrações equivalentes a:

* `Instant`;
* `Duration`;
* `Clock`;
* `Date`;
* `Time`;
* `DateTime`.

Outros tipos poderão ser incorporados em versões futuras.

---

## 11.3 Fontes de Tempo

A obtenção do tempo atual depende da plataforma de execução.

O módulo Time deverá encapsular essas diferenças através de interfaces uniformes, preservando comportamento consistente entre implementações.

---

## 11.4 Precisão

A precisão disponível para medições temporais dependerá da plataforma.

Entretanto, as APIs públicas deverão documentar claramente:

* resolução disponível;
* precisão esperada;
* limitações conhecidas.

---

## 11.5 Independência

A Biblioteca Padrão não estabelece calendários, fusos horários ou regras regionais específicas.

Esses recursos poderão ser fornecidos por módulos adicionais ou pelo ecossistema de pacotes.

---

# 12. Módulo Filesystem

O módulo **Filesystem** reúne os componentes responsáveis pela manipulação do sistema de arquivos.

Seu objetivo é oferecer uma API uniforme para acesso a arquivos e diretórios, abstraindo diferenças entre plataformas.

---

## 12.1 Objetivos

O módulo deverá disponibilizar recursos para:

* manipulação de caminhos;
* leitura de arquivos;
* escrita de arquivos;
* criação e remoção de diretórios;
* consulta de atributos do sistema de arquivos.

---

## 12.2 Caminhos

A Biblioteca Padrão deverá fornecer um tipo específico para representação de caminhos (`Path`).

Esse tipo deverá abstrair diferenças entre plataformas, como:

* separadores de diretório;
* caminhos absolutos;
* caminhos relativos;
* normalização.

---

## 12.3 Arquivos

As operações fundamentais sobre arquivos deverão incluir, no mínimo:

* abertura;
* leitura;
* escrita;
* cópia;
* movimentação;
* remoção.

As permissões e atributos específicos da plataforma permanecem sob responsabilidade da implementação.

Operações que dependam de permissões concedidas pelo sistema operacional deverão representar falhas através de `Result<T,E>`.

A verificação efetiva das permissões pertence ao sistema operacional e ao Runtime Mínimo.

---

## 12.4 Diretórios

O módulo deverá disponibilizar operações para:

* criação;
* remoção;
* listagem;
* verificação de existência;
* navegação na estrutura de diretórios.

Essas operações deverão utilizar APIs consistentes em todas as plataformas.

---

## 12.5 Integração com o Runtime

O acesso ao sistema de arquivos depende necessariamente de serviços fornecidos pela plataforma de execução.

A Biblioteca Padrão deverá utilizar exclusivamente as interfaces disponibilizadas pelo Runtime Mínimo para realizar essas operações, preservando a independência entre a API pública e os mecanismos específicos de cada sistema operacional.

---

# 13. Módulo Net

O módulo **Net** reúne as abstrações fundamentais para comunicação entre processos e sistemas através de redes de computadores.

Seu objetivo é fornecer uma API portável para operações de rede de baixo nível, preservando independência em relação aos protocolos de mais alto nível.

Protocolos específicos de aplicação poderão ser disponibilizados por bibliotecas adicionais ou pelo ecossistema de pacotes.

---

## 13.1 Objetivos

O módulo Net deverá fornecer recursos para:

* comunicação através de sockets;
* resolução de nomes;
* endereçamento de rede;
* estabelecimento de conexões;
* troca de dados entre aplicações.

---

## 13.2 Escopo

O módulo Net destina-se exclusivamente às abstrações fundamentais de comunicação.

Não fazem parte deste módulo:

* HTTP;
* FTP;
* SMTP;
* SSH;
* WebSocket;
* bancos de dados;
* protocolos específicos de aplicação.

Esses recursos pertencem ao ecossistema de bibliotecas da linguagem.

Recursos relacionados à segurança da comunicação, como criptografia, TLS, autenticação e gerenciamento de certificados, não fazem parte do módulo Net.

Essas funcionalidades deverão ser fornecidas por bibliotecas especializadas pertencentes ao ecossistema da linguagem.

---

## 13.3 Endereçamento

A Biblioteca Padrão deverá fornecer tipos apropriados para representar:

* endereços IP;
* portas;
* nomes de domínio;
* endpoints de comunicação.

Essas abstrações deverão permanecer independentes da plataforma.

---

## 13.4 Sockets

A comunicação através de sockets deverá ser disponibilizada por interfaces públicas uniformes.

A implementação poderá utilizar os mecanismos oferecidos pelo sistema operacional, desde que preserve o comportamento especificado para a API.

---

## 13.5 Integração com o Runtime

Todo acesso à infraestrutura de rede deverá ocorrer através das interfaces disponibilizadas pelo Runtime Mínimo.

A Biblioteca Padrão permanece responsável apenas pela definição da API pública e das abstrações disponibilizadas ao programador.

---

# 14. Módulo Concurrent

O módulo **Concurrent** reúne as abstrações relacionadas à execução concorrente de programas.

Seu objetivo é disponibilizar mecanismos seguros e consistentes para criação de fluxos concorrentes, comunicação entre tarefas e coordenação de execução, preservando integralmente o Modelo de Memória da Capi.

---

## 14.1 Objetivos

O módulo Concurrent deverá fornecer recursos para:

* criação de tarefas concorrentes;
* comunicação entre tarefas;
* sincronização de execução;
* coordenação de atividades independentes.

Todas essas operações deverão respeitar os princípios estabelecidos para Domains e Capacidade de Mutação.

---

## 14.2 Relação com Domains

A concorrência da Capi é baseada no conceito de **Domain**, conforme definido pelo Modelo de Memória e pela Semântica Operacional.

As abstrações disponibilizadas por este módulo deverão operar sobre esses conceitos, sem introduzir mecanismos que violem as garantias da linguagem.

Todas as primitivas disponibilizadas pelo módulo Concurrent deverão respeitar integralmente as regras da Capacidade de Mutação definidas pelo Sistema de Tipos e pela Semântica Operacional.

A Biblioteca Padrão não poderá fornecer mecanismos que contornem ou enfraqueçam essas garantias.

---

## 14.3 Primitivas de Concorrência

A Biblioteca Padrão poderá disponibilizar abstrações como:

* criação de tarefas (*tasks*);
* criação de novos fluxos de execução (*spawn*);
* canais de comunicação (*channels*);
* mecanismos de sincronização;
* executores (*executors*).

A especificação detalhada dessas primitivas poderá evoluir em versões futuras.

---

## 14.4 Comunicação

A troca de informações entre tarefas deverá ocorrer através de mecanismos explicitamente definidos pela Biblioteca Padrão.

Esses mecanismos deverão preservar:

* segurança de memória;
* isolamento entre Domains;
* integridade das identidades (`ObjectId`);
* regras da Capacidade de Mutação.

---

## 14.5 Independência da Implementação

A implementação poderá utilizar:

* threads do sistema operacional;
* fibras;
* corrotinas;
* escalonadores próprios;
* outras estratégias equivalentes.

A escolha pertence exclusivamente ao Runtime Mínimo e à implementação da Biblioteca Padrão.

---

# 15. Módulo Reflection

O módulo **Reflection** reúne os mecanismos destinados à inspeção de informações estruturais dos programas.

Seu objetivo é fornecer acesso controlado aos metadados produzidos pelo compilador, preservando as garantias de segurança da linguagem.

---

## 15.1 Objetivos

O módulo Reflection deverá fornecer mecanismos para consulta de informações como:

* tipos;
* classes;
* interfaces;
* traits;
* métodos;
* propriedades;
* atributos;
* módulos.

---

## 15.2 Escopo

Reflection destina-se à inspeção de metadados.

A utilização desse módulo não poderá modificar:

* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* a arquitetura baseada em Domains.

A reflexão constitui um mecanismo de observação, não de alteração da estrutura do programa.

---

## 15.3 Informações Disponíveis

As informações disponibilizadas por Reflection dependerão dos metadados preservados durante a compilação.

A especificação não obriga todas as implementações a disponibilizarem o mesmo conjunto de informações, desde que os contratos públicos sejam respeitados.

---

## 15.4 Integração com o Compilador

O módulo Reflection poderá utilizar metadados produzidos durante a compilação.

A geração, armazenamento e disponibilização desses metadados pertencem ao compilador e ao Runtime Mínimo.

---

## 15.5 Evolução

Novos recursos de reflexão poderão ser incorporados em versões futuras da Biblioteca Padrão.

Essa evolução deverá preservar compatibilidade com os contratos públicos existentes e respeitar os princípios arquiteturais estabelecidos pela linguagem.

---

# 16. Tratamento de Erros

A Biblioteca Padrão utiliza exclusivamente os mecanismos de tratamento de erros definidos pela linguagem, especialmente `Result<T,E>` e `Optional<T>`.

O objetivo é fornecer uma abordagem uniforme, previsível e consistente para representar operações que podem falhar, preservando integralmente o Sistema de Tipos e a Semântica Operacional.

Mecanismos relacionados a falhas da própria infraestrutura de execução pertencem ao Runtime Mínimo e não fazem parte da API pública da Biblioteca Padrão.

---

## 16.1 Princípios

Operações sujeitas a falhas esperadas deverão utilizar, preferencialmente, o tipo `Result<T,E>` definido pelo Sistema de Tipos.

A Biblioteca Padrão não deverá introduzir mecanismos alternativos de tratamento de erros que contrariem a especificação da linguagem.

---

## 16.2 Erros Recuperáveis

Falhas que podem ser tratadas pelo programa deverão ser representadas explicitamente.

Exemplos incluem:

* abertura de arquivos inexistentes;
* falhas de comunicação;
* erros de entrada e saída;
* conversões inválidas;
* recursos indisponíveis.

A responsabilidade pelo tratamento dessas situações pertence ao código da aplicação.

---

## 16.3 Ausência de Valor

Situações em que um valor pode legitimamente não existir deverão utilizar `Optional<T>`.

O uso de `Optional<T>` não representa uma condição de erro, mas apenas a ausência explícita de um resultado.

---

## 16.4 Consistência

Todos os módulos da Biblioteca Padrão deverão utilizar critérios uniformes para representar falhas.

Operações semanticamente equivalentes deverão utilizar os mesmos mecanismos de tratamento de erros.

---

## 16.5 Evolução

Novas categorias de erro poderão ser incorporadas à Biblioteca Padrão sem alterar os princípios definidos neste documento.

---

# 17. Internacionalização

A Biblioteca Padrão deverá fornecer mecanismos básicos para internacionalização e localização de aplicações.

Seu objetivo é permitir que programas sejam desenvolvidos de forma independente de idioma, região ou convenções culturais específicas.

---

## 17.1 Objetivos

O módulo de internacionalização deverá oferecer suporte para:

* representação de localidades (*Locales*);
* formatação de datas e horários;
* formatação de números;
* formatação de moedas;
* representação textual adaptada à região.

---

## 17.2 Unicode

Toda manipulação de texto realizada pela Biblioteca Padrão deverá utilizar Unicode como base.

Conversões entre codificações deverão ser disponibilizadas quando necessário, preservando consistência entre plataformas.

As funcionalidades deste capítulo utilizam as abstrações disponibilizadas pelo módulo Text para representação e manipulação de cadeias de caracteres.

As regras de Unicode definidas pelo módulo Text permanecem válidas para todas as operações de internacionalização.

---

## 17.3 Locales

A Biblioteca Padrão poderá disponibilizar abstrações para representar configurações regionais.

Essas abstrações poderão influenciar operações como:

* ordenação;
* comparação textual;
* formatação;
* apresentação de dados.

A especificação detalhada dos Locales poderá evoluir em versões futuras.

---

## 17.4 Independência da Plataforma

As APIs públicas de internacionalização deverão apresentar comportamento consistente em todas as plataformas oficialmente suportadas.

Diferenças específicas de sistemas operacionais deverão permanecer encapsuladas pela implementação.

---

## 17.5 Evolução

Novos recursos relacionados à internacionalização poderão ser adicionados em versões futuras da Biblioteca Padrão, preservando compatibilidade com as interfaces públicas existentes.

---

# 18. Relação com o Runtime Mínimo

A Biblioteca Padrão e o Runtime Mínimo desempenham papéis distintos e complementares na arquitetura da Capi.

Enquanto a Biblioteca Padrão fornece abstrações reutilizáveis para o desenvolvimento de aplicações, o Runtime Mínimo fornece a infraestrutura necessária para que essas abstrações possam operar sobre a plataforma de execução.

---

## 18.1 Responsabilidades

Compete à Biblioteca Padrão:

* definir APIs públicas;
* fornecer implementações reutilizáveis;
* encapsular funcionalidades de uso geral;
* oferecer abstrações independentes da plataforma.

Compete ao Runtime Mínimo:

* fornecer acesso aos recursos da plataforma;
* materializar o Modelo de Memória;
* suportar operações fundamentais da linguagem;
* disponibilizar serviços necessários à execução dos programas.

---

## 18.2 Comunicação

Sempre que uma funcionalidade depender diretamente da plataforma de execução, a Biblioteca Padrão deverá utilizar exclusivamente as interfaces públicas disponibilizadas pelo Runtime Mínimo.

Essa separação reduz o acoplamento entre os componentes da linguagem e facilita a portabilidade entre diferentes ambientes.

---

## 18.3 Independência

A Biblioteca Padrão não deverá assumir detalhes internos do Runtime Mínimo.

Da mesma forma, o Runtime Mínimo não deverá impor estruturas específicas à Biblioteca Padrão além das interfaces oficialmente definidas.

Essa independência permite que ambos evoluam de forma coordenada, porém desacoplada.

---

## 18.4 Implementações

Diferentes implementações da Capi poderão fornecer versões distintas do Runtime Mínimo.

A Biblioteca Padrão deverá permanecer compatível com qualquer Runtime que respeite os contratos definidos pela especificação oficial.

---

## 18.5 Princípio da Separação

A Biblioteca Padrão especifica **o que** é disponibilizado ao programador.

O Runtime Mínimo define **como** essas funcionalidades interagem com a plataforma de execução.

Essa separação reforça o princípio arquitetural adotado em toda a especificação da Capi: as garantias da linguagem permanecem estáveis, enquanto os mecanismos de implementação podem evoluir livremente.

---

# 19. Garantias da Biblioteca Padrão

A Biblioteca Padrão constitui parte integrante do ecossistema oficial da Capi e deverá preservar integralmente todas as garantias estabelecidas pelos documentos anteriores da especificação.

Independentemente da implementação adotada, nenhum componente da Biblioteca Padrão poderá modificar ou enfraquecer os princípios fundamentais da linguagem.

---

## 19.1 Preservação do Sistema de Tipos

Todos os componentes da Biblioteca Padrão deverão respeitar integralmente o Sistema de Tipos definido no Documento 02.

Isso inclui, entre outros aspectos:

* compatibilidade entre tipos;
* generics;
* variância;
* interfaces;
* traits;
* herança;
* regras de conversão;
* identidade dos objetos.

A Biblioteca Padrão não poderá introduzir regras próprias incompatíveis com o Sistema de Tipos da linguagem.

---

## 19.2 Preservação do Modelo de Memória

Os componentes da Biblioteca Padrão deverão operar de forma compatível com o Modelo de Memória definido no Documento 03.

Em particular, deverão preservar:

* Domains;
* `ObjectId`;
* lifetime;
* isolamento entre Domains;
* organização lógica dos objetos.

Nenhuma biblioteca poderá criar mecanismos alternativos de gerenciamento de memória que contrariem a especificação oficial.

---

## 19.3 Preservação da Semântica Operacional

As operações disponibilizadas pela Biblioteca Padrão deverão respeitar integralmente a Semântica Operacional da Capi.

Isso inclui:

* criação e destruição de objetos;
* chamadas de métodos;
* tratamento de erros;
* concorrência;
* sincronização;
* regras de visibilidade.

O comportamento observável dos programas deverá permanecer consistente independentemente da implementação utilizada.

---

## 19.4 Compatibilidade entre Implementações

Implementações distintas da Biblioteca Padrão poderão utilizar algoritmos, estruturas de dados e estratégias internas diferentes.

Entretanto, todas deverão preservar os contratos públicos definidos por esta especificação.

Diferenças de desempenho ou de organização interna não poderão alterar a semântica observável das APIs.

---

## 19.5 Evolução Compatível

Novos módulos, tipos e funcionalidades poderão ser incorporados à Biblioteca Padrão.

Essa evolução deverá preservar:

* estabilidade das interfaces públicas;
* compatibilidade semântica;
* coerência arquitetural;
* integração com os demais documentos da especificação.

---

# 20. Princípio da Separação entre Garantias e Implementação

Assim como os demais documentos da especificação da Capi, este documento distingue explicitamente entre as garantias oferecidas pela Biblioteca Padrão e os mecanismos utilizados para implementá-las.

A especificação define os contratos públicos e o comportamento esperado das APIs.

A implementação define como esses contratos serão concretizados em cada plataforma.

---

## 20.1 Garantias

Constituem garantias obrigatórias da Biblioteca Padrão:

* comportamento consistente das APIs públicas;
* respeito ao Sistema de Tipos;
* respeito ao Modelo de Memória;
* respeito à Semântica Operacional;
* compatibilidade entre plataformas oficialmente suportadas;
* integração com o Runtime Mínimo através de interfaces públicas.

Essas garantias fazem parte da especificação da linguagem.

---

## 20.2 Implementação

Pertencem exclusivamente à implementação:

* algoritmos utilizados;
* estruturas internas de dados;
* estratégias de otimização;
* utilização de recursos específicos da plataforma;
* organização física dos módulos;
* integração com bibliotecas nativas.

Cada implementação poderá adotar soluções diferentes, desde que preserve integralmente os contratos públicos definidos neste documento.

---

## 20.3 Evolução

A implementação da Biblioteca Padrão poderá evoluir continuamente.

Novas estratégias internas, otimizações e melhorias poderão ser incorporadas sem alterar o comportamento esperado das APIs públicas.

Esse princípio assegura que a evolução tecnológica da Biblioteca Padrão permaneça independente da evolução da especificação da linguagem.

---

# 21. Não Objetivos

Este documento especifica exclusivamente a organização e os contratos públicos da Biblioteca Padrão da Capi.

Diversos aspectos permanecem deliberadamente fora de seu escopo.

---

## 21.1 Implementação Interna

Este documento não especifica:

* algoritmos internos;
* estruturas de dados utilizadas;
* estratégias de otimização;
* gerenciamento interno de memória;
* formatos binários.

Esses aspectos pertencem exclusivamente à implementação.

---

## 21.2 Bibliotecas Especializadas

Não fazem parte da Biblioteca Padrão:

* frameworks Web;
* acesso a bancos de dados;
* bibliotecas gráficas;
* inteligência artificial;
* aprendizado de máquina;
* computação científica;
* protocolos específicos de aplicação.

Esses recursos pertencem ao ecossistema de pacotes da linguagem.

---

## 21.3 Recursos Dependentes da Plataforma

Funcionalidades específicas de sistemas operacionais ou arquiteturas deverão permanecer encapsuladas pelo Runtime Mínimo ou por bibliotecas especializadas.

A Biblioteca Padrão deverá expor apenas abstrações portáveis.

---

## 21.4 Políticas de Implementação

A especificação não determina:

* desempenho mínimo;
* consumo de memória;
* algoritmos obrigatórios;
* organização física dos módulos;
* tecnologias utilizadas na implementação.

Essas decisões pertencem exclusivamente aos mantenedores de cada implementação.

---

# 22. Considerações Finais

A Biblioteca Padrão representa a principal camada de reutilização da linguagem Capi.

Seu objetivo é disponibilizar um conjunto reduzido, consistente e bem definido de componentes fundamentais, capazes de atender às necessidades da maioria das aplicações sem comprometer a simplicidade da linguagem.

Ao separar claramente a Biblioteca Padrão do Sistema de Tipos, do Modelo de Memória, da Semântica Operacional, do Compilador e do Runtime Mínimo, a arquitetura da Capi preserva uma divisão clara de responsabilidades entre seus componentes.

Essa separação favorece a evolução independente de cada parte da plataforma, permitindo que novas funcionalidades sejam incorporadas sem comprometer as garantias estabelecidas pela especificação.

A implementação de referência da Biblioteca Padrão, escrita predominantemente na própria Capi, reforça a expressividade da linguagem e demonstra que seus recursos são suficientes para construir abstrações reutilizáveis sem depender de mecanismos privilegiados do compilador.

Com a conclusão deste documento, ficam definidos os princípios e a organização da Biblioteca Padrão oficial da Capi. O documento seguinte especificará o **Runtime Mínimo**, responsável por fornecer a infraestrutura necessária para materializar os conceitos definidos ao longo de toda a especificação da linguagem e dar suporte à execução das aplicações.
