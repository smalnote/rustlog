#[cfg(test)]
mod tests {
    #[test]
    fn display_trait_provide_to_string_trait() {
        struct Point {
            x: i32,
            y: i32,
        }

        impl std::fmt::Display for Point {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        assert_eq!((Point { x: 32, y: 42 }).to_string(), "(32, 42)");
        println!("The point is {}", Point { x: 32, y: 42 });
    }

    #[test]
    fn trait_from_str_for_convert_string_to_number() {
        use std::str::FromStr;

        let parsed: i32 = "5".parse().unwrap();
        let turbo_parsed = "10".parse::<i32>().unwrap(); // ::<i32> is called turbo fix syntax
        let from_str = i32::from_str("20").unwrap();
        assert_eq!(parsed + turbo_parsed + from_str, 35);
    }
}
