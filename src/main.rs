use rand::Rng;
use time::OffsetDateTime;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen();
    let now = OffsetDateTime::now_utc();
    println!("Hello, world! @{now} with {}",random_number);
}
