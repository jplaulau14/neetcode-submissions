class Solution:
    def isValidBST(self, root: TreeNode | None) -> bool:
        if root is None:
            return True
        stack = [(root, float("-inf"), float("inf"))]
        while stack:
            node, lower, upper = stack.pop()
            if not lower < node.val < upper:
                return False
            if node.right is not None:
                stack.append((node.right, node.val, upper))
            if node.left is not None:
                stack.append((node.left, lower, node.val))
        return True