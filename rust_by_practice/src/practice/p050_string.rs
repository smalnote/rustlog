#[cfg(test)]
mod tests {
    use core::str;

    // &str string slice
    #[test]
    fn ref_string_is_ref_str() {
        let s: String = String::from("hello world");
        // immutable &str, called str slice
        // here String is used as an array, `&array[<range>]` is a immutable view to array, called slice
        let hello: &str = &s[0..5];
        let world: &str = &s[6..11];
        assert_eq!(hello, "hello");
        assert_eq!(world, "world");
    }

    // str can be use in Box
    #[test]
    fn str_can_be_use_in_box() {
        let s: Box<str> = "hello world".into();
        let ss: &str = &s;
        assert_eq!(ss, "hello world");
    }

    // you can only concat String with &str
    #[test]
    fn you_can_only_concat_string_with_ref_str() {
        let s1: String = String::from("hello ");
        let s2: String = String::from("world!");
        let s3: String = s1 + &s2; // s1 ownership moved to s3, &String -> &str, impl Add<&str, Output = String> for String
        let s4: String = "hello ".to_string() + s2.as_str(); // cannot use s1 here
        assert_eq!(s3, s4);
    }

    // string byte escapes
    #[test]
    fn string_byte_escapes() {
        let s = "I'm writing Ru\x73\x74!";
        println!("{}", s);
    }

    // string unicode points
    #[test]
    fn string_unicode_points() {
        let unicode_code_point = "\u{211D}";
        let character_name = "\"Double-stroke capital R\"";
        println!(
            "Unicode point: {}, Character name: {}",
            unicode_code_point, character_name
        );
        assert_eq!(unicode_code_point.len(), 3);
    }

    // multiple lines string
    #[test]
    fn multiple_lines_string() {
        let long_string = "String literals
                can span multiple lines.
                The line break and indentation here \
                can be escaped too!";
        println!("{}", long_string);
    }

    // raw string
    #[test]
    fn raw_string() {
        let raw_str = r"I'm writing Ru\x73\x74!";
        println!("{}", raw_str);

        let paragraph = r#"Hello, World! \t
你好，世界 \t
        "#;
        println!("{}", paragraph);

        let bytes = b"Hello, world!";
        println!("{}", str::from_utf8(bytes).unwrap());
    }

    // String -> &str by slice index
    #[test]
    fn string_range_index_as_ref_str() {
        let s1 = String::from("hi,中国");
        let h1: &str = &s1[3..6];
        assert_eq!(h1, "中");
    }

    // iterating characters in a string
    #[test]
    fn iterating_chars_in_a_string() {
        let s1: String = String::from("hello, 世界");
        for c in s1.chars() {
            println!("{}", c);
        }
    }

    // index string chars with utf8_slice::slice()
    #[test]
    fn index_string_chars_with_lib_utf8_slice() {
        let s = "The 🚀 goes to the 🌑!";
        let rocket = utf8_slice::slice(s, 4, 5);
        assert_eq!(rocket, "🚀");
    }

    // String is a Vec<u8> that guarantee to be valid utf-8 sequences
    #[test]
    fn string_is_backed_by_vec_u8() {
        let mut s = String::new();
        s.push_str("hello");
        let v: Vec<u8> = vec![104, 101, 108, b'l', b'o']; // note that b'l' is u8 value of char 'l'
        let v = String::from_utf8(v).expect("invalid utf-8 sequences");
        assert_eq!(s, v);
    }

    // String is made up of three components: pointer to heap memory, length in bytes of content, capacity in bytes of memory
    // A String's capacity grows automatically to accommodate its length.
    #[test]
    fn string_type_components() {
        let mut s = String::with_capacity(5);

        println!("s = {}, capacity = {}", &s, s.capacity());
        for _ in 0..2 {
            s.push_str("hello");
            println!("s = {}, capacity = {}", &s, s.capacity());
        }
    }

    // Manually manage String memory and rebuild String from pointer, lenght and capacity.
    #[test]
    fn manage_string_memory_manually() {
        let story = "The 🚀 goes to the 🌑!".to_string();

        let mut manual_story = std::mem::ManuallyDrop::new(story);

        let ptr = manual_story.as_mut_ptr();
        let len = manual_story.len();
        let cap = manual_story.capacity();

        let story2 = unsafe { String::from_raw_parts(ptr, len, cap) };

        assert_eq!(*manual_story, story2);
    }

    // String index in bytes, a chinese character is 3 bytes
    #[test]
    fn string_chars_indexing() {
        let s: String = String::from("你好，世界!");

        // in bytes index
        let world: &str = &s[9..15];
        assert_eq!(world, "世界");

        // chars() iterator index
        let substring: String = s.chars().skip(3).take(2).collect::<String>();
        assert_eq!(substring, "世界");
    }

    // &String can be convert to &str implicitly
    #[test]
    fn ref_string_can_be_convert_to_ref_str_implicitly() {
        fn take_first_word(s: &str) -> &str {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[..i];
                }
            }
            s
        }
        let s: String = String::from("hello world!");
        let first_word: &str = take_first_word(&s);
        assert_eq!(first_word, "hello");
    }
}
