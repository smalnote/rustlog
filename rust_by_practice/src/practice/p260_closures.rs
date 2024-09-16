#[cfg(test)]
mod tests {
    /*
     * Closures:
     *   - Anonymous functions that are able to capture values from the scope in which they are defined
     *   - Can be defined inline(from example as function parameter)
     *   - Don't require type annotations
     *   - Can take ownership of value by using `move` keyword
     *
     * Capture types:
     *   - By reference: &T
     *   - By mutable reference: &mut T
     *   - By value: T
     */

    // `|x|` is pronounced `pipe x pipe`
    #[test]
    fn capture_with_least_restrictive_manner() {
        let x = 1;
        let closure = |v| v + x;
        assert_eq!(closure(2), 3);
        // by the least restrictive manner, a immutable reference of x is captured
        // rather than taking ownership of x
        let closure_annotated = |v: i32| -> i32 { v + x };
        assert_eq!(closure_annotated(2), 3);
    }

    #[test]
    fn closure_without_parameters() {
        let x = 42;
        let final_answer = || -> i32 { x };
        let answer = || x;
        assert_eq!(final_answer(), 42);
        assert_eq!(answer(), 42);
    }

    #[test]
    fn closure_capture_types() {
        let color = "green".to_string();
        let print = || println!("`color`: {}", color);

        print();
        print();

        let reborrow = &color;
        println!("reborrowed `color`: {}", reborrow);
    }

    #[test]
    fn capture_value_with_move_of_i32() {
        let mut count: i32 = 0;

        // `move` move the value count to the closure
        // in this case, a copy of count, for type i32 implements trait Copy
        // count is moved into inc, but it only requires a &mut count
        // so the closure `inc` impl FnMut
        let mut inc = move || {
            count += 1;
            println!("`count`: {}", count);
        };

        inc();

        let _reborrow = &count;

        inc();

        let _count_reborrowed = &mut count;

        assert_eq!(count, 0);
    }

    #[test]
    fn capture_value_of_custom_type() {
        struct Number(i32);

        let mut number = Number(0);

        // capture value moved here due to the keyword `move`
        let mut inc = move || {
            let n = &mut number;
            n.0 += 1;
            println!("`count`: {}", n.0);
        };

        inc();

        // error: value borrowed here after move
        // let _reborrow = &number;

        inc();
    }

    #[test]
    fn capture_value_of_custom_type_with_copy_trait() {
        #[derive(Clone, Copy)]
        struct Number(i32);

        let mut number = Number(0);

        let mut inc = move || {
            let n = &mut number; // capture value copy here with the copy trait
            n.0 += 1;
            println!("`count`: {}", n.0);
        };

        inc();

        // value borrow here is available, because the closure got of copy of number
        let _reborrow = &number;

        inc();

        assert_eq!(number.0, 0);
    }

    #[test]
    fn capture_value_without_trait_copy() -> Result<(), std::num::ParseIntError> {
        struct Number {
            count: String,
        }

        let mut number = Number {
            count: "0".to_string(),
        };

        let mut inc = move || -> Result<(), std::num::ParseIntError> {
            // value partial moved here
            let count = number.count.parse::<i32>()?;
            number.count = (count + 1).to_string();
            println!("`count`: {}", number.count);
            Ok(())
        };

        inc()?;

        // error: value borrowed here after partial move
        // let _reborrow = &number;

        inc()
    }

    #[test]
    fn movable_copy() {
        fn take<T>(_v: &T) {}

        let movable = Box::new(42);

        let consume = || {
            println!("`movable`: {}", movable);
            // movable is i32, which implements trait Copy, capture copy movable and moved the copy by calling take(movable)
            take(&movable);
        };

        consume();
        consume();

        assert_eq!(*movable, 42);
    }

    #[test]
    fn closure_pass_ownership() {
        let pass = |x| x;

        // this calling of closure `pass` infers the type parameter `x` to be `String`
        let _s = pass(String::from("hello"));

        // must convert `3` to string to comfort parameter type `x` of closure pass
        let _n = pass(3.to_string());
    }

    /*
     * Fn Traits:
     *   - Trait that defines signatures fro closures/functions
     *   - Describe types, number of arguments and return types
     *   - Three difference traits:
     *     - FnOnce: value captured by value (T)
     *       - Closure that can be called once
     *       - Take ownership of captured values
     *     - FnMut: value captured by mutable reference (&mut T)
     *       - Can be called more than once
     *       - Might mutable captured values
     *     - Fn: value captured by immutable reference (&T)
     *       - Doesn't take ownership of captured values
     *       - Doesn't mutate anything
     *       - Might not even capture anything from its environment
     */
    #[test]
    fn closure_trait() {
        fn call_fn<F>(is_equal: F)
        where
            F: Fn(usize) -> bool,
        {
            println!("{}", is_equal(3));
            println!("{}", is_equal(4));
        }

        let x = [1, 2, 3];
        call_fn(|z| z == x.len());
    }

    #[test]
    fn closure_trait_with_lifetime_annotation() {
        let mut s = String::new();

        let append_s = |str| s.push_str(str);

        fn execute_closure_with_hello<'a, F: FnMut(&'a str)>(mut append: F) {
            /*
             * The lifetime annotation `'a` indicates that closure trait F
             * must be able to accept a reference with the lifetime that the
             * function `execute_closure_with_hello` lives.
             *
             * In this case: string slice "hello" lives as long as the function
             * `execute_closure_with_hello`, so it can be passed to append: F.
             */
            append("hello");
        }

        /*
         * If omit the lifetime annotation:
         *   fn execute_closure_with_hello<F: FnMut(&str)>(mut append: F)
         * Result in compilation error:
         *   execute_closure_with_hello(append_s);
         *   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `FnMut` is not general enough
         * = note: closure with signature `fn(&'2 str)` must implement `FnMut<(&'1 str,)>`, for any lifetime `'1`...
         * = note: ...but it actually implements `FnMut<(&'2 str,)>`, for some specific lifetime `'2`
         *
         * `append_s` accept string slice of lifetime outlives function `execute_closure_with_hello`.
         */
        execute_closure_with_hello(append_s);

        assert_eq!(s, "hello");
    }

    /*
     * How the closure capture value is based on how the captured values are used in the closure,
     * and compiler will ultimately choose &T, &mut T or T in the least restrictive manner possible.
     * Annotated closure trait doesn't change the capturing behavior, but constraint the available capture behavior.
     * |------------|----------------------------|
     * | Annotated  | Available captures         |
     * |------------|----------------------------|
     * | `FnOnce`   | &T, &mut T, T              |
     * |------------|----------------------------|
     * | `FnMut`    | &T &mut T                  |
     * |------------|----------------------------|
     * | `Fn`       | &T                         |
     * |------------|----------------------------|
     */

    #[test]
    fn annotate_closure_trait() {
        fn apply<F>(f: F)
        where
            F: FnOnce(),
        {
            f();
        }

        fn apply_to_3<F>(f: F) -> i32
        where
            F: Fn(i32) -> i32,
        {
            f(3)
        }

        use std::mem;

        let greeting = "hello";
        let mut farewell = "goodbye".to_owned();

        let diary = || {
            // captured by immutable reference &T: &greeting, requires Fn
            println!("I said {}.", greeting);

            // captured by mutable reference &mut T: &mut farewell, requires FnMut
            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzz");

            // captured by value T: farewell, requires FnOnce
            mem::drop(farewell);
        };

        apply(diary);

        let double = |x| x * 2;
        assert_eq!(apply_to_3(double), 6);
    }

    /*
     * Move closure(FnOnce) may still implements trait FnMut or Fn, even though
     * they capture variables by move. This is because the traits implemented by
     * a closure type are determined by what the closure does with captured
     * values, not how it captures them. The `move` keyword only specifies the
     * latter.
     */

    #[test]
    fn capture_trait_not_depends_on_move() {
        fn apply_fn_once<F: FnOnce()>(f: F) {
            f();
        }

        fn apply_fn_mut<F: FnMut()>(f: &mut F) {
            f();
        }

        fn apply_fn<F: Fn()>(f: &F) {
            f();
        }

        /*
         * The closure `move || println!("{}", s)`
         * captures variable `s` by move, but how does it doing with `s` is
         * just borrow immutable reference, so it alow implements traits
         * `FnMut()` and `Fn()`
         */

        let s1 = String::new();
        let print_string_1 = move || println!("{}", s1);
        apply_fn_once(print_string_1);

        // `move closure` implements trait `FnMut()`
        let s2 = String::new();
        let mut print_string_2 = move || println!("{}", s2);
        apply_fn_mut(&mut print_string_2);

        // `move closure` implements trait `Fn()`
        let s3 = String::new();
        let print_string_3 = move || println!("{}", s3);
        apply_fn(&print_string_3);

        // But if the move closure use the mutable reference value, the it only
        // implements  traits `FnOnce` and `FnMut`
        let mut s4 = String::new();
        let append_string_4 = move || s4.push_str("mutated");
        apply_fn_once(append_string_4);

        let mut s5 = String::new();
        let mut append_string_5 = move || s5.push_str("mutated");
        apply_fn_mut(&mut append_string_5);

        // Buf it the move closure move the value, then it only implements trait `FnOnce`
        let s6 = String::new();
        let move_string_s6 = move || std::mem::drop(s6);
        apply_fn_once(move_string_s6);
    }

    #[test]
    fn decide_trait_bound() {
        let mut s = String::new();

        let update_and_return = |str| -> String {
            s.push_str(str);
            s
        };

        fn exec<'a, F: FnOnce(&'a str) -> String>(f: F) -> String {
            f("hello")
        }

        assert_eq!(exec(update_and_return), "hello");
    }

    #[test]
    fn function_can_use_as_arguments() {
        fn call<F: Fn()>(f: F) {
            f();
        }

        fn function() {
            println!("I'm a function!");
        }

        let closure = || println!("I'm a closure!");

        call(closure);
        call(function);
    }

    #[test]
    fn closure_as_return_type() {
        // Return type: `impl (Fn(i32) -> i32)` uses static dispatch, note that
        // `impl Trait` is a syntactic sugar of generic `<T: Trait>`
        // But if we use: fn create_closure<F: (Fn(i32) -> i32)>() -> F
        // The compiler will complain:
        //   *every closure has a distinct type* and so could not always match the
        //   caller-chosen type of parameter `F`
        fn create_closure() -> impl (Fn(i32) -> i32) {
            let num = 5;

            // How does the following closure capture the environment variable `num`
            // &T, &mut T, T
            // Answer: Here we return the closure, meaning `num` must outlive current function
            // so capture move value is required, use keyword `move` to force capture value.
            // For the trait bound of returning type, the closure only does with the immutable
            // reference of num, this decides the closure implements `Fn(i32) -> i32`,
            // as well as `FnMut(i32) -> i32` and `FnOnce(i32) -> i32`, here we choose
            // the lease restrictive one, `Fn(i32) -> i32`
            move |x| x + num
        }
        let adder = create_closure();
        assert_eq!(adder(5), 10);

        // Return type: `Box<dyn (Fn(i32) -> i32)>` using dynamic dispatch.
        fn create_closure2() -> Box<dyn (Fn(i32) -> i32)> {
            let num = 5;

            // How does the following closure capture the environment variable `num`
            // &T, &mut T, T
            Box::new(move |x| x + num)
        }
        let adder = create_closure2();
        assert_eq!(adder(5), 10);
    }

    /*
    * #1: fn create_closure<F: (Fn(i32) -> i32)>() -> F {
    *         |x| x + 1
    *     }
    *   (compilation error)
        This doesn't work because the F is as the return type,
        compiler has no information about which specific type
        implementing trait `Fn(i32) -> i32` it should return.
        So compiler cannot generate a concrete type function
        at compile time.
        Caller Responsibility: While generic functions often allow the caller
        to specify the concrete type, in this scenario, the caller has no means
        to indicate the desired type for F.
    * #2: fn create_closure() -> impl (Fn(i32) -> i32) {
    *         |x| x + 1
    *     }
    *   (ok)
        `impl Fn(i32) -> i32`: This signifies that the function will return some
        concrete type that implements the Fn(i32) -> i32 trait.
        Compiler Knowledge: While the caller doesn't know the exact type, the
        compiler does.
        Closure Definition: The closure |x| x + 1 has a specific, concrete type
        known to the compiler.
        No Ambiguity: Since the function body provides a concrete implementation,
        there's no ambiguity, and the compiler can generate the necessary code.

    * While `impl Trait` is a syntactic sugar of `<T: Trait>`, `impl Trait` works
    * slightly differs when it is used as return type. `impl Trait` allow the
    * function itself to specify the concrete return type, while the `<T: Trait>`
    * depending on the calling instance to determine its concrete type.
    *
    * But by the  calling `create_closure()`, compiler is impossible to infer return type.
    * The `impl Trait` in return position tells compiler infers return type from
    * the function implementation itself.
    *
    * While impl Trait can sometimes act as syntactic sugar for generics, their
    * usage in function return positions serves different purposes. In the context of these snippets:
    *
    * Snippet #1 fails because the compiler lacks information to determine the
    *   concrete type for F.
    * Snippet #2 succeeds because the function provides a specific implementation
    *   that the compiler can handle, abstracted away from the caller using impl Trait.
    */

    #[test]
    fn every_closure_has_a_unique_type() {
        fn factory(x: i32) -> Box<dyn (Fn(i32) -> i32)> {
            let num = 5;
            if x > 1 {
                Box::new(move |x| x + num)
            } else {
                Box::new(move |x| x + num)
            }
        }

        let adder: Box<dyn Fn(i32) -> i32> = factory(2);
        assert_eq!(adder(3), 8);

        /*
         * Another way of dynamic dispatch `&dyn` is impossible for returning local reference.

        fn factory2(x: i32) -> &dyn (Fn(i32) -> i32) {
            let num = 5;
            if x > 1 {
                &(move |x| x + num)
                ^ returns a value referencing data owned by the current function
            } else {
                &(move |x| x + num)
                ^ returns a value referencing data owned by the current function
            }
        }
        */
    }
}
