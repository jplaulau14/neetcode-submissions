import heapq


class Solution:
    def mergeKLists(self, lists):
        heap = []

        for index, node in enumerate(lists):
            if node is not None:
                heapq.heappush(heap, (node.val, index, node))

        dummy = ListNode(0)
        tail = dummy

        while heap:
            _, index, node = heapq.heappop(heap)
            tail.next = node
            tail = node

            if node.next is not None:
                heapq.heappush(heap, (node.next.val, index, node.next))

        return dummy.next