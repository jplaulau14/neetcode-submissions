class Solution:
    def maxPathSum(self, root: TreeNode | None) -> int:
        if root is None:
            return 0

        stack = [[root, 0, 0]]
        last_gain = 0
        best = root.val

        while stack:
            node, state, _ = stack[-1]
            if state == 0:
                stack[-1][1] = 1
                if node.left is not None:
                    stack.append([node.left, 0, 0])
            elif state == 1:
                stack[-1][2] = last_gain if node.left is not None else 0
                stack[-1][1] = 2
                if node.right is not None:
                    stack.append([node.right, 0, 0])
            else:
                _, _, left_gain = stack.pop()
                right_gain = last_gain if node.right is not None else 0
                left_arm = max(0, left_gain)
                right_arm = max(0, right_gain)
                through = node.val + left_arm + right_arm
                best = max(best, through)
                last_gain = node.val + max(left_arm, right_arm)

        return best