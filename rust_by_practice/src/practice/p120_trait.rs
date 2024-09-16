#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    /*
     * Derivable Traits
     * Trait that can be automatically implemented for a struct or an enum by the Rust compiler
     * Called "derivable" because they can be derived automatically
     * Most common derivable traits:
     *   - Debug: Allowing to output content via "{:?}"
     *   - Clone: Enables type to be duplicated with "clone()" method
     *   - Copy: Enables type to be copied implicitly, without requiring explicit "clone()" method
     *   - PartialEq: Enables comparison
     */
    #[test]
    fn derive_trait() {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let p = Point { x: 3.1, y: 4.1 };
        let q = p; // auto copy
        assert_eq!(p, q);

        fn use_clone_eq<T: Clone + PartialEq + std::fmt::Debug>(a: &T, b: &T) {
            assert_eq!(a.clone(), b.clone())
        }
        use_clone_eq(&p, &q);
    }

    // where clauses for heavy use of trait bounds
    #[test]
    fn where_clause_for_heavy_trait_bound() {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct Point<T> {
            x: T,
            y: T,
        }

        fn use_clone_eq<T, U>(a: &T, b: &U)
        where
            T: Clone + PartialEq + Debug,
            U: Clone + Debug,
        {
            assert_eq!(*a, a.clone());
            println!("b = {:?}", b.clone());
        }

        let p = Point { x: 'a', y: 'b' };
        let q = Point {
            x: "hello",
            y: "world",
        };
        use_clone_eq(&p, &q);
    }

    // trait as return type
    // impl Animal is called opaque type, and is only allow in arguments and return types of functions and methods
    #[test]
    fn trait_as_return_type() {
        trait Animal {
            fn new() -> impl Animal;
        }

        struct Cat;
        struct Dog;

        impl Animal for Cat {
            fn new() -> impl Animal {
                Cat
            }
        }
        impl Animal for Dog {
            fn new() -> impl Animal {
                Dog
            }
        }

        let _dog = Cat::new();
        let _cat = Dog::new();
    }

    // trait with default method
    #[test]
    fn trait_with_default_method() {
        trait Hello {
            fn say_hello(&self) -> String {
                "hello".to_string()
            }

            fn say_something(&self) -> String;
        }

        struct Student;
        struct Teacher;

        impl Hello for Student {
            fn say_something(&self) -> String {
                "I'm a bad student!".to_string()
            }
        }

        impl Hello for Teacher {
            fn say_hello(&self) -> String {
                "greeting".to_string()
            }

            fn say_something(&self) -> String {
                "I'm not a teacher!".to_string()
            }
        }

        let s = Student;
        assert_eq!(s.say_hello(), "hello");
        assert_eq!(s.say_something(), "I'm a bad student!");

        let t = Teacher;
        assert_eq!(t.say_hello(), "greeting");
        assert_eq!(t.say_something(), "I'm not a teacher!");
    }

    // derive PartialEq, PartialOrd
    #[test]
    fn derive_traits_for_compare() {
        #[derive(Debug, PartialEq, PartialOrd)]
        struct Seconds(u32);

        assert_eq!(Seconds(10), Seconds(10));
        assert!(Seconds(10) == Seconds(10));
        assert!(Seconds(20) > Seconds(10));
    }

    // operator trait
    // associated type in trait
    #[test]
    fn traits_for_operator_overriding() {
        #[derive(Debug, PartialEq)]
        struct Centimeters(f64);
        #[derive(Debug, PartialEq)]
        struct SquareCentimeters(f64);

        impl std::ops::Mul for Centimeters {
            type Output = SquareCentimeters; // associated type
            fn mul(self, rhs: Self) -> SquareCentimeters {
                SquareCentimeters(self.0 * rhs.0)
            }
        }

        assert_eq!(Centimeters(5.0) * Centimeters(6.0), SquareCentimeters(30.0));
    }

    // operator trait with different type of right hand side
    #[test]
    fn implement_trait_add_with_associated_type() {
        struct Foo;
        struct Bar;

        #[derive(Debug, PartialEq)]
        struct FooBar;
        #[derive(Debug, PartialEq)]
        struct BarFoo;

        impl std::ops::Add<Bar> for Foo {
            type Output = FooBar;
            fn add(self, _rhs: Bar) -> Self::Output {
                FooBar
            }
        }

        impl std::ops::Sub<Bar> for Foo {
            type Output = BarFoo;
            fn sub(self, _rhs: Bar) -> Self::Output {
                BarFoo
            }
        }

        assert_eq!(Foo + Bar, FooBar);
        assert_eq!(Foo - Bar, BarFoo);
    }

    /*
     * Dynamic Dispatch (&dyn Trait, Box<dyn Trait> )
     *   - Specific methods to be called is determined at runtime
     *   - Works by creating a reference(`&dyn`) or smart pointer(`Box<dyn >`) to a trait object
     *   - Compiler builds a `vtable` for a trait when a trait instance object is created
     *   - `vtable` contains a pointer to the implementation of each method in the trait for the specific type of the object
     *   - Compiler will do a lookup in a vtable to determine which method should be called for which type that implements the given trait
     *   - This lookup will cause overhead but allows for more flexible code
     */
    // dynamic trait object for return unknown-size trait instance
    // &dyn has ownership
    // &dyn T vs &T (Box<dyn T> vs Box<T>), &dyn T, Box<dyn T> is also called fat pointer
    // &dyn T has a `ptr` point to instance and a `vptr` point to the corresponding `vtable`
    // &T has a `ptr` point to instance in heap
    #[test]
    fn dynamic_dispatch_of_trait_implementation() {
        trait Animal {
            fn sound(&self) -> String;
        }

        struct Sheep;
        struct Cow;

        impl Animal for Sheep {
            fn sound(&self) -> String {
                "Maah".to_string()
            }
        }

        impl Animal for Cow {
            fn sound(&self) -> String {
                "Mooh".to_string()
            }
        }

        fn new_animal(species: &str) -> &dyn Animal {
            match species {
                "sheep" => &Sheep,
                "cow" => &Cow,
                _ => panic!("unknown animal"),
            }
        }
        fn bark(animal: &(impl Animal + ?Sized)) {
            println!("barking: {}", animal.sound());
        }

        let animals: [&dyn Animal; 3] =
            [new_animal("sheep"), new_animal("cow"), new_animal("sheep")];
        for animal in animals {
            bark(animal);
        }

        // return &dyn Animal value moved to _cow
        let mut _cow = new_animal("cow");
        // validate that _cow move to _cow_moved
        let _cow_moved = _cow;
    }

    // dynamic dispatch by using Box<dyn >
    #[test]
    fn dynamic_dispatch_by_using_box_dyn() {
        trait Animal {
            fn sound(&self) -> String;
        }

        struct Sheep;
        struct Cow;

        impl Animal for Sheep {
            fn sound(&self) -> String {
                "Maah".to_string()
            }
        }

        impl Animal for Cow {
            fn sound(&self) -> String {
                "Mooh".to_string()
            }
        }

        fn new_animal(a: &str) -> Box<dyn Animal> {
            match a {
                "sheep" => Box::new(Sheep),
                "cow" => Box::new(Cow),
                _ => panic!("unknown animal"),
            }
        }

        let animals = [new_animal("sheep"), new_animal("sheep"), new_animal("cow")];
        for animal in animals {
            println!("unbox dyn Animal sound: {}", animal.sound())
        }
    }

    /*
     * | ----------|-----------------------------------|------------------------------------------|
     * |           | & (reference)                     | Box                                      |
     * | ----------|-----------------------------------|------------------------------------------|
     * | Memory    | Only points to a value already in | Allocates data on heap and owns it, also |
     * |           | memory.                           | responsible for deallocating when values |
     * |           |                                   | goes out of scope.                       |
     * | ----------|-----------------------------------|------------------------------------------|
     * | Lifetime  | Limited                           | Can be passed across scopes              |
     * | ----------|-----------------------------------|------------------------------------------|
     * | Clonable  | No                                | Yes                                      |
     * | ----------|-----------------------------------|------------------------------------------|
     * | Pattern   | No                                | Yes                                      |
     * | ----------|-----------------------------------|------------------------------------------|
     */

    // generic, impl for specific trait
    #[test]
    fn implement_generic_type_with_trait_bound() {
        struct Pair<T> {
            x: T,
            y: T,
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        impl<T: std::fmt::Debug + PartialEq + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The larger member is x = {:?}", self.x);
                } else {
                    println!("The larger member is y = {:?}", self.y);
                }
            }
        }

        #[derive(Debug, PartialEq, PartialOrd)]
        struct Unit(f32);

        let pair = Pair::new(Unit(3.4), Unit(4.3));
        pair.cmp_display();
    }

    // trait with associated type, std::ops::Add is a example
    #[test]
    fn trait_with_associated_type() {
        trait Union<Rhs> {
            type Output;
            fn union(&self, rhs: &Rhs) -> Self::Output;
        }

        #[derive(Debug)]
        struct Point<T>(T, T);
        #[derive(Debug, PartialEq)]
        struct MixedPoint<T, U>(T, U, T, U);

        impl<T: Copy, U: Copy> Union<Point<U>> for Point<T> {
            type Output = MixedPoint<T, U>;

            fn union(&self, rhs: &Point<U>) -> Self::Output {
                MixedPoint(self.0, rhs.0, self.1, rhs.1)
            }
        }

        let p = Point(1, 2);
        let q = Point('a', 'b');
        let r = p.union(&q);
        assert_eq!(r, MixedPoint(1, 'a', 2, 'b'));
    }

    // implements trait for built-in type, with &dyn, Box<dyn > and static dispatch generic
    #[test]
    fn implement_trait_for_built_in_type() {
        trait Draw {
            fn draw(&self) -> String;
        }

        impl Draw for i8 {
            fn draw(&self) -> String {
                format!("i8: {}", self)
            }
        }

        impl Draw for f32 {
            fn draw(&self) -> String {
                format!("f32: {}", self)
            }
        }

        fn draw_with_ref(d: &dyn Draw) {
            println!("{}", d.draw());
        }

        fn draw_with_box(d: Box<dyn Draw>) {
            println!("{}", d.draw())
        }

        fn draw_with_static_generic<T: Draw>(d: T) {
            println!("{}", d.draw())
        }

        let x = 42_i8;
        draw_with_ref(&x);

        let y = 3.2_f32;
        draw_with_box(Box::new(y));

        draw_with_static_generic(y);
    }

    // Object-safe trait:
    //   - The return type isn't self.
    //   - There are no generic type parameters.
    #[allow(dead_code)]
    #[test]
    fn object_save_trait() {
        trait SelfUnsafeTrait {
            fn f(&self) -> Self;
        }

        trait GenericUnsafeTrait<T> {
            fn f(&self) -> T;
        }
        trait ObjectSaveTrait {
            fn f(&self) -> &dyn ObjectSaveTrait;
            fn b(&self) -> Box<dyn ObjectSaveTrait>;
        }
    }
}
