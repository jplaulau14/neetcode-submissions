class Solution:
    def maxDepth(self, root: TreeNode | None) -> int:
        if root is None:
            return 0

        stack = [(root, 1)]
        answer = 0
        while stack:
            node, depth = stack.pop()
            answer = max(answer, depth)
            if node.left is not None:
                stack.append((node.left, depth + 1))
            if node.right is not None:
                stack.append((node.right, depth + 1))
        return answer