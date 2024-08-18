#[cfg(test)]
mod tests {

    // tuple compound values of different type
    #[test]
    fn tuple_is_compound_values_of_difference_type() {
        let tt: (u8, (i16, &str)) = (42, (-42, "hello"));
        // acess tuple element by .index
        assert_eq!(tt.1 .1, "hello");
    }

    // tuple up to 12 elements can be print
    #[test]
    fn println_can_handle_tuple_with_up_to_12_elements() {
        let twelve = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, "twelve");
        println!("{:?}", twelve);
    }

    // tuple destructuring
    #[test]
    fn tuple_destructuring() {
        let t: (i32, i32, i32) = (1, 2, 3);
        let (.., z) = t;
        assert_eq!(z, 3);
    }

    // tuple as function parameter
    #[test]
    fn tuple_as_function_parameter() {
        fn sum_multiply(t: (i32, i32)) -> (i32, i32) {
            (t.0 + t.1, t.0 * t.1)
        }
        let t: (i32, i32) = (1, 2);
        assert_eq!(sum_multiply(t), (3, 2));

        fn sum_multiply_destruct((a, b): (i32, i32)) -> (i32, i32) {
            (a + b, a * b)
        }
        let q = (3, 4);
        assert_eq!(sum_multiply_destruct(q), (7, 12));
    }
}
