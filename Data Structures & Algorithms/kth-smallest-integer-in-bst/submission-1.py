class Solution:
    def kthSmallest(self, root: TreeNode | None, k: int) -> int:
        stack = []
        node = root

        while True:
            while node is not None:
                stack.append(node)
                node = node.left

            node = stack.pop()
            k -= 1
            if k == 0:
                return node.val
            node = node.right