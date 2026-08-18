use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        struct Frame {
            node: Rc<RefCell<TreeNode>>,
            state: u8,
            left_height: i32,
        }

        let mut stack = vec![Frame {
            node: root,
            state: 0,
            left_height: 0,
        }];
        let mut last_height = 0;
        let mut diameter = 0;

        while !stack.is_empty() {
            let state = stack.last().unwrap().state;
            if state == 0 {
                let left = stack.last().unwrap().node.borrow().left.clone();
                stack.last_mut().unwrap().state = 1;
                if let Some(left) = left {
                    stack.push(Frame {
                        node: left,
                        state: 0,
                        left_height: 0,
                    });
                }
            } else if state == 1 {
                let (left_exists, right) = {
                    let borrowed = stack.last().unwrap().node.borrow();
                    (borrowed.left.is_some(), borrowed.right.clone())
                };
                let parent = stack.last_mut().unwrap();
                parent.left_height = if left_exists { last_height } else { 0 };
                parent.state = 2;
                if let Some(right) = right {
                    stack.push(Frame {
                        node: right,
                        state: 0,
                        left_height: 0,
                    });
                }
            } else {
                let frame = stack.pop().unwrap();
                let right_exists = frame.node.borrow().right.is_some();
                let right_height = if right_exists { last_height } else { 0 };
                diameter = diameter.max(frame.left_height + right_height);
                last_height = 1 + frame.left_height.max(right_height);
            }
        }

        diameter
    }
}