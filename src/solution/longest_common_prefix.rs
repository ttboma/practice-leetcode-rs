use super::*;

impl Solution {
    /// # [14. Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/description/)
    ///
    /// Write a function to find the longest common prefix string amongst an array of strings.
    ///
    /// If there is no common prefix, return an empty string `""`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: strs = ["flower","flow","flight"]
    /// Output: "fl"
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: strs = ["dog","racecar","car"]
    /// Output: ""
    /// Explanation: There is no common prefix among the input strings.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= strs.length <= 200`
    /// - `0 <= strs[i].length <= 200`
    /// - `strs[i]` consists of only lowercase English letters.
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs[0].to_owned();
        }

        let min_length_of_string_in_strs = strs.iter().map(|s| s.len()).min().unwrap();
        let mut i = 0;
        while i != min_length_of_string_in_strs {
            for s in strs.iter().skip(1) {
                if s.as_bytes()[i] != strs[0].as_bytes()[i] {
                    return strs[0][..i].to_owned();
                }
            }
            i += 1;
        }
        return strs.first().unwrap()[0..i].to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!("fl", Solution::longest_common_prefix(strs));
    }

    #[test]
    fn example2() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!("", Solution::longest_common_prefix(strs));
    }

    #[test]
    fn example3() {
        let strs = vec!["ab".to_string(), "a".to_string()];
        assert_eq!("a", Solution::longest_common_prefix(strs));
    }

    #[test]
    fn example4() {
        let strs = vec!["a".to_string()];
        assert_eq!("a", Solution::longest_common_prefix(strs));
    }
}
