use super::*;

impl Solution {
    /// # [77. Combinations](https://leetcode.com/problems/combinations/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given two integers `n` and `k`, return all possible combinations of `k` numbers chosen from the range `[1, n]`.
    ///
    /// You may return the answer in **any order** .
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: n = 4, k = 2
    /// Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
    /// Explanation: There are 4 choose 2 = 6 total combinations.
    /// Note that combinations are unordered, i.e., [1,2] and [2,1] are considered to be the same combination.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 1, k = 1
    /// Output: [[1]]
    /// Explanation: There is 1 choose 1 = 1 total combination.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= n <= 20`
    /// - `1 <= k <= n`
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut combination = Vec::new();
        combine_backtrack(n, k, &mut result, &mut combination);
        result
    }
}

fn combine_backtrack(n: i32, k: i32, result: &mut Vec<Vec<i32>>, combination: &mut Vec<i32>) {
    if k == 0 {
        result.push(combination.clone());
        return;
    }
    for i in k..=n {
        combination.push(i);
        combine_backtrack(i - 1, k - 1, result, combination);
        combination.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![2, 1],
                vec![3, 1],
                vec![3, 2],
                vec![4, 1],
                vec![4, 2],
                vec![4, 3]
            ]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
    }
}
