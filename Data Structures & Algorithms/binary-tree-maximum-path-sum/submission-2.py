class Solution:
    def maxPathSum(self, root: TreeNode | None) -> int:
        answer = root.val if root is not None else 0

        def gain(node: TreeNode | None) -> int:
            nonlocal answer
            if node is None:
                return 0
            left_gain = max(0, gain(node.left))
            right_gain = max(0, gain(node.right))
            answer = max(answer, node.val + left_gain + right_gain)
            return node.val + max(left_gain, right_gain)

        gain(root)
        return answer