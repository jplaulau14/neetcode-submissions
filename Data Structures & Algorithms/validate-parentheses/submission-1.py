class Solution:
    def isValid(self, s: str) -> bool:
        stack = []

        for bracket in s:
            if bracket == "(":
                stack.append(")")
            elif bracket == "[":
                stack.append("]")
            elif bracket == "{":
                stack.append("}")
            elif not stack or stack.pop() != bracket:
                return False

        return not stack