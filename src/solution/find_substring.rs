use super::Solution;
use std::collections::HashMap;

impl Solution {
    /// # [30. Substring with Concatenation of All Words](https://leetcode.com/problems/substring-with-concatenation-of-all-words/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// You are given a string `s` and an array of strings `words`. All the strings of `words` are of **the same length** .
    ///
    /// A **concatenated string**  is a string that exactly contains all the strings of any permutation of `words` concatenated.
    ///
    /// - For example, if `words = ["ab","cd","ef"]`, then `"abcdef"`, `"abefcd"`, `"cdabef"`, `"cdefab"`, `"efabcd"`, and `"efcdab"` are all concatenated strings. `"acdbef"` is not a concatenated string because it is not the concatenation of any permutation of `words`.
    ///
    /// Return an array of the starting indices of all the concatenated substrings in `s`. You can return the answer in **any order** .
    ///
    /// **Example 1:**
    ///
    /// <div class="example-block">
    /// Input: s = "barfoothefoobarman", words = ["foo","bar"]
    ///
    /// Output: [0,9]
    ///
    /// Explanation:
    ///
    /// The substring starting at 0 is `"barfoo"`. It is the concatenation of `["bar","foo"]` which is a permutation of `words`.<br>
    /// The substring starting at 9 is `"foobar"`. It is the concatenation of `["foo","bar"]` which is a permutation of `words`.
    ///
    /// **Example 2:**
    ///
    /// <div class="example-block">
    /// Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
    ///
    /// Output: []
    ///
    /// Explanation:
    ///
    /// There is no concatenated substring.
    ///
    /// **Example 3:**
    ///
    /// <div class="example-block">
    /// Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
    ///
    /// Output: [6,9,12]
    ///
    /// Explanation:
    ///
    /// The substring starting at 6 is `"foobarthe"`. It is the concatenation of `["foo","bar","the"]`.<br>
    /// The substring starting at 9 is `"barthefoo"`. It is the concatenation of `["bar","the","foo"]`.<br>
    /// The substring starting at 12 is `"thefoobar"`. It is the concatenation of `["the","foo","bar"]`.
    ///
    /// **Constraints:**
    ///
    /// - `1 <= s.length <= 10^4`
    /// - `1 <= words.length <= 5000`
    /// - `1 <= words[i].length <= 30`
    /// - `s` and `words[i]` consist of lowercase English letters.
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut ret = vec![];
        let n = words[0].len();
        let m = words.len() * n;
        if s.len() < m {
            return ret;
        }
        let mut word_map = HashMap::new();
        words.iter().for_each(|word| {
            word_map
                .entry(word.as_str())
                .and_modify(|(num, _)| *num += 1)
                .or_insert((1, 0));
        });
        let mut i = 0;
        let mut j = 0;
        while j <= s.len() - n {
            match word_map.get_mut(&s[j..j + n]) {
                Some((num, count)) if count != num => {
                    *count += 1;
                }
                _ => {
                    i += 1;
                    j = i;
                    word_map.iter_mut().for_each(|(_, (_, count))| {
                        *count = 0;
                    });
                    continue;
                }
            }
            j += n;
            if j - i == m {
                ret.push(i as i32);
                i += 1;
                j = i;
                word_map.iter_mut().for_each(|(_, (_, count))| {
                    *count = 0;
                });
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()]
            ),
            vec![0, 9]
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ]
            ),
            vec![]
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::find_substring(
                "barfoofoobarthefoobarman".to_string(),
                vec!["bar".to_string(), "foo".to_string(), "the".to_string()]
            ),
            vec![6, 9, 12]
        );
    }
    #[test]
    fn example4() {
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![8]
        );
    }
    #[test]
    fn example5() {
        assert_eq!(
            Solution::find_substring(
                "lingmindraboofooowingdingbarrwingmonkeypoundcake".to_string(),
                vec![
                    "fooo".to_string(),
                    "barr".to_string(),
                    "wing".to_string(),
                    "ding".to_string(),
                    "wing".to_string()
                ]
            ),
            vec![13]
        );
    }
    #[test]
    fn example6() {
        assert_eq!(
            Solution::find_substring(
                "aaaaaaaaaaaaaa".to_string(),
                vec!["aa".to_string(), "aa".to_string()]
            ),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
    }
    #[test]
    fn example7() {
        assert_eq!(
            Solution::find_substring(
                "ababaab".to_string(),
                vec!["ab".to_string(), "ba".to_string(), "ba".to_string()]
            ),
            vec![1]
        );
    }
    #[test]
    fn example8() {
        assert_eq!(
            Solution::find_substring("mississippi".to_string(), vec!["mississippis".to_string()]),
            vec![]
        );
    }
}
