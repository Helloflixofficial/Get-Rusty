fn main() {
    println!("Hello world!");
}


use std::time::Instant;

fn main() {
    // Start measuring time
    let start = Instant::now();

    // Code block to measure
    let mut sum = 0;
    for i in 0..1_000_000 {
        sum += i;
    }

    // End measuring time
    let duration = start.elapsed();

    println!("Result: {}", sum);
    println!("Execution Time: {:?}", duration);
}
