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
///////////////////////////////////

#[tokio::main]
async fn main() {
    say_hii().await;
    say_hello().await;
    sleep(Duration::from_millis(1000)).await;
    say_world().await;
}


async fn say_hello() {
    sleep(Duration::from_millis(200)).await;
    println!("hello");
}

async fn say_world() {
    sleep(Duration::from_millis(100)).await;
    println!("world");
}

#[tokio::main]
async fn main() {
    let data1 = spawn(say_hello());
    let data2 = spawn(say_world());
    let _ = data1.await;
    let _ = data2.await;
    println!("! This is the end");
}
