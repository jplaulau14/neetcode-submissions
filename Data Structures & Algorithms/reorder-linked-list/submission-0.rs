impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.as_ref().is_none_or(|node| node.next.is_none()) {
            return;
        }

        unsafe {
            let mut slow = head.as_deref_mut().unwrap() as *mut ListNode;
            let mut fast = (*slow)
                .next
                .as_deref_mut()
                .map_or(std::ptr::null_mut(), |node| node as *mut ListNode);

            while !fast.is_null() && (*fast).next.is_some() {
                slow = (*slow).next.as_deref_mut().unwrap() as *mut ListNode;
                fast = (*fast)
                    .next
                    .as_deref_mut()
                    .unwrap()
                    .next
                    .as_deref_mut()
                    .map_or(std::ptr::null_mut(), |node| node as *mut ListNode);
            }

            let mut second = (*slow).next.take();
            let mut previous = None;
            while let Some(mut node) = second {
                second = node.next.take();
                node.next = previous;
                previous = Some(node);
            }

            let mut first = head.as_deref_mut().unwrap() as *mut ListNode;
            while let Some(mut node) = previous {
                previous = node.next.take();
                let first_following = (*first).next.take();
                (*first).next = Some(node);
                let inserted = (*first).next.as_deref_mut().unwrap() as *mut ListNode;
                (*inserted).next = first_following;
                first = (*inserted)
                    .next
                    .as_deref_mut()
                    .map_or(std::ptr::null_mut(), |node| node as *mut ListNode);
            }
        }
    }
}