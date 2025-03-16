use std::collections::HashMap;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut letter = HashMap::new();
        
        for char in magazine.chars() {
            *letter.entry(char).or_insert(0) += 1;
        }

        
        for char in ransom_note.chars() {
            match letter.get_mut(&char) {
                Some(count) if *count > 0 => *count -= 1,
                _ => return false,  
            }
        }

        true
    }
}


fn main(){
    let ransom_note = "aa".to_string();
    let magazine = "aab".to_string();
    println!("{}", Solution::can_construct(ransom_note, magazine));
}
