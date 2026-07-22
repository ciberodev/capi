# 11 — Toolchain (capi, capic, capi fmt, capi test, capi doc)

## Especificação da Linguagem Capi

Versão: 1.0 (Draft)

---

# 1. Introdução

Uma linguagem moderna não é composta apenas por sua sintaxe, compilador e ambiente de execução.

A experiência de desenvolvimento depende de um conjunto integrado de ferramentas capazes de apoiar todas as etapas do ciclo de vida de um projeto, desde sua criação até a compilação, testes, documentação e distribuição.

A Capi define uma **Toolchain oficial**, composta por ferramentas padronizadas que compartilham uma experiência de uso consistente e integrada.

---

## 1.1 Objetivo

Este documento especifica a Toolchain oficial da linguagem Capi.

São objetivos deste documento:

* definir as ferramentas oficiais da linguagem;
* estabelecer suas responsabilidades;
* padronizar a experiência de desenvolvimento;
* especificar a organização de projetos;
* definir a interação entre as ferramentas da plataforma.

A implementação de cada ferramenta pertence exclusivamente à Toolchain utilizada.

---

## 1.2 Escopo

Este documento especifica exclusivamente:

* a organização da Toolchain;
* as responsabilidades de cada ferramenta;
* a estrutura oficial de projetos;
* os comandos públicos disponibilizados ao desenvolvedor;
* a integração entre as ferramentas.

Não fazem parte deste documento:

* a implementação interna do Compilador;
* o Runtime Mínimo;
* o Gerenciador de Pacotes;
* detalhes da Biblioteca Padrão;
* ambientes de desenvolvimento (IDEs);
* sistemas de integração contínua.

---

## 1.3 Filosofia da Toolchain

A Toolchain da Capi segue três princípios fundamentais:

* simplicidade;
* consistência;
* previsibilidade.

Sempre que possível, operações relacionadas ao desenvolvimento deverão utilizar comandos padronizados e comportamento uniforme.

A linguagem procura reduzir a necessidade de ferramentas externas para tarefas comuns, oferecendo uma experiência integrada desde a criação do projeto até sua distribuição.

---

## 1.4 Componentes

A Toolchain oficial é composta pelos seguintes componentes:

* `capi` — interface principal da Toolchain;
* `capic` — compilador oficial da linguagem;
* `capi fmt` — formatação de código-fonte;
* `capi test` — execução de testes;
* `capi doc` — geração de documentação.

Implementações poderão fornecer ferramentas adicionais, desde que preservem a compatibilidade com os contratos públicos definidos por esta especificação.

---

## 1.5 Relação com os Demais Documentos

A Toolchain utiliza os componentes especificados nos documentos anteriores.

Em particular:

* o Compilador (Documento 06);
* a Intermediate Representation (Documento 07);
* a Biblioteca Padrão (Documento 08);
* o Runtime Mínimo (Documento 09);
* a ABI e Interoperabilidade (Documento 10).

Este documento especifica apenas como essas ferramentas são organizadas e disponibilizadas ao desenvolvedor.

---

# 2. Visão Geral da Toolchain

A Toolchain da Capi foi projetada para oferecer uma experiência uniforme em todas as plataformas suportadas.

Embora internamente seja composta por diversos componentes especializados, ela é apresentada ao desenvolvedor como um ambiente integrado.

Essa abordagem reduz a complexidade do uso cotidiano sem limitar a evolução individual de cada ferramenta.

---

## 2.1 Ferramenta Principal

A interface oficial da Toolchain é a ferramenta `capi`.

Ela representa o ponto de entrada recomendado para todas as atividades de desenvolvimento.

Operações como:

* compilação;
* execução;
* testes;
* formatação;
* documentação;
* gerenciamento de projetos;

deverão ser disponibilizadas através dessa interface.

---

## 2.2 Ferramentas Especializadas

Embora a interface principal seja `capi`, a Toolchain poderá ser composta internamente por ferramentas especializadas.

Entre elas:

* `capic`;
* formatadores;
* geradores de documentação;
* executores de testes;
* ferramentas auxiliares.

A existência dessas ferramentas não altera a interface pública oferecida ao desenvolvedor.

---

## 2.3 Fluxo de Desenvolvimento

Um fluxo típico de desenvolvimento poderá envolver:

1. criação do projeto;
2. edição do código;
3. formatação;
4. compilação;
5. execução dos testes;
6. geração da documentação;
7. geração dos artefatos finais.

A Toolchain procura manter esse fluxo uniforme independentemente da plataforma utilizada.

---

## 2.4 Arquitetura Modular

Cada componente da Toolchain possui responsabilidades bem definidas.

Essa separação favorece:

* manutenção;
* evolução independente;
* reutilização;
* integração com ferramentas externas.

A implementação interna dessa arquitetura pertence exclusivamente à Toolchain.

---

## 2.5 Independência da Implementação

A especificação define apenas o comportamento público das ferramentas.

Aspectos como:

* linguagem utilizada para implementá-las;
* organização interna;
* protocolos de comunicação;
* bibliotecas utilizadas;

pertencem exclusivamente à implementação.

---

# 3. `capi`

A ferramenta `capi` constitui a interface oficial da Toolchain da linguagem.

Seu objetivo é oferecer ao desenvolvedor um ponto único de acesso para todas as operações relacionadas ao desenvolvimento de aplicações Capi.

A ferramenta `capi` atua como um orquestrador da Toolchain.

Por meio de seus subcomandos, ela coordena a utilização dos componentes especializados da plataforma, como o compilador `capic`, o formatador oficial, o executor de testes e o gerador de documentação.

Essa organização permanece transparente ao desenvolvedor, que interage preferencialmente apenas com a interface pública da Toolchain.

---

## 3.1 Objetivo

A ferramenta `capi` centraliza as operações mais comuns realizadas durante o desenvolvimento.

Entre elas:

* criação de projetos;
* compilação;
* execução;
* testes;
* formatação;
* geração de documentação;
* gerenciamento do ambiente de desenvolvimento.

---

## 3.2 Estrutura Geral

A ferramenta organiza suas funcionalidades por meio de subcomandos.

Cada subcomando representa uma operação específica da Toolchain.

Essa organização favorece consistência, descoberta de funcionalidades e facilidade de aprendizado.

---

## 3.3 Subcomandos

A especificação define os seguintes subcomandos públicos:

* `build` — compila o projeto e produz os artefatos finais.
* `run` — compila (quando necessário) e executa o programa principal.
* `fmt` — aplica o estilo oficial de formatação ao código-fonte.
* `test` — localiza, compila e executa os testes do projeto.
* `doc` — gera a documentação do projeto.
* `clean` — remove artefatos produzidos pela Toolchain.
* `check` — realiza análise léxica, sintática e semântica sem gerar código executável.
* `new` — cria um novo projeto utilizando a estrutura oficial.
* `init` — inicializa um diretório existente como um projeto Capi.

Novos subcomandos poderão ser incorporados em versões futuras da Toolchain sem alterar a arquitetura geral da interface.

---

## 3.4 Configuração

A ferramenta `capi` utiliza as informações do projeto para determinar seu comportamento.

Entre elas:

* configuração do projeto;
* dependências;
* perfil de compilação;
* plataforma de destino;
* opções da Toolchain.

O formato dessas configurações será especificado posteriormente neste documento.

---

## 3.5 Extensibilidade

A arquitetura da ferramenta foi concebida para permitir evolução contínua.

Novas funcionalidades poderão ser incorporadas sem modificar a experiência de uso existente.

Esse princípio preserva a estabilidade da interface pública enquanto permite a evolução da Toolchain ao longo do tempo.

---

# 4. `capic`

O `capic` é o compilador oficial da linguagem Capi.

Sua responsabilidade é transformar código-fonte Capi em artefatos executáveis ou bibliotecas compatíveis com a ABI oficial da linguagem.

Embora a ferramenta `capi` seja a interface recomendada para os desenvolvedores, o `capic` constitui o componente responsável pela compilação propriamente dita e pode ser utilizado diretamente por ferramentas, IDEs e processos automatizados.

---

## 4.1 Objetivo

O `capic` é responsável por executar o pipeline completo de compilação definido pelos documentos anteriores.

Entre suas responsabilidades encontram-se:

* análise léxica;
* análise sintática;
* análise semântica;
* geração da Intermediate Representation (IR);
* otimizações;
* geração de código;
* produção dos artefatos finais.

O comportamento do compilador deverá permanecer consistente com todas as especificações da linguagem.

---

## 4.2 Pipeline de Compilação

O `capic` executa as fases de compilação na ordem definida pela Arquitetura do Compilador (Documento 06).

De forma geral, o pipeline compreende:

1. leitura dos arquivos-fonte;
2. análise léxica;
3. análise sintática;
4. construção da AST;
5. análise semântica;
6. geração da IR;
7. otimizações;
8. geração de código;
9. ligação dos artefatos;
10. produção do executável ou biblioteca.

A implementação poderá introduzir fases intermediárias adicionais, desde que preserve o comportamento definido pela especificação.

---

## 4.3 Opções de Compilação

O compilador poderá oferecer opções para controlar seu comportamento.

Entre elas:

* geração de informações de depuração;
* seleção do perfil de compilação;
* ativação ou desativação de otimizações;
* seleção da arquitetura de destino;
* geração de artefatos intermediários.

A sintaxe dessas opções pertence à implementação da Toolchain.

---

## 4.4 Plataformas de Destino

O `capic` poderá produzir código para diferentes plataformas.

Entre elas:

* sistemas desktop;
* servidores;
* dispositivos móveis;
* sistemas embarcados;
* WebAssembly;
* outras arquiteturas suportadas.

Cada plataforma deverá respeitar a ABI correspondente e os contratos definidos pela especificação da linguagem.

---

## 4.5 Integração com o Runtime

O código produzido pelo `capic` deverá integrar-se corretamente ao Runtime Mínimo especificado no Documento 09.

Essa integração inclui:

* inicialização da execução;
* gerenciamento dos Domains;
* suporte ao Sistema de Tipos;
* interoperabilidade com a Biblioteca Padrão;
* utilização da ABI oficial.

Os mecanismos concretos dessa integração pertencem exclusivamente à implementação.

---

# 5. `capi fmt`

A ferramenta `capi fmt` define o formatador oficial da linguagem.

Seu objetivo é produzir código-fonte uniforme, legível e consistente, reduzindo diferenças puramente estéticas entre projetos e facilitando revisões, manutenção e colaboração.

---

## 5.1 Objetivo

O formatador oficial aplica automaticamente o estilo de código definido para a linguagem.

Seu uso elimina divergências relacionadas a:

* indentação;
* espaçamento;
* quebras de linha;
* alinhamento;
* organização visual do código.

O comportamento do formatador deverá ser determinístico.

---

## 5.2 Estilo Oficial

A Capi define um estilo oficial de formatação.

Esse estilo deverá ser utilizado por padrão em toda a Toolchain, garantindo que projetos diferentes apresentem aparência consistente.

A especificação do estilo poderá evoluir em versões futuras da linguagem, preservando compatibilidade sempre que possível.

As regras de formatação são derivadas das convenções sintáticas estabelecidas pelo Documento 04 — Sintaxe da Linguagem.

O formatador não introduz novas regras para a linguagem, limitando-se a aplicar automaticamente um estilo oficial compatível com sua sintaxe.

---

## 5.3 Regras de Formatação

O formatador poderá reorganizar exclusivamente aspectos de apresentação do código.

Entre eles:

* espaços em branco;
* indentação;
* posicionamento de chaves;
* organização de listas;
* alinhamento de parâmetros;
* quebra de expressões longas.

Nenhuma transformação poderá alterar a semântica do programa.

---

## 5.4 Configuração

A Toolchain poderá permitir configurações relacionadas ao comportamento do formatador.

Essas configurações destinam-se principalmente à seleção de versões do estilo oficial e à compatibilidade entre projetos.

A especificação não incentiva personalizações extensivas que comprometam a uniformidade do ecossistema.

---

## 5.5 Evolução

O estilo oficial poderá evoluir ao longo do tempo.

Alterações significativas deverão ocorrer de maneira controlada, permitindo que projetos existentes continuem sendo formatados de forma previsível.

A Toolchain poderá oferecer mecanismos para selecionar diferentes versões do formatador quando necessário.

---

# 6. `capi test`

A ferramenta `capi test` constitui o mecanismo oficial para execução de testes na linguagem Capi.

Seu objetivo é oferecer uma forma padronizada, previsível e integrada de validar o comportamento de programas, bibliotecas e componentes desenvolvidos na linguagem.

A ferramenta faz parte da Toolchain oficial e integra-se naturalmente ao processo de compilação e desenvolvimento.

---

## 6.1 Objetivo

A ferramenta `capi test` é responsável por:

* localizar testes pertencentes ao projeto;
* compilar os testes quando necessário;
* executar os casos de teste;
* apresentar os resultados obtidos;
* produzir informações úteis para depuração de falhas.

Seu comportamento deverá ser uniforme em todas as plataformas suportadas.

---

## 6.2 Organização dos Testes

A Toolchain define uma organização padronizada para os testes de um projeto.

Poderão existir diferentes categorias, como:

* testes unitários;
* testes de integração;
* testes de documentação;
* testes de regressão.

A estrutura física utilizada para organizar esses testes será definida pela organização oficial de projetos apresentada neste documento.

---

## 6.3 Execução

A ferramenta deverá localizar automaticamente os testes pertencentes ao projeto.

Também poderá permitir:

* execução de um único teste;
* execução de grupos específicos;
* filtragem por nome;
* execução paralela, quando suportada;
* repetição de testes.

A estratégia utilizada para localizar e executar os testes pertence à implementação.

---

## 6.4 Relatórios

Ao término da execução, a ferramenta deverá apresentar informações suficientes para permitir a análise dos resultados.

Entre elas:

* quantidade de testes executados;
* quantidade de sucessos;
* quantidade de falhas;
* duração da execução;
* identificação dos testes que falharam.

A Toolchain poderá oferecer formatos adicionais de relatório destinados à integração com outras ferramentas.

---

## 6.5 Integração Contínua

A ferramenta foi projetada para integração com ambientes de automação.

Seu comportamento deverá ser determinístico e produzir códigos de retorno apropriados para utilização em:

* integração contínua;
* entrega contínua;
* pipelines automatizados;
* sistemas de build.

A forma de integração com essas plataformas pertence à implementação.

---

# 7. `capi doc`

A ferramenta `capi doc` é responsável pela geração da documentação oficial de projetos escritos em Capi.

Seu objetivo é transformar comentários estruturados presentes no código-fonte em documentação navegável, consistente e integrada à Biblioteca Padrão e às bibliotecas desenvolvidas pela comunidade.

---

## 7.1 Objetivo

A ferramenta deverá gerar documentação a partir das informações disponíveis no projeto.

Entre elas:

* módulos;
* tipos;
* funções;
* interfaces;
* traits;
* constantes;
* exemplos de utilização.

A documentação gerada deverá refletir fielmente a API pública do projeto.

---

## 7.2 Comentários de Documentação

A linguagem define um formato específico para comentários destinados à documentação.

Esses comentários fazem parte do código-fonte e acompanham os elementos públicos do projeto.

A sintaxe utilizada para esses comentários pertence ao Documento 04 — Sintaxe da Linguagem.

---

## 7.3 Geração da Documentação

A ferramenta deverá produzir documentação estruturada e navegável.

Implementações poderão gerar diferentes formatos, incluindo:

* HTML;
* Markdown;
* JSON;
* outros formatos equivalentes.

O formato padrão da Toolchain deverá privilegiar acessibilidade e facilidade de navegação.

---

## 7.4 Documentação da API

A documentação gerada deverá incluir, sempre que disponíveis:

* assinaturas;
* descrições;
* parâmetros;
* valores de retorno;
* exemplos;
* referências cruzadas;
* informações de versionamento.

A apresentação dessas informações pertence à implementação.

---

## 7.5 Publicação

A Toolchain poderá fornecer mecanismos para publicação da documentação produzida.

Esses mecanismos poderão integrar-se a:

* repositórios de pacotes;
* servidores de documentação;
* sistemas de integração contínua;
* plataformas de hospedagem.

A publicação da documentação não altera seu conteúdo nem modifica os contratos públicos definidos pelo projeto.

---

# 8. Diagnósticos

Os diagnósticos constituem a principal forma de comunicação entre a Toolchain e o desenvolvedor.

Seu objetivo é identificar problemas, explicar suas causas e fornecer informações suficientes para que possam ser corrigidos de forma rápida e segura.

Todas as ferramentas oficiais da Toolchain deverão produzir diagnósticos claros, consistentes e previsíveis.

---

## 8.1 Objetivo

Os diagnósticos têm como finalidade:

* identificar erros durante o desenvolvimento;
* localizar precisamente sua origem;
* facilitar a compreensão da causa do problema;
* orientar o desenvolvedor na sua correção;
* manter comportamento uniforme entre todas as ferramentas da Toolchain.

A qualidade dos diagnósticos é considerada parte integrante da experiência de desenvolvimento da linguagem.

---

## 8.2 Estrutura dos Diagnósticos

Sempre que possível, um diagnóstico deverá conter:

* categoria do problema;
* localização no código-fonte;
* descrição objetiva;
* contexto relevante;
* informações complementares;
* sugestões de correção, quando aplicáveis.

A apresentação visual dessas informações pertence à implementação da Toolchain.

---

## 8.3 Categorias

Os diagnósticos poderão ser classificados em diferentes categorias, incluindo:

* erros;
* avisos (*warnings*);
* informações;
* observações (*notes*).

Cada categoria representa um nível distinto de severidade e deverá ser apresentada de maneira consistente em toda a Toolchain.

---

## 8.4 Localização

Sempre que possível, os diagnósticos deverão indicar com precisão o ponto do código ao qual se referem.

Essa localização poderá incluir:

* arquivo;
* linha;
* coluna;
* trecho do código correspondente;
* elementos relacionados ao problema.

A forma de apresentação dessas informações pertence à implementação.

---

## 8.5 Consistência

Todas as ferramentas oficiais da Toolchain deverão utilizar um modelo uniforme de diagnósticos.

Independentemente da ferramenta responsável pela emissão da mensagem, o desenvolvedor deverá encontrar o mesmo padrão de organização, terminologia e classificação.

Essa consistência facilita o aprendizado da linguagem e reduz o esforço necessário para interpretar mensagens produzidas por diferentes componentes da Toolchain.

---

## 8.6 Recuperação

Sempre que tecnicamente possível, as ferramentas da Toolchain deverão continuar sua análise após identificar um problema.

O objetivo é permitir que múltiplos diagnósticos sejam apresentados em uma única execução, reduzindo o número de ciclos necessários para correção do código.

A estratégia utilizada para recuperação após erros pertence exclusivamente à implementação.

---

## 8.7 Evolução

Os diagnósticos poderão evoluir continuamente para fornecer informações mais precisas e úteis ao desenvolvedor.

Novas categorias, formatos de apresentação e mecanismos de auxílio poderão ser incorporados sem alterar a semântica da linguagem nem os contratos públicos definidos por esta especificação.

---

# 9. Organização dos Projetos

A Toolchain da Capi define uma organização padrão para projetos.

Essa organização tem como objetivos:

* facilitar a compreensão dos projetos;
* padronizar a localização dos arquivos;
* simplificar a atuação das ferramentas da Toolchain;
* favorecer a interoperabilidade entre bibliotecas e aplicações.

Embora implementações possam oferecer recursos adicionais, a estrutura básica deverá permanecer compatível com esta especificação.

---

## 9.1 Estrutura do Projeto

Todo projeto Capi deverá possuir uma estrutura organizada e previsível.

A Toolchain define uma organização oficial para os diretórios e arquivos de um projeto, permitindo que todas as ferramentas localizem automaticamente o código-fonte, os testes, a documentação, os arquivos de configuração e os artefatos gerados durante a compilação.

Uma estrutura típica de projeto é apresentada a seguir:

```text
projeto/
├── capi.toml
├── src/
├── tests/
├── docs/
└── target/
```

Nessa organização:

- `capi.toml` contém a configuração do projeto;
- `src/` contém o código-fonte da aplicação ou biblioteca;
- `tests/` reúne os testes do projeto;
- `docs/` armazena documentação complementar;
- `target/` contém os artefatos produzidos pela Toolchain.

Implementações poderão criar diretórios adicionais para uso interno, desde que preservem a estrutura oficial definida por esta especificação.

---

## 9.2 Workspaces

A Toolchain suporta projetos compostos por múltiplos módulos organizados em um único workspace.

Um workspace permite o desenvolvimento conjunto de:

* aplicações;
* bibliotecas;
* ferramentas;
* exemplos;
* testes compartilhados.

Os componentes pertencentes ao mesmo workspace poderão compartilhar configurações e dependências.

---

## 9.3 Bibliotecas

Projetos poderão produzir bibliotecas reutilizáveis.

Bibliotecas poderão ser utilizadas por:

* aplicações;
* outras bibliotecas;
* ferramentas da Toolchain;
* componentes do ecossistema.

Sua organização deverá seguir a mesma estrutura geral utilizada por aplicações.

---

## 9.4 Executáveis

Projetos destinados à geração de executáveis deverão definir claramente seus pontos de entrada.

A Toolchain deverá ser capaz de identificar automaticamente:

* o executável principal;
* executáveis auxiliares;
* ferramentas internas do projeto.

Os mecanismos utilizados para essa identificação pertencem à implementação.

---

## 9.5 Recursos

Projetos poderão conter recursos auxiliares utilizados durante a execução ou distribuição.

Entre eles:

* arquivos de configuração;
* imagens;
* modelos;
* arquivos de dados;
* documentação complementar.

A forma como esses recursos são incorporados ao artefato final pertence à implementação da Toolchain.

---

# 10. Arquivo de Configuração

A Toolchain utiliza um arquivo de configuração para descrever as características de cada projeto.

Esse arquivo constitui a principal fonte de informações utilizadas pelas ferramentas durante compilação, testes, documentação e distribuição.

Sua estrutura deverá permanecer estável e facilmente legível tanto por pessoas quanto por ferramentas automatizadas.

---

## 10.1 `capi.toml`

O arquivo oficial de configuração do projeto é denominado `capi.toml`.

Ele reúne os metadados necessários para que a Toolchain compreenda a organização do projeto e execute corretamente suas operações.

O formato TOML foi escolhido por oferecer simplicidade, legibilidade e ampla adoção em ecossistemas modernos.

---

## 10.2 Metadados

Entre os metadados que poderão ser definidos encontram-se:

* nome do projeto;
* versão;
* descrição;
* autores;
* licença;
* repositório;
* informações de publicação.

A especificação detalhada desses campos poderá evoluir em versões futuras da Toolchain.

---

## 10.3 Dependências

O arquivo de configuração declara as dependências utilizadas pelo projeto.

Essas dependências poderão referenciar:

* bibliotecas locais;
* bibliotecas do ecossistema;
* versões específicas;
* dependências utilizadas apenas durante o desenvolvimento.

Os mecanismos de resolução dessas dependências serão definidos pelo Gerenciador de Pacotes (Documento 12).

---

## 10.4 Plataformas de Destino

Projetos poderão definir configurações específicas para diferentes plataformas.

Entre elas:

* arquitetura;
* sistema operacional;
* Runtime;
* opções de compilação;
* recursos opcionais.

A interpretação dessas configurações pertence à Toolchain.

---

## 10.5 Perfis

O arquivo de configuração poderá definir diferentes perfis de construção.

Cada perfil poderá estabelecer parâmetros como:

* nível de otimização;
* geração de símbolos de depuração;
* verificações adicionais;
* opções específicas da Toolchain.

Os perfis oficiais são definidos no capítulo seguinte.

---

# 11. Perfis de Build

A Toolchain da Capi define perfis de build para atender diferentes fases do ciclo de desenvolvimento.

Os perfis de build representam conjuntos predefinidos das opções de compilação disponibilizadas pelo Compilador (Documento 06), incluindo níveis de otimização, geração de símbolos de depuração e verificações adicionais.

A correspondência entre cada perfil e as opções efetivamente utilizadas pertence à implementação da Toolchain.

Cada perfil representa um conjunto consistente de configurações utilizadas durante a compilação, permitindo adaptar o comportamento da Toolchain sem modificar o código-fonte do projeto.

Implementações poderão oferecer perfis adicionais, preservando os perfis oficiais definidos por esta especificação.

---

## 11.1 Debug

O perfil **Debug** destina-se ao desenvolvimento e à depuração de aplicações.

Esse perfil prioriza:

* geração de símbolos de depuração;
* mensagens detalhadas de diagnóstico;
* otimizações reduzidas;
* facilidade de inspeção durante a execução.

Seu objetivo é favorecer a identificação e correção de problemas durante o desenvolvimento.

---

## 11.2 Release

O perfil **Release** destina-se à distribuição de aplicações.

Esse perfil prioriza:

* otimizações de desempenho;
* redução do tamanho dos artefatos;
* eliminação de informações desnecessárias para depuração;
* eficiência da execução.

O comportamento observado pelos programas permanece idêntico ao perfil Debug, diferindo apenas nos aspectos relacionados à implementação.

---

## 11.3 Test

O perfil **Test** é utilizado durante a execução dos testes do projeto.

Além das configurações necessárias para compilação, poderá habilitar mecanismos destinados à validação do comportamento do programa, incluindo verificações adicionais e geração de informações úteis para diagnóstico de falhas.

---

## 11.4 Benchmark

O perfil **Benchmark** destina-se à medição de desempenho.

Seu objetivo é minimizar interferências externas na execução das medições, produzindo resultados consistentes e reprodutíveis.

As ferramentas da Toolchain poderão oferecer suporte específico para coleta e apresentação dessas medições.

---

## 11.5 Perfis Personalizados

A Toolchain poderá permitir a criação de perfis personalizados.

Esses perfis poderão herdar configurações dos perfis oficiais e sobrescrever apenas os parâmetros desejados.

A forma como essa herança é implementada pertence exclusivamente à Toolchain.

---

# 12. Integração com o Ecossistema

A Toolchain da Capi foi concebida para atuar de forma integrada com os demais componentes da plataforma.

Embora cada componente possua responsabilidades próprias, a experiência oferecida ao desenvolvedor deve apresentar-se como um ambiente único e consistente.

---

## 12.1 Integração com o Runtime

A Toolchain deverá produzir artefatos compatíveis com o Runtime Mínimo especificado pela linguagem.

Essa integração inclui:

* inicialização da aplicação;
* utilização da ABI oficial;
* carregamento da Biblioteca Padrão;
* suporte ao Modelo de Memória.

Os mecanismos concretos dessa integração pertencem à implementação.

---

## 12.2 Integração com a Biblioteca Padrão

Toda aplicação compilada pela Toolchain deverá ser compatível com a Biblioteca Padrão oficial da linguagem.

A forma como essa biblioteca é localizada, compilada, vinculada ou distribuída pertence exclusivamente à implementação.

---

## 11.3 Integração com o Gerenciador de Pacotes

A resolução, obtenção e atualização de dependências pertencem ao Gerenciador de Pacotes especificado no Documento 12 — Ecossistema e Gerenciamento de Pacotes.

A Toolchain deverá integrar-se a esse componente para permitir:

* obtenção de dependências;
* resolução de versões;
* compilação de bibliotecas;
* gerenciamento do workspace.

---

## 12.4 Integração com IDEs

A Toolchain foi projetada para facilitar integração com ambientes de desenvolvimento.

Entre os recursos normalmente disponibilizados encontram-se:

* compilação incremental;
* diagnósticos;
* navegação entre símbolos;
* formatação automática;
* documentação contextual.

Os protocolos utilizados para essa integração pertencem exclusivamente à implementação.

---

## 12.5 Integração com Ferramentas Externas

A Toolchain poderá interoperar com ferramentas externas, incluindo:

* sistemas de build;
* plataformas de integração contínua;
* geradores de documentação;
* analisadores estáticos;
* ferramentas de distribuição.

Essa interoperabilidade deverá ocorrer por meio de interfaces públicas estáveis, preservando a independência entre a especificação da linguagem e as implementações.

---

# 13. Extensibilidade

A Toolchain foi projetada para evoluir continuamente.

Novas ferramentas, comandos e funcionalidades poderão ser incorporados sem comprometer a estabilidade da interface pública nem a compatibilidade com projetos existentes.

---

## 13.1 Plugins

Implementações poderão oferecer mecanismos para extensão da Toolchain através de plugins.

Esses plugins poderão adicionar novas funcionalidades sem alterar os contratos públicos definidos por esta especificação.

A arquitetura utilizada para suportar plugins pertence exclusivamente à implementação.

---

## 13.2 Ferramentas Externas

Ferramentas desenvolvidas pela comunidade poderão integrar-se à Toolchain utilizando suas interfaces públicas.

Essa integração poderá abranger:

* análise de código;
* geração de artefatos;
* automação;
* validação;
* instrumentação.

---

## 13.3 Novos Subcomandos

Versões futuras da Toolchain poderão incorporar novos subcomandos à interface `capi`.

Esses comandos deverão manter consistência com a organização geral da ferramenta e preservar a experiência de uso já estabelecida.

---

## 13.4 APIs Públicas

A Toolchain poderá disponibilizar APIs destinadas à integração com ferramentas externas.

Essas APIs deverão permanecer estáveis e documentadas, permitindo que outras aplicações utilizem os serviços da Toolchain sem depender de detalhes internos de implementação.

---

## 13.5 Evolução

A evolução da Toolchain deverá preservar:

* compatibilidade com projetos existentes;
* estabilidade dos comandos públicos;
* previsibilidade da experiência de desenvolvimento.

Mudanças incompatíveis deverão ser claramente identificadas e documentadas, permitindo uma transição controlada entre versões.

---

# 14. Não Objetivos

Este documento especifica exclusivamente a organização e o comportamento público da Toolchain oficial da linguagem Capi.

Diversos aspectos permanecem deliberadamente fora de seu escopo para preservar a separação entre a especificação da linguagem e as implementações concretas das ferramentas.

---

## 14.1 Ambientes de Desenvolvimento

Este documento não especifica ambientes de desenvolvimento (IDEs).

Aspectos como:

* interface gráfica;
* organização de painéis;
* navegação entre arquivos;
* assistentes de edição;
* integração visual com a Toolchain;

pertencem exclusivamente às ferramentas utilizadas pelo desenvolvedor.

---

## 14.2 Editores de Texto

A linguagem não exige nenhum editor específico.

A Toolchain deverá ser utilizável a partir de qualquer ambiente capaz de executar seus comandos oficiais.

Integrações específicas com editores pertencem exclusivamente às respectivas implementações.

---

## 14.3 Integração Contínua

Este documento não especifica plataformas de Integração Contínua (CI) ou Entrega Contínua (CD).

Embora a Toolchain tenha sido projetada para facilitar essa integração, aspectos como:

* definição de pipelines;
* execução distribuída;
* gerenciamento de agentes;
* publicação de artefatos;

pertencem às plataformas de automação utilizadas.

---

## 14.4 Sistema Operacional

A Toolchain deverá permanecer independente do sistema operacional.

A especificação não determina:

* estrutura de diretórios;
* formatos de caminhos;
* convenções de terminal;
* mecanismos de instalação;
* gerenciamento de permissões.

Esses aspectos pertencem às plataformas suportadas.

---

## 14.5 Implementações

A especificação não define:

* linguagem utilizada para implementar a Toolchain;
* arquitetura interna das ferramentas;
* protocolos de comunicação entre componentes;
* algoritmos internos;
* bibliotecas empregadas.

Implementações diferentes poderão adotar soluções distintas, desde que preservem integralmente os contratos públicos definidos neste documento.

---

# 15. Princípio da Separação entre Garantias e Implementação

Assim como os demais documentos da especificação da Capi, este documento distingue claramente entre os contratos públicos da Toolchain e os mecanismos utilizados para implementá-los.

A especificação define **o que** as ferramentas devem oferecer ao desenvolvedor.

Cada implementação define **como** esses serviços serão realizados.

---

## 15.1 Garantias

Constituem garantias da Toolchain:

* disponibilidade dos comandos públicos especificados;
* comportamento consistente entre plataformas suportadas;
* integração com os demais componentes da linguagem;
* compatibilidade com a estrutura oficial de projetos;
* estabilidade da interface pública.

Essas garantias fazem parte da especificação oficial da Capi.

---

## 15.2 Implementação

Pertencem exclusivamente à implementação:

* arquitetura interna das ferramentas;
* organização dos componentes da Toolchain;
* algoritmos utilizados;
* estratégias de compilação;
* mecanismos de cache;
* otimizações de desempenho.

Implementações diferentes poderão utilizar soluções completamente distintas para alcançar os mesmos resultados.

---

## 15.3 Evolução

A Toolchain poderá evoluir continuamente.

Novos comandos, novas ferramentas e novos recursos poderão ser incorporados sem alterar os contratos públicos já estabelecidos.

Sempre que possível, a evolução deverá preservar compatibilidade com projetos existentes.

---

## 15.4 Compatibilidade

A evolução da Toolchain não deverá exigir alterações desnecessárias no código-fonte das aplicações.

Mudanças incompatíveis deverão ser claramente identificadas e acompanhadas por mecanismos que facilitem a migração entre versões.

---

## 15.5 Estabilidade

A interface pública da Toolchain constitui um contrato entre a linguagem e seus desenvolvedores.

Sua evolução deverá ocorrer de forma controlada, preservando previsibilidade, consistência e estabilidade ao longo do tempo.

---

# 16. Considerações Finais

A Toolchain oficial da Capi representa o ponto de integração entre todos os componentes definidos pela especificação da linguagem.

Ao reunir compilação, testes, formatação, documentação e gerenciamento de projetos em um ambiente unificado, a Capi oferece uma experiência de desenvolvimento consistente, previsível e independente da plataforma utilizada.

A separação entre a interface pública da Toolchain e suas implementações permite que diferentes distribuições evoluam livremente, preservando sempre os contratos definidos por esta especificação.

Essa abordagem favorece a interoperabilidade entre ferramentas, a estabilidade do ecossistema e a longevidade dos projetos desenvolvidos na linguagem.

Com a conclusão deste documento, encontra-se especificada toda a infraestrutura oficial de desenvolvimento da Capi.

O documento seguinte, **12 — Ecossistema e Gerenciamento de Pacotes**, definirá a organização do repositório oficial de pacotes, o modelo de distribuição de bibliotecas, a resolução de dependências e os mecanismos de compartilhamento de software dentro do ecossistema da linguagem, completando a especificação da plataforma Capi.
