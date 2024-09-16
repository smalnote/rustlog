#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    #[test]
    fn test_phantom_data_as_type_placeholder() {
        use std::marker::PhantomData;
        trait Sound {
            fn sound() -> String;
        }

        struct Sheep();

        impl Sound for Sheep {
            fn sound() -> String {
                "Maah".to_string()
            }
        }

        struct Cow();

        impl Sound for Cow {
            fn sound() -> String {
                "Mooh".to_string()
            }
        }

        struct Greeting<T: Sound> {
            // use type T avoid compiler complaining unused type, `*const T` indicates this struct doesn't own type T
            _marker: PhantomData<*const T>,
        }

        impl<T: Sound> Greeting<T> {
            fn new() -> Self {
                Greeting {
                    _marker: PhantomData,
                }
            }

            fn greet(&self) -> String {
                T::sound()
            }
        }

        let g1 = Greeting::<Sheep>::new();
        assert_eq!(g1.greet(), "Maah");

        let g2 = Greeting::<Cow>::new();
        assert_eq!(g2.greet(), "Mooh");
    }

    #[test]
    #[allow(dead_code)]
    fn test_phantom_data_as_borrow_marker() {
        use std::marker::PhantomData;
        struct Tag<'a, T: 'a> {
            pointer: *const T,
            _marker: PhantomData<&'a T>,
        }

        impl<'a, T: 'a> From<&'a T> for Tag<'a, T> {
            fn from(value: &'a T) -> Self {
                Self {
                    pointer: value,
                    _marker: PhantomData,
                }
            }
        }

        impl<'a, T: 'a> Tag<'a, T> {
            fn set(&mut self, value: &'a T) {
                self.pointer = value
            }
        }

        let mut tag = Tag::from(&42);

        {
            // PhantomData<&'a T> enforce compiler check lifetime of `&n`
            // acting like Tag owns `&'a T`, but actually use raw pointer
            // let n = 84;
            // tag.set_ptr(&n); // error: borrowed value does not live long enough
        }

        let n = 84;
        tag.set(&n);

        unsafe {
            assert_eq!(*tag.pointer, 84);
        }
    }

    #[derive(Debug)]
    struct Tag {
        name: String,
    }

    impl From<&str> for Tag {
        fn from(value: &str) -> Self {
            Self {
                name: value.to_string(),
            }
        }
    }

    impl Drop for Tag {
        fn drop(&mut self) {
            println!("tag {} dropped", self.name);
        }
    }

    #[test]
    fn test_tag_drop() {
        println!("before tag block");
        {
            let _tag = Tag::from("block_tag");
        }
        println!("after tag block");
    }

    #[test]
    fn test_tag_with_box_leak() {
        println!("before tag block");
        {
            let tag = Tag::from("leaked_tag");
            let _tag = Box::leak(Box::new(tag));
        }
        println!("after tag block");
    }

    #[test]
    fn test_tag_with_box_from_raw() {
        println!("before tag block");
        {
            let tag = Tag::from("wrapped_leaked_tag");
            let tag = Box::leak(Box::new(tag));
            unsafe {
                let _tag = Box::from_raw(tag);
            }
        }
        println!("after tag block");
    }

    #[test]
    #[allow(dead_code)]
    fn test_tag_with_phantom_data() {
        use std::{marker::PhantomData, ptr::NonNull};
        struct TagPointer {
            value: Option<NonNull<Tag>>,
            // tells compiler TagPointer owns Box<Tag>, NOTE: this is not necessary to compile the code
            _marker: PhantomData<Box<Tag>>,
        }

        impl From<&str> for TagPointer {
            fn from(value: &str) -> Self {
                let tag = Tag::from(value);
                let tag = NonNull::from(Box::leak(Box::new(tag)));
                Self {
                    value: Some(tag),
                    _marker: PhantomData,
                }
            }
        }
        impl From<TagPointer> for Option<Box<Tag>> {
            fn from(mut tag: TagPointer) -> Self {
                tag.value
                    .take()
                    .map(|tag| unsafe { Box::from_raw(tag.as_ptr()) })
            }
        }

        impl Drop for TagPointer {
            fn drop(&mut self) {
                if let Some(tag) = self.value {
                    unsafe {
                        let _ = Box::from_raw(tag.as_ptr());
                    }
                }
            }
        }

        println!("before tag block");
        {
            let _rag_tag = TagPointer::from("phantom_marked_tag");

            let tag = TagPointer::from("phantom_marked_tag_into");
            let _tag: Option<Box<Tag>> = tag.into();
        }
        println!("after tag block");
    }

    #[test]
    fn test_tag_with_phantom_data_of_reference() {
        use std::{marker::PhantomData, ptr::NonNull};
        trait Mutator {
            type Output;

            fn as_mut(&mut self) -> Self::Output;
        }

        struct TagPointer<'a> {
            value: Option<NonNull<Tag>>,
            // hold lifetime `'a` in PhantomData, NO
            _marker: PhantomData<&'a Tag>,
        }

        impl From<&str> for TagPointer<'_> {
            fn from(value: &str) -> Self {
                let tag = Tag::from(value);
                let tag = NonNull::from(Box::leak(Box::new(tag)));
                Self {
                    value: Some(tag),
                    _marker: PhantomData,
                }
            }
        }

        impl<'a> Mutator for TagPointer<'a> {
            // lifetime `'a` is required here
            type Output = Option<&'a mut Tag>;

            // the return type `&'a mut Tag` use lifetime `'a`
            fn as_mut(&mut self) -> Option<&'a mut Tag> {
                self.value.map(|mut tag| unsafe { tag.as_mut() })
            }
        }

        impl Drop for TagPointer<'_> {
            fn drop(&mut self) {
                if let Some(tag) = self.value {
                    unsafe {
                        let _ = Box::from_raw(tag.as_ptr());
                    }
                }
            }
        }

        println!("before tag block");
        {
            let mut tag = TagPointer::from("phantom_marked_tag_reference");
            let tag = tag.as_mut();
            if let Some(tag) = tag {
                tag.name.push_str("_mut");
            }
        }
        println!("after tag block");
    }

    #[test]
    fn test_phantom_data_ownership() {
        #[derive(Debug)]
        pub struct RawPointer<T> {
            ptr: *mut T,
            _marker: PhantomData<T>,
        }

        impl<T> RawPointer<T> {
            fn from(t: T) -> Self {
                Self {
                    ptr: std::ptr::from_mut(Box::leak(Box::new(t))),
                    _marker: PhantomData,
                }
            }

            fn take(&mut self) -> Option<T> {
                let ptr = self.ptr;
                if !ptr.is_null() {
                    self.ptr = std::ptr::null_mut();
                    unsafe {
                        return Some(*Box::from_raw(ptr));
                    }
                }
                None
            }
        }

        impl<T> Drop for RawPointer<T> {
            fn drop(&mut self) {
                if !self.ptr.is_null() {
                    unsafe {
                        // take ownership of T, and drop it
                        let _ = Box::from_raw(self.ptr);
                    }
                    self.ptr = std::ptr::null_mut();
                }
            }
        }

        #[derive(Debug)]
        struct Droppable(u32);

        impl Drop for Droppable {
            fn drop(&mut self) {
                println!("dropping Droppable({})", self.0);
            }
        }

        let mut raw_ptr = RawPointer::from(Droppable(0));
        {
            let _ = raw_ptr.take();
        }
        println!("{:?}", raw_ptr);
        let _ = RawPointer::from(Droppable(42));
    }
}
