//! This tests mod if for practicing concurrency of Rust in four aspects:
//!
//! #1 How to create threads to run multiple pieces of code at the same time
//! #2 Message-passing concurrency: where channels send messages between threads
//! #3 Shared-state concurrency: where multiple thread have access to some piece of data
//! #4 The Sync and Send trait, which extend Rust's concurrency guarantees to user-defined
//!    types as well as the types provided by the standard library
#[cfg(test)]
mod tests {
    use std::{
        sync::{mpsc, Arc, Mutex, RwLock},
        thread,
        time::Duration,
    };

    #[test]
    fn test_run_pieces_of_code_concurrently() {
        let spawned = thread::spawn(|| {
            for i in 1..=10 {
                println!("spawned thread: #{}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..=5 {
            println!("main thread: #{}", i);
            thread::sleep(Duration::from_millis(1));
        }

        spawned
            .join()
            .expect("Spawned thread should finished normally");
    }

    #[test]
    fn test_move_ownership_to_spawned_closure() {
        let message = "Hello, fearless concurrency!".to_string();

        let spawned = thread::spawn(move || {
            println!("Message from main thread `{}`", message);
        });

        spawned.join().unwrap();
    }

    #[test]
    fn test_channel_send_receive() {
        let (tx, rx) = mpsc::channel::<String>();

        thread::spawn(move || {
            tx.send("Hello from sender".to_string()).unwrap();
        });

        let message = rx.recv().unwrap();
        println!("Message received: `{}`", message);
    }

    #[test]
    fn test_loop_multiple_messages_on_receiver() {
        let (tx, rx) = mpsc::channel::<u32>();

        thread::spawn(move || {
            for i in 0..10 {
                tx.send(i as u32).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });

        let mut i: u32 = 0;
        for value in rx {
            assert_eq!(value, i);
            i += 1;
        }

        assert_eq!(i, 10 as u32);
    }

    #[test]
    fn test_multiple_senders_by_clone() {
        let (tx0, rx) = mpsc::channel();

        let tx1 = tx0.clone();
        thread::spawn(move || {
            for i in 1..100 {
                tx1.send(i).unwrap();
            }
        });

        let tx2 = tx0.clone();
        thread::spawn(move || {
            for i in 100..200 {
                tx2.send(i).unwrap();
            }
        });

        thread::spawn(move || {
            for i in 200..300 {
                tx0.send(i).unwrap();
            }
        });

        let mut sum = 0;
        for value in rx {
            sum += value;
        }

        assert_eq!(sum, (1 + 299) * 299 / 2);
    }

    #[test]
    fn test_use_mutex() {
        let m = Mutex::new(42);

        {
            let mut num = m.lock().unwrap();
            *num /= 6;
        }

        println!("{:?}", m);
    }

    #[test]
    fn test_use_atomic_rc_share_mutex() {
        let sum0 = Arc::new(Mutex::<u32>::new(0));

        let sum1 = Arc::clone(&sum0);
        let t1 = thread::spawn(move || {
            for i in 1..30 {
                let mut sum = sum1.lock().unwrap();
                *sum += i as u32;
            }
        });

        let sum2 = Arc::clone(&sum0);
        let t2 = thread::spawn(move || {
            for i in 30..70 {
                let mut sum = sum2.lock().unwrap();
                *sum += i as u32;
            }
        });

        let sum3 = Arc::clone(&sum0);
        let t3 = thread::spawn(move || {
            for i in 70..=100 {
                let mut sum = sum3.lock().unwrap();
                *sum += i as u32;
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();

        assert_eq!(*sum0.lock().unwrap(), (1 + 100) * 100 / 2);
    }

    #[test]
    fn test_use_atomic_rc_share_rw_lock() {
        let lock = Arc::new(RwLock::new(0));

        let mut adders = vec![];

        for _ in 0..3 {
            let lock = Arc::clone(&lock);
            let adder = thread::spawn(move || {
                for _ in 0..42 {
                    '_read: {
                        let value = lock.read().unwrap();
                        if *value >= 42 {
                            return;
                        }
                    }
                    '_add: {
                        let mut value = lock.write().unwrap();
                        if *value >= 42 {
                            return;
                        }
                        *value += 1;
                    }
                }
            });
            adders.push(adder);
        }

        for adder in adders {
            adder.join().unwrap();
        }

        let value = lock.read().unwrap();
        assert_eq!(*value, 42);
    }

    /// Implementing Send and Sync manually is unsafe
    /// Because types that made up of Send and Sync are automatically also Send and Sync,
    /// We don't have to manually implement those traits manually. As marker traits, they
    /// don't have any methods to implement. They're just useful for enforcing invariants
    /// related to concurrency.

    /// Allowing transference of ownership between threads with std::marker::Send trait
    /// The `Send` marker trait indicates that ownership of values of the type implementing
    /// Send can be transferred between threads.
    #[test]
    fn test_send_trait() {}

    /// Allowing access from multiple threads with std::marker::Sync trait
    /// The `Sync` marker trait indicates that it is safe for the type implementing `Sync`
    /// to be referenced from multiple threads.
    /// In other words, any type `T` is Sync if `&T` is Send, meaning the reference can
    /// be sent safely to another thread.
    #[test]
    fn test_sync_trait() {}
}
