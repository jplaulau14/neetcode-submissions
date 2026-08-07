impl Solution {
    pub fn has_cycle(head: *mut ListNode) -> bool {
        if head.is_null() {
            return false;
        }

        let mut slow = head;
        let mut fast = head;

        loop {
            unsafe {
                if (*fast).next.is_null() || (*(*fast).next).next.is_null() {
                    return false;
                }
                
                slow = (*slow).next;
                fast = (*(*fast).next).next;

                if slow == fast {
                    return true;
                }
            }
        }
    }
}