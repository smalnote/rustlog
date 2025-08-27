#[cfg(test)]
mod test {

    // Rust 1.89
    #[test]
    fn test_dash_as_const_generic_argument() {
        fn all_false<const LEN: usize>() -> [bool; LEN] {
            [false; _]
        }
        let indicates: [bool; 10] = all_false();
        assert_eq!(indicates, [false; 10]);

        assert_eq!(all_false::<5>(), [false; 5]);
    }
}
