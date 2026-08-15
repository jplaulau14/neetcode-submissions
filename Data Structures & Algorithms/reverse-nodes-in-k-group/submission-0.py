from typing import Optional

class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        dummy = ListNode(0, head)
        group_prev = dummy

        while True:
            kth = group_prev
            for _ in range(k):
                kth = kth.next
                if kth is None:
                    return dummy.next

            group_next = kth.next
            previous = group_next
            current = group_prev.next

            while current is not group_next:
                following = current.next
                current.next = previous
                previous = current
                current = following

            old_group_head = group_prev.next
            group_prev.next = kth
            group_prev = old_group_head