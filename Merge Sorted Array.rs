// impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut p1 = (m - 1) as usize; 
        let mut p2 = (n - 1) as usize; 
        let mut p = (m + n - 1) as usize;
        println!("The size is {} {} {}", p1, p2, p);// indexing here sire

        while p1 != usize::MAX && p2 != usize::MAX {
            if nums1[p1] > nums2[p2] {
                nums1[p] = nums1[p1];
                p1 = p1.wrapping_sub(1); 
            } else {
                nums1[p] = nums2[p2];
                p2 = p2.wrapping_sub(1); 
            }
            p = p.wrapping_sub(1);
        }

        // if some elements left here send nums2 to num1 hnn
        while p2 != usize::MAX {
            nums1[p] = nums2[p2];
            p2 = p2.wrapping_sub(1);
            p = p.wrapping_sub(1);
        }
    }
// }


fn main() {
    let mut nums1 = vec![1,2,3,0,0,0]; // Capacity 6
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &mut nums2, 3);
    println!("{:?}", nums1); // Output: [1, 2, 2, 3, 5, 6];
   
    let nums: Vec<i32> = vec![10, 20, 30, 40, 50];
    let index: usize = 3;  
    println!("Element at index {} is {}", index, nums[index]);
    for i in 0..nums.len() {
        let idx: usize = i;
        println!("Element at index {} is {}", idx, nums[idx]);
    
}

}
