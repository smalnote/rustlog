use std::{
    thread,
    time::{Duration, SystemTime},
};

use rust_by_practice::practice::p400_async_await::*;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn(async {
        let thread_id = thread::current().id();
        println!("thread #{:?}: async block start!", thread_id);
        // Wait for our timer future to complete after two seconds.
        let now = TimerFuture::new(Duration::from_millis(100)).await;
        println!("thread #{:?}: async block done!", thread_id);
        now
    });

    async fn wait() -> SystemTime {
        let thread_id = thread::current().id();
        println!("thread #{:?}: async function start!", thread_id);
        let now = TimerFuture::new(Duration::from_millis(50)).await;
        println!("thread #{:?}: async function done!", thread_id);
        now
    }

    let future = wait();
    spawner.spawn(future);

    spawner.spawn(TimerFuture::new(Duration::from_millis(10)));

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    println!("before starting executor");
    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();

    /*
      standard output:
       before starting executor
       thread #ThreadId(1): polling...
       thread #ThreadId(1): async block start!
       thread #ThreadId(1): polling...
       thread #ThreadId(1): async function start!
       thread #ThreadId(1): polling...
       thread #ThreadId(1): async function done!
       thread #ThreadId(1): polling...
       thread #ThreadId(1): async block done!

      The output indicates that async block/function is driven after first polling,
      aka lazy future.

      Since the async block and async function is declare first and return a future object,
      the `async block start` and `async function start` are printed after the executor
      starts and polls from their futures.

      The async block/function is running in main thread as well as the executor.run().
    */
}
