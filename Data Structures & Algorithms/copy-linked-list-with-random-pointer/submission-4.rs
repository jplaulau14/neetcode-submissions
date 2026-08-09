impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        if head.is_none() {
            return None;
        }

        let mut map = HashMap::new();

        // First pass: create all nodes and store them in a map
        let mut current = head.clone();
        while let Some(node) = current {
            let new_node = Rc::new(RefCell::new(Node::new(node.borrow().val)));
            map.insert(Rc::as_ptr(&node), new_node);
            current = node.borrow().next.clone();
        }

        // Second pass: assign next and random pointers
        current = head.clone();
        while let Some(node) = current {
            let node_borrow = node.borrow();
            let new_node = map.get(&Rc::as_ptr(&node)).unwrap();
            
            if let Some(ref next_node) = node_borrow.next {
                new_node.borrow_mut().next = Some(map.get(&Rc::as_ptr(next_node)).unwrap().clone());
            }
            
            if let Some(ref random_node) = node_borrow.random {
                new_node.borrow_mut().random = Some(map.get(&Rc::as_ptr(random_node)).unwrap().clone());
            }
            
            current = node_borrow.next.clone();
        }

        Some(map.get(&Rc::as_ptr(&head.unwrap())).unwrap().clone())
    }
}