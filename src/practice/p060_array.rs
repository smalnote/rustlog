#[cfg(test)]
mod tests {
    // array literal
    #[test]
    fn array_literal() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(std::mem::size_of_val(&arr), 5 * std::mem::size_of::<i32>());
        let arr: [i64; 5] = [1, 2, 3, 4, 5];
        assert_eq!(arr.len(), 5);
        assert_eq!(std::mem::size_of_val(&arr), 5 * std::mem::size_of::<i64>());
        assert_eq!(arr[3], 4);
    }

    // init array with same value
    #[test]
    fn init_array_with_samve_value() {
        let arr: [i32; 5] = [0; 5];
        assert_eq!(arr, [0, 0, 0, 0, 0]);
    }

    // get array element by get
    #[test]
    fn get_array_element_by_get_method() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let third: &i32 = arr.get(2).unwrap();
        assert_eq!(*third, 3);
    }

    // slice if type of &[T]
    #[test]
    fn slice_if_readonly_ref_of_array() {
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
    #[test]
    fn ref_str_is_slice_of_char() {
        let s: String = String::from("hello, world!");
        let world: &str = &s[7..12];
        assert_eq!(world, "world");
    }
}
