impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut islands = 0;
        let mut stack: Vec<(usize, usize)> = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '1' {
                    continue;
                }
                islands += 1;
                grid[r][c] = '0';
                stack.push((r, c));
                while let Some((row, col)) = stack.pop() {
                    for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                        let nr = row as i32 + dr;
                        let nc = col as i32 + dc;
                        if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                            continue;
                        }
                        let (nr, nc) = (nr as usize, nc as usize);
                        if grid[nr][nc] == '1' {
                            grid[nr][nc] = '0';
                            stack.push((nr, nc));
                        }
                    }
                }
            }
        }
        islands
    }
}