#[cfg(test)]
mod tests {
    // if-let
    #[test]
    fn if_let() {
        let optional: Option<i32> = Some(42);
        if let Some(number) = optional {
            assert_eq!(number, 42);
        }
    }

    // if return
    #[test]
    fn if_with_return_value() {
        let x: i32 = 42;
        let y: i32 = if x == 42 { 1 } else { 0 };
        assert_eq!(y, 1);
    }

    // if/else expression in assignment
    #[test]
    fn if_else_with_return_value() {
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

    // loop return
    #[test]
    fn loop_with_return_value() {
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
    #[test]
    #[allow(clippy::never_loop)]
    fn loop_with_label_for_break() {
        let x;
        'outer: loop {
            loop {
                x = 42;
                break 'outer;
            }
        }
        assert_eq!(x, 42);
    }

    // while loop
    #[test]
    fn while_loop() {
        let mut x = 0;
        while x < 42 {
            x += 1;
        }
        assert_eq!(x, 42);
    }

    // for in range
    #[test]
    fn for_in_range() {
        let mut sum: i32 = 0;
        for i in 0..42 {
            sum += i;
        }
        assert_eq!(sum, 861);
    }

    // for in array
    #[test]
    fn for_in_array() {
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let mut sum = 0;
        for i in a {
            sum += i;
        }
        assert_eq!(sum, 15);
    }

    // for-in move ownership by default
    #[test]
    fn for_in_loop_move_ownership_by_default() {
        let names: [String; 2] = [String::from("Alice"), String::from("Bob")];
        // for-in moves names due to implicit call to names.into_iter();
        for _name in names {}

        // invalid after `names` moved
        // println!("{:?}", names);
        // println!("[{}, {}]", names[0], names[1]);

        let mut names: [String; 2] = [String::from("Alice"), String::from("Bob")];
        // for-in of `&mut names` calls names.iter_mut()
        for name in &mut names {
            // borrow names
            *name = name.to_lowercase();
            println!("{}", name);
        }

        for _name in names.iter_mut() {}
        println!("{:?}", names);
    }

    // for-in get copy of array of primary types
    #[test]
    fn for_in_loop_copy_array_with_elements_implements_trait_copy() {
        let numbers: [i32; 3] = [1, 2, 3];
        for number in numbers {
            println!("{}", number);
        }

        // Type i32 implements the trait Copy, calls of into_iter() copy the whole array
        for _number in numbers.into_iter() {}
        for _number in numbers.into_iter() {}

        // numbers is still available after copy
        println!("{:?}", numbers);
    }

    // for-in iterate with index
    #[test]
    fn for_in_with_index_by_iter_enumerate() {
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
    #[test]
    fn break_with_return_value_in_loop() {
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

    // labeled loop with 'outer, `'outer` is called lifetime annotation
    // `'outer` pronounce `tick lifetime outer` or `tick outer`
    #[test]
    fn loop_with_label() {
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
}
