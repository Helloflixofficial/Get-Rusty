struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut max_len = 0;
        let mut zeros = 0;

}
}

fn main() {
    let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
    let k = 2;
    let result = Solution::longest_ones(nums, k);
    println!("Max consecutive 1s (with {} flips): {}", k, result); 
}
//First dat took me understand the que &next day thanks
//i have to see the code ; i cant solve this with my given method so im taking help Ai 
