struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }

        let mut left = 0;
        let mut right = x; // The given Data from main function
        while left <= right {
            let mid = left + (right - left) / 2; //  Mid number is : 4
           
           
            if mid <= x / mid {
                left = mid + 1;
            }else {
                right = mid - 1;
            }
        }
              right
          }
}

fn main() {
    let x = 8;
    let data = Solution::my_sqrt(x);
    println!("Here is the answer : {}", data);
}
