fn main() {
    let s1 = String::from("this is String data");
    let s2 = s1.clone();
    string_data(s2)
}

fn string_data(s: String) {
    println!("{s}")
}


fn main() {
    let s1 = String::from("Let's Get Rusty!");
    print_string(&s1);
    print_string(&s1);
}

fn print_string(s: &str) {
    println!("{s}");
}
