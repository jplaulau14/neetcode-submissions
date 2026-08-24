class Solution:
    def rightSideView(self, root):
        result = []

        def visit(node, depth):
            if node is None:
                return
            if depth == len(result):
                result.append(node.val)
            visit(node.right, depth + 1)
            visit(node.left, depth + 1)

        visit(root, 0)
        return result