struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let cleaned: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let mut left = 0;
        let mut right = cleaned.len().saturating_sub(1);

        while left < right {
            if cleaned[left] != cleaned[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}

fn main() {
    let test = String::from("A man, a plan, a canal: Panama");
    let result = Solution::is_palindrome(test);
    println!("Is palindrome? {}", result);



    let test = String::from("Hello");
    let result = Solution::is_palindrome(test);
    println!("Is palindrome? {}", result);
}
