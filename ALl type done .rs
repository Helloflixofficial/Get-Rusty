fn main() {
    // Scalar Types

    // Integer Types
    let x: i32 = -10;
    let y: u32 = 42;

    // Floating-Point Types
    let a: f32 = 3.14;
    let b: f64 = 2.71828;

    // Boolean Type
    let is_active: bool = true;

    // Character Type
    let letter: char = 'A';
    let emoji: char = '😊';

    // Compound Types

    // Tuple
    let tup: (i32, f64, char) = (500, 6.4, 'Z');
    let (x, y, z) = tup; // Destructuring

    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];

    // User-Defined Types

    // Struct
    struct Point {
        x: f64,
        y: f64,
    }

    let p = Point { x: 1.0, y: 2.0 };

    // Enum
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;

    // Option Type
    let some_number: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;

    // Result Type
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(numerator / denominator)
        }
    }

    let result = divide(10.0, 2.0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    }

    // Special Types

    // Slice
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    println!("Slice: {:?}", slice);

    // Strings
    let s: String = String::from("Hello, world!");
    let slice: &str = &s[0..5];
    println!("String slice: {}", slice);
}
