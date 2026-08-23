use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return Vec::new();
        };

        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut result = Vec::new();

        while !queue.is_empty() {
            let level_len = queue.len();
            let mut level = Vec::with_capacity(level_len);

            for _ in 0..level_len {
                let node = queue.pop_front().unwrap();
                let (value, left, right) = {
                    let borrowed = node.borrow();
                    (borrowed.val, borrowed.left.clone(), borrowed.right.clone())
                };
                level.push(value);
                if let Some(left) = left {
                    queue.push_back(left);
                }
                if let Some(right) = right {
                    queue.push_back(right);
                }
            }

            result.push(level);
        }

        result
    }
}