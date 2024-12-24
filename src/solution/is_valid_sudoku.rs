use super::Solution;
use std::collections::HashSet;

impl Solution {
    /// # [36. Valid Sudoku](https://leetcode.com/problems/valid-sudoku/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Determine if a`9 x 9` Sudoku board is valid.Only the filled cells need to be validated**according to the following rules** :
    ///
    /// - Each row must contain the digits`1-9` without repetition.
    /// - Each column must contain the digits`1-9`without repetition.
    /// - Each of the nine`3 x 3` sub-boxes of the grid must contain the digits`1-9`without repetition.
    ///
    /// **Note:**
    ///
    /// - A Sudoku board (partially filled) could be valid but is not necessarily solvable.
    /// - Only the filled cells need to be validated according to the mentioned rules.
    ///
    /// **Example 1:**
    /// <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png" style="height: 250px; width: 250px;">
    ///
    /// ```txt
    /// Input: board =
    /// [["5","3",".",".","7",".",".",".","."]
    /// ,["6",".",".","1","9","5",".",".","."]
    /// ,[".","9","8",".",".",".",".","6","."]
    /// ,["8",".",".",".","6",".",".",".","3"]
    /// ,["4",".",".","8",".","3",".",".","1"]
    /// ,["7",".",".",".","2",".",".",".","6"]
    /// ,[".","6",".",".",".",".","2","8","."]
    /// ,[".",".",".","4","1","9",".",".","5"]
    /// ,[".",".",".",".","8",".",".","7","9"]]
    /// Output: true
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: board =
    /// [["8","3",".",".","7",".",".",".","."]
    /// ,["6",".",".","1","9","5",".",".","."]
    /// ,[".","9","8",".",".",".",".","6","."]
    /// ,["8",".",".",".","6",".",".",".","3"]
    /// ,["4",".",".","8",".","3",".",".","1"]
    /// ,["7",".",".",".","2",".",".",".","6"]
    /// ,[".","6",".",".",".",".","2","8","."]
    /// ,[".",".",".","4","1","9",".",".","5"]
    /// ,[".",".",".",".","8",".",".","7","9"]]
    /// Output: false
    /// Explanation: Same as Example 1, except with the **5**  in the top left corner being modified to **8** . Since there are two 8's in the top left 3x3 sub-box, it is invalid.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `board.length == 9`
    /// - `board[i].length == 9`
    /// - `board[i][j]` is a digit `1-9` or `'.'`.
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for row in board.iter() {
            let mut row_set: HashSet<char> = HashSet::new();
            for c in row.iter().filter(|c| **c != '.') {
                if !row_set.insert(*c) {
                    return false;
                }
            }
        }
        for col in 0..9 {
            let mut col_set: HashSet<char> = HashSet::new();
            for c in board.iter().filter_map(|row| {
                if row[col] != '.' {
                    Some(row[col])
                } else {
                    None
                }
            }) {
                if !col_set.insert(c) {
                    return false;
                }
            }
        }
        for k in 0..9 {
            let mut square_set: HashSet<char> = HashSet::new();
            let row_idx = (k / 3) * 3;
            let col_idx = (k % 3) * 3;
            for i in 0..3 {
                for j in 0..3 {
                    let c = board[row_idx + i][col_idx + j];
                    if c == '.' {
                        continue;
                    }
                    if !square_set.insert(c) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(Solution::is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]));
    }

    #[test]
    fn example2() {
        assert!(!Solution::is_valid_sudoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]),);
    }
}
