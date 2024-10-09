pub struct Solution;
 impl Solution{
   pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_area = 0;



    while left < right {
        let h = std::cmp::min(height[left], height[right]);
        let width = (right - left) as i32;
         println!("data is : {}",width);
        let current_area = h * width;

        if current_area > max_area {
            max_area = current_area;
        }

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}
}

fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let result = Solution::max_area(height);
    println!("The maximum area is: {}", result);
}



// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn example1() {
//           let height = vec![1,8,6,2,5,4,8,3,7];

//         assert_eq!(Solution::max_area(height),49);
//     }
// }
