from collections import deque


class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: list[str]) -> int:
        words = set(wordList)
        if endWord not in words:
            return 0
        words.discard(beginWord)
        queue = deque([beginWord])
        length = 1
        while queue:
            for _ in range(len(queue)):
                word = queue.popleft()
                letters = list(word)
                for i in range(len(letters)):
                    saved = letters[i]
                    for letter in "abcdefghijklmnopqrstuvwxyz":
                        if letter == saved:
                            continue
                        letters[i] = letter
                        nxt = "".join(letters)
                        if nxt not in words:
                            continue
                        if nxt == endWord:
                            return length + 1
                        words.remove(nxt)
                        queue.append(nxt)
                    letters[i] = saved
            length += 1
        return 0