class Solution:
    def lowestCommonAncestor(self, root, p, q):
        low = min(p.val, q.val)
        high = max(p.val, q.val)
        node = root

        while node is not None:
            if high < node.val:
                node = node.left
            elif low > node.val:
                node = node.right
            else:
                return node

        return None