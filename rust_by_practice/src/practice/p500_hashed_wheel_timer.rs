use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    ptr::NonNull,
    sync::{
        atomic::{AtomicU8, Ordering},
        mpsc, Arc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
use tokio::runtime::Runtime;

/// HashedWheelTimer implements a timer with single level timing wheel.
/// Timeout tasks can be submitted into a [`mpsc::channel`] from multiple thread.
/// The timer launch a separate thread polls tasks from the channel and put them
/// into the wheel every tick. The default tick interval is 100ms, and wheel size
/// is 512. Within the wheel, there are bucket for every tick, every bucket is a
/// doubly linked list, timeout tasks polled from channel are appended at the end
/// of bucket with a round. When timer tick a bucket, it decrease the round of all
/// timeout task in it, if round is zero, means reaching task timeout deadline,
/// execute them in a separate thread pool.
///
/// Inspired by: [**Netty HashedWheelTimer**](https://sourcegraph.com/github.com/netty/netty/-/blob/common/src/main/java/io/netty/util/HashedWheelTimer.java)
pub struct HashedWheelTimer {
    /// Tick count from timer start, time elapsed since timer start is
    /// current_tick * tick_duration
    current_tick: usize,
    tick_duration_ns: u128,
    wheel: Vec<Bucket>,

    /// The wheel size is round up to nearest 2^n, mask = 2^n -1,
    /// so bucket_index = expired_tick % 2^n = expired_tick & mask
    mask: usize,

    /// System clock, as the time origin or epoch of timer start
    epoch: Instant,

    worker_state: Arc<AtomicU8>,

    incoming_queue: mpsc::Receiver<Timeout>,

    /// Executor for executing timeout task without blocking the scheduling
    /// thread.
    executor: Runtime,
}

type Task = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// Emitter is a thread safe handler for submitting timeout task, if succeed,
/// the emitter return a canceller for cancelling timeout task before the task
/// expiring. This type implements Clone trait, user can have multiple instance,
/// submit tasks from different threads.
#[derive(Debug, Clone)]
pub struct Emitter {
    epoch: Instant,
    task_sender: mpsc::Sender<Timeout>,
}

impl Emitter {
    /// Submit a timeout task with delay. Tasks are enqueued into a mpsc channel,
    /// if failed, returns a SendError, otherwise a canceller for cancelling
    /// task.
    pub fn new_timeout<F>(&self, f: F, delay: Duration) -> Result<Canceller, &'static str>
    where
        F: Future<Output = ()>,
        F: Send + 'static,
    {
        let deadline = (Instant::now() + delay)
            .duration_since(self.epoch)
            .as_nanos();
        let timeout = Timeout::new(Box::pin(f), deadline);
        let canceller = Canceller {
            timeout_state: timeout.state.clone(),
        };
        self.task_sender
            .send(timeout)
            .map(|_| canceller)
            .map_err(|_| "timer is already stopped")
    }
}

/// A handle for cancelling emitted timeout, succeed if timeout is pending,
/// otherwise return a error.
pub struct Canceller {
    timeout_state: Arc<AtomicU8>,
}

impl Canceller {
    /// Cancel timeout before expiring, if the timeout already cancelled or
    /// expired, returns a error.
    pub fn cancel(&self) -> Result<(), &'static str> {
        match self.timeout_state.compare_exchange(
            Timeout::STATE_INIT,
            Timeout::STATE_CANCELLED,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => Ok(()),
            Err(actual) => match actual {
                Timeout::STATE_CANCELLED => Err("timeout already cancelled"),
                Timeout::STATE_EXPIRED => Err("timeout already expired"),
                _ => panic!("invalid timeout state"),
            },
        }
    }
}

/// Handler for stopping timer and drain all timeout tasks.
pub struct Stopper {
    timer_worker_state: Arc<AtomicU8>,
    handle: JoinHandle<Vec<Task>>,
}

impl Stopper {
    /// Stop the timer and drain out all remaining tasks.
    pub fn stop_timer(self) -> Result<Vec<Task>, &'static str> {
        if self
            .timer_worker_state
            .compare_exchange(
                HashedWheelTimer::WORKER_STATE_STARTED,
                HashedWheelTimer::WORKER_STATE_SHUTDOWN,
                Ordering::Relaxed,
                Ordering::Relaxed,
            )
            .is_err()
        {
            // In this case, worker state can be INIT or SHUTDOWN, let it always be SHUTDOWN
            self.timer_worker_state
                .swap(HashedWheelTimer::WORKER_STATE_SHUTDOWN, Ordering::Relaxed);
        }

        self.handle.join().map_err(|_| "timer thread panics")
    }
}

impl HashedWheelTimer {
    const WORKER_STATE_INIT: u8 = 0;
    const WORKER_STATE_STARTED: u8 = 1;
    const WORKER_STATE_SHUTDOWN: u8 = 2;

    /// Creates a timer running in separate thread and return a [`Emitter`] and
    /// a [`Stopper`].
    pub fn with_default() -> (Emitter, Stopper) {
        Self::with_tick(Duration::from_millis(100), 512)
    }

    /// Creates timer with tick duration and wheel size, tick duration affects
    /// time resolution, wheel size is the number of [`Bucket`], more buckets
    /// means less mean length of bucket.
    pub fn with_tick(tick_duration: Duration, wheel_size: usize) -> (Emitter, Stopper) {
        let epoch = Instant::now();
        let (tx, rx) = mpsc::channel();
        let mut timing_wheel = HashedWheelTimer::new(tick_duration, wheel_size, epoch, rx);
        let emitter = Emitter {
            epoch,
            task_sender: tx,
        };
        let timer_worker_state = timing_wheel.worker_state.clone();
        let handle = thread::spawn(move || timing_wheel.start());

        let stopper = Stopper {
            timer_worker_state,
            handle,
        };
        (emitter, stopper)
    }

    fn new(
        tick_duration: Duration,
        wheel_size: usize,
        epoch: Instant,
        incoming_queue: mpsc::Receiver<Timeout>,
    ) -> HashedWheelTimer {
        assert!(wheel_size >= 32, "wheel size must not less than 32");
        assert!(
            tick_duration.as_millis() >= 10,
            "tick duration must not less than 10ms"
        );
        let (wheel, mask) = Self::create_wheel(wheel_size);
        let executor = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap();

        Self {
            current_tick: 0,
            tick_duration_ns: tick_duration.as_nanos(),
            wheel,
            worker_state: Arc::new(AtomicU8::new(Self::WORKER_STATE_INIT)),
            mask,
            epoch,
            incoming_queue,
            executor,
        }
    }

    fn create_wheel(wheel_size: usize) -> (Vec<Bucket>, usize) {
        let mut n = 1_usize;
        while n < wheel_size {
            n <<= 1;
        }
        let mut wheel = Vec::with_capacity(n);
        for _ in 0..n {
            wheel.push(Bucket::new());
        }
        (wheel, n - 1)
    }

    fn start(&mut self) -> Vec<Task> {
        match self.worker_state.load(Ordering::Relaxed) {
            Self::WORKER_STATE_INIT => {
                if self
                    .worker_state
                    .compare_exchange(
                        Self::WORKER_STATE_INIT,
                        Self::WORKER_STATE_STARTED,
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    )
                    .is_ok()
                {
                    loop {
                        let deadline = self.wait_for_next_tick();
                        self.transfer_timeouts_to_bucket();
                        let expired = self.do_tick().expired(deadline).collect::<Vec<_>>();
                        for timeout in expired {
                            self.executor.spawn(timeout.task);
                        }
                        if self.worker_state.load(Ordering::Relaxed) != Self::WORKER_STATE_STARTED {
                            break;
                        }
                    }
                }
            }
            Self::WORKER_STATE_STARTED => {}
            Self::WORKER_STATE_SHUTDOWN => panic!("cannot be started once stopped"),
            _ => panic!("invalid worker state"),
        }
        self.collect_unprocessed_timeouts()
    }

    fn wait_for_next_tick(&self) -> u128 {
        let deadline = self.tick_duration_ns * (self.current_tick + 1) as u128;
        loop {
            let current = self.epoch.elapsed().as_nanos();
            if deadline <= current {
                return current;
            }
            let sleep_ms = ((deadline - current + 999_999) / 1_000_000) as u64;
            thread::sleep(Duration::from_millis(sleep_ms));
        }
    }

    fn do_tick(&mut self) -> &mut Bucket {
        let index = self.current_tick & self.mask;
        self.current_tick += 1;
        &mut self.wheel[index]
    }

    fn transfer_timeouts_to_bucket(&mut self) {
        // transfer only max. 100_000 timeouts per tick to prevent a thread to stale the
        // worker thread
        for _ in 0..100_000 {
            let timeout = self.incoming_queue.try_recv();
            if timeout.is_err() {
                break;
            }
            let mut timeout = timeout.unwrap();
            if timeout.is_cancelled() {
                continue;
            }
            let mut expired_tick = (timeout.deadline / self.tick_duration_ns) as usize;
            if expired_tick < self.current_tick {
                // Ensure we don't schedule for past
                expired_tick = self.current_tick
            }
            timeout.rounds = if expired_tick < self.current_tick {
                0
            } else {
                (expired_tick - self.current_tick) / self.wheel.len()
            };
            let bucket_index = expired_tick & self.mask;
            self.wheel[bucket_index].push(timeout);
        }
    }

    fn collect_unprocessed_timeouts(&mut self) -> Vec<Task> {
        let mut unprocessed = Vec::new();
        for bucket in self.wheel.iter_mut() {
            for timeout in bucket.drain() {
                unprocessed.push(timeout.task);
            }
        }
        while let Ok(timeout) = self.incoming_queue.try_recv() {
            unprocessed.push(timeout.task);
        }
        unprocessed
    }
}

/// Bucket is a doubly linked list of timeout task for a tick.
struct Bucket {
    head: Option<NonNull<BucketNode>>,
    tail: Option<NonNull<BucketNode>>,
    _marker: PhantomData<Box<BucketNode>>,
}

unsafe impl Send for Bucket {}

struct BucketNode {
    timeout: Timeout,
    next: Option<NonNull<BucketNode>>,
    prev: Option<NonNull<BucketNode>>,
}

impl Bucket {
    fn new() -> Bucket {
        Bucket {
            head: None,
            tail: None,
            _marker: PhantomData,
        }
    }

    fn push_back_node(&mut self, node: NonNull<BucketNode>) {
        unsafe {
            (*node.as_ptr()).next = None;
            (*node.as_ptr()).prev = self.tail;
            let node = Some(node);

            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }
            self.tail = node;
        }
    }

    fn push(&mut self, timeout: Timeout) {
        let node = Box::leak(Box::new(BucketNode {
            timeout,
            next: None,
            prev: None,
        }));
        self.push_back_node(NonNull::from(node));
    }

    fn remove(&mut self, node: NonNull<BucketNode>) -> BucketNode {
        unsafe {
            let prev = (*node.as_ptr()).prev;
            let next = (*node.as_ptr()).next;
            match prev {
                None => self.head = next,
                Some(prev) => (*prev.as_ptr()).next = next,
            }
            match next {
                None => self.tail = prev,
                Some(next) => (*next.as_ptr()).prev = prev,
            }
            let mut node = *Box::from_raw(node.as_ptr());
            node.next = None;
            node.prev = None;
            node
        }
    }
}

impl Drop for Bucket {
    fn drop(&mut self) {
        for _ in self.drain() {}
    }
}

struct Expired<'a> {
    cursor: Option<NonNull<BucketNode>>,
    bucket: &'a mut Bucket,
    deadline: u128,
}

struct Drain<'a> {
    cursor: Option<NonNull<BucketNode>>,
    bucket: &'a mut Bucket,
}

impl Bucket {
    fn expired(&mut self, deadline: u128) -> Expired {
        Expired {
            cursor: self.head,
            bucket: self,
            deadline,
        }
    }

    fn drain(&mut self) -> Drain {
        Drain {
            cursor: self.head,
            bucket: self,
        }
    }
}

impl Iterator for Drain<'_> {
    type Item = Timeout;
    fn next(&mut self) -> Option<Self::Item> {
        // Eventually all element will be removed, simply take them out,
        // and clean head and tail in Drop trait.
        match self.cursor {
            None => None,
            Some(node_ptr) => {
                let node = unsafe { *(Box::from_raw(node_ptr.as_ptr())) };
                self.cursor = unsafe { (*node_ptr.as_ptr()).next };
                Some(node.timeout)
            }
        }
    }
}

impl Drop for Drain<'_> {
    fn drop(&mut self) {
        for _ in &mut *self {}
        self.bucket.head = None;
        self.bucket.tail = None;
    }
}

impl Iterator for Expired<'_> {
    type Item = Timeout;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node_ptr) = self.cursor {
            let node = unsafe { &mut *node_ptr.as_ptr() };
            let next: Option<NonNull<BucketNode>> = node.next;
            self.cursor = next;
            if node.timeout.rounds == 0 {
                let node = self.bucket.remove(node_ptr);
                let timeout = node.timeout;
                if !timeout.expire() {
                    continue;
                }
                if timeout.deadline <= self.deadline {
                    return Some(timeout);
                } else {
                    panic!("timeout.deadline > deadline");
                }
            } else if node.timeout.is_cancelled() {
                self.bucket.remove(node_ptr);
            } else {
                node.timeout.rounds -= 1;
            }
        }
        None
    }
}

impl Drop for Expired<'_> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

struct Timeout {
    task: Task,
    /// Nano seconds offset beyond the timing wheel start time.
    deadline: u128,
    /// Thread safe state.
    state: Arc<AtomicU8>,
    /// The wheel level of task, when equal to zero, task is expiring, should be
    /// executed.
    rounds: usize,
}

impl Timeout {
    const STATE_INIT: u8 = 0;
    const STATE_CANCELLED: u8 = 1;
    const STATE_EXPIRED: u8 = 2;

    fn new(task: Task, deadline: u128) -> Timeout {
        Timeout {
            task,
            deadline,
            state: Arc::new(AtomicU8::new(Self::STATE_INIT)),
            rounds: 0,
        }
    }

    fn expire(&self) -> bool {
        self.state
            .compare_exchange(
                Self::STATE_INIT,
                Self::STATE_EXPIRED,
                Ordering::Relaxed,
                Ordering::Relaxed,
            )
            .is_ok()
    }

    fn is_cancelled(&self) -> bool {
        self.state.load(Ordering::Relaxed) == Self::STATE_CANCELLED
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_dyn_future() {
        type Task = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

        let (tx, rx) = mpsc::channel::<Task>();

        thread::scope(|scoped| {
            scoped.spawn(move || {
                let executor = tokio::runtime::Builder::new_multi_thread().build().unwrap();
                let mut handles = Vec::new();
                while let Ok(task) = rx.recv() {
                    handles.push(executor.spawn(task)); // Task as a Future must implement Pin trait for polling.
                }
                for handle in handles {
                    let _ = executor.block_on(handle);
                }
            });
            scoped.spawn(move || {
                for i in 0..5 {
                    tx.send(Box::pin(async move {
                        println!("task #{}", i);
                    }))
                    .unwrap();
                }
            });
        });
    }

    #[test]
    fn test_bucket() {
        let mut bucket = Bucket::new();

        for _ in 0..5 {
            let timeout = Timeout::new(Box::pin(async {}), 0);
            dbg!(&timeout as *const _);
            bucket.push(timeout);
        }

        for timeout in bucket.drain() {
            dbg!(&timeout as *const _);
        }

        bucket.push(Timeout::new(Box::pin(async {}), 0));

        for _ in bucket.expired(0) {}
    }

    #[test]
    fn test_timer() {
        let (timer, stopper) = HashedWheelTimer::with_tick(Duration::from_millis(10), 32);

        for i in 0..5 {
            let start_instant = Instant::now();
            let delay = Duration::from_millis((i + 1) * 10 - 1);
            timer
                .new_timeout(
                    async move {
                        let now = Instant::now();
                        println!(
                            "task expired, expected delay: {:?}, actual delay: {:?} diff: {:?}",
                            delay,
                            now.duration_since(start_instant),
                            now.duration_since(start_instant + delay), // timer start delay only affect task within delay
                        );
                    },
                    delay,
                )
                .unwrap();
        }
        thread::sleep(Duration::from_millis(100));

        let unprocessed = stopper.stop_timer();
        dbg!(unprocessed.unwrap().len());
    }

    #[test]
    fn test_multi_emitter() {
        let (emitter, stopper) = HashedWheelTimer::with_tick(Duration::from_millis(10), 32);
        let second_emitter = emitter.clone();

        thread::scope(|scoped| {
            let spawn_start = Instant::now();
            scoped.spawn(move || {
                emitter
                    .new_timeout(
                        async move {
                            println!(
                                "first emitter, actual delay {:?}",
                                Instant::now().duration_since(spawn_start)
                            );
                        },
                        Duration::from_millis(30),
                    )
                    .unwrap();
            });
            scoped.spawn(move || {
                second_emitter
                    .new_timeout(
                        async move {
                            println!(
                                "second emitter, actual delay {:?}",
                                Instant::now().duration_since(spawn_start)
                            );
                        },
                        Duration::from_millis(660),
                    )
                    .unwrap();
                second_emitter
                    .new_timeout(
                        async move {
                            println!(
                                "second emitter, actual delay {:?}",
                                Instant::now().duration_since(spawn_start)
                            );
                        },
                        Duration::from_millis(720),
                    )
                    .unwrap();
            });
        });

        thread::sleep(Duration::from_millis(800));
        dbg!(stopper.stop_timer().unwrap().len());
    }

    #[test]
    fn test_cancel_timeout() {
        let (emitter, stopper) = HashedWheelTimer::with_default();

        let c1 = emitter
            .new_timeout(
                async {
                    println!("task 1");
                },
                Duration::from_millis(42),
            )
            .unwrap();
        let c2 = emitter
            .new_timeout(
                async {
                    println!("task 2");
                },
                Duration::from_millis(242),
            )
            .unwrap();

        thread::sleep(Duration::from_millis(150));
        assert!(c1.cancel().is_err());

        assert!(c2.cancel().is_ok());
        assert!(c2.cancel().is_err());
        thread::sleep(Duration::from_millis(150));

        let unprocessed = stopper.stop_timer().unwrap();
        assert_eq!(unprocessed.len(), 0);
    }
}
