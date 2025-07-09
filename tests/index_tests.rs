use megastore_search::index::ProductIndex;
use megastore_search::product::Product;

#[test]
fn test_search_by_brand() {
    let mut index = ProductIndex::new();

    let p1 = Product {
        id: "001".to_string(),
        name: "Smartphone X".to_string(),
        brand: "TechBrand".to_string(),
        category: "Eletrônicos".to_string(),
        price: 1999.99,
    };

    let p2 = Product {
        id: "002".to_string(),
        name: "Smartphone Y".to_string(),
        brand: "TechBrand".to_string(),
        category: "Eletrônicos".to_string(),
        price: 2499.99,
    };

    index.insert(p1.clone());
    index.insert(p2.clone());

    let results = index.search_by_brand("TechBrand");
    assert!(results.is_some());
    let list = results.unwrap();
    assert_eq!(list.len(), 2);
    assert!(list.contains(&p1));
    assert!(list.contains(&p2));
}