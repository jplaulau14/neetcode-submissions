class TrieNode:
    def __init__(self):
        self.children = {}
        self.word = None


class Solution:
    def findWords(self, board, words):
        root = TrieNode()
        for word in words:
            node = root
            for char in word:
                if char not in node.children:
                    node.children[char] = TrieNode()
                node = node.children[char]
            node.word = word

        rows = len(board)
        cols = len(board[0])
        result = []

        def dfs(row, col, node):
            char = board[row][col]
            child = node.children.get(char)
            if child is None:
                return
            if child.word is not None:
                result.append(child.word)
                child.word = None
            board[row][col] = "#"
            if row > 0 and board[row - 1][col] != "#":
                dfs(row - 1, col, child)
            if row + 1 < rows and board[row + 1][col] != "#":
                dfs(row + 1, col, child)
            if col > 0 and board[row][col - 1] != "#":
                dfs(row, col - 1, child)
            if col + 1 < cols and board[row][col + 1] != "#":
                dfs(row, col + 1, child)
            board[row][col] = char
            if child.word is None and not child.children:
                node.children.pop(char)

        for row in range(rows):
            for col in range(cols):
                dfs(row, col, root)
        return result