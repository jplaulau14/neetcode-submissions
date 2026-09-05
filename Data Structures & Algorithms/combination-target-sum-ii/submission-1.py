from collections import Counter
from typing import List


class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        groups = sorted(Counter(candidates).items())
        result = []
        path = []

        def dfs(index, remaining):
            if index == len(groups):
                if remaining == 0:
                    result.append(path.copy())
                return
            value, available = groups[index]
            for count in range(min(available, remaining // value) + 1):
                path.extend([value] * count)
                dfs(index + 1, remaining - count * value)
                if count:
                    del path[-count:]

        dfs(0, target)
        return result