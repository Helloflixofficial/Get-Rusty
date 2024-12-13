///Codes needed to be saved in Notes
fn main() {
    let number = 3;

    match number {
        1 => println!("One!"),
        2 | 3 => println!("Two or Three!"), // Multiple patterns
        4..=6 => println!("Between Four and Six!"), // Range
        _ => println!("Something else!"), // Catch-all pattern
    }
}



fn main() {
    let point = (0, -2);

    match point {
        (0, y) => println!("On the y-axis at y = {}", y),
        (x, 0) => println!("On the x-axis at x = {}", x),
        (_, _) => println!("Off the axes!"), // Wildcard for all other cases
    }
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
    }
}


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 0, y: 7 };

    match point {
        Point { x: 0, y } => println!("Point on the y-axis at y = {}", y),
        Point { x, y: 0 } => println!("Point on the x-axis at x = {}", x),
        Point { x, y } => println!("Point at ({}, {})", x, y),
    }
}


fn main() {
    let favorite_color: Option<&str> = Some("blue");

    if let Some(color) = favorite_color {
        println!("Your favorite color is {}!", color);
    } else {
        println!("You don't have a favorite color.");
    }
}


fn main() {
    let numbers = (2, 4, 8, 16);

    match numbers {
        (first, _, third, _) => println!("First: {}, Third: {}", first, third),
    }
}


fn main() {
    let num = 7;

    match num {
        n @ 1..=5 => println!("Number {} is between 1 and 5", n),
        n @ 6..=10 => println!("Number {} is between 6 and 10", n),
        _ => println!("Number is something else!"),
    }
}


enum Nested {
    One,
    Two(i32),
    Three { x: i32, y: i32 },
}

fn main() {
    let value = Nested::Three { x: 5, y: -10 };

    match value {
        Nested::One => println!("It's One!"),
        Nested::Two(num) => println!("It's Two with value {}", num),
        Nested::Three { x, y: 0 } => println!("On the x-axis at x = {}", x),
        Nested::Three { x, y } => println!("Somewhere at ({}, {})", x, y),
    }
}


fn main() {
    let numbers = [1, 2, 3];

    match numbers {
        [1, 2, 3] => println!("Matched [1, 2, 3]!"),
        [1, _, _] => println!("Starts with 1"),
        _ => println!("Something else"),
    }
}


fn main() {
    let number = Some(10);

    match &number {
        Some(&val) if val > 5 => println!("Got a large number: {}", val),
        Some(_) => println!("Got a number"),
        None => println!("Got nothing"),
    }
}


fn main() {
    let value = Some(5);

    match value {
        Some(1 | 2 | 3) => println!("Matched 1, 2, or 3"),
        Some(4..=10) => println!("Matched in range 4 to 10"),
        None => println!("No value!"),
        _ => println!("Something else!"),
    }
}


//Still in process to perfact thisss
