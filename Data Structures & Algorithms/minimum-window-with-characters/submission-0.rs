impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        if t_bytes.is_empty() || t_bytes.len() > s_bytes.len() {
            return String::new();
        }

        let mut required = [0i32; 128];
        let mut window = [0i32; 128];
        let mut needed_types = 0;

        for &byte in t_bytes {
            let index = byte as usize;
            if required[index] == 0 {
                needed_types += 1;
            }
            required[index] += 1;
        }

        let mut formed = 0;
        let mut left = 0;
        let mut best_start = 0;
        let mut best_length = s_bytes.len() + 1;

        for right in 0..s_bytes.len() {
            let index = s_bytes[right] as usize;
            window[index] += 1;

            if required[index] > 0 && window[index] == required[index] {
                formed += 1;
            }

            while formed == needed_types {
                let length = right - left + 1;
                if length < best_length {
                    best_start = left;
                    best_length = length;
                }

                let left_index = s_bytes[left] as usize;
                if required[left_index] > 0 && window[left_index] == required[left_index] {
                    formed -= 1;
                }
                window[left_index] -= 1;
                left += 1;
            }
        }

        if best_length == s_bytes.len() + 1 {
            String::new()
        } else {
            s[best_start..best_start + best_length].to_string()
        }
    }
}