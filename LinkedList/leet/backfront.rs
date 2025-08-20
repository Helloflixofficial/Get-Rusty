use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[allow(dead_code)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            next: None,
            prev: None,
        }))
    }
}

struct Deque<T> {
    begin: Option<Rc<RefCell<Node<T>>>>,
    end: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Deque<T>
where
    T: Debug,
{
    fn new() -> Self {
        Self {
            begin: None,
            end: None,
        }
    }

    fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.begin.take() {
            Some(old_begin) => {
                new_node.borrow_mut().next = Some(old_begin.clone());
                old_begin.borrow_mut().prev = Some(new_node.clone());
                self.begin = Some(new_node);
            }
            None => {
                self.end = Some(new_node.clone());
                self.begin = Some(new_node);
            }
        }
    }

    fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.end.take() {
            Some(old_end) => {
                old_end.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_end);
                self.end = Some(new_node);
            }
            None => {
                self.begin = Some(new_node.clone());
                self.end = Some(new_node);
            }
        }
    }

    fn bug_debug(&self) {
        let mut iter = self.begin.clone();
        while let Some(node) = iter {
            print!("{:?} ", node.borrow().value);
            iter = node.borrow().next.clone();
        }
        println!();
    }
}

fn main() {
    let mut data = Deque::<String>::new();
    data.push_front("10".to_string());
    data.push_back("20".to_string());
    data.push_front("30".to_string());
    data.bug_debug();
}
