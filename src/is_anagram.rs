use crate::Solution;
use std::collections::HashMap;

impl Solution {
    /// # [242. Valid Anagram](https://leetcode.com/problems/valid-anagram/)
    ///
    /// Given two strings `s` and `t`, return `true` if `t` is an anagram of `s`, and `false` otherwise.
    ///  
    /// An **Anagram**  is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
    ///  
    /// **Example 1:**
    ///  
    /// ```
    /// Input: s = "anagram", t = "nagaram"
    /// Output: true
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```
    /// Input: s = "rat", t = "car"
    /// Output: false
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= s.length, t.length <= 5 * 10^4`
    /// - `s` and `t` consist of lowercase English letters.
    ///
    /// **Follow up:**  What if the inputs contain Unicode characters? How would you adapt your solution to such a case?
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut count = HashMap::<char, usize>::new();
        for s_char in s.chars() {
            count.entry(s_char).and_modify(|counter| *counter += 1).or_insert(1);
        }
        for t_char in t.chars() {
            match count.get_mut(&t_char) {
                Some(v) if *v != 0 => *v -= 1,
                _ => {
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
        let s = "anagram";
        let t = "nagaram";
        assert_eq!(Solution::is_anagram(s.to_string(), t.to_string()), true);
    }

    #[test]
    fn example2() {
        let s = "rat";
        let t = "car";
        assert_eq!(Solution::is_anagram(s.to_string(), t.to_string()), false);
    }

    #[test]
    fn example3() {
        let s = "aacc";
        let t = "ccac";
        assert_eq!(Solution::is_anagram(s.to_string(), t.to_string()), false);
    }
}
