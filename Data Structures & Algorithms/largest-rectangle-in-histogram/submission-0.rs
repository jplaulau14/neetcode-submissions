impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut left = vec![0; n];
        let mut right = vec![n; n];
        let mut stack = Vec::with_capacity(n);

        for i in 0..n {
            while let Some(&index) = stack.last() {
                if heights[index] < heights[i] {
                    break;
                }

                stack.pop();
            }

            left[i] = stack.last().map_or(0, |&index| index + 1);
            stack.push(i);
        }

        stack.clear();

        for i in (0..n).rev() {
            while let Some(&index) = stack.last() {
                if heights[index] < heights[i] {
                    break;
                }

                stack.pop();
            }

            right[i] = stack.last().copied().unwrap_or(n);
            stack.push(i);
        }

        let mut best = 0;

        for i in 0..n {
            best = best.max(heights[i] * (right[i] - left[i]) as i32);
        }

        best
    }
}