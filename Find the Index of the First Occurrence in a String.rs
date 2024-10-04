
    pub fn str_str(haystack: String, needle: String) -> i32 {
        
        if needle.is_empty() {
            return 0;
        }

        if let Some(index) = haystack.find(&needle) {
            index as i32
        } else {
            -1 
        }
    }


fn main() {
    let haystack = String::from("hello");
    let needle = String::from("ll");
    let result = str_str(haystack, needle);
    println!("Index: {}", result);
}
