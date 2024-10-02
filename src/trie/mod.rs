use std::{collections::HashMap, str::Chars};

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
#[derive(Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }

    pub fn insert(&mut self, word: String) {
        self.root.insert_recursive(word.chars());
    }

    pub fn search(&self, word: String) -> bool {
        self.root.search_recursive(word.chars(), SearchMode::Search)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.root
            .search_recursive(prefix.chars(), SearchMode::StartsWith)
    }
}

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    fn insert_recursive(&mut self, mut chars: Chars) {
        match chars.next() {
            Some(ch) => {
                self.children
                    .entry(ch)
                    .or_insert(TrieNode {
                        children: HashMap::new(),
                        is_end: false,
                    })
                    .insert_recursive(chars);
            }
            None => {
                self.is_end = true;
            }
        }
    }

    fn search_recursive(&self, mut chars: Chars, search_mode: SearchMode) -> bool {
        match chars.next() {
            Some(ch) => match self.children.get(&ch) {
                Some(node) => node.search_recursive(chars, search_mode),
                None => false,
            },
            None => match search_mode {
                SearchMode::Search => self.is_end,
                SearchMode::StartsWith => true,
            },
        }
    }
}

enum SearchMode {
    Search,
    StartsWith,
}

pub mod word_dictionary;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple".to_string()), true);
        assert_eq!(trie.search("app".to_string()), false);
        assert_eq!(trie.starts_with("app".to_string()), true);
        trie.insert("app".to_string());
        assert_eq!(trie.search("app".to_string()), true);
    }
}
