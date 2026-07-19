class Solution:
    def evalRPN(self, tokens: list[str]) -> int:
        stack = []

        for token in tokens:
            if token in {"+", "-", "*", "/"}:
                right = stack.pop()
                left = stack.pop()

                if token == "+":
                    stack.append(left + right)
                elif token == "-":
                    stack.append(left - right)
                elif token == "*":
                    stack.append(left * right)
                else:
                    quotient = abs(left) // abs(right)
                    stack.append(quotient if (left < 0) == (right < 0) else -quotient)
            else:
                stack.append(int(token))

        return stack[-1]