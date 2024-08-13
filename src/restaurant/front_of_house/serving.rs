pub fn take_order() {
    println!("Function called: take_order()");
}

pub fn serve_order() {
    // mod path of parent
    super::hosting::seat_at_table();
    // mod path from crate root
    crate::restaurant::back_of_house::cooking::cook_fish();
    println!("Function called: serve_order()");
}

pub fn take_payment() {
    println!("Function called: take_payment()");
}
