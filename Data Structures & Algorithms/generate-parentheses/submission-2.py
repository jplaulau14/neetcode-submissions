from typing import List


class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        dp = [[] for _ in range(n + 1)]
        dp[0] = [""]
        for pairs in range(1, n + 1):
            for inside in range(pairs):
                outside = pairs - 1 - inside
                for left in dp[inside]:
                    for right in dp[outside]:
                        dp[pairs].append("(" + left + ")" + right)
        return dp[n]