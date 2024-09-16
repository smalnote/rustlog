#[cfg(test)]
mod tests {

    #[test]
    fn implement_trait_from_get_into_trait_for_given_type() {
        #[derive(Debug, PartialEq)]
        struct Number {
            value: i32,
        }
        // impl From<i32> for Number
        // get impl Into<Number> fro i32 implicitly
        impl From<i32> for Number {
            fn from(value: i32) -> Self {
                Self { value }
            }
        }

        /*
         * The `Into` trait is a reciprocal of trait `From`
         * impl<T, U> Into<U> for T
         *     where
         *         U: From<T>,
         */

        let n = Number::from(42);
        let t: Number = 42_i32.into();

        assert_eq!(t, n);
        assert_eq!(t.value, 42);
        assert_eq!(n.value, 42);
    }

    // `?` question mark converts error to target error type with Into trait in Result<_, TargetError>
    #[test]
    fn question_mark_for_auto_coercion() {
        #[derive(Debug, PartialEq)]
        #[allow(dead_code)]
        enum CliError {
            IoError,
            ParseError,
        }

        impl From<std::io::Error> for CliError {
            fn from(_: std::io::Error) -> Self {
                Self::IoError
            }
        }

        impl From<std::num::ParseIntError> for CliError {
            fn from(_: std::num::ParseIntError) -> Self {
                Self::ParseError
            }
        }

        fn simulate_io_error(io_err: bool, parse_err: bool) -> Result<String, std::io::Error> {
            if io_err {
                Err(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "io error",
                ))
            } else if parse_err {
                Ok("NaN".to_string())
            } else {
                Ok("42".to_string())
            }
        }

        // using question mark `?` to convert error type by using trait `From`
        fn read_io_and_parse_int(io_err: bool, parse_err: bool) -> Result<i32, CliError> {
            // `?` automatically converts io:Error to CliError
            // std::io::Error.into() CliError, by the From<std::io::Error> trait
            let s = simulate_io_error(io_err, parse_err)?;
            // `?` automatically converts num::ParseIntError to CliError
            // std::num::ParseIntError.into() CliError, by the  From<std::num::ParseIntError> trait
            let v = s.parse::<i32>()?;
            Ok(v)
        }

        assert_eq!(
            read_io_and_parse_int(true, false).err(),
            Some(CliError::IoError)
        );
        assert_eq!(
            read_io_and_parse_int(false, true).err(),
            Some(CliError::ParseError)
        );
        assert_eq!(read_io_and_parse_int(false, false).ok(), Some(42));
    }

    // trait TryInto and TryFrom is a fallible version of Into and From that return a Result
    #[test]
    fn try_into() {
        let v: u8 = match 420.try_into() {
            Ok(n) => n,
            Err(e) => {
                println!("there is an error when converting: {}, but we catch it", e);
                0
            }
        };
        assert_eq!(v, 0);
    }

    // Like `Into` and `From` being reciprocal traits.
    // `TryInto` and `TryFrom` are reciprocal traits, implementing `TryFrom` trait get `TryInto` trait free.
    #[test]
    fn try_from() {
        #[derive(Debug, PartialEq)]
        struct EvenNumber(i32);

        impl TryFrom<i32> for EvenNumber {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value & 1 == 0 {
                    Ok(EvenNumber(value))
                } else {
                    Err(())
                }
            }
        }

        assert_eq!(EvenNumber::try_from(1), Err(()));
        assert_eq!(EvenNumber::try_from(42), Ok(EvenNumber(42)));

        let result: Result<EvenNumber, ()> = 42.try_into();
        assert_eq!(result, Ok(EvenNumber(42)));
        let result: Result<EvenNumber, ()> = 3.try_into();
        assert_eq!(result, Err(()));
    }
}
