use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // define the tasks (just their bodies, no execution here)
    // a Future is returned by the async block
    // Future is a trait that represents a value that will be computed in the future
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

    // Execute the tasks sequentially
    task1.await;
    task2.await;
    task3.await;

    println!("All tasks completed!");
}
