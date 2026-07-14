impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];
        let mut prefix = 1;

        for i in 0..nums.len() {
            result[i] = prefix;
            prefix *= nums[i];
        }

        let mut suffix = 1;

        for i in (0..nums.len()).rev() {
            result[i] *= suffix;
            suffix *= nums[i];
        }

        result
    }
}