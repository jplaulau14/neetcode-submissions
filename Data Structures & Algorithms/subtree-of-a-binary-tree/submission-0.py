class Solution:
    def isSubtree(self, root, subRoot):
        if subRoot is None:
            return True

        null_token = 10001

        def serialize(node):
            result = []
            stack = [node]
            while stack:
                current = stack.pop()
                if current is None:
                    result.append(null_token)
                    continue
                result.append(current.val)
                stack.append(current.right)
                stack.append(current.left)
            return result

        pattern = serialize(subRoot)
        text = serialize(root)
        prefix = [0] * len(pattern)
        length = 0
        for index in range(1, len(pattern)):
            while length and pattern[index] != pattern[length]:
                length = prefix[length - 1]
            if pattern[index] == pattern[length]:
                length += 1
            prefix[index] = length

        length = 0
        for token in text:
            while length and token != pattern[length]:
                length = prefix[length - 1]
            if token == pattern[length]:
                length += 1
            if length == len(pattern):
                return True

        return False