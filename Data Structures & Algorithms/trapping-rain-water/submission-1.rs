impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = height.len() - 1;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut water = 0;

        while left < right {
            if height[left] <= height[right] {
                left_max = left_max.max(height[left]);
                water += left_max - height[left];
                left += 1;
            } else {
                right_max = right_max.max(height[right]);
                water += right_max - height[right];
                right -= 1;
            }
        }

        water
    }
}