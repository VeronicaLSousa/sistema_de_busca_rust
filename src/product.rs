use serde::{Serialize, Deserialize};
use std::fmt;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub price: OrderedFloat<f64>, // <- Alterado
}

// Exibição personalizada
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}\nNome: {}\nMarca: {}\nCategoria: {}\nPreço: R${:.2}",
            self.id,
            self.name,
            self.brand,
            self.category,
            self.price.into_inner() // <- extrai o valor f64 de OrderedFloat
        )
    }
}
