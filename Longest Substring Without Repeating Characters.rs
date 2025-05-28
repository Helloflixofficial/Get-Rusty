use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut left, mut right) = (0, 0);
        let mut set = HashSet::new();
        let mut max_len = 0;
        let chars: Vec<char> = s.chars().collect();

        while right < chars.len() {
           
        }

        max_len as i32
    }
}
