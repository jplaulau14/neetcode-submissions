impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let mut counts = [0usize; 26];
        let mut left = 0usize;
        let mut maximum_frequency = 0usize;

        for (right, &byte) in bytes.iter().enumerate() {
            let index = (byte - b'A') as usize;
            counts[index] += 1;
            maximum_frequency = maximum_frequency.max(counts[index]);

            if right - left + 1 - maximum_frequency > k as usize {
                counts[(bytes[left] - b'A') as usize] -= 1;
                left += 1;
            }
        }

        (bytes.len() - left) as i32
    }
}