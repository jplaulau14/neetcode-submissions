impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![0; temperatures.len()];
        let mut unresolved = Vec::with_capacity(temperatures.len());

        for (day, &temperature) in temperatures.iter().enumerate() {
            while let Some(&earlier_day) = unresolved.last() {
                if temperatures[earlier_day] >= temperature {
                    break;
                }

                unresolved.pop();
                answer[earlier_day] = (day - earlier_day) as i32;
            }

            unresolved.push(day);
        }

        answer
    }
}