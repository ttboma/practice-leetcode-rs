use super::*;
use std::collections::HashMap;

impl Solution {
    /// # [383. Ransom Note](https://leetcode.com/problems/ransom-note/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given two strings `ransomNote` and `magazine`, return `true` if `ransomNote` can be constructed by using the letters from `magazine` and `false` otherwise.
    ///
    /// Each letter in `magazine` can only be used once in `ransomNote`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: ransomNote = "a", magazine = "b"
    /// Output: false
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: ransomNote = "aa", magazine = "ab"
    /// Output: false
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: ransomNote = "aa", magazine = "aab"
    /// Output: true
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= ransomNote.length, magazine.length <= 10^5`
    /// - `ransomNote` and `magazine` consist of lowercase English letters.
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }
        let mut table = HashMap::new();
        for c in magazine.chars() {
            let count = table.entry(c).or_insert(0);
            *count += 1;
        }
        for c in ransom_note.chars() {
            match table.get_mut(&c) {
                Some(v) => {
                    if *v == 0 {
                        return false;
                    } else {
                        *v -= 1;
                    }
                }
                None => return false,
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
        assert!(!Solution::can_construct("a".to_string(), "b".to_string()),)
    }

    #[test]
    fn example2() {
        assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()),)
    }

    #[test]
    fn example3() {
        assert!(Solution::can_construct("aa".to_string(), "aab".to_string()))
    }
}
