class Solution:
    def copyRandomList(self, head: 'Node | None') -> 'Node | None':
        old_to_copy = {None: None}
        current = head

        while current is not None:
            old_to_copy[current] = Node(current.val)
            current = current.next

        current = head
        while current is not None:
            copy = old_to_copy[current]
            copy.next = old_to_copy[current.next]
            copy.random = old_to_copy[current.random]
            current = current.next

        return old_to_copy[head]