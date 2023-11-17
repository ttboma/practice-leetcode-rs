use crate::Solution;
use std::mem;

fn remainder<'a>(a: &'a str, b: &'a str) -> Option<&'a str> {
    let mut a = a;
    if !a.starts_with(b) {
        return None;
    }
    while a.starts_with(b) {
        a = &a[b.len()..];
    }
    Some(a)
}

impl Solution {
    /// # 1071. Greatest Common Divisor of Strings
    ///
    /// For two strings `s` and `t`, we say "`t` divides `s`" if and only if `s = t + ... + t` (i.e., `t` is concatenated
    /// with itself one or more times).
    /// Given two strings `str1` and `str2`, return the largest string `x` such that `x` divides both `str1`
    /// and `str2`.
    ///
    /// **Example 1:**
    ///
    /// - **Input:** str1 = "ABCABC", str2 = "ABC"
    /// - **Output:** "ABC"
    ///
    /// **Example 2:**
    ///
    /// - **Input:** str1 = "ABABAB", str2 = "ABAB"
    /// - **Output:** "AB"
    ///
    /// **Example 3:**
    ///
    /// - **Input:** str1 = "LEET", str2 = "CODE"
    /// - **Output:** ""
    ///
    /// **Constraints:**
    ///
    /// - `1 <= str1.length, str2.length <= 1000`
    /// - `str1` and `str2` consist of English uppercase letters.
    ///
    /// ------
    ///
    /// ***Extracted from:*** [greatest-common-divisor-of-strings](https://leetcode.com/problems/greatest-common-divisor-of-strings/)
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut a = str1.as_str();
        let mut b = str2.as_str();
        if a.len() < b.len() {
            mem::swap(&mut a, &mut b);
        } // so that a.len() is always greater and equal than b.len()
        while !b.is_empty() {
            if let Some(r) = remainder(a, b) {
                a = b;
                b = r;
            } else {
                return "".to_string();
            }
        }
        a.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let str1 = "ABCABC";
        let str2 = "ABC";
        let output = "ABC";
        assert_eq!(
            Solution::gcd_of_strings(str1.to_string(), str2.to_string()),
            output
        );
    }

    #[test]
    fn example2() {
        let str1 = "ABABAB";
        let str2 = "AB";
        let output = "AB";
        assert_eq!(
            Solution::gcd_of_strings(str1.to_string(), str2.to_string()),
            output
        );
    }

    #[test]
    fn example3() {
        let str1 = "LEET";
        let str2 = "CODE";
        let output = "";
        assert_eq!(
            Solution::gcd_of_strings(str1.to_string(), str2.to_string()),
            output
        );
    }
}
