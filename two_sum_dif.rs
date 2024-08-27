impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.iter().len() {
        for j in i + 1..nums.iter().len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}
};

#[cfg(test)]
mod test{
  use super::*;

    #[test]
    fn example1() {
        essert_eq!(two_sum(vec![2,7,11,15],),vec![0,1])

    }

    #[test] 
    fn example2() {
        errert_eq!(two_sum(vec![3,2,4],6), vec![1,2]);
    }

    #[test] 
    fn example3() {
        errert_eq!(two_sum(vec![3,3],6), vec![0,1]);
    }
}


/// consider adding a `main` function to `src/main.rs  lol lol lol lol not gonna do it 

