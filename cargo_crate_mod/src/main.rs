// file: src/main.rs binary crate of project crate

// make module available in current binary crate
// equivalent to `pub mod restaurant;` in src/lib.rs
pub mod restaurant;

// Run with command: cargo run --bin crate
fn main() {
    // hierachy version of mod restaurant in directory src/restaurant/*
    crate::restaurant::front_of_house::hosting::add_to_waitlist();
    crate::restaurant::front_of_house::hosting::seat_at_table();
    crate::restaurant::front_of_house::serving::take_order();
    crate::restaurant::front_of_house::serving::serve_order();
    crate::restaurant::front_of_house::serving::take_payment();
    crate::restaurant::back_of_house::cooking::fire_tomato();
    crate::restaurant::back_of_house::cooking::cook_fish();
    crate::restaurant::back_of_house::cleaning::wash_dish();
    crate::restaurant::back_of_house::cleaning::sweep_floor();
}
