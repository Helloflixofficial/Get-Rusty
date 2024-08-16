#[derive(Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> ListNode {
        ListNode { val, next: None }
    }
}

fn to_list(vector: Vec<i32>) -> Option<Box<ListNode>> {
    let mut cur = None;
    for &Value in vector.iter().rev() {
        let mut new_node = ListNode::new(Value);
        new_node.next = cur;
        cur = Some(Box::new(new_node));
    }
    cur
}

fn main() {
    let vector = vec![0, 1, 2, 3, 4];
    println!("{:?}", to_list(vector));
}






impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_node = None;
        let mut original_node = head;

        while let Some(mut node) = original_node {
            original_node = node.next; // advancing the current node
            node.next = new_node;
            new_node = Some(node);
        }
        return new_node;
    }
}




