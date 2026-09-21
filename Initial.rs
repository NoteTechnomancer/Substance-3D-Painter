use std::collections::HashMap;

struct Product {
    name: String,
    price: f64,
    quantity: u32,
}

struct Inventory {
    products: HashMap<String, Product>,
}

impl Inventory {
    fn new() -> Self {
        Self {
            products: HashMap::new(),
        }
    }

    fn add_product(&mut self, name: &str, price: f64, quantity: u32) {
        self.products.insert(
            name.to_string(),
            Product {
                name: name.to_string(),
                price,
                quantity,
            },
        );
    }

    fn total_value(&self) -> f64 {
        self.products
            .values()
            .map(|product| product.price * product.quantity as f64)
            .sum()
    }

    fn print_report(&self) {
        println!("Inventory Report");
        println!("================");

        let mut products: Vec<&Product> = self.products.values().collect();
        products.sort_by(|a, b| a.name.cmp(&b.name));

        for product in products {
            let value = product.price * product.quantity as f64;

            println!(
                "{} | ${:.2} | {} units | ${:.2}",
                product.name,
                product.price,
                product.quantity,
                value
            );
        }

        println!("================");
        println!("Total Value: ${:.2}", self.total_value());
    }
}

fn main() {
    let mut inventory = Inventory::new();

    inventory.add_product("Laptop", 899.99, 5);
    inventory.add_product("Keyboard", 79.50, 12);
    inventory.add_product("Mouse", 39.99, 20);
    inventory.add_product("Monitor", 249.99, 8);

    inventory.print_report();
}