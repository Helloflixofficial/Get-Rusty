

fn main() {
    let is_morning = true;

    if is_morning {
        println!("Good morning!");
    } else {
        println!("Good evening!");
    }
    
    let is_evening = !is_morning;
    if is_evening {
        println!("Good evening!");
    }
}
