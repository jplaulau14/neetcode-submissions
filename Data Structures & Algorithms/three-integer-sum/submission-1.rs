impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        if nums.len() < 3 {
            return result;
        }

        nums.sort_unstable();

        for first in 0..nums.len() - 2 {
            if nums[first] > 0 {
                break;
            }

            if first > 0 && nums[first] == nums[first - 1] {
                continue;
            }

            let mut left = first + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let total = nums[first] + nums[left] + nums[right];

                if total < 0 {
                    left += 1;
                } else if total > 0 {
                    right -= 1;
                } else {
                    result.push(vec![nums[first], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }

                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                }
            }
        }

        result
    }
}