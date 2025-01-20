use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Item {
    id: u32,
    name: String,
    quantity: u32,
    price: f64,
}

#[derive(Debug)]
struct Inventory {
    items: HashMap<u32, Item>,
    next_item_id: u32,
}

#[derive(Debug)]
enum InventoryError {
    DuplicateItem(String),
    ItemNotFound(u32),
    InvalidUpdate,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            items: HashMap::new(),
            next_item_id: 1,
        }
    }

    fn add_item(&mut self, name: String, quantity: u32, price: f64) -> Result<(), InventoryError> {
        if self.items.values().any(|item| item.name == name) {
            // Return a DuplicateItem error with the name of the duplicate item
            return Err(InventoryError::DuplicateItem(name));
        }

        let new_item = Item {
            id: self.next_item_id,
            name,
            quantity,
            price,
        };

        self.items.insert(new_item.id, new_item);
        self.next_item_id += 1;

        Ok(())
    }

    fn update_item(
        &mut self,
        id: u32,
        name: Option<String>,
        quantity: Option<u32>,
        price: Option<f64>,
    ) -> Result<(), InventoryError> {
        if let Some(item) = self.items.get_mut(&id) {
            if let Some(name) = name {
                item.name = name;
            }
            if let Some(quantity) = quantity {
                item.quantity = quantity;
            }
            if let Some(price) = price {
                item.price = price;
            }

            Ok(())
        } else {
            // Return an ItemNotFound error with the ID of the missing item
            Err(InventoryError::ItemNotFound(id))
        }
    }

    fn delete_item(&mut self, id: u32) -> Result<(), InventoryError> {
        if self.items.remove(&id).is_some() {
            Ok(())
        } else {
            // Return an ItemNotFound error with the ID of the missing item
            Err(InventoryError::ItemNotFound(id))
        }
    }

    fn list_items(&self) -> Vec<&Item> {
        self.items.values().collect()
    }

    fn find_item(&self, name: &str) -> Result<&Item, InventoryError> {
        self.items.values().find(|item| item.name == name).ok_or_else(|| {
            // Return an ItemNotFound error with the name of the missing item
            InventoryError::ItemNotFound(0) // Using 0 for the ID as it's missing
        })
    }
}

fn main() {
    let mut inventory = Inventory::new();

    // Example usage
    match inventory.add_item("Laptop".to_string(), 10, 999.99) {
        Ok(_) => println!("Item added!"),
        Err(e) => println!("Error: {:?}", e),
    }

    match inventory.add_item("Smartphone".to_string(), 20, 499.99) {
        Ok(_) => println!("Item added!"),
        Err(e) => println!("Error: {:?}", e),
    }

    println!("/////Inventory/////");
    for item in inventory.list_items() {
        println!("{:?}", item);
    }
    println!("/////End/////");

    // Test updating an item
    match inventory.update_item(1, Some("Gaming Laptop".to_string()), None, Some(1299.99)) {
        Ok(_) => println!("Item updated!"),
        Err(e) => println!("Error: {:?}", e),
    }

    // Test deleting an item
    match inventory.delete_item(2) {
        Ok(_) => println!("Item deleted!"),
        Err(e) => println!("Error: {:?}", e),
    }

    // Test finding an item
    match inventory.find_item("Gaming Laptop") {
        Ok(item) => println!("Found item: {:?}", item),
        Err(e) => println!("Error: {:?}", e),
    }

    // Test finding a non-existing item
    match inventory.find_item("Business Laptop") {
        Ok(item) => println!("Found item: {:?}", item),
        Err(e) => println!("Error: {:?}", e),
    }

    // Try adding a duplicate item
    match inventory.add_item("Gaming Laptop".to_string(), 10, 879.99) {
        Ok(_) => println!("Item added!"),
        Err(e) => println!("Error: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_item_should_be_ok() {
        let mut inventory = Inventory::new();
        assert!(inventory.add_item("Test Item".to_string(), 10, 9.99).is_ok());
    }

    #[test]
    fn test_add_duplicate_item_should_fail() {
        let mut inventory = Inventory::new();
        inventory.add_item("Test Item".to_string(), 10, 9.99).unwrap();
        let result = inventory.add_item("Test Item".to_string(), 5, 19.99);
        assert!(result.is_err());
        if let Err(InventoryError::DuplicateItem(name)) = result {
            assert_eq!(name, "Test Item");
        }
    }

    #[test]
    fn test_update_item_should_fail_if_not_found() {
        let mut inventory = Inventory::new();
        inventory.add_item("Test Item".to_string(), 10, 9.99).unwrap();
        let result = inventory.update_item(999, Some("Updated Item".to_string()), None, None);
        assert!(result.is_err());
        if let Err(InventoryError::ItemNotFound(id)) = result {
            assert_eq!(id, 999);
        }
    }
}
