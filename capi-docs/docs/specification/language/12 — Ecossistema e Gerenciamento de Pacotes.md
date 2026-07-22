# 12 — Ecossistema e Gerenciamento de Pacotes

## Especificação da Linguagem Capi

Versão: 1.0 (Draft)

---

# 1. Introdução

Uma linguagem de programação moderna depende de um ecossistema capaz de facilitar o compartilhamento, reutilização e evolução de software.

Além da linguagem, da Toolchain e da Biblioteca Padrão, a Capi define um modelo oficial para organização, distribuição e gerenciamento de pacotes, permitindo que bibliotecas e aplicações sejam desenvolvidas de forma consistente e interoperável.

Este documento estabelece os princípios que orientam o ecossistema da linguagem e define os contratos públicos para gerenciamento de pacotes.

---

## 1.1 Objetivo

Este documento especifica o modelo oficial de ecossistema e gerenciamento de pacotes da linguagem Capi.

São objetivos deste documento:

* definir o conceito de pacote;
* estabelecer o modelo de distribuição de bibliotecas;
* especificar o gerenciamento de dependências;
* definir a organização do ecossistema oficial;
* estabelecer os contratos públicos entre a Toolchain e os registros de pacotes.

A implementação desses mecanismos pertence às ferramentas que compõem a plataforma.

---

## 1.2 Escopo

Este documento especifica exclusivamente:

* organização do ecossistema;
* conceito de pacote;
* gerenciamento de dependências;
* registros de pacotes;
* publicação e obtenção de bibliotecas;
* integração com a Toolchain.

Não fazem parte deste documento:

* implementação do Gerenciador de Pacotes;
* infraestrutura dos registros;
* protocolos de transporte;
* mecanismos internos de armazenamento;
* políticas administrativas dos registros.

---

## 1.3 Filosofia do Ecossistema

O ecossistema da Capi foi concebido para favorecer:

* simplicidade;
* interoperabilidade;
* reprodutibilidade;
* abertura;
* independência entre implementações.

A especificação define apenas os contratos necessários para permitir que diferentes implementações cooperem de maneira consistente.

---

## 1.4 Componentes

O ecossistema oficial é composto pelos seguintes elementos:

* pacotes;
* registros de pacotes;
* Gerenciador de Pacotes;
* Toolchain;
* Biblioteca Padrão;
* projetos desenvolvidos pela comunidade.

Cada componente possui responsabilidades próprias, preservando a separação entre especificação e implementação.

---

## 1.5 Relação com os Demais Documentos

Este documento complementa a infraestrutura definida pelos documentos anteriores.

Em particular, relaciona-se com:

* Biblioteca Padrão (Documento 08);
* Runtime Mínimo (Documento 09);
* Toolchain (Documento 11).

O gerenciamento de pacotes utiliza a organização de projetos definida pela Toolchain e os mecanismos de compilação especificados pelo Compilador.

---

# 2. Visão Geral do Ecossistema

O ecossistema da Capi organiza a forma como bibliotecas, aplicações e ferramentas são compartilhadas entre desenvolvedores.

Seu objetivo é permitir que componentes independentes possam ser reutilizados de maneira segura, previsível e consistente.

A especificação não impõe uma infraestrutura única, mas estabelece contratos comuns que permitem interoperabilidade entre diferentes implementações.

---

## 2.1 Bibliotecas

Bibliotecas constituem unidades reutilizáveis de software desenvolvidas para fornecer funcionalidades específicas.

Uma biblioteca poderá ser utilizada por:

* aplicações;
* outras bibliotecas;
* ferramentas da Toolchain;
* componentes da própria plataforma.

Sua distribuição ocorre por meio do sistema oficial de pacotes.

---

## 2.2 Aplicações

Aplicações representam os artefatos finais executados pelos usuários.

Embora normalmente não sejam distribuídas como dependências, utilizam o mesmo mecanismo de gerenciamento de pacotes para obter bibliotecas necessárias durante sua compilação.

---

## 2.3 Pacotes

Pacotes constituem a unidade oficial de distribuição do ecossistema.

Cada pacote representa um componente versionado que pode ser obtido, instalado e utilizado pela Toolchain.

Pacotes poderão conter:

* bibliotecas;
* aplicações;
* ferramentas;
* recursos auxiliares;
* documentação.

---

## 2.4 Registros de Pacotes

Os pacotes são disponibilizados através de registros (*Package Registries*).

Um registro fornece serviços para:

* publicação;
* descoberta;
* distribuição;
* atualização;
* consulta de versões.

A especificação não exige a existência de um único registro oficial.

Implementações poderão utilizar registros públicos, privados ou internos, desde que respeitem os contratos definidos por este documento.

---

## 2.5 Comunidade

O ecossistema da Capi incentiva o desenvolvimento colaborativo.

Bibliotecas produzidas pela comunidade constituem parte fundamental da plataforma e poderão ser distribuídas pelos mesmos mecanismos utilizados pelos componentes oficiais.

Essa abordagem favorece um ecossistema aberto, interoperável e independente de fornecedores específicos.

---

# 3. Pacotes

Pacotes representam a unidade fundamental de distribuição de software no ecossistema da Capi.

Eles encapsulam código, metadados, documentação e demais recursos necessários para reutilização por outros projetos.

A Toolchain opera sobre pacotes como sua principal unidade de compartilhamento.

---

## 3.1 Definição

Um pacote é um componente versionado que pode ser publicado em um registro e posteriormente utilizado como dependência por outros projetos.

Cada pacote possui identidade própria e ciclo de vida independente.

---

## 3.2 Identificação

Todo pacote deverá possuir um identificador único dentro do registro onde está publicado.

Esse identificador é utilizado pela Toolchain durante:

* resolução de dependências;
* instalação;
* atualização;
* publicação.

O formato exato desse identificador pertence à implementação do registro.

---

## 3.3 Estrutura

Um pacote poderá conter:

* código-fonte;
* metadados;
* documentação;
* exemplos;
* testes;
* recursos auxiliares.

A organização interna deverá permanecer compatível com a estrutura oficial de projetos definida pela Toolchain.

---

## 3.4 Metadados

Todo pacote deverá fornecer metadados suficientes para permitir sua utilização pelo ecossistema.

Entre eles poderão estar:

* nome;
* versão;
* descrição;
* autores;
* licença;
* dependências;
* compatibilidade.

A estrutura detalhada desses metadados pertence ao Gerenciador de Pacotes.

---

## 3.5 Publicação

A publicação de um pacote torna-o disponível para utilização por outros projetos.

A especificação define apenas os contratos necessários para esse processo.

Questões relacionadas à autenticação, autorização, aprovação, moderação e políticas de publicação pertencem exclusivamente aos registros utilizados.

---

# 4. Dependências

O gerenciamento de dependências permite que projetos reutilizem bibliotecas desenvolvidas por terceiros de maneira organizada, consistente e reprodutível.

A Toolchain utiliza as informações declaradas pelo projeto para localizar, obter e integrar automaticamente as dependências necessárias durante o processo de compilação.

A resolução das dependências deverá produzir resultados determinísticos, preservando a estabilidade das construções realizadas em diferentes ambientes.

---

## 4.1 Declaração

As dependências utilizadas por um projeto deverão ser declaradas em seu arquivo oficial de configuração.

Cada dependência poderá informar, entre outros elementos:

* identificador do pacote;
* versão desejada;
* origem do pacote;
* dependências opcionais;
* configurações específicas da plataforma.

A sintaxe utilizada para essa declaração pertence à Toolchain.

---

## 4.2 Resolução

O Gerenciador de Pacotes é responsável por resolver todas as dependências declaradas por um projeto.

Esse processo poderá incluir:

* localização dos pacotes;
* seleção de versões compatíveis;
* resolução de dependências transitivas;
* identificação de conflitos;
* preparação do ambiente de compilação.

A estratégia utilizada para realizar essa resolução pertence exclusivamente à implementação.

---

## 4.3 Versionamento

A seleção das versões deverá respeitar as restrições declaradas pelo projeto e os princípios de compatibilidade definidos por esta especificação.

Sempre que múltiplas versões forem compatíveis, a Toolchain deverá produzir um resultado determinístico.

As regras detalhadas de versionamento são apresentadas no Capítulo 7.

---

## 4.4 Dependências Transitivas

Dependências poderão utilizar outras dependências para implementar suas funcionalidades.

O Gerenciador de Pacotes deverá resolver automaticamente essas dependências transitivas, preservando consistência e compatibilidade entre todos os componentes envolvidos.

Conflitos identificados durante esse processo deverão ser comunicados ao desenvolvedor por meio de diagnósticos claros.

---

## 4.5 Reprodutibilidade

Uma mesma configuração de projeto deverá produzir, sempre que possível, o mesmo conjunto de dependências.

Esse princípio garante construções reproduzíveis (*reproducible builds*) e reduz diferenças entre ambientes de desenvolvimento, integração contínua e produção.

Os mecanismos utilizados para assegurar essa reprodutibilidade pertencem à implementação do Gerenciador de Pacotes.

---

# 5. Registros de Pacotes

Os registros de pacotes (*Package Registries*) constituem a infraestrutura responsável pelo armazenamento e distribuição dos pacotes utilizados pela linguagem.

A especificação define apenas os contratos públicos desses registros, permitindo que diferentes implementações interoperem de maneira consistente.

Não existe exigência de um único registro oficial.

A organização mantenedora da linguagem poderá disponibilizar um registro público oficial como implementação de referência.

Esse registro utilizará exatamente os mesmos contratos definidos por esta especificação, não possuindo privilégios em relação a outros registros compatíveis.

---

## 5.1 Objetivo

Os registros têm como finalidade:

* armazenar pacotes;
* disponibilizar versões publicadas;
* permitir descoberta de bibliotecas;
* fornecer metadados;
* disponibilizar atualizações.

Esses serviços constituem a interface pública utilizada pelo Gerenciador de Pacotes.

---

## 5.2 Organização

Cada registro organiza seus pacotes conforme suas próprias políticas administrativas.

A especificação não determina:

* estrutura interna;
* tecnologia utilizada;
* banco de dados;
* arquitetura dos servidores;
* modelo de armazenamento.

Esses aspectos pertencem exclusivamente à implementação do registro.

---

## 5.3 Publicação

Registros deverão oferecer mecanismos para publicação de novos pacotes e novas versões.

O processo poderá envolver:

* validação dos metadados;
* verificação de integridade;
* autenticação dos publicadores;
* registro da nova versão.

Os procedimentos específicos pertencem às políticas do registro utilizado.

---

## 5.4 Imutabilidade

Uma versão publicada de um pacote deverá ser considerada imutável.

Após sua publicação, seu conteúdo não poderá ser alterado.

Correções deverão ser disponibilizadas por meio da publicação de uma nova versão.

Esse princípio garante reprodutibilidade e previsibilidade para todos os projetos que utilizam o pacote.

---

## 5.5 Atualizações

Os registros deverão disponibilizar informações sobre novas versões dos pacotes publicados.

O Gerenciador de Pacotes poderá utilizar essas informações para:

* verificar atualizações;
* resolver novas dependências;
* identificar versões obsoletas;
* auxiliar processos de manutenção.

A frequência e a forma de disponibilização dessas informações pertencem ao registro.

---

## 5.6 Espelhos (*Mirrors*)

Registros poderão possuir espelhos (*mirrors*) destinados a aumentar disponibilidade, desempenho ou proximidade geográfica.

Todos os espelhos deverão disponibilizar informações compatíveis com o registro de origem.

A forma como sincronização, replicação e consistência são mantidas pertence exclusivamente à implementação da infraestrutura do registro.

---

# 6. Gerenciador de Pacotes

O Gerenciador de Pacotes é o componente responsável por integrar a Toolchain aos registros de pacotes utilizados pelo projeto.

Sua função é localizar, obter, instalar, atualizar e disponibilizar dependências para o processo de compilação, preservando os contratos definidos por esta especificação.

Embora faça parte da infraestrutura oficial da plataforma, sua implementação permanece independente da linguagem e da Toolchain.

---

## 6.1 Objetivo

O Gerenciador de Pacotes tem como objetivos:

* resolver dependências;
* obter pacotes a partir dos registros configurados;
* manter um cache local;
* disponibilizar bibliotecas para compilação;
* garantir consistência entre diferentes ambientes de desenvolvimento.

Seu comportamento deverá permanecer compatível com a organização oficial do ecossistema.

---

## 6.2 Integração com a Toolchain

A Toolchain utiliza automaticamente o Gerenciador de Pacotes sempre que uma operação depender da resolução de dependências.

Durante comandos como compilação, execução de testes, geração de documentação ou atualização do projeto, a Toolchain verifica a disponibilidade das dependências declaradas e, quando necessário, obtém os pacotes correspondentes antes da execução da operação solicitada.

Essa integração permanece transparente ao desenvolvedor, que interage exclusivamente com a interface pública da Toolchain.

---

## 6.3 Download

Quando uma dependência ainda não estiver disponível localmente, o Gerenciador de Pacotes deverá obtê-la a partir dos registros configurados.

Esse processo poderá incluir:

* localização do pacote;
* verificação da versão;
* transferência dos arquivos;
* validação da integridade;
* armazenamento no cache local.

A forma como essas operações são realizadas pertence exclusivamente à implementação.

---

## 6.4 Cache

Implementações poderão manter um cache local contendo os pacotes utilizados pelos projetos.

O cache tem como finalidade:

* reduzir downloads repetidos;
* acelerar compilações;
* permitir reutilização de dependências;
* reduzir dependência da disponibilidade dos registros.

A organização interna desse cache pertence exclusivamente à implementação.

---

## 6.5 Atualização

O Gerenciador de Pacotes poderá verificar periodicamente a existência de novas versões das dependências utilizadas por um projeto.

A atualização deverá respeitar:

* as restrições de versão declaradas;
* as políticas de compatibilidade;
* as configurações definidas pelo desenvolvedor.

A instalação automática de novas versões somente poderá ocorrer quando permanecer compatível com as regras estabelecidas pelo projeto.

---

# 7. Versionamento

O versionamento permite controlar a evolução dos pacotes distribuídos pelo ecossistema da Capi.

Seu objetivo é oferecer previsibilidade durante a atualização de dependências e preservar compatibilidade entre diferentes versões de bibliotecas.

A especificação recomenda a adoção de práticas amplamente reconhecidas pela comunidade de desenvolvimento de software.

---

## 7.1 Semantic Versioning

A Capi adota o **Semantic Versioning (SemVer)** como modelo recomendado para identificação de versões.

Cada versão é composta por três números:

* versão principal (*major*);
* versão secundária (*minor*);
* versão de correção (*patch*).

Alterações incompatíveis deverão resultar em uma nova versão principal.

O modelo também admite identificadores de pré-lançamento e metadados de build conforme definido pelo Semantic Versioning 2.0.

Exemplos:

- 2.0.0-alpha.1
- 2.0.0-beta.3
- 2.0.0-rc.1
- 2.0.0+build.42

---

## 7.2 Compatibilidade

Versões compatíveis poderão ser atualizadas automaticamente quando respeitarem as restrições declaradas pelo projeto.

O conceito de compatibilidade deverá seguir as regras estabelecidas pelo Semantic Versioning.

A interpretação dessas regras pertence ao Gerenciador de Pacotes.

---

## 7.3 Alterações Incompatíveis

Mudanças que modifiquem contratos públicos de um pacote deverão ser consideradas incompatíveis.

Essas alterações normalmente incluem:

* remoção de APIs públicas;
* modificação de assinaturas;
* alteração da semântica previamente documentada;
* mudanças incompatíveis no comportamento esperado.

A publicação dessas alterações deverá resultar em uma nova versão principal.

---

## 7.4 Seleção de Versões

Quando existirem múltiplas versões compatíveis de uma dependência, o Gerenciador de Pacotes deverá selecionar uma única versão de maneira determinística.

Os critérios utilizados para essa seleção pertencem exclusivamente à implementação, desde que respeitem as restrições declaradas pelo projeto.

---

## 7.5 Evolução

O modelo de versionamento poderá evoluir para acomodar novas necessidades do ecossistema.

Essa evolução deverá preservar, sempre que possível:

* compatibilidade entre projetos;
* previsibilidade das atualizações;
* estabilidade do processo de resolução de dependências.

Mudanças incompatíveis deverão ser claramente documentadas e identificadas.

---

# 8. Segurança

A segurança do ecossistema da Capi tem como objetivo preservar a integridade dos pacotes distribuídos e aumentar a confiança entre desenvolvedores, registros e ferramentas da plataforma.

Este documento estabelece apenas os princípios gerais relacionados à segurança do gerenciamento de pacotes.

Os mecanismos concretos utilizados para implementá-los pertencem ao Gerenciador de Pacotes e aos registros utilizados.

---

## 8.1 Integridade

Todo pacote disponibilizado por um registro deverá possuir mecanismos que permitam verificar sua integridade.

Essa verificação tem como finalidade assegurar que o conteúdo obtido corresponde exatamente ao conteúdo originalmente publicado.

A forma utilizada para essa validação pertence exclusivamente à implementação.

---

## 8.2 Assinaturas

Registros poderão oferecer mecanismos de assinatura digital para autenticação da origem dos pacotes.

Essas assinaturas poderão ser utilizadas para:

* identificar o publicador;
* verificar autenticidade;
* detectar alterações indevidas;
* fortalecer a cadeia de confiança do ecossistema.

A especificação não determina um algoritmo específico de assinatura.

---

## 8.3 Hashes

Pacotes poderão possuir valores de verificação (*hashes*) destinados à validação de sua integridade.

O Gerenciador de Pacotes poderá utilizar esses valores durante:

* download;
* instalação;
* atualização;
* utilização do cache local.

Os algoritmos utilizados pertencem exclusivamente à implementação.

---

## 8.4 Cadeia de Confiança

A confiança estabelecida entre a Toolchain, o Gerenciador de Pacotes e os registros deverá basear-se em mecanismos verificáveis.

A especificação não define um modelo único de confiança.

Implementações poderão utilizar diferentes estratégias, incluindo:

* assinaturas digitais;
* certificados;
* listas de confiança;
* outros mecanismos equivalentes.

---

## 8.5 Auditoria

Ferramentas do ecossistema poderão disponibilizar mecanismos para auditoria das dependências utilizadas por um projeto.

Esses mecanismos poderão incluir:

* identificação de versões obsoletas;
* verificação de integridade;
* análise de compatibilidade;
* inspeção dos metadados dos pacotes.

A especificação não determina como essas auditorias deverão ser implementadas.

---

## 8.6 Vulnerabilidades

Ferramentas do ecossistema poderão disponibilizar mecanismos para identificação de vulnerabilidades conhecidas nas dependências utilizadas por um projeto.

Esses mecanismos poderão consultar bases públicas ou privadas de vulnerabilidades e emitir diagnósticos destinados a auxiliar o desenvolvedor durante a manutenção do software.

A especificação não define como essas informações são obtidas nem a forma de sua apresentação.

---

# 9. Workspaces

Um *workspace* permite organizar múltiplos projetos relacionados em um único ambiente de desenvolvimento.

Essa organização facilita o compartilhamento de código, a administração de dependências e a construção conjunta de aplicações, bibliotecas e ferramentas.

A Toolchain utiliza o conceito de *workspace* para simplificar projetos compostos por diversos componentes.

---

## 9.1 Organização

A configuração de um workspace é realizada através do arquivo `capi.toml` localizado em seu diretório raiz.

Esse arquivo poderá definir os projetos pertencentes ao workspace e compartilhar configurações comuns entre eles.

A estrutura dessas configurações pertence à Toolchain.

Um *workspace* poderá agrupar diferentes tipos de projetos, incluindo:

* aplicações;
* bibliotecas;
* ferramentas;
* exemplos;
* projetos auxiliares.

Cada componente mantém sua identidade própria, preservando independência de compilação e versionamento quando necessário.

---

## 9.2 Bibliotecas Compartilhadas

Projetos pertencentes ao mesmo *workspace* poderão compartilhar bibliotecas desenvolvidas localmente.

Esse compartilhamento reduz duplicação de código e facilita o desenvolvimento conjunto de componentes relacionados.

A forma como essas bibliotecas são localizadas pertence ao Gerenciador de Pacotes.

---

## 9.3 Dependências Internas

Dependências entre projetos do mesmo *workspace* poderão ser resolvidas localmente, sem necessidade de publicação em registros de pacotes.

Essa abordagem simplifica o desenvolvimento colaborativo e reduz o tempo necessário para validação das alterações.

---

## 9.4 Builds

A Toolchain poderá compilar todo o *workspace* ou apenas componentes específicos.

Também poderá determinar automaticamente a ordem de compilação com base nas dependências existentes entre os projetos.

Os algoritmos utilizados para essa determinação pertencem exclusivamente à implementação.

---

## 9.5 Publicação

Projetos pertencentes a um mesmo *workspace* poderão ser publicados individualmente ou em conjunto.

A política de publicação depende das necessidades de cada projeto e das capacidades oferecidas pelo registro utilizado.

A especificação não impõe restrições sobre a forma como essa publicação deverá ocorrer.

---

# 10. Ecossistema Aberto

O ecossistema da Capi foi concebido para incentivar colaboração, interoperabilidade e evolução contínua.

Embora a linguagem defina uma Biblioteca Padrão e uma Toolchain oficiais, seu crescimento depende da participação da comunidade por meio da criação de bibliotecas, ferramentas e novas implementações compatíveis com a especificação.

A abertura do ecossistema constitui um princípio fundamental da plataforma.

---

## 10.1 Bibliotecas da Comunidade

Bibliotecas desenvolvidas pela comunidade representam parte essencial do ecossistema da Capi.

Essas bibliotecas poderão ser distribuídas pelos mesmos mecanismos utilizados pelos componentes oficiais, respeitando os contratos definidos para pacotes, versionamento e gerenciamento de dependências.

A especificação não diferencia bibliotecas oficiais e comunitárias quanto ao seu processo de utilização.

A especificação não impõe qualquer modelo de licenciamento para os pacotes distribuídos pelo ecossistema.

Entretanto, recomenda-se que toda biblioteca publique sua licença como parte de seus metadados, permitindo que ferramentas automatizadas realizem verificações de compatibilidade quando necessário.

---

## 10.2 Implementações Alternativas

A especificação permite a existência de diferentes implementações para os componentes do ecossistema.

Entre elas poderão existir:

* Compiladores;
* Runtimes;
* Toolchains;
* Gerenciadores de Pacotes;
* Registros de Pacotes.

Todas deverão preservar os contratos públicos definidos pela especificação da linguagem.

---

## 10.3 Ferramentas

Ferramentas desenvolvidas por terceiros poderão integrar-se ao ecossistema da Capi.

Entre elas:

* analisadores estáticos;
* geradores de código;
* depuradores;
* profiladores;
* ferramentas de automação;
* ambientes de desenvolvimento.

Essa integração deverá ocorrer utilizando as interfaces públicas disponibilizadas pela plataforma.

---

## 10.4 Integrações

A Capi incentiva integração com tecnologias externas.

Projetos poderão utilizar:

* sistemas de build;
* plataformas de integração contínua;
* serviços de distribuição;
* registros privados;
* ambientes corporativos.

Essas integrações não alteram os contratos definidos pela especificação da linguagem.

---

## 10.5 Evolução

O ecossistema poderá evoluir continuamente.

Novos componentes, serviços e ferramentas poderão surgir sem necessidade de modificar a especificação da linguagem.

Essa evolução deverá preservar:

* compatibilidade entre implementações;
* interoperabilidade dos componentes;
* estabilidade da Toolchain;
* previsibilidade da experiência do desenvolvedor.

---

# 11. Não Objetivos

Este documento define exclusivamente os contratos relacionados ao ecossistema e ao gerenciamento de pacotes da linguagem Capi.

Diversos aspectos permanecem deliberadamente fora de seu escopo para preservar a separação entre especificação e implementação.

---

## 11.1 Infraestrutura dos Registros

A especificação não define:

* arquitetura dos servidores;
* bancos de dados;
* mecanismos de replicação;
* políticas de disponibilidade;
* infraestrutura de hospedagem.

Esses aspectos pertencem exclusivamente aos operadores dos registros.

---

## 11.2 Modelo de Negócio

A especificação não impõe qualquer modelo comercial para os registros de pacotes.

Implementações poderão utilizar:

* registros públicos;
* registros privados;
* serviços gratuitos;
* serviços comerciais;
* infraestruturas próprias.

Todos permanecem igualmente compatíveis com esta especificação.

---

## 11.3 Curadoria

A especificação não estabelece políticas de curadoria para os pacotes publicados.

Questões como:

* aprovação de publicações;
* moderação de conteúdo;
* classificação de bibliotecas;
* destaque de projetos;
* certificação de qualidade;

pertencem exclusivamente aos registros utilizados.

---

## 11.4 Infraestrutura da Comunidade

A linguagem não define como a comunidade deverá organizar:

* fóruns;
* documentação colaborativa;
* canais de comunicação;
* organizações mantenedoras;
* grupos de trabalho.

Essas iniciativas poderão evoluir independentemente da especificação da linguagem.

---

## 11.5 Implementações

A especificação não determina:

* linguagem utilizada para implementar o Gerenciador de Pacotes;
* arquitetura interna dos registros;
* protocolos internos de comunicação;
* mecanismos de cache;
* algoritmos de resolução de dependências.

Implementações diferentes poderão utilizar soluções distintas, desde que preservem os contratos públicos definidos neste documento.

---

# 12. Princípio da Separação entre Garantias e Implementação

Assim como os demais documentos da especificação da Capi, este documento distingue claramente entre os contratos públicos do ecossistema e as decisões particulares de suas implementações.

A especificação define **o que** deve ser garantido.

Cada implementação define **como** esses contratos serão concretizados.

---

## 12.1 Garantias

Constituem garantias deste documento:

* existência do conceito de pacote;
* gerenciamento padronizado de dependências;
* interoperabilidade entre Toolchain e registros;
* organização consistente do ecossistema;
* compatibilidade entre implementações.

Essas garantias fazem parte da especificação oficial da linguagem.

---

## 12.2 Implementação

Pertencem exclusivamente à implementação:

* arquitetura dos registros;
* protocolos de comunicação;
* mecanismos de autenticação;
* infraestrutura de distribuição;
* algoritmos de resolução;
* estratégias de armazenamento.

Implementações diferentes poderão utilizar soluções completamente distintas para atender aos mesmos contratos.

---

## 12.3 Evolução

O ecossistema poderá evoluir continuamente para incorporar novas tecnologias, novos registros e novas formas de distribuição de software.

Essa evolução deverá preservar, sempre que possível:

* compatibilidade entre implementações;
* estabilidade da Toolchain;
* previsibilidade para os desenvolvedores.

---

## 12.4 Compatibilidade

Mudanças na organização do ecossistema não deverão exigir alterações desnecessárias nos projetos desenvolvidos em Capi.

Sempre que mudanças incompatíveis forem inevitáveis, deverão ser claramente documentadas e acompanhadas por mecanismos que facilitem a migração entre versões.

---

## 12.5 Estabilidade

Os contratos públicos definidos por este documento constituem a base para interoperabilidade entre projetos, Toolchains, Gerenciadores de Pacotes e Registros.

Sua evolução deverá ocorrer de forma controlada, preservando estabilidade, consistência e confiança em todo o ecossistema da linguagem.

---

# 13. Considerações Finais

O ecossistema da Capi completa a infraestrutura necessária para que a linguagem possa ser utilizada de forma consistente em projetos de qualquer porte.

Ao estabelecer contratos claros para organização de pacotes, gerenciamento de dependências, registros, versionamento e integração com a Toolchain, a especificação cria uma base estável para o desenvolvimento colaborativo e para a evolução sustentável da plataforma.

A separação entre os contratos públicos definidos por este documento e as decisões particulares de implementação permite que diferentes Toolchains, Gerenciadores de Pacotes e Registros coexistam de maneira interoperável, preservando a liberdade tecnológica sem comprometer a experiência do desenvolvedor.

Essa abordagem também favorece a existência de ecossistemas públicos, privados ou corporativos, permitindo que organizações adotem a linguagem sem depender de uma infraestrutura única ou de um fornecedor específico.

Com a conclusão deste documento, encontra-se especificada toda a infraestrutura oficial da plataforma Capi, abrangendo:

* arquitetura da linguagem;
* sistema de tipos;
* modelo de memória;
* sintaxe;
* semântica operacional;
* arquitetura do compilador;
* representação intermediária (IR);
* Biblioteca Padrão;
* Runtime Mínimo;
* ABI e interoperabilidade;
* Toolchain;
* ecossistema e gerenciamento de pacotes.

Esses documentos constituem a especificação oficial da linguagem Capi e definem os contratos públicos que deverão ser preservados por todas as implementações compatíveis.

A evolução futura da linguagem deverá ocorrer de forma incremental, mantendo o compromisso com a estabilidade, a interoperabilidade e o princípio da separação entre garantias e implementação que orienta toda a arquitetura da Capi.
