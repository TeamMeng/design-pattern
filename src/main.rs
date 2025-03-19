trait Animal {
    fn eat(&self);
}

struct Tiger;
impl Animal for Tiger {
    fn eat(&self) {
        println!("Tiger eating...")
    }
}

struct Person;
impl Animal for Person {
    fn eat(&self) {
        println!("Person eating...")
    }
}

fn animal_eat<T: Animal>(animal: T) {
    animal.eat();
}

fn main() {
    let tiger = Tiger;
    let person = Person;

    animal_eat(tiger);
    animal_eat(person);
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
