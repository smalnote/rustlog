use rand::Rng;
use time::OffsetDateTime;

fn main() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let greeting: &str = "Hello, world!";
    let mut rng = rand::rng();
    let random_number: i32 = rng.random_range(0..100);
    let now = OffsetDateTime::now_local().expect("Failed to get local date time");
    // marcos, generated code by function arguments.
    println!(
        "{greeting} @{now} with {}",
        add(random_number, random_number)
    );
}
