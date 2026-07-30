class Solution:
    def maxProfit(self, prices: list[int]) -> int:
        lowest_price = prices[0]
        best_profit = 0

        for day in range(1, len(prices)):
            price = prices[day]
            profit = price - lowest_price
            best_profit = max(best_profit, profit)
            lowest_price = min(lowest_price, price)

        return best_profit