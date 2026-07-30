impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut lowest_price = prices[0];
        let mut best_profit = 0;

        for &price in &prices[1..] {
            let profit = price - lowest_price;
            best_profit = best_profit.max(profit);
            lowest_price = lowest_price.min(price);
        }

        best_profit
    }
}