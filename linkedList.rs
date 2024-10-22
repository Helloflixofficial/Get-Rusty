use std::option::Option;
use std::mem;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}



#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    // Create a new empty LinkedList
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Add a value to the front of the list
    fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Remove the first element from the list
    fn pop(&mut self) -> Option<i32> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.value)
        } else {
            None
        }
    }

    // Display the elements in the list
    fn print_list(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}







fn main() {
    let mut list = LinkedList::new();
    
    // Add values to the list
    list.push(1);
    list.push(2);
    list.push(3);

    // Print the current list
    println!("Initial list:");
    list.print_list();

    // Pop a value from the list
    let popped_value = list.pop();
    println!("Popped value: {:?}", popped_value);

    // Print the list after popping
    println!("List after pop:");
    list.print_list();
}

