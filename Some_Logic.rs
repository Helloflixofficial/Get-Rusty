fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Use an iterator to filter even numbers and double them
    let doubled_evens: Vec<i32> = numbers
        .iter() // Create an iterator over the vector
        .filter(|&x| x % 2 == 0) // Keep only even numbers
        .map(|x| x * 2) // Double the even numbers
        .collect(); // Collect the results into a new vector

    println!("Original numbers: {:?}", numbers);
    println!("Doubled evens: {:?}", doubled_evens);
}
