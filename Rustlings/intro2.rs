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



let expr = "12+3-4";
let mut num = String::new();

for c in expr.chars() {
    match c {
        '0'..='9' => num.push(c),
        '+' | '-' => {
            println!("Number: {}", num);
            println!("Operator: {}", c);
            num.clear();
        }
        _ => (),
    }
}
if !num.is_empty() {
    println!("Number: {}", num);
}
