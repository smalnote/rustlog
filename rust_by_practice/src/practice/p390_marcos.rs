#[cfg(test)]
mod tests {
    #[test]
    fn test_procedural_marcos() {
        #[macro_export]
        macro_rules! simple_vec {
            ( $( $x:expr ),* ) => {
                {
                    let mut temp_vec = Vec::new();
                    $(
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }

        let _numbers = simple_vec![1, 2, 3, 4, 5];
        let _strs = simple_vec!("hello", "world");

        assert_eq!(_numbers, vec!(1, 2, 3, 4, 5));
        assert_eq!(_strs, vec!["hello", "world"]);

        // brackets also work for procedural marcos
        println!["hello, world"];
    }

    /// Custom derive marco should be in a proc-marco lib crate, skip.
    #[test]
    fn test_custom_derive_attribute_marcos() {
        use hello_marco::HelloMarco;
        use hello_marco_derive::HelloMarco;
        #[derive(HelloMarco)]
        struct Pancakes;

        Pancakes::hello_marco();
    }
}
