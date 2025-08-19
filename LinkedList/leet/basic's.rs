use std::{fmt::Debug, rc::Rc};
#[allow(dead_code)]
struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
    prev: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value: value,
            next: None,
            prev: None,
        }
    }
}
struct Deque<T> {
    begin: Option<Rc<Node<T>>>,
    end: Option<Rc<Node<T>>>,
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
        if let Some(node) = &mut &self.begin {
            todo!()
        } else {
            assert!(self.end.is_none());
            let mut new_node = Rc::new(Node::new(value));
            self.begin = Some(new_node.clone());
            self.end = Some(new_node.clone());
        }
    }

    fn push_back(&mut self, value: T) {
        todo!()
    }

    fn bug_debug(&self) {
        let mut iter = &self.begin;
        while let Some(node) = iter {
            print!("{:?} ", node.value);
            iter = &node.next;
        }
        println!("")
    }
}

fn main() {
    let mut data = Deque::<String>::new();
    data.push_front("10".to_string());
    data.push_back("20".to_string());
    data.push_front("30".to_string());
}
