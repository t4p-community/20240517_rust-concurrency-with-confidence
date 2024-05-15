use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let task1 = async {
        println!("Task 1 starts");
        sleep(Duration::from_secs(5)).await;
        println!("Task 1 ends");
    };

    let task2 = async {
        println!("Task 2 starts");
        sleep(Duration::from_secs(5)).await;
        println!("Task 2 ends");
    };

    let task3 = async {
        println!("Task 3 starts");
        sleep(Duration::from_secs(5)).await;
        println!("Task 3 ends");
    };

    println!("Start Join!");

    // compare to other programming languages
    // - Java - similar to CompletableFuture.allOf
    // - JavaScript - similar to Promise.all
    // - Python - similar to asyncio.gather
    // - C# - similar to Task.WhenAll
    tokio::join!(task1, task2, task3);
    println!("End Join!");
}
