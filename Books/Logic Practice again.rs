fn main() {
    let rows = 5; 
    
    for i in 0..rows {
    
        for _ in 0..(rows - i - 1) {
            print!(" ");
        }
        
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}
