use rand::Rng;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

// make the thread sleep for a random time
fn random_sleep() -> u64 {
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(1..5);
    thread::sleep(Duration::from_secs(sleep_time));
    sleep_time
}

fn main() {
    // create an Arc to share ownership of the RwLock containing the data
    // RwLock allows multiple readers or one writer at a time
    let data = Arc::new(RwLock::new(0));

    let mut handles = vec![];

    // spawn some threads to read the data
    for thread_id in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // - uncomment the next code line, and the first & second writes
            //   below must complete before these read threads can get a lock
            // - leave it commented and multile threads will acquire a read
            //   lock at the same time
            // thread::sleep(Duration::from_secs(1));
            let read_lock = data.read().unwrap();
            println!("Thread {}: First Read Value: {}", thread_id, *read_lock);
            random_sleep();
            println!("Thread {}: Second Read Value: {}", thread_id, *read_lock);
            // read_lock is dropped here
        });
        handles.push(handle);
    }

    // spawn a thread to write the data
    {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut write_lock = data.write().unwrap();
            *write_lock += 10;
            println!("Write Thread: First Written Value: {}", *write_lock);
            // thread::sleep(Duration::from_secs(1));
            *write_lock += 10;
            println!("Write Thread: Second Written Value: {}", *write_lock);
            // write_lock is dropped here
        });
        handles.push(handle);
    }

    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // read the final value in the main thread
    let final_value = *data.read().unwrap();
    println!("Final value: {}", final_value);
}
