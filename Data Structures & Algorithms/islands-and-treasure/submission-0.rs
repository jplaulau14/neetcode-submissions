use std::collections::VecDeque;

impl Solution {
    pub fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
        const INF: i32 = i32::MAX;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 0 {
                    queue.push_back((r, c));
                }
            }
        }
        while let Some((row, col)) = queue.pop_front() {
            let next_dist = grid[row][col] + 1;
            for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let nr = row as i32 + dr;
                let nc = col as i32 + dc;
                if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] == INF {
                    grid[nr][nc] = next_dist;
                    queue.push_back((nr, nc));
                }
            }
        }
    }
}