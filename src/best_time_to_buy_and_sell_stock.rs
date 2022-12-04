use crate::Solution;

impl Solution {
    /// # 121. Best Time to Buy and Sell Stock
    /// 
    /// You are given an array prices where prices[i] is the price of 
    /// a given stock on the ith day. You want to maximize your profit 
    /// by choosing a single day to buy one stock and choosing a 
    /// different day in the future to sell that stock. Return the 
    /// maximum profit you can achieve from this transaction. If you 
    /// cannot achieve any profit, return 0.
    /// 
    /// **Example 1:**
    /// 
    /// - **Input:** prices = [7,1,5,3,6,4]
    /// - **Output:** 5
    /// - **Explanation:** Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
    ///   Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
    ///
    /// **Example 2:**
    /// 
    /// - **Input:** prices = [7,6,4,3,1]
    /// - **Output:** 0
    /// - **Explanation:** In this case, no transactions are done and the max profit = 0.
    ///  
    /// 
    /// **Constraints:**
    /// 
    /// - 1 <= prices.length <= 105
    /// - 0 <= prices[i] <= 104
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut buy  = prices[0];
        let mut sell = prices[0];

        for i in prices.iter().skip(1) {
            if *i < buy {
                let profit = sell - buy;
                max = if profit > max {
                          profit 
                      } 
                      else {
                          max
                      };
                sell = *i;
                buy  = *i;
            }  
            else if *i > sell {
                sell = *i;
            }
        }
        let profit = sell - buy;
        if max < profit {
           profit 
        } 
        else {
            max
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let prices = vec![7,1,5,3,6,4];
        assert_eq!(Solution::max_profit(prices), 5);
    }
    #[test]
    fn example2() {
        let prices = vec![7,6,4,3,1];
        assert_eq!(Solution::max_profit(prices), 0);
    }
}
