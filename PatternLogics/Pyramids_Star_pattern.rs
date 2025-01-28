fn main() {
    fn print_pyramids() {
        let mut data = 5; 
        while data <= 120 {
            for i in 1..=data {
                for _ in 0..(data - i) {
                    print!(" ");
                }

             
                for _ in 0..(2 * i - 1) {
                    print!("*");
                }
                println!(); 
            }

            println!();
            data *= 2; 
        }
    }

    print_pyramids();
}
