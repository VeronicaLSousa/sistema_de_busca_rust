use std::collections::HashMap;
use crate::product::Product;

pub struct ProductIndex {
    pub by_name: HashMap<String, Vec<Product>>,
    pub by_brand: HashMap<String, Vec<Product>>,
    pub by_category: HashMap<String, Vec<Product>>,
}

impl ProductIndex {
    pub fn new() -> Self {
        Self {
            by_name: HashMap::new(),
            by_brand: HashMap::new(),
            by_category: HashMap::new(),
        }
    }

    pub fn insert(&mut self, product: Product) {
        self.by_name
            .entry(product.name.to_lowercase())
            .or_default()
            .push(product.clone());

        self.by_brand
            .entry(product.brand.to_lowercase())
            .or_default()
            .push(product.clone());

        self.by_category
            .entry(product.category.to_lowercase())
            .or_default()
            .push(product);
    }

    pub fn search_by_name(&self, name: &str) -> Option<&Vec<Product>> {
        self.by_name.get(&name.to_lowercase())
    }

    pub fn search_by_brand(&self, brand: &str) -> Option<&Vec<Product>> {
        self.by_brand.get(&brand.to_lowercase())
    }

    pub fn search_by_category(&self, category: &str) -> Option<&Vec<Product>> {
        self.by_category.get(&category.to_lowercase())
    }
}