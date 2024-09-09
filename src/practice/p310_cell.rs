#[cfg(test)]
mod tests {
    use std::cell::Cell;

    #[test]
    #[allow(dead_code)]
    fn test_cell_enables_interior_mutability() {
        struct Person {
            name: String,
            height_cm: Cell<u8>,
        }

        let person = Person {
            name: "Alice".to_string(),
            height_cm: Cell::new(165),
        };

        // mutate Cell inside immutable struct `person`
        person.height_cm.set(175);

        assert_eq!(person.height_cm.get(), 175);
    }
}
