use std::cell::RefCell;
use std::rc::Rc;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut tokens = Vec::new();
        let mut stack = vec![root];

        while let Some(node) = stack.pop() {
            match node {
                Some(node) => {
                    let (value, left, right) = {
                        let node = node.borrow();
                        (node.val, node.left.clone(), node.right.clone())
                    };
                    tokens.push(value.to_string());
                    stack.push(right);
                    stack.push(left);
                }
                None => tokens.push("N".to_owned()),
            }
        }

        tokens.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut tokens = data.split(',');
        let first = tokens.next()?;
        if first == "N" {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(first.parse().ok()?)));
        let mut stack = vec![(root.clone(), false)];

        for token in tokens {
            let parent = stack.last().unwrap().0.clone();
            let is_right = stack.last().unwrap().1;
            let child = if token == "N" {
                None
            } else {
                Some(Rc::new(RefCell::new(TreeNode::new(token.parse().ok()?))))
            };

            if is_right {
                parent.borrow_mut().right = child.clone();
                stack.pop();
            } else {
                parent.borrow_mut().left = child.clone();
                stack.last_mut().unwrap().1 = true;
            }

            if let Some(child) = child {
                stack.push((child, false));
            }
        }

        Some(root)
    }
}