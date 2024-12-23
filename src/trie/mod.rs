use std::collections::HashMap;

/// # [208. Implement Trie (Prefix Tree)](https://leetcode.com/problems/implement-trie-prefix-tree/description/?envType=study-plan-v2&envId=top-interview-150)
///
/// A <a href="https://en.wikipedia.org/wiki/Trie" target="_blank">**trie** </a> (pronounced as "try") or **prefix tree**  is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.
///
/// Implement the Trie class:
///
/// - `Trie()` Initializes the trie object.
/// - `void insert(String word)` Inserts the string `word` into the trie.
/// - `boolean search(String word)` Returns `true` if the string `word` is in the trie (i.e., was inserted before), and `false` otherwise.
/// - `boolean startsWith(String prefix)` Returns `true` if there is a previously inserted string `word` that has the prefix `prefix`, and `false` otherwise.
///
/// **Example 1:**
///
/// ```txt
/// Input
///
/// ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
/// [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
/// Output
///
/// [null, null, true, false, true, null, true]
///
/// Explanation
///
/// Trie trie = new Trie();
/// trie.insert("apple");
/// trie.search("apple");   // return True
/// trie.search("app");     // return False
/// trie.startsWith("app"); // return True
/// trie.insert("app");
/// trie.search("app");     // return True
/// ```
///
/// **Constraints:**
///
/// - `1 <= word.length, prefix.length <= 2000`
/// - `word` and `prefix` consist only of lowercase English letters.
/// - At most `3 * 10^4` calls **in total**  will be made to `insert`, `search`, and `startsWith`.
pub struct Trie {
    root: usize,
    nodes: Vec<TrieNode>,
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: 0,
            nodes: vec![TrieNode::default()],
        }
    }

    pub fn insert(&mut self, word: String) {
        if word.is_empty() {
            return;
        }
        let mut idx = self.root;
        for ch in word.chars() {
            let new_idx = self.nodes.len();
            idx = *self.nodes[idx].children.entry(ch).or_insert(new_idx);
            if idx == new_idx {
                self.nodes.push(TrieNode::default());
            }
        }
        self.nodes[idx].data = word;
    }

    pub fn search(&self, word: String) -> bool {
        self.search_dispatch(word, SearchMode::Search)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.search_dispatch(prefix, SearchMode::StartsWith)
    }

    fn search_dispatch(&self, word: String, mode: SearchMode) -> bool {
        let mut idx = self.root;
        for ch in word.chars() {
            match self.nodes[idx].children.get(&ch) {
                Some(&new_idx) => idx = new_idx,
                None => return false,
            }
        }
        match mode {
            SearchMode::Search => !self.nodes[idx].data.is_empty(),
            SearchMode::StartsWith => true,
        }
    }

    pub fn iter(&self) -> TrieIter {
        TrieIter::new(self)
    }

    pub fn iter_mut(&mut self) -> TrieIterMut {
        TrieIterMut::new(self)
    }
}

pub struct TrieIter<'a> {
    trie: &'a Trie,
    stack: Vec<usize>,
}

impl<'a> TrieIter<'a> {
    pub fn new(trie: &'a Trie) -> Self {
        TrieIter {
            stack: vec![trie.root],
            trie,
        }
    }
}

impl<'a> Iterator for TrieIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|idx| {
            self.trie.nodes[idx].children.values().for_each(|&new_idx| {
                self.stack.push(new_idx);
            });
            self.trie.nodes[idx].data.as_str()
        })
    }
}

pub struct TrieIterMut<'a> {
    trie: &'a mut Trie,
    stack: Vec<usize>,
}

impl<'a> TrieIterMut<'a> {
    pub fn new(trie: &'a mut Trie) -> Self {
        TrieIterMut {
            stack: vec![trie.root],
            trie,
        }
    }

    fn next(&mut self) -> Option<&mut String> {
        match self.stack.pop() {
            Some(idx) => {
                self.trie.nodes[idx].children.values().for_each(|&new_idx| {
                    self.stack.push(new_idx);
                });
                Some(&mut self.trie.nodes[idx].data)
            }
            None => None,
        }
    }
}

impl<'a> Iterator for TrieIterMut<'a> {
    type Item = &'a mut String;

    fn next(&mut self) -> Option<Self::Item> {
        // Use unsafe to return a mutable reference
        self.next().map(|e| {
            let data_ptr: *mut String = e;
            unsafe { &mut *data_ptr }
        })
    }
}

pub trait FindWords {
    fn board_dfs_pruned_by_searching_trie(&mut self, board: &mut Vec<Vec<char>>) -> Vec<String>;
}

impl FindWords for Trie {
    fn board_dfs_pruned_by_searching_trie(&mut self, board: &mut Vec<Vec<char>>) -> Vec<String> {
        let mut result = vec![];
        let m = board.len();
        let n = board[0].len();
        let mut visited_str = vec![false; self.nodes.len()];

        for i in 0..m {
            for j in 0..n {
                self.board_dfs_pruned_by_searching_trie_helper(
                    i,
                    j,
                    board,
                    self.root,
                    &mut result,
                    &mut visited_str,
                );
            }
        }

        result
    }
}

impl Trie {
    fn board_dfs_pruned_by_searching_trie_helper(
        &mut self,
        i: usize,
        j: usize,
        board: &mut Vec<Vec<char>>,
        mut idx: usize,
        result: &mut Vec<String>,
        visited_str: &mut Vec<bool>,
    ) {
        let ch = board[i][j];

        if let Some(next_idx) = self.nodes[idx].children.get(&ch) {
            idx = *next_idx;
        } else {
            return;
        }

        if !self.nodes[idx].data.is_empty() && !visited_str[idx] {
            result.push(self.nodes[idx].data.clone());
            visited_str[idx] = true;
        }

        board[i][j] = '#';
        if i > 0 {
            self.board_dfs_pruned_by_searching_trie_helper(
                i - 1,
                j,
                board,
                idx,
                result,
                visited_str,
            );
        }
        if j > 0 {
            self.board_dfs_pruned_by_searching_trie_helper(
                i,
                j - 1,
                board,
                idx,
                result,
                visited_str,
            );
        }
        if i + 1 < board.len() {
            self.board_dfs_pruned_by_searching_trie_helper(
                i + 1,
                j,
                board,
                idx,
                result,
                visited_str,
            );
        }
        if j + 1 < board[0].len() {
            self.board_dfs_pruned_by_searching_trie_helper(
                i,
                j + 1,
                board,
                idx,
                result,
                visited_str,
            );
        }
        board[i][j] = ch;
    }
}

enum SearchMode {
    Search,
    StartsWith,
}

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, usize>, // map to index in nodes
    data: String,                   // empty string means not a word
}

pub mod word_dictionary;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
    }
}
