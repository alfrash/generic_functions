fn pick<T>(value: u8, even: T, odd: T) -> T {
    if value % 2 == 0 { even } else { odd }
}

trait Random {
    fn generate() -> Self;
}

impl Random for u64 {
    fn generate() -> Self {
        //return any random number
        55
    }
}

impl Random for &str {
    fn generate() -> Self {
        "ahmed"
    }
}

fn my_random<T>() -> T
where
    T: Random,
{
    T::generate()
}

fn my_pick(v: u64) {
    println!("you picked {}", v);
}

fn main() {
    let value = rand::random();

    println!("you picked: {}, it's {}", value, pick(value, "even", "odd"));
    println!("you picked: {}, it's {}", value, pick(value, true, false));
    println!("you picked: {}, it's {}", value, pick(value, 100, 200));

    let v = my_random();
    my_pick(v);
}
