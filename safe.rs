use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Use Arc (Atomic Reference Counted) to share ownership across threads
    let counter = Arc::new(Mutex::new(0)); // A thread-safe counter wrapped in Mutex
    let mut handles = vec![];

    // Spawn 10 threads to increment the counter concurrently
    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Clone the Arc to give each thread shared ownership
        let handle = thread::spawn(move || {
            // Lock the Mutex to gain mutable access to the counter
            let mut num = counter.lock().unwrap(); // Panic-safe unwrapping in case of errors
            *num += 1; // Increment the counter
        });
        handles.push(handle); // Collect thread handles for later joining
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap(); // Join each thread to ensure they complete
    }

    // Safely access the counter's final value
    println!("Final counter value: {}", *counter.lock().unwrap());
}
