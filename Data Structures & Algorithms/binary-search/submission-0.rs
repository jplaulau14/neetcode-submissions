impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0usize;
        let mut right = nums.len();

        while left < right {
            let middle = left + (right - left) / 2;

            if nums[middle] == target {
                return middle as i32;
            }

            if nums[middle] < target {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        -1
    }
}