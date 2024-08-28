
impl Solution {
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut vec = Vec::new();
    let mut current = head;

    while let Some(mut node) = current {
        current = node.next.take();
        if node.val != val {
            vec.push(node);
        }
    }

  
    let mut new_head = None;
    while let Some(mut node) = vec.pop() {
        node.next = new_head;
        new_head = Some(node);
    }

    new_head
}
}
