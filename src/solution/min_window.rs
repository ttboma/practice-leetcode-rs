use std::collections::HashMap;

use super::Solution;

impl Solution {
    /// # [76. Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given two strings `s` and `t` of lengths `m` and `n` respectively, return the **minimum window**  <div aria-expanded="false" data-headlessui-state="" id="headlessui-popover-button-:rt:">**substring** <div style="position: fixed; z-index: 40; inset: 0px auto auto 0px; transform: translate(644px, 183px);"> of `s` such that every character in `t` (**including duplicates** ) is included in the window. If there is no such substring, return the empty string `""`.
    ///
    /// The testcases will be generated such that the answer is **unique** .
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: s = "ADOBECODEBANC", t = "ABC"
    /// Output: "BANC"
    /// Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: s = "a", t = "a"
    /// Output: "a"
    /// Explanation: The entire string s is the minimum window.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: s = "a", t = "aa"
    /// Output: ""
    /// Explanation: Both 'a's from t must be included in the window.
    /// Since the largest window of s only has one 'a', return empty string.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `m == s.length`
    /// - `n == t.length`
    /// - `1 <= m, n <= 10^5`
    /// - `s` and `t` consist of uppercase and lowercase English letters.
    ///
    /// **Follow up:**  Could you find an algorithm that runs in `O(m + n)` time?
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::new();
        }
        let mut t_map = HashMap::new();
        for c in t.chars() {
            t_map.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        MinWindow::new(&s, &mut t_map, t.len())
            .min_by_key(|s| s.len())
            .map(|s| s.to_owned())
            .unwrap_or_default()
    }
}

use std::str::CharIndices;
struct MinWindow<'a, 'b> {
    s: &'a str,
    t_map: &'b mut HashMap<char, i32>,
    t_len: usize,
    i: CharIndices<'a>,
    j: CharIndices<'a>,
}

impl<'a, 'b> MinWindow<'a, 'b> {
    fn new(s: &'a str, t_map: &'b mut HashMap<char, i32>, t_len: usize) -> Self {
        let i = s.char_indices();
        let j = s.char_indices();
        Self {
            s,
            t_map,
            t_len,
            i,
            j,
        }
    }

    fn move_j_until_window_is_complete(&mut self) -> Option<usize> {
        for (idx, c) in self.j.by_ref() {
            if let Some(e) = self.t_map.get_mut(&c) {
                *e -= 1;
                if *e >= 0 {
                    self.t_len -= 1;
                }
                if self.t_len == 0 {
                    return Some(idx);
                }
            }
        }
        None
    }

    fn move_i_until_window_is_not_complete(&mut self) -> Option<usize> {
        for (idx, c) in self.i.by_ref() {
            if let Some(e) = self.t_map.get_mut(&c) {
                *e += 1;
                if *e > 0 {
                    self.t_len += 1;
                    return Some(idx);
                }
            }
        }
        None
    }
}

impl<'a> Iterator for MinWindow<'a, '_> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let j = self.move_j_until_window_is_complete()?;
        let i = self.move_i_until_window_is_not_complete().unwrap();
        Some(&self.s[i..j + 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::min_window("a".to_string(), "a".to_string()),
            "a".to_string()
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::min_window("a".to_string(), "aa".to_string()),
            "".to_string()
        );
    }
    #[test]
    fn example4() {
        assert_eq!(
            Solution::min_window("bba".to_string(), "ab".to_string()),
            "ba".to_string()
        );
    }
    #[test]
    fn example5() {
        assert_eq!(
            Solution::min_window("aaaaaaaaaaaabbbbbcdd".to_string(), "abcdd".to_string()),
            "abbbbbcdd".to_string()
        );
    }
}
