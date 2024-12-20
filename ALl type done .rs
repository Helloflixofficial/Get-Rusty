fn main() {
    // Scalar Types

    // Integer Types
    let x: i32 = -10;
    let y: u32 = 42;

    // Floating-Point Types
    let a: f32 = 3.14;
    let b: f64 = 2.71828;

    // Boolean Type
    let is_active: bool = true;

    // Character Type
    let letter: char = 'A';
    let emoji: char = '😊';

    // Compound Types

    // Tuple
    let tup: (i32, f64, char) = (500, 6.4, 'Z');
    let (x, y, z) = tup; // Destructuring

    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];

    // User-Defined Types

    // Struct
    struct Point {
        x: f64,
        y: f64,
    }

    let p = Point { x: 1.0, y: 2.0 };

    // Enum
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;

    // Option Type
    let some_number: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;

    // Result Type
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(numerator / denominator)
        }
    }

    let result = divide(10.0, 2.0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    }

    // Special Types

    // Slice
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    println!("Slice: {:?}", slice);

    // Strings
    let s: String = String::from("Hello, world!");
    let slice: &str = &s[0..5];
    println!("String slice: {}", slice);
}


struct LinkedList {
    head: Option<Box<ListNode>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, val: i32) {
        let new_node = Box::new(ListNode { val, next: self.head.take() });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

}


// Linked List Types done

use std::collections::LinkedList;

fn main() {
    // Create a new empty linked list
    let mut list: LinkedList<i32> = LinkedList::new();

    // Push elements to the front and back of the list
    list.push_back(10);
    list.push_back(20);
    list.push_front(5);

    // Print the linked list
    println!("List after pushing elements: {:?}", list);

    // Pop elements from the front and back
    let front = list.pop_front();
    let back = list.pop_back();

    println!("Popped front: {:?}", front);
    println!("Popped back: {:?}", back);
    println!("List after popping: {:?}", list);

    // Access the front and back elements without removing them
    if let Some(first) = list.front() {
        println!("Front element: {}", first);
    }

    if let Some(last) = list.back() {
        println!("Back element: {}", last);
    }

    // Iterate over the list
    println!("Iterating over the list:");
    for elem in list.iter() {
        println!("{}", elem);
    }

    // Clear the list
    list.clear();
    println!("List after clearing: {:?}", list);
}

