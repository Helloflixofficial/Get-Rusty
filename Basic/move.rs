fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];
    let result = sum(vec1);
    println!("Sum of elements: {}", result);
}

fn sum(v: Vec<i32>) -> i32 {
    v.iter().sum() 
}


fn is_palindrome(s: &str) -> bool {
    let s: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    s.eq_ignore_ascii_case(&s.chars().rev().collect::<String>())
}

fn main() {
    let input = "A man, a plan, a canal: Panama";
    let result = is_palindrome(input);

    println!("Is palindrome? {}", result); // Output: true
}
