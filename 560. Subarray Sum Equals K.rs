use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut Count = 0;
        let mut sum = 0;
        let mut map = HashMap::new();
        map.insert(0, 1); 
    }
}

fn main() {
    let nums1 = vec![1, 1, 1];
    let k1 = 2;
    let result1 = Solution::subarray_sum(nums1, k1);
    println!("Total subarrays with sum {} ", result1);

    let nums2 = vec![1, 2, 3];
    let k2 = 3;
    let result2 = Solution::subarray_sum(nums2, k2);
    println!("Tollal subarrays with sum {}", result2);
}
