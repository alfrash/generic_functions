fn pick(value: u32) -> &'static str {
    if value % 2 == 0 { "even" } else { "odd" }
}

fn main() {
    let value = rand::random();
    let my_string = "try this";

    println!("you picked: {}, it's {}", value, pick(value));
}
