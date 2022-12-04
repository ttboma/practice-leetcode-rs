use crate::Solution;

impl Solution {
    /// # 392. Is Subsequence
    ///
    /// Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
    /// A subsequence of a string is a new string that is formed from the original string by 
    /// deleting some (can be none) of the characters without disturbing the relative 
    /// positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while 
    /// "aec" is not).
    /// 
    /// **Example 1:**
    /// 
    /// - **Input:** s = "abc", t = "ahbgdc"
    /// - **Output:** true
    ///
    /// **Example 2:**
    /// 
    /// - **Input:** s = "axc", t = "ahbgdc"
    /// - **Output:** false
    ///  
    /// 
    /// **Constraints:**
    /// 
    /// - 0 <= s.length <= 100
    /// - 0 <= t.length <= 104
    /// - s and t consist only of lowercase English letters.
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut it = s.chars();
        if let Some(mut n) = it.next() {
            for m in t.chars() {
                if m == n {
                    if let Some(x) = it.next() {
                        n = x;
                    } else {
                        return true;
                    }
                }
            }
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        assert_eq!(Solution::is_subsequence(s, t), true)
    }
    #[test]
    fn example2() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");
        assert_eq!(Solution::is_subsequence(s, t), false)
    }
}
