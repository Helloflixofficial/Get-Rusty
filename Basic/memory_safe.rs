fn main() {
    // Ownership: `s1` owns the String
    let s1 = String::from("Hello, Rust!");
    print_ownership(s1); // s1 is MOVED and cannot be used anymore

    // Borrowing (Immutable Reference)
    let s2 = String::from("Borrowing Example");
    print_reference(&s2); // s2 is borrowed, can still be used
    println!("After borrowing: {}", s2);

    // Borrowing (Mutable Reference)
    let mut s3 = String::from("Mutable Borrow");
    modify_string(&mut s3); // s3 is modified through a mutable reference
    println!("After modification: {}", s3);

    // Moving Ownership (Transfer)
    let s4 = String::from("Move Example");
    let s5 = move_ownership(s4); // s4 is moved, ownership transferred to s5
    println!("After move, new owner: {}", s5);

    // Copy Trait (Ownership not moved)
    let x = 10;
    copy_example(x); // Integers implement Copy, so x is still valid
    println!("After function call, x: {}", x);
}

// Function that takes ownership (s1 is MOVED)
fn print_ownership(s: String) {
    println!("Ownership taken: {}", s);
} // s is dropped here

// Function that borrows (read-only reference)
fn print_reference(s: &String) {
    println!("Borrowed reference: {}", s);
}

// Function that borrows mutably (modifies the value)
fn modify_string(s: &mut String) {
    s.push_str(" - Modified!");
}

// Function that moves ownership (returns a value)
fn move_ownership(s: String) -> String {
    println!("Ownership moved: {}", s);
    s // Return ownership to caller
}

// Function that uses Copy trait (i32 is copied, not moved)
fn copy_example(x: i32) {
    println!("Copied value: {}", x);
}
