class Solution:
    def rob(self, nums: List[int]) -> int:
        def linear(lo: int, hi: int) -> int:
            prev = 0
            curr = 0
            for i in range(lo, hi):
                prev, curr = curr, max(curr, prev + nums[i])
            return curr

        if len(nums) == 1:
            return nums[0]
        return max(linear(0, len(nums) - 1), linear(1, len(nums)))