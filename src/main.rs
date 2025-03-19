struct Product {
    id: i32,
    price: f32,
    name: String,
}

impl Product {
    fn new() -> Self {
        Self {
            id: 100,
            price: 9.9,
            name: "mobie phone case".to_string(),
        }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }

    fn set_price(&mut self, price: f32) {
        self.price = price
    }
}

struct Alipay;

impl Alipay {
    fn pay(&self, pid: i32) {
        println!("use wechat to pay, pid by {}", pid);
    }
}

fn main() {
    let mut product = Product::new();
    product.set_price(10.0);
    product.set_name("Xiao mi");

    let alipay = Alipay;
    alipay.pay(product.id);
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
