use rand::Rng;
use time::OffsetDateTime;

fn main() {
    let greeting: &str = "Hello, world!";
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(0..100);
    let now = OffsetDateTime::now_local().expect("Failed to get local date time");
    // marcos, generated code by function arguments.
    println!("{greeting} @{now} with {}", add(random_number, random_number));
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
