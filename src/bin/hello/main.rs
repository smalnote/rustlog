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
}
