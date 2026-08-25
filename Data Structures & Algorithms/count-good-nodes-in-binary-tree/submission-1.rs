use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let mut stack = vec![(root, i32::MIN)];
        let mut answer = 0;
        while let Some((node, path_max)) = stack.pop() {
            let (value, left, right) = {
                let borrowed = node.borrow();
                (borrowed.val, borrowed.left.clone(), borrowed.right.clone())
            };
            if value >= path_max {
                answer += 1;
            }
            let current_max = value.max(path_max);
            if let Some(right) = right {
                stack.push((right, current_max));
            }
            if let Some(left) = left {
                stack.push((left, current_max));
            }
        }
        answer
    }
}