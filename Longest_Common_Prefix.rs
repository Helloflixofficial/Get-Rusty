pub struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        let mut prefix = strs[0].clone();
        for s in strs.iter().skip(1) {
            while !s.starts_with(&prefix) {
                prefix.pop();

            }
        }

        prefix
    }
}



// #######for Running purpose########
fn main() {
    let strs = vec!["flower", "flow", "flight"]
    .into_iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let result = Solution::longest_common_prefix(strs);
    println!("Longest common prefix: {}", result);
}

// ##########for Test purpose ######
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(Solution::longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]), "fl".to_string());
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::longest_common_prefix(vec!["dog".to_string(),"racecar".to_string(),"car".to_string()]),"".to_string());
    }
}
