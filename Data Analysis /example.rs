use std::thread;
fn main() {
    let mut handles = vec![];
    let mut x = 0;
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            x += 1;
            println!("Hello from thread {} with x = {}", i, x);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
