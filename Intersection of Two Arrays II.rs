use std::collections::HashMap;
pub struct Solution;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut counts = HashMap::new();
        let mut result = Vec::new();

   
        for num in nums1 {
            *counts.entry(num).or_insert(0) += 1;
        }

    
        for num in nums2 {
            if let Some(count) = counts.get_mut(&num) {
                if *count > 0 {
                    result.push(num);
                    *count -= 1;
                }
            }
        }
        result
    }
}

fn main() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    let result = Solution::intersect(nums1, nums2);
    println!(" The data is {:?}", result); 
}





#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let result = Solution::intersect(nums1, nums2);
        assert_eq!(result, vec![2, 2]);  
    }

    #[test]
    fn example2() {
        let nums1 = vec![4,9,5];
        let nums2 = vec![9,4,9,8,4];
        let result = Solution::intersect(nums1, nums2);
        assert_eq!(result, vec![9,4]); 
    }
}
