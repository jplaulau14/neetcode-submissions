class Solution:
    def isSameTree(self, p, q):
        stack = [(p, q)]

        while stack:
            left, right = stack.pop()
            if left is None or right is None:
                if left is not right:
                    return False
                continue
            if left.val != right.val:
                return False
            stack.append((left.left, right.left))
            stack.append((left.right, right.right))

        return True