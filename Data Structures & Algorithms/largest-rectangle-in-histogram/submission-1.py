class Solution:
    def largestRectangleArea(self, heights: list[int]) -> int:
        n = len(heights)
        left = [0] * n
        right = [n] * n
        stack = []

        for i, height in enumerate(heights):
            while stack and heights[stack[-1]] >= height:
                stack.pop()

            left[i] = stack[-1] + 1 if stack else 0
            stack.append(i)

        stack = []

        for i in range(n - 1, -1, -1):
            while stack and heights[stack[-1]] >= heights[i]:
                stack.pop()

            right[i] = stack[-1] if stack else n
            stack.append(i)

        best = 0

        for i, height in enumerate(heights):
            best = max(best, height * (right[i] - left[i]))

        return best