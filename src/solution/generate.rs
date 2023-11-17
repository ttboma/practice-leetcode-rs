use crate::Solution;

impl Solution {
    /// # [118. Pascal's Triangle](https://leetcode.com/problems/pascals-triangle/)
    ///
    /// Given an integer `numRows`, return the first numRows of **Pascal's triangle** .
    ///
    /// In **Pascal's triangle** , each number is the sum of the two numbers directly above it as shown:
    /// <img alt="" src="https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif" style="height: 240px; width: 260px;">
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: numRows = 5
    /// Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: numRows = 1
    /// Output: [[1]]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= numRows <= 30`
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = vec![];
        for i in 0..(num_rows as usize) {
            let mut j = 0;
            let ele = std::iter::from_fn(|| {
                let ret_ij = if j > i {
                    None
                } else if j == 0 || j == i {
                    Some(1)
                } else {
                    Some(ret[i - 1][j - 1] + ret[i - 1][j])
                };
                j += 1;
                ret_ij
            });
            ret.push(ele.collect::<Vec<i32>>());
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
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }
}
