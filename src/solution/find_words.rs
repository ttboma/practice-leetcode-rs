use crate::*;

impl Solution {
    /// # [212. Word Search II](https://leetcode.com/problems/word-search-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an `m x n` `board`of characters and a list of strings `words`, return all words on the board.
    ///
    /// Each word must be constructed from letters of sequentially adjacent cells, where **adjacent cells**  are horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/11/07/search1.jpg" style="width: 322px; height: 322px;">
    ///
    /// ```txt
    /// Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]
    /// Output: ["eat","oath"]
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/11/07/search2.jpg" style="width: 162px; height: 162px;">
    ///
    /// ```txt
    /// Input: board = [["a","b"],["c","d"]], words = ["abcb"]
    /// Output: []
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `m == board.length`
    /// - `n == board[i].length`
    /// - `1 <= m, n <= 12`
    /// - `board[i][j]` is a lowercase English letter.
    /// - `1 <= words.length <= 3 * 10^4`
    /// - `1 <= words[i].length <= 10`
    /// - `words[i]` consists of lowercase English letters.
    /// - All the strings of `words` are unique.
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::default();
        for word in words {
            trie.insert(word);
        }

        trie.board_dfs_pruned_by_searching_trie(&mut board)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = ["oath", "pea", "eat", "rain"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected: Vec<String> = ["oath", "eat"].iter().map(|s| s.to_string()).collect();
        let ans = Solution::find_words(board, words);
        assert_eq!(ans, expected);
    }

    #[test]
    fn example2() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let words = ["abcb"].iter().map(|s| s.to_string()).collect();
        let expected: Vec<String> = vec![];
        assert_eq!(Solution::find_words(board, words), expected);
    }
}
