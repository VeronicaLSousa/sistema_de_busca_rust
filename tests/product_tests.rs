use megastore_search::product::Product;

#[test]
fn test_product_creation() {
    let p = Product {
        id: "001".to_string(),
        name: "Smartphone X".to_string(),
        brand: "TechBrand".to_string(),
        category: "Eletrônicos".to_string(),
        price: 1999.99,
    };

    assert_eq!(p.name, "Smartphone X");
    assert_eq!(p.brand, "TechBrand");
    assert_eq!(p.price, 1999.99);
}
