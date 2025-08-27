#[cfg(test)]
mod tests {

    /// `*mut T` is invariant because `&mut T` is invariant
    /// `&mut T` is invariant because &mut indicate mutable
    /// but we cannot assign &mut Sub as &mut Super
    /// eg: &mut &'static str = &mut &'a str
    #[allow(dead_code)]
    struct InVariantMyVec<T> {
        ptr: *mut T,
    }

    impl<T> InVariantMyVec<T> {
        fn new() -> InVariantMyVec<T> {
            InVariantMyVec {
                ptr: std::ptr::null_mut(),
            }
        }
    }

    #[test]
    fn test_variance_of_my_vec() {
        fn assign<T>(input: &mut InVariantMyVec<T>, value: &InVariantMyVec<T>) {
            input.ptr = value.ptr;
        }

        let mut hello = InVariantMyVec::<&'static str>::new();

        {
            let world = InVariantMyVec::<&str>::new();
            assign(&mut hello, &world); // assign world to hello
        }

        // println!("{:?}", *hello.ptr); // use after free
    }

    use std::alloc::{self, alloc, dealloc, Layout};
    use std::collections::HashSet;
    use std::fmt::Debug;
    use std::mem::{self, size_of};
    use std::ops::{Deref, DerefMut};
    use std::ptr::{copy, read, write, NonNull};

    /// This is a implementation from Rustonomicon:
    /// [Vec](https://doc.rust-lang.org/nomicon/vec/vec.html)
    struct MyVec<T> {
        buf: RawVec<T>,
        len: usize,
    }

    // MyVec<T> is Send if T is Send
    // Since both buf and len inside MyVec<T> is Send and Sync
    // MyVec<T> as a trivial wrap of them, is derived Send and Sync
    // unsafe impl<T: Send> Send for MyVec<T> {}
    // unsafe impl<T: Sync> Sync for MyVec<T> {}

    impl<T> MyVec<T> {
        fn new() -> MyVec<T> {
            MyVec {
                buf: RawVec::new(),
                len: 0,
            }
        }

        fn with_capacity(cap: usize) -> MyVec<T> {
            MyVec {
                buf: RawVec::with_capacity(cap),
                len: 0,
            }
        }

        fn ptr(&self) -> *mut T {
            self.buf.ptr.as_ptr()
        }

        fn cap(&self) -> usize {
            self.buf.cap
        }

        fn push(&mut self, elem: T) {
            if self.cap() == self.len {
                self.buf.grow();
            }
            unsafe {
                write(self.ptr().add(self.len), elem);
            }
            self.len += 1;
        }

        fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                None
            } else {
                self.len -= 1;
                Some(unsafe { read(self.ptr().add(self.len)) })
            }
        }

        fn insert(&mut self, index: usize, elem: T) {
            assert!(index <= self.len, "index out of bounds");
            if index == self.cap() {
                self.buf.grow();
            } else if index < self.len {
                unsafe {
                    copy(
                        self.ptr().add(index),
                        self.ptr().add(index + 1),
                        self.len - index,
                    );
                }
            }
            unsafe {
                write(self.ptr().add(index), elem);
            }
            self.len += 1;
        }

        fn remove(&mut self, index: usize) -> T {
            assert!(index < self.len, "index out of bounds");
            let elem: T = unsafe { read(self.ptr().add(index)) };

            self.len -= 1;
            if index < self.len {
                unsafe {
                    copy(
                        self.ptr().add(index + 1),
                        self.ptr().add(index),
                        self.len - index,
                    );
                }
            }

            elem
        }

        fn drain(&'_ mut self) -> Drain<'_, T> {
            let iter = unsafe { RawValIter::new(self) };
            // Here Drain only take RawValIter(start, end) from MyVec,
            // What if Drain doesn't consume all elements first,
            // and MyVec write new element to range within RawValIter(start, end).
            // This might cause leak.
            // The answer is impossible:
            // drain(&mut self) take mutable reference of MyVec
            // Meaning before the returned Drain instance going out of scope and Drop,
            // there is no chance to borrow mutable MyVec and mutate it.
            // If Drain if forgotten, we just leak the whole MyVec's contents.
            // But for MyVec, elements are handed over to Drain, set len to
            // zero is safe here.
            self.len = 0;
            Drain {
                vec: PhantomData,
                iter,
            }
        }
    }

    impl<T: Debug> Debug for MyVec<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let slice: &[T] = self;
            write!(f, "{:?}", slice)
        }
    }

    impl<T> Deref for MyVec<T> {
        type Target = [T];
        fn deref(&self) -> &Self::Target {
            unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
        }
    }

    impl<T> DerefMut for MyVec<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
        }
    }

    struct IntoIter<T> {
        _buf: RawVec<T>,
        iter: RawValIter<T>,
    }

    impl<T> IntoIterator for MyVec<T> {
        type IntoIter = IntoIter<T>;
        type Item = T;
        fn into_iter(self) -> Self::IntoIter {
            unsafe {
                let iter = RawValIter::new(&self);
                let buf = read(&self.buf);
                mem::forget(self);
                IntoIter { _buf: buf, iter }
            }
        }
    }

    impl<I> FromIterator<I> for MyVec<I> {
        fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
            let iter = iter.into_iter();
            let (size, _) = iter.size_hint();
            let mut my_vec = MyVec::with_capacity(size);
            for item in iter {
                my_vec.push(item);
            }
            my_vec
        }
    }

    impl<T> Drop for MyVec<T> {
        fn drop(&mut self) {
            while self.pop().is_some() {}
        }
    }

    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
    }

    impl<T> DoubleEndedIterator for IntoIter<T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.iter.next_back()
        }
    }

    impl<T> Drop for IntoIter<T> {
        fn drop(&mut self) {
            // drop any remaining elements
            // for _ in &mut *self would mutably borrow self, and no ownership is taken.
            // for _ in self would call self.into_iter() and consume self,
            // meaning self is moved into the loop and cannot be used afterward.
            // ```rust
            // for _ in self {}
            // for _ in &mut *self{} // borrow of moved value: `self` value borrowed here after move
            // ```
            // Drain impl Iterator -> &mut Drain impl Iterator -> &mut Drain impl IntoIterator
            // Drain impl Iterator also-> Drain impl Iterator
            // Relies on generic traits from std::iter
            // impl<I: Iterator + ?Sized> Iterator for &mut I
            // impl<I: Iterator> IntoIterator for I {}
            // actually call on (&mut *self).into_iter();
            for _ in &mut *self {}
        }
    }

    use std::marker::PhantomData;
    use std::thread;

    struct Drain<'a, T: 'a> {
        // Need to bound the lifetime here, so we do it with `&'a mut MyVec<T>`
        // because that's semantically what we contain. We're just calling
        // `pop()` and `remove(0)`
        vec: PhantomData<&'a mut MyVec<T>>,
        iter: RawValIter<T>,
    }

    impl<T> Iterator for Drain<'_, T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
    }

    impl<T> DoubleEndedIterator for Drain<'_, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.iter.next_back()
        }
    }

    #[allow(clippy::needless_lifetimes)]
    impl<'a, T> Drop for Drain<'a, T> {
        fn drop(&mut self) {
            dbg!("drop rest elements in drain");
            for _ in &mut *self {}
        }
    }

    struct RawVec<T> {
        ptr: NonNull<T>,
        cap: usize,
    }

    // MyVec<T> is Send if T is Send
    unsafe impl<T: Send> Send for RawVec<T> {}
    unsafe impl<T: Sync> Sync for RawVec<T> {}

    impl<T> RawVec<T> {
        fn new() -> RawVec<T> {
            RawVec {
                ptr: NonNull::dangling(),
                cap: if size_of::<T>() == 0 { usize::MAX } else { 0 },
            }
        }

        fn with_capacity(cap: usize) -> RawVec<T> {
            if cap == 0 {
                return Self::new();
            }
            let (ptr, cap) = if size_of::<T>() == 0 {
                (NonNull::dangling(), usize::MAX)
            } else {
                let layout = Layout::array::<T>(cap).unwrap();
                let raw_ptr = unsafe { alloc(layout) as *mut T };
                let ptr = match NonNull::new(raw_ptr) {
                    Some(null_ptr) => null_ptr,
                    None => alloc::handle_alloc_error(layout),
                };
                (ptr, cap)
            };
            RawVec { ptr, cap }
        }

        fn grow(&mut self) {
            // since cap set to usize::MAX for ZSTs, grow for ZSTs would overflow
            assert!(size_of::<T>() != 0, "capacity overflow");

            let (new_cap, new_layout) = if self.cap == 0 {
                (1, Layout::array::<T>(1).unwrap())
            } else {
                let new_cap = 2 * self.cap;
                (new_cap, Layout::array::<T>(new_cap).unwrap())
            };

            assert!(
                new_layout.size() <= isize::MAX as usize,
                "Allocation too large"
            );

            let new_ptr = if self.cap == 0 {
                unsafe { alloc::alloc(new_layout) }
            } else {
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                let old_ptr = self.ptr.as_ptr() as *mut u8;
                unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
            };

            self.ptr = match NonNull::new(new_ptr as *mut T) {
                Some(ptr) => ptr,
                None => alloc::handle_alloc_error(new_layout),
            };
            self.cap = new_cap;
        }
    }

    impl<T> Drop for RawVec<T> {
        fn drop(&mut self) {
            if self.cap != 0 && size_of::<T>() != 0 {
                let layout = Layout::array::<T>(self.cap).unwrap();
                unsafe {
                    dealloc(self.ptr.as_ptr() as *mut u8, layout);
                }
            }
        }
    }

    struct RawValIter<T> {
        start: *const T,
        end: *const T,
    }

    impl<T> RawValIter<T> {
        // unsafe to construct because it has no associated lifetimes.
        // This is necessary to store a RawValIter in the same struct as
        // its actual allocation. OK since it's a private implementation
        // detail.
        unsafe fn new(slice: &[T]) -> RawValIter<T> {
            RawValIter {
                start: slice.as_ptr(),
                end: if size_of::<T>() == 0 {
                    // for ZST ptr.add() are no-ops, so cast the ptr to integer, increment, and the cast them back
                    ((slice.as_ptr() as usize) + slice.len()) as *const T
                } else if slice.is_empty() {
                    // if `len() == 0`, then this is not actually allocated memory.
                    // Need to avoid offsetting because that will give wrong
                    // information to LLVM via GEP.
                    slice.as_ptr()
                } else {
                    unsafe { slice.as_ptr().add(slice.len()) }
                },
            }
        }
    }

    impl<T> Iterator for RawValIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.start == self.end {
                None
            } else {
                unsafe {
                    if size_of::<T>() == 0 {
                        self.start = ((self.start as usize) + 1) as *const T;
                        // for ZST, read is a noop
                        Some(read(NonNull::<T>::dangling().as_ptr()))
                    } else {
                        let elem: T = read(self.start);
                        self.start = self.start.add(1);
                        Some(elem)
                    }
                }
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let elem_size = size_of::<T>();
            let len = (self.end as usize - self.start as usize)
                / if elem_size == 0 { 1 } else { elem_size };
            (len, Some(len))
        }
    }

    impl<T> DoubleEndedIterator for RawValIter<T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.start == self.end {
                None
            } else {
                unsafe {
                    if size_of::<T>() == 0 {
                        self.end = (self.end as usize - 1) as *const T;
                        Some(read(NonNull::<T>::dangling().as_ptr()))
                    } else {
                        self.end = self.end.offset(-1);
                        Some(read(self.end))
                    }
                }
            }
        }
    }

    #[test]
    fn test_my_vec() {
        let mut numbers = MyVec::new();

        for i in 0..10 {
            numbers.push(i);
        }

        for _ in 0..5 {
            numbers.pop();
        }

        assert_eq!(numbers[2], 2);
        assert_eq!(vec![0, 1, 2, 3, 4], numbers[..]);

        numbers.remove(4);
        numbers.remove(0);
        numbers.remove(1);
        numbers.insert(1, 5);

        let mut iter = numbers.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), Some(5));
    }

    #[test]
    fn test_drain() {
        let mut numbers = MyVec::new();

        for i in 0..5 {
            numbers.push(i);
        }

        numbers.remove(4);
        numbers.remove(2);
        numbers.remove(0);

        {
            let mut drain = numbers.drain();
            assert_eq!(drain.next(), Some(1));
            dbg!("leave drain with one element");
        }

        dbg!("operating in place");
        assert!(numbers.is_empty());
    }

    #[test]
    fn test_from_iterator() {
        let set: HashSet<String> = HashSet::from([
            "alpha".into(),
            "bravo".into(),
            "caesar".into(),
            "delta".into(),
        ]);

        // MyVec implements FromIterator
        // let my_vec = MyVec::<String>::from_iter(set.clone().into_iter());
        // Iterator::collect() use FromIterator automatically
        let my_vec: MyVec<String> = set.clone().into_iter().collect();
        dbg!(&my_vec);
        let new_set: HashSet<String> = my_vec.into_iter().collect();
        assert_eq!(set, new_set);
    }

    #[test]
    fn my_vec_send_trait() {
        let mut vec = MyVec::new();
        vec.push(42);

        thread::scope(|scoped| {
            for _ in 0..4 {
                scoped.spawn(|| {
                    dbg!(&vec); // &MyVec<T: Send>: Send
                });
            }
        });

        thread::spawn(move || {
            dbg!(vec); // MyVec<T: Send>: Send
        })
        .join()
        .unwrap();
    }
}
