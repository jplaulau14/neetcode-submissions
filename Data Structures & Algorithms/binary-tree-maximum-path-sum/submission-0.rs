use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        struct Frame {
            node: Rc<RefCell<TreeNode>>,
            state: u8,
            left_gain: i32,
        }

        let mut stack = vec![Frame {
            node: root,
            state: 0,
            left_gain: 0,
        }];
        let mut last_gain = 0;
        let mut best = stack[0].node.borrow().val;

        while !stack.is_empty() {
            let state = stack.last().unwrap().state;
            if state == 0 {
                let left = stack.last().unwrap().node.borrow().left.clone();
                stack.last_mut().unwrap().state = 1;
                if let Some(left) = left {
                    stack.push(Frame {
                        node: left,
                        state: 0,
                        left_gain: 0,
                    });
                }
            } else if state == 1 {
                let (left_exists, right) = {
                    let borrowed = stack.last().unwrap().node.borrow();
                    (borrowed.left.is_some(), borrowed.right.clone())
                };
                let parent = stack.last_mut().unwrap();
                parent.left_gain = if left_exists { last_gain } else { 0 };
                parent.state = 2;
                if let Some(right) = right {
                    stack.push(Frame {
                        node: right,
                        state: 0,
                        left_gain: 0,
                    });
                }
            } else {
                let frame = stack.pop().unwrap();
                let (value, right_exists) = {
                    let borrowed = frame.node.borrow();
                    (borrowed.val, borrowed.right.is_some())
                };
                let right_gain = if right_exists { last_gain } else { 0 };
                let left_arm = frame.left_gain.max(0);
                let right_arm = right_gain.max(0);
                let through = value + left_arm + right_arm;
                best = best.max(through);
                last_gain = value + left_arm.max(right_arm);
            }
        }

        best
    }
}