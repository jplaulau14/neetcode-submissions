impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut result = Vec::new();
        let mut path = String::new();

        fn dfs(index: usize, digits: &[u8], map: &[&str; 10], path: &mut String, result: &mut Vec<String>) {
            if index == digits.len() {
                result.push(path.clone());
                return;
            }
            let letters = map[(digits[index] - b'0') as usize];
            for letter in letters.bytes() {
                path.push(letter as char);
                dfs(index + 1, digits, map, path, result);
                path.pop();
            }
        }

        if !digits.is_empty() {
            dfs(0, digits.as_bytes(), &map, &mut path, &mut result);
        }
        result
    }
}