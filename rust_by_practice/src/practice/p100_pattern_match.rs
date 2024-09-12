#[cfg(test)]
mod tests {
    // match with wildcard
    #[test]
    fn match_with_wildcard() {
        let x: i32 = 42;
        match x {
            0 => println!("zero"),
            1 => println!("one"),
            _ => println!("other"),
        }
    }

    // match with returned value
    #[test]
    fn match_with_return_value() {
        let x: i32 = 42;
        let y = match x {
            0 => "zero",
            1 => "one",
            _ => "other",
        };
        assert_eq!(y, "other");
    }

    // rename matched value
    #[test]
    fn rename_matched_value() {
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
    // `==` requires Type implements trait PartialEq
    // while `matches!` not, and is more powerful
    #[test]
    fn matches_marco_for_range() {
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
    #[test]
    fn matches_marco_for_enum() {
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
    #[test]
    fn match_enum_with_tuple() {
        enum Foo {
            Bar,
            Baz,
            Qux(u32),
        }

        let foos = [Foo::Bar, Foo::Baz, Foo::Qux(42), Foo::Qux(100)];
        for foo in foos.iter() {
            match foo {
                Foo::Bar => println!("Bar"), // Foo::Bar as u8 is invalid here
                Foo::Baz => println!("Baz"), // Foo::Baz as u8 is invalid here
                Foo::Qux(_n @ ..100) => println!("small Qux"),
                Foo::Qux(_n @ 100..) => println!("large Qux"),
            }
        }
    }

    // pattern, use | to match serval values
    #[test]
    fn match_multiple_value_in_one_case() {
        for i in 0..3 {
            match i {
                0 | 1 => println!("zero or one"),
                _ => println!("other"),
            }
        }
    }

    // pattern, use ..= to match inclusive range
    #[test]
    fn match_range() {
        for i in 0..3 {
            match i {
                0..=1 => println!("zero or one"),
                _ => println!("other"),
            }
        }
    }

    // pattern, use @ to create a variable that holds a value
    #[test]
    fn match_specific_collection_of_values_with_at_sign() {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 10 };
        match p {
            Point { x, y: 0 } => println!("Ont the x axis at x = {x}"),
            Point {
                // rename x with new_x
                x: new_x @ 0..=5,
                y: y @ (10 | 20 | 30),
            } => println!("On the y axis at y = {y}, x = {new_x}"),
            Point { x, y } => println!("x = {x}, y = {y}"),
        }
    }

    // pattern, use .. to match any value
    #[test]
    fn match_open_range() {
        let x = 1;
        match x {
            1.. => println!("one or more"),
            _ => println!("other"),
        }
    }

    // pattern match enum struct
    #[test]
    fn match_enum_struct() {
        enum Message {
            Hello { id: i32 },
        }

        let msg: Message = Message::Hello { id: 5 };
        match msg {
            Message::Hello { id: id @ 3..=7 } => println!("Found and id in range [3, 7]: {id}"),
            Message::Hello {
                id: new_id @ (42 | 53 | 67),
            } => println!("Found specific id {}", new_id),
            Message::Hello { id } => println!("Found some other id: {id}"),
        }
    }

    // pattern match with if statement
    #[test]
    fn match_case_with_if_statement() {
        let num: Option<i32> = Some(4);
        let split: i32 = 5;
        match num {
            Some(x) if x < split => println!("x is less than {split}"),
            Some(_) => println!("x is greater than or equal to {split}"),
            None => {}
        }
    }

    // pattern match, using .. to ignore remaining parts of the value
    #[test]
    fn match_remaining_parts_of_tuple() {
        let numbers = (2, 4, 6, 8);
        match numbers {
            (2, .., last) => println!("first = 2, last = {last}"),
            (first @ (..2 | 3..), ..) => println!("first = {first}"),
        }
    }

    #[test]
    fn match_refutable_pattern_with_while_let() {
        let mut stack: Vec<i32> = [5; 10].into();

        let mut count = 0;
        while let Some(element) = stack.pop() {
            count += 1;
            assert_eq!(element, 5);
        }
        assert_eq!(count, 10);
    }

    #[test]
    fn match_pattern_in_destructuring_struct() {
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Point { x: 42, y: 6 };
        let Point { x: a, y: b } = point;
        assert_eq!(a, 42);
        assert_eq!(b, 6);

        let point = Point { x: a, y: b };
        let Point { x, y } = point;
        assert_eq!(x, 42);
        assert_eq!(y, 6);
    }

    #[test]
    fn match_multiple_value_with_at_binding() {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: ranged_id @ 3..=7,
            } => println!("Found an id in range: {}", ranged_id),
            Message::Hello { id: 0..=12 } => println!("Found anther id"),
            Message::Hello { id: _ } => println!("Nevertheless"),
        }
    }
}
