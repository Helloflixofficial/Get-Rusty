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
