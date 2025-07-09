use crate::product::Product;
use ordered_float::OrderedFloat;

/// Busca produtos pelo nome (case-insensitive)
pub fn buscar_por_nome<'a>(produtos: &'a [Product], termo: &str) -> Vec<&'a Product> {
    let termo = termo.to_lowercase();
    produtos
        .iter()
        .filter(|p| p.name.to_lowercase().contains(&termo))
        .collect()
}

/// Busca produtos pela marca (case-insensitive)
pub fn buscar_por_marca<'a>(produtos: &'a [Product], termo: &str) -> Vec<&'a Product> {
    let termo = termo.to_lowercase();
    produtos
        .iter()
        .filter(|p| p.brand.to_lowercase().contains(&termo))
        .collect()
}

/// Busca produtos pela categoria (case-insensitive)
pub fn buscar_por_categoria<'a>(produtos: &'a [Product], termo: &str) -> Vec<&'a Product> {
    let termo = termo.to_lowercase();
    produtos
        .iter()
        .filter(|p| p.category.to_lowercase().contains(&termo))
        .collect()
}

/// Busca produtos com preço menor ou igual ao preço dado
pub fn buscar_por_preco<'a>(produtos: &'a [Product], preco_max: OrderedFloat<f64>) -> Vec<&'a Product> {
    produtos
        .iter()
        .filter(|p| p.price <= preco_max)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::Product;
    use ordered_float::OrderedFloat;

    fn produtos_mock() -> Vec<Product> {
        vec![
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
                name: "Fone de Ouvido".to_string(),
                brand: "SoundMax".to_string(),
                category: "Eletrônicos".to_string(),
                price: OrderedFloat(299.90),
            },
        ]
    }

    #[test]
    fn testa_buscar_por_categoria() {
        let produtos = produtos_mock();
        let resultado = buscar_por_categoria(&produtos, "eletrônicos");
        assert_eq!(resultado.len(), 2);
        assert!(resultado.iter().all(|p| p.category.to_lowercase().contains("eletrônicos")));
    }

    #[test]
    fn testa_buscar_por_categoria_sem_resultado() {
        let produtos = produtos_mock();
        let resultado = buscar_por_categoria(&produtos, "wearables");
        assert!(resultado.is_empty());
    }

    #[test]
    fn testa_buscar_por_nome() {
        let produtos = produtos_mock();
        let resultado = buscar_por_nome(&produtos, "fone");
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0].name, "Fone de Ouvido");
    }

    #[test]
    fn testa_buscar_por_preco() {
        let produtos = produtos_mock();
        let resultado = buscar_por_preco(&produtos, OrderedFloat(2000.00));
        assert_eq!(resultado.len(), 2);
        assert!(resultado.iter().all(|p| p.price <= OrderedFloat(2000.00)));
    }
}

