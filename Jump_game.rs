struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach = 0;
        for (i, &jump) in nums.iter().enumerate() {
            if i > max_reach {
                // println!("{},{}",i,&jump);
                return false;
            }


            max_reach = max_reach.max(i + jump as usize);
            //   println!("{}",max_reach);
            if max_reach >= nums.len() - 1 {
                return true;
            }
        }

        false
    }
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    println!("Can jump: {}", Solution::can_jump(nums));

    // let nums2 = vec![3, 2, 1, 0, 4];
    // println!("Can jump: {}", Solution::can_jump(nums2));
}
