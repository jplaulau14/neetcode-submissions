class Solution:
    def buildTree(self, preorder, inorder):
        if not preorder:
            return None

        root = TreeNode(preorder[0])
        stack = [root]
        inorder_index = 0

        for value in preorder[1:]:
            node = stack[-1]
            if node.val != inorder[inorder_index]:
                node.left = TreeNode(value)
                stack.append(node.left)
            else:
                while stack and stack[-1].val == inorder[inorder_index]:
                    node = stack.pop()
                    inorder_index += 1
                node.right = TreeNode(value)
                stack.append(node.right)

        return root