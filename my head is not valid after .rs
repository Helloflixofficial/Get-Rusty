impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 || word.len() > 20 { return false; }

        let vowels = "aeiouAEIOU".chars().collect::<Vec<_>>();
        let (mut has_vowel, mut has_consonant) = (false, false);

        for ch in word.chars() {
            if !ch.is_alphanumeric() { return false; }
            if vowels.contains(&ch) { has_vowel = true; }
            else if ch.is_alphabetic() { has_consonant = true; }

            if has_vowel && has_consonant { return true; }
        }

        has_vowel && has_consonant
    }
}
