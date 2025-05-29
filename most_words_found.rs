impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences.iter()
            .map(|s| s.split_whitespace().count() as i32)
            .max()
            .unwrap_or(0)
    }
}
