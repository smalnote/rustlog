// file: src/main.rs binary crate of project rustlog

// make module available in current binary crate
// equivlent to `pub mod restaurant;` in src/lib.rs
pub mod restaurant;

fn main() {
    println!("\nUsing functions of mod restaurant:");
    // pure file mod of src/restaurant.rs rustlog::restaurant::*;
    rustlog::restaurant::front_of_house::hosting::add_to_waitlist();
    rustlog::restaurant::front_of_house::hosting::seat_at_table();
    rustlog::restaurant::front_of_house::serving::take_order();
    rustlog::restaurant::front_of_house::serving::serve_order();
    rustlog::restaurant::front_of_house::serving::take_payment();
    rustlog::restaurant::back_of_house::cooking::fire_tomato();
    rustlog::restaurant::back_of_house::cooking::cook_fish();
    rustlog::restaurant::back_of_house::cleaning::wash_dish();
    rustlog::restaurant::back_of_house::cleaning::sweap_floor();

    println!("\nUsing functions of mod restaurant2:");
    // hierachy version of mod restaurant in directory src/restaurant/*
    rustlog::restaurant2::front_of_house::hosting::add_to_waitlist();
    rustlog::restaurant2::front_of_house::hosting::seat_at_table();
    rustlog::restaurant2::front_of_house::serving::take_order();
    rustlog::restaurant2::front_of_house::serving::serve_order();
    rustlog::restaurant2::front_of_house::serving::take_payment();
    rustlog::restaurant2::back_of_house::cooking::fire_tomato();
    rustlog::restaurant2::back_of_house::cooking::cook_fish();
    rustlog::restaurant2::back_of_house::cleaning::wash_dish();
    rustlog::restaurant2::back_of_house::cleaning::sweap_floor();
}
