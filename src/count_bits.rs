use crate::Solution;

impl Solution {
    /// # 338. Counting Bits
    /// 
    /// Given an integer `n`, return an array `ans` of length `n + 1` such that
    /// for each `i` (`0 <= i <= n`), `ans[i]` is the number of `1`'s in the
    /// binary representation of `i`.
    ///
    /// **Example 1:**
    /// 
    /// - **Input:** n = 2
    /// - **Output:** [0,1,1]
    /// - **Explanation:** 
    ///     ```txt
    ///     0 --> 0
    ///     1 --> 1
    ///     2 --> 10
    ///     ```
    ///
    /// **Example 2:**
    /// 
    /// - **Input:** n = 5
    /// - **Output:** [0,1,1,2,1,2]
    /// - **Explanation:**
    ///     ```txt
    ///     0 --> 0
    ///     1 --> 1
    ///     2 --> 10
    ///     3 --> 11
    ///     4 --> 100
    ///     5 --> 101
    ///     ```
    /// 
    /// **Constraints:**
    /// 
    /// - `0 <= n <= 10^5`
    ///
    /// ------
    ///
    /// ***Extracted from:*** [counting-bits](https://leetcode.com/problems/counting-bits/)
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ret = vec![0];
        if n != 0 {
            let mut i: usize = 0;
            let mut j: usize = 1;
            while j <= n as usize {
                for k in 1..i {
                    ret.push(ret[k] + 1);
                }
                ret.push(1);
                i = j;
                j *= 2;
            }
            for k in 1..(n as usize - i + 1) {
                ret.push(ret[k] + 1);
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
        assert_eq!(Solution::count_bits(0), vec![0]);
    }
    
    #[test]
    fn example2() {
        assert_eq!(Solution::count_bits(1), vec![0, 1]);
    }
    
    #[test]
    fn example3() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
