#[cfg(test)]
mod tests {
    use core::f32;

    // enum with additional tuple
    #[test]
    fn enum_with_additional_tuple() {
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
    #[test]
    fn enum_values_default_start_from_zero() {
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
    #[test]
    fn enum_with_explicit_integer_value() {
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
    #[test]
    fn convert_enum_to_integer_by_keyword_as() {
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
    #[test]
    fn enum_can_hold_different_types() {
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
    #[test]
    fn test_enum_value_with_if_let_statement() {
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
    #[test]
    fn built_in_option_is_implemented_with_enum() {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            x.map(|x| x + 1)
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
    #[test]
    fn implement_linked_lit_via_enum() {
        #[derive(Debug)]
        enum List<T> {
            Cons(T, Box<List<T>>),
            None,
        }

        impl<T> List<T> {
            fn new() -> List<T> {
                List::None
            }

            fn prepend(self, elem: T) -> List<T> {
                List::Cons(elem, Box::new(self))
            }

            fn len(&self) -> u32 {
                match *self {
                    List::Cons(_, ref tail) => tail.len() + 1,
                    List::None => 0,
                }
            }

            fn awkward_len(&mut self) -> u32 {
                let mut node = self;
                let mut size = 0_u32;
                while let List::Cons(_, ref mut next) = node {
                    node = &mut **next;
                    size += 1;
                }
                size
            }
        }

        impl<T: std::fmt::Display> List<T> {
            fn stringify(&self) -> String {
                match *self {
                    List::Cons(ref head, ref tail) => {
                        format!("{} -> {}", head, tail.stringify())
                    }
                    List::None => "None".to_owned(),
                }
            }
        }

        let mut list = List::<f32>::new();
        list = list.prepend(3.2);
        list = list.prepend(4.48);
        list = list.prepend(1.67);
        assert_eq!(list.stringify(), "1.67 -> 4.48 -> 3.2 -> None");
        assert_eq!(list.len(), 3);
        assert_eq!(list.awkward_len(), 3);
        println!("{}", list.stringify());
    }
}
