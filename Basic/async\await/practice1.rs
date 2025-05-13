use std::io::{stdout, Write};
use tokio::time::{sleep, Duration};

async fn say_hello() {
    print!("hello, ");
    stdout().flush().unwrap();
}

async fn say_hii() {
    println!("hii sire nice to meet you");
    stdout().flush().unwrap();
}

async fn say_world() {
    println!("world!");
}

#[tokio::main]
async fn main() {
    say_hii().await;
    say_hello().await;
    sleep(Duration::from_millis(1000)).await;
    say_world().await;
}
