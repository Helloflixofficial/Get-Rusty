use std::io;
fn main() {
    let mut inputone = String::new();
    let mut inputtwo = String::new();
    println!("Please enter your first input : ");
    io::stdin()
        .read_line(&mut inputone)
        .expect("HAHA LOL so funny please add input");
    println!("please enter you sec input here : ");
    io::stdin()
        .read_line(&mut inputtwo)
        .expect("haha why man just give me some input : ");

    let numone: i32 = inputone
        .trim()
        .parse()
        .expect("Please enter a valid number.");

    let numtwo: i32 = inputtwo
        .trim()
        .parse()
        .expect("Please enter a valid number.");

    let data = numone + numtwo;
    println!(
        "your first input {:?} and second input {:?}  is = {:?} ",
        numone, numtwo, data
    );
}
