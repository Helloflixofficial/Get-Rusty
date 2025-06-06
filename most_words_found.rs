struct Solution;

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .iter()
            .map(|s| s.split_whitespace().count() as i32)
            .max()
            .unwrap_or(0)
    }
}

fn main() {
    let sentences = vec![
        "Hello world".to_string(),
        "This is a Rust program".to_string(),
        "Write code. Change the world.".to_string(),
    ];

    let result = Solution::most_words_found(sentences);
    println!("Maximum number of words in a sentence: {}", result);
}
