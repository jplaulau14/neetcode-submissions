use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(
        preorder: Vec<i32>,
        inorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        let mut stack = vec![root.clone()];
        let mut inorder_index = 0usize;

        for &value in preorder.iter().skip(1) {
            let node = stack.last().unwrap().clone();
            if node.borrow().val != inorder[inorder_index] {
                let child = Rc::new(RefCell::new(TreeNode::new(value)));
                node.borrow_mut().left = Some(child.clone());
                stack.push(child);
            } else {
                let mut parent = node;
                while !stack.is_empty() {
                    let matches = {
                        let last = stack.last().unwrap().borrow();
                        last.val == inorder[inorder_index]
                    };
                    if !matches {
                        break;
                    }
                    parent = stack.pop().unwrap();
                    inorder_index += 1;
                }
                let child = Rc::new(RefCell::new(TreeNode::new(value)));
                parent.borrow_mut().right = Some(child.clone());
                stack.push(child);
            }
        }

        Some(root)
    }
}