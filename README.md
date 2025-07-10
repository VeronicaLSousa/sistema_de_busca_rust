## Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore
Este projeto implementa um sistema de busca otimizado para um catálogo de produtos, focado em oferecer consultas rápidas e eficientes por nome, marca, categoria e preço. O objetivo é facilitar a localização de produtos em grandes catálogos, utilizando estruturas de dados eficientes para garantir desempenho e escalabilidade.

## Tecnologias Utilizadas
Linguagem Rust
- Crates:
ordered-float (para manipulação de preços com precisão e ordenação)
- Ferramentas de teste:
Framework de testes nativo do Rust (cargo test)
Sistema de compilação: Cargo (gerenciador de pacotes e build do Rust)

## Clonar Repositório
git clone https://github.com/VeronicaLSousa/sistema_de_busca_Rust.git
cd megastore-search

## Instalando as dependênvias
cargo build

## Execução do sistema de busca
cargo run

## Criação dos arquivos
- Graph.rs representa uma estrutura de dados grafo (por exemplo, para recomendar produtos relacionados)
- Index.rs Implementa o ProductIndex com HashMaps para acelerar buscas por nome, marca e categoria.
- Lib.rs Arquivo que compila a lógica principal da sua biblioteca. Aqui você declara os módulos e publica o que quer que seja acessível para fora.
- Main.rs é o ponto de entrada do programa. Usa os módulos para criar produtos, fazer buscas e mostrar resultados.
- Product.rs define a estrutura principal que representa os produtos do seu catálogo no sistema
- Search.rs tem suas funções de buscar o catálogo (nome, categoria, marca)

## Testes
O sistema conta com uma suíte de testes unitários para verificar o correto funcionamento das funções de busca e dos componentes principais, como o índice de produtos (ProductIndex).

Execute: 
cargo test

O Cargo irá compilar o código (em modo de teste) e rodar todos os testes automaticamente. Você verá uma saída indicando quais testes passaram ou falharam.

Tipos de testes incluídos:
Testes de busca por nome, marca, categoria e preço
Testes do índice (ProductIndex) para garantir que produtos são inseridos e encontrados corretamente
Testes de cobertura de casos vazios ou inexistentes

## Licença
Este projeto está licenciado sob a MIT License.
Consulte o arquivo LICENSE para mais detalhes.
A MIT License permite uso, modificação, distribuição e sublicenciamento do software, desde que a licença original e o aviso de direitos autorais sejam mantidos.
