use super::Solution;

impl Solution {
    /// # [28. Find the Index of the First Occurrence in a String](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given two strings `needle` and `haystack`, return the index of the first occurrence of `needle` in `haystack`, or `-1` if `needle` is not part of `haystack`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: haystack = "sadbutsad", needle = "sad"
    /// Output: 0
    /// Explanation: "sad" occurs at index 0 and 6.
    /// The first occurrence is at index 0, so we return 0.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: haystack = "leetcode", needle = "leeto"
    /// Output: -1
    /// Explanation: "leeto" did not occur in "leetcode", so we return -1.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= haystack.length, needle.length <= 10^4`
    /// - `haystack` and `needle` consist of only lowercase English characters.
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let overlap = overlap(needle);
        let mut i = 0;
        let mut j = 0;
        while i != haystack.len() {
            if haystack[i] == needle[j] {
                i += 1;
                j += 1;
                if j == needle.len() {
                    return i as i32 - j as i32;
                }
            } else if j != 0 {
                j = overlap[j - 1];
            } else if i >= haystack.len() - needle.len() {
                break;
            } else {
                i += 1;
            }
        }
        -1
    }
}

fn overlap(needle: &[u8]) -> Vec<usize> {
    let mut ret = vec![0; needle.len()];
    for i in 1..needle.len() {
        let mut j = ret[i - 1];
        while j != 0 && needle[j] != needle[i] {
            j = ret[j - 1];
        }
        if needle[j] == needle[i] {
            ret[i] = j + 1;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();
        assert_eq!(Solution::str_str(haystack, needle), 0);
    }

    #[test]
    fn example2() {
        let haystack = "leetcode".to_string();
        let needle = "leeto".to_string();
        assert_eq!(Solution::str_str(haystack, needle), -1);
    }

    #[test]
    fn example3() {
        let haystack = "mississippi".to_string();
        let needle = "issip".to_string();
        assert_eq!(Solution::str_str(haystack, needle), 4);
    }

    #[test]
    fn example4() {
        let haystack = "mississippi".to_string();
        let needle = "pi".to_string();
        assert_eq!(Solution::str_str(haystack, needle), 9);
    }

    #[test]
    fn example5() {
        let haystack = "aabaaabaaac".to_string();
        let needle = "aabaaac".to_string();
        assert_eq!(Solution::str_str(haystack, needle), 4);
    }

    #[test]
    fn example6() {
        let haystack = "aaa".to_string();
        let needle = "aaaa".to_string();
        assert_eq!(Solution::str_str(haystack, needle), -1);
    }
}
