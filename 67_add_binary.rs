struct Solution;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::new();
        let mut carry = 0;

        let bytes_a = a.as_bytes();
        let bytes_b = b.as_bytes();

        let mut i = a.len() as i32 - 1;
        let mut j = b.len() as i32 - 1;

        while i >= 0 || j >= 0 || carry > 0 {
            let mut sum = carry;

            if i >= 0 {
                sum += (bytes_a[i as usize] - b'0') as u8;
                i -= 1;
            }

            if j >= 0 {
                sum += (bytes_b[j as usize] - b'0') as u8;
                j -= 1;
            }

            result.push(((sum % 2) + b'0') as char);
            carry = (sum / 2) as u8;
        }
        result.chars().rev().collect()
    }
}

fn main() {
    let a = "1010".to_string();
    let b = "1011".to_string();
    let result = Solution::add_binary(a, b);
    println!("Binary sum = {}", result);
}
