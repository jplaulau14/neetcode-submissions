impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let start = node?;
        let mut clones: Vec<Option<Rc<RefCell<Node>>>> = vec![None; 101];
        let start_val = start.borrow().val as usize;
        clones[start_val] = Some(Rc::new(RefCell::new(Node::new(start.borrow().val))));
        let mut stack = vec![start];
        while let Some(current) = stack.pop() {
            let current_val = current.borrow().val as usize;
            let copy = Rc::clone(clones[current_val].as_ref().unwrap());
            for neighbor in current.borrow().neighbors.iter() {
                let neighbor_val = neighbor.borrow().val as usize;
                if clones[neighbor_val].is_none() {
                    clones[neighbor_val] = Some(Rc::new(RefCell::new(Node::new(neighbor.borrow().val))));
                    stack.push(Rc::clone(neighbor));
                }
                let neighbor_copy = Rc::clone(clones[neighbor_val].as_ref().unwrap());
                copy.borrow_mut().neighbors.push(neighbor_copy);
            }
        }
        clones[start_val].take()
    }
}