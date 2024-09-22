
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        for j in 1..nums.len() {
            if nums[j] != nums[i] {
                i += 1;
                nums[i] = nums[j]; 
            }
        }
        
        (i + 1) as i32 
    }




fn main() {
    let mut nums = vec![1, 1, 2,3,4,4,4,5,6];
    let length = remove_duplicates(&mut nums);
    println!("Modified array: {:?}", &nums[..length as usize]);
}
