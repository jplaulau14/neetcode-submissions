impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1i64;
        let mut right = *piles.iter().max().unwrap() as i64;

        while left < right {
            let speed = left + (right - left) / 2;
            let hours: i64 = piles
                .iter()
                .map(|&pile| (pile as i64 + speed - 1) / speed)
                .sum();

            if hours <= h as i64 {
                right = speed;
            } else {
                left = speed + 1;
            }
        }

        left as i32
    }
}