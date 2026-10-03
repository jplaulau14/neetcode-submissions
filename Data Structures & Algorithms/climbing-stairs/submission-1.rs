impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut prev = 1;
        let mut curr = 1;
        for _ in 1..n {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        curr
    }
}