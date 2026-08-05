impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn reverse(
            current: Option<Box<ListNode>>,
            previous: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match current {
                None => previous,
                Some(mut node) => {
                    let following = node.next.take();
                    node.next = previous;
                    reverse(following, Some(node))
                }
            }
        }

        reverse(head, None)
    }
}