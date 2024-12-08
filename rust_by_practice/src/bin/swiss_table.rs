use std::{thread, time::Duration};

use rust_by_practice::practice::p510_swiss_table::SwissTable;

/// Use command: `procs -W 0.1 --thread swiss_table`
/// to monitor memory usage.
fn main() {
    for i in 1..=10 {
        if i <= 5 {
            swiss_table(i * 512);
        } else {
            swiss_table((11 - i) * 512);
        }
        thread::sleep(Duration::from_millis(2000));
    }
    println!("exiting");
    // allocator won't release base on previous allocation
    // decrease it slowly
    swiss_table(128);
    swiss_table(64);
    swiss_table(32);
    swiss_table(16);
    // when exiting, usage is still remaining last table size
    thread::sleep(Duration::from_millis(5000));
}

fn swiss_table(k: usize) {
    println!(
        "swiss table use memory: {}MB",
        128_f32 * k as f32 / 1024_f32
    );
    let mut table = SwissTable::new();
    for _ in 0..10 {
        for i in 0..128 {
            let data = Vec::<u8>::with_capacity(k * 1024);
            table.insert(i, Box::new(data));
        }
        for i in 0..128 {
            table.remove(&i);
        }
    }
}
