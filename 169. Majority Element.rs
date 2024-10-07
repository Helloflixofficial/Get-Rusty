   
   impl Solution{
 pub fn majority_element(data: Vec<i32>) -> i32 {
       use std::collections::HashMap;
        let mut nums = HashMap::new();

        for n in data {
        *nums.entry(n).or_insert(0) +=1;
        }

        let mut vec = nums.into_iter().collect::<Vec<(i32,i32)>>();
        vec.sort_by(|a,b|   b.1.cmp(&a.1));

        vec[0].0

    }
    }





impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut majority = nums[0];
        let mut count: usize = 0;

        for num in nums {
            if count == 0 {
                majority = num;
                count += 1;
            } else if majority == num {
                count += 1;
            } else {
                count -= 1;
            }
        }

        majority
    }
}
