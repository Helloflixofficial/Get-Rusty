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
