pub struct Solution;
 impl Solution{
pub fn is_palindrome(s: String) -> bool {
    let filtered: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    filtered == filtered.chars().rev().collect::<String>()
}
 }



fn main() {
    let input = String::from("A man, a plan, a canal: Panama");
    println!("Is palindrome: {}",Solution::is_palindrome(input)); // Output: true
}



// fn main() {
//     let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
//     let result = Solution::max_area(height);
//     println!("The maximum area is: {}", result);
// }



// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn example1() {
//           let height = vec![1,8,6,2,5,4,8,3,7];

//         assert_eq!(Solution::max_area(height),49);
//     }
// }
