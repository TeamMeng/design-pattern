trait Payment {
    fn process(&self);
}

struct PaymentManagement;

impl PaymentManagement {
    fn process<T: Payment>(&self, payment: T) {
        payment.process();
    }
}

struct WeChatPay;

impl Payment for WeChatPay {
    fn process(&self) {
        println!("WeChat pay...")
    }
}

struct AliPay;

impl Payment for AliPay {
    fn process(&self) {
        println!("Ali pay...")
    }
}

// Add code - open and close
struct Card;

impl Payment for Card {
    fn process(&self) {
        println!("Card pay...")
    }
}

fn main() {
    let w = WeChatPay;
    let a = AliPay;

    let p = PaymentManagement;
    p.process(w);
    p.process(a);

    let c = Card;
    p.process(c);
}
