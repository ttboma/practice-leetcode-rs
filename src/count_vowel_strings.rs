use crate::Solution;

impl Solution {
    /// # [1641. Count Sorted Vowel Strings](https://leetcode.com/problems/count-sorted-vowel-strings/)
    ///
    /// Given an integer `n`, return the number of strings of length `n` that consist only of vowels (`a`, `e`, `i`, `o`, `u`) and are **lexicographically sorted** .
    ///
    /// A string `s` is **lexicographically sorted**  if for all valid `i`, `s[i]` is the same as or comes before `s[i+1]` in the alphabet.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: n = 1
    /// Output: 5
    /// Explanation: The 5 sorted strings that consist of vowels only are `["a","e","i","o","u"].`
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 2
    /// Output: 15
    /// Explanation: The 15 sorted strings that consist of vowels only are
    /// ["aa","ae","ai","ao","au","ee","ei","eo","eu","ii","io","iu","oo","ou","uu"].
    /// Note that "ea" is not a valid string since 'e' comes after 'a' in the alphabet.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: n = 33
    /// Output: 66045
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= n <= 50`
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut count = [1, 2, 3, 4, 5];
        for _ in 1..n {
            for j in 1..count.len() {
                count[j] += count[j - 1];
            }
        }
        count[4]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::count_vowel_strings(1), 5);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::count_vowel_strings(2), 15);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::count_vowel_strings(33), 66045);
    }
}
