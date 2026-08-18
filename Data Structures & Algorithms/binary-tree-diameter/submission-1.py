class Solution:
    def diameterOfBinaryTree(self, root: TreeNode | None) -> int:
        if root is None:
            return 0

        stack = [[root, 0, 0]]
        last_height = 0
        diameter = 0

        while stack:
            node, state, _ = stack[-1]
            if state == 0:
                stack[-1][1] = 1
                if node.left is not None:
                    stack.append([node.left, 0, 0])
            elif state == 1:
                stack[-1][2] = last_height if node.left is not None else 0
                stack[-1][1] = 2
                if node.right is not None:
                    stack.append([node.right, 0, 0])
            else:
                _, _, left_height = stack.pop()
                right_height = last_height if node.right is not None else 0
                diameter = max(diameter, left_height + right_height)
                last_height = 1 + max(left_height, right_height)

        return diameter