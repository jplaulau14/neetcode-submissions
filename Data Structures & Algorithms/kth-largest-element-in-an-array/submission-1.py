from typing import List

class Solution:
    def findKthLargest(self, nums: List[int], k: int) -> int:
        counts = [0] * 20001
        for value in nums:
            counts[value + 10000] += 1
        for index in range(20000, -1, -1):
            k -= counts[index]
            if k <= 0:
                return index - 10000
        raise ValueError("Invalid rank")