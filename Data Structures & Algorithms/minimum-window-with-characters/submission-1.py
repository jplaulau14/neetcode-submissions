class Solution:
    def minWindow(self, s: str, t: str) -> str:
        if not t or len(t) > len(s):
            return ""

        required = [0] * 128
        window = [0] * 128
        needed_types = 0

        for character in t:
            index = ord(character)
            if required[index] == 0:
                needed_types += 1
            required[index] += 1

        formed = 0
        left = 0
        best_start = 0
        best_length = len(s) + 1

        for right, character in enumerate(s):
            index = ord(character)
            window[index] += 1

            if required[index] > 0 and window[index] == required[index]:
                formed += 1

            while formed == needed_types:
                length = right - left + 1
                if length < best_length:
                    best_start = left
                    best_length = length

                left_index = ord(s[left])
                if required[left_index] > 0 and window[left_index] == required[left_index]:
                    formed -= 1
                window[left_index] -= 1
                left += 1

        if best_length == len(s) + 1:
            return ""

        return s[best_start:best_start + best_length]