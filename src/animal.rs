pub trait Animal {
    fn eat(&self) {
        println!("Eat something");
    }
}

pub struct Tiger;

impl Animal for Tiger {
    fn eat(&self) {
        println!("Tiger is eating...");
    }
}

impl Tiger {
    pub fn sleep(&self) {
        println!("Tiger is sleeping...")
    }
}
