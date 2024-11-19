#[cfg(test)]
mod tests {
    use std::{cell::RefCell, slice};

    #[test]
    fn test_unsafe_split_slice() {
        fn split_at_mut<T>(slice: &mut [T], at: usize) -> (&mut [T], &mut [T]) {
            assert!(at <= slice.len());

            let pointer = slice.as_mut_ptr();

            unsafe {
                (
                    slice::from_raw_parts_mut(pointer, at),
                    slice::from_raw_parts_mut(pointer.add(at), slice.len() - at),
                )
            }
        }

        let mut array = [1, 2, 3, 4, 5];

        let (first, second) = split_at_mut(&mut array, 3);

        assert_eq!(first, &[1, 2, 3]);
        assert_eq!(second, &[4, 5]);
    }

    #[test]
    fn test_calling_c_function_within_unsafe_block() {
        // `"C"` defines thc C language's ABI(application binary interface)
        unsafe extern "C" {
            fn abs(value: i32) -> i32;
        }

        let value = -42;
        unsafe {
            assert_eq!(abs(value), 42);
        }
    }

    /// Static vs. Constant
    /// Static variable have a fixed address in memory. Using the value will always
    /// access the same data. Static variables can be mutable.
    ///
    /// Constants are allowed to duplicate their data whenever they're used.
    #[test]
    fn accessing_static_variable_is_unsafe() {
        use std::sync::Mutex;
        static COUNTER: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

        {
            let counter = COUNTER.lock().expect("acquire counter lock failed");
            let mut counter = counter.borrow_mut();
            *counter += 42;
            // contain MutexGuard in block, release when go beyond block
        }

        assert_eq!(*COUNTER.lock().unwrap().borrow(), 42);
    }

    /// A trait is unsafe when at least one of its methods has some invariant that
    /// the compiler can't verify.
    ///
    #[test]
    #[allow(dead_code)]
    fn impl_send_sync_for_raw_pointer_is_unsafe() {
        struct Tag {
            pointer: *mut String,
        }

        unsafe impl Send for Tag {}
    }
}
