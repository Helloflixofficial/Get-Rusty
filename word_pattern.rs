use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split_whitespace().collect();
        if pattern.len() != words.len() {
            return false;
        }

        let mut char_to_word = HashMap::with_capacity(pattern.len());
        let mut word_to_char = HashMap::with_capacity(pattern.len());
        for (c, &word) in pattern.chars().zip(words.iter()) {
            
            if let Some(&mapped_word) = char_to_word.get(&c) {
                if mapped_word != word {
                    return false;
                }
            } else {
                char_to_word.insert(c, word);
            }

           
            if let Some(&mapped_char) = word_to_char.get(word) {
                if mapped_char != c {
                    return false;
                }
            } else {
                word_to_char.insert(word, c);
            }
        }
        true
    }
}

fn main() {
    let pattern = "abba".to_string(); 
    let s = "dog cat cat dog".to_string(); 
    println!("{}", Solution::word_pattern(pattern.clone(), s)); 

    let s2 = "dog cat cat fish".to_string(); 
    println!("{}", Solution::word_pattern(pattern, s2)); 
}
