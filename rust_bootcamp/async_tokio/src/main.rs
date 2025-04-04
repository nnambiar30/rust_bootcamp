use tokio::time::{sleep, Duration};

async fn greet(task_num: u64) -> String {
    sleep(Duration::from_secs(task_num)).await;
    format!("Task {}", task_num)
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let task1 = tokio::spawn(async { greet(1).await });
    let task2 = tokio::spawn(async { greet(2).await });
    let task3 = tokio::spawn(async { greet(3    ).await });
    let result1 = task1.await.unwrap();
    let result2 = task2.await.unwrap();
    let result3 = task3.await.unwrap();

    println!("{}", result1);
    println!("{}", result2);
    println!("{}", result3);
}
