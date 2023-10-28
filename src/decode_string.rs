use nom::AsChar;

use crate::Solution;

impl Solution {
    /// # [394. Decode String](https://leetcode.com/problems/decode-string/)
    ///
    /// Given an encoded string, return its decoded string.
    ///
    /// The encoding rule is: `k[encoded_string]`, where the `encoded_string` inside the square brackets is being repeated exactly `k` times. Note that `k` is guaranteed to be a positive integer.
    ///
    /// You may assume that the input string is always valid; there are no extra white spaces, square brackets are well-formed, etc. Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, `k`. For example, there will not be input like `3a` or `2[4]`.
    ///
    /// The test cases are generated so that the length of the output will never exceed `10^5`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: s = "3[a]2[bc]"
    /// Output: "aaabcbc"
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: s = "3[a2[c]]"
    /// Output: "accaccacc"
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: s = "2[abc]3[cd]ef"
    /// Output: "abcabccdcdcdef"
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= s.length <= 30`
    /// - `s` consists of lowercase English letters, digits, and square brackets `'[]'`.
    /// - `s` is guaranteed to be **a valid**  input.
    /// - All the integers in `s` are in the range `[1, 300]`.
    pub fn decode_string(s: String) -> String {
        let mut i = 0_usize;
        decode_string_impl(s.as_bytes(), &mut i)
    }
}

fn decode_string_impl(s: &[u8], i: &mut usize) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut ret = String::new();
    while *i != s.len() {
        if s[*i] == b']' {
            *i += 1;
            return ret;
        } else if s[*i].is_ascii_alphabetic() {
            ret.push(s[*i] as char);
            *i += 1;
        } else if s[*i].is_dec_digit() {
            let mut j = *i;
            while s[j] != b'[' {
                j += 1;
            }
            let n = std::str::from_utf8(&s[*i..j])
                .unwrap()
                .parse::<u32>()
                .unwrap();
            *i = j + 1;
            let sub = decode_string_impl(s, i);
            for _ in 0..n {
                ret.push_str(&sub);
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = String::from("3[a]2[bc]");
        assert_eq!(Solution::decode_string(s), "aaabcbc");
    }

    #[test]
    fn example2() {
        let s = String::from("3[a2[c]]");
        assert_eq!(Solution::decode_string(s), "accaccacc");
    }

    #[test]
    fn example3() {
        let s = String::from("2[abc]3[cd]ef");
        assert_eq!(Solution::decode_string(s), "abcabccdcdcdef");
    }
}
