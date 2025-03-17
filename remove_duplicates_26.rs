
struct Solution;


    impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut i = 0;
        
        for j in 1..nums.len() {
            if nums[j] != nums[i] {
                i += 1;
                nums[i] = nums[j]; 
            }
        }

        (i + 1) as i32 
    }
}
fn main() {
    let mut nums = vec![1, 1, 2, 2, 3, 4, 4, 5];
    let new_length = Solution::remove_duplicates(&mut nums);
    
    println!("New length: {}", new_length);
    println!("Updated array: {:?}", &nums[..new_length as usize]);
}
