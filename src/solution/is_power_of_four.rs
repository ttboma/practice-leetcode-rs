use crate::Solution;

impl Solution {
    /// # [342. Power of Four](https://leetcode.com/problems/power-of-four/)
    ///
    /// Given an integer `n`, return `true` if it is a power of four. Otherwise, return `false`.
    ///
    /// An integer `n` is a power of four, if there exists an integer `x` such that `n == 4^x`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: n = 16
    /// Output: true
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 5
    /// Output: false
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: n = 1
    /// Output: true
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `-2^31 <= n <= 2^31 - 1`
    ///
    /// **Follow up:**  Could you solve it without loops/recursion?
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0 && (n - 1).count_ones() % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_power_of_four(16), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_power_of_four(5), false);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::is_power_of_four(1), true);
    }
}
