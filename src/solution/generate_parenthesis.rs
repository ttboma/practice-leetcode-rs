use crate::Solution;

impl Solution {
    /// # [22. Generate Parentheses](https://leetcode.com/problems/generate-parentheses/)
    ///
    /// Given `n` pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: n = 3
    /// Output: ["((()))","(()())","(())()","()(())","()()()"]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 1
    /// Output: ["()"]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= n <= 8`
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut table = vec![vec!["".to_string()], vec!["()".to_string()]];
        for k in 2..=n as usize {
            let mut next_row = vec![];
            for (i, j) in (0..k).map(|i| (i, k - i - 1)) {
                for first in &table[i] {
                    for second in &table[j] {
                        let item = "(".to_string() + first + ")" + second;
                        next_row.push(item);
                    }
                }
            }
            table.push(next_row);
        }
        table.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["()()()", "()(())", "(())()", "(()())", "((()))"]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    }
}
