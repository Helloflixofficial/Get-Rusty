use std::collections::HashSet;
//understand the question and practice sec time...
fn length_of_longest_substring(s: String) -> i32 {
    let mut chars: Vec<char> = s.chars().collect();
    let mut set: HashSet<char> = HashSet::new();
    let mut left = 0;
    let mut max_len = 0;
   
}



fn main() {
    let s = String::from("abcabcbb");
    let result = length_of_longest_substring(s);
    println!(
        "Length of longest substring without repeating characters: {}",
        result
    );
}
