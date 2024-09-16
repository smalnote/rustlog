#[allow(dead_code)]
fn divide(x: f32, y: f32) -> Result<f32, &'static str> {
    if y == 0.0 {
        return Err("divided by zero");
    }
    Ok(x / y)
}

#[cfg(test)]
mod tests {
    /*
     * - `Result` is an enum type that represents the outcome of an operation
     *   that could potentially fail.
     * - Two possible variants of `Result<T, E>`:
     *   - Ok(T): A value T was found
     *   - Err(e): An error was found with a value e
     * - The expected outcome is Ok, the unexpected outcome is Err
     * - Since the `Result` is an enum, the possible variants can be matched
     *   using a match pattern.
     */

    use super::*;
    use std::num::{IntErrorKind, ParseIntError};

    #[test]
    fn safe_divide() {
        match divide(10_f32, 0.0) {
            Ok(_) => {}
            Err(e) => println!("Error message: {}", e),
        }
    }

    // Result.unwrap() expects a Ok(T) and takes out the value T, otherwise
    // it panics.
    #[test]
    fn unwrap_result() {
        let d = divide(10.0, 4.0);
        let d = d.unwrap();
        assert_eq!(d, 2.5);
    }

    #[test]
    #[should_panic]
    fn unwrap_result_with_err_will_panic() {
        let d = divide(10.0, 0.0);
        let _d = d.unwrap();
    }

    #[test]
    fn question_mark_unwrap_value_or_return_an_error() -> Result<(), &'static str> {
        fn divide_rest(nums: &[f32]) -> Result<f32, &'static str> {
            if nums.len() < 2 {
                return Err("no enough numbers");
            }

            let mut quotient: f32 = 0.0;
            for (i, v) in nums.iter().enumerate() {
                if i == 0 {
                    quotient = *v;
                } else {
                    // `?` unwrap divide result, or return if has error
                    quotient = divide(quotient, *v)?;
                }
            }
            Ok(quotient)
        }

        let nums: Vec<f32> = vec![1024.0, 2.0, 4.0, 8.0, 16.0];
        let quotient = divide_rest(&nums)?;
        assert_eq!(quotient, 1.0);
        Ok(())
    }

    #[test]
    fn elegant_string_multiply() {
        fn multiply(n1: &str, n2: &str) -> Result<i32, ParseIntError> {
            let n1 = n1.parse::<i32>()?;
            let n2 = n2.parse::<i32>()?;
            Ok(n1 * n2)
        }

        assert_eq!(multiply("10", "2"), Ok(20));
        assert_eq!(multiply("2", "4").unwrap(), 8);
    }

    #[test]
    fn question_dot_syntax() -> Result<(), ParseIntError> {
        fn multiply_str(n1: &str, n2: &str) -> Result<String, ParseIntError> {
            let n1 = n1.parse::<i32>()?;
            let n2 = n2.parse::<i32>()?;
            Ok((n1 * n2).to_string())
        }

        // `?.` unwrap value of Ok and call method on it
        assert_eq!(multiply_str("10", "20")?.parse::<i32>()?, 200);
        Ok(())
    }

    #[test]
    fn assert_error_type() {
        fn multiply(n1: &str, n2: &str) -> Result<i32, ParseIntError> {
            let n1 = n1.parse::<i32>()?;
            let n2 = n2.parse::<i32>()?;
            Ok(n1 * n2)
        }

        let result = multiply("n1", "n2");
        // the follow two has same semantic
        assert!(matches!(result, Err(ParseIntError { .. })));
        match result {
            Err(ParseIntError { .. }) => {}
            Ok(_) => panic!("Expected an error, but got Ok"),
        }

        assert!(result.is_err());
        assert!(result.is_err_and(|x| x.kind() == &std::num::IntErrorKind::InvalidDigit));

        // matches! is a marco for `match {}` with specific arm, and others return false
        assert!(matches!(
            multiply("n1", "n2").unwrap_err(),
            ParseIntError { .. }
        ));
    }

    #[test]
    #[allow(clippy::bind_instead_of_map)]
    fn map_result_value() {
        // Result<T, Error>.map(FnOnce(T) -> U) -> Result<U, Error>
        fn double(n: &str) -> Result<i32, ParseIntError> {
            n.parse::<i32>().map(|n| n * 2)
        }

        assert_eq!(double("42"), Ok(84));

        // Result<T, Error>.and_then(FnOnce(T) -> Result<U, Error>) -> Result<U, Error>
        fn double_then(n: &str) -> Result<u32, ParseIntError> {
            n.parse::<i32>().and_then(|n| Ok(n as u32 * 2))
        }
        assert_eq!(double_then("42"), Ok(84));
    }

    #[test]
    fn map_and_then() {
        fn multiply(n1: &str, n2: &str) -> Result<i32, ParseIntError> {
            n1.parse::<i32>()
                .and_then(|n1| n2.parse::<i32>().map(|n2| n1 * n2))
        }

        assert_eq!(multiply("2", "4"), Ok(8));
        assert!(multiply("n1", "4").is_err_and(|x| x.kind() == &IntErrorKind::InvalidDigit));
        assert!(multiply("2", "n2").is_err_and(|x| x.kind() == &IntErrorKind::InvalidDigit));
    }

    #[test]
    fn type_alias() {
        type Res<T> = Result<T, ParseIntError>;

        fn multiply(n1: &str, n2: &str) -> Res<i32> {
            n1.parse::<i32>()
                .and_then(|n1| n2.parse::<i32>().map(|n2| n1 * n2))
        }
        assert_eq!(multiply("2", "4"), Ok(8));
    }
}
