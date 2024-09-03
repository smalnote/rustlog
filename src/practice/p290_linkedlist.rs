use std::{marker::PhantomData, ptr::NonNull};

pub struct List<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<Box<Node<T>>>,
}

pub struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    element: T,
}

impl<T> Node<T> {
    fn new(element: T) -> Node<T> {
        Node {
            next: None,
            element,
        }
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
            len: 0,
            _marker: PhantomData,
        }
    }

    fn push_back(&mut self, element: T) {
        let node = Box::new(Node::new(element));
        let node_ptr = NonNull::from(Box::leak(node));
        unsafe {
            self.push_back_node(node_ptr);
        }
    }

    unsafe fn push_back_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            (*node.as_ptr()).next = None;
            let node = Some(node);
            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }
            self.tail = node;
            self.len += 1;
        }
    }

    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            if matches!(self.head, None) {
                self.tail = None
            }
            self.len -= 1;
            node
        })
    }

    fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(|node| (*node).element)
    }
}

pub struct IntoIter<T> {
    list: List<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

mod tests {
    use super::*;

    #[test]
    fn use_linked_list() {
        let mut list = List::<i32>::new();
        for i in 1..11 {
            list.push_back(i);
        }

        for elem in list {
            println!("{}", elem);
        }
    }

    #[test]
    fn use_option_move() {
        let node = Box::new(Node::new(42));
        let node_ptr = NonNull::from(Box::leak(node));
        let node = Option::from(node_ptr);

        // NonNull<T> implements `Copy` trait
        // m1, m2 has a copy of NonNull<Node<i32>> inside node
        let m1 = node;
        let m2 = node;
        println!("{:?} == {:?} == {:?}", m1, m2, node);

        unsafe {
            // use Box::from_raw to unleak node
            let _node = node.map(|node| Box::from_raw(node.as_ptr()));
        }
    }
}
