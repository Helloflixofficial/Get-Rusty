
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    invert(&root)
}
}

fn invert(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = node {
        let right = invert(&node.borrow().left);
        let left = invert(&node.borrow().right);

        Some(Rc::new(RefCell::new(TreeNode {
            val: node.borrow().val,
            left,
            right,
        })))
    } else {
        None
    }
}
