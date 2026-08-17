use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let mut stack = vec![(root, 1)];
        let mut answer = 0;
        while let Some((node, depth)) = stack.pop() {
            answer = answer.max(depth);
            let borrowed = node.borrow();
            if let Some(left) = borrowed.left.clone() {
                stack.push((left, depth + 1));
            }
            if let Some(right) = borrowed.right.clone() {
                stack.push((right, depth + 1));
            }
        }
        answer
    }
}