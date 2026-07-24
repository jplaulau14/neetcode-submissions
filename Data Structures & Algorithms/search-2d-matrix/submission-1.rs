impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }

        let mut top = 0i32;
        let mut bottom = matrix.len() as i32 - 1;

        while top <= bottom {
            let row = top + (bottom - top) / 2;
            let values = &matrix[row as usize];

            if target < values[0] {
                bottom = row - 1;
            } else if target > values[values.len() - 1] {
                top = row + 1;
            } else {
                let mut left = 0i32;
                let mut right = values.len() as i32 - 1;

                while left <= right {
                    let column = left + (right - left) / 2;
                    let value = values[column as usize];

                    if value == target {
                        return true;
                    }

                    if value < target {
                        left = column + 1;
                    } else {
                        right = column - 1;
                    }
                }

                return false;
            }
        }

        false
    }
}