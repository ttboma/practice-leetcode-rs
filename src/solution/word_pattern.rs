use super::*;
use std::collections::HashMap;

impl Solution {
    /// # [290. Word Pattern](https://leetcode.com/problems/word-pattern/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given a `pattern` and a string `s`, find if `s`follows the same pattern.
    ///
    /// Here <b>follow</b> means a full match, such that there is a bijection between a letter in `pattern` and a <b>non-empty</b> word in `s`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: pattern = "abba", s = "dog cat cat dog"
    /// Output: true
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: pattern = "abba", s = "dog cat cat fish"
    /// Output: false
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: pattern = "aaaa", s = "dog cat cat dog"
    /// Output: false
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= pattern.length <= 300`
    /// - `pattern` contains only lower-case English letters.
    /// - `1 <= s.length <= 3000`
    /// - `s` contains only lowercase English letters and spaces `' '`.
    /// - `s` **does not contain**  any leading or trailing spaces.
    /// - All the words in `s` are separated by a **single space** .
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let pattern = pattern.as_bytes();
        let s: Vec<&str> = s.split(' ').collect();
        let mut p2s = HashMap::<u8, &str>::new();
        let mut s2p = HashMap::<&str, u8>::new();

        if pattern.len() != s.len() {
            return false;
        }

        for (&a, &b) in pattern.iter().zip(s.iter()) {
            let ma = p2s.get(&a);
            let mb = s2p.get(&b);
            if ma.is_none() && mb.is_none() {
                p2s.insert(a, b);
                s2p.insert(b, a);
            } else if ma.is_some() && mb.is_some() {
                if *ma.unwrap() != b || *mb.unwrap() != a {
                    return false;
                }
            } else {
                return false;
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
        let pattern = String::from("abba");
        let s = String::from("dog cat cat dog");
        assert!(Solution::word_pattern(pattern, s))
    }
    #[test]
    fn example2() {
        let pattern = String::from("abba");
        let s = String::from("dog cat cat fish");
        assert!(!Solution::word_pattern(pattern, s))
    }
    #[test]
    fn example3() {
        let pattern = String::from("aaaa");
        let s = String::from("dog cat cat dog");
        assert!(!Solution::word_pattern(pattern, s))
    }
}
