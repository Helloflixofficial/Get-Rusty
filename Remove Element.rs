// impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
    let n = nums.len();
    for j in 0..n {
        if nums[j] != val {
            nums[i] = nums[j];
            i += 1;
        }
    }

    i as i32
        
    }
// }

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let length = remove_element(&mut nums, val);
    println!("New length: {}", length);
    println!("Modified array: {:?}", &nums[0..length as usize]);
}
