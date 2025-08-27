#[cfg(test)]
mod tests {
    use core::fmt;
    use std::sync::Arc;

    #[test]
    fn test_trait_with_default_generic_type() {
        trait MyAdd<Rhs = Self> {
            type Output;
            fn add(self, rhs: Rhs) -> Self::Output;
        }

        struct Millimeters(f64);
        struct Meters(f64);

        impl MyAdd for Millimeters {
            type Output = Millimeters;
            fn add(self, rhs: Self) -> Self::Output {
                Millimeters(self.0 + rhs.0)
            }
        }

        impl MyAdd for Meters {
            type Output = Meters;
            fn add(self, rhs: Self) -> Self::Output {
                Meters(self.0 + rhs.0)
            }
        }

        impl From<Meters> for Millimeters {
            fn from(value: Meters) -> Self {
                Self(value.0 * 1000.0)
            }
        }

        impl MyAdd<Meters> for Millimeters {
            type Output = Millimeters;
            fn add(self, rhs: Meters) -> Self::Output {
                // fully qualified syntax to call method of specified trait
                self.add(<Meters as Into<Millimeters>>::into(rhs))
            }
        }

        let mm = Millimeters(30.0);
        let m = Meters(0.3);

        let mm = mm.add(m);
        assert_eq!(mm.0, 330.0);
    }

    #[test]
    fn test_super_trait() {
        struct Point {
            x: i32,
            y: i32,
        }

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        trait OutlineDisplay: fmt::Display {
            fn outline_fmt(&self) {
                let output = self.to_string(); // from fmt::Display
                let len = output.len();
                println!(" {} ", "-".repeat(len + 2));
                println!("< {} >", output);
                println!(" {} ", "-".repeat(len + 2));
            }
        }

        impl OutlineDisplay for Point {}

        let point = Point { x: 42, y: 0 };

        <Point as OutlineDisplay>::outline_fmt(&point);
    }

    #[test]
    fn test_newtype_as_zero_cost_wrapper_for_implementation() {
        struct OutlineString<'a>(&'a str);

        impl fmt::Display for OutlineString<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for line in self.0.lines() {
                    writeln!(f, "{}", line)?;
                    writeln!(f, "{}", "-".repeat(line.len()))?;
                }
                Ok(())
            }
        }

        let p = "This is a multiple line";

        print!("{}", OutlineString(p));
    }

    // Since Rust 1.86
    #[test]
    fn test_trait_upcasting() {
        trait Bird: Animal {}
        trait Animal {}

        fn upcast(b: &dyn Bird) -> &dyn Animal {
            b
        }

        struct Sparrow {}
        impl Animal for Sparrow {}
        impl Bird for Sparrow {}

        // &dyn Trait -> &dyn SuperTrait
        let sparrow = Sparrow {};
        let _: &dyn Animal = upcast(&sparrow);

        // Arc<dyn Trait> -> Arc<dyn SuperTrait>
        let sparrow = Sparrow {};
        let _: Arc<dyn Animal> = Arc::new(sparrow);

        let sparrow = Sparrow {};
        let raw_sparrow: *mut dyn Bird = Box::into_raw(Box::new(sparrow));
        // *const dyn Trait -> *const dyn SuperTrait
        let _: *const dyn Animal = raw_sparrow;
        // *mut dyn Trait -> *mut dyn SuperTrait
        let _: *mut dyn Animal = raw_sparrow;
        // cleanup
        let _ = unsafe { Box::from_raw(raw_sparrow) };
    }

    #[test]
    #[allow(dead_code)]
    fn downcasting_from_std_any_trait() {
        use std::any::Any;
        trait Phone: Any {}
        trait Device {}
        impl dyn Phone {
            fn downcast_ref<T>(&'static self) -> Option<&'static T> {
                (self as &dyn Any).downcast_ref()
            }
        }
    }
}
