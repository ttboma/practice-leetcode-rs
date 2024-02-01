use super::*;
use std::{collections::HashSet, ops::Rem};

impl Solution {
    /// # [202. Happy Number](https://leetcode.com/problems/happy-number/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Write an algorithm to determine if a number `n` is happy.
    ///
    /// A **happy number**  is a number defined by the following process:
    ///
    /// - Starting with any positive integer, replace the number by the sum of the squares of its digits.
    /// - Repeat the process until the number equals 1 (where it will stay), or it **loops endlessly in a cycle**  which does not include 1.
    /// - Those numbers for which this process **ends in 1**  are happy.
    ///
    /// Return `true` if `n` is a happy number, and `false` if not.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: n = 19
    /// Output: true
    /// Explanation:
    /// 1^2 + 9^2 = 82
    /// 8^2 + 2^2 = 68
    /// 6^2 + 8^2 = 100
    /// 1^2 + 0^2 + 0^2 = 1
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 2
    /// Output: false
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= n <= 2^31 - 1`
    pub fn is_happy(mut n: i32) -> bool {
        if n == 1 {
            return true;
        }
        let mut map = HashSet::new();
        map.insert(n);
        while !n.is_happy() {
            if !map.insert(n) {
                return false;
            }
        }
        true
    }
}

trait Happy {
    fn is_happy(&mut self) -> bool;
}

impl Happy for i32 {
    fn is_happy(&mut self) -> bool {
        let mut n = 0;
        while *self != 0 {
            n += self.rem(10).pow(2);
            *self /= 10;
        }
        if n == 1 {
            true
        } else {
            *self = n;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_happy(19), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_happy(2), false);
    }
}
