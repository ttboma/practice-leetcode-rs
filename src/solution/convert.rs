use super::*;

impl Solution {
    /// # [6. Zigzag Conversion](https://leetcode.com/problems/zigzag-conversion/description/)
    ///
    /// The string `"PAYPALISHIRING"` is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
    ///
    /// ```txt
    /// P   A   H   N
    /// A P L S I I G
    /// Y   I   R
    /// ```
    ///
    /// And then read line by line: `"PAHNAPLSIIGYIR"`
    ///
    /// Write the code that will take a string and make this conversion given a number of rows:
    ///
    /// ```txt
    /// string convert(string s, int numRows);
    /// ```
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: s = "PAYPALISHIRING", numRows = 3
    /// Output: "PAHNAPLSIIGYIR"
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: s = "PAYPALISHIRING", numRows = 4
    /// Output: "PINALSIGYAHRPI"
    /// Explanation:
    /// P     I    N
    /// A   L S  I G
    /// Y A   H R
    /// P     I
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: s = "A", numRows = 1
    /// Output: "A"
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= s.length <= 1000`
    /// - `s` consists of English letters (lower-case and upper-case), `','` and `'.'`.
    /// - `1 <= numRows <= 1000`
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut ret = String::new();
        let s = s.as_bytes();
        let col_width = num_rows as usize - 1;
        let mut j = 0;
        while j < s.len() {
            ret.push(s[j] as char);
            j += 2 * col_width;
        }
        for i in 1..num_rows as usize - 1 {
            let mut j = i;
            while j < s.len() {
                ret.push(s[j] as char);
                j += 2 * (num_rows as usize - i - 1);
                if j < s.len() {
                    ret.push(s[j] as char);
                    j += 2 * i;
                } else {
                    break;
                }
            }
        }
        let mut j = num_rows as usize - 1;
        while j < s.len() {
            ret.push(s[j] as char);
            j += 2 * col_width;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use std::iter::successors;

    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
    }
}
