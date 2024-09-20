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


//Methods for practice and more

fn main() {
    let mut data = String::from("Rust");

 
    let len = calculate_length(&data); 
    println!("The length of '{}' is {}.", data, len);


    let first_borrow = &data;
    let second_borrow = &data;
    println!("First borrow: {}", first_borrow);
    println!("Second borrow: {}", second_borrow);


    modify_string(&mut data);
    println!("After modification: {}", data);

    add_suffix(&mut data);
    println!("Final string: {}", data);
}


fn calculate_length(s: &String) -> usize {
    s.len()
}


fn modify_string(s: &mut String) {
    s.push_str(" Programming");
}


fn add_suffix(s: &mut String) {
    s.push_str(" is fun!");
}
