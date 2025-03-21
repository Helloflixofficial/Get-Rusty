use std::collections::HashMap;
use std::io;

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman_map: HashMap<char, i32> = HashMap::new();
        roman_map.insert('I', 1);
        roman_map.insert('V', 5);
        roman_map.insert('X', 10);
        roman_map.insert('L', 50);
        roman_map.insert('C', 100);
        roman_map.insert('D', 500);
        roman_map.insert('M', 1000);

        let mut total = 0;
        let mut prev_value = 0;

        for ch in s.chars().rev() {
            let value = *roman_map.get(&ch).unwrap_or(&0);
            if value < prev_value {
                total -= value;
            } else {
                total += value;
            }
            prev_value = value;
        }

        total
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter a Roman numeral: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let roman = input.trim().to_string(); // Convert to String
    let result = Solution::roman_to_int(roman); // ✅ Semicolon added

    println!("Integer value: {}", result);
}
