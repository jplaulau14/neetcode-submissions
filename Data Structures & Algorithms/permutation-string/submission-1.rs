impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let window_size = s1.len();

        if window_size > s2.len() {
            return false;
        }

        let mut required = [0i32; 26];
        let mut window = [0i32; 26];

        for i in 0..window_size {
            required[(s1[i] - b'a') as usize] += 1;
            window[(s2[i] - b'a') as usize] += 1;
        }

        if window == required {
            return true;
        }

        for right in window_size..s2.len() {
            window[(s2[right] - b'a') as usize] += 1;
            window[(s2[right - window_size] - b'a') as usize] -= 1;

            if window == required {
                return true;
            }
        }

        false
    }
}