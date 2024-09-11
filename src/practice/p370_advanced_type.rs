#[cfg(test)]
mod tests {
    use std::{fmt, result, thread};

    /// Newtype of unit wrapper
    #[test]
    fn test_unit_wrapper_as_newtype() {
        struct Kilometers(f64);

        impl fmt::Display for Kilometers {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}km", self.0)
            }
        }

        let five_km = Kilometers(5.25);
        assert_eq!(five_km.to_string(), "5.25km");
    }

    /// Type alias for convenient and consistency
    #[test]
    fn test_type_alias() {
        type Thunk = Box<dyn Fn() + Send + 'static>;
        type Result<T> = result::Result<T, &'static str>;

        let magic_number = 42;
        let f: Thunk = Box::new(move || println!("{}", magic_number));

        fn run_thunk(f: Thunk) -> Result<()> {
            thread::spawn(f).join().map_err(|_| "Something went wrong")
        }

        if let Err(msg) = run_thunk(f) {
            eprintln!("{}", msg);
        }
    }

    #[test]
    fn test_never_type() {
        let values = vec![None, None, Some(1), None, Some(2)];

        for value in values {
            let value = match value {
                Some(value) => value,
                None => continue, // continue has a `!` value
            };
            println!("{}", value);
        }
    }

    /// The golden rule of dynamically sized types is that we must always put
    /// values of dynamically sized types behind a pointer of some kind.
    ///
    /// Well-known wrapper of DSTs(dynamically sized types or unsized types)
    /// &str: pointer and length, Box<str> or Rc<str> would work too
    /// &dyn Trait: pointer and type
    /// Box<dyn Trait>: pointer and type
    /// Rc<dyn Trait>: reference and pointer
    ///
    /// To work with DSTs, Rust provides the `Sized` trait to determine whether
    /// or not a type's size is know at compile time. This trait is automatically
    /// implemented for everything whose size is known at compile time.
    /// In addition, Rust implicitly adds a bound on `Sized` to every generic
    /// function.
    /// ```
    /// fn generic<T>(t: T) {}
    /// ```
    /// is actually treated as
    /// ```
    /// fn generic<T: Sized>(t: T) {}
    /// ```
    /// We can use `?Sized` to relax this restriction
    /// ```
    /// fn generic<T: ?Sized>(t: &T) {}
    /// ```
    /// The `?Trait` syntax with this meaning is only available for `Sized`, not other traits.
    #[test]
    fn test_dynamic_size_type() {
        fn print_unsized<T: ?Sized + fmt::Display>(t: &T) {
            println!("{}", t);
        }

        let s = "hello";
        print_unsized(s); // str is DST, str: ?Sized, &str: &T

        let hello = "hello".to_string();
        let displays: Vec<&dyn fmt::Display> = vec![&1, &false, &"str", &hello];

        for value in displays {
            print_unsized(value);
        }

        let displays: Vec<Box<dyn fmt::Display>> = vec![
            Box::new(42),
            Box::new(true),
            Box::new("str"),
            Box::new("hello".to_string()),
        ];

        for value in displays {
            print_unsized(value.as_ref());
        }
    }
}
