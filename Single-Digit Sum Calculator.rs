use std::io;
fn main() {
    let mut input = String::new();
    println!("PLEASE ENTER YOUR POSITIVE INGITER");

    io::stdin()
        .read_line(&mut input)
        .expect("Faild to read the line");

    let n: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("PLEASE ENTER THE VAILD INTEGER");
            return;
        }
    };

    if n <= 0 {
        println!("PLEASE ENTER A POSITIVE INTEGER");
        return;
    }
    let result = single_digit(n);
    println!("THE SINGLE DIGIT INTEGER SUM : {} ", result);
    return;
}

fn single_digit(mut n: i32) -> i32 {
    while n >= 10 {
        n = sumofdigit(n);
    }
    n
}

fn sumofdigit(mut n: i32) -> i32 {
    let mut sum = 0;
    while n != 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}
