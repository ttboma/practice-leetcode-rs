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
        n > 0 && (n - 1) & n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(Solution::is_power_of_two(1));
    }

    #[test]
    fn example2() {
        assert!(Solution::is_power_of_two(16));
    }

    #[test]
    fn example3() {
        assert!(!Solution::is_power_of_two(3));
    }

    #[test]
    fn example4() {
        assert!(!Solution::is_power_of_two(0));
    }
}
