use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let Some(node) = node else {
                return 0;
            };

            let (left, right) = {
                let node = node.borrow();
                (node.left.clone(), node.right.clone())
            };

            let left_height = height(&left);
            if left_height == -1 {
                return -1;
            }

            let right_height = height(&right);
            if right_height == -1 {
                return -1;
            }

            if (left_height - right_height).abs() > 1 {
                return -1;
            }

            1 + left_height.max(right_height)
        }

        height(&root) != -1
    }
}