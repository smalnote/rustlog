#[cfg(test)]
mod tests {
    use std::{
        num::NonZeroU16,
        ops::{Range, RangeInclusive},
    };

    use rustc_version::{Version, version_meta};

    // variable declaration shadowing
    #[test]
    fn variable_declaration_shadowing() {
        let number = "42";
        let number = number.parse::<i32>().expect("Failed to parse number");
        assert_eq!(number, 42);
    }

    // destructuring
    #[test]
    fn tuple_destructuring() {
        let (mut x, y) = (3, 4);
        x += 1;
        assert_eq!(x, 4);
        assert_eq!(y, 4);

        let (x, y);
        (x, ..) = (1, 2, 3, 4);
        [.., y] = [1, 2, 3, 4];
        let [first, .., last] = [1, 2, 3, 4];
        assert_eq!((x, y), (1, 4));
        assert_eq!((x, y), (first, last));
    }

    // tuple index
    #[test]
    fn tuple_index() {
        let t = (String::from("hello"), String::from("world"));
        let _s: String = t.0;
        println!("t.1 = {}", t.1); // t.0 is moved to _s, t.1 is still owned by t
    }

    // destruct struct tuple
    #[test]
    fn desruct_tuple_inside_struct() {
        struct Centimeters(f64);
        struct Inches(i32);
        impl Inches {
            fn to_centimeters(&self) -> Centimeters {
                let &Inches(inches) = self; // destruct struct tuple
                Centimeters(inches as f64 * 2.54)
            }
        }

        let five_inches = Inches(5);
        let cm = five_inches.to_centimeters();
        let Centimeters(cm) = cm; // destruct struct tuple and shadow variable cm
        assert_eq!(cm, 2.54 * 5.0);
    }

    // integer types
    #[test]
    fn integer_types() {
        let x: i8 = 42;
        let y: i16 = 42;
        let z: i32 = 42;
        let p: i64 = 42;
        let q: i128 = 42;
        let r: isize = 42; // architecture dependent 32bit or 64bit
        assert_eq!(x, y as i8);
        assert_eq!(y, z as i16);
        assert_eq!(z, p as i32);
        assert_eq!(p, q as i64);
        assert_eq!(r, q as isize);
        println!("bytes of isize: {}", std::mem::size_of_val(&r));

        let l: u64 = 42;
        let m: u128 = 42;
        let n: usize = 42;
        assert_eq!(l, m as u64);
        assert_eq!(m, n as u128);
        assert_eq!(n, l as usize);
        println!("bytes of usize: {}", std::mem::size_of_val(&n));
    }

    // char is backed by u8
    #[test]
    fn char_u8() {
        let a: u8 = b'a';
        assert_eq!(a, 97);
    }

    // float types
    #[test]
    fn float_types() {
        let x: f32 = 42.0;
        let y: f64 = 42.0;
        assert_eq!(x, y as f32);
    }

    // literal type
    #[test]
    fn literal_type() {
        let x = 42_f64;
        assert_eq!(std::any::type_name_of_val(&x), "f64");
        let y = 42_u128;
        assert_eq!(std::any::type_name_of_val(&y), "u128");
    }

    // type suffix in Option
    #[test]
    fn integer_type_suffix() {
        let n = Some(42_u32);

        if let Some(n) = n {
            assert_eq!(n, 42);
        }
    }

    #[test]
    fn test_while_let_pattern_match() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut current = 3;
        while let Some(value) = stack.pop() {
            assert_eq!(value, current);
            current -= 1;
        }

        assert_eq!(current, 0);
    }

    // const
    #[test]
    fn predefined_integer_constants() {
        assert_eq!(i8::MAX, 127);
        assert_eq!(i16::MAX, 32767);
        assert_eq!(i32::MAX, 2147483647);
        assert_eq!(u8::MAX, 255);
    }

    // decimal hex octal binary
    #[test]
    fn integer_form_decimal_hex_otcal_binary() {
        let v = 1_000_000 + 0xff + 0o77 + 0b1111_1111;
        println!("v = {v}")
    }

    // range
    #[test]
    fn range_syntax_sugar_and_built_in_type() {
        let mut sum: i32 = 0;
        for i in -128..=128 {
            sum += i;
        }
        println!("sum of range -128..=128 is {sum}");

        let mut s = String::new();
        for c in 'a'..='z' {
            s.push(c);
        }
        println!("s = {s}");

        assert_eq!(
            (-128..128),
            Range {
                start: -128,
                end: 128
            }
        );
        assert_eq!(('a'..='z'), RangeInclusive::new('a', 'z'));
    }

    // boolean logic
    #[test]
    #[allow(clippy::nonminimal_bool, clippy::bool_assert_comparison)]
    fn boolean_logical_calculation() {
        assert_eq!(true && false, false);
        assert_eq!(true || false, true);
        assert_eq!(!false, true);
    }

    // bitwise operation
    #[test]
    fn bitwise_operation() {
        assert_eq!(0b1111_1111 & 0b0000_1111, 0b0000_1111);
        assert_eq!(0b1111_1111 ^ 0b0000_1111, 0b1111_0000);
        assert_eq!(0b1111_1111 | 0b0000_1111, 0b1111_1111);
        assert_eq!(0b0000_1111 >> 4, 0b0000);
        assert_eq!(0b0000_1111 << 4, 0b1111_0000);
    }

    // char 4-bytes for UTF-16
    #[test]
    fn char_bytes() {
        let c: char = 'a';
        println!("size_of_val('a') = {}", std::mem::size_of_val(&c));
        assert_eq!(std::mem::size_of_val(&c), 4);
        let c: char = '中';
        println!("size_of_val('中') = {}", std::mem::size_of_val(&c));
        assert_eq!(std::mem::size_of_val(&c), 4);
        println!("size_of<char> = {}", std::mem::size_of::<char>());
        assert_eq!(std::mem::size_of::<char>(), 4);
    }

    // char in String is dynamically-sized of UTF-8(1~4 bytes)
    #[test]
    fn char_bytes_of_string() {
        let hello: &str = "hello, 世界!";
        for c in hello.chars() {
            println!(
                "char string: {c}, length in bytes, c.to_string().len() = {}",
                c.to_string().len(),
            );
        }
    }

    // if-statement
    #[test]
    fn if_statement() {
        if 'a' != '中' {
            println!("'a' != '中'");
        }
    }

    // unit type
    #[test]
    #[allow(clippy::unit_cmp, clippy::unused_unit)]
    fn unit_type_for_empty() {
        fn implicitly_return_unit() {
            println!("I return a unit type implicitly.")
        }
        let x: () = ();
        assert_eq!(x, implicitly_return_unit());

        fn explicitly_return_unit() -> () {}
        assert_eq!(x, explicitly_return_unit());
    }

    // block statement
    #[test]
    fn block_statement_with_return_value() {
        let x: u32 = 42;
        let y = {
            let a: u32 = 6;
            let b: u32 = 7;
            a * b // statement without ending semicolon as value of y
        };
        assert_eq!(x, y);
    }

    // fn return statement
    #[test]
    fn statement_without_semicolon_for_return() {
        fn sum(a: i32, b: i32) -> i32 {
            a + b // statement without ending semicolon as return value
        }
        println!("sum(1, 2) = {}", sum(1, 2));
    }

    // fn never return
    #[test]
    fn function_never_return() {
        fn _never_return() -> ! {
            panic!()
        }

        fn _unimplemented() -> ! {
            unimplemented!()
        }

        fn _todo() -> ! {
            todo!()
        }

        fn _never_return_without_mark() {
            panic!()
        }
    }

    // match
    #[test]
    fn match_boolean_and_return_value() {
        let v = true;
        let _x = match v {
            true => 1,
            _ => {
                panic!("The value for matching is not true!");
            }
        };
    }

    // catch panic
    #[test]
    fn catch_panic() {
        let result = std::panic::catch_unwind(|| {
            let v = false;
            let _x = match v {
                true => 1,
                _ => {
                    panic!("The value for matching is not true!");
                }
            };
        });

        match result {
            Ok(_) => {}
            Err(err) => {
                if let Some(message) = err.downcast_ref::<&str>() {
                    println!("Caught a panic with message(&str): {message}");
                } else if let Some(message) = err.downcast_ref::<String>() {
                    println!("Caught a panic with message(String): {message}");
                } else {
                    println!("Caught a panic message: {:?}", err);
                }
            }
        }

        println!("Running after panic catching");
    }

    #[test]
    fn test_non_zero_numeric_types() {
        let magic_number = unsafe { NonZeroU16::new_unchecked(42) };
        assert_eq!(magic_number.trailing_zeros(), 1);
    }

    #[test]
    fn test_let_chains() {
        if let Ok(version) = version_meta()
            && let Version { major, minor, .. } = version.semver
            && major == 1
            && minor >= 88
        {
            println!("`let_chains` was stabilized in this version")
        }
    }
}
