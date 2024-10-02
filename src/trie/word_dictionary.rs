use std::collections::HashMap;

/// # [211. Design Add and Search Words Data Structure](https://leetcode.com/problems/design-add-and-search-words-data-structure/description/?envType=study-plan-v2&envId=top-interview-150)
///
/// Design a data structure that supports adding new words and finding if a string matches any previously added string.
///
/// Implement the `WordDictionary` class:
///
/// - `WordDictionary()`Initializes the object.
/// - `void addWord(word)` Adds `word` to the data structure, it can be matched later.
/// - `bool search(word)`Returns `true` if there is any string in the data structure that matches `word`or `false` otherwise. `word` may contain dots `'.'` where dots can be matched with any letter.
///
/// **Example:**
///
/// ```txt
/// Input
///
/// ["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
/// [[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
/// Output
///
/// [null,null,null,null,false,true,true,true]
///
/// Explanation
///
/// WordDictionary wordDictionary = new WordDictionary();
/// wordDictionary.addWord("bad");
/// wordDictionary.addWord("dad");
/// wordDictionary.addWord("mad");
/// wordDictionary.search("pad"); // return False
/// wordDictionary.search("bad"); // return True
/// wordDictionary.search(".ad"); // return True
/// wordDictionary.search("b.."); // return True
/// ```
///
/// **Constraints:**
///
/// - `1 <= word.length <= 25`
/// - `word` in `addWord` consists of lowercase English letters.
/// - `word` in `search` consist of `'.'` or lowercase English letters.
/// - There will be at most `2` dots in `word` for `search` queries.
/// - At most `10^4` calls will be made to `addWord` and `search`.
#[derive(Default)]
pub struct WordDictionary {
    root: WordDictionaryNode,
}

impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary::default()
    }

    pub fn add_word(&mut self, word: String) {
        if word.is_empty() {
            return;
        }
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        Self::search_helper(word.chars(), &self.root)
    }

    fn search_helper(mut iter: std::str::Chars, node: &WordDictionaryNode) -> bool {
        match iter.next() {
            Some('.') => {
                for child in node.children.values() {
                    if Self::search_helper(iter.clone(), child) {
                        return true;
                    }
                }
                false
            }
            Some(ch) => node
                .children
                .get(&ch)
                .map_or(false, |child| Self::search_helper(iter, child)),
            None => node.is_word,
        }
    }
}

#[derive(Default)]
struct WordDictionaryNode {
    children: HashMap<char, WordDictionaryNode>,
    is_word: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut wd = WordDictionary::new();
        wd.add_word("bad".to_string());
        wd.add_word("dad".to_string());
        wd.add_word("mad".to_string());
        assert_eq!(wd.search("pad".to_string()), false);
        assert_eq!(wd.search("bad".to_string()), true);
        assert_eq!(wd.search(".ad".to_string()), true);
        assert_eq!(wd.search("b..".to_string()), true);
    }

    #[test]
    fn example2() {
        let mut wd = WordDictionary::new();
        wd.add_word("at".to_string());
        wd.add_word("and".to_string());
        wd.add_word("an".to_string());
        wd.add_word("add".to_string());
        assert_eq!(wd.search("a".to_string()), false);
        assert_eq!(wd.search(".at".to_string()), false);
        wd.add_word("bat".to_string());
        assert_eq!(wd.search(".at".to_string()), true);
        assert_eq!(wd.search("an.".to_string()), true);
        assert_eq!(wd.search("a.d.".to_string()), false);
        assert_eq!(wd.search("b.".to_string()), false);
        assert_eq!(wd.search("a.d".to_string()), true);
        assert_eq!(wd.search(".".to_string()), false);
    }
}
