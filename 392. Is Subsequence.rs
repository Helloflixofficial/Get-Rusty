struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
       let (mut i, mut j) = (0,0);
       let data_s: Vec<char> = s.chars().collect();
       let data_t: Vec<char> = t.chars().collect();

       
        while i < data_s.len() && j < data_t.len() {
            if data_s[i] == data_t[j] {
                   i += 1;
            }
            j+=1;
        }
        i == data_s.len()

    }
}

fn main() {
    let s = String::from("abc");
    let t = String::from("ahbgdc");
    let result = Solution::is_subsequence(s, t);
    println!("Is 's' a subsequence of 't'? {}", result);
}
