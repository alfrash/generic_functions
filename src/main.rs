use rand::*;

fn pick<T>(value: u8, even: T, odd: T) -> T {
    if value % 2 == 0 { even } else { odd }
}

impl Random for u8 {
    fn generate() -> Self {
        let mut rng = rand::rng();
        rng.random_range(1..101)
    }
}

impl Random for &str {
    fn generate() -> Self {
        let names = ["Ahmed", "Sara", "Ali", "Fatima", "John"];
        let mut rng = rand::rng();
        let random_index = rng.random_range(0..names.len());
        names[random_index]
    }
}

trait Random {
    fn generate() -> Self;
}
fn my_random<T>() -> T
where
    T: Random,
{
    Random::generate()
}

// you can change type to u8 or &str
fn pick_random(v: &str) {
    println!("you picked: {}", v);
}

fn main() {
    let value = rand::random();

    println!("you picked: {}, it's {}", value, pick(value, "even", "odd"));
    println!("you picked: {}, it's {}", value, pick(value, true, false));
    println!("you picked: {}, it's {}", value, pick(value, 100, 200));

    let v = my_random();
    pick_random(v);
}
