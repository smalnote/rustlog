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
    tick: u64,
    tick_duration: Duration,
    wheel: Vec<Bucket>,
    worker_state: Arc<AtomicU8>,

    /// wheel size -> 2^n -> -1 for modulo
    mask: u64,

    /// TODO: add constraint for max pending timeouts
    /// System clock, as the time origin or epoch for this timer
    epoch: Instant,
    /// System clock nano seconds when worker thread start
    /// TODO: nano seconds prefer u128
    start_time: u64,

    incoming_queue: mpsc::Receiver<Timeout>,
    unprocessed_timeouts: Vec<Timeout>,
}

pub struct Handle {
    worker_state: Arc<AtomicU8>,
}

/// TODO: create timeout from start time.
impl Handle {
    pub fn stop(&self) {
        self.worker_state
            .store(HashedTimingWheel::WORKER_STATE_SHUTDOWN, Ordering::Relaxed);
    }
}

impl HashedTimingWheel {
    const WORKER_STATE_INIT: u8 = 0;
    const WORKER_STATE_STARTED: u8 = 1;
    const WORKER_STATE_SHUTDOWN: u8 = 2;

    /// TODO: return handler, not the timer
    pub fn new(
        tick_duration: Duration,
        ticks_per_wheel: usize,
        epoch: Instant,
        incoming_queue: mpsc::Receiver<Timeout>,
    ) -> HashedTimingWheel {
        assert!(
            ticks_per_wheel > 0,
            "ticks per wheel must greater than zero"
        );
        let wheel = Self::create_wheel(ticks_per_wheel);
        let mask = (wheel.len() - 1) as u64;

        Self {
            tick: 0,
            tick_duration,
            wheel,
            worker_state: Arc::new(AtomicU8::new(Self::WORKER_STATE_INIT)),
            mask,
            epoch,
            start_time: 0,
            incoming_queue,
            unprocessed_timeouts: Vec::new(),
        }
    }

    fn create_wheel(ticks_per_wheel: usize) -> Vec<Bucket> {
        let mut n = 1_usize;
        while n < ticks_per_wheel {
            n <<= 1;
        }
        let mut wheel = Vec::with_capacity(n);
        for _ in 0..n {
            wheel.push(Bucket::new());
        }
        wheel
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
                    self.start_time = self.epoch.elapsed().as_nanos() as u64;
                    dbg!(self.start_time);
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

    pub fn handle(&self) -> Handle {
        Handle {
            worker_state: self.worker_state.clone(),
        }
    }

    fn wait_for_next_tick(&self) -> u64 {
        let deadline = self.tick_duration.as_nanos() as u64 * (self.tick + 1);
        loop {
            let current = self.epoch.elapsed().as_nanos() as u64 - self.start_time;
            if deadline <= current {
                return current;
            }
            let sleep_ms = (deadline - current + 999_999) / 1_000_000;
            thread::sleep(Duration::from_millis(sleep_ms));
        }
    }

    fn do_tick(&mut self) -> &mut Bucket {
        let index = self.tick & self.mask;
        self.tick += 1;
        &mut self.wheel[index as usize]
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
            let mut expired_tick = timeout.deadline / self.tick_duration.as_nanos() as u64;
            if expired_tick < self.tick {
                // Ensure we don't schedule for past
                expired_tick = self.tick
            }
            timeout.rounds = if expired_tick < self.tick {
                0
            } else {
                (expired_tick - self.tick) / self.wheel.len() as u64
            };
            let bucket_index = (expired_tick & self.mask) as usize;
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

    fn expire_timeouts(&mut self, deadline: u64) {
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
    task: Box<dyn FnOnce()>,
    /// Nano seconds offset beyond the timing wheel start time.
    deadline: u64,
    /// Thread safe state.
    state: AtomicU8,
    /// The wheel level of task, when equal to zero, task is expiring, should be
    /// executed.
    rounds: u64,
}

/// TODO: Box<dyn FnOnce()> can be !Send, try a safe way
unsafe impl Send for Timeout {}

impl Timeout {
    pub const STATE_INIT: u8 = 0;
    pub const STATE_CANCELLED: u8 = 1;
    pub const STATE_EXPIRED: u8 = 2;

    pub fn new(task: Box<dyn FnOnce()>, deadline: u64) -> Timeout {
        Timeout {
            task,
            deadline, // TODO: deadline base on timer start_time
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
    use std::time::SystemTime;

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
        let (tx, rx) = mpsc::channel();
        let epoch = Instant::now();

        thread::scope(|scoped| {
            scoped.spawn(move || {
                let mut timer = HashedTimingWheel::new(Duration::from_millis(100), 512, epoch, rx);
                let handle = timer.handle();
                scoped.spawn(move || {
                    thread::sleep(Duration::from_secs(6));
                    handle.stop();
                });
                timer.start();
            });
            scoped.spawn(move || {
                for i in 0..5 {
                    let timeout = Timeout::new(
                        Box::new(move || {
                            println!(
                                "task expired, current instant: {}, now: {:?}",
                                epoch.elapsed().as_nanos(),
                                SystemTime::now(),
                            );
                        }),
                        Duration::from_secs(i).as_nanos() as u64,
                    );
                    tx.send(timeout).unwrap();
                }
            });
        });
    }
}
