use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use crate::product::Product;

pub struct ProductGraph {
    pub graph: Graph<Product, f64, Undirected>,
    pub index_map: HashMap<String, NodeIndex>, // Map ID do produto → nó
}

impl ProductGraph {
    /// Cria um novo grafo de produtos
    pub fn new() -> Self {
        Self {
            graph: Graph::new_undirected(),
            index_map: HashMap::new(),
        }
    }

    /// Adiciona um produto como nó no grafo
    pub fn add_product(&mut self, product: Product) {
        let node = self.graph.add_node(product.clone());
        self.index_map.insert(product.id.clone(), node);
    }

    /// Conecta dois produtos manualmente com um peso de similaridade
    pub fn connect_similar(&mut self, id1: &str, id2: &str, weight: f64) {
        if let (Some(&a), Some(&b)) = (self.index_map.get(id1), self.index_map.get(id2)) {
            self.graph.add_edge(a, b, weight);
        }
    }

    /// Conecta automaticamente produtos com base em categoria e marca
    pub fn auto_connect(&mut self) {
        let nodes: Vec<_> = self.graph.node_indices().collect();

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let a = &self.graph[nodes[i]];
                let b = &self.graph[nodes[j]];

                let mut weight = 0.0;
                if a.category == b.category {
                    weight += 0.8;
                }
                if a.brand == b.brand {
                    weight += 0.6;
                }

                if weight > 0.0 {
                    self.graph.add_edge(nodes[i], nodes[j], weight);
                }
            }
        }
    }

    /// Retorna os produtos conectados ao produto com o ID fornecido, filtrando por peso mínimo
    pub fn recommend(&self, product_id: &str, min_weight: f64) -> Vec<(&Product, f64)> {
        if let Some(&node) = self.index_map.get(product_id) {
            let mut recommendations: Vec<_> = self
                .graph
                .edges(node)
                .filter_map(|edge| {
                    let weight = *edge.weight();
                    if weight >= min_weight {
                        let neighbor = edge.target();
                        Some((&self.graph[neighbor], weight))
                    } else {
                        None
                    }
                })
                .collect();

            recommendations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            recommendations
        } else {
            vec![]
        }
    }
}