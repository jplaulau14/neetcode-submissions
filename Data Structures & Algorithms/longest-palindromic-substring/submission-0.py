class Solution:
    def longestPalindrome(self, s: str) -> str:
        n = len(s)
        best_l = 0
        best_r = 0

        def expand(left: int, right: int) -> None:
            nonlocal best_l, best_r
            while left >= 0 and right < n and s[left] == s[right]:
                if right - left > best_r - best_l:
                    best_l = left
                    best_r = right
                left -= 1
                right += 1

        for i in range(n):
            expand(i, i)
            expand(i, i + 1)
        return s[best_l : best_r + 1]