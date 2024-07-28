use std::ops::{Range, RangeInclusive};

use rand::Rng;
use time::OffsetDateTime;

fn main() {
    let greeting: &str = "Hello, world!";
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(0..100);
    let now = OffsetDateTime::now_local().expect("Failed to get local date time");
    // marcos, generated code by function arguments.
    println!(
        "{greeting} @{now} with {}",
        add(random_number, random_number)
    );

    // variable declaration shadowing
    {
        let number = "42";
        let number = number.parse::<i32>().expect("Failed to parse number");
        assert_eq!(number, 42);
    }

    // destructuring
    {
        let (mut x, y) = (3, 4);
        x += 1;
        assert_eq!(x, 4);
        assert_eq!(y, 4);
    }

    {
        let (x, y);
        (x, ..) = (1, 2, 3, 4);
        [.., y] = [1, 2, 3, 4];
        assert!((x, y) == (1, 4));
    }

    // integer types
    {
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

    // float types
    {
        let x: f32 = 42.0;
        let y: f64 = 42.0;
        assert_eq!(x, y as f32);
    }

    // literal type
    {
        let x = 42_f64;
        println!("type of 42_f64: {}", std::any::type_name_of_val(&x));
        let y = 42_u128;
        println!("type of 42_u128: {}", std::any::type_name_of_val(&y));
    }

    // const
    {
        assert_eq!(std::i8::MAX, 127);
        assert_eq!(std::i16::MAX, 32767);
        assert_eq!(std::i32::MAX, 2147483647);
        assert_eq!(std::u8::MAX, 255);
    }

    // decimal hex otcal binary
    {
        let v = 1_000_000 + 0xff + 0o77 + 0b1111_1111;
        println!("v = {v}")
    }

    // range
    {
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
    {
        assert_eq!(true && false, false);
        assert_eq!(true || false, true);
        assert_eq!(!false, true);
    }

    // bitwise operation
    {
        assert_eq!(0b1111_1111 & 0b0000_1111, 0b0000_1111);
        assert_eq!(0b1111_1111 ^ 0b0000_1111, 0b1111_0000);
        assert_eq!(0b1111_1111 | 0b0000_1111, 0b1111_1111);
        assert_eq!(0b0000_1111 >> 4, 0b0000);
        assert_eq!(0b0000_1111 << 4, 0b1111_0000);
    }

    // char
    {
        let c: char = 'a';
        println!("size_of_val('a') = {}", std::mem::size_of_val(&c));
        let c: char = '中';
        println!("size_of_val('中') = {}", std::mem::size_of_val(&c));
    }

    // if-statement
    {
        if 'a' != '中' {
            println!("'a' != '中'");
        }
    }

    // unit type
    {
        fn implicit_return_unit() {
            println!("I return a unit type implicitly.")
        }
        let x: () = ();
        assert_eq!(x, implicit_return_unit());
    }

    // block statement
    {
        let x: u32 = 42;
        let y = {
            let a: u32 = 6;
            let b: u32 = 7;
            a * b // statement without ending semicolon as value of y
        };
        assert_eq!(x, y);
    }

    // fn return statement
    {
        fn sum(a: i32, b: i32) -> i32 {
            a + b // statement without ending semicolon as return value
        }
        println!("sum(1, 2) = {}", sum(1, 2));
    }

    // fn never return
    {
        fn _never_return() -> ! {
            panic!()
        }

        fn _unimplemented() -> ! {
            unimplemented!()
        }

        fn _todo() -> ! {
            todo!()
        }
    }

    // match
    {
        let v = true;
        let _x = match v {
            true => 1,
            _ => {
                panic!("The value for matching is not true!");
            }
        };
    }

    // catch panic
    {
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
            Ok(_) => {},
            Err(err) => {
                if let Some(message) = err.downcast_ref::<&str>() {
                    println!("Caught a panic with message(&str): {message}");
                } else if let Some(message) = err.downcast_ref::<String>() {
                    println!("Caught a panic with message(String): {message}");
                } else {
                    println!("Caught a panic message: {:?}", err);
                }
            },
        }

        println!("Running after panic catching");
    }

    // ownership move by assignment
    {
        let s1 = String::from("hello");
        let s2 = s1; // move ownership
        // println!("s1 = {s1}"); // error: s1 moved to s2
        println!("s2 = {s2}");
    }

    // ownership move by function call
    {
        fn takes_ownership(s: String) {
            println!("s = {s}");
        }
        let s1 = String::from("hello");
        takes_ownership(s1);
        // println!("s1 = {s1}"); // error: s1 moved to takes_ownership

        fn gives_ownership() -> String {
            let s = String::from("hello");
            s
        }
        let s2 = gives_ownership();
        println!("s2 = {s2}");
    }

    // box reference
    {
        let mut x = Box::new(42);
        *x /= 6;
        println!("x = {}", x);
        println!("*x = {}", *x);
        println!("&x = {}", &x);
    }

    // partial move by assignment, partial borrow by reference
    {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: Box<u8>,
        }
        let person = Person {
            name: String::from("Alice"),
            age: Box::new(30),
        };

        println!("person = {:?}", person);

        let Person { name, ref age } = person;
        println!("name = {name}, age = {age}");

        // error: value borrowed here after move
        // println!("person = {:?}", person);

        // ok: name is moved, age is borrowed
        println!("person.age = {}", person.age);
    }

    // tuple index
    {
        let t = (String::from("hello"), String::from("world"));
        let _s: String = t.0;
        println!("t.1 = {}", t.1); // t.0 is moved to _s, t.1 is still owned by t
    }

    // borrowing
    {
        let t: (String, String) = (String::from("hello"), String::from("world"));
        // borrowing by preceeding `ref` or `ref mut`
        let (ref s1, ref s2) = t;
        println!("borrowing by `ref s1, ref s2`, s1 = {s1}, s2 = {s2}, t = {:?}", t);
        // another form of borrowing
        let (s3, s4) = &t;
        println!("borrowing by `&t` s3 = {s3}, s4 = {s4}, t = {:?}", t);

        // by clone
        let (s5, s6) = t.clone();
        println!("by t.clone(), s5 = {s5}, s6 = {s6}, t = {:?}", t);
    }
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
