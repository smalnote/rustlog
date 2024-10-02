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
}
