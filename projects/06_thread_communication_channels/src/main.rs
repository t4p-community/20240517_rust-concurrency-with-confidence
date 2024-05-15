use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn random_sleep() -> u64 {
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(1..5);
    std::thread::sleep(Duration::from_secs(sleep_time));
    sleep_time
}

fn main() {
    // create a channel with a sender and receiver
    let (tx, rx) = mpsc::channel();

    // spawn a thread to send messages
    let sender_handle = thread::spawn(move || {
        let messages = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("thread"),
        ];

        for message in messages {
            println!("Sending a message from the sender thread");
            tx.send(message).unwrap();
            random_sleep();
        }
    });

    // in the main thread, receive messages
    random_sleep();
    println!("Main thread is ready to receive messages");
    for received in rx {
        println!("Received: {}", received);
    }

    // wait for the sender thread to finish
    sender_handle.join().unwrap();
}
