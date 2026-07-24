class Solution:
    def searchMatrix(self, matrix: list[list[int]], target: int) -> bool:
        if not matrix or not matrix[0]:
            return False

        top = 0
        bottom = len(matrix) - 1

        while top <= bottom:
            row = top + (bottom - top) // 2

            if target < matrix[row][0]:
                bottom = row - 1
            elif target > matrix[row][-1]:
                top = row + 1
            else:
                left = 0
                right = len(matrix[row]) - 1

                while left <= right:
                    column = left + (right - left) // 2

                    if matrix[row][column] == target:
                        return True
                    if matrix[row][column] < target:
                        left = column + 1
                    else:
                        right = column - 1

                return False

        return False