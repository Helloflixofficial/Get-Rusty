use std::collections::HashMap;
pub struct Solution;
impl Solution{
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = map.get(&complement) {
            return vec![index as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
 
}



