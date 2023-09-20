use std::iter::once;

use crate::Solution;

impl Solution {
    /// # [119. Pascal's Triangle II](https://leetcode.com/problems/pascals-triangle-ii/)
    ///
    /// Given an integer `rowIndex`, return the `rowIndex^th` (**0-indexed** ) row of the **Pascal's triangle** .
    ///
    /// In **Pascal's triangle** , each number is the sum of the two numbers directly above it as shown:
    /// <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" style="height: 240px; width: 260px;">
    ///
    /// **Example 1:**
    ///
    /// ```
    /// Input: rowIndex = 3
    /// Output: [1,3,3,1]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```
    /// Input: rowIndex = 0
    /// Output: [1]
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```
    /// Input: rowIndex = 1
    /// Output: [1,1]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `0 <= rowIndex <= 33`
    ///
    /// **Follow up:**  Could you optimize your algorithm to use only `O(rowIndex)` extra space?
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = vec![1];
        for _ in 0..row_index {
            row = compute_next_row(&row);
        }
        row
    }
}

fn compute_next_row(row: &Vec<i32>) -> Vec<i32> {
    once(1)
        .chain(row.windows(2).map(|item| item[0] + item[1]))
        .chain(once(1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::get_row(0), vec![1]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::get_row(1), vec![1, 1]);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::get_row(2), vec![1, 2, 1]);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }
}
