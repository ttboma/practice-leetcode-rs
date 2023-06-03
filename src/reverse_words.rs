use crate::Solution;

impl Solution {
    /// # 151. Reverse Words in a String
    ///
    /// Given an input string `s`, reverse the order of the words.
    ///
    /// A word is defined as a sequence of non-space characters. The words in `s` will be separated by at least one space.
    /// 
    /// Return a string of the words in reverse order concatenated by a single space.
    ///
    /// Note that `s` may contain leading or trailing spaces or multiple spaces between two words.
    /// The returned string should only have a single space separating the words. Do not include any extra spaces.
    ///
    /// **Example 1:**
    ///
    /// - **Input:** s = "the sky is blue"
    /// - **Output:** "blue is sky the"
    ///
    /// **Example 2:**
    ///
    /// - **Input:** s = "  hello world  "
    /// - **Output:** "world hello"
    /// - **Explanation: ** Your reversed string should not contain leading or trailing spaces.
    /// 
    /// **Example 3:**
    ///
    /// - **Input:** s = "a good   example"
    /// - **Output:** "example good a"
    /// - **Explanation: ** You need to reduce multiple spaces between two words to a single space in the reversed string.
    ///
    /// **Constraints:**
    ///
    /// - `1 <= s.length <= 10^4`
    /// - `s` contains English letters (upper-case and lower-case), digits, and spaces `' '`.
    /// - There is at least one word in `s`.
    ///  
    /// Follow up: If the string data type is mutable in your language, can you solve it in-place with O(1) extra space?
    ///
    /// ------
    ///
    /// ***Extracted from:*** [reverse-words-in-a-string](https://leetcode.com/problems/reverse-words-in-a-string/)
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "the sky is blue".to_string();
        let output = "blue is sky the".to_string();
        assert_eq!(Solution::reverse_words(input), output);
    }

    #[test]
    fn example2() {
        let input = "a good   example".to_string();
        let output = "example good a".to_string();
        assert_eq!(Solution::reverse_words(input), output);
    }
}
