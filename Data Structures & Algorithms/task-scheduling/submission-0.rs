impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts = [0i32; 26];
        for task in tasks.iter() {
            counts[(*task as u8 - b'A') as usize] += 1;
        }
        let maximum = *counts.iter().max().unwrap();
        let leaders = counts.iter().filter(|&&count| count == maximum).count() as i32;
        let skeleton = (maximum - 1) * (n + 1) + leaders;
        (tasks.len() as i32).max(skeleton)
    }
}