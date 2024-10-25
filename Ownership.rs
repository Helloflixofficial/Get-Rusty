fn main() {
    let s1 = String::from("this is String data");
    let s2 = s1.clone();
    string_data(s2)
}

fn string_data(s: String) {
    println!("{s}")
}
