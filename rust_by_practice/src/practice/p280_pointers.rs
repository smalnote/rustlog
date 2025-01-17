#[cfg(test)]
mod tests {
    use std::rc::Rc;
    /*
     * Sized: In order to store value on stack, the Rust compiler needs to know the
     *   size of that memory at compilation time.
     */

    /* Smart Pointers:
     * - Box<T>: the enclosed data needs to be stored on heap, others like a &mut
     *     - Store dynamic value, like Trait, can have different size at runtime, by `Box<dyn Trait>`
     *     - Use in recursive data type, struct List<T> { value: T, next: Option <Box<T>> }
     */
    #[test]
    fn smart_pointer_box() {
        trait Vehicle {
            fn drive(&self);
        }

        struct Truck {}

        impl Vehicle for Truck {
            fn drive(&self) {
                println!("Truck is driving");
            }
        }

        #[allow(clippy::needless_late_init)]
        let v: Box<dyn Vehicle>;
        v = Box::new(Truck {});
        v.drive();

        struct TruckNode {
            current: Truck,
            next: Option<Box<TruckNode>>,
        }

        impl TruckNode {
            fn new(t: Truck) -> Self {
                Self {
                    current: t,
                    next: None,
                }
            }

            fn append(&mut self, t: Truck) {
                let mut p = self;
                loop {
                    match p.next {
                        None => break,
                        Some(ref mut b) => p = &mut **b,
                    }
                }
                p.next = Some(Box::new(Self {
                    current: t,
                    next: None,
                }))
            }

            fn drive(&self) {
                let mut p = self;
                loop {
                    p.current.drive();
                    match p.next {
                        None => break,
                        Some(ref b) => p = &**b,
                    }
                }
            }
        }
        let mut tt = TruckNode::new(Truck {});
        tt.append(Truck {});
        tt.append(Truck {});
        tt.append(Truck {});
        tt.drive();
    }

    /*
     * Smart pointers:
     * - Rc<T>: reference counting memory, memory got recycled after last reference goes out of scope
     *     -
     */
    #[test]
    fn reference_count() {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Truck {
            name: String,
        }

        impl Truck {
            fn new(name: &str) -> Self {
                Self {
                    name: name.to_owned(),
                }
            }
        }

        let truck_alpha: Rc<Truck> = Rc::new(Truck::new("alpha"));
        let truck_bravo: Rc<Truck> = Rc::new(Truck::new("bravo"));
        let truck_gamma: Rc<Truck> = Rc::new(Truck::new("gamma"));

        let team_one = vec![truck_alpha, Rc::clone(&truck_bravo)];
        let team_two = vec![Rc::clone(&truck_bravo), truck_gamma];

        assert_eq!(
            addr_of!(*team_two[0].as_ref()),
            addr_of!(*team_one[1].as_ref())
        );

        assert_eq!(Rc::strong_count(&truck_bravo), 3); // the original one plus two clones
        println!("team_one = {:?}", team_one);
        println!("team_two = {:?}", team_two);
        println!(
            "strong count reference of truck bravo = {}",
            Rc::strong_count(&truck_bravo)
        );
        println!(
            "team_one bravo addr = {:p}, team_two bravo addr = {:p}",
            addr_of!(*team_two[0].as_ref()),
            addr_of!(*team_one[1].as_ref())
        );
    }

    #[test]
    #[allow(clippy::useless_vec)]
    fn test_rc_and_ref() {
        let truck_1 = String::from("truck_1");
        let truck_2 = String::from("truck_2");
        let team_truck = vec![&truck_1, &truck_2];
        assert_eq!(team_truck[0], "truck_1");
        drop(truck_1);
        // team_truck unavailable
        // assert_eq!(team_truck[0], "truck"); // compilation error, ownership still belong to variable truck

        let car_1 = Rc::new(String::from("car_1"));
        let car_2 = Rc::new(String::from("car_2"));
        let car_3 = Rc::new(String::from("car_3"));
        let team_one = vec![Rc::clone(&car_1), Rc::clone(&car_2)];
        let team_two = vec![Rc::clone(&car_2), Rc::clone(&car_3)];
        drop(car_1);
        assert_eq!(*team_one[0], "car_1");
        assert_eq!(*team_two[0], "car_2");
    }

    /* Smart pointers:
     * Arc: atomic reference counter
     *   - For sending value to spawn thread by implementing trait `Send`
     *   - Use atomic type to order value visiting thread safe, without `Lock`
     */
    #[test]
    fn arc() {
        use std::sync::Arc;

        #[derive(Debug)]
        #[allow(dead_code)]
        struct Truck {
            name: String,
        }

        impl Truck {
            fn new(name: &str) -> Self {
                Self {
                    name: name.to_owned(),
                }
            }
        }

        let truck_alpha: Arc<Truck> = Arc::new(Truck::new("alpha"));
        let truck_bravo: Arc<Truck> = Arc::new(Truck::new("bravo"));
        let truck_gamma: Arc<Truck> = Arc::new(Truck::new("gamma"));

        let truck_bravo_one = Arc::clone(&truck_bravo);
        let truck_bravo_two = Arc::clone(&truck_bravo);

        let thread_one = std::thread::spawn(move || {
            let team_one = vec![truck_alpha, truck_bravo_one];
            team_one
        });
        let team_one = thread_one.join().unwrap();

        let thread_two = std::thread::spawn(move || {
            let team_two = vec![truck_bravo_two, truck_gamma];
            team_two
        });
        let team_two = thread_two.join().unwrap();

        println!("team_one = {:?}, team_two = {:?}", team_one, team_two);
    }

    /*
     * Raw pointer `*const T`, `*mut T`:
     *   - Raw pointer is a nullable version of reference.
     *   - Creating raw pointer:
     *       - Coerce reference to raw pointer
     *       - Box::into_raw
     *       - Use marcos std::ptr::addr_of!() and std::ptr::addr_of_mut!()
     *   - Raw pointer does not take ownership of the original allocation, but using
     *     raw pointer after value's lifetime result in undefined behavior, the old variable is still
     *     used for memory management.
     *   - Using of raw pointer must be in unsafe block.
     *   - Check null: use method `is_null` of `*const T` and `*mut T`.
     *   - Storing through a raw pointer using *ptr = data calls drop on the old value
     *     `*ptr` if it implements Drop, so it must be initialized ahead.
     *   - Dereference a null raw pointer result in segment fault.
     *   - Using `ptr` after going out scope of pointed value is undefined behavior.
     */

    use std::ptr::{addr_of, NonNull};

    #[test]
    fn coerce_reference_to_raw_pointers() {
        let number = 42;
        let number_pointer: *const i32 = &number;
        let mut mut_number = 84;
        let number_mut_pointer: *mut i32 = &mut mut_number;

        unsafe {
            assert_eq!(*number_pointer, 42);
            *number_mut_pointer /= 2;
            assert_eq!(*number_mut_pointer, 42);
        }

        assert_eq!(mut_number, 42);
    }

    #[test]
    fn dereference_dangling_pointer_is_undefined_behavior() {
        let mut magic = 42;
        let mut ptr: *mut i32 = &mut magic;

        unsafe {
            assert_eq!(*ptr, 42);
        }

        {
            let mut number = 84;
            ptr = &mut number;
            // lifetime of value `number` end here
            unsafe {
                assert_eq!(*ptr, 84);
            }
        }

        // ptr is dangling now, using of ptr is undefined behavior
        // unsafe {
        //    *ptr = 42; // undefined behavior
        //}
    }

    #[test]
    fn using_null_raw_pointer() {
        let ptr: *const i32 = std::ptr::null();

        assert!(ptr.is_null());

        // Pointing to null pointer result in crash:
        // (signal: 11, SIGSEGV: invalid memory reference)
        // unsafe {
        //    println!("{}", *ptr);
        //}
    }

    /*
     * Storing raw pointer:
     * From: https://doc.rust-lang.org/std/primitive.pointer.html
     * Storing through a raw pointer using *ptr = data calls drop on the old value,
     * so write must be used if the type has drop glue and memory is not already
     * initialized - otherwise drop would be called on the uninitialized memory.
     *
     * Note: ptr = &mut d1, if d2 is immutable, *ptr = d2 make it possible to mutate d2.
     */
    #[test]
    fn custom_drop_trait_on_dereference_ptr_assignment() {
        use std::alloc::{alloc, dealloc, Layout};

        #[derive(Debug, PartialEq)]
        struct Droppable(String, *mut u8);

        impl Droppable {
            fn new(name: String) -> Self {
                let layout = Layout::array::<u8>(1).unwrap();
                Droppable(name, unsafe { alloc(layout) })
            }
        }

        impl Drop for Droppable {
            fn drop(&mut self) {
                println!("{}.drop()", self.0);
                let layout = Layout::array::<u8>(1).unwrap();
                unsafe { dealloc(self.1, layout) }
            }
        }

        let mut d1 = Droppable::new(String::from("d1"));
        let ptr: *mut Droppable = &mut d1;

        let d2 = Droppable::new(String::from("d2"));

        unsafe {
            // This assignment does three things:
            // 1. Calls drop() on the old value: d1, d1 still has ownership of its *mut u8
            // 2. Frees the old value's resources(heap memory)
            // 2. Overwrites the memory location
            *ptr = d2; // *ptr drop d1.0, d1.1 and d2 move to *ptr(d1), now d2.0, d2.1 is owned by *ptr(d1)
        }

        // *ptr(d1) owned the moved d2.0, d2.1
        assert_eq!(d1.0, "d2");

        // now you can mutate the original immutable d2 content
        d1.0 = String::from("dd");

        // *ptr(d1) goes out of scope, ("d2", 0x2) is dropped
    }

    #[test]
    fn variable_shadowing_does_not_drop_old_value() {
        struct Droppable<'a>(&'a str);

        impl Drop for Droppable<'_> {
            fn drop(&mut self) {
                println!("{}.drop()", self.0);
            }
        }

        let _d = Droppable("d1");
        let ref_d1 = &_d;
        println!("before shadowing");
        let _d = 42_i32;
        println!("after shadowing");
        println!("ref of shadowed value = {}", ref_d1.0);
    }

    // Null pointer optimization: https://doc.rust-lang.org/std/option/index.html#representation
    // Thanks to the `null pointer optimization` for Option<T>, NonNull<T> and Option<NonNull<T>>
    // are guaranteed to have the same size and alignment.
    #[test]
    fn non_null_w_o_option_same_size_and_aligned() {
        use std::ptr::NonNull;
        assert_eq!(size_of::<NonNull<i16>>(), size_of::<Option<NonNull<i16>>>());
        assert_eq!(
            align_of::<NonNull<i16>>(),
            align_of::<Option<NonNull<i16>>>()
        );

        assert_eq!(size_of::<NonNull<str>>(), size_of::<Option<NonNull<str>>>());
        assert_eq!(
            align_of::<NonNull<str>>(),
            align_of::<Option<NonNull<str>>>()
        );
    }

    /*
     * std::ptr::NonNull<T>
     *   NonNull is like raw pointer `*mut T`, but is guaranteed to be non-zero and covariant.
     *
     * Variance:
     *   Variance describes how the type system behaves when dealing with lifetime or generics,
     *   particularly when types are nested within others(like references or pointers).
     *
     * Covariance:
     *   Covariance means that if a type `T` can be converted to another type `U`,
     *   then a type that contains `T`(like NonNull<T>) can also be converted to the
     *   corresponding type that contains `U`(NonNull<U>).
     *   Specifically, if `T` has a shorter lifetime than `U`, `NonNull<T>` can be converted
     *   to `NonNull<U>` without any issues.
     */
    #[test]
    fn non_null_is_non_zero() {
        let ptr = NonNull::new(std::ptr::null_mut::<i32>());
        assert_eq!(ptr, Option::None);

        let ptr = NonNull::new(&mut 42).unwrap();
        assert_eq!(*unsafe { ptr.as_ref() }, 42);

        let unchecked_ptr = unsafe { NonNull::new_unchecked(&mut 42 as *mut _) };
        assert_eq!(unsafe { *unchecked_ptr.as_ptr() }, 42);
    }

    #[test]
    #[allow(dead_code, unused_variables)]
    fn non_null_is_covariant() {
        struct Animal;
        struct Dog {
            animal: Animal,
        } // Dos is a subtype of Animal

        let dog = Dog { animal: Animal };
        let dog_ptr: NonNull<Dog> = NonNull::new(&dog as *const _ as *mut _).unwrap();
        // Dog -> Animal, NonNull<Dog> -> NonNull<Animal>
        let animal_ptr: NonNull<Animal> = dog_ptr.cast();
    }
}
