use super::Solution;

impl Solution {
    /// # [73. Set Matrix Zeroes](https://leetcode.com/problems/set-matrix-zeroes/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an `m x n` integer matrix `matrix`, if an element is `0`, set its entire row and column to `0`'s.
    ///
    /// You must do it <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in place</a>.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/mat1.jpg" style="width: 450px; height: 169px;">
    ///
    /// ```txt
    /// Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
    /// Output: [[1,0,1],[0,0,0],[1,0,1]]
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/mat2.jpg" style="width: 450px; height: 137px;">
    ///
    /// ```txt
    /// Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
    /// Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `m == matrix.length`
    /// - `n == matrix[0].length`
    /// - `1 <= m, n <= 200`
    /// - `-2^31 <= matrix[i][j] <= 2^31 - 1`
    ///
    /// **Follow up:**
    ///
    /// - A straightforward solution using `O(mn)` space is probably a bad idea.
    /// - A simple improvement uses `O(m + n)` space, but still not the best solution.
    /// - Could you devise a constant space solution?
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        let first_row_zero = matrix[0].iter().any(|x| *x == 0);
        let first_col_zero = matrix.iter().any(|row| row[0] == 0);
        for row in matrix.iter_mut().skip(1) {
            if row.iter().any(|x| *x == 0) {
                row[0] = 0;
            };
        }
        for j in 1..n {
            if matrix.iter().any(|row| row[j] == 0) {
                matrix[0][j] = 0;
            };
        }

        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        if first_row_zero {
            matrix[0].iter_mut().for_each(|x| *x = 0);
        }
        if first_col_zero {
            matrix.iter_mut().for_each(|row| row[0] = 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }
    #[test]
    fn example2() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }
}
