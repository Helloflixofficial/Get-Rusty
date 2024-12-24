fn main() {
    let mut i = 0; // Declare a mutable variable
    let max_limit = 20; // Maximum value for the counter

    println!("Starting the counter:");

    while i < max_limit {
        i += 1; // Increment

        // Skip even numbers using a conditional
        if i % 2 == 0 {
            println!("Skipping even number: {}", i);
            continue;
        }

        // Stop if a certain condition is met
        if i == 15 {
            println!("Reached the special number 15, stopping early.");
            break;
        }

        // Print the current value of `i`
        println!("Counter at: {}", i);
    }

    // Show final value
    println!("Counter stopped at: {}", i);
}
