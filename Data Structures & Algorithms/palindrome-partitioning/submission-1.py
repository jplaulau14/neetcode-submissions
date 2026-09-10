from typing import List


class Solution:
    def partition(self, s: str) -> List[List[str]]:
        n = len(s)
        pal = [[False] * n for _ in range(n)]
        for length in range(1, n + 1):
            for start in range(n - length + 1):
                end = start + length - 1
                pal[start][end] = s[start] == s[end] and (length <= 2 or pal[start + 1][end - 1])

        result = []
        path = []

        def dfs(start):
            if start == n:
                result.append([s[left:right] for left, right in path])
                return
            for end in range(start, n):
                if not pal[start][end]:
                    continue
                path.append((start, end + 1))
                dfs(end + 1)
                path.pop()

        dfs(0)
        return result