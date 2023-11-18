use crate::Solution;

impl Solution {
    /// # [121. Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given an array `prices` where `prices[i]` is the price of a given stock on the `i^th` day.
    ///
    /// You want to maximize your profit by choosing a **single day**  to buy one stock and choosing a **different day in the future**  to sell that stock.
    ///
    /// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return `0`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: prices = [7,1,5,3,6,4]
    /// Output: 5
    /// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
    /// Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: prices = [7,6,4,3,1]
    /// Output: 0
    /// Explanation: In this case, no transactions are done and the max profit = 0.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= prices.length <= 10^5`
    /// - `0 <= prices[i] <= 10^4`
    pub fn max_profit1(prices: Vec<i32>) -> i32 {
        let mut min_price = prices[0];
        let mut max_profit = 0;
        for price in prices.iter().skip(1) {
            min_price = min_price.min(*price);
            max_profit = max_profit.max(price - min_price);
        }
        max_profit
    }

    /// # [122. Best Time to Buy and Sell Stock II](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given an integer array `prices` where `prices[i]` is the price of a given stock on the `i^th` day.
    ///
    /// On each day, you may decide to buy and/or sell the stock. You can only hold **at most one**  share of the stock at any time. However, you can buy it then immediately sell it on the **same day** .
    ///
    /// Find and return the **maximum**  profit you can achieve.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: prices = [7,1,5,3,6,4]
    /// Output: 7
    /// Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
    /// Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
    /// Total profit is 4 + 3 = 7.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: prices = [1,2,3,4,5]
    /// Output: 4
    /// Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
    /// Total profit is 4.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: prices = [7,6,4,3,1]
    /// Output: 0
    /// Explanation: There is no way to make a positive profit, so we never buy the stock to achieve the maximum profit of 0.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= prices.length <= 3 * 10^4`
    /// - `0 <= prices[i] <= 10^4`
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        debug_assert!(!prices.is_empty());
        debug_assert!(prices.len() <= 3 * 10_000);

        (1..prices.len())
            .map(|i| (prices[i] - prices[i - 1]).max(0))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit1(prices), 5);
    }
    #[test]
    fn example1_2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit1(prices), 0);
    }

    #[test]
    fn example2_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit2(prices), 7);
    }

    #[test]
    fn example2_2() {
        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_profit2(prices), 4);
    }

    #[test]
    fn example2_3() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit2(prices), 0);
    }
}
