use crate::Solution;

impl Solution {
    /// # [1277. Count Square Submatrices with All Ones](https://leetcode.com/problems/count-square-submatrices-with-all-ones/)
    ///
    /// Given a `m * n` matrix of ones and zeros, return how many **square**  submatrices have all ones.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: matrix =
    /// [
    ///  [0,1,1,1],
    ///  [1,1,1,1],
    ///  [0,1,1,1]
    /// ]
    /// Output: 15
    /// Explanation:
    /// There are **10**  squares of side 1.
    /// There are **4**  squares of side 2.
    /// There is  **1**  square of side 3.
    /// Total number of squares = 10 + 4 + 1 = **15** .
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: matrix =
    /// [
    ///   [1,0,1],
    ///   [1,1,0],
    ///   [1,1,0]
    /// ]
    /// Output: 7
    /// Explanation:
    /// There are <b>6</b> squares of side 1.
    /// There is **1**  square of side 2.
    /// Total number of squares = 6 + 1 = <b>7</b>.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= arr.length<= 300`
    /// - `1 <= arr[0].length<= 300`
    /// - `0 <= arr[i][j] <= 1`
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut matrix = matrix;
        for i in 1..matrix.len() {
            for j in 1..matrix[0].len() {
                if matrix[i][j] == 0 {
                    continue;
                }
                let neighbor = [matrix[i - 1][j], matrix[i][j - 1], matrix[i - 1][j - 1]];
                matrix[i][j] += neighbor.iter().min().unwrap();
            }
        }
        matrix.into_iter().flatten().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let matrix = vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]];
        assert_eq!(Solution::count_squares(matrix), 15);
    }

    #[test]
    fn example2() {
        let matrix = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::count_squares(matrix), 7);
    }
}
