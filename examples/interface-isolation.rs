trait BaseAuth {
    fn register(&self);

    fn login(&self);
}

trait TestAuth {
    fn test(&self);
}

trait DevAuth {
    fn dev(&self);
}

trait PMAuth {
    fn pm(&self) {}
}

struct Test;
impl BaseAuth for Test {
    fn register(&self) {
        println!("Test register");
    }

    fn login(&self) {
        println!("Test login");
    }
}

impl TestAuth for Test {
    fn test(&self) {
        println!("Test test");
    }
}

struct Dev;
impl BaseAuth for Dev {
    fn register(&self) {
        println!("Dev register");
    }

    fn login(&self) {
        println!("Dev login");
    }
}

impl DevAuth for Dev {
    fn dev(&self) {
        println!("Dev dev");
    }
}

struct PM;
impl BaseAuth for PM {
    fn register(&self) {
        println!("PM register");
    }

    fn login(&self) {
        println!("PM login");
    }
}

impl PMAuth for PM {
    fn pm(&self) {
        println!("PM pm");
    }
}

fn main() {
    let test = Test;
    test.login();
    test.register();

    test.test();

    let dev = Dev;
    dev.login();
    dev.register();

    dev.dev();

    let pm = PM;
    pm.login();
    pm.register();

    pm.pm();
}
