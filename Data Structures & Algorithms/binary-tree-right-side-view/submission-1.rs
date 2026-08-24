use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let Some(root) = root else {
            return Vec::new();
        };

        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut result = Vec::new();

        while !queue.is_empty() {
            let level_size = queue.len();

            for index in 0..level_size {
                let node = queue.pop_front().unwrap();
                let (value, left, right) = {
                    let borrowed = node.borrow();
                    (borrowed.val, borrowed.left.clone(), borrowed.right.clone())
                };

                if index == level_size - 1 {
                    result.push(value);
                }
                if let Some(left) = left {
                    queue.push_back(left);
                }
                if let Some(right) = right {
                    queue.push_back(right);
                }
            }
        }

        result
    }
}