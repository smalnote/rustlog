use std::{
    future::Future,
    pin::Pin,
    sync::{
        mpsc::{self},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
    thread,
    time::{Duration, SystemTime},
};

use futures::{
    future::{BoxFuture, FutureExt},
    task::{self, ArcWake},
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

pub struct SharedState {
    completed_time: Option<SystemTime>,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = SystemTime;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if let Some(completed_time) = shared_state.completed_time.take() {
            Poll::Ready(completed_time)
        } else {
            shared_state.waker = Some(Waker::clone(&cx.waker()));
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = SharedState {
            completed_time: None,
            waker: None,
        };
        let shared_state = Arc::new(Mutex::new(shared_state));

        let thread_shared_state = Arc::clone(&shared_state);
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed_time = Some(SystemTime::now());

            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

pub struct Executor {
    ready_queue: mpsc::Receiver<Arc<Task>>,
}

#[derive(Clone)]
pub struct Spawner {
    task_sender: mpsc::SyncSender<Arc<Task>>,
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, SystemTime>>>,
    task_sender: mpsc::SyncSender<Arc<Task>>,
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = mpsc::sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    pub fn spawn<F: Future<Output = SystemTime> + 'static + Send>(&self, future: F) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many task queued");
    }
}

impl Executor {
    pub fn run(&self) {
        let thread_id = thread::current().id();
        while let Ok(task) = self.ready_queue.recv() {
            println!("thread #{:?}: receive task...", thread_id);
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = task::waker_ref(&task);
                let context = &mut Context::from_waker(&waker);

                // Future.poll trigger async flow forward
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{executor::block_on, join};
    use std::{thread, time::Duration};

    #[test]
    fn async_await_primer() {
        async fn learn_song() -> String {
            thread::sleep(Duration::from_micros(100));
            "Red - Taylor Swift".to_string()
        }

        async fn sing_song(song: String) -> String {
            thread::sleep(Duration::from_micros(200));
            format!("Singed {}!", song)
        }

        async fn dance() -> String {
            thread::sleep(Duration::from_micros(200));
            "Danced".to_string()
        }

        async fn sing_and_dance() -> String {
            let song = learn_song().await;
            let sing = sing_song(song);
            let dance = dance();
            let (sing, dance) = join!(sing, dance);
            format!("{}, {}", sing, dance)
        }

        let script = block_on(sing_and_dance());
        println!("{}", script);
    }

    /// # Real `Future` trait:
    ///
    /// ```
    /// trait Future {
    /// type Output;
    /// fn poll(
    ///     self: Pin<&mut Self>,
    ///     cx: &mut Context<'_>,
    /// ) -> Poll<Self::Output>;
    /// ```
    /// Note the change from `&mut self` to `Pin<&mut Self>`:
    /// Pin<&mut Self>  allows us to create futures that are immovable.
    /// and the change from `wake: fn()` to `cx: &mut Context<'_>`:
    /// cx for telling the future executor that the future in question
    /// should be pulled.
    ///
    #[test]
    fn test_future_trait() {
        let completed_time = block_on(async { TimerFuture::new(Duration::from_millis(1)).await });
        println!("Time future completed at: {:?}", completed_time);

        async fn after(duration: Duration) -> SystemTime {
            TimerFuture::new(duration).await
        }

        let completed_time = block_on(after(Duration::from_micros(1000)));
        println!("Time future completed at: {:?}", completed_time);
    }

    #[test]
    fn test_custom_executor() {
        let (executor, spawner) = new_executor_and_spawner();

        // Rust compiles the async block into state machines that implement the `Future` trait.
        // Specifically, type `impl Future<Output = System> + 'static + Send`.
        // Inside this block, there are another two `Future`s for `await`ing,
        // the `.await` keyword breaks down the block into different state.
        // Each state represents a point in the block before or after and `.await`.
        // State machine: Future1: A --future2_10ms--> B --future3_10ms--> C
        // Executor run the state machine:
        //   Future1.poll(), state machine forward one step
        //     run state A
        //   Future1.poll() -> Future2.poll() -> Pending -> Move control back to executor
        //     after 10ms wake, context.wake() put task(with Future2) back to executor's queue
        //     executor receive task of Future2
        //   Future1.poll() -> Future2.poll() -> Ready(), state machine forward one step
        //     run state B
        //   Future1.poll() -> Future3.poll() -> Pending -> Move control back to executor
        //     after 10ms wake, context.wake() put task(with Future3) back to executor's queue
        //     executor receive task of Future3
        //   Future1.poll() -> Future3.poll() -> Ready(), state machine forward one step
        //     run state C
        //   Future1.poll() -> Ready, over.
        spawner.spawn(async move {
            // state: A
            let thread_id = thread::current().id();
            println!("thread #{:?}: howdy!", thread_id);

            // await
            let _ = TimerFuture::new(Duration::from_millis(10)).await;

            // state: B
            println!("thread #{:?}: middle!", thread_id);

            // await
            let now = TimerFuture::new(Duration::from_millis(10)).await;

            // state: C
            println!("thread #{:?}: done!", thread_id);
            now
        });

        drop(spawner);

        executor.run();
    }
}
