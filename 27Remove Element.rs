struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let new_length = Solution::remove_element(&mut nums, val);
    println!("New length: {}", new_length);
    println!("Modified array: {:?}", &nums[..new_length as usize]);
}
