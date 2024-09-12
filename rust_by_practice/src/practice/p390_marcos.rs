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

    #[test]
    fn test_marco_for_length_unit_calculation() {
        use length::AddLengths;
        use std::ops::Add;

        #[derive(Debug, PartialEq, AddLengths)]
        struct Millimeters(f64);

        #[derive(Debug, PartialEq, AddLengths)]
        struct Meters(f64);

        #[derive(Debug, PartialEq, AddLengths)]
        struct Kilometers(f64);

        impl From<Meters> for Millimeters {
            fn from(value: Meters) -> Self {
                Millimeters(value.0 * 1000.0)
            }
        }

        impl From<Kilometers> for Millimeters {
            fn from(value: Kilometers) -> Self {
                Millimeters(value.0 * 1_000_000.0)
            }
        }

        impl From<Millimeters> for Meters {
            fn from(value: Millimeters) -> Self {
                Meters(value.0 / 1000.0)
            }
        }

        impl From<Kilometers> for Meters {
            fn from(value: Kilometers) -> Self {
                Meters(value.0 * 1000.0)
            }
        }

        impl From<Millimeters> for Kilometers {
            fn from(value: Millimeters) -> Self {
                Kilometers(value.0 / 1_000_000.0)
            }
        }

        impl From<Meters> for Kilometers {
            fn from(value: Meters) -> Self {
                Kilometers(value.0 / 1000.0)
            }
        }

        assert_eq!(Millimeters(1.0) + Meters(1.0), Millimeters(1001.0));
        assert_eq!(
            Millimeters(1.0) + Meters(1.0) + Kilometers(1.0),
            Millimeters(1001001.0)
        );
        assert_eq!(
            Meters(1.0) + Millimeters(500.0) + Kilometers(2.0),
            Meters(2001.5)
        );

        assert_eq!(
            Kilometers(1.0) + Meters(1.0) + Millimeters(2_000.0),
            Kilometers(1.003)
        );
    }
}
