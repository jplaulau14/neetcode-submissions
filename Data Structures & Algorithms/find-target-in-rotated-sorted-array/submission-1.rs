impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0i32;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let middle = left + (right - left) / 2;
            let index = middle as usize;

            if nums[index] == target {
                return middle;
            }

            if nums[left as usize] <= nums[index] {
                if nums[left as usize] <= target && target < nums[index] {
                    right = middle - 1;
                } else {
                    left = middle + 1;
                }
            } else if nums[index] < target && target <= nums[right as usize] {
                left = middle + 1;
            } else {
                right = middle - 1;
            }
        }

        -1
    }
}