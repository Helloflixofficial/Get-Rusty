fn print_data(s: &String) {
    println!("there is the data we looking for {}", s.len());
}

fn main() {
    let data = String::from("Megaman");
    print_data(&data);
    println!("the real the with the real lenth is {}", data.len());

    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}", s, len);

    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("Changed value: {}", s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
