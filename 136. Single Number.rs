struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, num| acc ^ num)
    }
}

fn main() {
    let nums = vec![4, 1, 2, 1, 2];
    let result = Solution::single_number(nums);
    println!("The single number is: {}", result);
}
// this solu needed to be solve as dif way for practice....
