use std::iter::successors;

use crate::Solution;

impl Solution {
    /// # [746. Min Cost Climbing Stairs](https://leetcode.com/problems/min-cost-climbing-stairs/)
    ///
    /// You are given an integer array `cost` where `cost[i]` is the cost of `i^th` step on a staircase. Once you pay the cost, you can either climb one or two steps.
    ///
    /// You can either start from the step with index `0`, or the step with index `1`.
    ///
    /// Return the minimum cost to reach the top of the floor.
    ///
    /// **Example 1:**
    ///
    /// ```
    /// Input: cost = [10,15,20]
    /// Output: 15
    /// Explanation: You will start at index 1.
    /// - Pay 15 and climb two steps to reach the top.
    /// The total cost is 15.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```
    /// Input: cost = [1,100,1,1,1,100,1,1,100,1]
    /// Output: 6
    /// Explanation: You will start at index 0.
    /// - Pay 1 and climb two steps to reach index 2.
    /// - Pay 1 and climb two steps to reach index 4.
    /// - Pay 1 and climb two steps to reach index 6.
    /// - Pay 1 and climb one step to reach index 7.
    /// - Pay 1 and climb two steps to reach index 9.
    /// - Pay 1 and climb one step to reach the top.
    /// The total cost is 6.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `2 <= cost.length <= 1000`
    /// - `0 <= cost[i] <= 999`
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut cost_iter = cost.windows(2);
        successors(Some([0, 0]), |item| {
            cost_iter
                .by_ref()
                .map(|c| [item[1], std::cmp::min(c[0] + item[0], c[1] + item[1])])
                .next()
        })
        .nth(cost.len() - 1)
        .unwrap()[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let cost = vec![10, 15, 20];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);
    }

    #[test]
    fn example2() {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
    }
}
