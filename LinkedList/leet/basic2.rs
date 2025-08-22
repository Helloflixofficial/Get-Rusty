use std::collections::LinkedList;
fn main() {
    let mut my_list = LinkedList::new();
    my_list.push_back(10);
    my_list.push_back(20);
    my_list.push_back(30);
    // Look at the first item without removing it
    match my_list.front() {
        Some(value) => println!("First item is: {}", value),
        None => println!("List is empty!"),
    }
    
    // Look at the last item without removing it
    match my_list.back() {
        Some(value) => println!("Last item is: {}", value),
        None => println!("List is empty!"),
    }
}
