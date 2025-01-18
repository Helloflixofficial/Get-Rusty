struct Solution;
impl Solution {
     pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
  let mut low = 0;
        let mut high = nums.len() as i32 - 1;

        while low <= high {
            let mid = low + (high - low) / 2;

            if nums[mid as usize] == target {
                return mid;

                

            } else if nums[mid as usize] < target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        low
      
        

    }
}

fn main() {
   let nums = vec![1, 3, 5, 6];
   let target = 5;
    println!("Insert Here : {}",Solution::search_insert(nums,target))

}
