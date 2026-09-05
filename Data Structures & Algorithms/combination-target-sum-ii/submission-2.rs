impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(
            values: &[i32],
            start: usize,
            remaining: i32,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if remaining == 0 {
                result.push(path.clone());
                return;
            }
            for i in start..values.len() {
                if i > start && values[i] == values[i - 1] {
                    continue;
                }
                let value = values[i];
                if value > remaining {
                    break;
                }
                path.push(value);
                dfs(values, i + 1, remaining - value, path, result);
                path.pop();
            }
        }

        candidates.sort_unstable();
        let mut result = Vec::new();
        let mut path = Vec::new();
        dfs(&candidates, 0, target, &mut path, &mut result);
        result
    }
}