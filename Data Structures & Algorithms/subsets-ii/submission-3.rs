impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result = Vec::with_capacity(1usize << nums.len());
        let mut path = Vec::with_capacity(nums.len());

        fn dfs(
            nums: &[i32],
            start: usize,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            result.push(path.clone());
            for i in start..nums.len() {
                if i > start && nums[i] == nums[i - 1] {
                    continue;
                }
                path.push(nums[i]);
                dfs(nums, i + 1, path, result);
                path.pop();
            }
        }

        dfs(&nums, 0, &mut path, &mut result);
        result
    }
}