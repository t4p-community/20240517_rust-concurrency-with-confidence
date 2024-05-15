use rand::Rng;
use tokio::time::{sleep, Duration};

async fn sleep_for_a_while() {
    let mut rng = rand::thread_rng();
    let sleep_time = rng.gen_range(1..6);
    sleep(Duration::from_secs(sleep_time)).await;
}

#[tokio::main]
async fn main() {
    let task1 = async {
        sleep_for_a_while().await;
        println!("Task 1 completes first");
    };

    let task2 = async {
        sleep_for_a_while().await;
        println!("Task 2 completes first");
    };

    let task3 = async {
        sleep_for_a_while().await;
        println!("Task 3 completes first");
    };

    // similar to `tokio::join!` but only the result of the first task
    // to complete is returned
    // compare to other programming languages
    // - Java- similar to `CompletableFuture.anyOf`
    // - JavaScript - similar to `Promise.race`
    // - Python - similar to `asyncio.wait`
    // - C# - similar to `Task.WhenAny`
    tokio::select! {
        _ = task1 => println!("Task 1 wins the race"),
        _ = task2 => println!("Task 2 wins the race"),
        _ = task3 => println!("Task 3 wins the race"),
    }
}
