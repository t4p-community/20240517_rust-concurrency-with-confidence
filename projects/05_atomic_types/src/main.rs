use rand::Rng;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn random_sleep() -> u64 {
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(1..5);
    std::thread::sleep(Duration::from_secs(sleep_time));
    sleep_time
}

// simulate handling a web request
fn handle_request(active_connections: Arc<AtomicUsize>) {
    // increment the counter
    active_connections.fetch_add(1, Ordering::SeqCst);
    println!(
        "New connection! Active connections: {}",
        active_connections.load(Ordering::SeqCst)
    );

    // simulate request handling time
    random_sleep();

    // decrement the counter
    active_connections.fetch_sub(1, Ordering::SeqCst);
    println!(
        "Connection closed. Active connections: {}",
        active_connections.load(Ordering::SeqCst)
    );
}

fn main() {
    // shared atomic counter for active connections
    let active_connections = Arc::new(AtomicUsize::new(0));

    // vector to hold thread handles
    let mut handles = vec![];

    // thread to print the number of active connections every 500 milliseconds
    {
        let active_connections = Arc::clone(&active_connections);
        let handle = thread::spawn(move || loop {
            println!(
                "Active connections: {}",
                active_connections.load(Ordering::SeqCst)
            );
            thread::sleep(Duration::from_millis(500));
        });
        handles.push(handle);
    }

    // simulate 10 incoming connections
    for _ in 0..10 {
        // simulate waiting for the next request
        random_sleep();
        let active_connections = Arc::clone(&active_connections);
        let handle = thread::spawn(move || {
            handle_request(active_connections);
        });
        handles.push(handle);
    }

    // - wait for all request handling threads to finish
    // - the `skip` function is used to skip the first handle (the monitoring thread)
    // - to use the `skip` function an iterator must be created from the vector
    // - `into_iter` is used instead of `iter` because `join` consumes the handles
    // - "consumes" means that the handles are no longer available after the call to `join`
    for handle in handles.into_iter().skip(1) {
        // skip the first handle (the monitoring thread)
        handle.join().unwrap();
    }

    // handles vector is empty at this point, try uncommenting the next line to see the error
    // println!("{:?}", handles);

    // since the monitoring thread runs indefinitely, we can just print a final message and exit
    println!("All request handling threads completed.");
}
