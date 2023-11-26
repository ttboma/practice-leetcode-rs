use crate::Solution;

impl Solution {
    /// # [274. H-Index](https://leetcode.com/problems/h-index/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an array of integers `citations` where `citations[i]` is the number of citations a researcher received for their `i^th` paper, return the researcher's h-index.
    ///
    /// According to the <a href="https://en.wikipedia.org/wiki/H-index" target="_blank">definition of h-index on Wikipedia</a>: The h-index is defined as the maximum value of `h` such that the given researcher has published at least `h` papers that have each been cited at least `h` times.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: citations = [3,0,6,1,5]
    /// Output: 3
    /// Explanation: [3,0,6,1,5] means the researcher has 5 papers in total and each of them had received 3, 0, 6, 1, 5 citations respectively.
    /// Since the researcher has 3 papers with at least 3 citations each and the remaining two with no more than 3 citations each, their h-index is 3.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: citations = [1,3,1]
    /// Output: 1
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `n == citations.length`
    /// - `1 <= n <= 5000`
    /// - `0 <= citations[i] <= 1000`
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_by(|a, b| b.cmp(a));
        let citations: Vec<(usize, i32)> = citations.into_iter().enumerate().collect();
        match citations.binary_search_by(|probe| (probe.0).cmp(&(probe.1 as usize))) {
            Ok(index) => index as i32,
            Err(index) => index as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let citations = vec![3, 0, 6, 1, 5];
        assert_eq!(Solution::h_index(citations), 3);
    }

    #[test]
    fn example2() {
        let citations = vec![1, 3, 1];
        assert_eq!(Solution::h_index(citations), 1);
    }
}
