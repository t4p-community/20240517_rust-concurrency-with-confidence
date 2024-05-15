use tokio::time::{sleep, Duration};

async fn function_task_with_param_and_return(duration: Duration) -> &'static str {
    println!("Function Task Starts");
    sleep(duration).await;
    println!("Function Task Ends");
    "Function Task completed with result"
}

#[tokio::main]
async fn main() {
    let block_task = async {
        println!("Block Task Starts");
        sleep(Duration::from_secs(2)).await;
        println!("Block Task Ends");
    };

    let closure_task_with_param_and_return = |duration: Duration| async move {
        println!("Closure Task Starts");
        sleep(duration).await;
        println!("Closure Task Ends");
        "Closure Task completed with result" // Return value
    };

    // the Block Task, Closure Task and Function Task are all Futures
    // they are not executed until awaited

    block_task.await;

    let result = closure_task_with_param_and_return(Duration::from_secs(2)).await;
    println!("{}", result);

    let result = function_task_with_param_and_return(Duration::from_secs(2)).await;
    println!("{}", result);

    println!("All tasks completed!");
}
