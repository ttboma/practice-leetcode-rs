use super::Solution;

impl Solution {
    /// # [48. Rotate Image](https://leetcode.com/problems/rotate-image/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given an `n x n` 2D `matrix` representing an image, rotate the image by **90**  degrees (clockwise).
    ///
    /// You have to rotate the image <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">**in-place** </a>, which means you have to modify the input 2D matrix directly. **DO NOT**  allocate another 2D matrix and do the rotation.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/08/28/mat1.jpg" style="width: 500px; height: 188px;">
    ///
    /// ```txt
    /// Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
    /// Output: [[7,4,1],[8,5,2],[9,6,3]]
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/08/28/mat2.jpg" style="width: 500px; height: 201px;">
    ///
    /// ```txt
    /// Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
    /// Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `n == matrix.length == matrix[i].length`
    /// - `1 <= n <= 20`
    /// - `-1000 <= matrix[i][j] <= 1000`
    pub fn rotate_image(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            for j in 0..n - 2 * i - 1 {
                let (mut a, mut b) = (i, i + j);
                let mut tmp = matrix[n - b - 1][a];
                for _ in 0..4 {
                    std::mem::swap(&mut matrix[a][b], &mut tmp);
                    (a, b) = (b, n - a - 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate_image(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }
    #[test]
    fn example2() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate_image(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }
}
