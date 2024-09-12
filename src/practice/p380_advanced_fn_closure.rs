#[cfg(test)]
mod tests {
    /// Closure trait: Fn(), FnMut(), FnOnce()
    /// Function pointer: fn()
    /// Function pointer implement all three of the closure traits(Fn, FnMut, FnOnce),
    /// meaning you can always pass a function pointer as an argument for a function
    /// that expects a closure.
    /// It's best to write your function using a generic type bound to one of the
    /// closure traits so your function can accept either functions or closures.
    #[test]
    fn test_function_pointer_and_closure_trait() {
        fn double(value: i32) -> i32 {
            value * 2
        }
        fn do_twice(value: i32, fn_pointer: fn(value: i32) -> i32) -> i32 {
            let value = fn_pointer(value);
            fn_pointer(value)
        }
        assert_eq!(do_twice(42, double), 168);

        let add_one = |value: i32| -> i32 { value + 1 };
        fn do_closure_twice<F: Fn(i32) -> i32>(value: i32, f: F) -> i32 {
            f(f(value))
        }
        assert_eq!(do_closure_twice(42, add_one), 44);

        // use fn pointer `double` as Fn trait
        assert_eq!(do_closure_twice(42, double), 168);
    }

    /// Closure is an unsized type, returning closure by wrap it in Box<dyn Fn()>
    #[test]
    fn test_returning_closure_trait_object() {
        // passing in `Fn() -> f64` is static dispatch
        // returning `Fn() -> f64` that is unsized, requires Box<dyn Fn() -> f64>
        fn tax<F: Fn() -> f64 + 'static>(calc_fee: F) -> Box<dyn Fn() -> f64> {
            let calc_fee_with_tax = move || -> f64 {
                let fee = calc_fee();
                match fee {
                    0.0..5000.0 => fee,
                    5000.0..10000.0 => 5000.0 + (fee - 5000.0) * 1.016,
                    10000.0.. => 5000.0 + (5000.0) * 1.016 + (fee - 10000.0) * 1.020,
                    _ => panic!("invalid fee"),
                }
            };
            Box::new(calc_fee_with_tax)
        }

        let calc_fee = || 12000.00;

        let f = tax(calc_fee);

        assert_eq!(f(), 12120.0);
    }
}
