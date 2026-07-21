impl Solution {
    pub fn car_fleet(
        target: i32,
        position: Vec<i32>,
        speed: Vec<i32>,
    ) -> i32 {
        let mut cars: Vec<(i32, i32)> =
            position.into_iter().zip(speed).collect();
        cars.sort_unstable_by(|left, right| right.0.cmp(&left.0));

        let mut fleets = 0;
        let mut latest_distance = 0i64;
        let mut latest_speed = 1i64;

        for (car_position, car_speed) in cars {
            let distance = (target - car_position) as i64;
            let car_speed = car_speed as i64;

            if fleets == 0
                || distance * latest_speed
                    > latest_distance * car_speed
            {
                fleets += 1;
                latest_distance = distance;
                latest_speed = car_speed;
            }
        }

        fleets
    }
}