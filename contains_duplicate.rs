
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
     use std::collections::HashSet;
     let mut data = HashSet::new(); 
     for n in nums {
        if !data.insert(n) {
            return true;
        }
     }      
     false
}}
