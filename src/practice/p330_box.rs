#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};

    #[test]
    fn test_deref_box() {
        let x = 42;
        let y = &x;
        let mut z = Box::new(6);

        *z *= 7;

        assert_eq!(*y, *z);
    }

    #[test]
    fn test_custom_type_with_trait_deref() {
        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(value: T) -> Self {
                Self(value)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> DerefMut for MyBox<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        let mut x: MyBox<i32> = MyBox::new(42);
        *x /= 7;
        assert_eq!(*x, 6);

        fn multiply_seven(value: &mut i32) {
            *value *= 7;
        }
        // &mut MyBox<i32> with implicit deref coercion, converted to type &mut i32
        multiply_seven(&mut x);
        assert_eq!(*x, 42);

        let mut y: MyBox<MyBox<i32>> = MyBox::new(x);
        // double deref from &mut MyBox<MyBox<i32>> to &mut i32
        multiply_seven(&mut y);
        assert_eq!(**y, 294);
    }
}
