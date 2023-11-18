use crate::Solution;

impl Solution {
    /// # [409. Longest Palindrome](https://leetcode.com/problems/longest-palindrome/)
    ///
    /// Given a string `s` which consists of lowercase or uppercase letters, return the length of the **longest palindrome** that can be built with those letters.
    ///
    /// Letters are **case sensitive** , for example,`"Aa"` is not considered a palindrome here.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: s = "abccccdd"
    /// Output: 7
    /// Explanation: One longest palindrome that can be built is "dccaccd", whose length is 7.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: s = "a"
    /// Output: 1
    /// Explanation: The longest palindrome that can be built is "a", whose length is 1.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= s.length <= 2000`
    /// - `s` consists of lowercase **and/or**  uppercase English letters only.
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        for ch in s.chars() {
            m.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
        }

        let mut out = 0;
        let mut has_center = false;

        for (_ch, counter) in m {
            if counter % 2 == 1 {
                has_center = true;
                out += counter - 1;
            } else {
                out += counter;
            }
        }
        if has_center {
            out + 1
        } else {
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = String::from("abccccdd");
        assert_eq!(Solution::longest_palindrome(s), 7);
    }
    #[test]
    fn example2() {
        let s = String::from("a");
        assert_eq!(Solution::longest_palindrome(s), 1);
    }
}
