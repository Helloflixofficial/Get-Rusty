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
