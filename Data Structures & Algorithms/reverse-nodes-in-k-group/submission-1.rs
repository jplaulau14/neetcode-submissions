impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut group_prev = &mut dummy.next;

        loop {
            {
                let mut check = group_prev.as_ref();
                for _ in 0..k {
                    match check {
                        Some(node) => check = node.next.as_ref(),
                        None => return dummy.next,
                    }
                }
            }

            let mut current = group_prev.take();
            let mut reversed = None;

            for _ in 0..k {
                let mut node = current.take().unwrap();
                current = node.next.take();
                node.next = reversed.take();
                reversed = Some(node);
            }

            *group_prev = reversed;
            for _ in 0..k {
                group_prev = &mut group_prev.as_mut().unwrap().next;
            }
            *group_prev = current;
        }
    }
}