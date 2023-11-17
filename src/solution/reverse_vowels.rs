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
        fn is_vowel(c: u8) -> bool {
            b"aeiouAEIOU".contains(&c)
        }
        let mut s = s.into_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        loop {
            while !is_vowel(s[i]) && i < j {
                i += 1;
            }
            while !is_vowel(s[j]) && i < j {
                j -= 1;
            }
            if i < j {
                s.swap(i, j);
                i += 1;
                j -= 1;
            } else {
                break;
            }
        }
        String::from_utf8(s).unwrap()
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
