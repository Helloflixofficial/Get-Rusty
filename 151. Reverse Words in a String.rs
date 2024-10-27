pub struct Solution;
impl Solution {
    pub fn reverse_words(data: String) -> String {
        data.split_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

fn main() {
    let input = String::from(" world hello im good  ");
    let result = Solution::reverse_words(input);
    println!("{}", result);
}
