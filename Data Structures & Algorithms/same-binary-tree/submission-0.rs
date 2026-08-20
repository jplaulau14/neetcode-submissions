use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn same_tree(
            p: &Option<Rc<RefCell<TreeNode>>>,
            q: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(p), Some(q)) => {
                    let (p_val, p_left, p_right) = {
                        let p = p.borrow();
                        (p.val, p.left.clone(), p.right.clone())
                    };
                    let (q_val, q_left, q_right) = {
                        let q = q.borrow();
                        (q.val, q.left.clone(), q.right.clone())
                    };
                    p_val == q_val && same_tree(&p_left, &q_left) && same_tree(&p_right, &q_right)
                }
                _ => false,
            }
        }

        same_tree(&p, &q)
    }
}