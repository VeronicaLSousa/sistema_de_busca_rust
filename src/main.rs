mod product;
mod search;
mod graph;
mod index;

use product::Product;
use search::{buscar_por_nome, buscar_por_marca, buscar_por_categoria};
use ordered_float::OrderedFloat;

fn main() {
    // Lista de produtos simulados
    let produtos = vec![
        Product {
            id: "001".to_string(),
            name: "Smartphone X".to_string(),
            brand: "TechBrand".to_string(),
            category: "Eletrônicos".to_string(),
            price: OrderedFloat(1999.99),
        },
        Product {
            id: "002".to_string(),
            name: "Notebook Pro".to_string(),
            brand: "MegaTech".to_string(),
            category: "Informática".to_string(),
            price: OrderedFloat(4500.00),
        },
        Product {
            id: "003".to_string(),
            name: "Fone Bluetooth".to_string(),
            brand: "SoundTop".to_string(),
            category: "Acessórios".to_string(),
            price: OrderedFloat(299.90),
        },
        Product {
            id: "004".to_string(),
            name: "Smartwatch Fit".to_string(),
            brand: "TechBrand".to_string(),
            category: "Wearables".to_string(),
            price: OrderedFloat(899.99),
        },
    ];

    // 🔍 Busca por nome
    let resultado_nome = buscar_por_nome(&produtos, "fone");
    println!("\n📦 Resultado da busca por nome ('fone'):");
    imprimir_resultados(&resultado_nome);

    // 🔍 Busca por marca
    let resultado_marca = buscar_por_marca(&produtos, "tech");
    println!("\n🏷️ Resultado da busca por marca ('tech'):");
    imprimir_resultados(&resultado_marca);

    // 🔍 Busca por categoria
    let resultado_categoria = buscar_por_categoria(&produtos, "informática");
    println!("\n📁 Resultado da busca por categoria ('informática'):");
    imprimir_resultados(&resultado_categoria);
}

// Função auxiliar para imprimir produtos de forma organizada
fn imprimir_resultados(produtos: &[&Product]) {
    if produtos.is_empty() {
        println!("- Nenhum produto encontrado.");
    } else {
        for produto in produtos {
            println!(
                "- ID: {}\n  Nome: {}\n  Marca: {}\n  Categoria: {}\n  Preço: R${:.2}\n",
                produto.id,
                produto.name,
                produto.brand,
                produto.category,
                produto.price.into_inner()
            );
        }
    }
}
