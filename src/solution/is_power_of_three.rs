use crate::Solution;

impl Solution {
    /// # [326. Power of Three](https://leetcode.com/problems/power-of-three/)
    ///
    /// Given an integer `n`, return `true` if it is a power of three. Otherwise, return `false`.
    ///
    /// An integer `n` is a power of three, if there exists an integer `x` such that `n == 3^x`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: n = 27
    /// Output: true
    /// Explanation: 27 = 3^3
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 0
    /// Output: false
    /// Explanation: There is no x where 3^x = 0.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: n = -1
    /// Output: false
    /// Explanation: There is no x where 3^x = (-1).
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `-2^31 <= n <= 2^31 - 1`
    ///
    /// **Follow up:**  Could you solve it without loops/recursion?
    pub fn is_power_of_three(n: i32) -> bool {
        // 1162261467 is the largest power of three that can be represented as a 32-bit signed integer
        n > 0 && 1162261467 % n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_power_of_three(27), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_power_of_three(0), false);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::is_power_of_three(-1), false);
    }
}
