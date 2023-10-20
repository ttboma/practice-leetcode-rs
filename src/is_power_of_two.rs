use crate::Solution;

impl Solution {
    /// # [231. Power of Two](https://leetcode.com/problems/power-of-two/)
    ///
    /// Given an integer `n`, return `true` if it is a power of two. Otherwise, return `false`.
    ///
    /// An integer `n` is a power of two, if there exists an integer `x` such that `n == 2^x`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: n = 1
    /// Output: true
    /// Explanation: 2^0 = 1
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 16
    /// Output: true
    /// Explanation: 2^4 = 16
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: n = 3
    /// Output: false
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `-2^31 <= n <= 2^31 - 1`
    ///
    /// **Follow up:**  Could you solve it without loops/recursion?
    pub fn is_power_of_two(n: i32) -> bool {
        if n == 0 {
            false
        } else if n == 1 {
            true
        } else if n % 2 == 1 {
            false
        } else {
            Self::is_power_of_two(n / 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_power_of_two(1), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_power_of_two(16), true);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::is_power_of_two(3), false);
    }
}
