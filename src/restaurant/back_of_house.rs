// file: src/restaurant/back_of_house.rs
// must have a mod.rs donates for directory src/restaurant/back_of_house
// mod path: rustlog::restaurant::back_of_house

// mod cleaning inside: rustlog::restaurant::back_of_house::cleaning
pub mod cleaning {
    pub fn wash_dish() {
        println!("Function called: wash_dish()");
    }

    pub fn sweap_floor() {
        println!("Function called: add_to_waitlist()");
    }
}

pub mod cooking {
    pub fn fire_tomato() {
        crate::restaurant::front_of_house::serving::serve_order();
        println!("Function called: fire_tomato()");
        super::cleaning::wash_dish();
    }

    pub fn cook_fish() {
        println!("Function called: cook_fish()");
    }
}
