fn main() {
    let sum = add_numbers(5, 10); 
    println!("The sum is: {}", sum);

    let square = square_number(4);
    println!("The square is: {}", square);

    let message = greet("Alice");
    println!("{}", message);
}


fn add_numbers(a: i32, b: i32) -> i32 {
    a + b 

fn square_number(num: i32) -> i32 {
    return num * num; 
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name) 
