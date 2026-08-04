use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut candidates = VecDeque::new();
        let mut answer = Vec::with_capacity(nums.len() - k + 1);

        for right in 0..nums.len() {
            while candidates.front().is_some_and(|&index| index + k <= right) {
                candidates.pop_front();
            }

            while candidates.back().is_some_and(|&index| nums[index] <= nums[right]) {
                candidates.pop_back();
            }

            candidates.push_back(right);

            if right + 1 >= k {
                answer.push(nums[*candidates.front().unwrap()]);
            }
        }

        answer
    }
}