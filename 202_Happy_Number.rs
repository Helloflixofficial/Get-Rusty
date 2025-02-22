use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn is_happy(mut number: i32) -> bool {
        let mut seen = HashSet::new();
        while number != 1 && !seen.contains(&number) {
            seen.insert(number);
            number = Solution::sum_squares(number);
        }
        return number == 1;
    }

    fn sum_squares(mut num: i32) -> i32 {
        let mut sum = 0;

        while num > 0 {
            let digit = num % 10;
            sum += digit * digit;
            num /= 10;
        }
        return sum;
    }
}

fn main() {
    println!("{}", Solution::is_happy(19));
    println!("{}", Solution::is_happy(2));
}
