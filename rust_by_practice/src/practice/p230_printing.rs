#[cfg(test)]
mod tests {
    use std::fmt::Display;

    /*
     * Some printing marcos defined in [std::fmt]
     *   - format!: write formatted text to [String]
     *   - print!: write formatted text to io::stdout
     *   - println!: print! with newline appended
     *   - eprint!: write formatted text to io::stderr
     *   - eprintln!: eprint! with newline appended
     */
    #[test]
    #[allow(clippy::print_literal)]
    fn printing_marcos() {
        let text = format!("Hello, {}", 42);
        print!("{}!{}", text, "\n");
        println!("{}!", text);
        eprint!("{}!{}", text, "\n");
        eprintln!("{}!", text);
    }

    #[test]
    #[allow(dead_code)]
    fn debug_trait_for_print() {
        #[derive(Debug)]
        struct Point(i32, i32);

        let point = Point(3, 4);
        assert_eq!(format!("{:?}", point), "Point(3, 4)");

        // multiline string, `\` backslash for omitting newline
        let hash_formatted = "\
Point(
    3,
    4,
)";
        assert_eq!(format!("{:#?}", point), hash_formatted);
    }

    // implements trait `Debug` for format placeholders `{:?}`, `{:#?}`, etc
    #[test]
    fn custom_debug_trait_implementation() {
        struct Point(i32, i32);

        use std::fmt;

        impl fmt::Debug for Point {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {})", self.0, self.1)
            }
        }

        assert_eq!(format!("{:?}", Point(3, 4)), "(3, 4)");
        assert_eq!(format!("{:#?}", Point(3, 4)), "(3, 4)");
    }

    // implements trait `Display` for format placeholder `{}`
    #[test]
    fn implements_display_trait() {
        struct Point(f64, f64);

        use std::fmt;
        impl fmt::Display for Point {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Point{{ x: {}, y: {} }}", self.0, self.1)
            }
        }
        assert_eq!(
            format!("{}", Point(3.27, 4.13)),
            "Point{ x: 3.27, y: 4.13 }"
        );

        impl fmt::Debug for Point {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "({}, {})", self.0, self.1)
            }
        }
        assert_eq!(format!("{:?}", Point(3.27, 4.13)), "(3.27, 4.13)");
    }

    // `?` question mark for error handling in Display
    #[test]
    fn implements_display_vector() {
        use std::fmt;

        /*
         * Type wrapper for implement outside trait std::fmt::Display.
         * Orphan rule: When implementing `Trait` for `Type`, either the
         * `Trait` or the `Type` must be defined in current crate, the preventing
         * potential conflicts and ensures that implementation are unique and
         * predictable across different parts of a program.
         *
         * *Note*: struct T(U); is a zero-cost wrapper at runtime.
         */
        struct Vector<T>(Vec<T>);

        impl<T: Display> fmt::Display for Vector<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[")?; // `?` return if write fail

                for (i, v) in self.0.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?; // `?` return if write fail
                    }
                    write!(f, "{}: {}", i, v)?; // `?` return on errors
                }

                write!(f, "]")
            }
        }
        let v = Vector(vec![1, 2, 3, 4, 5]);

        let vector_displayed = format!("{}", v);
        assert_eq!(vector_displayed, "[0: 1, 1: 2, 2: 3, 3: 4, 4: 5]");
    }
}
