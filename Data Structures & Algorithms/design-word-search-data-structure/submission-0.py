class WordDictionary:
    def __init__(self):
        self.children = [None] * 26
        self.is_word = False

    def addWord(self, word: str) -> None:
        node = self
        for char in word:
            index = ord(char) - ord("a")
            if node.children[index] is None:
                node.children[index] = WordDictionary()
            node = node.children[index]
        node.is_word = True

    def search(self, word: str) -> bool:
        return self._search(word, 0)

    def _search(self, word: str, index: int) -> bool:
        if index == len(word):
            return self.is_word
        char = word[index]
        if char != ".":
            child = self.children[ord(char) - ord("a")]
            return child is not None and child._search(word, index + 1)
        return any(child is not None and child._search(word, index + 1)
                   for child in self.children)