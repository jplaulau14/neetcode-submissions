from collections import deque
from typing import List


class Solution:
    def islandsAndTreasure(self, grid: List[List[int]]) -> None:
        INF = 2147483647
        rows = len(grid)
        cols = len(grid[0])
        queue = deque((r, c) for r in range(rows) for c in range(cols) if grid[r][c] == 0)
        while queue:
            row, col = queue.popleft()
            next_dist = grid[row][col] + 1
            for nr, nc in ((row - 1, col), (row + 1, col), (row, col - 1), (row, col + 1)):
                if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] == INF:
                    grid[nr][nc] = next_dist
                    queue.append((nr, nc))