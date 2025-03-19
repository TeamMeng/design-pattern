pub struct Phone {
    name: String,
    price: f32,
}

impl Phone {
    pub fn new(name: &str, price: f32) -> Self {
        Self {
            name: name.to_string(),
            price,
        }
    }

    pub fn info(&self) {
        println!("{} phone price is {}", self.name, self.price);
    }
}
