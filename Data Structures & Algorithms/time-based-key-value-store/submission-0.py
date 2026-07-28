class TimeMap:
    def __init__(self):
        self.store = {}

    def set(self, key: str, value: str, timestamp: int) -> None:
        self.store.setdefault(key, []).append((timestamp, value))

    def get(self, key: str, timestamp: int) -> str:
        history = self.store.get(key, [])
        left = 0
        right = len(history)

        while left < right:
            middle = left + (right - left) // 2

            if history[middle][0] <= timestamp:
                left = middle + 1
            else:
                right = middle

        return "" if left == 0 else history[left - 1][1]