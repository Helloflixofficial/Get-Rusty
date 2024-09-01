
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for ch in s.chars() {
            match ch {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                ')' | '}' | ']' => {
                    if stack.pop() != Some(ch) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        stack.is_empty()
    }

fn main() {
    let s = String::from("()[]{}");
    let data = is_valid(s);
    println!("is valid: {}", data);
}
//easy 11min
