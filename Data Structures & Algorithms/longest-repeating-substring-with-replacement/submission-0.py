class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        counts = [0] * 26
        left = 0
        maximum_frequency = 0

        for right, character in enumerate(s):
            index = ord(character) - ord("A")
            counts[index] += 1
            maximum_frequency = max(maximum_frequency, counts[index])

            if right - left + 1 - maximum_frequency > k:
                counts[ord(s[left]) - ord("A")] -= 1
                left += 1

        return len(s) - left