struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
if n <= 2 {
            return n;
        }
        let mut prev1 = 1;
        let mut prev2 = 2; 
        for _ in 3..=n {
            let current = prev1 + prev2; 
            prev1 = prev2; 
            prev2 = current; 
        }
        prev2 
         }
}
