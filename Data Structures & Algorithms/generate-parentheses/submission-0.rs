impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut result = Vec::new();
        let mut path = String::with_capacity(2 * n);

        fn dfs(n: usize, open_count: usize, close_count: usize, path: &mut String, result: &mut Vec<String>) {
            if open_count == n && close_count == n {
                result.push(path.clone());
                return;
            }
            if open_count < n {
                path.push('(');
                dfs(n, open_count + 1, close_count, path, result);
                path.pop();
            }
            if close_count < open_count {
                path.push(')');
                dfs(n, open_count, close_count + 1, path, result);
                path.pop();
            }
        }

        dfs(n, 0, 0, &mut path, &mut result);
        result
    }
}