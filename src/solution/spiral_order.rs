use super::Solution;

impl Solution {
    /// # [54. Spiral Matrix](https://leetcode.com/problems/spiral-matrix/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an `m x n` `matrix`, return all elements of the `matrix` in spiral order.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiral1.jpg" style="width: 242px; height: 242px;">
    ///
    /// ```txt
    /// Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
    /// Output: [1,2,3,6,9,8,7,4,5]
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiral.jpg" style="width: 322px; height: 242px;">
    ///
    /// ```txt
    /// Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
    /// Output: [1,2,3,4,8,12,11,10,9,5,6,7]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `m == matrix.length`
    /// - `n == matrix[i].length`
    /// - `1 <= m, n <= 10`
    /// - `-100 <= matrix[i][j] <= 100`
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = Vec::new();
        let (m, n) = (matrix.len(), matrix[0].len());
        for k in 0..(m.min(n) + 1) / 2 {
            ret.extend_from_slice(&matrix[k][k..n - k]);
            ret.extend((1 + k..m - k - 1).map(|i| matrix[i][n - k - 1]));
            if k >= m - k - 1 {
                break;
            }
            ret.extend(matrix[m - k - 1][k..n - k].iter().rev());
            if k >= n - k - 1 {
                break;
            }
            ret.extend((1 + k..m - k - 1).rev().map(|i| matrix[i][k]));
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::spiral_order(vec![vec![7], vec![9], vec![6]]),
            vec![7, 9, 6]
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            Solution::spiral_order(vec![vec![2, 5, 8], vec![4, 0, -1],]),
            vec![2, 5, 8, -1, 0, 4]
        );
    }

    #[test]
    fn example5() {
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 11],
                vec![2, 12],
                vec![3, 13],
                vec![4, 14],
                vec![5, 15],
                vec![6, 16],
                vec![7, 17],
                vec![8, 18],
                vec![9, 19],
                vec![10, 20],
            ]),
            vec![1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 10, 9, 8, 7, 6, 5, 4, 3, 2]
        );
    }
}
