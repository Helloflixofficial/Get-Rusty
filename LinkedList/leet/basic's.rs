use std::{cell::RefCell, fmt::Debug, rc::Rc};
#[allow(dead_code)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }
       pub fn is_empty(&self) -> bool {
        self.head.is_none()
}

  pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.next;
            old_head.value
        })
    }

fn main() {
    let mut list = LinkedList::new();
    assert!(list.is_empty());
    list.push_front(10);
    list.push_front(20);
    list.push_front(30);
    list.print(); 
    println!("peek = {:?}", list.peek()); 

    // POP THE DATA
    println!("pop = {:?}", list.pop_front());
    println!("pop = {:?}", list.pop_front()); 
    println!("pop = {:?}", list.pop_front()); 
    println!("pop = {:?}", list.pop_front()); 
    list.print();
}
