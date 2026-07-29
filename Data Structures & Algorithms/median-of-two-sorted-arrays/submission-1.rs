impl Solution {
    pub fn find_median_sorted_arrays(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
    ) -> f64 {
        let (short, long) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };

        let total_length = short.len() + long.len();
        let left_size = (total_length + 1) / 2;
        let mut low = 0isize;
        let mut high = short.len() as isize;

        while low <= high {
            let short_cut = (low + (high - low) / 2) as usize;
            let long_cut = left_size - short_cut;

            let short_left = if short_cut == 0 {
                i32::MIN
            } else {
                short[short_cut - 1]
            };
            let short_right = if short_cut == short.len() {
                i32::MAX
            } else {
                short[short_cut]
            };
            let long_left = if long_cut == 0 {
                i32::MIN
            } else {
                long[long_cut - 1]
            };
            let long_right = if long_cut == long.len() {
                i32::MAX
            } else {
                long[long_cut]
            };

            if short_left <= long_right && long_left <= short_right {
                let max_left = short_left.max(long_left);

                if total_length % 2 == 1 {
                    return max_left as f64;
                }

                let min_right = short_right.min(long_right);
                return (max_left as f64 + min_right as f64) / 2.0;
            }

            if short_left > long_right {
                high = short_cut as isize - 1;
            } else {
                low = short_cut as isize + 1;
            }
        }

        unreachable!()
    }
}