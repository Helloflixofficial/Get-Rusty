use tokio::time::{sleep, Duration};

async fn fetch_data_from_server(server_name: &str, delay_secs: u64) -> String {
    println!("Starting fetch from {} (will take {}s)...", server_name, delay_secs);
    sleep(Duration::from_secs(delay_secs)).await;
    let result = format!("Data from {}", server_name);
    println!("Finished fetch from {}.", server_name);
    result
}

#[tokio::main]
async fn main() {
    println!("Fetching data concurrently...");

    let (data1, data2) = tokio::join!(
        fetch_data_from_server("Server X", 3), 
        fetch_data_from_server("Server Y", 2) 
    );

    println!("\nReceived concurrently:");
    println!("From Server X: {}", data1);
    println!("From Server Y: {}", data2);

    println!("All concurrent fetches finished.");
}
