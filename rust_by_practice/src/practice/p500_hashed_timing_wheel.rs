use std::{
    marker::PhantomData,
    ptr::NonNull,
    sync::{
        Arc,
        atomic::{AtomicU8, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};

pub struct HashedTimingWheel {
    /// Tick count from timer start, time elapsed since timer start is
    /// current_tick * tick_duration
    current_tick: usize,
    tick_duration_ns: u128,
    wheel: Vec<Bucket>,

    /// The wheel size is round up to nearest 2^n, mask = 2^n -1,
    /// so bucket_index = expired_tick % 2^n = expired_tick & mask
    mask: usize,

    // TODO: add constraint for max pending timeouts
    /// System clock, as the time origin or epoch of timer start
    epoch: Instant,

    pub(crate) worker_state: Arc<AtomicU8>,

    incoming_queue: mpsc::Receiver<Timeout>,
    unprocessed_timeouts: Vec<Timeout>,
}

#[derive(Debug, Clone)]
pub struct Timer {
    epoch: Instant,
    task_sender: mpsc::Sender<Timeout>,
    timer_worker_state: Arc<AtomicU8>,
}

impl Timer {
    pub fn stop_timer(&self) {
        self.timer_worker_state
            .store(HashedTimingWheel::WORKER_STATE_SHUTDOWN, Ordering::Relaxed);
    }

    pub fn new_timeout<F>(&self, f: F, delay: Duration) -> Result<(), mpsc::SendError<Timeout>>
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        let deadline = (Instant::now() + delay)
            .duration_since(self.epoch)
            .as_nanos();
        let timeout = Timeout::new(Box::new(f), deadline);
        self.task_sender.send(timeout)
    }
}

impl HashedTimingWheel {
    const WORKER_STATE_INIT: u8 = 0;
    const WORKER_STATE_STARTED: u8 = 1;
    const WORKER_STATE_SHUTDOWN: u8 = 2;

    pub fn with_default() -> Timer {
        let epoch = Instant::now();
        let (tx, rx) = mpsc::channel();
        let mut timing_wheel = HashedTimingWheel::new(Duration::from_millis(100), 512, epoch, rx);
        let timer = Timer {
            epoch,
            task_sender: tx,
            timer_worker_state: timing_wheel.worker_state.clone(),
        };
        thread::spawn(move || {
            timing_wheel.start();
        });
        timer
    }

    fn new(
        tick_duration: Duration,
        ticks_per_wheel: usize,
        epoch: Instant,
        incoming_queue: mpsc::Receiver<Timeout>,
    ) -> HashedTimingWheel {
        assert!(
            ticks_per_wheel >= 64,
            "ticks per wheel must not less than 64"
        );
        assert!(
            tick_duration.as_millis() >= 10,
            "tick duration must not less than 10ms"
        );
        let (wheel, mask) = Self::create_wheel(ticks_per_wheel);

        Self {
            current_tick: 0,
            tick_duration_ns: tick_duration.as_nanos(),
            wheel,
            worker_state: Arc::new(AtomicU8::new(Self::WORKER_STATE_INIT)),
            mask,
            epoch,
            incoming_queue,
            unprocessed_timeouts: Vec::new(),
        }
    }

    fn create_wheel(ticks_per_wheel: usize) -> (Vec<Bucket>, usize) {
        let mut n = 1_usize;
        while n < ticks_per_wheel {
            n <<= 1;
        }
        let mut wheel = Vec::with_capacity(n);
        for _ in 0..n {
            wheel.push(Bucket::new());
        }
        (wheel, n - 1)
    }

    pub fn start(&mut self) {
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
                        let current_bucket = self.do_tick();
                        current_bucket.expire_timeouts(deadline);
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
        self.collect_unprocessed_timeouts();
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
            println!(
                "bucket_index: {}, timeout rounds: {}, timeout deadline: {}",
                bucket_index, timeout.rounds, timeout.deadline
            );
            self.wheel[bucket_index].push(timeout);
        }
    }

    fn collect_unprocessed_timeouts(&mut self) {
        for bucket in self.wheel.iter_mut() {
            for timeout in bucket.drain() {
                self.unprocessed_timeouts.push(timeout);
            }
        }
        while let Ok(timeout) = self.incoming_queue.try_recv() {
            self.unprocessed_timeouts.push(timeout);
        }
    }
}

/// Bucket is a doubly linked list of timeout task for a tick.
/// TODO: check drop and drain
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

    fn expire_timeouts(&mut self, deadline: u128) {
        let mut cursor = self.head;
        while let Some(node_ptr) = cursor {
            let node = unsafe { &mut *node_ptr.as_ptr() };
            let next = node.next;
            if node.timeout.rounds == 0 {
                let node = self.remove(node_ptr);
                let timeout = node.timeout;
                if timeout.deadline <= deadline {
                    (timeout.task)();
                } else {
                    panic!("timeout.deadline > deadline");
                }
            } else if node.timeout.is_cancelled() {
                self.remove(node_ptr);
            } else {
                node.timeout.rounds -= 1;
            }
            cursor = next;
        }
    }
}

impl Drop for Bucket {
    fn drop(&mut self) {
        let mut cursor = self.head;
        while let Some(node) = cursor {
            let node = unsafe { Box::from_raw(node.as_ptr()) };
            cursor = node.next;
        }
    }
}

struct Drain<'a> {
    cursor: Option<NonNull<BucketNode>>,
    bucket: &'a mut Bucket,
}

impl Bucket {
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

pub struct Timeout {
    task: Box<dyn FnOnce() + Send + 'static>,
    /// Nano seconds offset beyond the timing wheel start time.
    deadline: u128,
    /// Thread safe state.
    state: AtomicU8,
    /// The wheel level of task, when equal to zero, task is expiring, should be
    /// executed.
    rounds: usize,
}

type Task = Box<dyn FnOnce() + Send + 'static>;

impl Timeout {
    pub const STATE_INIT: u8 = 0;
    pub const STATE_CANCELLED: u8 = 1;
    pub const STATE_EXPIRED: u8 = 2;

    pub fn new(task: Task, deadline: u128) -> Timeout {
        Timeout {
            task,
            deadline,
            state: AtomicU8::new(Self::STATE_INIT),
            rounds: 0,
        }
    }

    pub fn cancel(&self) -> bool {
        self.state
            .compare_exchange(
                Self::STATE_INIT,
                Self::STATE_CANCELLED,
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
    fn test_bucket() {
        let mut bucket = Bucket::new();

        for _ in 0..5 {
            let timeout = Timeout::new(Box::new(|| {}), 0);
            dbg!(&timeout as *const _);
            bucket.push(timeout);
        }

        for timeout in bucket.drain() {
            dbg!(&timeout as *const _);
        }

        bucket.push(Timeout::new(Box::new(|| {}), 0));
    }

    #[test]
    fn test_timer() {
        let timer = HashedTimingWheel::with_default();

        for i in 0..5 {
            let start_instant = Instant::now();
            let delay = Duration::from_millis((i + 1) * 670);
            timer
                .new_timeout(
                    move || {
                        let now = Instant::now();
                        println!(
                            "task expired, start instant: {:?}, delay: {:?}, now: {:?}, diff: {:?}",
                            start_instant,
                            delay,
                            now,
                            now.duration_since(start_instant + delay), // timer start delay only affect task within delay
                        );
                    },
                    delay,
                )
                .unwrap();
        }

        thread::sleep(Duration::from_secs(5));
        timer.stop_timer();
    }
}
