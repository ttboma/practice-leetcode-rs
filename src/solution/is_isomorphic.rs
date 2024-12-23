use crate::Solution;
use std::collections::HashMap;

impl Solution {
    /// # [205. Isomorphic Strings](https://leetcode.com/problems/isomorphic-strings/)
    ///
    /// Given two strings `s` and `t`, determine if they are isomorphic.
    ///
    /// Two strings `s` and `t` are isomorphic if the characters in `s` can be replaced to get `t`.
    ///
    /// All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: s = "egg", t = "add"
    /// Output: true
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: s = "foo", t = "bar"
    /// Output: false
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: s = "paper", t = "title"
    /// Output: true
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= s.length <= 5 * 10^4`
    /// - `t.length == s.length`
    /// - `s` and `t` consist of any valid ascii character.
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut table1 = HashMap::new();
        let mut table2 = HashMap::new();
        for (a, b) in s.chars().zip(t.chars()) {
            if let Some(ma) = table1.insert(a, b) {
                if ma != b {
                    return false;
                }
            }
            if let Some(mb) = table2.insert(b, a) {
                if mb != a {
                    return false;
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
        let s = String::from("egg");
        let t = String::from("add");
        assert!(Solution::is_isomorphic(s, t))
    }
    #[test]
    fn example2() {
        let s = String::from("foo");
        let t = String::from("bar");
        assert!(!Solution::is_isomorphic(s, t))
    }
    #[test]
    fn example3() {
        let s = String::from("paper");
        let t = String::from("title");
        assert!(Solution::is_isomorphic(s, t))
    }
    #[test]
    fn example4() {
        let s = String::from("");
        let t = String::from("");
        assert!(Solution::is_isomorphic(s, t))
    }
    #[test]
    fn example5() {
        let s = String::from("badc");
        let t = String::from("baba");
        assert!(!Solution::is_isomorphic(s, t))
    }
}
