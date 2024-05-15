use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    // shared state: a tuple of a boolean flag and a counter
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    // clone the Arc for the thread
    let pair_clone = Arc::clone(&pair);

    // spawn a thread that will wait for the condition variable to be notified
    let handle = thread::spawn(move || {
        // get a reference to the mutex and condition variable inside the arc
        let (lock, cvar) = &*pair_clone;

        // lock the mutex
        let mut started = lock.lock().unwrap();

        // wait until the boolean flag is set to true
        // the while loop is to protect against spurious wakeups
        while !*started {
            started = cvar.wait(started).unwrap();
        }

        // the condition has been met
        println!("received notification, counter updated");
    });

    // simulate some work in the main thread
    thread::sleep(std::time::Duration::from_secs(2));

    // notify the waiting thread
    {
        // get a reference to the mutex and condition variable inside the arc
        let (lock, cvar) = &*pair;

        // lock the mutex
        let mut started = lock.lock().unwrap();

        // set the boolean flag to true
        *started = true;

        // notify the waiting thread that the condition has been met
        cvar.notify_one();
    }

    // wait for the spawned thread to finish
    handle.join().unwrap();
}
