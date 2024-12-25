struct Solution;

impl Solution {
      pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }
}

fn main() {
    let number = 16;
    if Solution::is_power_of_two(number) {
        println!("{} is a power of two.", number);
    } else {
        println!("{} is not a power of two.", number);
    }
}
