from typing import List

class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        n = len(nums)
        left_max = [0] * n
        right_max = [0] * n

        for index in range(n):
            if index % k == 0:
                left_max[index] = nums[index]
            else:
                left_max[index] = max(left_max[index - 1], nums[index])

        for index in range(n - 1, -1, -1):
            if index == n - 1 or (index + 1) % k == 0:
                right_max[index] = nums[index]
            else:
                right_max[index] = max(right_max[index + 1], nums[index])

        return [
            max(right_max[left], left_max[left + k - 1])
            for left in range(n - k + 1)
        ]