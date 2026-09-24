use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut fresh = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 2 {
                    queue.push_back((r, c));
                } else if grid[r][c] == 1 {
                    fresh += 1;
                }
            }
        }
        let mut minutes = 0;
        while !queue.is_empty() && fresh > 0 {
            for _ in 0..queue.len() {
                let (row, col) = queue.pop_front().unwrap();
                for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                    let nr = row as i32 + dr;
                    let nc = col as i32 + dc;
                    if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] == 1 {
                        grid[nr][nc] = 2;
                        fresh -= 1;
                        queue.push_back((nr, nc));
                    }
                }
            }
            minutes += 1;
        }
        if fresh == 0 { minutes } else { -1 }
    }
}