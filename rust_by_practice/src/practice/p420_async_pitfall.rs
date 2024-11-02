#[cfg(test)]
mod tests {
    use std::{
        thread,
        time::{Duration, Instant},
    };

    use futures::future::join_all;

    #[tokio::test]
    async fn cpu_blocking_block_executor() {
        async fn sleep_ms(start: &Instant, id: u64, duration_ms: u64) {
            std::thread::sleep(Duration::from_millis(duration_ms));
            println!(
                "thread #{:?} future {id} slept for {duration_ms}ms, finished after {}ms",
                thread::current().id(),
                start.elapsed().as_millis()
            );
        }

        let start = Instant::now();
        // every std::tread::sleep() block cpu which then block executor,
        // leads to later futures start after former futures completed
        let sleep_futures = (1..=10).map(|id| sleep_ms(&start, id, id * 10));
        join_all(sleep_futures).await;
    }

    #[tokio::test]
    async fn cpu_blocking_does_not_block_spawn_blocking_executor() {
        fn sleep_ms(start: &Instant, id: u64, duration_ms: u64) {
            std::thread::sleep(Duration::from_millis(duration_ms));
            println!(
                "thread #{:?} future {id} slept for {duration_ms}ms, finished after {}ms",
                thread::current().id(),
                start.elapsed().as_millis()
            );
        }

        let start = Instant::now();
        let sleep_futures =
            (1..=10).map(|id| tokio::task::spawn_blocking(move || sleep_ms(&start, id, id * 10)));
        join_all(sleep_futures).await;
    }

    #[tokio::test]
    async fn tokio_sleep_future_does_not_block_executor() {
        async fn sleep_ms(start: &Instant, id: u64, duration_ms: u64) {
            tokio::time::sleep(Duration::from_millis(duration_ms)).await;
            println!(
                "thread #{:?} future {id} slept for {duration_ms}ms, finished after {}ms",
                thread::current().id(),
                start.elapsed().as_millis()
            );
        }

        let start = Instant::now();
        // every std::tread::sleep() block cpu which then block executor,
        // leads to later futures start after former futures completed
        let sleep_futures = (1..=10).map(|id| sleep_ms(&start, id, id * 10));
        join_all(sleep_futures).await;
    }

    /// Async block and function's returned local variables are wrapped inside future,
    /// some of those variables can hold pointers to other local variables. Because of
    /// that, the future should never be moved to a different memory location, as it
    /// would invalidate those pointers.
    /// `Pin` prevents moving the wrapped variables in memory.
    /// Data that contains pointers to itself is called self-referential.
    /// `Pin` is a wrapper around a reference. And object cannot be moved from its place
    /// using a pinned pointer. However, it can still be moved through an unpinned pointer.
    /// The `poll` method of the `Future` trait uses `Pin<&mut Self>` instead of `&mut Self`
    /// to refer to the instance. That's why it can only be called on a pinned pointer.
    #[tokio::test]
    async fn pin_variables_in_memory() {
        use tokio::sync::{mpsc, oneshot};
        use tokio::task::spawn;
        use tokio::time::{sleep, Duration};
        let (sender, receiver) = mpsc::channel(10);
        spawn(worker(receiver));
        for i in 0..100 {
            let response = do_work(&sender, i).await;
            println!("work result for iteration {}: {}", i, response);
        }

        struct Work {
            input: u32,
            respond_on: oneshot::Sender<u32>,
        }

        async fn worker(mut receiver: mpsc::Receiver<Work>) {
            // the `Future` returned by sleep() can only be called on a pinned pointer
            let mut timeout_future = Box::pin(sleep(Duration::from_millis(100)));
            loop {
                tokio::select! {
                    Some(work) = receiver.recv() => {
                        sleep(Duration::from_millis(10)).await;
                        work.respond_on.send(work.input * 1000).unwrap();
                    },
                    _ = &mut timeout_future => {
                        println!("worker timeout");
                        timeout_future = Box::pin(sleep(Duration::from_millis(100)));
                    },
                }
            }
        }

        async fn do_work(sender: &mpsc::Sender<Work>, input: u32) -> u32 {
            let (respond_on, receiver) = oneshot::channel();
            let work = Work { input, respond_on };
            sender.send(work).await.unwrap();
            receiver.await.unwrap()
        }
    }
}
