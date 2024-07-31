use std::ops::{Range, RangeInclusive};

use rand::Rng;
use time::OffsetDateTime;

fn main() {
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
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
        println!(
            "borrowing by `ref s1, ref s2`, s1 = {s1}, s2 = {s2}, t = {:?}",
            t
        );
        // another form of borrowing
        let (s3, s4) = &t;
        println!("borrowing by `&t` s3 = {s3}, s4 = {s4}, t = {:?}", t);

        // by clone
        let (s5, s6) = t.clone();
        println!("by t.clone(), s5 = {s5}, s6 = {s6}, t = {:?}", t);
    }

    // if-let
    {
        let optional = Some(42);
        if let Some(number) = optional {
            assert_eq!(number, 42);
        }
    }

    // if return
    {
        let x = 42;
        let y = if x == 42 { 1 } else { 0 };
        assert_eq!(y, 1);
    }

    // loop return
    {
        let mut x = 0;
        let y = loop {
            x += 1;
            if x == 42 {
                break x;
            }
        };
        assert_eq!(y, 42);
    }

    // nested loop break
    {
        let mut x = 0;
        'outer: loop {
            loop {
                x += 1;
                if x == 42 {
                    break 'outer;
                }
            }
        }
        assert_eq!(x, 42);
    }

    // while loop
    {
        let mut x = 0;
        while x < 42 {
            x += 1;
        }
        assert_eq!(x, 42);
    }

    // for in range
    {
        let mut sum = 0;
        for i in 0..42 {
            sum += i;
        }
        assert_eq!(sum, 861);
    }

    // for in array
    {
        let a = [1, 2, 3, 4, 5];
        let mut sum = 0;
        for i in a.iter() {
            sum += i;
        }
        assert_eq!(sum, 15);
    }

    // borrow - only one mutable reference can exist at a time
    {
        let mut s = String::from("hello");
        {
            let s1 = &mut s;
            s1.push_str(" world");
            // s1 reference goes out of scope here
        }
        // s2 is the only mutable reference to s at this point
        let s2 = &mut s;
        s2.push_str("!");
        assert_eq!(s, "hello world!");
    }

    // print pointer address of reference
    {
        let s = String::from("hello");
        let s = &s;
        println!("s = {:p}", s);
        println!("s = {s:p}");
    }

    // dereference
    {
        let n = 42;
        let m = &n;
        assert_eq!(n, *m);
    }

    // two way of reference
    {
        let c = '中';
        let c1 = &c;
        let ref c2 = c;
        assert_eq!(*c1, *c2);
        assert_eq!(c1, c2); // c1, c2 are both references(pointers) to c
    }

    // &str string slice
    {
        let s: String = String::from("hello world");
        let hello: &str = &s[0..5]; // immutable &str
        let world: &str = &s[6..11]; // immutable &str
        assert_eq!(hello, "hello");
        assert_eq!(world, "world");
    }

    // str can be use in Box
    {
        let s: Box<str> = "hello world".into();
        let ss: &str = &(*s);
        assert_eq!(ss, "hello world");
    }

    // you can only concat String with &str
    {
        let s1: String = String::from("hello ");
        let s2: String = String::from("world!");
        let s3: String = s1 + &s2; // s1 ownership moved to s3, &String -> &str
        let s4: String = "hello ".to_string() + s2.as_str(); // cannot use s1 here
        assert_eq!(s3, s4);
    }

    // string byte escapes
    {
        let s = "I'm writing Ru\x73\x74!";
        println!("{}", s);
    }

    // string unicode points
    {
        let unicode_codepoint = "\u{211D}";
        let character_name = "\"Double-stroke capital R\"";
        println!(
            "Unicode point: {}, Character name: {}",
            unicode_codepoint, character_name
        );
    }

    // multiple lines string
    {
        let long_string = "String literals
                can span multiple lines.
                The linebreak and indentation here \
                can be escaped too!";
        println!("{}", long_string);
    }

    // raw string
    {
        let raw_str = r"I'm writing Ru\x73\x74!";
        println!("{}", raw_str);
    }

    // String -> &str by slice index
    {
        let s1 = String::from("hi,中国");
        let h1: &str = &s1[3..6];
        assert_eq!(h1, "中");
    }

    // iterating characters in a string
    {
        let s1 = String::from("hello, 世界");
        for c in s1.chars() {
            println!("{}", c);
        }
    }

    // array literal
    {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(std::mem::size_of_val(&arr), 5 * std::mem::size_of::<i32>());
        let arr: [i64; 5] = [1, 2, 3, 4, 5];
        assert_eq!(arr.len(), 5);
        assert_eq!(std::mem::size_of_val(&arr), 5 * std::mem::size_of::<i64>());
        assert_eq!(arr[3], 4);
    }

    // init array with same value
    {
        let arr: [i32; 5] = [0; 5];
        assert_eq!(arr, [0, 0, 0, 0, 0]);
    }

    // get array element by get
    {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let third: &i32 = arr.get(2).unwrap();
        assert_eq!(*third, 3);
    }

    // slice if type of &[T]
    {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let slice: &[i32] = &arr;
        assert_eq!(slice.len(), 5);
        assert_eq!(slice[3], 4);
        let first_three: &[i32] = &arr[..3];
        assert_eq!(first_three, [1, 2, 3]);
        // slice has ptr and len, size_of_val is 2 * size_of<usize>
        assert_eq!(
            std::mem::size_of_val(&first_three),
            2 * std::mem::size_of::<usize>()
        );
    }

    // &str is a slice of char
    {
        let s: String = String::from("hello, world!");
        let world: &str = &s[7..12];
        assert_eq!(world, "world");
    }

    // String index in bytes, a chinese character is 3 bytes
    {
        let s: String = String::from("你好，世界!");
        let world: &str = &s[9..15];
        assert_eq!(world, "世界");
    }

    // &String can be convert to &str implicitly
    {
        fn take_firt_word(s: &str) -> &str {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[..i];
                }
            }
            s
        }
        let s: String = String::from("hello world!");
        let first_word: &str = take_firt_word(&s);
        assert_eq!(first_word, "hello");
    }

    // tuple compound values of different type
    {
        let tt: (u8, (i16, &str)) = (42, (-42, "hello"));
        // acess tuple element by .index
        assert_eq!(tt.1 .1, "hello");
    }

    // tuple up to 12 elements can be print
    {
        let twelve_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, "twelve");
        println!("{:?}", twelve_tuple);
    }

    // tuple destructuring
    {
        let t = (1, 2, 3);
        let (.., z) = t;
        assert_eq!(z, 3);
    }

    // tuple as function parameter
    {
        fn sum_multiply(t: (i32, i32)) -> (i32, i32) {
            (t.0 + t.1, t.0 * t.1)
        }
        let t = (1, 2);
        assert_eq!(sum_multiply(t), (3, 2));
    }

    // struct is compund type with named fields with different types
    {
        struct User {
            name: String,
            age: u8,
            email: String,
        }

        fn new_user(name: String, age: u8, email: String) -> User {
            User { name, age, email } // shothand syntax for struct initialization
        }
        let member = User {
            name: String::from("Alice"),
            age: 30,
            email: String::from("alice@example.com"),
        };
        let member_new = new_user(String::from("Alice"), 30, String::from("alice@example.com"));
        assert_eq!(member.name, member_new.name);
        assert_eq!(member.age, member_new.age);
        assert_eq!(member.email, member_new.email);
    }

    // copy struct with spread syntax
    {
        struct User {
            name: String,
            age: u8,
            email: String,
        }

        let mut user1: User = User {
            name: String::from("Alice"),
            age: 30,
            email: String::from("alice@example.com"),
        };
        let user2: User = User { age: 31, ..user1 }; // clone user1 and update age

        // update user1
        user1.name = String::from("Bob");
        user1.email = String::from("bob@example.com");
        assert_eq!(user1.name, "Bob");
        assert_eq!(user1.age, 30);
        assert_eq!(user1.email, "bob@example.com");

        // user2 remain a clone of user1 before update
        assert_eq!(user2.name, "Alice");
        assert_eq!(user2.age, 31);
        assert_eq!(user2.email, "alice@example.com");
    }

    // init struct by field name shorthand
    {
        struct User {
            name: String,
            age: u8,
            email: String,
        }

        fn new_user(name: String, age: u8, email: String) -> User {
            User { name, age, email }
        }

        let user1: User = new_user(String::from("Alice"), 31, String::from("alice@example.com"));
        assert_eq!(
            std::mem::size_of_val(&user1),
            std::mem::size_of_val(&user1.name) // String 8bytes * 3
                + std::mem::size_of_val(&user1.age) // u8 1byte
                + 7 // alignment padding
                + std::mem::size_of_val(&user1.email) // String 8bytes * 3
        );
    }

    // print struct with debug format
    {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct User {
            name: String,
            age: u8,
            email: String,
        }

        let user: User = User {
            name: String::from("Alice"),
            age: 30,
            email: String::from("alice@example.com"),
        };
        dbg!(&user); // print to stderr with debug format
        println!("{:?}", &user);
    }

    // unit struct is a struct without fields
    {
        struct Unit;
        let unit = Unit;
        assert_eq!(std::mem::size_of_val(&unit), 0);
    }

    // tuple struct is a struct with unnamed fields
    {
        struct Color(u8, u8, u8);
        let black = Color(0, 0, 0);
        assert_eq!(black.0, 0);
        assert_eq!(black.1, 0);
        assert_eq!(black.2, 0);
    }

    // think of struct is a type definition keyword
    {
        struct User {
            name: String,
        }
        struct Point(i32, i32, i32);
        struct Empty;
        struct EmptyTuple();
        let alice: User = User {
            name: String::from("Alice"),
        };
        let origin: Point = Point(0, 0, 0);
        let empty: Empty = Empty;
        let empty_tuple: EmptyTuple = EmptyTuple();
        assert_eq!(std::mem::size_of_val(&alice), std::mem::size_of::<String>());
        assert_eq!(
            std::mem::size_of_val(&origin),
            3 * std::mem::size_of::<i32>()
        );
        assert_eq!(std::mem::size_of_val(&empty), 0);
        assert_eq!(std::mem::size_of_val(&empty_tuple), 0);

        assert_eq!(alice.name, "Alice");
        assert_eq!(origin.0 + origin.1 + origin.2, 0);
    }
}
