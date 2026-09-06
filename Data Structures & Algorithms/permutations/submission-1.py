from typing import List


class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        values = nums.copy()
        result = []

        def dfs(start):
            if start == len(values):
                result.append(values.copy())
                return
            for i in range(start, len(values)):
                values[start], values[i] = values[i], values[start]
                dfs(start + 1)
                values[start], values[i] = values[i], values[start]

        dfs(0)
        return result