from typing import List


class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        rows = len(grid)
        cols = len(grid[0])
        best = 0
        for r in range(rows):
            for c in range(cols):
                if grid[r][c] != 1:
                    continue
                grid[r][c] = 0
                area = 1
                stack = [(r, c)]
                while stack:
                    row, col = stack.pop()
                    for nr, nc in ((row - 1, col), (row + 1, col), (row, col - 1), (row, col + 1)):
                        if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] == 1:
                            grid[nr][nc] = 0
                            area += 1
                            stack.append((nr, nc))
                if area > best:
                    best = area
        return best