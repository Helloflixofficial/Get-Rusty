fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = String::from("Hello");
    let string2 = String::from("World!");

    let result = longest(&string1, &string2);
    println!("The longest string is: {}", result);
}
