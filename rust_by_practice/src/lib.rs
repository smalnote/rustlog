//! # rustlog
//!
//! `rustlog` is a collection of code snippets for practicing Rust.
//! file: src/lib.rs library crate of project rustlog
//! file: src/restaurant.rs

// make module available in src/bin/*, src/main.rs
pub mod restaurant;

pub mod practice;

use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

// re-export function with a short namespace rustlog::add_one
pub use self::practice::p320_documentation::add_one;

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_one() {
        let x = 41;
        let y = super::add_one(x);
        assert_eq!(y, 42);

        let z = super::practice::p320_documentation::add_one(y);
        assert_eq!(z, 43);
    }
}

pub struct ThreadPool {
    sender: Option<Sender<Job>>,
    threads: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            sender: Some(sender),
            threads,
        }
    }

    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(func)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(_) => {
                    println!("Worker {} stopped.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
