# 09 — Runtime Mínimo

## Especificação da Linguagem Capi

Versão: 1.0 (Draft)

---

# 1. Introdução

A execução de um programa escrito em Capi depende de uma infraestrutura mínima responsável por materializar os conceitos definidos ao longo da especificação da linguagem.

Essa infraestrutura é denominada **Runtime Mínimo**.

O Runtime Mínimo fornece os serviços fundamentais necessários para que programas compilados possam executar corretamente, preservando todas as garantias estabelecidas pelo Sistema de Tipos, pelo Modelo de Memória e pela Semântica Operacional.

Ele não constitui uma biblioteca de uso geral nem faz parte da sintaxe da linguagem. Seu papel é oferecer os mecanismos essenciais sobre os quais a Biblioteca Padrão e as aplicações são executadas.

---

## 1.1 Objetivo

Este documento especifica as responsabilidades, os princípios arquiteturais e os contratos mínimos do Runtime da linguagem Capi.

São objetivos deste documento:

* definir o papel do Runtime na arquitetura da linguagem;
* estabelecer os serviços mínimos necessários à execução dos programas;
* definir os limites entre Runtime, Compilador e Biblioteca Padrão;
* especificar as garantias obrigatórias de qualquer implementação;
* preservar a independência entre especificação e implementação.

A implementação física do Runtime não faz parte desta especificação.

---

## 1.2 Escopo

Este documento especifica exclusivamente a infraestrutura mínima necessária para executar programas Capi.

Não fazem parte deste documento:

* a sintaxe da linguagem (Documento 04);
* o Sistema de Tipos (Documento 02);
* o Modelo de Memória (Documento 03);
* a Semântica Operacional (Documento 05);
* a Arquitetura do Compilador (Documento 06);
* a Intermediate Representation (Documento 07);
* a Biblioteca Padrão (Documento 08);
* a ABI e a Interoperabilidade (Documento 10).

---

## 1.3 Relação com os Documentos Anteriores

O Runtime Mínimo constitui a camada responsável por materializar, durante a execução, os conceitos definidos pelos documentos anteriores.

Enquanto:

* o Sistema de Tipos define os tipos da linguagem;
* o Modelo de Memória define a organização lógica da memória;
* a Semântica Operacional define o comportamento dos programas;
* o Compilador transforma código-fonte em executáveis;
* a Biblioteca Padrão fornece abstrações reutilizáveis;

o Runtime fornece os serviços fundamentais que permitem que essas abstrações sejam executadas sobre a plataforma de destino.

---

# 2. Papel do Runtime Mínimo

O Runtime Mínimo representa a infraestrutura de execução da linguagem Capi.

Ele atua como uma camada intermediária entre os programas compilados e os serviços oferecidos pelo sistema operacional ou pela plataforma de execução.

Seu objetivo não é fornecer funcionalidades diretamente ao programador, mas disponibilizar os mecanismos necessários para que a linguagem possa cumprir todas as garantias estabelecidas por sua especificação.

---

## 2.1 Infraestrutura de Execução

O Runtime fornece os serviços fundamentais utilizados durante a execução dos programas.

Entre suas responsabilidades encontram-se:

* gerenciamento de Domains;
* gerenciamento de objetos;
* inicialização do ambiente de execução;
* coordenação da execução concorrente;
* acesso controlado aos serviços da plataforma;
* suporte à Biblioteca Padrão.

Os mecanismos concretos utilizados para implementar esses serviços pertencem exclusivamente à implementação.

---

## 2.2 Relação com a Biblioteca Padrão

A Biblioteca Padrão utiliza os serviços disponibilizados pelo Runtime sempre que necessita acessar recursos dependentes da plataforma.

Essa comunicação ocorre exclusivamente através das interfaces públicas definidas pelo Runtime.

A Biblioteca Padrão não depende de detalhes internos da implementação do Runtime.

---

## 2.3 Relação com o Compilador

O Compilador gera código capaz de utilizar os serviços definidos pelo Runtime.

Essa integração ocorre através de contratos previamente estabelecidos pela especificação.

O Compilador não depende da organização interna do Runtime, apenas das interfaces públicas disponibilizadas por ele.

---

## 2.4 Independência da Plataforma

O Runtime constitui a camada responsável por encapsular diferenças entre sistemas operacionais, arquiteturas de processador e ambientes de execução.

Essa abstração permite que:

* o Compilador permaneça independente da plataforma;
* a Biblioteca Padrão apresente comportamento uniforme;
* programas Capi sejam portáveis entre diferentes ambientes.

---

## 2.5 Runtime Mínimo

Este documento especifica apenas os serviços considerados indispensáveis para a execução da linguagem.

Implementações poderão oferecer funcionalidades adicionais, desde que:

* não alterem as garantias estabelecidas pela especificação;
* não modifiquem o comportamento observável da linguagem;
* preservem compatibilidade com os contratos públicos do Runtime Mínimo.

---

# 3. Arquitetura Geral

O Runtime Mínimo é composto por um conjunto reduzido de serviços internos que dão suporte à execução da linguagem.

Esses serviços não constituem APIs de uso geral, mas infraestrutura utilizada pelo Compilador, pela Biblioteca Padrão e pelos programas gerados.

---

## 3.1 Componentes

Conceitualmente, o Runtime Mínimo é composto pelos seguintes subsistemas:

```text
                 Aplicação
                      │
              Biblioteca Padrão
                      │
         ┌────────────┴────────────┐
         │      Runtime Mínimo     │
         ├─────────────────────────┤
         │ Gerenciamento de Domains│
         │ Gerenciamento de Objetos│
         │ Scheduler               │
         │ Sincronização           │
         │ Serviços da Plataforma  │
         │ Inicialização           │
         │ Metadados               │
         └────────────┬────────────┘
                      │
           Sistema Operacional
```

Essa organização é conceitual.

A implementação poderá organizar seus componentes de maneira diferente, desde que preserve os contratos definidos por este documento.

---

## 3.2 Interfaces Públicas

Os serviços disponibilizados pelo Runtime deverão ser acessados exclusivamente através de interfaces públicas definidas pela especificação.

Essas interfaces constituem o contrato entre:

* Compilador;
* Biblioteca Padrão;
* Runtime.

Implementações não deverão depender de detalhes internos que não façam parte desse contrato.

---

## 3.3 Inicialização

Antes da execução do programa, o Runtime deverá realizar todas as operações necessárias para preparar o ambiente de execução.

Entre essas operações poderão estar:

* inicialização dos serviços internos;
* criação do Domain inicial;
* preparação de estruturas de gerenciamento;
* inicialização da Biblioteca Padrão.

A ordem e os mecanismos utilizados pertencem exclusivamente à implementação.

---

## 3.4 Encerramento

Ao término da execução, o Runtime deverá realizar o encerramento ordenado do ambiente.

Isso poderá incluir:

* finalização dos Domains ativos;
* execução das rotinas de finalização previstas pela linguagem;
* liberação de recursos internos;
* encerramento dos serviços da plataforma.

O comportamento observável deverá permanecer consistente independentemente da estratégia utilizada pela implementação.

---

## 3.5 Evolução

A arquitetura interna do Runtime poderá evoluir continuamente.

Novos componentes, novos algoritmos e novas estratégias poderão ser incorporados sem alterar os contratos públicos estabelecidos por este documento.

Esse princípio assegura que a evolução tecnológica do Runtime permaneça independente da evolução da especificação da linguagem, preservando integralmente as garantias da Capi.

---

# 4. Gerenciamento de Domains

O Runtime Mínimo é responsável por materializar o conceito de **Domain** definido pelo Modelo de Memória.

Embora Domains constituam um conceito semântico da linguagem, sua existência durante a execução depende dos serviços fornecidos pelo Runtime.

O Runtime não altera a definição de Domain; apenas implementa os mecanismos necessários para sua existência.

---

## 4.1 Responsabilidades

Compete ao Runtime:

* criar Domains;
* manter seu ciclo de vida;
* controlar seu isolamento;
* disponibilizar os serviços necessários para sua utilização;
* destruí-los quando seu lifetime terminar.

As regras que determinam quando um Domain deve existir pertencem ao Modelo de Memória e à Semântica Operacional.

---

## 4.2 Criação

Todo Domain existente durante a execução deverá ser criado pelo Runtime. O primeiro Domain criado durante a inicialização do programa constitui o Domain inicial. Ele representa o contexto raiz da execução e permanece sujeito exatamente às mesmas regras estabelecidas pelo Modelo de Memória. Todo programa possui pelo menos um Domain ativo durante sua execução.

A criação poderá ocorrer:

* durante a inicialização do programa;
* por solicitação da Biblioteca Padrão;
* através de primitivas de concorrência;
* por mecanismos previstos pela Semântica Operacional.

A estratégia utilizada pertence exclusivamente à implementação.

---

## 4.3 Lifetime

Cada Domain possui um lifetime claramente definido.

Durante esse período:

* novos objetos poderão ser criados;
* objetos existentes permanecerão válidos;
* a identidade dos objetos será preservada.

Ao término do lifetime do Domain, todos os objetos pertencentes a ele deixarão de existir simultaneamente, conforme definido pelo Modelo de Memória.

---

## 4.4 Isolamento

O Runtime deverá preservar o isolamento entre Domains.

Esse isolamento garante que:

* objetos pertencentes a Domains distintos permaneçam independentes;
* o gerenciamento de memória seja realizado separadamente;
* as garantias da linguagem sejam preservadas.

Nenhuma implementação poderá permitir que o isolamento entre Domains seja violado.

---

## 4.5 Destruição

Quando um Domain atingir o final de seu lifetime, o Runtime deverá realizar sua destruição.

Esse processo deverá respeitar integralmente a Semântica Operacional da linguagem, incluindo a execução das operações de finalização previstas antes da liberação definitiva dos recursos associados ao Domain.

Os mecanismos utilizados para realizar essa destruição pertencem exclusivamente à implementação.

---

# 5. Gerenciamento de Objetos

O Runtime fornece os mecanismos necessários para materializar fisicamente os objetos utilizados pelos programas.

A organização lógica dos objetos permanece definida pelo Modelo de Memória.

---

## 5.1 Responsabilidades

Compete ao Runtime:

* alocar memória para novos objetos;
* manter metadados necessários à execução;
* preservar a identidade dos objetos;
* liberar recursos ao término do lifetime do Domain.

As regras semânticas relacionadas aos objetos permanecem definidas pelos documentos anteriores.

---

## 5.2 Alocação

Sempre que a Semântica Operacional determinar a criação de um novo objeto, o Runtime deverá realizar sua alocação.

A implementação poderá utilizar qualquer estratégia adequada, incluindo:

* arenas;
* regiões;
* pools;
* heaps tradicionais;
* outras técnicas equivalentes.

A estratégia escolhida não poderá alterar o comportamento observável da linguagem.

---

## 5.3 Identidade

Todo objeto deverá possuir uma identidade única durante seu lifetime.

O Runtime é responsável por garantir que essa identidade permaneça estável enquanto o objeto existir.

A representação física dessa identidade pertence exclusivamente à implementação.

---

## 5.4 Metadados

O Runtime poderá manter metadados associados aos objetos.

Exemplos incluem:

* informações de tipo;
* informações de despacho dinâmico;
* informações para depuração;
* dados auxiliares necessários à execução.

A existência, organização e formato desses metadados não fazem parte da especificação da linguagem.

---

## 5.5 Liberação

Objetos individuais não são destruídos isoladamente.

A liberação de seus recursos ocorre juntamente com a destruição do Domain ao qual pertencem, conforme definido pelo Modelo de Memória.

O Runtime deverá garantir que essa operação preserve todas as garantias estabelecidas pela especificação.

---

# 6. Suporte ao Sistema de Tipos

O Runtime fornece a infraestrutura necessária para que determinados recursos do Sistema de Tipos possam ser materializados durante a execução.

Ele não define novos tipos nem altera o comportamento do Sistema de Tipos.

O Runtime deverá preservar, durante a execução, as condições necessárias para que as garantias da Capacidade de Mutação permaneçam válidas. Embora sua verificação ocorra predominantemente durante a compilação, nenhuma implementação poderá introduzir mecanismos que permitam violações das restrições definidas pelo Sistema de Tipos.

---

## 6.1 Objetivos

O Runtime poderá fornecer infraestrutura para:

* identificação dinâmica de tipos;
* despacho dinâmico;
* metadados de tipos;
* suporte à reflexão;
* integração com a Biblioteca Padrão.

A necessidade de cada um desses recursos dependerá da implementação adotada.

---

## 6.2 Informações de Tipo

Sempre que necessário, o Runtime poderá manter informações adicionais associadas aos tipos definidos pela linguagem.

Essas informações poderão ser utilizadas por:

* operações de reflexão;
* despacho dinâmico;
* verificações em tempo de execução;
* ferramentas de depuração.

A forma de armazenamento dessas informações pertence exclusivamente à implementação.

---

## 6.3 Despacho Dinâmico

Chamadas polimórficas poderão utilizar mecanismos disponibilizados pelo Runtime para selecionar a implementação correta durante a execução.

A estratégia utilizada poderá incluir:

* tabelas virtuais;
* tabelas de despacho;
* ponteiros para funções;
* outras estruturas equivalentes.

O mecanismo adotado não poderá alterar a Semântica Operacional da linguagem.

---

## 6.4 Suporte à Reflection

Quando a implementação oferecer suporte ao módulo Reflection da Biblioteca Padrão, o Runtime poderá disponibilizar os metadados necessários para sua operação.

A quantidade de informações preservadas dependerá das opções de compilação e da implementação adotada.

---

## 6.5 Independência

O Sistema de Tipos permanece integralmente definido pelo Documento 02.

O Runtime apenas fornece a infraestrutura necessária para que determinados aspectos desse sistema possam ser utilizados durante a execução.

Essa separação preserva o princípio arquitetural segundo o qual as garantias da linguagem pertencem à especificação, enquanto os mecanismos utilizados para implementá-las pertencem exclusivamente ao Runtime.

---

# 7. Execução do Programa

O Runtime Mínimo é responsável por preparar o ambiente necessário para que um programa compilado possa iniciar sua execução.

Esse processo compreende a inicialização da infraestrutura da linguagem, a preparação dos serviços internos e a transferência de controle para o ponto de entrada da aplicação.

As regras que determinam o comportamento observável da execução pertencem à Semântica Operacional. O Runtime apenas materializa esses comportamentos.

---

## 7.1 Inicialização

Antes da execução do programa, o Runtime deverá:

* inicializar seus subsistemas internos;
* preparar os serviços da plataforma;
* criar o Domain inicial;
* inicializar a Biblioteca Padrão, quando necessário; 
* preparar o ambiente de execução.

A ordem exata dessas operações pertence exclusivamente à implementação.

Nota: a inicialização da Biblioteca Padrão poderá envolver a preparação de estruturas globais, serviços internos e recursos utilizados pelos módulos da Biblioteca Padrão. Sua extensão depende exclusivamente da implementação.

---

## 7.2 Ponto de Entrada

Após concluir sua inicialização, o Runtime deverá transferir o controle para o ponto de entrada definido pelo programa.

A forma como esse ponto de entrada é localizado pertence ao Compilador e à ABI definida para a plataforma.

O Runtime apenas garante que o ambiente esteja corretamente preparado antes da transferência de controle.

---

## 7.3 Ambiente de Execução

Durante a execução do programa, o Runtime deverá manter disponíveis todos os serviços necessários ao funcionamento da linguagem.

Entre eles:

* gerenciamento de Domains;
* gerenciamento de objetos;
* serviços de concorrência;
* sincronização;
* integração com a Biblioteca Padrão;
* acesso aos serviços da plataforma.

---

## 7.4 Encerramento

Ao término da execução do programa, o Runtime deverá realizar o encerramento ordenado do ambiente.

Esse processo deverá incluir, quando aplicável:

* finalização dos Domains ativos;
* execução das operações de finalização previstas pela linguagem;
* liberação dos recursos internos;
* encerramento dos serviços da plataforma.

---

## 7.5 Independência da Implementação

A estratégia utilizada para inicialização e encerramento do programa pertence exclusivamente à implementação.

Qualquer implementação compatível deverá preservar o comportamento observável definido pela Semântica Operacional, independentemente dos mecanismos internos utilizados.

---

# 8. Concorrência

O Runtime fornece a infraestrutura necessária para execução concorrente de programas escritos em Capi.

Entretanto, o modelo de concorrência da linguagem permanece definido pela Semântica Operacional e pelo Modelo de Memória.

O Runtime apenas implementa os mecanismos necessários para materializar essas definições.

---

## 8.1 Responsabilidades

Compete ao Runtime:

* criar novos contextos de execução;
* agendar tarefas concorrentes;
* manter o isolamento entre Domains;
* coordenar a comunicação entre tarefas;
* fornecer infraestrutura de sincronização.

---

## 8.2 Fluxos de Execução

O Runtime poderá implementar os fluxos concorrentes utilizando diferentes mecanismos da plataforma.

Entre eles:

* threads do sistema operacional;
* fibras (*fibers*);
* corrotinas (*coroutines*);
* escalonadores cooperativos;
* escalonadores preemptivos.

A escolha da estratégia pertence exclusivamente à implementação.

---

## 8.3 Scheduler

Quando necessário, o Runtime poderá fornecer um escalonador responsável por distribuir tarefas concorrentes entre os recursos disponíveis da plataforma.

A política de escalonamento utilizada não faz parte da especificação da linguagem.

O único requisito é preservar integralmente o comportamento definido pela Semântica Operacional.

---

## 8.4 Integração com Domains

Toda execução concorrente deverá respeitar a organização baseada em Domains.

O Runtime deverá garantir que:

* o isolamento entre Domains seja preservado;
* as regras da Capacidade de Mutação permaneçam válidas;
* nenhuma implementação introduza compartilhamento incompatível com o Modelo de Memória.

---

## 8.5 Evolução

Novos mecanismos de concorrência poderão ser incorporados ao Runtime em versões futuras.

Essa evolução não poderá modificar o modelo de concorrência definido pela linguagem.

---

# 9. Sincronização

O Runtime fornece os mecanismos necessários para coordenar a execução concorrente quando exigido pela Semântica Operacional ou pela Biblioteca Padrão.

Esses mecanismos pertencem exclusivamente à infraestrutura de execução e não alteram o Modelo de Memória nem o Sistema de Tipos.

---

## 9.1 Objetivos

Os mecanismos de sincronização têm como finalidade:

* coordenar fluxos concorrentes;
* proteger recursos internos;
* preservar consistência da execução;
* implementar primitivas disponibilizadas pela Biblioteca Padrão.

---

## 9.2 Primitivas

O Runtime poderá disponibilizar internamente mecanismos como:

* mutexes;
* semáforos;
* barreiras;
* variáveis de condição;
* operações atômicas;
* outras primitivas equivalentes.

A existência dessas primitivas não implica sua exposição direta ao programador.

---

## 9.3 Operações Atômicas

Quando a Biblioteca Padrão disponibilizar operações atômicas, sua implementação poderá utilizar mecanismos específicos fornecidos pelo Runtime.

As garantias semânticas dessas operações permanecem definidas pela linguagem.

O Runtime apenas fornece os mecanismos necessários para implementá-las.

---

## 9.4 Coordenação

Os mecanismos de sincronização utilizados pelo Runtime deverão preservar:

* a integridade dos Domains;
* a identidade dos objetos;
* o lifetime dos objetos;
* as regras da Capacidade de Mutação;
* a consistência da execução concorrente.

Nenhuma estratégia de sincronização poderá enfraquecer as garantias da linguagem.

---

## 9.5 Independência da Implementação

A especificação não determina algoritmos específicos de sincronização.

Cada implementação poderá utilizar os mecanismos mais adequados à plataforma de destino, desde que preserve integralmente os contratos públicos e as garantias estabelecidas pela especificação da Capi.

---

# 10. Integração com a Biblioteca Padrão

O Runtime Mínimo e a Biblioteca Padrão desempenham papéis distintos e complementares na arquitetura da Capi.

Enquanto a Biblioteca Padrão fornece abstrações reutilizáveis para o desenvolvimento de aplicações, o Runtime disponibiliza a infraestrutura necessária para que essas abstrações possam operar sobre a plataforma de execução.

A comunicação entre ambos ocorre exclusivamente através de interfaces públicas definidas pela especificação.

---

## 10.1 Responsabilidades

Compete à Biblioteca Padrão:

* fornecer APIs públicas;
* encapsular funcionalidades reutilizáveis;
* implementar abstrações da linguagem;
* oferecer comportamento uniforme entre plataformas.

Compete ao Runtime:

* executar operações dependentes da plataforma;
* fornecer infraestrutura de execução;
* materializar os conceitos definidos pela linguagem;
* disponibilizar serviços internos utilizados pela Biblioteca Padrão.

---

## 10.2 Serviços Disponibilizados

O Runtime poderá disponibilizar serviços utilizados por diferentes módulos da Biblioteca Padrão, incluindo:

* entrada e saída de dados;
* sistema de arquivos;
* comunicação em rede;
* obtenção de tempo;
* concorrência;
* sincronização;
* acesso a serviços específicos da plataforma.

A forma como esses serviços são implementados pertence exclusivamente ao Runtime.

---

## 10.3 Abstração da Plataforma

A Biblioteca Padrão não deverá acessar diretamente recursos do sistema operacional.

Toda interação com a plataforma deverá ocorrer através das interfaces públicas fornecidas pelo Runtime.

Essa separação preserva a portabilidade da linguagem e reduz o acoplamento entre os diferentes componentes da arquitetura.

---

## 10.4 Evolução Independente

Biblioteca Padrão e Runtime poderão evoluir de forma independente.

Novas implementações do Runtime poderão oferecer melhorias internas sem exigir alterações na API pública da Biblioteca Padrão.

Da mesma forma, novos componentes da Biblioteca Padrão poderão ser desenvolvidos sem modificar a infraestrutura fundamental do Runtime.

---

## 10.5 Compatibilidade

Qualquer implementação compatível da Biblioteca Padrão deverá operar corretamente sobre qualquer Runtime que implemente os contratos definidos por esta especificação.

Essa compatibilidade constitui um dos princípios fundamentais da arquitetura da Capi.

---

# 11. Integração com o Compilador

O Compilador e o Runtime possuem responsabilidades claramente distintas.

O Compilador é responsável por traduzir programas escritos em Capi para código executável.

O Runtime fornece os serviços necessários para que esse código possa ser executado corretamente.

A comunicação entre ambos ocorre através de contratos previamente definidos pela especificação.

---

## 11.1 Responsabilidades

Compete ao Compilador:

* gerar código executável;
* produzir metadados necessários;
* emitir chamadas para os serviços do Runtime;
* preservar todas as garantias definidas pela linguagem.

Compete ao Runtime:

* executar esses serviços;
* manter a infraestrutura de execução;
* fornecer os recursos necessários durante a vida do programa.

---

## 11.2 Chamadas ao Runtime

Durante a geração de código, o Compilador poderá produzir chamadas para serviços do Runtime.

Essas chamadas poderão envolver, entre outras operações:

* criação de Domains;
* alocação de objetos;
* inicialização da execução;
* operações de sincronização;
* acesso a recursos da plataforma.

A interface utilizada deverá permanecer estável e independente da implementação.

---

## 11.3 Metadados

O Compilador poderá gerar metadados destinados ao Runtime.

Esses metadados poderão conter informações relacionadas a:

* tipos;
* símbolos;
* depuração;
* reflexão;
* inicialização do programa.

A estrutura desses metadados pertence à implementação.

---

## 11.4 Independência

O Compilador não deverá depender da organização interna do Runtime.

Da mesma forma, o Runtime não deverá depender das estruturas internas utilizadas pelo Compilador.

A única dependência permitida entre ambos é o conjunto de interfaces públicas definido pela especificação.

---

## 11.5 Evolução

Compilador e Runtime poderão evoluir independentemente.

Desde que os contratos públicos permaneçam preservados, implementações poderão modificar livremente suas arquiteturas internas.

---

# 12. Integração com a Intermediate Representation (IR)

A Intermediate Representation (IR) constitui a representação intermediária produzida pelo Compilador antes da geração do código executável.

O Runtime não executa diretamente a IR.

Entretanto, diversos elementos presentes na IR representam operações que serão posteriormente materializadas através dos serviços disponibilizados pelo Runtime.

---

## 12.1 Relação com a IR

A IR descreve, de forma independente da plataforma, operações que dependem do Runtime para sua execução.

Entre elas podem estar:

* criação de objetos;
* criação de Domains;
* chamadas dinâmicas;
* sincronização;
* inicialização do programa.

A tradução dessas operações para chamadas concretas ao Runtime ocorre durante o Backend do Compilador.

---

## 12.2 Serviços Materializados

Durante a geração de código, instruções presentes na IR poderão ser convertidas em chamadas para serviços do Runtime.

Essa conversão constitui parte da implementação do Compilador e não altera a semântica definida pela IR.

---

## 12.3 Independência

A estrutura da IR permanece completamente independente da implementação do Runtime.

Da mesma forma, o Runtime não possui conhecimento sobre a organização interna da IR.

A integração entre ambos ocorre exclusivamente através do código executável produzido pelo Compilador.

---

## 12.4 Evolução

A IR e o Runtime poderão evoluir de forma independente.

Novas instruções da IR poderão utilizar serviços já existentes do Runtime ou motivar a criação de novos serviços, desde que essa evolução preserve os contratos públicos estabelecidos pela especificação.

---

## 12.5 Princípio da Separação

A IR descreve **o que** deverá ocorrer durante a execução do programa.

O Runtime define **como** essas operações serão efetivamente realizadas na plataforma de destino.

Essa separação preserva a independência entre o processo de compilação e a infraestrutura de execução, permitindo que ambos evoluam continuamente sem comprometer as garantias da linguagem.

---

# 13. Segurança

O Runtime Mínimo desempenha papel fundamental na preservação das garantias de segurança estabelecidas pela linguagem Capi.

Embora o Sistema de Tipos, o Modelo de Memória e a Semântica Operacional definam essas garantias, cabe ao Runtime assegurar que sua implementação não introduza comportamentos que possam comprometê-las durante a execução.

---

## 13.1 Objetivos

O Runtime deverá preservar, durante toda a execução do programa:

* integridade dos Domains;
* estabilidade da identidade dos objetos;
* isolamento entre contextos de execução;
* consistência do gerenciamento de memória;
* integridade das estruturas internas do Runtime.

Essas garantias independem da estratégia de implementação adotada.

---

## 13.2 Isolamento

O Runtime deverá impedir que objetos pertencentes a Domains distintos compartilhem estado de maneira incompatível com o Modelo de Memória.

Qualquer comunicação entre Domains deverá ocorrer exclusivamente através dos mecanismos previstos pela linguagem e pela Biblioteca Padrão.

---

## 13.3 Validação de Invariantes

O Runtime poderá realizar verificações destinadas a preservar invariantes internas necessárias ao funcionamento da linguagem.

Essas verificações poderão envolver:

* consistência de estruturas internas;
* validade de metadados;
* integridade de objetos;
* estado dos serviços internos.

A existência e a forma dessas verificações pertencem exclusivamente à implementação.

---

## 13.4 Código `unsafe`

Operações realizadas dentro de blocos `unsafe` permanecem sujeitas às limitações definidas pela Semântica Operacional.

O Runtime deverá executar tais operações sem modificar seu comportamento esperado, preservando a distinção entre as garantias oferecidas pela linguagem e a responsabilidade assumida explicitamente pelo programador.

---

## 13.5 Evolução

Novos mecanismos de proteção poderão ser incorporados ao Runtime.

Esses mecanismos não poderão alterar a semântica observável da linguagem nem impor restrições incompatíveis com os contratos estabelecidos pela especificação.

---

# 14. Tratamento de Falhas

O Runtime deverá tratar adequadamente falhas relacionadas à própria infraestrutura de execução.

Essas falhas diferem dos erros representados por `Result<T,E>` ou `Optional<T>`, que fazem parte da lógica normal das aplicações.

O objetivo deste capítulo é definir como o Runtime lida com situações em que sua própria infraestrutura deixa de conseguir garantir a execução correta do programa.

---

## 14.1 Falhas da Infraestrutura

São exemplos de falhas pertencentes ao Runtime:

* impossibilidade de inicialização;
* falha na criação de Domains;
* indisponibilidade de recursos fundamentais;
* corrupção de estruturas internas;
* inconsistências detectadas durante a execução.

Essas situações não fazem parte da API pública da linguagem.

---

## 14.2 Diagnóstico

Sempre que possível, o Runtime deverá produzir informações diagnósticas que auxiliem na identificação da causa da falha.

Essas informações poderão incluir:

* descrição da falha;
* localização aproximada;
* estado dos serviços internos;
* informações adicionais de depuração.

O formato dessas informações pertence à implementação.

---

## 14.3 Encerramento Seguro

Quando não for possível prosseguir com segurança, o Runtime deverá interromper a execução do programa de forma controlada.

Esse encerramento deverá preservar, sempre que possível:

* a integridade dos dados persistidos;
* a liberação ordenada de recursos internos;
* a consistência do ambiente de execução.

---

## 14.4 Recuperação

A especificação não exige que o Runtime seja capaz de recuperar falhas de sua própria infraestrutura.

Cada implementação poderá definir estratégias de recuperação quando apropriado, desde que essas estratégias não comprometam as garantias estabelecidas pela linguagem.

---

## 14.5 Independência

Os mecanismos utilizados para detectar, registrar ou tratar falhas pertencem exclusivamente à implementação.

A especificação define apenas os princípios gerais que devem orientar esse comportamento.

---

# 15. Portabilidade

Um dos objetivos fundamentais da arquitetura da Capi é permitir que programas possam ser executados em diferentes plataformas preservando comportamento consistente.

O Runtime é o principal responsável por encapsular as diferenças existentes entre sistemas operacionais, arquiteturas e ambientes de execução.

---

## 15.1 Independência da Plataforma

A API pública do Runtime deverá permanecer independente da plataforma.

Diferenças específicas de:

* sistemas operacionais;
* arquiteturas de processador;
* modelos de memória;
* mecanismos de sincronização;
* serviços disponíveis;

deverão permanecer encapsuladas pela implementação.

---

## 15.2 Sistemas Operacionais

Implementações poderão oferecer suporte a diferentes sistemas operacionais.

A forma como os serviços do Runtime são mapeados para cada sistema operacional pertence exclusivamente à implementação.

---

## 15.3 Arquiteturas

O Runtime poderá ser implementado para diferentes arquiteturas de processador.

Questões como:

* tamanho de palavra;
* alinhamento;
* convenções de chamada;
* instruções atômicas;
* organização da memória;

deverão ser tratadas internamente, sem alterar o comportamento observado pelos programas.

---

## 15.4 Integração com a ABI

A comunicação entre o código executável produzido pelo Compilador e o Runtime deverá respeitar a ABI definida para a plataforma de destino.

Os detalhes relativos à ABI e à interoperabilidade são especificados no **Documento 10 — ABI e Interoperabilidade (FFI)**.

O Runtime deverá respeitar a representação física dos tipos definida pela ABI da plataforma de destino, incluindo requisitos de tamanho, alinhamento, convenções de chamada e representação binária dos tipos fundamentais. Esses aspectos são especificados no Documento 10 e constituem parte do contrato entre o Compilador e o Runtime.

---

## 15.5 Evolução

Novas plataformas poderão ser suportadas futuramente sem necessidade de alterar a especificação da linguagem.

Essa evolução deverá preservar integralmente os contratos públicos definidos pelo Runtime Mínimo e manter a compatibilidade com os demais componentes da arquitetura da Capi.

---

# 16. Extensibilidade

O Runtime Mínimo define apenas os serviços indispensáveis para a execução da linguagem Capi.

Implementações poderão oferecer funcionalidades adicionais, desde que preservem integralmente os contratos públicos estabelecidos por esta especificação e não alterem o comportamento observável da linguagem.

---

## 16.1 Implementações Alternativas

A especificação da Capi permite a existência de múltiplas implementações do Runtime.

Cada implementação poderá adotar arquiteturas, algoritmos e mecanismos internos distintos, desde que respeite:

* o Sistema de Tipos;
* o Modelo de Memória;
* a Semântica Operacional;
* os contratos públicos definidos neste documento.

---

## 16.2 Funcionalidades Adicionais

Implementações poderão disponibilizar serviços além do Runtime Mínimo.

Esses serviços adicionais:

* não poderão modificar as garantias da linguagem;
* não poderão alterar o comportamento da Biblioteca Padrão;
* deverão permanecer claramente identificados como extensões da implementação.

Programas que dependam dessas extensões deixam de ser portáveis entre diferentes implementações da Capi.

---

## 16.3 Perfis de Execução

O Runtime poderá ser adaptado para diferentes perfis de utilização.

Exemplos incluem:

* sistemas embarcados;
* aplicações desktop;
* servidores;
* ambientes de nuvem;
* sistemas de tempo real.

Cada perfil poderá otimizar aspectos específicos da execução, preservando os contratos definidos pela especificação.

---

## 16.4 Evolução

Novos serviços internos poderão ser incorporados ao Runtime.

Essa evolução não deverá exigir alterações na Biblioteca Padrão nem no Compilador, desde que as interfaces públicas permaneçam compatíveis.

---

## 16.5 Princípio da Modularidade

A implementação do Runtime deverá favorecer uma arquitetura modular.

Sempre que possível, serviços independentes deverão permanecer desacoplados, permitindo evolução, substituição e manutenção sem impacto sobre os demais componentes da infraestrutura.

---

# 17. Garantias do Runtime

Independentemente da implementação adotada, todo Runtime compatível com a Capi deverá preservar integralmente as garantias estabelecidas pela especificação da linguagem.

Essas garantias constituem parte do contrato entre o Runtime, o Compilador, a Biblioteca Padrão e os programas executados.

---

## 17.1 Preservação do Sistema de Tipos

O Runtime não poderá modificar, ampliar ou reduzir as regras definidas pelo Sistema de Tipos.

Toda implementação deverá preservar:

* identidade dos objetos;
* regras de herança;
* polimorfismo;
* interfaces;
* traits;
* generics;
* demais propriedades definidas pelo Documento 02.

---

## 17.2 Preservação do Modelo de Memória

O Runtime deverá materializar o Modelo de Memória exatamente como definido pela especificação.

Isso inclui:

* Domains;
* lifetime;
* isolamento;
* gerenciamento de objetos;
* destruição de Domains;
* preservação da identidade dos objetos.

Nenhuma implementação poderá alterar esses conceitos.

---

## 17.3 Preservação da Semântica Operacional

Todas as operações executadas pelo Runtime deverão respeitar integralmente a Semântica Operacional da linguagem.

Diferenças internas de implementação jamais poderão modificar o comportamento observável dos programas.

---

## 17.4 Compatibilidade

Implementações diferentes do Runtime deverão produzir comportamento semanticamente equivalente quando executarem programas compatíveis com a especificação oficial.

Diferenças de desempenho, organização interna ou utilização de recursos da plataforma não constituem diferenças semânticas.

---

## 17.5 Evolução Compatível

O Runtime poderá evoluir continuamente.

Essa evolução deverá preservar:

* compatibilidade das interfaces públicas;
* comportamento definido pela especificação;
* integração com o Compilador;
* integração com a Biblioteca Padrão;
* portabilidade das aplicações.

---

# 18. Princípio da Separação entre Garantias e Implementação

Assim como os demais documentos da especificação da Capi, este documento distingue explicitamente entre as garantias oferecidas pelo Runtime e os mecanismos utilizados para implementá-las.

A especificação define **o que** o Runtime deve fornecer.

A implementação define **como** esses serviços são concretizados em cada plataforma.

---

## 18.1 Garantias

Constituem garantias obrigatórias do Runtime:

* materialização correta do Modelo de Memória;
* preservação da Semântica Operacional;
* suporte ao Sistema de Tipos;
* integração com o Compilador;
* integração com a Biblioteca Padrão;
* comportamento consistente entre plataformas.

Essas garantias fazem parte da especificação da linguagem.

---

## 18.2 Implementação

Pertencem exclusivamente à implementação:

* algoritmos internos;
* estruturas de gerenciamento;
* organização dos serviços;
* mecanismos de sincronização;
* estratégias de alocação;
* estruturas de metadados;
* integração com o sistema operacional.

Implementações diferentes poderão adotar soluções distintas para esses aspectos.

---

## 18.3 Evolução

A implementação do Runtime poderá evoluir continuamente.

Novas estratégias internas poderão ser incorporadas sem modificar os contratos públicos definidos por esta especificação.

Esse princípio assegura que a evolução tecnológica da infraestrutura permaneça independente da evolução da linguagem.

---

## 18.4 Independência entre Componentes

A evolução do Runtime não deverá impor alterações ao Compilador, à Biblioteca Padrão ou às aplicações, desde que as interfaces públicas permaneçam compatíveis.

Da mesma forma, a evolução desses componentes não deverá exigir modificações na arquitetura interna do Runtime além daquelas necessárias para suportar novos contratos oficialmente definidos.

---

## 18.5 Estabilidade da Especificação

Os contratos públicos definidos neste documento constituem a interface oficial entre a linguagem e sua infraestrutura de execução.

Sua evolução deverá ocorrer de forma controlada, preservando compatibilidade sempre que possível e mantendo a separação entre garantias da linguagem e mecanismos de implementação.

---

# 19. Não Objetivos

Este documento especifica exclusivamente o **Runtime Mínimo** necessário para a execução da linguagem Capi.

Diversos aspectos permanecem deliberadamente fora de seu escopo para preservar a separação de responsabilidades entre a linguagem, o compilador, a biblioteca padrão e a infraestrutura de execução.

---

## 19.1 Políticas de Implementação

Este documento não especifica:

* algoritmos de alocação de memória;
* organização interna dos Domains;
* estruturas de gerenciamento de objetos;
* políticas de escalonamento;
* estratégias de sincronização;
* formatos internos de metadados.

Essas decisões pertencem exclusivamente à implementação do Runtime.

---

## 19.2 Sistema Operacional

Este documento não define como os serviços do Runtime deverão ser implementados em cada sistema operacional.

A utilização de:

* chamadas de sistema;
* bibliotecas nativas;
* APIs específicas;
* recursos exclusivos da plataforma;

pertence integralmente à implementação.

---

## 19.3 Otimizações

A especificação não determina:

* políticas de cache;
* pools de objetos;
* arenas de memória;
* estratégias de reutilização;
* otimizações de sincronização;
* técnicas de redução de consumo de memória.

Esses mecanismos poderão ser utilizados livremente pelas implementações, desde que preservem integralmente o comportamento definido pela especificação.

---

## 19.4 Ferramentas de Diagnóstico

Este documento não especifica:

* depuradores;
* profiladores;
* analisadores de desempenho;
* coletores de métricas;
* sistemas de monitoramento.

Essas ferramentas poderão utilizar informações fornecidas pelo Runtime, mas não fazem parte de sua especificação.

---

## 19.5 Funcionalidades Avançadas

Recursos como:

* coleta distribuída de métricas;
* gerenciamento remoto;
* alta disponibilidade;
* migração de execução;
* monitoramento em tempo real;
* integração com ambientes específicos;

não fazem parte do Runtime Mínimo.

Esses recursos poderão ser oferecidos por implementações específicas ou por componentes adicionais do ecossistema da Capi.

---

# 20. Considerações Finais

O Runtime Mínimo representa a camada responsável por materializar a execução da linguagem Capi.

Sua função não é ampliar as capacidades da linguagem nem fornecer APIs diretamente ao programador, mas disponibilizar a infraestrutura necessária para que todas as garantias estabelecidas pela especificação possam ser preservadas durante a execução dos programas.

Ao separar claramente o Runtime do Compilador, da Biblioteca Padrão e da própria linguagem, a arquitetura da Capi estabelece responsabilidades bem definidas para cada componente da plataforma.

Essa divisão favorece a evolução independente de cada parte do sistema, permitindo que implementações diferentes adotem estratégias distintas de gerenciamento de memória, concorrência, sincronização e integração com a plataforma, sem alterar a semântica observável dos programas.

O Runtime Mínimo também estabelece uma base comum para diferentes ambientes de execução, possibilitando a existência de implementações voltadas para servidores, aplicações desktop, sistemas embarcados, dispositivos móveis ou ambientes de tempo real, preservando sempre os contratos públicos definidos por esta especificação.

Com a conclusão deste documento, ficam definidos todos os elementos necessários para que programas escritos em Capi possam ser compilados e executados de forma consistente.

O documento seguinte, **10 — ABI e Interoperabilidade (FFI)**, especificará a interface binária da linguagem, as convenções de chamada, a representação física dos tipos e os mecanismos de interoperabilidade com bibliotecas e linguagens externas, completando a infraestrutura fundamental da plataforma Capi.
