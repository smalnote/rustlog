// file: src/restaurant2/back_of_house/mod.rs
// must have a mod.rs donates for directory src/restaurant2/back_of_house
// mod path: rustlog::restaurant2::back_of_house

// make mod available
pub mod cooking;

// mod cleaning inside: rustlog::restaurant2::back_of_house::cleaning
pub mod cleaning {
    pub fn wash_dish() {
        println!("Function called: wash_dish()")
    }

    pub fn sweap_floor() {
        println!("Function called: add_to_waitlist()")
    }
}
