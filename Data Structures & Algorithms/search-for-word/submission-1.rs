impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let rows = board.len();
        let cols = board[0].len();
        let word: Vec<char> = word.chars().collect();
        if word.len() > rows * cols {
            return false;
        }

        let mut board = board;

        fn dfs(board: &mut [Vec<char>], word: &[char], row: i32, col: i32, index: usize) -> bool {
            if index == word.len() {
                return true;
            }
            if row < 0
                || row >= board.len() as i32
                || col < 0
                || col >= board[0].len() as i32
            {
                return false;
            }

            let row = row as usize;
            let col = col as usize;
            if board[row][col] != word[index] {
                return false;
            }

            let original = board[row][col];
            board[row][col] = '#';
            let found = dfs(board, word, row as i32 - 1, col as i32, index + 1)
                || dfs(board, word, row as i32 + 1, col as i32, index + 1)
                || dfs(board, word, row as i32, col as i32 - 1, index + 1)
                || dfs(board, word, row as i32, col as i32 + 1, index + 1);
            board[row][col] = original;
            found
        }

        for row in 0..rows {
            for col in 0..cols {
                if dfs(&mut board, &word, row as i32, col as i32, 0) {
                    return true;
                }
            }
        }
        false
    }
}