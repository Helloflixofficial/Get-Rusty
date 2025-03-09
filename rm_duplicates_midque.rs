struct Solution;
impl Solution {
  fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut zero = 0;

    for i in 0..nums.len() {
        if zero < 2 || nums[zero - 2] != nums[i] {
            nums[zero] = nums[i];
            zero += 1;
        }
    }

    zero as i32
}
}


fn main(){
let mut nums1 = vec![1, 1, 1, 2, 2, 3];
let mut nums2 = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
let k1 = Solution::remove_duplicates(&mut nums1);
let k2 = Solution::remove_duplicates(&mut nums2);
println!("The length of nums1 is : {}", k1);
println!("The length of nums2 is : {}", k2);

}
