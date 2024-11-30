use std::time::Instant;

fn sum_of_elements(n: usize) {
    // Create a vector of size n
    let data: Vec<usize> = (1..=n).collect();

    // Start timing
    let start = Instant::now();

    // Perform the O(n) operation: summing the elements
    let sum: usize = data.iter().sum();

    // End timing
    let duration = start.elapsed();

    println!("Sum of {} elements: {}", n, sum);
    println!("Time taken: {:?}", duration);
}

fn main() {
    // Demonstrate O(n) for different input sizes
    let sizes = [1_000, 10_000, 100_000, 1_000_000];

    for &size in &sizes {
        println!("\nInput size: {}", size);
        sum_of_elements(size);
    }
}
