struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) {
        let mut i = m as isize - 1;
        let mut j = n as isize - 1;
        let mut k = (m + n) as isize - 1;
        

        while j >= 0 {
            
            if i >= 0 && nums1[i as usize] > nums2[j as usize] {

                
                nums1[k as usize] = nums1[i as usize];
                i -= 1;





            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let nums2 = vec![2, 5, 6];
    let m = 3;
    let n = 3;
    Solution::merge(&mut nums1, m, &nums2, n);
    println!("{:?}", nums1);
}
