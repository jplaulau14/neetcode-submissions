impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut result = Vec::new();
        let mut board = vec![vec!['.'; n]; n];
        let mut columns = vec![false; n];
        let mut diagonals_down = vec![false; 2 * n - 1];
        let mut diagonals_up = vec![false; 2 * n - 1];

        fn dfs(
            row: usize,
            n: usize,
            board: &mut [Vec<char>],
            columns: &mut [bool],
            diagonals_down: &mut [bool],
            diagonals_up: &mut [bool],
            result: &mut Vec<Vec<String>>,
        ) {
            if row == n {
                result.push(
                    board
                        .iter()
                        .map(|board_row| board_row.iter().collect())
                        .collect(),
                );
                return;
            }
            for col in 0..n {
                let diagonal_down = row + n - 1 - col;
                let diagonal_up = row + col;
                if columns[col]
                    || diagonals_down[diagonal_down]
                    || diagonals_up[diagonal_up]
                {
                    continue;
                }
                columns[col] = true;
                diagonals_down[diagonal_down] = true;
                diagonals_up[diagonal_up] = true;
                board[row][col] = 'Q';
                dfs(
                    row + 1,
                    n,
                    board,
                    columns,
                    diagonals_down,
                    diagonals_up,
                    result,
                );
                board[row][col] = '.';
                diagonals_up[diagonal_up] = false;
                diagonals_down[diagonal_down] = false;
                columns[col] = false;
            }
        }

        dfs(
            0,
            n,
            &mut board,
            &mut columns,
            &mut diagonals_down,
            &mut diagonals_up,
            &mut result,
        );
        result
    }
}