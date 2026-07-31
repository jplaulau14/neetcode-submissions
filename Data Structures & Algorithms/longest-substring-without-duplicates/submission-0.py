class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        next_start = {}
        left = 0
        best = 0

        for right, character in enumerate(s):
            left = max(left, next_start.get(character, 0))
            next_start[character] = right + 1
            best = max(best, right - left + 1)

        return best