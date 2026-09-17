from collections import Counter
from typing import List

class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        frequencies = Counter(tasks).values()
        maximum = max(frequencies)
        leaders = sum(count == maximum for count in frequencies)
        return max(len(tasks), (maximum - 1) * (n + 1) + leaders)