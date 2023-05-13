use crate::Solution;

impl Solution {
    /// # 1768. Merge Strings Alternately
    /// 
    /// You are given two strings `word1` and `word2`. Merge the strings by adding letters in alternating order, 
    /// starting with `word1`. If a string is longer than the other, append the additional letters
    /// merged string.
    ///
    /// Return the merged string.
    ///
    /// **Example 1:**
    /// 
    /// - **Input:** word1 = "abc", word2 = "pqr"
    /// - **Output:** "apbqcr"
    /// - **Explanation:** The merged string will be merged as so:
    ///     ```txt
    ///     word1:  a   b   c
    ///     word2:    p   q   r
    ///     merged: a p b q c r
    ///     ```
    ///
    /// **Example 2:**
    /// 
    /// - **Input:** word1 = "ab", word2 = "pqrs"
    /// - **Output:** "apbqrs"
    /// - **Explanation:** Notice that as word2 is longer, "rs" is appended to the end.
    ///     ```txt
    ///     word1:  a   b 
    ///     word2:    p   q   r   s
    ///     merged: a p b q   r   s
    ///     ```
    /// 
    /// **Example 3:**
    /// 
    /// - **Input:** word1 = "abcd", word2 = "pq"
    /// - **Output:** "apbqcd"
    /// - **Explanation:** Notice that as word1 is longer, "cd" is appended to the end.
    ///     ```txt
    ///     word1:  a   b   c   d 
    ///     word2:    p   q 
    ///     merged: a p b q c   d
    ///     ```
    ///     
    /// **Constraints:**
    /// 
    /// - `1 <= word1.length, word2.length <= 100`
    /// - `word1` and `word2` consist of lowercase English letters.
    ///
    /// ------
    ///
    /// ***Extracted from:*** [merge-strings-alternately](https://leetcode.com/problems/merge-strings-alternately/)
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut ret = String::new();
        let mut it1 = word1.chars();
        let mut it2 = word2.chars();
        loop {
            if let Some(val) = it1.next() {
                ret.push(val);
            } else {
                ret.extend(it2);
                break ret;
            } 
            if let Some(val) = it2.next() {
                ret.push(val);
            } else {
                ret.extend(it1);
                break ret;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let word1 = "abc";
        let word2 = "pqr";
        let output = "apbqcr";
        assert_eq!(
            Solution::merge_alternately(word1.to_owned(), word2.to_owned()),
            output
        );
    }
    #[test]
    fn example2() {
        let word1 = "ab";
        let word2 = "pqrs";
        let output = "apbqrs";
        assert_eq!(
            Solution::merge_alternately(word1.to_owned(), word2.to_owned()),
            output
        );
    }
    #[test]
    fn example3() {
        let word1 = "abcd";
        let word2 = "pq";
        let output = "apbqcd";
        assert_eq!(
            Solution::merge_alternately(word1.to_owned(), word2.to_owned()),
            output
        );
    }
}
