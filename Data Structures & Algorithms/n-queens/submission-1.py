from typing import List


class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        result = []
        board = [['.'] * n for _ in range(n)]
        columns = [False] * n
        diagonals_down = [False] * (2 * n - 1)
        diagonals_up = [False] * (2 * n - 1)

        def dfs(row: int) -> None:
            if row == n:
                result.append([''.join(board_row) for board_row in board])
                return
            for col in range(n):
                diagonal_down = row - col + n - 1
                diagonal_up = row + col
                if (
                    columns[col]
                    or diagonals_down[diagonal_down]
                    or diagonals_up[diagonal_up]
                ):
                    continue
                columns[col] = True
                diagonals_down[diagonal_down] = True
                diagonals_up[diagonal_up] = True
                board[row][col] = 'Q'
                dfs(row + 1)
                board[row][col] = '.'
                diagonals_up[diagonal_up] = False
                diagonals_down[diagonal_down] = False
                columns[col] = False

        dfs(0)
        return result