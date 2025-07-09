use megastore_search::graph::ProductGraph;
use megastore_search::product::Product;

fn sample_products() -> Vec<Product> {
    vec![
        Product {
            id: "001".to_string(),
            name: "Smartphone X".to_string(),
            brand: "TechBrand".to_string(),
            category: "Eletrônicos".to_string(),
            price: 1999.99,
        },
        Product {
            id: "002".to_string(),
            name: "Smartphone Y".to_string(),
            brand: "TechBrand".to_string(),
            category: "Eletrônicos".to_string(),
            price: 2499.99,
        },
    ]
}

#[test]
fn test_graph_recommendation() {
    let mut graph = ProductGraph::new();
    for p in sample_products() {
        graph.add_product(p);
    }

    graph.auto_connect();

    let recs = graph.recommend("001", 0.5);
    assert_eq!(recs.len(), 1);
    assert_eq!(recs[0].0.id, "002");
}