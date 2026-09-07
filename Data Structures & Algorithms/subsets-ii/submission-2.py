from collections import Counter
from typing import List


class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        groups = sorted(Counter(nums).items())
        result = []
        path = []

        def dfs(index):
            if index == len(groups):
                result.append(path.copy())
                return
            value, available = groups[index]
            for count in range(available + 1):
                path.extend([value] * count)
                dfs(index + 1)
                if count:
                    del path[-count:]

        dfs(0)
        return result