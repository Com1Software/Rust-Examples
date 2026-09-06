use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Starting async timer...");

    for i in 1..=5 {
        println!("Tick {}", i);
        sleep(Duration::from_secs(1)).await;
    }

    println!("Timer complete");
}
