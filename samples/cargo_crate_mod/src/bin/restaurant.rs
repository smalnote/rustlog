/// Binary in src/bin have to import mod from src/* */ with a package name instead
/// `crate::`, make the src/bin as a external user of mods of src/*.
use cargo_crate_mod::restaurant;

// mod from ./bin_mod/mod.rs is used as a internal mod.
mod bin_mod;

// Run this binary crate with command: cargo run --bin restaurant
fn main() {
    restaurant::front_of_house::hosting::add_to_waitlist();
    restaurant::front_of_house::hosting::seat_at_table();
    restaurant::front_of_house::serving::take_order();
    restaurant::front_of_house::serving::serve_order();
    restaurant::front_of_house::serving::take_payment();
    restaurant::back_of_house::cooking::fire_tomato();
    restaurant::back_of_house::cooking::cook_fish();
    restaurant::back_of_house::cleaning::wash_dish();
    restaurant::back_of_house::cleaning::sweep_floor();
    bin_mod::restaurant::post_cleaning();
}
