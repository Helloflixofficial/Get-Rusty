enum Dieaction {
    left,
    right,
    up,
    down,
}

fn main() {
    let Go = Dieaction::left;
    match Go {
        Dieaction::left => print!("Left data got printed"),
        Dieaction::right => print!("right data got printed"),
        Dieaction::up => print!("upper data got printed"),
        Dieaction::down => print!("down data got printed"),
    }
}
