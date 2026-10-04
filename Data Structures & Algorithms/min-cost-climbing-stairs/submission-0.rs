impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut curr = 0;
        for i in 2..=cost.len() {
            let next = (curr + cost[i - 1]).min(prev + cost[i - 2]);
            prev = curr;
            curr = next;
        }
        curr
    }
}