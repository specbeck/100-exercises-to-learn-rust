// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    product_name: String,
    quantity: u16,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u16, unit_price: u32) -> Order {
        Order::validate_name(&product_name);
        Order::validate_quantity(&quantity);
        Order::validate_price(&unit_price);

        Order {
            product_name,
            quantity,
            unit_price
        }
    }
    // Helper functions
    fn validate_name(name: &str) {
        if name.is_empty() {
            panic!("Product name cannot be empty!")
        }
        if name.len() > 300 {
            panic!("Product name cannot be longer than 300 bytes")
        }
    }

    fn validate_quantity(q: &u16) {
        if *q <= 0 {
            panic!("Quantity cannot be negative!");
        }
    }

    fn validate_price(p: &u32) {
        if *p <= 0 {
            panic!("Price cannot be less than zero!")
        }
    }

    // getters
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &u16 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    // setters
    pub fn set_product_name(&mut self, name: String) {
        Order::validate_name(&name);
        self.product_name = name
    }
    pub fn set_quantity(&mut self, quantity: u16) {
        Order::validate_quantity(&quantity);
        self.quantity = quantity
    }
    pub fn set_unit_price(&mut self, price: u32) {
        Order::validate_price(&price);
        self.unit_price = price
    }
    
    pub fn total(&self) -> u32 {
        *&self.quantity as u32 * &self.unit_price
    }
}
