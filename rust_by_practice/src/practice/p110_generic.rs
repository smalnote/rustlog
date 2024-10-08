#[cfg(test)]
mod tests {
    // struct method and associated function
    #[test]
    fn struct_method_and_associated_function() {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        // impl block defines type struct methods and associated functions
        impl Rectangle {
            // associated function, no parameter `self`, reference to type struct
            pub fn new(width: u32, height: u32) -> Rectangle {
                Rectangle { width, height }
            }

            // use `Self` as synonym of struct Rectangle
            fn square(size: u32) -> Self {
                Self {
                    width: size,
                    height: size,
                }
            }

            // method, consume parameter `&self` to borrow type struct instance
            pub fn area(&self) -> u32 {
                self.width * self.height
            }

            // parameter `self` move ownership of instance
            fn clear(self) {}

            // parameter `&mut self` borrow mutable instance
            fn rotate(&mut self) {
                std::mem::swap(&mut self.width, &mut self.height);
            }

            // self must be the first paramter
            fn zoom(&mut self, ratio: f32) {
                self.width = (self.width as f32 * ratio) as u32;
                self.height = (self.height as f32 * ratio) as u32;
            }

            // canonical form of self parameter
            fn normalize(&mut self) {
                if self.width > self.height {
                    self.height = self.width;
                } else {
                    self.width = self.height;
                }
            }
        }

        let mut rect = Rectangle::new(30_u32, 50_u32);
        assert_eq!(rect.area(), 1500);

        rect.rotate();
        assert_eq!(rect.width, 50);
        assert_eq!(rect.height, 30);

        rect.zoom(2.2);
        assert_eq!(rect.width, 110);
        assert_eq!(rect.height, 66);

        rect.normalize();
        assert_eq!(rect.width, 110);
        assert_eq!(rect.height, 110);

        rect.clear(); // value ownership moved

        let square = Rectangle::square(10);
        assert_eq!(square.area(), 100);
    }

    // can have multiple impl block for a same type
    #[test]
    fn struct_with_multiple_impl_blocks() {
        struct TrafficLight {
            color: String,
        }

        impl TrafficLight {
            pub fn new() -> Self {
                Self {
                    color: "red".to_string(),
                }
            }
        }

        impl TrafficLight {
            pub fn get_state(&self) -> &str {
                self.color.as_str()
            }
        }

        let light = TrafficLight::new();
        assert_eq!(light.get_state(), "red")
    }

    // implement method and associated functions for type enum
    #[allow(dead_code)]
    #[test]
    fn enum_method_and_associated_functions() {
        enum TrafficLight {
            Red,
            Green,
            Yellow,
        }

        impl TrafficLight {
            pub fn new(color: &str) -> TrafficLight {
                match color {
                    "red" => TrafficLight::Red,
                    "green" => TrafficLight::Green,
                    "yellow" => TrafficLight::Yellow,
                    _ => panic!("unknown color `{}`", color),
                }
            }
            pub fn color(&self) -> &str {
                match self {
                    TrafficLight::Green => "green",
                    TrafficLight::Red => "red",
                    Self::Yellow => "yellow", // can also use `Self` to reference type to implement
                }
            }
        }

        let c = TrafficLight::Yellow;
        assert_eq!(c.color(), "yellow");

        let y = TrafficLight::new(c.color());
        assert!(matches!(y, TrafficLight::Yellow));
    }

    // generic function
    // use turbofish `::<T>` to explicitly specify type parameter for a generic type or function.
    #[test]
    fn generic_function() {
        struct A; // type A is a unit type
        struct S(A); // type S is a unary tuple of A, `(A)`
        struct SGen<T>(T); // type SGen is a unary tuple of abitary type

        fn reg_fn(_s: S) {}
        fn gen_spec_t(_s: SGen<A>) {}
        fn gen_spec_i32(_s: SGen<i32>) {}
        fn generic<T>(_s: SGen<T>) {}

        reg_fn(S(A));
        gen_spec_t(SGen(A));
        gen_spec_i32(SGen(42_i32));

        // specify type parameter explicitly by using ::<T> of generic function
        generic::<char>(SGen('A'));

        // specify type parameter implicitly by passing concrete type argument
        generic(SGen('C'));

        // specify struct type parameter by using ::<T> of generic struct
        generic(SGen::<u32>(42));
    }

    // generic with trait bound
    #[test]
    fn generic_with_trait_bound() {
        fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
            a + b
        }

        assert_eq!(2, sum(1_i8, 1_i8));
        assert_eq!(49, sum(42, 7));
        assert_eq!(3.27, sum(3.0, 0.27));
    }

    // generic struct
    #[test]
    fn generic_struct() {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer: Point<i32> = Point { x: 5, y: 10 };
        let float: Point<f64> = Point { x: 1.0, y: 4.0 };

        println!("integer point = {integer:?}");
        println!("float point = {float:?}");
    }

    // generic struct with two type parameter
    #[allow(dead_code)]
    #[test]
    fn generic_struct_with_two_type_parameters() {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let _p = Point {
            x: 32,
            y: "hello".to_string(),
        };
    }

    // generic struct with method return value reference
    #[test]
    fn impl_struct_with_generic_type_parameter() {
        struct Val<T> {
            val: T,
        }

        impl<T> Val<T> {
            fn value(&self) -> &T {
                &self.val
            }
        }

        let i = Val { val: 42 };
        let s = Val {
            val: "hello".to_string(),
        };

        assert_eq!(*i.value(), 42);
        assert_eq!(s.value(), "hello");
    }

    // generic struct with method that has generic type parameters
    #[test]
    fn generic_struct_with_method_specific_generic_type_parameters() {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 1, y: 2 };
        let p2 = Point {
            x: "hello",
            y: '中',
        };

        let p3 = p1.mixup(p2);
        assert_eq!(p3.x, 1);
        assert_eq!(p3.y, '中');
    }

    // generic struct with specified type implementation
    #[test]
    fn impl_concrete_type_for_generic_struct() {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl Point<f64> {
            fn distance_from_origin(&self) -> f64 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let p = Point { x: 3.0, y: 4.0 };
        assert_eq!(p.distance_from_origin(), 5.0);
    }

    // array element type and length is part of array type
    // use const as generic parameter
    #[test]
    fn array_element_type_and_length_is_part_of_array_type() {
        #[allow(dead_code)]
        struct Array<T, const N: usize> {
            data: [T; N],
        }

        let _a: Array<i32, 3> = Array { data: [1, 2, 3] };
        let _b = Array::<u32, 3> { data: [1, 2, 3] };
        let _c = Array::<u32, 10> {
            data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
        };

        let _arrays: [Array<i32, 3>; 3] = [
            Array { data: [1, 2, 3] },
            Array { data: [1, 2, 3] },
            Array { data: [1, 2, 3] },
        ];
    }

    // trait, trait as parameter type, trait bound
    #[test]
    fn impl_trait_for_struct() {
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

        fn bark_static<T: Animal>(animal: &T) {
            println!("barking: {}", animal.sound());
        }

        /*
         * Syntax `impl Trait` is syntactic sugar for generic function with a trait bound.
         * `t: impl Trait` is equivalent to <T: Trait> (t: T) and is static dispatch.
         * By means static dispatch, the Rust compiler will generate a concrete type version
         * when a function call occurs for a concrete type.
         * For example:
         *   for function:
         *     fn bark(animal: &impl Animal)
         *   at compile time, Rust compiler will generate concrete function for type Sheep and type Cow:
         *     fn bark(animal: &Sheep)
         *     fn bark(animal: &Cow)
         *   Just like concrete version of fn bark<T: Animal>(animal: &T) -> fn bark(animal: &Sheep)
         *
         * In contrast to static dispatch, `dyn Trait` is for dynamic dispatch:
         *   for function:
         *     fn bark(animal: &dyn Animal)
         *   at runtime, call of this function with &Sheep and &Cow, in addition to the reference of value, it will carry
         *   a dynamic pointer to the concrete type, so that it can decide to call method of which type.
         *     for: let s = Sheep;
         *     calling: bark(&s)
         *     will carry a reference to `s` and a pointer to type Sheep, so it knows to use type Sheep's impl for trait Animal.
         *
         * In short: `impl Trait` is a trait bound for generic, `dyn Trait` is a trait object for allowing dynamic dispatch.
         *
         * Box<dyn Trait> is another dynamic dispatch trait object, like `&dyn Trait`
         */
        fn bark_static_sugar(animal: &impl Animal) {
            println!("barking: {}", animal.sound());
        }
        bark_static(&Sheep);
        bark_static_sugar(&Cow);

        fn bark_both<T: Animal>(a: &T, b: &T) {
            println!("barking both: {}, {}", a.sound(), b.sound());
        }
        bark_both(&Cow, &Cow);

        fn bark_both_sugar(a: &impl Animal, b: &impl Animal) {
            println!("barking both: {}, {}", a.sound(), b.sound());
        }
        bark_both_sugar(&Cow, &Sheep);

        fn bark_dynamic(four_animals: &[&dyn Animal; 4]) {
            let [a, b, c, d] = four_animals;
            println!(
                "barking four: {}, {}, {}, {}",
                a.sound(),
                b.sound(),
                c.sound(),
                d.sound()
            );
        }
        bark_dynamic(&[&Cow, &Sheep, &Cow, &Sheep])
    }
}
