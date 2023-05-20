use std::collections::HashSet;

use crate::Solution;

impl Solution {
    /// # 345. Reverse Vowels of a String
    ///
    /// Given a string `s`, reverse only all the vowels in the string and return it.
    ///
    /// The vowels are `'a'`, `'e'`, `'i'`, `'o'`, and `'u'`, and they can appear in both lower and upper
    /// cases, more than once.
    ///
    /// **Example 1:**
    ///
    /// - **Input:** s = "hello"
    /// - **Output:** "holle"
    ///
    /// **Example 2:**
    ///
    /// - **Input:** s = "leetcode"
    /// - **Output:** "leotcede"
    ///
    /// **Constraints:**
    ///
    /// - `1 <= s.length <= 3 * 10^5`
    /// - `s` consist of printable ASCII characters.
    ///  
    /// Follow up: A linked list can be reversed either iteratively or recursively. Could you implement both?
    ///
    /// ------
    ///
    /// ***Extracted from:*** [reverse-vowels-of-a-string](https://leetcode.com/problems/reverse-vowels-of-a-string/)
    pub fn reverse_vowels(s: String) -> String {
        let mut ret = s.clone();
        let vowels: HashSet<char> = "aeiouAEIOU".chars().collect();
        let mut i = s.char_indices().filter(|(_idx, c)| vowels.contains(c));
        let mut j = s.char_indices().filter(|(_idx, c)| vowels.contains(c)).rev();
        loop {
            match (i.next(), j.next()) {
                (Some((i, v)), Some((j, u))) if i < j => {
                   ret.replace_range(i..i+1, &u.to_string());
                   ret.replace_range(j..j+1, &v.to_string());
                }
                _ => {
                    break ret;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        let s = String::from("hello");
        let output = String::from("holle");
        assert_eq!(Solution::reverse_vowels(s), output);
    }
    
    #[test]
    fn example2() {
        let s = String::from("leetcode");
        let output = String::from("leotcede");
        assert_eq!(Solution::reverse_vowels(s), output);
    }
}
