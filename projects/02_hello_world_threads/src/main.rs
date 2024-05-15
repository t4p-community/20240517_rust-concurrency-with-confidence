use std::thread;
use std::time::Duration;

fn main() {
    // spawn a new thread - the thread is "spawned" when thread::spawn is called
    // a closure is passed to thread::spawn
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from the spawned thread! {}", i);
            // sleep will nudge the operating system to switch to another thread
            thread::sleep(Duration::from_millis(1));
        }
    });

    // continue execution in the main thread
    for i in 1..5 {
        println!("Hello from the main thread! {}", i);
        // sleep will nudge the operating system to switch to another thread
        thread::sleep(Duration::from_millis(1));
    }

    // wait for the spawned thread to finish
    // "join" is the common term for waiting for a thread to finish
    handle.join().unwrap();

    // the join above consumes the handle, so it can't be used again
    // uncommenting the line below will result in a compile error
    // println!("{:?}", handle);
}
