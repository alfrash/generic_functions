use rand::*;

use crate::Either::That;

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

enum Either<A, B> {
    This(A),
    That(B),
}

fn pick_either<A, B>(value: u8, even: A, odd: B) -> Either<A, B> {
    if value % 2 == 0 {
        Either::This(even)
    } else {
        Either::That(odd)
    }
}
fn main() {
    let value = rand::random();

    println!("you picked: {}, it's {}", value, pick(value, "even", "odd"));
    println!("you picked: {}, it's {}", value, pick(value, true, false));
    println!("you picked: {}, it's {}", value, pick(value, 100, 200));

    let v = my_random();
    pick_random(v);
    //you can change even or odd to be any type
    // string and number
    // string and string
    // number and number
    let output = pick_either(value, "eve", 3);
    match output {
        Either::This(even) => {
            println!("you picked either: {}", even)
        }
        Either::That(odd) => {
            println!("you picked either: {}", odd)
        }
    }
}
