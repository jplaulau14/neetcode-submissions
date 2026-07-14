use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut frequencies: HashMap<i32, usize> = HashMap::new();

        for number in nums {
            *frequencies.entry(number).or_insert(0) += 1;
        }

        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); n + 1];

        for (number, frequency) in frequencies {
            buckets[frequency].push(number);
        }

        let mut result = Vec::with_capacity(k as usize);

        for frequency in (1..=n).rev() {
            for &number in &buckets[frequency] {
                result.push(number);

                if result.len() == k as usize {
                    return result;
                }
            }
        }

        result
    }
}