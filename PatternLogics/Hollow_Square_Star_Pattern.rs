fn main() {
    let size = 10; 
    for row in 0..size { 
        for col in 0..size {  
let is_border = row == 0 || row == size - 1 || col == 0 || col == size - 1;
            if is_border {
                print!("* ");
            } else {
                print!("+ ");
            }
        }
        println!();
}
}
