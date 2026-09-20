from typing import List


class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        rows = len(grid)
        cols = len(grid[0])
        islands = 0
        for r in range(rows):
            for c in range(cols):
                if grid[r][c] != "1":
                    continue
                islands += 1
                grid[r][c] = "0"
                stack = [(r, c)]
                while stack:
                    row, col = stack.pop()
                    for nr, nc in ((row - 1, col), (row + 1, col), (row, col - 1), (row, col + 1)):
                        if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] == "1":
                            grid[nr][nc] = "0"
                            stack.append((nr, nc))
        return islands