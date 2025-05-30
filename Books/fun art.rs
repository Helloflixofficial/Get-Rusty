fn main() {
    let n = 10; // Height  for the bow
                // Top half
    for i in 0..n {
        for _ in 0..i {
            print!(" ");
        }
        for _ in 0..(2 * (n - i) - 1) {
            print!("*");
        }
        println!();
    }

    // Bottom half
    for i in (0..n).rev() {
        for _ in 0..i {
            print!(" ");
        }
        for _ in 0..(2 * (n - i) - 1) {
            print!("*");
        }
        println!();
    }
}

fn main() {
    let size = 10; // Size of the star

    // Top half
    for i in 0..size {
        for _j in 0..(size - i - 1) {
            print!(" ");
        }
        for _k in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }

    // Bottom half
    for i in (0..size - 1).rev() {
        for _j in 0..(size - i - 1) {
            print!(" ");
        }
        for _k in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}

fn main() {
    let size = 10;

    for i in 0..size {
        for _j in 0..(size - i - 1) {
            print!(" ");
        }
        for _k in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }

    for i in (0..size - 1).rev() {
        for _j in 0..(size - i - 1) {
            print!(" ");
        }
        for _k in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}

fn main() {
    let size = 5; // Size of the diamond (number of layers)

    // Top half of the diamond
    for i in 0..size {
        for _ in 0..(size - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }

    // Bottom half of the diamond
    for i in (0..size - 1).rev() {
        for _ in 0..(size - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}

fn main() {
    let size = 50; // Size of the diamond (number of layers)

    // Top half of the diamond
    for i in 0..size {
        for _ in 0..(size - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }

    // Bottom half of the diamond
    for i in (0..size - 1).rev() {
        for _ in 0..(size - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}


//ice Cream
fn main() {
    let size = 10;
    
    for i in 0..size {
        for _ in 0..(size - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("0");
        }
        println!();
    }

    for i in (0..size - 1).rev() {
        for _ in 0..(size - i - 1) {
            print!(" ");
        }
        for _ in 0..(2 * i + 1) {
            print!("1");
        }
        println!();
    }
}
