use crate::Solution;

impl Solution {
    /// # 509. Fibonacci Number
    ///
    /// The Fibonacci numbers, commonly denoted F(n) form a sequence,
    /// called the Fibonacci sequence, such that each number is the
    /// sum of the two preceding ones, starting from 0 and 1. That is,
    ///
    /// - F(0) = 0, F(1) = 1
    /// - F(n) = F(n - 1) + F(n - 2), for n > 1.
    /// - Given n, calculate F(n).
    ///
    /// **Example 1:**
    ///
    /// - **Input:** n = 2
    /// - **Output:** 1
    /// - **Explanation:** F(2) = F(1) + F(0) = 1 + 0 = 1.
    ///
    /// **Example 2:**
    ///
    /// - **Input:** n = 3
    /// - **Output:** 2
    /// - **Explanation:** F(3) = F(2) + F(1) = 1 + 1 = 2.
    ///
    /// **Example 3:**
    ///
    /// - **Input:** n = 4
    /// - **Output:** 3
    /// - **Explanation:** F(4) = F(3) + F(2) = 2 + 1 = 3.
    ///
    /// **Constraints:**
    ///
    /// - 0 <= n <= 30
    ///
    /// ------
    ///
    /// ***Extracted from:*** [fibonacci-number](https://leetcode.com/problems/fibonacci-number/)
    pub fn fib(n: i32) -> i32 {
        let mut state = (0, 1);
        let mut it = std::iter::from_fn(move || {
            state = (state.1, state.0 + state.1);
            Some(state.1)
        });
        match n {
            0 => 0,
            1 => 1,
            _ => it.nth((n - 2) as usize).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::fib(2), 1);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::fib(3), 2);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::fib(4), 3);
    }
}
