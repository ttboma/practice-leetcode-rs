use super::*;
impl Solution {
    /// # [52. N-Queens II](https://leetcode.com/problems/n-queens-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// The **n-queens**  puzzle is the problem of placing `n` queens on an `n x n` chessboard such that no two queens attack each other.
    ///
    /// Given an integer `n`, return the number of distinct solutions to the**n-queens puzzle** .
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/queens.jpg" style="width: 600px; height: 268px;">
    ///
    /// ```txt
    /// Input: n = 4
    /// Output: 2
    /// Explanation: There are two distinct solutions to the 4-queens puzzle as shown.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 1
    /// Output: 1
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= n <= 9`
    pub fn total_n_queens(n: i32) -> i32 {
        let mut x = TotalNQueens {
            n,
            result: 0,
            solution: vec![-1; n as usize],
        };
        x.backtrace(0);
        x.result
    }
}

struct TotalNQueens {
    n: i32,
    result: i32,
    solution: Vec<i32>,
}

impl TotalNQueens {
    fn backtrace(&mut self, row: i32) {
        if row == self.n {
            self.result += 1;
            return;
        }
        'outer: for col in 0..self.n {
            if self.solution[col as usize] != -1 {
                continue;
            }
            for i in 1..=row {
                let left = col - i;
                let right = col + i;
                if left >= 0 && self.solution[left as usize] == row - i {
                    continue 'outer;
                }
                if (right as usize) < self.solution.len()
                    && self.solution[right as usize] == row - i
                {
                    continue 'outer;
                }
            }
            self.solution[col as usize] = row;
            self.backtrace(row + 1);
            self.solution[col as usize] = -1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::total_n_queens(4), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::total_n_queens(1), 1);
    }
}
