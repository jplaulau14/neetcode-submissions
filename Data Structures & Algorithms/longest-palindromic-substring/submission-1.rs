impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut best_l = 0usize;
        let mut best_r = 0usize;

        let mut expand = |mut left: isize, mut right: isize| {
            while left >= 0
                && (right as usize) < n
                && bytes[left as usize] == bytes[right as usize]
            {
                if (right - left) as usize > best_r - best_l {
                    best_l = left as usize;
                    best_r = right as usize;
                }
                left -= 1;
                right += 1;
            }
        };

        for i in 0..n {
            expand(i as isize, i as isize);
            expand(i as isize, (i + 1) as isize);
        }
        s[best_l..=best_r].to_string()
    }
}