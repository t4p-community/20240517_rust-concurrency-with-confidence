use std::thread;
use std::time::Duration;

// one way to test concurrent code is to place a thread::sleep in random places
// if the code continue to work, then concurrency is likely coded correctly
// if the code fails, then there is likely a flaw in your concurrency code
//
// another useful benefit of thread::sleep is to nudge the operating system
// to switch to another thread

fn main() {
    println!("Hello!");

    // sleep for 3 seconds
    thread::sleep(Duration::from_secs(3));

    // sleep for 500 milliseconds
    //thread::sleep(Duration::from_millis(500));

    println!("Goodbye!");
}
