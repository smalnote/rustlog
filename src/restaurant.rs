pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Function called: add_to_waitlist()")
        }

        pub fn seat_at_table() {
            println!("Function called: seat_at_table()")
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("Function called: take_order()")
        }

        pub fn serve_order() {
            println!("Function called: serve_order()")
        }

        pub fn take_payment() {
            println!("Function called: take_payment()")
        }
    }
}

pub mod back_of_house {
    pub mod cooking {
        pub fn fire_tomato() {
            println!("Function called: fire_tomato()")
        }

        pub fn cook_fish() {
            println!("Function called: cook_fish()")
        }
    }

    pub mod cleaning {
        pub fn wash_dish() {
            println!("Function called: wash_dish()")
        }

        pub fn sweap_floor() {
            println!("Function called: add_to_waitlist()")
        }
    }
}
