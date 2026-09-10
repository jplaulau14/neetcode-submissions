impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn dfs(
            start: usize,
            bytes: &[u8],
            n: usize,
            pal: &[bool],
            path: &mut Vec<(usize, usize)>,
            result: &mut Vec<Vec<String>>,
        ) {
            if start == n {
                let row = path
                    .iter()
                    .map(|&(left, right)| std::str::from_utf8(&bytes[left..right]).unwrap().to_owned())
                    .collect();
                result.push(row);
                return;
            }

            for end in start..n {
                if !pal[start * n + end] {
                    continue;
                }
                path.push((start, end + 1));
                dfs(end + 1, bytes, n, pal, path, result);
                path.pop();
            }
        }

        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut pal = vec![false; n * n];
        for length in 1..=n {
            for start in 0..=n - length {
                let end = start + length - 1;
                pal[start * n + end] = bytes[start] == bytes[end]
                    && (length <= 2 || pal[(start + 1) * n + end - 1]);
            }
        }

        let mut result = Vec::new();
        let mut path = Vec::new();
        dfs(0, bytes, n, &pal, &mut path, &mut result);
        result
    }
}