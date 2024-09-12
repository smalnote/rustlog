// file: src/main.rs binary crate of project rust_by_practice

// make module available in current binary crate
// equivalent to `pub mod restaurant;` in src/lib.rs
pub mod restaurant;

// Run with command: cargo run --bin rust_by_practice
fn main() {
    // hierachy version of mod restaurant in directory src/restaurant/*
    rust_by_practice::restaurant::front_of_house::hosting::add_to_waitlist();
    rust_by_practice::restaurant::front_of_house::hosting::seat_at_table();
    rust_by_practice::restaurant::front_of_house::serving::take_order();
    rust_by_practice::restaurant::front_of_house::serving::serve_order();
    rust_by_practice::restaurant::front_of_house::serving::take_payment();
    rust_by_practice::restaurant::back_of_house::cooking::fire_tomato();
    rust_by_practice::restaurant::back_of_house::cooking::cook_fish();
    rust_by_practice::restaurant::back_of_house::cleaning::wash_dish();
    rust_by_practice::restaurant::back_of_house::cleaning::sweap_floor();
}
