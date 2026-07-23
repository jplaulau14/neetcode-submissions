class Solution:
    def search(self, nums: list[int], target: int) -> int:
        left = 0
        right = len(nums)

        while left < right:
            middle = left + (right - left) // 2

            if nums[middle] == target:
                return middle

            if nums[middle] < target:
                left = middle + 1
            else:
                right = middle

        return -1