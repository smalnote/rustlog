#[cfg(test)]
mod tests {
    // print pointer address of reference
    #[test]
    fn print_reference_address() {
        let s = String::from("hello");
        let s = &s;
        println!("s = {:p}", s);
        println!("s = {s:p}");
    }

    // dereference
    #[test]
    fn using_reference() {
        let n = 42;
        let m = &n;
        assert_eq!(n, *m);
    }

    // two way of reference
    #[test]
    fn two_way_of_reference() {
        let c: char = '中';
        let c1: &char = &c;
        #[allow(clippy::toplevel_ref_arg)]
        let ref c2 = c;
        assert_eq!(*c1, *c2);
        assert_eq!(c1, c2); // c1, c2 are both references(pointers) to c
    }

    // use ref for pattern matching
    #[test]
    #[allow(clippy::unnecessary_literal_unwrap)]
    fn use_reference_within_matching_arm_for_borrowing() {
        let maybe_name: Option<String> = Some(String::from("alice"));
        match maybe_name {
            // ref here just borrow the value, not move ownership
            // it is not matched against Some(String), but Some(&String)
            Some(ref name) => {
                assert_eq!(name, "alice");
            }
            None => unimplemented!(),
        }
        // maybe_name still available here
        assert_eq!(maybe_name.unwrap(), "alice");
    }
}
