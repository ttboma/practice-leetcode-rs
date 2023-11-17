use crate::Solution;

impl Solution {
    /// # [1638. Count Substrings That Differ by One Character](https://leetcode.com/problems/count-substrings-that-differ-by-one-character/)
    ///
    /// Given two strings `s` and `t`, find the number of ways you can choose a non-empty substring of `s` and replace a **single character**  by a different character such that the resulting substring is a substring of `t`. In other words, find the number of substrings in `s` that differ from some substring in `t` by **exactly**  one character.
    ///
    /// For example, the underlined substrings in `"computer"` and `"computation"` only differ by the `'e'`/`'a'`, so this is a valid way.
    ///
    /// Return the number of substrings that satisfy the condition above.
    ///
    /// A **substring**  is a contiguous sequence of characters within a string.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: s = "aba", t = "baba"
    /// Output: 6
    /// Explanation: The following are the pairs of substrings from s and t that differ by exactly 1 character:
    /// ("aba", "baba")
    /// ("aba", "baba")
    /// ("aba", "baba")
    /// ("aba", "baba")
    /// ("aba", "baba")
    /// ("aba", "baba")
    /// The underlined portions are the substrings that are chosen from s and t.
    /// ```
    ///
    /// ​​**Example 2:**
    ///
    /// ```txt
    /// Input: s = "ab", t = "bb"
    /// Output: 3
    /// Explanation: The following are the pairs of substrings from s and t that differ by 1 character:
    /// ("ab", "bb")
    /// ("ab", "bb")
    /// ("ab", "bb")
    /// ​​​​The underlined portions are the substrings that are chosen from s and t.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= s.length, t.length <= 100`
    /// - `s` and `t` consist of lowercase English letters only.
    pub fn count_substrings(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut n = 0;
        for (i, j) in (0..s.len()).flat_map(|i| (0..t.len()).map(move |j| (i, j))) {
            if s[i] == t[j] {
                continue;
            }
            let mut l = 0;
            for k in 1..=std::cmp::min(i, j) {
                if s[i - k] != t[j - k] {
                    break;
                }
                l += 1;
            }
            let mut r = 0;
            for k in 1..std::cmp::min(s.len() - i, t.len() - j) {
                if s[i + k] != t[j + k] {
                    break;
                }
                r += 1;
            }
            n += (l + 1) * (r + 1);
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::count_substrings("aba".into(), "baba".into()), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::count_substrings("ab".into(), "bb".into()), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::count_substrings("abe".into(), "bbc".into()), 10);
    }
}
