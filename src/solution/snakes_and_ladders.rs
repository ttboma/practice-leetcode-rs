use std::collections::VecDeque;

use crate::Solution;

impl Solution {
    /// # [909. Snakes and Ladders](https://leetcode.com/problems/snakes-and-ladders/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given an `n x n` integer matrix `board` where the cells are labeled from `1` to `n^2` in a <a href="https://en.wikipedia.org/wiki/Boustrophedon" target="_blank">**Boustrophedon style** </a> starting from the bottom left of the board (i.e. `board[n - 1][0]`) and alternating direction each row.
    ///
    /// You start on square `1` of the board. In each move, starting from square `curr`, do the following:
    ///
    /// - Choose a destination square `next` with a label in the range `[curr + 1, min(curr + 6, n^2)]`.
    ///
    /// - This choice simulates the result of a standard **6-sided die roll** : i.e., there are always at most 6 destinations, regardless of the size of the board.
    ///
    /// - If `next` has a snake or ladder, you **must**  move to the destination of that snake or ladder. Otherwise, you move to `next`.
    /// - The game ends when you reach the square `n^2`.
    ///
    /// A board square on row `r` and column `c` has a snake or ladder if `board[r][c] != -1`. The destination of that snake or ladder is `board[r][c]`. Squares `1` and `n^2` are not the starting points of any snake or ladder.
    ///
    /// Note that you only take a snake or ladder at most once per move. If the destination to a snake or ladder is the start of another snake or ladder, you do **not**  follow the subsequentsnake or ladder.
    ///
    /// - For example, suppose the board is `[[-1,4],[-1,3]]`, and on the first move, your destination square is `2`. You follow the ladder to square `3`, but do **not**  follow the subsequent ladder to `4`.
    ///
    /// Return the least number of moves required to reach the square `n^2`. If it is not possible to reach the square, return `-1`.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2018/09/23/snakes.png" style="width: 500px; height: 394px;">
    ///
    /// ```txt
    /// Input: board = [[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,35,-1,-1,13,-1],[-1,-1,-1,-1,-1,-1],[-1,15,-1,-1,-1,-1]]
    /// Output: 4
    /// Explanation:
    /// In the beginning, you start at square 1 (at row 5, column 0).
    /// You decide to move to square 2 and must take the ladder to square 15.
    /// You then decide to move to square 17 and must take the snake to square 13.
    /// You then decide to move to square 14 and must take the ladder to square 35.
    /// You then decide to move to square 36, ending the game.
    /// This is the lowest possible number of moves to reach the last square, so return 4.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: board = [[-1,-1],[-1,3]]
    /// Output: 1
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `n == board.length == board[i].length`
    /// - `2 <= n <= 20`
    /// - `board[i][j]` is either `-1` or in the range `[1, n^2]`.
    /// - The squares labeled `1` and `n^2` are not the starting points of any snake or ladder.
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let m = n * n;
        let mut graph = vec![-1; m];
        let mut color = vec![0; m];
        let mut dist = vec![0; m];
        let mut queue = VecDeque::new();
        let mut flip = n % 2 == 0;
        let mut idx = m;
        for row in board {
            if flip {
                for ele in row {
                    idx -= 1;
                    if ele != -1 {
                        graph[idx] = ele - 1;
                    }
                }
            } else {
                for ele in row.iter().rev() {
                    idx -= 1;
                    if *ele != -1 {
                        graph[idx] = ele - 1;
                    }
                }
            }
            flip = !flip;
        }

        let start = if graph[0] == -1 { 0 } else { graph[0] as usize };
        color[start] = 1;
        queue.push_back(start);
        while let Some(idx) = queue.pop_front() {
            if idx == m - 1 {
                return dist[idx];
            }
            for k in 1..=6 {
                let mut next = idx + k;
                if next >= m {
                    break;
                }
                if graph[next] != -1 {
                    next = graph[next] as usize;
                }
                if color[next] == 0 {
                    color[next] = 1;
                    queue.push_back(next);
                    dist[next] = dist[idx] + 1;
                }
            }
            color[idx] = 2;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let board = vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1],
        ];
        assert_eq!(Solution::snakes_and_ladders(board), 4);
    }

    #[test]
    fn example2() {
        let board = vec![vec![-1, -1], vec![-1, 3]];
        assert_eq!(Solution::snakes_and_ladders(board), 1);
    }

    #[test]
    fn example3() {
        let board = vec![vec![-1, -1, -1], vec![-1, 9, 8], vec![-1, 8, 9]];
        assert_eq!(Solution::snakes_and_ladders(board), 1);
    }

    #[test]
    fn example4() {
        let board = vec![
            vec![-1, -1, -1, -1, 48, 5, -1],
            vec![12, 29, 13, 9, -1, 2, 32],
            vec![-1, -1, 21, 7, -1, 12, 49],
            vec![42, 37, 21, 40, -1, 22, 12],
            vec![42, -1, 2, -1, -1, -1, 6],
            vec![39, -1, 35, -1, -1, 39, -1],
            vec![-1, 36, -1, -1, -1, -1, 5],
        ];
        assert_eq!(Solution::snakes_and_ladders(board), 3);
    }
}
