struct Solution;
impl Solution {
fn longest_palindromic_substring(s: &str) -> &str {

    fn expand_around_center(s: &str, mut left: isize, mut right: isize) -> (usize, usize) {
        let chars: Vec<char> = s.chars().collect();
    //    println!("{:?} {}", chars,s);


        while left >= 0 && right < chars.len() as isize && chars[left as usize] == chars[right as usize] {
            left -= 1;
            right += 1;
        }
        // println!("{}", left);
        ((left + 1) as usize, right as usize) 
    }

    let mut start = 0;
    let mut end = 0;
    for i in 0..s.len() {
        let (left1, right1) = expand_around_center(s, i as isize, i as isize);    
        let (left2, right2) = expand_around_center(s, i as isize, i as isize + 1);   

        if right1 - left1 > end - start {
            start = left1;
            end = right1;
        }
        if right2 - left2 > end - start {
            start = left2;
            end = right2;
        }
    }
    &s[start..end]
}
}




fn main() {
    let s = "babad";
    let result = Solution::longest_palindromic_substring(s);
    println!(" {}", result);
}
