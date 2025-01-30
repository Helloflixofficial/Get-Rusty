fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

fn main() {
    let num = 8;
    println!("Is {} a power of 2? {}", num, is_power_of_two(num));
}
