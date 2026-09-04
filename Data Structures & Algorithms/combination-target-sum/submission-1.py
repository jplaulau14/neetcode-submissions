from typing import List


class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        values = sorted(candidates)
        result = []
        path = []

        def dfs(start, remaining):
            if remaining == 0:
                result.append(path.copy())
                return
            for i in range(start, len(values)):
                value = values[i]
                if value > remaining:
                    break
                path.append(value)
                dfs(i, remaining - value)
                path.pop()

        dfs(0, target)
        return result