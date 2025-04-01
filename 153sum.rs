use std::collections::HashSet;

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort(); // Sorting helps in avoiding duplicates easily
    let mut result = Vec::new();
    
    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue; // Skip duplicates
        }
        
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            
            if sum == 0 {
                result.push(vec![nums[i], nums[left], nums[right]]);
                
                // Skip duplicates
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }
                
                left += 1;
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    
    result
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let triplets = three_sum(nums);
    println!("{:?}", triplets);
}
