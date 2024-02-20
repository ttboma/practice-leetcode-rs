use super::*;

impl Solution {
    /// # [79. Word Search](https://leetcode.com/problems/word-search/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an `m x n` grid of characters `board` and a string `word`, return `true` if `word` exists in the grid.
    ///
    /// The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/word2.jpg" style="width: 322px; height: 242px;">
    ///
    /// ```txt
    /// Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
    /// Output: true
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/word-1.jpg" style="width: 322px; height: 242px;">
    ///
    /// ```txt
    /// Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
    /// Output: true
    /// ```
    ///
    /// **Example 3:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/15/word3.jpg" style="width: 322px; height: 242px;">
    ///
    /// ```txt
    /// Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
    /// Output: false
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `m == board.length`
    /// - `n = board[i].length`
    /// - `1 <= m, n <= 6`
    /// - `1 <= word.length <= 15`
    /// - `board` and `word` consists of only lowercase and uppercase English letters.
    ///
    /// **Follow up:**  Could you use search pruning to make your solution faster with a larger `board`?
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.as_bytes();
        let mut exist = Exist {
            visited: vec![vec![false; board.len()]; board[0].len()],
            idx: 0,
            board: &board,
            word,
        };
        for (i, row) in board.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c != word[0] as char {
                    continue;
                }
                if exist.backtrace(i, j) {
                    return true;
                }
            }
        }
        false
    }
}

struct Exist<'a> {
    visited: Vec<Vec<bool>>,
    idx: usize,
    board: &'a Vec<Vec<char>>,
    word: &'a [u8],
}

impl<'a> Exist<'a> {
    fn backtrace(&mut self, i: usize, j: usize) -> bool {
        self.idx += 1;
        if self.idx == self.word.len() {
            return true;
        }
        self.visited[i][j] = true;
        if (i != 0
            && !self.visited[i - 1][j]
            && self.board[i - 1][j] == self.word[self.idx] as char
            && self.backtrace(i - 1, j))
            || (j != 0
                && !self.visited[i][j - 1]
                && self.board[i][j - 1] == self.word[self.idx] as char
                && self.backtrace(i, j - 1))
            || (i + 1 < self.visited.len()
                && !self.visited[i + 1][j]
                && self.board[i + 1][j] == self.word[self.idx] as char
                && self.backtrace(i + 1, j))
            || (j + 1 < self.visited[i].len()
                && !self.visited[i][j + 1]
                && self.board[i][j + 1] == self.word[self.idx] as char
                && self.backtrace(i, j + 1))
        {
            true
        } else {
            self.visited[i][j] = false;
            self.idx -= 1;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                String::from("ABCCED")
            ),
            true
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                String::from("SEE")
            ),
            true
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                String::from("ABCB")
            ),
            false
        );
    }
}
