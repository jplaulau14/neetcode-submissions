impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = nums.len() - 1;

        while left < right {
            let middle = left + (right - left) / 2;

            if nums[middle] > nums[right] {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        nums[left]
    }
}