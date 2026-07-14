impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [0u16; 9];
        let mut columns = [0u16; 9];
        let mut boxes = [0u16; 9];

        for row in 0..9 {
            for column in 0..9 {
                let value = board[row][column];
                if value == '.' {
                    continue;
                }

                let bit = 1u16 << (value as u8 - b'1');
                let box_index = (row / 3) * 3 + column / 3;

                if rows[row] & bit != 0
                    || columns[column] & bit != 0
                    || boxes[box_index] & bit != 0
                {
                    return false;
                }

                rows[row] |= bit;
                columns[column] |= bit;
                boxes[box_index] |= bit;
            }
        }

        true
    }
}
