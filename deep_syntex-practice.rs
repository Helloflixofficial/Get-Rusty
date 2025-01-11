//////////////////////////////////////////////// Scalar variable types (primative variables)
fn main() {
    // unsigned integer
    // u8, u16, u32, u64, u128
    let unsigned: u8 = 10;
    // signed integer
    // i8, i16, i32, i64, i128
    let signed: i8 = -100;
    // float is used for decimals
    let float: f32 = 1.0;
    println!("unsign: {} sign: {} float: {}", unsigned, signed, float);
    // char - can only be a single character
    let letter = 'c';
    let emoji = '\u{1F600}'; // 😀 (grinning face emoji)
    println!("letter: {}, emoji: {}", letter, emoji);
    let is_true: bool = true;
    println!("isTrue: {}", is_true);
}

///////////////////////////////////////////////////////////////////// Array
fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5];
    let sata: [u8;4] = [1,2,3,4];
  println!("{:?}",sata);
    println!("index: {}, length: {}", arr[0], other_arr.len());
    println!("{:?}", other_arr);
}
// Tuple
fn main() {
    let tuple: (u8, bool, f32) = (5, true, 2.1);
    let tuple2 = (3, 5);
    // print structure of array and other objects
    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);
    let (a, b, c) = tuple;
    // destructuring
    println!("first {}, second {}, third {}", a, b, c);
}

//function
fn main(){
    println!("{}", is_even(2))
}

pub fn is_even(num:u8) -> bool{
    let digit: u8 = num % 2;
    digit == 0
}
// Mutability
fn main(){
    let mut data = 10;
    let data = 5;
    println!("{}",data);
}


// arr & Slice 

fn main() {
    let arr = [0, 1, 2, 3, 4, 5]; // The full array
    let slice = &arr[1..3]; // Slice containing elements [1, 2]
    borrowing_slice(arr, slice);
}

fn borrowing_slice(arr: [u8; 6], slice: &[u8]) {
    println!("{:?}", arr); // Print the full array
    println!("{:?}", slice); // Print the slice
    println!("length: {}", slice.len()); // Length of the slice
    println!("{} {}", slice[0], slice[1]); // Access elements of the slice
}


//vector
fn main(){
   let mut data: Vec<i64> = vec![1,2,3,4,5];
    data.len();
    data[0];
    data.push(6);
    data.remove(2);
    println!("{:?}",data);
}

// HashMap 
use std::collections::HashMap;

fn main(){
let mut map = HashMap::new();
map.insert(0, "Hii");
map.insert(1,", Sire");
println!("{:?}", map);

match map.get(&0) {
    Some(str) => println!("{:?}",str),
    _ => println!("Doesn't exist in mao")
}

match map.get(&2) {
    Some(str) => println!("{}",str),
    _ => println!("doesn't exist in map")
}

map.remove(&0);
println!("{:?}",map);
}

//
fn main() {
    let name = String::from("Bird");
    let bird = Bird { name, attack: 5 };
    bird.print_name();
}

struct Bird {
    name: String,
    attack: u64,
}

impl Bird {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}
// interface

