use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_front(&mut self, value: i32) {
        let newnode = Node::new(value);
        newnode.borrow_mut().next = self.head.take();
        self.head = Some(newnode);
    }

    fn print(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} -> ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!("NON");
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(10);
    list.print();
}
