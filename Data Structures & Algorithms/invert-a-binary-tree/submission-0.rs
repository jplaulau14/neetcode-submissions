use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            let (left, right) = {
                let mut borrowed = node.borrow_mut();
                (borrowed.left.take(), borrowed.right.take())
            };
            let inverted_left = Self::invert_tree(right);
            let inverted_right = Self::invert_tree(left);
            let mut borrowed = node.borrow_mut();
            borrowed.left = inverted_left;
            borrowed.right = inverted_right;
        }
        root
    }
}