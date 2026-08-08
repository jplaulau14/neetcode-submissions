from typing import Optional


class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        if head is None or head.next is None:
            return

        slow = head
        fast = head.next
        while fast is not None and fast.next is not None:
            slow = slow.next
            fast = fast.next.next

        second = slow.next
        slow.next = None

        previous = None
        while second is not None:
            following = second.next
            second.next = previous
            previous = second
            second = following

        first = head
        second = previous
        while second is not None:
            first_following = first.next
            second_following = second.next
            first.next = second
            second.next = first_following
            first = first_following
            second = second_following