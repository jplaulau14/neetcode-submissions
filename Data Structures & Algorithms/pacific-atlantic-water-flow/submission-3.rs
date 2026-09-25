use std::collections::VecDeque;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = heights.len();
        let cols = heights[0].len();
        let reach = |starts: Vec<(usize, usize)>| -> Vec<Vec<bool>> {
            let mut seen = vec![vec![false; cols]; rows];
            let mut queue = VecDeque::new();
            for (r, c) in starts {
                if !seen[r][c] {
                    seen[r][c] = true;
                    queue.push_back((r, c));
                }
            }
            while let Some((row, col)) = queue.pop_front() {
                for (nr, nc) in [(row.wrapping_sub(1), col), (row + 1, col), (row, col.wrapping_sub(1)), (row, col + 1)] {
                    if nr < rows && nc < cols && !seen[nr][nc] && heights[nr][nc] >= heights[row][col] {
                        seen[nr][nc] = true;
                        queue.push_back((nr, nc));
                    }
                }
            }
            seen
        };
        let pacific = reach((0..cols).map(|c| (0, c)).chain((0..rows).map(|r| (r, 0))).collect());
        let atlantic = reach((0..cols).map(|c| (rows - 1, c)).chain((0..rows).map(|r| (r, cols - 1))).collect());
        let mut result = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                if pacific[r][c] && atlantic[r][c] {
                    result.push(vec![r as i32, c as i32]);
                }
            }
        }
        result
    }
}