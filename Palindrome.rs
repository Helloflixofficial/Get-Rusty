struct Solution;
impl Solution {
fn is_palindrome(num: i32) -> bool {
    let num_str = num.to_string();
    num_str == num_str.chars().rev().collect::<String>()
  
}
}
fn main() {
    let number = 121;
    if Solution::is_palindrome(number) {
        println!("{} YES", number);
    } else {
        println!("{} NO", number);
    }
}
