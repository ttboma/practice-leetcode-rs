use crate::Solution;

impl Solution {
    /// # 205. Isomorphic Strings
    ///
    /// Given two strings s and t, determine if they are isomorphic.
    /// Two strings s and t are isomorphic if the characters in s can
    /// be replaced to get t. All occurrences of a character must be
    /// replaced with another character while preserving the order of
    /// characters. No two characters may map to the same character,
    /// but a character may map to itself.
    ///
    /// **Example 1:**
    ///
    /// - **Input:** s = "egg", t = "add"
    /// - **Output:** true
    ///
    /// **Example 2:**
    ///
    /// - **Input:** s = "foo", t = "bar"
    /// - **Output:** false
    ///
    /// **Example 3:**
    ///
    /// - **Input:** s = "paper", t = "title"
    /// - **Output:** true
    ///  
    /// **Constraints:**
    ///
    /// - 1 <= s.length <= 5 * 104
    /// - t.length == s.length
    /// - s and t consist of any valid ascii character.
    ///
    /// ------
    ///
    /// ***Extracted from:*** [isomorphic-strings](https://leetcode.com/problems/isomorphic-strings/)
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;

        let mut m_s = HashMap::new();
        let mut m_t = HashMap::new();

        let mut it_s = s.chars().enumerate();
        let mut it_t = t.chars().enumerate();

        while let (Some((i, n)), Some((j, m))) = (it_s.next(), it_t.next()) {
            if let Some(x) = m_s.get(&n) {
                if let Some(y) = m_t.get(&m) {
                    if x != y {
                        return false;
                    }
                } else {
                    return false;
                }
            } else if m_t.get(&m).is_some() {
                return false;
            } else {
                m_s.insert(n, i);
                m_t.insert(m, j);
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
        assert_eq!(Solution::is_isomorphic(s, t), true)
    }
    #[test]
    fn example2() {
        let s = String::from("foo");
        let t = String::from("bar");
        assert_eq!(Solution::is_isomorphic(s, t), false)
    }
    #[test]
    fn example3() {
        let s = String::from("paper");
        let t = String::from("title");
        assert_eq!(Solution::is_isomorphic(s, t), true)
    }
    #[test]
    fn example4() {
        let s = String::from("");
        let t = String::from("");
        assert_eq!(Solution::is_isomorphic(s, t), true)
    }
    #[test]
    fn example5() {
        let s = String::from("badc");
        let t = String::from("baba");
        assert_eq!(Solution::is_isomorphic(s, t), false)
    }
}
