class Solution:
    def maxArea(self, height: list[int]) -> int:
        left = 0
        right = len(height) - 1
        best = 0

        while left < right:
            width = right - left

            if height[left] <= height[right]:
                best = max(best, height[left] * width)
                left += 1
            else:
                best = max(best, height[right] * width)
                right -= 1

        return best