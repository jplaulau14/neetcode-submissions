class Solution:
    def goodNodes(self, root: TreeNode | None) -> int:
        if root is None:
            return 0

        stack = [(root, float("-inf"))]
        answer = 0
        while stack:
            node, path_max = stack.pop()
            if node.val >= path_max:
                answer += 1
            current_max = max(path_max, node.val)
            if node.right is not None:
                stack.append((node.right, current_max))
            if node.left is not None:
                stack.append((node.left, current_max))
        return answer