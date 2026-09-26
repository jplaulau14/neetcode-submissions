impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows = board.len();
        let cols = board[0].len();
        let mut stack = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in [0, cols - 1] {
                if board[r][c] == 'O' {
                    board[r][c] = '#';
                    stack.push(r * cols + c);
                }
            }
        }
        for c in 0..cols {
            for r in [0, rows - 1] {
                if board[r][c] == 'O' {
                    board[r][c] = '#';
                    stack.push(r * cols + c);
                }
            }
        }
        while let Some(id) = stack.pop() {
            let r = id / cols;
            let c = id % cols;
            let neighbors = [
                (r.wrapping_sub(1), c),
                (r + 1, c),
                (r, c.wrapping_sub(1)),
                (r, c + 1),
            ];
            for (nr, nc) in neighbors {
                if nr < rows && nc < cols && board[nr][nc] == 'O' {
                    board[nr][nc] = '#';
                    stack.push(nr * cols + nc);
                }
            }
        }
        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == 'O' {
                    *cell = 'X';
                } else if *cell == '#' {
                    *cell = 'O';
                }
            }
        }
    }
}