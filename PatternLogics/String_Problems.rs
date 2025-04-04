fn main() {
    let input = "Hello World, how are you?";
    let result = input.replace(" ", "%20");
    println!("{}", result);
}



fn permute(s: &mut Vec<char>, left: usize, right: usize) {
    if left == right {
        println!("{}", s.iter().collect::<String>());
    } else {
        for i in left..=right {
            s.swap(left, i);
            permute(s, left + 1, right);
            s.swap(left, i); // backtrack
        }
    }
}

fn main() {
    let input = "abc".to_string();
    let mut chars: Vec<char> = input.chars().collect();
    permute(&mut chars, 0, chars.len() - 1);
}




//////////////////
fn compress_string(s: &str) -> String {
    let mut compressed = String::new();
    let mut chars = s.chars().peekable();
    while let Some(current) = chars.next() {
        let mut count = 1;
        while chars.peek() == Some(&current) {
            count += 1;
            chars.next();
        }
        compressed.push(current);
        if count > 1 {
            compressed.push_str(&count.to_string());
        }
    }
    compressed
}

fn main() {
    let input = "aaabbcddd";
    let compressed = compress_string(&input);
    println!("Original: {}", input);
    println!("Compressed: {}", compressed);
}




///
use std::collections::HashMap;

fn count_char_frequency(s: &str) -> HashMap<char, usize> {
    let mut frequency_map = HashMap::new();

    for ch in s.chars() {
        *frequency_map.entry(ch).or_insert(0) += 1;
    }

    frequency_map
}

fn main() {
    let input = "hello world";
    let frequency = count_char_frequency(input);

    println!("Character Frequency:");
    for (ch, count) in &frequency {
        println!("'{}': {}", ch, count);
    }
}
