#[cfg(test)]
mod tests {
    // When casting to an unsigned type T, T::MAX + 1 is added or subtracted until
    // the value fits into the new type.
    #[test]
    fn unsigned_integer_coercion() {
        let one_thousand = 1000_i32;
        let small = one_thousand as u8;
        assert_eq!(small as i32, one_thousand - (std::u8::MAX as i32 + 1) * 3);
        // For positive numbers, this the same as the modulus
        assert_eq!(small, (one_thousand % 256) as u8);

        let u32_plus_one = std::u32::MAX as u64 + 1;
        assert_eq!(u32_plus_one as u32, 0);

        assert_eq!(-1_i8 as u8, 255);
        assert_eq!((std::u32::MAX as u16 as u8), std::u8::MAX);
    }

    #[test]
    fn keyword_as_can_be_chained() {
        assert_eq!(97_i32 as u8 as char, 'a');
        assert_eq!((('a' as u8) + 1) as char, 'b');
    }

    #[test]
    fn cast_float_to_integer_trim_fraction() {
        assert_eq!(3.1415_f32 as u8, 3);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn suppress_overflow_errors() {
        assert_eq!(u8::MAX, 255);

        let v = 1000 as u8;
        assert_eq!(v, 1000);
        assert_eq!(v, 232);
        assert_eq!(format!("{}", v), "232");
    }

    /*
     * Since Rust 1.45, the `as` keyword performs a *saturating cast*
     * when casting from float to int, If the floating point value exceeds
     * the upper bound or is less than the lower bound, the returned value
     * will be equal to the bound crossed.
     */
    #[test]
    fn saturating_casting() {
        assert_eq!(-314.523_f64 as i8, std::i8::MIN);

        assert_eq!(-314.523_f64 as u8, std::u8::MIN);
        assert_eq!(-314.523_f64 as u8, 0);

        assert_eq!(65536.313_f64 as u16, std::u16::MAX);
    }

    /*
     * Type coercion incurs a small runtime cost and cab be avoided
     * with unsafe methods, however the results might overflow and
     * return **unsound values**. Use these methods wisely:
     */
    #[test]
    fn unsafe_coercion() {
        unsafe {
            // 300.0 as u8 is 44 -> 300 - (u8::MAX + 1) = 300 - 256 = 44
            assert_eq!(300.0_f32.to_int_unchecked::<u8>(), 44);

            // -100.0 as u8 is 156 -> -100 + (u8::MAX + 1) = -100 + 256 = 156
            assert_eq!((-100.0_f32).to_int_unchecked::<u8>(), 156);

            // Nan as u8 is 0
            assert_eq!(f32::NAN.to_int_unchecked::<u8>(), 0);
        }
    }

    #[test]
    fn unsafe_pointer_calculation() {
        let mut values: [i32; 2] = [1, 2];

        let p1: *mut i32 = values.as_mut_ptr();
        let first_address: usize = p1 as usize;
        let second_address: usize = first_address + std::mem::size_of::<i32>();
        let p2: *mut i32 = second_address as *mut i32;
        unsafe {
            *p2 = *p2 + 1;
            assert_eq!(values[1], 3);

            // both reference and pointer us `*` to reach pointed value
            let ref_p2: &mut i32 = &mut *p2;
            *ref_p2 = *ref_p2 + 1;
            assert_eq!(values[1], 4);
        }
    }

    #[test]
    fn array_pointer() {
        let arr: [u64; 13] = [0; 13];
        assert_eq!(std::mem::size_of_val(&arr), std::mem::size_of::<u64>() * 13);

        let arr_ref: &[u64; 13] = &arr;
        let arr_ptr: *const [u64] = &arr;

        assert_eq!(std::any::type_name_of_val(&arr_ref), "&[u64; 13]");
        assert_eq!(std::any::type_name_of_val(&arr_ptr), "*const [u64]");

        let arr_ptr_casted: *const [u8] = arr_ptr as *const [u8];
        assert_eq!(std::any::type_name_of_val(&arr_ptr_casted), "*const [u8]");
        unsafe {
            // slice and array pointer: holds element type and length
            assert_eq!(
                std::mem::size_of_val(&*arr_ptr_casted),
                std::mem::size_of::<u8>() * 13
            );
        }
    }
}
