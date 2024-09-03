use crate::Solution;

impl Solution {
    /// # [130. Surrounded Regions](https://leetcode.com/problems/surrounded-regions/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given an `m x n` matrix `board` containing **letters**  `'X'` and `'O'`, **capture regions**  that are **surrounded** :
    ///
    /// - **Connect** : A cell is connected to adjacent cells horizontally or vertically.
    /// - **Region** : To form a region **connect every**  `'O'` cell.
    /// - **Surround** : The region is surrounded with `'X'` cells if you can **connect the region ** with `'X'` cells and none of the region cells are on the edge of the `board`.
    ///
    /// A **surrounded region is captured**  by replacing all `'O'`s with `'X'`s in the input matrix `board`.
    ///
    /// **Example 1:**
    ///
    /// <div class="example-block">
    /// Input: board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
    ///
    /// Output: [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
    ///
    /// Explanation:
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/xogrid.jpg" style="width: 367px; height: 158px;">
    /// In the above diagram, the bottom region is not captured because it is on the edge of the board and cannot be surrounded.
    ///
    /// **Example 2:**
    ///
    /// <div class="example-block">
    /// Input: board = [["X"]]
    ///
    /// Output: [["X"]]
    ///
    /// **Constraints:**
    ///
    /// - `m == board.length`
    /// - `n == board[i].length`
    /// - `1 <= m, n <= 200`
    /// - `board[i][j]` is `'X'` or `'O'`.
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board.first().unwrap().len();
        for i in 0..m {
            dfs(board, i, 0, m, n);
            dfs(board, i, n - 1, m, n);
        }
        for j in 1..n - 1 {
            dfs(board, 0, j, m, n);
            dfs(board, m - 1, j, m, n);
        }
        for row in board {
            for cell in row {
                if *cell == 'O' {
                    *cell = 'X';
                }
                if *cell == '#' {
                    *cell = 'O';
                }
            }
        }
    }
}

fn dfs(visited: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) {
    if visited[i][j] != 'O' {
        return;
    }
    visited[i][j] = '#';
    if i > 0 {
        dfs(visited, i - 1, j, m, n);
    }
    if i < m - 1 {
        dfs(visited, i + 1, j, m, n);
    }
    if j > 0 {
        dfs(visited, i, j - 1, m, n)
    }
    if j < n - 1 {
        dfs(visited, i, j + 1, m, n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );
    }

    #[test]
    fn example2() {
        let mut board = vec![vec!['X']];
        Solution::solve(&mut board);
        assert_eq!(board, vec![vec!['X']]);
    }
}
