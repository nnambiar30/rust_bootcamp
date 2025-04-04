use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Waiting...");
    sleep(Duration::from_secs(2)).await;
    println!("Done!");
}