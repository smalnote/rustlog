#[cfg(test)]
mod tests {
    /*
     * Lifetimes:
     *   - Another kind of generic ensuring that `references` are valid as long as needed
     *   - Every reference has a lifetime, which is the scope for which that reference is valid
     *   - Most of the time implicit and inferred, don't need to worry
     *   - Sometimes lifetime annotations are needed, if the compiler can't infer it
     *   - Lifetime annotations is a concept which most other programming languages don't have
     *
     * Borrow Checker:
     *   - Borrow checker compares scopes to determine whether all borrows are valid
     *   - Key part of Rust's ownership system
     *   - Tracks lifetimes of references and ensures that they don't violate the ownership rules
     *   - Rules ensures that a value is not accessed after it has been moved or free from memory
     *   - Important: A reference to a value must never outlive the value itself!
     *
     * Note: Borrow ot type T has type &T, to get a type T from &T, use the dereferencing *&T.
     *   In short, *&T == T, this is useful when comparing by `==`, `!=` or `assert_eq!`.
     */

    #[test]
    fn disjoint_borrow_references() {
        let i = 42;

        {
            let borrow1 = &i;
            assert_eq!(*borrow1, i); // dereferencing
            if *borrow1 != i {
                panic!("should not reach here!")
            }
        } // borrow1 lifetime end

        {
            let borrow2 = &i;
            assert_eq!(*borrow2, i);
            // when printing type of &T, T, &&T, they are the same
            assert_eq!(format!("{}", borrow2), "42");
            assert_eq!(format!("{}", *borrow2), "42");
            assert_eq!(format!("{}", &borrow2), "42");
            println!(
                "borrow2 = {}, *borrow2 = {}, &borrow2 = {}",
                borrow2, *borrow2, &borrow2
            )
        } // borrow2 lifetime end
    }

    /*
     * Ignoring elision rules, lifetimes in function signatures have a few constraints:
     *   - Any `reference` must have an annotated lifetime
     *   - Any `reference` being returned must have the the same lifetime as one of the inputs or be static
     *   - Lifetimes of parameters must be at least as long as the function
     */
    #[test]
    fn function_input_reference_with_lifetime() {
        #[allow(clippy::needless_lifetimes)]
        fn is_one<'a>(value: &'a i32) {
            assert_eq!(value, &1_i32);
        }
        let one = 1_i32;
        is_one(&one);
    }

    // mutable reference with life time `value: &'a mut i32`
    /*
     * `&'a mut i32`:
     *   - `&` indicates a reference
     *   - `'a` is the lifetime annotation with the reference itself
     *   - `mut i32` specifies that the reference is mutable and points to an `i32`
     *
     * **Why not `'a &mut i32`**:
     *   - The lifetime annotation `'a` is applied to reference, so its stick to the reference sign `&`, forms `&'a`
     *   - `'a &mut i32` imply the lifetime `'a` is associated with something other than the reference it self
     *   - For example, `'a i32` or `'a mut i32` comes without reference `&`, the lifetime annotation `'a` is meaningless
     *
     * The lifetime annotation 'a in Rust is pronounced as:
     *   - "tick a" or "tick lifetime a"
     *  Here's a breakdown:
     *   - `'` (the apostrophe) is often referred to as "tick" in spoken form.
     *   - `a` is the specific name of the lifetime, so you would say it as the letter "a."
     *  Example Pronunciations:
     *   - `'a`: "tick a" or "tick lifetime a"
     *   - `'b`: "tick b" or "tick lifetime b"
     *   - `'static`: "tick static" or "tick lifetime static"
     */
    #[test]
    fn lifetime_annotation_for_mutable_reference() {
        #[allow(clippy::needless_lifetimes)]
        fn double<'a>(value: &'a mut i32) {
            *value *= 2
        }

        let mut value = 42_i32;
        double(&mut value);
        assert_eq!(value, 84);
    }

    /*
     * As states before, lifetime annotation is another kind of generic.
     * Like function can having multiple generic type parameters,
     * it can have multiple lifetimes.
     *
     * Any reference being returned must have the same life as one of the input or static
     */
    #[test]
    fn multiple_lifetimes() {
        #[allow(clippy::needless_lifetimes)]
        fn multiply_lhs_by_rsh<'a, 'b, T>(lhs: &'a mut T, rhs: &'b mut T) -> &'a mut T
        where
            T: std::ops::Mul<Output = T> + Copy + Sized,
        {
            *lhs = *lhs * *rhs;
            lhs
        }

        let mut lhs = 6;
        let mut rhs = 7;
        let result = multiply_lhs_by_rsh(&mut lhs, &mut rhs);
        assert_eq!(*result, 42_i32);

        // result is a reference of lhs, changing it affect the lhs
        *result *= 2;
        assert_eq!(lhs, 84_i32);
    }

    #[test]
    fn return_str_reference_with_lifetime() {
        // x and y must have the same lifetime a for returning
        fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let x = "long";
        let y = "longer";

        {
            let yy = "longer_longer";
            let zz = longer(x, yy); // x has 'a, yy has 'b, 'a :> 'b, so 'a can be coerced to 'b to satisfy function `longer`
            assert_eq!(zz, "longer_longer");
        }

        let z = longer(x, y);
        assert_eq!(z, "longer");
    }

    // static lifetime means value live throughout the entire program
    #[test]
    fn passing_local_value() {
        #[allow(clippy::needless_lifetimes)]
        fn output_input_reference<'a>(s: &'a mut String) -> &'a String {
            *s = String::from("hello");
            s
        }

        let mut s: String = String::new();
        output_input_reference(&mut s);
        assert_eq!(s, "hello");

        #[allow(clippy::extra_unused_lifetimes)]
        fn output_value<'a>() -> String {
            String::from("hello")
        }
        assert_eq!(output_value(), "hello");

        #[allow(clippy::extra_unused_lifetimes)]
        fn output_static_lifetime_reference<'a>() -> &'static str {
            "hello" // &str literal has static lifetime, it's hardcoded into executable binary
        }
        assert_eq!(output_static_lifetime_reference(), "hello");
    }

    #[test]
    #[allow(dead_code)]
    fn borrow_type_with_lifetime_annotations() {
        #[derive(Debug)]
        struct Borrowed<'a, T>(&'a T);

        #[derive(Debug)]
        struct NamedBorrowed<'a, T> {
            x: &'a T,
            y: &'a T,
        }

        #[derive(Debug)]
        enum Either<'a, T> {
            Value(T),
            Ref(&'a T),
        }

        let x = 42;
        let single = Borrowed(&x);
        println!("x is borrowed in {:?}", single);

        let u = std::f64::consts::PI;
        let v = 4.13;
        let double = NamedBorrowed { x: &u, y: &v };
        println!("u and v is borrowed in {:?}", double);

        let y = "world".to_string();
        let reference = Either::Ref(&y);
        println!("y is borrowed in {:?}", reference);

        let value = Either::Value(y);
        println!("y is moved into {:?}", value);

        let s = "hello";
        let box_s = Box::new(s);
        let borrowed_str = Borrowed(&box_s);
        println!("&str {} is borrowed in {:?}", s, borrowed_str);
    }

    #[test]
    fn lifetime_in_nested_struct() {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct NoCopyType();

        #[derive(Debug)]
        #[allow(dead_code)]
        struct Both<'a, 'b> {
            x: &'a u32,
            y: &'b NoCopyType,
        }

        impl<'a, 'b> Both<'a, 'b> {
            fn get_ref_y(&'b self) -> &'b NoCopyType {
                self.y
            }

            fn get_ref_x(&'a self) -> &'a u32 {
                self.x
            }
        }

        let no_copy = NoCopyType();

        let both = Both {
            x: &42,
            y: &no_copy,
        };

        println!(
            "get ref of NoCopyType `{:?}` from struct Both `{:?}`",
            both.get_ref_y(),
            both
        );

        println!(
            "get ref of u32 `{}` from struct Both `{:?}",
            both.get_ref_x(),
            both
        );
    }

    #[test]
    fn reference_change() {
        let mut x = String::from("x");
        let mut y: String = String::from("y");

        // r, x are the same value
        let mut r = &mut x;
        assert_eq!(r, "x");

        // r reassign to y, x not effected
        r = &mut y;
        assert_eq!(x, "x");

        // r, y are the same value, assign *r will change y as well
        *r = String::from("z");
        assert_eq!(y, "z");
    }

    #[test]
    fn lifetime_of_ref_str_in_struct() {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        impl<'a> ImportantExcerpt<'a> {
            fn part(&'a self) -> &'a str {
                self.part
            }
        }

        impl ImportantExcerpt<'_> {
            #[allow(clippy::needless_lifetimes)]
            fn level<'a>(&'a self) -> i32 {
                3
            }
        }

        let part = "hello";
        let r = ImportantExcerpt { part };

        assert_eq!(r.level(), 3);
        assert_eq!(r.part(), "hello");
    }

    /*
     * Static lifetime means the value will outlive the entire lifetime of program.
     */
    #[test]
    fn static_lifetime() {
        struct ImportantExcerpt {
            part: &'static str,
        }

        impl ImportantExcerpt {
            fn part(&self) -> &'static str {
                self.part
            }
        }

        let excerpt = ImportantExcerpt {
            part: "static_part",
        };
        assert_eq!(excerpt.part(), "static_part");
    }

    /*
     * Lifetime elision: lifetimes have common case in which lifetime annotation
     * can be elided(omitted).
     */
    #[test]
    fn lifetime_without_elision() {
        #[allow(clippy::needless_lifetimes)]
        fn input<'a>(x: &'a i32) {
            println!("`annotated_input`: {}", x)
        }

        #[allow(clippy::needless_lifetimes)]
        fn pass<'a>(x: &'a i32) -> &'a i32 {
            x
        }

        #[allow(clippy::needless_lifetimes)]
        fn longest<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
            x
        }

        struct Owner(i32);

        impl Owner {
            #[allow(clippy::needless_lifetimes)]
            fn add_one<'a>(&'a mut self) {
                self.0 += 1;
            }

            #[allow(clippy::needless_lifetimes)]
            fn print<'a>(&'a self) {
                println!("`print:`: {}", self.0);
            }
        }

        let mut x = 42;
        x += 2;
        input(&x);
        let passed_x = pass(&x);
        assert_eq!(*passed_x, 44);
        x += 2;
        assert_eq!(x, 46);

        let mx = 41;
        let mut owner = Owner(mx);
        Owner::add_one(&mut owner);
        owner.print();
        // mx is copied to owner, so the value remains immutable 41;
        assert_eq!(mx, 41);

        let xx = "xx";
        let yy = "yy";
        let ret = longest(xx, yy);
        assert_eq!(ret, "xx");
    }

    /*
     * Three rules of compiler used to figure out lifetimes of references that
     * aren't explicit annotation:
     *   - Compiler assigns a lifetime parameter to each parameter that's a reference
     *   - If there is exactly one input lifetime parameter that lifetime is assigned to
     *     all output lifetime parameters.
     *   - If there are multiple lifetime parameters but one of them is &self or &mut self
     *     the lifetime of self if assigned to all output lifetime parameters.
     */
    #[test]
    fn lifetime_with_elision() {
        #[allow(clippy::needless_lifetimes)]
        fn _input<'a>(x: &'a i32) {
            println!("`annotated_input`: {}", x)
        }

        // Rule #1
        fn input(x: &i32) {
            println!("`annotated_input`: {}", x)
        }

        #[allow(clippy::needless_lifetimes)]
        fn _pass<'a>(x: &'a i32) -> &'a i32 {
            x
        }

        // Rule #1 and #2
        // #1 Assigns lifetime parameter for references
        // #2 Exactly one lifetime parameter, assigns to all output references
        fn pass(x: &i32) -> &i32 {
            x
        }

        #[allow(clippy::needless_lifetimes)]
        fn _longest<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
            x
        }

        // Rule #1
        // #1 Assigns another lifetime parameter for _y
        // There a multiple lifetime parameters, so the output must specify lifetime parameters
        fn longest<'a>(x: &'a str, _y: &str) -> &'a str {
            x
        }

        struct Owner(i32);

        impl Owner {
            // Rule #1
            fn add_one(&mut self) {
                self.0 += 1;
            }

            // Rule #1
            fn print(&self) {
                println!("`print:`: {}", self.0);
            }

            // Rule #1 #3
            // #3 Assign returned reference's lifetime as the lifetime of &mut self.
            fn add_self(&mut self, increment: &i32) -> &mut i32 {
                self.0 += *increment;
                &mut self.0
            }
        }

        let mut x = 42;
        x += 2;
        input(&x);
        let passed_x = pass(&x);
        assert_eq!(*passed_x, 44);
        x += 2;
        assert_eq!(x, 46);

        let mx = 41;
        let mut owner = Owner(mx);
        Owner::add_one(&mut owner);
        owner.print();
        assert_eq!(owner.0, 42);
        // mx is copied to owner, so the value remains immutable 41;
        assert_eq!(mx, 41);

        let increment = 42;
        let ref_owner = owner.add_self(&increment);
        assert_eq!(ref_owner, &84);

        let xx = "xx";
        let yy = "yy";
        let ret = longest(xx, yy);
        assert_eq!(ret, "xx");
    }

    #[test]
    fn compiler_annotate_lifetime() {
        fn ____first_world(s: &str) -> &str {
            s
        }

        #[allow(clippy::needless_lifetimes)]
        #[allow(mismatched_lifetime_syntaxes)]
        fn __first_world<'a>(s: &'a str) -> &str {
            s
        }

        #[allow(clippy::needless_lifetimes)]
        fn _first_world<'a>(s: &'a str) -> &'a str {
            s
        }
    }
}
