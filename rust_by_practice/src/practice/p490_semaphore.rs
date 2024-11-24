#[cfg(test)]
mod tests {
    use std::{cell::UnsafeCell, sync::Arc};

    use tokio::sync::Semaphore;

    struct SemaphoreMutex<T> {
        data: UnsafeCell<T>,
        sema: Semaphore,
    }

    unsafe impl<T: Send> Send for SemaphoreMutex<T> {}
    unsafe impl<T: Sync> Sync for SemaphoreMutex<T> {}

    impl<T> SemaphoreMutex<T> {
        fn new(data: T) -> SemaphoreMutex<T> {
            Self {
                data: UnsafeCell::new(data),
                sema: Semaphore::const_new(1),
            }
        }
        async fn with_lock<F, R>(&self, f: F) -> R
        where
            F: FnOnce(&mut T) -> R,
        {
            let _ = self.sema.acquire().await.unwrap();
            f(unsafe { &mut *self.data.get() })
        }
    }

    #[tokio::test]
    async fn test_async_semaphore_mutex() {
        let counter = Arc::new(SemaphoreMutex::new(0));
        const CONCURR: usize = 14;
        const COUNT: usize = 12_588;
        let handles: Vec<_> = (0..CONCURR)
            .map(|_| {
                let counter = counter.clone();
                tokio::spawn(async move {
                    for _ in 0..COUNT {
                        counter.with_lock(|count| *count += 1).await;
                    }
                })
            })
            .collect();
        for handle in handles {
            handle.await.unwrap();
        }

        let count = counter.with_lock(|count| *count).await;
        assert_eq!(count, CONCURR * COUNT);
    }
}
