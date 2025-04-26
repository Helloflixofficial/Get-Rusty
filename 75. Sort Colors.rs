fn sort_colors(nums: &mut Vec<i32>) {
    let mut low = 0;
    let mut mid = 0;
    let mut high = nums.len() - 1;
    println!("The data is : {}" , high);
     while mid <= high {
          match nums[mid] {

             0 => {
                nums.swap(low, mid);
                low += 1;
                mid += 1;
            },
             1 => {
                mid += 1;
            },






          }
     }
}


fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums);
    println!("Sorted colors: {:?}", nums);
}
