impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(
            nums: &[i32],
            index: usize,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if index == nums.len() {
                result.push(path.clone());
                return;
            }

            dfs(nums, index + 1, path, result);
            path.push(nums[index]);
            dfs(nums, index + 1, path, result);
            path.pop();
        }

        let mut result = Vec::with_capacity(1usize << nums.len());
        let mut path = Vec::with_capacity(nums.len());
        dfs(&nums, 0, &mut path, &mut result);
        result
    }
}