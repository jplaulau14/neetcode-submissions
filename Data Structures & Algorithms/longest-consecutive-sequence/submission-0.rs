use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let values: HashSet<i32> = nums.into_iter().collect();
        let mut longest = 0;

        for &value in &values {
            if value != i32::MIN && values.contains(&(value - 1)) {
                continue;
            }

            let mut current = value;
            let mut length = 1;

            while current != i32::MAX && values.contains(&(current + 1)) {
                current += 1;
                length += 1;
            }

            longest = longest.max(length);
        }

        longest
    }
}
