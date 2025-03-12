struct Solution;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let a = nums.len();
        let b = (k as usize) % a;

        nums.reverse();
        nums[..b].reverse();
        nums[b..].reverse();
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 4, 5, 6, 7];
    let k1 = 3;
    println!("{:?}", Solution::rotate(&mut nums1, k1));

    let mut nums2 = vec![-1, -100, 3, 99];
    let k2 = 2;
    println!("{:?}", Solution::rotate(&mut nums2, k2));
}
