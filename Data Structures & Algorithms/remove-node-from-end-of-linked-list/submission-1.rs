impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut lead = dummy
            .next
            .as_deref()
            .map_or(std::ptr::null(), |node| node as *const ListNode);

        for _ in 0..n {
            unsafe {
                lead = (*lead)
                    .next
                    .as_deref()
                    .map_or(std::ptr::null(), |node| node as *const ListNode);
            }
        }

        let mut trail = &mut dummy.next;
        while !lead.is_null() {
            unsafe {
                lead = (*lead)
                    .next
                    .as_deref()
                    .map_or(std::ptr::null(), |node| node as *const ListNode);
            }
            trail = &mut trail.as_mut().unwrap().next;
        }

        let mut removed = trail.take().unwrap();
        *trail = removed.next.take();
        dummy.next
    }
}