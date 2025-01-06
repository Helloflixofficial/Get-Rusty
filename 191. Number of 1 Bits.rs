pub fn hamming_weight(mut n: u32) -> i32 {
    let mut count = 0;
    while n != 0 {
        count += (n & 1) as i32;
        n >>= 1;
    }
    count
}

fn main() {
    let num = 0b00000000000000000000000000001011;
    println!("Number of 1 bits: {}", hamming_weight(num));

    let num2 = 0b11111111111111111111111111111001;
    println!("Number of 1 bits: {}", hamming_weight(num2));
}
