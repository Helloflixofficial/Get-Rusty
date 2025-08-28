use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type Node = Option<Rc<RefCell<TreeNode>>>;

// Helpers to build trees easily
fn node(val: i32, left: Node, right: Node) -> Node {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}
fn leaf(val: i32) -> Node {
    node(val, None, None)
}





fn main() {
    let root = node(1, None, node(2, leaf(3), None));
    let mut out = Vec::new();
    inorder_recursive(&root, &mut out);
    println!("Recursive inorder: {:?}", out);
    let out2 = inorder_iterative(&root);
    println!("Iterative inorder: {:?}", out2);
}
