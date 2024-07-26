#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use rand::Rng;
use time::OffsetDateTime;
fn main() {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(0..100);
    let now = OffsetDateTime::now_local().expect("Failed to get local date time");
    {
        ::std::io::_print(
            format_args!(
                "Hello, world! @{1} with {0}\n",
                add(random_number, random_number),
                now,
            ),
        );
    };
}
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
