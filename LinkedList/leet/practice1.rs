fn main() {
    let mut ll = LinkedList::new();
    ll.push_frunt(10);
    ll.push_back(20);
    ll.push_frunt(30);
}

struct Node {
    element: u32,
    next: List,
}

enum List {
    Empty,
    Link(Box<Node>),
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test1() {
        let list = List::Link(Box::new(Node {
            element: 1024,
            next: List::Empty,
        }));
    }
}
