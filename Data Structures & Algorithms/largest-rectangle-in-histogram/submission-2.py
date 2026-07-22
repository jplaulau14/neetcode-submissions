class Solution:
    def largestRectangleArea(self, heights: list[int]) -> int:
        best = 0
        stack = []

        for i in range(len(heights) + 1):
            height = heights[i] if i < len(heights) else 0
            start = i

            while stack and stack[-1][1] > height:
                index, previous_height = stack.pop()
                best = max(best, previous_height * (i - index))
                start = index

            stack.append((start, height))

        return best