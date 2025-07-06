struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid + 1;
            }
        }
        left
    }
}

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    println!("{}", Solution::search_insert(nums, target));
}
