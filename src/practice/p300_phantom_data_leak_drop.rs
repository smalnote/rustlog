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

mod tests {

    #[test]
    fn test_tag_drop() {
        use super::Tag;
        println!("before tag block");
        {
            let _tag = Tag::from("block_tag");
        }
        println!("after tag block");
    }

    #[test]
    fn test_tag_with_box_leak() {
        use super::Tag;
        println!("before tag block");
        {
            let tag = Tag::from("leaked_tag");
            let _tag = Box::leak(Box::new(tag));
        }
        println!("after tag block");
    }

    #[test]
    fn test_tag_with_box_from_raw() {
        use super::Tag;
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
        use super::Tag;
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

        impl Into<Option<Box<Tag>>> for TagPointer {
            fn into(mut self) -> Option<Box<Tag>> {
                match self.value {
                    None => None,
                    Some(tag) => unsafe {
                        // takes ownership of Box<Tag>, which PhantomData<Box<Tag>> declares ownership
                        let tag = Box::from_raw(tag.as_ptr());
                        // prevent Drop from running again on this pointer by setting it to None
                        self.value = None;
                        // move Box<Tag> as return value
                        Some(tag)
                    },
                }
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
        use super::Tag;
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
}
