use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(root) = root else {
            return true;
        };

        let mut stack = vec![(root, i64::MIN, i64::MAX)];
        while let Some((node, lower, upper)) = stack.pop() {
            let (value, left, right) = {
                let borrowed = node.borrow();
                (borrowed.val as i64, borrowed.left.clone(), borrowed.right.clone())
            };
            if value <= lower || value >= upper {
                return false;
            }
            if let Some(right) = right {
                stack.push((right, value, upper));
            }
            if let Some(left) = left {
                stack.push((left, lower, value));
            }
        }
        true
    }
}