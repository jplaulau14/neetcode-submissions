impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut next_start = [0usize; 256];
        let mut left = 0usize;
        let mut best = 0usize;

        for (right, byte) in s.bytes().enumerate() {
            let character = byte as usize;
            left = left.max(next_start[character]);
            next_start[character] = right + 1;
            best = best.max(right - left + 1);
        }

        best as i32
    }
}