use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CartItem {
    pub product_id: String,
    pub quantity: u32,
}

#[derive(Debug)]
pub struct Cart {
    items: HashMap<String, CartItem>,
}

impl Cart {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    // ➕ Dodanie produktu do koszyka
    pub fn add(&mut self, product_id: String, quantity: u32) {
        let item: &mut CartItem = self.items.entry(product_id.clone()).or_insert(CartItem {
            product_id,
            quantity: 0,
        });

        item.quantity += quantity;
    }

    // ❌ Usunięcie produktu z koszyka
    pub fn delete(&mut self, product_id: &str) {
        self.items.remove(product_id);
    }

    // 📋 (opcjonalnie) pobranie wszystkich produktów
    pub fn items(&self) -> Vec<&CartItem> {
        self.items.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_empty_cart() {
        let cart = Cart::new();

        assert_eq!(cart.items().len(), 0);
    }

    #[test]
    fn should_add_product_to_cart() {
        let mut cart = Cart::new();

        cart.add("product-1".to_string(), 2);

        let items = cart.items();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].product_id, "product-1");
        assert_eq!(items[0].quantity, 2);
    }

    #[test]
    fn should_increase_quantity_when_adding_same_product() {
        let mut cart = Cart::new();

        cart.add("product-1".to_string(), 2);
        cart.add("product-1".to_string(), 3);

        let items = cart.items();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].quantity, 5);
    }

    #[test]
    fn should_delete_product_from_cart() {
        let mut cart = Cart::new();

        cart.add("product-1".to_string(), 2);
        cart.delete("product-1");

        assert_eq!(cart.items().len(), 0);
    }

    #[test]
    fn should_not_fail_when_deleting_non_existing_product() {
        let mut cart = Cart::new();

        cart.delete("not-exists");

        assert_eq!(cart.items().len(), 0);
    }
}