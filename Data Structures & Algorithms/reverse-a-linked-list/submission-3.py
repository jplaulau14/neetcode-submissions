import sys
from typing import Optional

sys.setrecursionlimit(6000)

class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None or head.next is None:
            return head

        new_head = self.reverseList(head.next)
        head.next.next = head
        head.next = None
        return new_head