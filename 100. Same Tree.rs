use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    #[inline]
    pub fn rc(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }
}

struct Solution;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
    }
}


fn main() {
    let t1 = Some(TreeNode::rc(1));
    t1.as_ref().unwrap().borrow_mut().left = Some(TreeNode::rc(2));
    t1.as_ref().unwrap().borrow_mut().right = Some(TreeNode::rc(3));

    let t2 = Some(TreeNode::rc(1));
    t2.as_ref().unwrap().borrow_mut().left = Some(TreeNode::rc(2));
    t2.as_ref().unwrap().borrow_mut().right = Some(TreeNode::rc(3));

    println!(
        "Are trees same? {}",
        Solution::is_same_tree(t1.clone(), t2.clone())
    );

    let t3 = Some(TreeNode::rc(1));
    t3.as_ref().unwrap().borrow_mut().left = Some(TreeNode::rc(2));
    let t4 = Some(TreeNode::rc(1));
    t4.as_ref().unwrap().borrow_mut().right = Some(TreeNode::rc(2));
    println!("Are trees same? {}", Solution::is_same_tree(t3, t4));
}
