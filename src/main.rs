mod phone;

use phone::Phone;

fn main() {
    let phone = Phone::new("小米", 3999.9);
    phone.info();
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1);
    }
}
