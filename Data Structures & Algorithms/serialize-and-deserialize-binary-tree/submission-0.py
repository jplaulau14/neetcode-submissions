class Codec:
    def serialize(self, root):
        tokens = []
        stack = [root]
        while stack:
            node = stack.pop()
            if node is None:
                tokens.append("N")
            else:
                tokens.append(str(node.val))
                stack.append(node.right)
                stack.append(node.left)
        return ",".join(tokens)

    def deserialize(self, data):
        if not data:
            return None

        tokens = data.split(",")
        if tokens[0] == "N":
            return None

        root = TreeNode(int(tokens[0]))
        stack = [(root, 0)]
        for token in tokens[1:]:
            parent, side = stack[-1]
            child = None if token == "N" else TreeNode(int(token))
            if side == 0:
                parent.left = child
                stack[-1] = (parent, 1)
            else:
                parent.right = child
                stack.pop()
            if child is not None:
                stack.append((child, 0))
        return root