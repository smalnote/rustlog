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

pub struct Iter<'a, T> {
    current: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| unsafe {
            let node = &*node.as_ptr();
            self.current = node.next;
            &node.element
        })
    }
}

pub struct IterMut<'a, T> {
    current: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| unsafe {
            let node = &mut *node.as_ptr();
            self.current = node.next;
            &mut node.element
        })
    }
}

impl<'a, T> List<T> {
    fn iter(&self) -> Iter<'a, T> {
        Iter {
            current: self.head,
            _marker: PhantomData,
        }
    }

    fn iter_mut(&mut self) -> IterMut<'a, T> {
        IterMut {
            current: self.head,
            _marker: PhantomData,
        }
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

        let mut i = 1;
        for elem in list {
            assert_eq!(i, elem);
            i += 1;
        }
        assert_eq!(i, 11);
    }

    #[test]
    fn use_iter() {
        let mut list = List::<f32>::new();
        for value in 100..150 {
            list.push_back(value as f32);
        }

        for value in list.iter_mut() {
            *value += 1.0;
        }

        for (i, value) in list.iter().enumerate() {
            assert_eq!(*value, (101 + i) as f32);
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
