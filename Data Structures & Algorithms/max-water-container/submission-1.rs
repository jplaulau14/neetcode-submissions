impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = height.len() - 1;
        let mut best = 0;

        while left < right {
            let width = (right - left) as i32;

            if height[left] <= height[right] {
                best = best.max(height[left] * width);
                left += 1;
            } else {
                best = best.max(height[right] * width);
                right -= 1;
            }
        }

        best
    }
}