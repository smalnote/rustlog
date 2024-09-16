#[cfg(test)]
mod tests {
    // ownership move by assignment
    #[test]
    fn ownership_moved_by_assignment() {
        let s1: String = String::from("hello");
        let s2: String = s1; // memory area of s1 move ownership from s1 to s2, s1 is not unavailable
        println!("s2 = {s2}");
    }

    // ownership move by function call
    #[test]
    fn ownership_moved_by_function_call() {
        fn takes_ownership(s: String) {
            println!("s = {s}");
        }
        let s1 = String::from("hello");
        takes_ownership(s1);
        // println!("s1 = {s1}"); // error: s1 moved to takes_ownership

        fn gives_ownership() -> String {
            String::from("hello")
        }
        let s2 = gives_ownership();
        println!("s2 = {s2}");
    }

    // box reference
    #[test]
    fn box_reference() {
        let mut x: Box<i32> = Box::new(42);
        *x /= 6;

        assert_eq!(x, Box::new(7));
        assert_eq!(*x, 7);
    }

    // partial move by assignment, partial borrow by reference
    #[test]
    fn partial_moved_of_struct() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: Box<u8>,
        }
        let person = Person {
            name: String::from("Alice"),
            age: Box::new(30),
        };

        println!("person = {:?}", person);

        // destructuring `person`
        let Person { name, ref age } = person;
        println!("name = {name}, age = {age}");

        // error: value borrowed here after partial move of name,
        // println!("person = {:?}", person);

        // ok: name is moved, age is borrowed
        println!("person.age = {}", person.age);
    }

    // borrowing
    #[test]
    fn borrowing() {
        let t: (String, String) = (String::from("hello"), String::from("world"));
        // borrowing by preceeding `ref` or `ref mut`
        let (ref s1, ref s2) = t;
        println!(
            "borrowing by `ref s1, ref s2`, s1 = {s1}, s2 = {s2}, t = {:?}",
            t
        );
        // another form of borrowing
        let (s3, s4) = &t;
        println!("borrowing by `&t` s3 = {s3}, s4 = {s4}, t = {:?}", t);

        // by clone
        let (s5, s6) = t.clone();
        println!("by t.clone(), s5 = {s5}, s6 = {s6}, t = {:?}", t);
    }

    // borrow - only one mutable reference can exist at a time
    #[test]
    fn only_borrow_one_mutable_reference_at_a_time() {
        let mut s = String::from("hello");
        {
            let s1 = &mut s;
            s1.push_str(" world");
            // s1 reference goes out of scope here
        }
        // s2 is the only mutable reference to s at this point
        let s2 = &mut s;
        s2.push('!');
        assert_eq!(s, "hello world!");
    }

    // cannot borrow mutable value from immutable value
    /*
    {
        let v = 42; // immutable value
        let w = &mut v; // invalid mutable borrowing
        assert_eq!(v, *w);
    }
    */
}
