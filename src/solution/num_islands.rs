use crate::Solution;

impl Solution {
    /// # [200. Number of Islands](https://leetcode.com/problems/number-of-islands/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an `m x n` 2D binary grid `grid` which represents a map of `'1'`s (land) and `'0'`s (water), return the number of islands.
    ///
    /// An **island**  is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: grid = [
    ///   ["1","1","1","1","0"],
    ///   ["1","1","0","1","0"],
    ///   ["1","1","0","0","0"],
    ///   ["0","0","0","0","0"]
    /// ]
    /// Output: 1
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: grid = [
    ///   ["1","1","0","0","0"],
    ///   ["1","1","0","0","0"],
    ///   ["0","0","1","0","0"],
    ///   ["0","0","0","1","1"]
    /// ]
    /// Output: 3
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `m == grid.length`
    /// - `n == grid[i].length`
    /// - `1 <= m, n <= 300`
    /// - `grid[i][j]` is `'0'` or `'1'`.
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut visited: Vec<Vec<Option<State>>> = grid
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| if c == '1' { Some(State::White) } else { None })
                    .collect()
            })
            .collect();

        let mut num_islands = 0;
        for i in 0..m {
            for j in 0..n {
                if visited[i][j] == Some(State::White) {
                    dfs_visit(&mut visited, i, j, m, n);
                    num_islands += 1;
                }
            }
        }

        num_islands
    }
}

fn dfs_visit(visited: &mut Vec<Vec<Option<State>>>, i: usize, j: usize, m: usize, n: usize) {
    visited[i][j] = Some(State::Gray);
    for (i, j) in neighbor(i, j, m, n) {
        if visited[i][j] == Some(State::White) {
            dfs_visit(visited, i, j, m, n);
        }
    }
    visited[i][j] = Some(State::Black);
}

fn neighbor(i: usize, j: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if i > 0 {
        neighbors.push((i - 1, j));
    }
    if i < m - 1 {
        neighbors.push((i + 1, j));
    }
    if j > 0 {
        neighbors.push((i, j - 1));
    }
    if j < n - 1 {
        neighbors.push((i, j + 1));
    }
    neighbors
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    White,
    Gray,
    Black,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn example2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }
}
