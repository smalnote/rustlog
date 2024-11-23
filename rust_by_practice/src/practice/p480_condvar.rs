#[cfg(test)]
mod tests {
    use std::{
        mem,
        sync::{Arc, Condvar, Mutex},
        thread,
        time::{self, Duration},
    };

    struct Chan<T> {
        data: Arc<Mutex<Closeable<T>>>,
        cond: Arc<Condvar>,
    }

    enum Closeable<T> {
        Some(T),
        None,
        Closed,
    }

    impl<T> Closeable<T> {
        fn take(&mut self) -> Self {
            mem::replace(self, Closeable::None)
        }

        fn unwrap(self) -> T {
            match self {
                Closeable::Some(data) => data,
                _ => panic!("no data"),
            }
        }
    }

    struct Sender<T> {
        chan: Chan<T>,
    }

    struct Receiver<T> {
        chan: Chan<T>,
    }

    impl<T> Chan<T> {
        fn single() -> (Sender<T>, Receiver<T>) {
            let rx_chan = Self::new();
            let tx_chan = rx_chan.clone();
            (Sender { chan: tx_chan }, Receiver { chan: rx_chan })
        }

        fn new() -> Self {
            Self {
                data: Arc::new(Mutex::new(Closeable::None)),
                cond: Arc::new(Condvar::new()),
            }
        }

        fn clone(&self) -> Chan<T> {
            Self {
                data: self.data.clone(),
                cond: self.cond.clone(),
            }
        }
    }

    impl<T> Sender<T> {
        fn send(&self, data: T) {
            self.send_raw(Closeable::Some(data));
        }

        fn send_raw(&self, data: Closeable<T>) {
            let mut mutex = self.chan.data.lock().unwrap();
            loop {
                match *mutex {
                    Closeable::Closed => panic!("send on closed channel"),
                    Closeable::Some(_) => {
                        mutex = self.chan.cond.wait(mutex).unwrap();
                    }
                    Closeable::None => {
                        *mutex = data;
                        self.chan.cond.notify_one();
                        return;
                    }
                }
            }
        }

        fn close(&self) {
            self.send_raw(Closeable::Closed);
        }
    }

    impl<T> Receiver<T> {
        fn receive(&self) -> Option<T> {
            let mut mutex = self.chan.data.lock().unwrap();
            loop {
                match *mutex {
                    Closeable::Closed => return None,
                    Closeable::Some(_) => {
                        let data = mutex.take();
                        self.chan.cond.notify_one();
                        return Some(data.unwrap());
                    }
                    Closeable::None => {
                        mutex = self.chan.cond.wait(mutex).unwrap();
                    }
                }
            }
        }
    }

    #[test]
    fn test_chan() {
        let (tx, rx) = Chan::<usize>::single();

        thread::scope(|scoped| {
            scoped.spawn(move || {
                let mut first = 1;
                let mut second = 1;
                for _ in 0..10 {
                    tx.send(first);
                    first += second;
                    mem::swap(&mut first, &mut second);
                    thread::sleep(Duration::from_millis(first as u64));
                }
                tx.close();
            });
            scoped.spawn(move || {
                let start = time::Instant::now();
                while let Some(data) = rx.receive() {
                    dbg!(start.elapsed().as_millis(), data);
                }
            });
        });
    }
}
