#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    mod naive_my_arc {
        use std::sync::atomic;

        // Version issues:
        // #1 MyArc<T> is invariant on T
        // #2 Doesn't give ownership information of T belonging to MyArc eventually,
        //    when reference count equal to one.
        // How to fix:
        // #1 Use NonNull<MyArcInner<T>> instead of *mut MyArcInner to make sure covariance
        // #2 Add PhantomData<MyArcInner<T>> to tell the Rust compile correct ownership.
        struct MyArc<T> {
            ptr: *mut MyArcInner<T>,
        }

        // Here we use lock-free atomic type for thread-safe reference count
        struct MyArcInner<T> {
            data: T,
            rc: atomic::AtomicUsize,
        }
    }

    use std::{
        marker::PhantomData,
        ops::Deref,
        ptr::NonNull,
        sync::{
            Arc,
            atomic::{self},
        },
        thread::{self},
    };

    struct MyArc<T> {
        ptr: NonNull<MyArcInner<T>>,
        _marker: PhantomData<MyArcInner<T>>,
    }

    struct MyArcInner<T> {
        data: T,
        rc: atomic::AtomicUsize,
    }

    impl<T> MyArc<T> {
        fn new(data: T) -> Self {
            Self {
                ptr: {
                    let inner = MyArcInner {
                        data,
                        rc: atomic::AtomicUsize::new(1),
                    };
                    NonNull::new(Box::into_raw(Box::new(inner))).unwrap()
                },
                _marker: PhantomData,
            }
        }
    }

    impl<T> Clone for MyArc<T> {
        fn clone(&self) -> Self {
            let inner = unsafe { self.ptr.as_ref() };
            // Using a relaxed ordering is alright here as we don't need any atomic
            // synchronization here as we're not modifying or accessing the inner data.
            let old_rc = inner.rc.fetch_add(1, atomic::Ordering::Relaxed);
            if old_rc >= isize::MAX as usize {
                std::process::abort();
            }
            Self {
                ptr: self.ptr, // copy
                _marker: PhantomData,
            }
        }
    }

    /// MyArc<T> involves shared ownership and potential concurrent access,
    /// so T must implement Send + Sync for simultaneous access from multiple
    /// thread and multiple reference.
    /// While MyVec<T> has sole ownership of T.
    unsafe impl<T: Send + Sync> Send for MyArc<T> {}
    unsafe impl<T: Send + Sync> Sync for MyArc<T> {}

    impl<T> Deref for MyArc<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            let inner = unsafe { self.ptr.as_ref() };
            &inner.data
        }
    }

    impl<T> Drop for MyArc<T> {
        fn drop(&mut self) {
            let inner = unsafe { self.ptr.as_ref() };
            if inner.rc.fetch_sub(1, atomic::Ordering::Release) != 1 {
                return;
            }
            // This fence is needed to prevent reordering of the use and deletion
            // of the data.
            atomic::fence(atomic::Ordering::Acquire);
            unsafe {
                let _ = Box::from_raw(self.ptr.as_ptr());
            };
        }
    }
    struct DropDbg {}

    impl Drop for DropDbg {
        fn drop(&mut self) {
            dbg!(self as *const Self);
        }
    }

    #[test]
    fn test_my_arc() {
        let drop_indicator = MyArc::new(DropDbg {});

        thread::scope(|scoped| {
            for _ in 0..10 {
                let number_ref = drop_indicator.clone();
                scoped.spawn(move || {
                    for _ in 0..1000 {
                        let _ = number_ref.clone();
                    }
                });
            }
            drop(drop_indicator);
        });
    }

    #[test]
    fn test_arc() {
        let drop_indicator = Arc::new(DropDbg {});

        thread::scope(|scoped| {
            for _ in 0..10 {
                let number_ref = drop_indicator.clone();
                scoped.spawn(move || {
                    for _ in 0..1000 {
                        let _ = number_ref.clone();
                    }
                });
            }
            drop(drop_indicator);
        });
    }
}
