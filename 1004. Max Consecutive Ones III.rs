impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut zero = 0;
        let mut max_len = 0;

        while right < nums.len(){
            if nums[right] == 0 {
                zero += 1;
            }

            while zero > k {
                if nums[left] == 0{
                    zero -=1;
                }
                left += 1;
            }

            max_len = max_len.max(right - left + 1);
            right += 1;
        }
        max_len as i32

    }
}
