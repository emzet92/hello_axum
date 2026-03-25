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
        let item = self.items.entry(product_id.clone()).or_insert(CartItem {
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