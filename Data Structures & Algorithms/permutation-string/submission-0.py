class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        window_size = len(s1)
        if window_size > len(s2):
            return False

        required = [0] * 26
        window = [0] * 26

        for i in range(window_size):
            required[ord(s1[i]) - ord("a")] += 1
            window[ord(s2[i]) - ord("a")] += 1

        matches = sum(required[i] == window[i] for i in range(26))
        if matches == 26:
            return True

        for right in range(window_size, len(s2)):
            entering = ord(s2[right]) - ord("a")
            window[entering] += 1
            if window[entering] == required[entering]:
                matches += 1
            elif window[entering] == required[entering] + 1:
                matches -= 1

            leaving = ord(s2[right - window_size]) - ord("a")
            window[leaving] -= 1
            if window[leaving] == required[leaving]:
                matches += 1
            elif window[leaving] == required[leaving] - 1:
                matches -= 1

            if matches == 26:
                return True

        return False