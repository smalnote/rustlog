#[cfg(test)]
mod tests {
    // struct is compound type with named fields of different types
    #[test]
    fn struct_is_compund_type_with_named_fields_of_difference_type() {
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

    // assign struct with spread syntax
    #[test]
    fn assign_struct_with_spread_syntax() {
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
    #[test]
    fn init_struct_by_field_name_shorthand() {
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
            std::mem::size_of_val(&user1.name) // String 8bytes * 3, on aarch64
                + std::mem::size_of_val(&user1.age) // u8 1byte
                + 7 // alignment padding
                + std::mem::size_of_val(&user1.email) // String 8bytes * 3
        );
    }

    // print struct with debug format
    #[test]
    fn derive_debug_trait_for_print_struct() {
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
    #[test]
    fn unit_type_is_struct_without_fields() {
        struct Unit;
        let unit = Unit;
        assert_eq!(std::mem::size_of_val(&unit), 0);
    }

    // tuple struct is a struct with anonymous fields
    #[test]
    fn tuple_struct_is_a_struct_with_anonymous_fields() {
        struct Color(u8, u8, u8);
        let black = Color(0, 0, 0);
        assert_eq!(black.0, 0);
        assert_eq!(black.1, 0);
        assert_eq!(black.2, 0);
    }

    // think of struct is a type definition keyword
    #[test]
    fn think_of_struct_as_type_definition_keyword() {
        struct User {
            name: String,
        }
        struct Point(i32, i32, i32);
        struct Empty;
        struct EmptyTuple();
        let alice: User = User {
            name: String::from("Alice"),
        };
        // struct is zero-cost abstraction
        assert_eq!(std::mem::size_of_val(&alice), std::mem::size_of::<String>());

        let origin: Point = Point(0, 0, 0);
        assert_eq!(
            std::mem::size_of_val(&origin),
            3 * std::mem::size_of::<i32>()
        );

        let empty: Empty = Empty;
        let empty_tuple: EmptyTuple = EmptyTuple();

        assert_eq!(std::mem::size_of_val(&empty), 0);
        assert_eq!(std::mem::size_of_val(&empty_tuple), 0);

        assert_eq!(alice.name, "Alice");
        assert_eq!(origin.0 + origin.1 + origin.2, 0);
    }
}
