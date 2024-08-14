#[cfg(test)]
mod tests {
    /*
     * Static Lifetime:
     *   - Refers to a lifetime that last for the entire duration of the program's
     *     execution
     *   - Any reference or borrowed value with static lifetime can be safely used
     *     throughout the program
     *   - Can be coerced to a shorter lifetime if needed
     *
     * String literals have a static lifetime because they are hardcoded into the
     * executable library.
     */

    // &'static and T: 'static
    #[test]
    fn static_ref_and_static_trait_bound() {
        let s: &'static str = "hello";

        fn static_trait_bound<T: 'static + ?Sized>(_s: &'static T) {}

        static_trait_bound(s);
    }

    #[test]
    fn static_str() {
        fn need_static(r: &'static str) {
            assert_eq!(r, "hello");
        }

        let v = "hello";
        need_static(v);

        let vv: &'static str = "hello";
        need_static(vv)
    }

    #[test]
    fn make_static_lifetime_by_box_leak() {
        static mut _CONFIG_NONE: Option<&mut Config> = Option::None;

        #[derive(Debug)]
        #[allow(dead_code)]
        struct Config {
            a: String,
            b: String,
        }

        fn init() -> Option<&'static mut Config> {
            let box_config = Box::new(Config {
                a: "A".to_string(),
                b: "B".to_string(),
            });
            Some(Box::leak(box_config))
        }

        let _config: Option<&'static mut Config> = init();
    }

    /*
     * &'static only indicates that the value can live throughout the program
     * running. The reference will be constrained by its scope.
     */
    #[test]
    fn static_reference_is_constrained() {
        let outer_string: &str;

        {
            let static_string = "I'm in read-only memory";
            println!("static_string: {}", static_string);

            outer_string = static_string;
        }
        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
        // So the borrowed `outer_string` can outlive scope of `static_string`.

        println!("outer_string: {}", outer_string);
    }

    #[test]
    fn coerced_static_lifetime_to_shorter_lifetime() {
        fn need_shorter<'a>(s: &'a str) {
            assert_eq!(s, "hello");
        }

        let s: &'static str = "hello";
        need_shorter(s);

        static NUM: i32 = 42;
        fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
            &NUM
        }

        {
            let shorter_lifetime_num = 6;
            let v = coerce_static(&shorter_lifetime_num);
            assert_eq!(v, &42);
        }

        println!("NUM: {} stays accessible!", NUM);
    }

    #[test]
    fn static_lifetime_trait_bound() {
        use std::fmt::Debug;

        fn print_it1<T: Debug + 'static>(input: T) {
            println!("'static value passed in is: {:?}", input);
        }

        // exactly the same thing as print_it1
        fn print_it2(input: impl Debug + 'static) {
            println!("'static value passed in is: {:?}", input);
        }
        fn print_it3<T: Debug + 'static>(input: &T) {
            println!("'static value passed in is: {:?}", input);
        }

        // i is owned and contains no references, thus it's 'static:
        let i = 42;
        print_it1(i);
        print_it2(i);

        // &i only has the lifetime defined by the scope of current function,
        // so it's not static
        // print_it1(&i);
        // print_it2(&i);

        // print_it3<T: Debug + 'static>(input: &T)
        // print_it3 requires T has static lifetime and accept type &T
        // here `i` has static lifetime and we passed `&i`, T -> i
        print_it3(&i);

        // constant and constant reference has static lifetime
        const N: i32 = 84;
        print_it1(N);
        print_it1(N);
        print_it1(&N);
        print_it2(&N);
        print_it3(&N);

        // static variable and static variable reference has static lifetime
        static M: i32 = 168;
        print_it1(M);
        print_it1(M);
        print_it1(&M);
        print_it2(&M);
        print_it3(&M);
    }

    #[test]
    fn static_lifetime_and_box() {
        use std::fmt::Display;

        fn print_a<T: Display + 'static>(t: &T) {
            println!("{}", t);
        }

        fn print_b<T>(t: &T)
        where
            T: Display + 'static,
        {
            println!("{}", t);
        }

        fn print_c(t: &'static dyn Display) {
            println!("{}", t)
        }

        fn print_d(t: &'static impl Display) {
            println!("{}", t)
        }

        fn print_e(t: &(dyn Display + 'static)) {
            println!("{}", t)
        }

        fn print_f(t: &(impl Display + 'static)) {
            println!("{}", t)
        }

        fn print_g(t: &'static String) {
            println!("{}", t);
        }

        let mut string = "First".to_owned();
        string.push_str(string.to_uppercase().as_str());

        // fn print_a<T: Display + 'static>(t: &T)
        print_a(&string);
        // fn print_b<T>(t: &T)
        // where
        //     T: Display + 'static,
        print_b(&string);

        // fn print_c(t: &'static dyn Display)
        let dyn_str: &dyn Display = Box::leak(Box::new(string.clone()));
        print_c(dyn_str); // Compilation error

        let static_str = Box::leak(Box::new(string.clone()));
        // fn print_d(t: &'static impl Display)
        print_d(static_str); // Compilation error

        // fn print_e(t: &(dyn Display + 'static))
        print_e(&string);
        // print_f(t: &(impl Display + 'static))
        print_f(&string);

        // print_g(t: &'static String)
        print_g(static_str); // Compilation error
    }
}
