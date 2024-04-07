use super::Solution;
use std::collections::HashMap;

impl Solution {
    /// # [3. Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given a string `s`, find the length of the **longest**  <div aria-expanded="false" data-headlessui-state="" id="headlessui-popover-button-:rj:">**substring** <div style="position: fixed; z-index: 40; inset: 0px auto auto 0px; transform: translate(367px, 215px);"> without repeating characters.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: s = "abcabcbb"
    /// Output: 3
    /// Explanation: The answer is "abc", with the length of 3.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: s = "bbbbb"
    /// Output: 1
    /// Explanation: The answer is "b", with the length of 1.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: s = "pwwkew"
    /// Output: 3
    /// Explanation: The answer is "wke", with the length of 3.
    /// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `0 <= s.length <= 5 * 10^4`
    /// - `s` consists of English letters, digits, symbols and spaces.
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() <= 1 {
            return s.len() as i32;
        }
        let mut map: HashMap<char, usize> = HashMap::from([(s.chars().next().unwrap(), 0)]);
        let mut max = 1;
        let mut start = 0;
        for (i, c) in s.chars().enumerate().skip(1) {
            map.entry(c)
                .and_modify(|j| {
                    start = start.max(*j + 1);
                    max = max.max(i - start + 1);
                    *j = i;
                })
                .or_insert_with(|| {
                    max = max.max(i - start + 1);
                    i
                });
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);
    }

    #[test]
    fn example5() {
        assert_eq!(
            Solution::length_of_longest_substring("aabaab!bb".to_string()),
            3
        );
    }
}
