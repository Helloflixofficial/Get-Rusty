fn main() {
    let text = "Hello, world!";

    // Basic operations
    println!("Length: {}", text.len());
    println!("Is empty: {}", text.is_empty());

    // Comparison and searching
    println!("Starts with 'Hello': {}", text.starts_with("Hello"));
    println!("Contains 'world': {}", text.contains("world"));
    println!("Index of 'world': {}", text.find("world").unwrap());

    // Case manipulation
    println!("To uppercase: {}", text.to_uppercase());
    println!("To lowercase: {}", text.to_lowercase());

    // Trimming and padding
    let trimmed = text.trim();
    println!("Trimmed: {}", trimmed);
    println!("Padded with spaces: {}", trimmed.pad_left(20, ' '));

    // Conversion and formatting
    let number_str = "123";
    let number: i32 = number_str.parse().unwrap();
    println!("Parsed number: {}", number);

    let formatted_string = format!("The number is: {}", number);
    println!("Formatted string: {}", formatted_string);
}
