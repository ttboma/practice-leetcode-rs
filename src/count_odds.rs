use crate::Solution;

impl Solution {
    /// # 1523. Count Odd Numbers in an Interval Range
    /// 
    /// Given two non-negative integers `low` and `high`. Return the count of
    /// odd numbers between `low` and `high` (inclusive).
    ///
    /// **Example 1:**
    /// 
    /// - **Input:** low = 3, high = 7
    /// - **Output:** 3
    /// - **Explanation:** The odd numbers between 3 and 7 are [3,5,7].
    ///
    /// **Example 2:**
    /// 
    /// - **Input:** low = 8, high = 10
    /// - **Output:** 1
    /// - **Explanation:** The odd numbers between 8 and 10 are [9].
    /// 
    /// **Constraints:**
    /// 
    /// - 0 <= low <= high <= 10^9
    ///
    /// ------
    ///
    /// ***Extracted from:*** [count-odd-numbers-in-an-interval-range](https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/)
    pub fn count_odds(low: i32, high: i32) -> i32 {
        ((high + 1) / 2) - (low / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::count_odds(3, 7), 3);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::count_odds(4, 8), 2);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::count_odds(3, 8), 3);
    }
    #[test]
    fn example4() {
        assert_eq!(Solution::count_odds(4, 9), 3);
    }
}
