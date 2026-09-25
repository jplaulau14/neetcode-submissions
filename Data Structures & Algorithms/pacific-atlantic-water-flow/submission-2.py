from collections import deque
from typing import List


class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        rows = len(heights)
        cols = len(heights[0])
        mask = [[0] * cols for _ in range(rows)]
        queue = deque()
        for r in range(rows):
            for c in range(cols):
                bits = (1 if r == 0 or c == 0 else 0) | (2 if r == rows - 1 or c == cols - 1 else 0)
                if bits:
                    mask[r][c] = bits
                    queue.append((r, c))
        while queue:
            row, col = queue.popleft()
            bits = mask[row][col]
            for nr, nc in ((row - 1, col), (row + 1, col), (row, col - 1), (row, col + 1)):
                if 0 <= nr < rows and 0 <= nc < cols and heights[nr][nc] >= heights[row][col] and mask[nr][nc] | bits != mask[nr][nc]:
                    mask[nr][nc] |= bits
                    queue.append((nr, nc))
        return [[r, c] for r in range(rows) for c in range(cols) if mask[r][c] == 3]