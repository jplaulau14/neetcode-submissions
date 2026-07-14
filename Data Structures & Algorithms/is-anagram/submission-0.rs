impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count = vec![0; 26];

        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        for i in 0..s.len() {
            count[(s_bytes[i] - b'a') as usize] += 1;
            count[(t_bytes[i] - b'a') as usize] -= 1;
        }

        for num in count {
            if num != 0 {
                return false;
            }
        }

        true
    }
}