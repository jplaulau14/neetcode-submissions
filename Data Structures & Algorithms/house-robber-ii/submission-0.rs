impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        let linear = |lo: usize, hi: usize| -> i32 {
            let mut prev = 0;
            let mut curr = 0;
            for &money in &nums[lo..hi] {
                let next = curr.max(prev + money);
                prev = curr;
                curr = next;
            }
            curr
        };
        linear(0, n - 1).max(linear(1, n))
    }
}