use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn same_tree(
            first: &Option<Rc<RefCell<TreeNode>>>,
            second: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            let mut stack = vec![(first.clone(), second.clone())];

            while let Some((left, right)) = stack.pop() {
                match (left, right) {
                    (None, None) => {}
                    (Some(left), Some(right)) => {
                        let (left_value, left_left, left_right) = {
                            let node = left.borrow();
                            (node.val, node.left.clone(), node.right.clone())
                        };
                        let (right_value, right_left, right_right) = {
                            let node = right.borrow();
                            (node.val, node.left.clone(), node.right.clone())
                        };
                        if left_value != right_value {
                            return false;
                        }
                        stack.push((left_left, right_left));
                        stack.push((left_right, right_right));
                    }
                    _ => return false,
                }
            }

            true
        }

        if sub_root.is_none() {
            return true;
        }
        if root.is_none() {
            return false;
        }

        let target_value = sub_root.as_ref().unwrap().borrow().val;
        let mut candidates = vec![root];

        while let Some(Some(node)) = candidates.pop() {
            let (value, left, right) = {
                let node = node.borrow();
                (node.val, node.left.clone(), node.right.clone())
            };
            if value == target_value {
                let candidate = Some(node.clone());
                if same_tree(&candidate, &sub_root) {
                    return true;
                }
            }
            if let Some(right) = right {
                candidates.push(Some(right));
            }
            if let Some(left) = left {
                candidates.push(Some(left));
            }
        }

        false
    }
}