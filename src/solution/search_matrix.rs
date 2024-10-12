use crate::*;

impl Solution {
    /// # [74. Search a 2D Matrix](https://leetcode.com/problems/search-a-2d-matrix/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given an `m x n` integer matrix `matrix` with the following two properties:
    ///
    /// - Each row is sorted in non-decreasing order.
    /// - The first integer of each row is greater than the last integer of the previous row.
    ///
    /// Given an integer `target`, return `true` if `target` is in `matrix` or `false` otherwise.
    ///
    /// You must write a solution in `O(log(m * n))` time complexity.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/05/mat.jpg" style="width: 322px; height: 242px;">
    ///
    /// ```txt
    /// Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
    /// Output: true
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/05/mat2.jpg" style="width: 322px; height: 242px;">
    ///
    /// ```txt
    /// Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
    /// Output: false
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `m == matrix.length`
    /// - `n == matrix[i].length`
    /// - `1 <= m, n <= 100`
    /// - `-10^4 <= matrix[i][j], target <= 10^4`
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let i = Solution::binary_search_column(&matrix, target, 0, m);
        matrix
            .get(i)
            .and_then(|row| {
                if row[0] == target {
                    Some(true)
                } else {
                    i.checked_sub(1).map(|i| &matrix[i]).and_then(|prev_row| {
                        let j = Solution::binary_search_row(prev_row, target, 0, n);
                        prev_row.get(j).map(|&x| x == target)
                    })
                }
            })
            .or_else(|| {
                matrix.get(m - 1).and_then(|prev_row| {
                    let j = Solution::binary_search_row(prev_row, target, 0, n);
                    prev_row.get(j).map(|&x| x == target)
                })
            })
            .unwrap_or(false)
    }

    fn binary_search_column(
        matrix: &Vec<Vec<i32>>,
        target: i32,
        start: usize,
        end: usize,
    ) -> usize {
        if start == end {
            return start;
        }
        let mid = start + (end - start) / 2;
        match matrix[mid][0].cmp(&target) {
            std::cmp::Ordering::Equal => mid,
            std::cmp::Ordering::Less => {
                Solution::binary_search_column(matrix, target, mid + 1, end)
            }
            std::cmp::Ordering::Greater => {
                Solution::binary_search_column(matrix, target, start, mid)
            }
        }
    }

    fn binary_search_row(matrix: &Vec<i32>, target: i32, start: usize, end: usize) -> usize {
        if start == end {
            return start;
        }
        let mid = start + (end - start) / 2;
        match matrix[mid].cmp(&target) {
            std::cmp::Ordering::Equal => mid,
            std::cmp::Ordering::Less => Solution::binary_search_row(matrix, target, mid + 1, end),
            std::cmp::Ordering::Greater => Solution::binary_search_row(matrix, target, start, mid),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        let result = true;
        assert_eq!(Solution::search_matrix(matrix, target), result);
    }

    #[test]
    fn example2() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;
        let result = false;
        assert_eq!(Solution::search_matrix(matrix, target), result);
    }

    #[test]
    fn example3() {
        let matrix = vec![vec![1, 3]];
        let target = 3;
        let result = true;
        assert_eq!(Solution::search_matrix(matrix, target), result);
    }

    #[test]
    fn example4() {
        let matrix = vec![vec![1], vec![3]];
        let target = 0;
        let result = false;
        assert_eq!(Solution::search_matrix(matrix, target), result);
    }
}
