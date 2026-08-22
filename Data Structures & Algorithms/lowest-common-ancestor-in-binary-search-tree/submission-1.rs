use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_value = p.as_ref()?.borrow().val;
        let q_value = q.as_ref()?.borrow().val;
        let low = p_value.min(q_value);
        let high = p_value.max(q_value);
        let mut node = root;

        while let Some(current) = node {
            let (value, left, right) = {
                let current_ref = current.borrow();
                (
                    current_ref.val,
                    current_ref.left.clone(),
                    current_ref.right.clone(),
                )
            };
            if high < value {
                node = left;
            } else if low > value {
                node = right;
            } else {
                return Some(current);
            }
        }

        None
    }
}