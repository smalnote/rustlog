#[cfg(test)]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };

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

    #[test]
    fn test_ref_cell_enables_interior_mutability() {
        let history: RefCell<Vec<String>> = RefCell::new(Vec::<String>::new());

        struct Tag<'a> {
            name: &'a str,
            history: &'a RefCell<Vec<String>>,
        }

        impl<'a> Tag<'a> {
            fn new(name: &'a str, history: &'a RefCell<Vec<String>>) -> Self {
                Self { name, history }
            }
        }

        impl Drop for Tag<'_> {
            fn drop(&mut self) {
                self.history.borrow_mut().push(self.name.to_string());
            }
        }

        {
            let _alice = Tag::new("alice", &history);

            {
                let _joe = Tag::new("joe", &history);
            }
        }

        assert_eq!(
            *(history.borrow()),
            vec!["joe".to_string(), "alice".to_string()]
        );
    }

    #[test]
    fn test_combin_rc_and_ref_cell() {
        #[derive(PartialEq)]
        enum Group {
            Alpha,
            Beta,
            Gamma,
        }

        struct Object {
            id: u32,
            group: Rc<RefCell<Group>>,
        }

        impl Object {
            fn new(id: u32, group: Rc<RefCell<Group>>) -> Self {
                Self { id, group }
            }
        }

        let group_one = Rc::new(RefCell::new(Group::Alpha));
        let group_two = Rc::new(RefCell::new(Group::Beta));

        let all_objects = vec![
            // multiple mutable reference to group one
            Object::new(0, Rc::clone(&group_one)),
            Object::new(1, Rc::clone(&group_one)),
            Object::new(2, Rc::clone(&group_one)),
            // multiple mutable reference to group two
            Object::new(3, Rc::clone(&group_two)),
            Object::new(4, Rc::clone(&group_two)),
            Object::new(5, Rc::clone(&group_two)),
        ];

        fn filter_group_ids(objects: &[Object], group: Group) -> Vec<u32> {
            objects
                .iter()
                .filter_map(move |object| {
                    if *object.group.borrow() == group {
                        Some(object.id)
                    } else {
                        None
                    }
                })
                .collect::<Vec<u32>>()
        }

        assert_eq!(filter_group_ids(&all_objects, Group::Alpha), vec![0, 1, 2]);

        // change multiple object's group by Rc<RefCell<Group>> at once
        *group_one.borrow_mut() = Group::Gamma;
        assert_eq!(filter_group_ids(&all_objects, Group::Gamma), vec![0, 1, 2]);
    }
}
