use std::collections::HashSet;
struct Solution;
impl Solution{
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
          let mut nums = nums;
          nums.sort();
          let mut result = Vec::new()

   for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
   }

 let mut left = i + 1;
        let mut right = nums.len() - 1;
        
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
        
//with Rust.. busy 
//Incomplete Brain is Not braining right now



    }
}

fn main(){
    let  nums = vec![-1, 0, 1, 2, -1, -4];
    let data = Solution::three_sum(nums);
    println!("{:?}",data);
}
