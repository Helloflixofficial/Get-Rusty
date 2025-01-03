use num_bigint::BigUint;
use tokio::time::{sleep, Duration};
async fn async_add_big_numbers(a: BigUint, b: BigUint) -> BigUint {
    sleep(Duration::from_secs(1)).await;
    a + b
}

#[tokio::main]
async fn main() {
    let num1 = BigUint::parse_bytes(b"123456789123456789123456789", 10).unwrap();
    let num2 = BigUint::parse_bytes(b"987654321987654321987654321", 10).unwrap();

    println!("Adding big numbers...");
    let result = async_add_big_numbers(num1, num2).await;

    println!("Result: {}", result);
}
