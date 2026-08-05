impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous = None;
        let mut current = head;

        while let Some(mut node) = current {
            current = node.next.take();
            node.next = previous;
            previous = Some(node);
        }

        previous
    }
}