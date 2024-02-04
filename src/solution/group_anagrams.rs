use super::*;
use std::collections::HashMap;

impl Solution {
    /// # [49. Group Anagrams](https://leetcode.com/problems/group-anagrams/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an array of strings `strs`, group **the anagrams**  together. You can return the answer in **any order** .
    ///
    /// An **Anagram**  is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: strs = ["eat","tea","tan","ate","nat","bat"]
    /// Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: strs = [""]
    /// Output: [[""]]
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: strs = ["a"]
    /// Output: [["a"]]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= strs.length <= 10^4`
    /// - `0 <= strs[i].length <= 100`
    /// - `strs[i]` consists of lowercase English letters.
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut m = HashMap::new();
        for str_ in strs.iter() {
            let mut sorted_str = str_.as_bytes().to_vec();
            sorted_str.sort();
            m.entry(sorted_str)
                .and_modify(|v: &mut Vec<String>| v.push(str_.clone()))
                .or_insert(vec![str_.clone()]);
        }
        m.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ]),
            vec![vec!["bat"], vec!["eat", "tea", "ate"], vec!["tan", "nat"]]
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::group_anagrams(vec!["".to_string()]),
            vec![vec!["".to_string()]]
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::group_anagrams(vec!["a".to_string()]),
            vec![vec!["a".to_string()]]
        );
    }
}
