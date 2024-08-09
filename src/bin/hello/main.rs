use std::ops::{Range, RangeInclusive};

use rand::Rng;
use std::fmt::Debug;
use time::OffsetDateTime;
use utf8_slice;

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

    // type suffix in Option
    {
        let n = Some(42_u32);

        if let Some(n) = n {
            assert_eq!(n, 42);
        }
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

    // char 4-bytes for UTF-16
    {
        let c: char = 'a';
        println!("size_of_val('a') = {}", std::mem::size_of_val(&c));
        let c: char = '中';
        println!("size_of_val('中') = {}", std::mem::size_of_val(&c));
        println!("size_of<char> = {}", std::mem::size_of::<char>());
    }

    // char in String is dynamically-sized of UTF-8(1-4 bytes)
    {
        let hello: &str = "hello, 世界!";
        for c in hello.chars() {
            println!(
                "char string: {c}, c.to_string().as_bytes().len() = {}",
                c.to_string().as_bytes().len(),
            );
        }
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

        fn _never_return_wo_mark() {
            panic!()
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
        let s1: String = String::from("hello");
        let s2: String = s1; // memory area of s1 move ownership from s1 to s2
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
        let mut x: Box<i32> = Box::new(42);
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
        let optional: Option<i32> = Some(42);
        if let Some(number) = optional {
            assert_eq!(number, 42);
        }
    }

    // if return
    {
        let x: i32 = 42;
        let y: i32 = if x == 42 { 1 } else { 0 };
        assert_eq!(y, 1);
    }

    // loop return
    {
        let mut x: i32 = 0;
        let y: i32 = loop {
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
        let mut sum: i32 = 0;
        for i in 0..42 {
            sum += i;
        }
        assert_eq!(sum, 861);
    }

    // for in array
    {
        let a: [i32; 5] = [1, 2, 3, 4, 5];
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

    // cannot borrow mutable value from immutable value
    /*
    {
        let v = 42; // immutable value
        let w = &mut v; // invalid mutable borrowing
        assert_eq!(v, *w);
    }
    */

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
        let c: char = '中';
        let c1: &char = &c;
        let ref c2 = c;
        assert_eq!(*c1, *c2);
        assert_eq!(c1, c2); // c1, c2 are both references(pointers) to c
    }

    // use ref for pattern matching
    {
        let maybe_name: Option<String> = Some(String::from("alice"));
        match maybe_name {
            // ref here just borrow the value, not move ownership
            // it is not matched against Some(String), but Some(&String)
            Some(ref name) => {
                assert_eq!(name, "alice");
            }
            _ => {}
        }
        // maybe_name still available here
        assert_eq!(maybe_name.unwrap(), "alice");
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
        let s1: String = String::from("hello, 世界");
        for c in s1.chars() {
            println!("{}", c);
        }
    }

    // index string chars with utf8_slice::slice()
    {
        let s = "The 🚀 goes to the 🌑!";
        let rocket = utf8_slice::slice(s, 4, 5);
        assert_eq!(rocket, "🚀");
    }

    // String is a Vec<u8> that guarantee to be valid utf-8 sqeuences
    {
        let mut s = String::new();
        s.push_str("hello");
        let v: Vec<u8> = vec![104, 101, 108, 108, 111];
        let v = String::from_utf8(v).expect("invalid utf-8 sequences");
        assert_eq!(s, v);
    }

    // String is made up of three components: pointer to heap memory, length in bytes of content, capacity in bytes of memory
    // A String's capacity grows automatically to acommodate its length.
    {
        let mut s = String::with_capacity(5);

        println!("s = {}, capacity = {}", &s, s.capacity());
        for _ in 0..2 {
            s.push_str("hello");
            println!("s = {}, capacity = {}", &s, s.capacity());
        }
    }

    // Manually manage String memory and rebuild String from pointer, lenght and capacity.
    {
        let story = "The 🚀 goes to the 🌑!".to_string();

        let mut s = std::mem::ManuallyDrop::new(story);

        let ptr = s.as_mut_ptr();
        let len = s.len();
        let cap = s.capacity();

        let raw_story = unsafe { String::from_raw_parts(ptr, len, cap) };

        assert_eq!(*s, raw_story);
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
        let arr: [i32; 5] = [0, 1, 2, 3, 4];
        let slice: &[i32] = &arr;
        assert_eq!(slice.len(), 5);
        assert_eq!(slice[3], 3);
        let first_three: &[i32] = &arr[..3];
        assert_eq!(first_three, [0, 1, 2]);
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

        // in bytes index
        let world: &str = &s[9..15];
        assert_eq!(world, "世界");

        // chars() iterator index
        let substring: String = s.chars().skip(3).take(2).collect::<String>();
        assert_eq!(substring, "世界");
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
        let t: (i32, i32, i32) = (1, 2, 3);
        let (.., z) = t;
        assert_eq!(z, 3);
    }

    // tuple as function parameter
    {
        fn sum_multiply(t: (i32, i32)) -> (i32, i32) {
            (t.0 + t.1, t.0 * t.1)
        }
        let t: (i32, i32) = (1, 2);
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

        let user1: User = User {
            name: String::from("Alice"),
            age: 30,
            email: String::from("alice@example.com"),
        };
        let user2: User = User { age: 31, ..user1 }; // user1.name and user1.email are moved

        // update user1
        // error: value borrowed here after move
        // assert_eq!(user1.name, "Alice");
        assert_eq!(user1.age, 30);
        // error: value borrowed here after move
        // assert_eq!(user1.email, "alice@example.com");

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

    // enum with additional tuple
    {
        #[derive(Debug)]
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let addrs: [IpAddr; 2] = [
            IpAddr::V4(String::from("127.0.0.1")),
            IpAddr::V6(String::from("::1")),
        ];

        for addr in addrs.iter() {
            match addr {
                IpAddr::V4(ip) => println!("IPv4 address: {ip}"),
                IpAddr::V6(ip) => println!("IPv6 address: {ip}"),
            }
        }
    }

    // enum with default integer start from 0
    {
        #[derive(Debug)]
        enum Color {
            Red,   // 0
            Green, // 1
            Blue,  // 2
        }

        let colors = [Color::Red, Color::Green, Color::Blue];
        for color in colors.iter() {
            match color {
                Color::Red => println!("Red"),
                Color::Green => println!("Green"),
                Color::Blue => println!("Blue"),
            }
        }
    }

    // enum with explicit integer value
    {
        #[derive(Debug)]
        enum Color {
            Red = 0xff0000,
            Green = 0x00ff00,
            Blue = 0x0000ff,
        }

        let colors: [Color; 3] = [Color::Red, Color::Green, Color::Blue];
        for color in colors {
            match color {
                Color::Red => println!("Red"),
                Color::Green => println!("Green"),
                Color::Blue => println!("Blue"),
            }
        }
    }

    // enum can be convert to integer by `as`
    {
        #[derive(Debug)]
        enum Color {
            Red = 0xff0000,
            Green = 0x00ff00,
            Blue = 0x0000ff,
        }

        let red: u32 = Color::Red as u32;
        let green: u32 = Color::Green as u32;
        let blue: u32 = Color::Blue as u32;
        assert_eq!(red, 0xff0000);
        assert_eq!(green, 0x00ff00);
        assert_eq!(blue, 0x0000ff);
    }

    // enum hold different types
    {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let messages: [Message; 4] = [
            Message::Quit,
            Message::Move { x: 1, y: 2 },
            Message::Write(String::from("hello")),
            Message::ChangeColor(0xff, 0xff, 0xff),
        ];

        for message in messages {
            match message {
                Message::Quit => println!("Quit"),
                Message::Move { x, y } => println!("Move to ({x}, {y})"),
                Message::Write(s) => println!("Write {s}"),
                Message::ChangeColor(r, g, b) => println!("Change color to ({r}, {g}, {b})"),
            }
        }
    }

    // extract enum with if-let
    {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let messages: [Message; 4] = [
            Message::Quit,
            Message::Move { x: 1, y: 2 },
            Message::Write(String::from("hello")),
            Message::ChangeColor(0xff, 0xff, 0xff),
        ];

        for message in messages {
            if let Message::Write(s) = message {
                println!("Write {s}");
            } else if let Message::Quit = message {
                println!("Quit");
            } else if let Message::Move { x, y } = message {
                println!("Move to ({x}, {y})");
            } else if let Message::ChangeColor(r, g, b) = message {
                println!("Change color to ({r}, {g}, {b})");
            }
        }
    }

    // Option<T> is an enum donates for nullable value
    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }
        let five: Option<i32> = Some(5);
        let six: Option<i32> = plus_one(five);
        let none: Option<i32> = plus_one(None);
        assert_eq!(six, Some(6));
        assert_eq!(none, None);

        if let Some(i) = six {
            assert_eq!(i, 6);
        }
    }

    // implement a linked-list via enums
    {
        #[derive(Debug)]
        #[allow(dead_code)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        println!("{:?}", list);
    }

    // if/else expression in assignment
    {
        let n: i32 = 5;
        let big_n: i32 = if -10 < n && n < 10 {
            println!("{} is a small number", n);
            10 * n
        } else {
            println!("{} is a big number", n);
            n / 2
        };
        println!("big_n = {}", big_n);
    }

    // for-in move ownership by default
    {
        let names: [String; 2] = [String::from("Alice"), String::from("Bob")];
        for name in &names {
            // borrow names
            println!("{}", name);
        }
        println!("{:?}", names);
    }

    // for-in get copy of array of primary types
    {
        let numbers: [i32; 3] = [1, 2, 3];
        for number in numbers {
            // copy numbers
            println!("{}", number);
        }
        println!("{:?}", numbers);
    }

    // for-in iterate with index
    {
        let names: [String; 3] = [
            String::from("Alice"),
            String::from("Bob"),
            String::from("Charlie"),
        ];

        // iter().enumerate() borrow names
        for (i, name) in names.iter().enumerate() {
            println!("{}: {}", i, name);
        }

        println!("{:?}", names);
    }

    // loop with break and return value
    {
        let numbers: [i32; 8] = [1, 2, 3, 4, 5, 15, 21, 31];
        let mut i = 0;
        let key: Option<i32> = loop {
            if i >= numbers.len() {
                break Option::None;
            }
            let number = numbers[i];
            if number % 7 == 0 {
                break Some(number);
            }
            i += 1;
        };

        println!("key = {:?}", key);
    }

    // labeled loop with 'name
    {
        let p: &str = "Hello, I'm \"Alice\", and he is \"Bob\".";
        let bytes: &[u8] = p.as_bytes();
        let mut i = 0;
        'outer: loop {
            if i >= p.len() {
                break 'outer;
            }
            if bytes[i] == b'\"' {
                let start = i;
                i += 1;
                if i >= p.len() {
                    break 'outer;
                }
                'inner: loop {
                    if bytes[i] == b'\"' {
                        let end = i;
                        println!("Quoted string: {}", &p[start..=end]);
                        break 'inner;
                    }
                    i += 1;
                    if i >= p.len() {
                        break 'outer;
                    }
                }
                i += 1;
            } else {
                i += 1;
            }
        }
    }

    // match with wildcard
    {
        let x: i32 = 42;
        match x {
            0 => println!("zero"),
            1 => println!("one"),
            _ => println!("other"),
        }
    }

    // match with returned value
    {
        let x: i32 = 42;
        let y = match x {
            0 => "zero",
            1 => "one",
            _ => "other",
        };
        assert_eq!(y, "other");
    }

    // rename matched value
    {
        let x: i32 = 42;
        match x {
            0 => println!("zero"),
            1 => println!("one"),
            n => println!("other: {n}"),
        }

        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let messages: [Message; 4] = [
            Message::Quit,
            Message::Move { x: 1, y: 2 },
            Message::Write(String::from("hello")),
            Message::ChangeColor(0xff, 0xff, 0xff),
        ];

        for message in messages {
            match message {
                Message::Quit => println!("Quit"),
                Message::Move { x: a, y: b } => println!("Move to ({a}, {b})"), // rename x to a, y to b
                Message::Write(s) => println!("Write {s}"),
                Message::ChangeColor(r, g, b) => println!("Change color to ({r}, {g}, {b})"),
            }
        }
    }

    // matches! macro
    {
        let alphabets = ['a', 'b', 'c', 'd', 'e', '0', 'A', 'L'];

        // for-in [char; 8] array is copy of primary type
        for c in alphabets {
            assert!(matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'));
            if matches!(c, 'a'..='z') {
                println!("lowercase: {c}");
            } else if matches!(c, 'A'..='Z') {
                println!("uppercase: {c}");
            } else {
                println!("other: {c}");
            }
        }

        // alphabet is still available here
        println!("{:?}", alphabets);
    }

    // matches! for match enum
    {
        enum MyEnum {
            Foo,
            Bar,
        }
        let v: Vec<MyEnum> = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
        for e in v {
            // e == MyEnum::Foo requires MyEnum implementing PartialEq
            // if e == MyEnum::Foo {
            // using matches! macro
            if matches!(e, MyEnum::Foo) {
                println!("Foo");
            } else if matches!(e, MyEnum::Bar) {
                println!("Bar");
            }
        }
    }

    // enum with arbitrary u32
    {
        enum Foo {
            Bar,
            Baz,
            Qux(u32),
        }

        let foos = [Foo::Bar, Foo::Baz, Foo::Qux(42)];
        for foo in foos.iter() {
            match foo {
                Foo::Bar => println!("Bar"), // Foo::Bar as u8 is invalid here
                Foo::Baz => println!("Baz"), // Foo::Baz as u8 is invalid here
                Foo::Qux(n) => println!("Qux({n})"),
            }
        }
    }

    // pattern, use | to match serval values
    {
        for i in 0..3 {
            match i {
                0 | 1 => println!("zero or one"),
                _ => println!("other"),
            }
        }
    }

    // pattern, use ..= to match inclusive range
    {
        for i in 0..3 {
            match i {
                0..=1 => println!("zero or one"),
                _ => println!("other"),
            }
        }
    }

    // pattern, use @ to create a variable that holds a value
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 10 };
        match p {
            Point { x, y: 0 } => println!("Ont the x axis at x = {x}"),
            Point {
                x: newx @ 0..=5,
                y: y @ (10 | 20 | 30),
            } => println!("On the y axis at y = {y}, x = {newx}"),
            Point { x, y } => println!("x = {x}, y = {y}"),
        }
    }

    // pattern, use .. to match any value
    {
        let x = 1;
        match x {
            1.. => println!("one or more"),
            _ => println!("other"),
        }
    }

    // pattern match enum struct
    {
        enum Message {
            Hello { id: i32 },
        }

        let msg: Message = Message::Hello { id: 5 };
        match msg {
            Message::Hello { id: id @ 3..=7 } => println!("Found and id in range [3, 7]: {id}"),
            Message::Hello {
                id: newid @ (42 | 53 | 67),
            } => println!("Found specific id {newid}"),
            Message::Hello { id } => println!("Found some other id: {id}"),
        }
    }

    // pattern match with if statement
    {
        let num: Option<i32> = Some(4);
        let split: i32 = 5;
        match num {
            Some(x) if x < split => println!("x is less than {split}"),
            Some(_) => println!("x is greater than or equal to {split}"),
            None => {}
        }
    }

    // pattern match, using .. to ignore remaing parts of the value
    {
        let numbers = (2, 4, 6, 8);
        match numbers {
            (2, .., last) => println!("first = 2, last = {last}"),
            (first @ (..2 | 3..), ..) => println!("first = {first}"),
        }
    }

    // struct method and associated function
    {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        // impl block defines type struct methods and associated functions
        impl Rectangle {
            // associated function, no parameter `self`, reference to type struct
            pub fn new(width: u32, height: u32) -> Rectangle {
                Rectangle { width, height }
            }

            // use `Self` as synoym of struct
            fn square(size: u32) -> Self {
                Self {
                    width: size,
                    height: size,
                }
            }

            // method, consume parameter `&self` to borrow type struct instance
            pub fn area(&self) -> u32 {
                self.width * self.height
            }

            // paramter `self` move ownership of instance
            fn clear(self) {}

            // paramter `&mut self` borrow mutable instance
            fn rotate(&mut self) {
                let w: u32 = self.width;
                self.width = self.height;
                self.height = w;
            }

            // self must be the first paramter
            fn zoom(&mut self, ratio: f32) {
                self.width = (self.width as f32 * ratio) as u32;
                self.height = (self.height as f32 * ratio) as u32;
            }

            // canonical form of self paramter
            fn normalize(self: &mut Self) {
                if self.width > self.height {
                    self.height = self.width
                } else {
                    self.width = self.height
                }
            }
        }

        let mut rect = Rectangle::new(30_u32, 50_u32);
        assert_eq!(rect.area(), 1500);

        rect.rotate();
        assert_eq!(rect.width, 50);
        assert_eq!(rect.height, 30);

        rect.zoom(2.2);
        assert_eq!(rect.width, 110);
        assert_eq!(rect.height, 66);

        rect.normalize();
        assert_eq!(rect.width, 110);
        assert_eq!(rect.height, 110);

        rect.clear(); // value ownership moved

        let square = Rectangle::square(10);
        assert_eq!(square.area(), 100);
    }

    // can have multiple impl block for a same type
    {
        struct TrafficLight {
            color: String,
        }

        impl TrafficLight {
            pub fn new() -> Self {
                Self {
                    color: "red".to_string(),
                }
            }
        }

        impl TrafficLight {
            pub fn get_state(&self) -> &str {
                self.color.as_str()
            }
        }

        let light = TrafficLight::new();
        assert_eq!(light.get_state(), "red")
    }

    // implement method and assocaited functions for type enum
    #[allow(dead_code)]
    {
        enum TrafficLight {
            Red,
            Green,
            Yellow,
        }

        impl TrafficLight {
            pub fn color(&self) -> &str {
                match self {
                    TrafficLight::Green => "green",
                    TrafficLight::Red => "red",
                    Self::Yellow => "yellow", // can also use `Self` to reference type to implement
                }
            }
        }

        let c = TrafficLight::Yellow;
        assert_eq!(c.color(), "yellow");
    }

    // generic function
    {
        struct A; // type A is a unit type
        struct S(A); // type S is a unary tuple of A, `(A)`
        struct SGen<T>(T); // type SGen is a unary tuple of abitary type

        fn reg_fn(_s: S) {}
        fn gen_spec_t(_s: SGen<A>) {}
        fn gen_spec_i32(_s: SGen<i32>) {}
        fn generic<T>(_s: SGen<T>) {}

        reg_fn(S(A));
        gen_spec_t(SGen(A));
        gen_spec_i32(SGen(42_i32));

        // specify type parameter explicitly by using ::<T> of generic function
        generic::<char>(SGen('A'));

        // specify type parameter implicitly by passing concrete type argument
        generic(SGen('C'));

        // specify struct type parameter by using ::<T> of generic struct
        generic(SGen::<u32>(42));
    }

    // generic with trait bound
    {
        fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
            a + b
        }

        assert_eq!(2, sum(1_i8, 1_i8));
        assert_eq!(49, sum(42, 7));
        assert_eq!(3.14, sum(3.0, 0.14));
    }

    // generic struct
    {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer: Point<i32> = Point { x: 5, y: 10 };
        let float: Point<f64> = Point { x: 1.0, y: 4.0 };

        println!("integer point = {integer:?}");
        println!("float point = {float:?}");
    }

    // generic struct with two type parameter
    #[allow(dead_code)]
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let _p = Point {
            x: 32,
            y: "hello".to_string(),
        };
    }

    // generic struct with method return value reference
    {
        struct Val<T> {
            val: T,
        }

        impl<T> Val<T> {
            fn value(&self) -> &T {
                &self.val
            }
        }

        let i = Val { val: 42 };
        let s = Val {
            val: "hello".to_string(),
        };

        assert_eq!(*i.value(), 42);
        assert_eq!(s.value(), "hello");
    }

    // generic struct with method that has generic type parameters
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 1, y: 2 };
        let p2 = Point {
            x: "hello",
            y: '中',
        };

        let p3 = p1.mixup(p2);
        assert_eq!(p3.x, 1);
        assert_eq!(p3.y, '中');
    }

    // generic struct with specified type implementation
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl Point<f64> {
            fn distance_from_origin(&self) -> f64 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let p = Point { x: 3.0, y: 4.0 };
        assert_eq!(p.distance_from_origin(), 5.0);
    }

    // array element type and length is part of array type
    {
        #[allow(dead_code)]
        struct Array<T, const N: usize> {
            data: [T; N],
        }

        let _a: Array<i32, 3> = Array { data: [1, 2, 3] };
        let _b = Array::<u32, 3> { data: [1, 2, 3] };
        let _c = Array::<u32, 10> {
            data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
        };

        let _arrays: [Array<i32, 3>; 3] = [
            Array { data: [1, 2, 3] },
            Array { data: [1, 2, 3] },
            Array { data: [1, 2, 3] },
        ];
    }

    // trait, trait as parameter type, trait bound
    {
        trait Animal {
            fn sound(&self) -> String;
        }

        struct Sheep;
        struct Cow;

        impl Animal for Sheep {
            fn sound(&self) -> String {
                "Maah".to_string()
            }
        }

        impl Animal for Cow {
            fn sound(&self) -> String {
                "Mooh".to_string()
            }
        }

        fn bark(animal: &impl Animal) {
            println!("barking: {}", animal.sound());
        }

        bark(&Sheep);

        fn bark_both<T: Animal>(a: &T, b: &T) {
            println!("barking both: {}, {}", a.sound(), b.sound());
        }

        bark_both(&Cow, &Cow);
    }

    /*
     * Derivable Traits
     * Trait taht can be automatically implemented for a struct or an enum by the Rust compiler
     * Called "derivable" because they can be derived automatically
     * Most common derivable traits:
     *   - Debug: Allowing to output content via "{:?}"
     *   - Clone: Enables type to be duplicated with "clone()" method
     *   - Copy: Enables type to be copied implicity, without requiring explicit "clone()" method
     *   - PartialEq: Enables comparison
     */
    {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let p = Point { x: 3.1, y: 4.1 };
        let q = p.clone();
        assert_eq!(p, q);

        fn use_clone_eq<T: Clone + PartialEq + std::fmt::Debug>(a: &T, b: &T) {
            assert_eq!(a.clone(), b.clone())
        }
        use_clone_eq(&p, &q);
    }

    // where clauses for heavy use of trait bounds
    {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct Point<T> {
            x: T,
            y: T,
        }

        fn use_clone_eq<T, U>(a: &T, b: &U)
        where
            T: Clone + PartialEq + Debug,
            U: Clone + Debug,
        {
            assert_eq!(*a, a.clone());
            println!("b = {:?}", b.clone());
        }

        let p = Point { x: 'a', y: 'b' };
        let q = Point {
            x: "hello",
            y: "world",
        };
        use_clone_eq(&p, &q);
    }

    // trait as return type
    {
        trait Animal {}

        struct Cat;
        struct Dog;

        impl Animal for Cat {}
        impl Animal for Dog {}

        fn return_dog() -> impl Animal {
            Dog
        }

        fn return_cat() -> impl Animal {
            Cat
        }

        let _dog = return_dog();
        let _cat = return_cat();
    }

    // trait with default method
    {
        trait Hello {
            fn say_hello(&self) -> String {
                "hello".to_string()
            }

            fn say_something(&self) -> String;
        }

        struct Student;
        struct Teacher;

        impl Hello for Student {
            fn say_something(&self) -> String {
                "I'm a bad student!".to_string()
            }
        }

        impl Hello for Teacher {
            fn say_hello(&self) -> String {
                "greeting".to_string()
            }

            fn say_something(&self) -> String {
                "I'm not a teacher!".to_string()
            }
        }

        let s = Student;
        assert_eq!(s.say_hello(), "hello");
        assert_eq!(s.say_something(), "I'm a bad student!");

        let t = Teacher;
        assert_eq!(t.say_hello(), "greeting");
        assert_eq!(t.say_something(), "I'm not a teacher!");
    }

    // destruct struct tuple
    {
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

    // derive PartialEq, PartialOrd
    {
        #[derive(Debug, PartialEq, PartialOrd)]
        struct Seconds(u32);

        assert_eq!(Seconds(10), Seconds(10));
        assert!(Seconds(10) == Seconds(10));
        assert!(Seconds(20) > Seconds(10));
    }

    // operator trait
    {
        #[derive(Debug, PartialEq)]
        struct Centimeters(f64);
        #[derive(Debug, PartialEq)]
        struct SquareCentimeters(f64);

        impl std::ops::Mul for Centimeters {
            type Output = SquareCentimeters;
            fn mul(self, rhs: Self) -> SquareCentimeters {
                let Centimeters(lhs) = self;
                let Centimeters(rhs) = rhs;
                SquareCentimeters(lhs * rhs)
            }
        }

        assert_eq!(Centimeters(5.0) * Centimeters(6.0), SquareCentimeters(30.0));
    }

    // operator trait with different type of right hand side
    {
        struct Foo;
        struct Bar;

        #[derive(Debug, PartialEq)]
        struct FooBar;
        #[derive(Debug, PartialEq)]
        struct BarFoo;

        impl std::ops::Add<Bar> for Foo {
            type Output = FooBar;
            fn add(self, _rhs: Bar) -> Self::Output {
                FooBar
            }
        }

        impl std::ops::Sub<Bar> for Foo {
            type Output = BarFoo;
            fn sub(self, _rhs: Bar) -> Self::Output {
                BarFoo
            }
        }

        assert_eq!(Foo + Bar, FooBar);
        assert_eq!(Foo - Bar, BarFoo);
    }

    /*
     * Dynamic Dispatch (&dyn Trait, Box<dyn Trait> )
     *   - Specific methods to be called is determined at runtime
     *   - Works by creating a reference(`&dyn`) or smart pointer(`Box<dyn >`) to a trait object
     *   - Compiler builds a `vtable` for a trait when a trait instance object is created
     *   - `vtable` contains a pointer to the implementation of each method in the trait for the specific type of the object
     *   - Compiler will do a lookup in a vtable to determine which method should be called for which type that implements the given trait
     *   - This lookup will cause overhead but allows for more flexible code
     */
    // dynamic trait object for return unknown-size trait instance
    // &dyn has ownership
    // &dyn T vs &T (Box<dyn T> vs Box<T>), &dyn T, Box<dyn T> is also called fat pointer
    // &dyn T has a `ptr` point to instance and a `vptr` point to the corresponding `vtable`
    // &T has a `ptr` point to instance in heap
    {
        trait Animal {
            fn sound(&self) -> String;
        }

        struct Sheep;
        struct Cow;

        impl Animal for Sheep {
            fn sound(&self) -> String {
                "Maah".to_string()
            }
        }

        impl Animal for Cow {
            fn sound(&self) -> String {
                "Mooh".to_string()
            }
        }

        fn new_animal(species: &str) -> &dyn Animal {
            match species {
                "sheep" => &Sheep,
                "cow" => &Cow,
                _ => panic!("unknown animal"),
            }
        }
        fn bark(animal: &(impl Animal + ?Sized)) {
            println!("barking: {}", animal.sound());
        }

        let animals: [&dyn Animal; 3] =
            [new_animal("sheep"), new_animal("cow"), new_animal("sheep")];
        for animal in animals {
            bark(animal);
        }

        // return &dyn Animal value moved to _cow
        let mut _cow = new_animal("cow");
        // validate that _cow move to _cow_moved
        let _cow_moved = _cow;
    }

    // dynamic dispatch by using Box<dyn >
    {
        trait Animal {
            fn sound(&self) -> String;
        }

        struct Sheep;
        struct Cow;

        impl Animal for Sheep {
            fn sound(&self) -> String {
                "Maah".to_string()
            }
        }

        impl Animal for Cow {
            fn sound(&self) -> String {
                "Mooh".to_string()
            }
        }

        fn new_animal(a: &str) -> Box<dyn Animal> {
            match a {
                "sheep" => Box::new(Sheep),
                "cow" => Box::new(Cow),
                _ => panic!("unknown animal"),
            }
        }

        let animals = [new_animal("sheep"), new_animal("sheep"), new_animal("cow")];
        for animal in animals {
            println!("unbox dyn Animal sound: {}", animal.sound())
        }
    }

    /*
     * | ----------|-----------------------------------|------------------------------------------|
     * |           | & (reference)                     | Box                                      |
     * | ----------|-----------------------------------|------------------------------------------|
     * | Memory    | Only points to a value alread in  | Allocates data on heap and owns it, also |
     * |           | memory.                           | responsible for deallocating when values |
     * |           |                                   | goes out of scope.                       |
     * | ----------|-----------------------------------|------------------------------------------|
     * | Lifetime  | Limited                           | Can be passed across scopes              |
     * | ----------|-----------------------------------|------------------------------------------|
     * | Clonable  | No                                | Yes                                      |
     * | ----------|-----------------------------------|------------------------------------------|
     * | Pattern   | No                                | Yes                                      |
     * | ----------|-----------------------------------|------------------------------------------|
     */

    // generic, impl for specific trait
    {
        struct Pair<T> {
            x: T,
            y: T,
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        impl<T: std::fmt::Debug + PartialEq + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The larger member is x = {:?}", self.x);
                } else {
                    println!("The larger member is y = {:?}", self.y);
                }
            }
        }

        #[derive(Debug, PartialEq, PartialOrd)]
        struct Unit(f32);

        let pair = Pair::new(Unit(3.4), Unit(4.3));
        pair.cmp_display();
    }

    // trait with associated type, std::ops::Add is a example
    {
        trait Union<Rhs> {
            type Output;
            fn union(&self, rhs: &Rhs) -> Self::Output;
        }

        #[derive(Debug)]
        struct Point<T>(T, T);
        #[derive(Debug, PartialEq)]
        struct MixedPoint<T, U>(T, U, T, U);

        impl<T: Copy, U: Copy> Union<Point<U>> for Point<T> {
            type Output = MixedPoint<T, U>;

            fn union(&self, rhs: &Point<U>) -> Self::Output {
                MixedPoint(self.0, rhs.0, self.1, rhs.1)
            }
        }

        let p = Point(1, 2);
        let q = Point('a', 'b');
        let r = p.union(&q);
        assert_eq!(r, MixedPoint(1, 'a', 2, 'b'));
    }

    // implements trait for built-in type, with &dyn, Box<dyn > and static dispatch generic
    {
        trait Draw {
            fn draw(&self) -> String;
        }

        impl Draw for i8 {
            fn draw(&self) -> String {
                format!("i8: {}", self)
            }
        }

        impl Draw for f32 {
            fn draw(&self) -> String {
                format!("f32: {}", self)
            }
        }

        fn draw_with_ref(d: &dyn Draw) {
            println!("{}", d.draw());
        }

        fn draw_with_box(d: Box<dyn Draw>) {
            println!("{}", d.draw())
        }

        let x = 42_i8;
        draw_with_ref(&x);

        let y = 3.2_f32;
        draw_with_box(Box::new(y));

        fn draw_with_static_generic<T: Draw>(d: T) {
            println!("{}", d.draw())
        }
        draw_with_static_generic(y);
    }

    // Object-safe trait:
    //   - The return type isn't self.
    //   - There are no generic type parameters.
    #[allow(dead_code)]
    {
        trait SelfUnsafeTrait {
            fn f(&self) -> Self;
        }

        trait GenericUnsafeTrait<T> {
            fn f(&self) -> T;
        }
        trait ObjectSaveTrait {
            fn f(&self) -> &dyn ObjectSaveTrait;
            fn b(&self) -> Box<dyn ObjectSaveTrait>;
        }
    }
}
