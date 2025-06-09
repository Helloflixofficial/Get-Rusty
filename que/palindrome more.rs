struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let cleaned: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let mut left = 0;
        let mut right = cleaned.len().saturating_sub(1);

        while left < right {
            if cleaned[left] != cleaned[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}

fn main() {
    let test = String::from("A man, a plan, a canal: Panama");
    let result = Solution::is_palindrome(test);
    println!("Is palindrome? {}", result); // Output: true
    let test = String::from("Hello");
    let result = Solution::is_palindrome(test);
    println!("Is palindrome? {}", result); // Output: true
}

// 🔁 Sliding Window & Two Pointer

//     1004. Max Consecutive Ones III – Meta, LinkedIn, Google, Amazon

//     125. Valid Palindrome – Meta, Google

// 🔍 Binary Search

//     35. Search Insert Position – Google, Microsoft

// ➕ Prefix Sum

//     303. Range Sum Query - Immutable – Google, Meta, Amazon

// 📚 Stack

//     71. Simplify Path – Meta, Amazon, Google

// 📊 Queue

//     362. Design Hit Counter – Uber, Google, Apple

// ⏳ Heap / Priority Queue

//     253. Meeting Rooms II – Amazon, Google, Meta, Bloomberg, TikTok

// 🔗 Linked List

//     725. Split Linked List in Parts – Google

// 🌳 Binary Tree

//     94. Binary Tree Inorder Traversal – Meta, Google

// 🌐 Graph

//     207. Course Schedule – Amazon, Meta, Google, TikTok

//     323. Number of Connected Components in an Undirected Graph – Amazon, LinkedIn

// 🧮 Dynamic Programming

//     198. House Robber – Amazon, Google, Microsoft, TikTok
