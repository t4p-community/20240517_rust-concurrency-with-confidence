use tokio::time::{sleep, Duration};

// `async` functions return a `Future` that can be awaited
async fn sleep_for_a_while() {
    println!("Going to sleep...");
    sleep(Duration::from_secs(5)).await;
    println!("Woke up after 5 seconds!");
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // spawn a task that will sleep for a while
    // `async` functions return a `Future` that can be awaited
    let handle = tokio::spawn(async {
        // await the `Future` returned by `sleep_for_a_while`
        sleep_for_a_while().await;
    });

    // await the future returned by `tokio::spawn`
    handle.await.unwrap();

    println!("Done sleeping!");
}
