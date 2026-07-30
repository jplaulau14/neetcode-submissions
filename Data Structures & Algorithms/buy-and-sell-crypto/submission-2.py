class Solution:
    def maxProfit(self, prices: list[int]) -> int:
        current_profit = 0
        best_profit = 0

        for day in range(1, len(prices)):
            daily_change = prices[day] - prices[day - 1]
            current_profit = max(
                0,
                current_profit + daily_change,
            )
            best_profit = max(best_profit, current_profit)

        return best_profit