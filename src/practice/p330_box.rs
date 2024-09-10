#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        ops::{Deref, DerefMut},
        rc::{Rc, Weak},
    };

    #[test]
    fn test_deref_box() {
        let x = 42;
        let y = &x;
        let mut z = Box::new(6);

        *z *= 7;

        assert_eq!(*y, *z);
    }

    #[test]
    fn test_custom_type_with_trait_deref() {
        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(value: T) -> Self {
                Self(value)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> DerefMut for MyBox<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        let mut x: MyBox<i32> = MyBox::new(42);
        *x /= 7;
        assert_eq!(*x, 6);

        fn multiply_seven(value: &mut i32) {
            *value *= 7;
        }
        // &mut MyBox<i32> with implicit deref coercion, converted to type &mut i32
        multiply_seven(&mut x);
        assert_eq!(*x, 42);

        let mut y: MyBox<MyBox<i32>> = MyBox::new(x);
        // double deref from &mut MyBox<MyBox<i32>> to &mut i32
        multiply_seven(&mut y);
        assert_eq!(**y, 294);
    }

    #[test]
    #[allow(dead_code)]
    fn test_ref_cell_rc_circle() {
        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    List::Cons(_, item) => Some(item),
                    List::Nil => None,
                }
            }
        }

        let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // NOTE: print a or a.tail will cause Rust print an infinite list
        // println!("a next item = {:?}", a.tail());
    }

    #[test]
    #[allow(dead_code)]
    fn test_rc_weak_for_tree_parent() {
        #[derive(Debug)]
        struct Node<T> {
            element: T,
            parent: RefCell<Weak<Node<T>>>,
            children: RefCell<Vec<Rc<Node<T>>>>,
        }

        impl<T> Node<T> {
            fn new(element: T) -> Self {
                Self {
                    element,
                    parent: Default::default(),
                    children: Default::default(),
                }
            }
        }

        let leaf = Rc::new(Node::<i32>::new(0));

        let branch = Rc::new(Node::<i32>::new(10));

        branch.children.borrow_mut().push(Rc::clone(&leaf));
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("{:?}", &branch);
    }
}
