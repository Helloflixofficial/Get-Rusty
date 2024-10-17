pub struct Solution;
 impl Solution{
   pub fn missing_number(nums : Vec<i32>) -> i32 {
     let n = nums.len() as i32;
     let expected_sum = n * (n + 1) / 2;
        let  actual_sum: i32 = nums.iter().sum();
        let data =  expected_sum - actual_sum;
               data
}
}

fn main(){
    let data = vec![3,0,1];
    let nums = Solution::missing_number(data);
    println!("the data should be here : {}", nums);
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example1() {
        let nums =  vec![3,0,1];
        assert_eq!(Solution::missing_number(nums),2);
    }
}
