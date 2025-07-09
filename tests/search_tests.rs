#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::Product;
    use crate::search::{buscar_por_categoria, buscar_por_preco};
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
    fn test_buscar_por_categoria() {
        let produtos = produtos_mock();
        let resultado = buscar_por_categoria(&produtos, "Eletrônicos");
        assert_eq!(resultado.len(), 2);
        assert!(resultado.iter().any(|p| p.name == "Smartphone X"));
        assert!(resultado.iter().any(|p| p.name == "Fone de Ouvido"));
    }

    #[test]
    fn test_buscar_por_preco() {
        let produtos = produtos_mock();
        let resultado = buscar_por_preco(&produtos, 2000.0);
        assert_eq!(resultado.len(), 2);
        assert!(resultado.iter().all(|p| p.price.into_inner() <= 2000.0));
    }
}

