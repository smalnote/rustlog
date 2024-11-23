#[cfg(test)]
mod tests {
    use std::{
        cell::UnsafeCell,
        hint,
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
        thread,
    };

    struct AtomicMutex<T> {
        locked: AtomicBool,
        data: UnsafeCell<T>,
    }

    impl<T> AtomicMutex<T> {
        const UNLOCKED: bool = false;
        const LOCKED: bool = true;

        fn new(data: T) -> Self {
            AtomicMutex {
                locked: AtomicBool::new(Self::UNLOCKED),
                data: UnsafeCell::new(data),
            }
        }

        fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            // Let't take a view from compiler and CPU
            // the lock/unlock instructions and the data operation instructions
            // has no order dependencies, since the variable they access is disjoint.
            // The dependency is out business logic.
            // Without Ordering::Acquire and Ordering::Release, compiler or CPU
            // might reorder the data operation go beyond the locked scoped.
            // Ordering::Acquire make sure data operation won't be reordered before
            // this locked instruction.
            while self
                .locked
                .compare_exchange_weak(
                    Self::UNLOCKED,
                    Self::LOCKED,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                )
                .is_err()
            {
                while self.locked.load(Ordering::Relaxed) {
                    hint::spin_loop();
                }
            }
            // Shared data operation
            let ret = f(unsafe { &mut *self.data.get() });
            // Release make sure data operation won't be reordered after unlocked,
            // so the data updated is visible to other thread's unlocked instructions.
            self.locked.store(Self::UNLOCKED, Ordering::Release);
            ret
        }
    }

    unsafe impl<T: Sync> Sync for AtomicMutex<T> {}

    #[test]
    fn test_atomic_mutex() {
        const CONCURR: i32 = 14;
        const OFFSET: i32 = 10_000;
        let counter = Arc::new(AtomicMutex::new(0));

        thread::scope(|scoped| {
            for _ in 0..CONCURR {
                let counter = Arc::clone(&counter);
                scoped.spawn(move || {
                    for _ in 0..OFFSET {
                        counter.with_lock(|count| {
                            *count += 1;
                            *count
                        });
                    }
                });
            }
        });

        let count = counter.with_lock(|count| *count);
        assert_eq!(count, CONCURR * OFFSET);
    }
}
