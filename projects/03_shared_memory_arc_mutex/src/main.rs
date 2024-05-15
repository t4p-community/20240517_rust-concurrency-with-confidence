use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn random_sleep() -> u64 {
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(1..=5);
    thread::sleep(Duration::from_secs(sleep_time));
    sleep_time
}

fn main() {
    // - arc is an atomic reference counter
    // - mutex is a mutual exclusion lock
    // - arc is used to share ownership of the counter variable
    // - the main function will own the counter variable
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for thread_num in 0..10 {
        // - clone the arc counter, so it can be moved into the thread
        // - moved means moving ownership of the variable into the thread
        // - the closure function called in the new thread will take ownership
        //   of the counter variable
        let counter = Arc::clone(&counter);

        // - move the counter into the thread
        // - the move statement is used to move ownership of the counter variable
        //   into the thread
        let handle = thread::spawn(move || {
            let sleep_time = random_sleep();
            println!(
                "thread number: {} slept for: {} secs",
                thread_num, sleep_time
            );

            // will unlock the mutex when the variable goes out of scope
            let mut num = counter.lock().unwrap();

            // *num is dereferencing the MutexGuard to get the value inside
            *num += 1;
        });

        // collect a vector of thread handles, so they can be joined later
        handles.push(handle);
    }

    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // locking is needed for reading the value as well
    println!("Result: {}", *counter.lock().unwrap());
}
