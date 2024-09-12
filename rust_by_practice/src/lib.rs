//! # rustlog
//!
//! `rustlog` is a collection of code snippets for practicing Rust.
//! file: src/lib.rs library crate of project rustlog
//! file: src/restaurant.rs

// make module available in src/bin/*, src/main.rs
pub mod restaurant;

pub mod practice;

// re-export function with a short namespace rustlog::add_one
pub use self::practice::p320_documentation::add_one;

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_one() {
        let x = 41;
        let y = super::add_one(x);
        assert_eq!(y, 42);

        let z = super::practice::p320_documentation::add_one(y);
        assert_eq!(z, 43);
    }
}
