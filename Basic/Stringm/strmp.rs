let s = "Hello, Rust!";
let result = s
    .chars()
    .filter(|c| c.is_alphabetic())
    .map(|c| c.to_ascii_uppercase())
    .collect::<String>();

println!("{}", result); // "HELLORUST"



let name = "BOBY";
let mut msg = String::new();
msg.push_str("Hello, ");
msg.push_str(name);
msg.push_str("! ");
msg.push_str(&format!("Your name has {} letters.", name.len()));
println!("{}", msg);


let input = "Rust is awesome";
let rotated: String = input
    .split_whitespace()
    .rev()
    .collect::<Vec<_>>()
    .join(" ");

println!("{}", rotated); // "awesome is Rust"
