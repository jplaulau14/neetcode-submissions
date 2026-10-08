impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut count = 0i32;

        let mut expand = |mut left: isize, mut right: isize| {
            while left >= 0
                && (right as usize) < n
                && bytes[left as usize] == bytes[right as usize]
            {
                count += 1;
                left -= 1;
                right += 1;
            }
        };

        for i in 0..n {
            expand(i as isize, i as isize);
            expand(i as isize, (i + 1) as isize);
        }
        count
    }
}