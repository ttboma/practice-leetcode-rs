use crate::Solution;

impl Solution {
    /// # [779. K-th Symbol in Grammar](https://leetcode.com/problems/k-th-symbol-in-grammar/)
    ///
    /// We build a table of `n` rows (**1-indexed** ). We start by writing `0` in the `1^st` row. Now in every subsequent row, we look at the previous row and replace each occurrence of `0` with `01`, and each occurrence of `1` with `10`.
    ///
    /// - For example, for `n = 3`, the `1^st` row is `0`, the `2^nd` row is `01`, and the `3^rd` row is `0110`.
    ///
    /// Given two integer `n` and `k`, return the `k^th` (**1-indexed** ) symbol in the `n^th` row of a table of `n` rows.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: n = 1, k = 1
    /// Output: 0
    /// Explanation: row 1: 0
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 2, k = 1
    /// Output: 0
    /// Explanation:
    /// row 1: 0
    /// row 2: 01
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: n = 2, k = 2
    /// Output: 1
    /// Explanation:
    /// row 1: 0
    /// row 2: 01
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= n <= 30`
    /// - `1 <= k <= 2^n - 1`
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        let v = Self::kth_grammar(n - 1, (k + 1) / 2);
        if k % 2 == 0 {
            if v == 0 {
                1
            } else {
                0
            }
        } else {
            v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
        assert_eq!(Solution::kth_grammar(5, 10), 0);
    }
}
