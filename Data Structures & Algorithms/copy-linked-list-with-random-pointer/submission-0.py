class Solution:
    def copyRandomList(self, head: 'Node | None') -> 'Node | None':
        if head is None:
            return None

        current = head
        while current is not None:
            copy = Node(current.val, current.next)
            current.next = copy
            current = copy.next

        current = head
        while current is not None:
            copy = current.next
            copy.random = current.random.next if current.random is not None else None
            current = copy.next

        copied_head = head.next
        original = head
        while original is not None:
            copy = original.next
            original.next = copy.next
            copy.next = original.next.next if original.next is not None else None
            original = original.next

        return copied_head