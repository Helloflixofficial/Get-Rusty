fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(4.0, 0.0);
    
    match result {
        Ok(value) => println!("The result is: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
///java to rust collage,.,.,.,,.,


use std::fs::File;
use std::io::{self, Read};

fn read_file_with_question_mark(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn unwrap_example() {
    let result = divide(10.0, 0.0);
    let value = result.unwrap();
    println!("Unwrapped value: {}", value);
}

fn expect_example() {
    let result = divide(10.0, 0.0);
    let value = result.expect("Division failed!");
    println!("Expected value: {}", value);
}

fn find_in_array(arr: &[i32], index: usize) -> i32 {
    arr.get(index).unwrap()
}

fn main() {
    match read_file_with_question_mark("example.txt") {
        Ok(content) => println!("File content (?):\n{}", content),
        Err(e) => println!("Error reading file (?): {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(value) => println!("Division result: {}", value),
        Err(e) => println!("Division error: {}", e),
    }

    // unwrap_example();
    // expect_example();

    let arr = [1, 2, 3, 4];
    let value = find_in_array(&arr, 2);
    println!("Found value in array: {}", value);
}
