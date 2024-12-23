use super::*;
use std::cmp::Ordering;

impl Solution {
    /// # [39. Combination Sum](https://leetcode.com/problems/combination-sum/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an array of **distinct** integers `candidates` and a target integer `target`, return a list of all **unique combinations**  of `candidates` where the chosen numbers sum to `target`. You may return the combinations in **any order** .
    ///
    /// The **same** number may be chosen from `candidates` an **unlimited number of times** . Two combinations are unique if the <div aria-expanded="false" data-headlessui-state="" id="headlessui-popover-button-:rj:">frequency<div style="position: fixed; z-index: 40; inset: 0px auto auto 0px; transform: translate(167px, 283px);"> of at least one of the chosen numbers is different.
    ///
    /// The test cases are generated such that the number of unique combinations that sum up to `target` is less than `150` combinations for the given input.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: candidates = [2,3,6,7], target = 7
    /// Output: [[2,2,3],[7]]
    /// Explanation:
    /// 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
    /// 7 is a candidate, and 7 = 7.
    /// These are the only two combinations.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: candidates = [2,3,5], target = 8
    /// Output: [[2,2,2,2],[2,3,3],[3,5]]
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: candidates = [2], target = 1
    /// Output: []
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= candidates.length <= 30`
    /// - `2 <= candidates[i] <= 40`
    /// - All elements of `candidates` are **distinct** .
    /// - `1 <= target <= 40`
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut x = CombinationSum {
            output: vec![],
            combination: vec![],
            candidates: &mut candidates,
            target,
            sum: 0,
        };
        x.backtrace(0);
        x.output
    }
}

struct CombinationSum<'a> {
    pub output: Vec<Vec<i32>>,
    combination: Vec<i32>,
    candidates: &'a Vec<i32>,
    target: i32,
    sum: i32,
}

impl CombinationSum<'_> {
    fn backtrace(&mut self, start: usize) {
        for i in start..self.candidates.len() {
            self.combination.push(self.candidates[i]);
            self.sum += self.candidates[i];
            match self.sum.cmp(&self.target) {
                Ordering::Equal => {
                    self.output.push(self.combination.clone());
                    self.combination.pop();
                    self.sum -= self.candidates[i];
                    return;
                }
                Ordering::Greater => {
                    self.combination.pop();
                    self.sum -= self.candidates[i];
                    return;
                }
                Ordering::Less => {
                    self.backtrace(i);
                    self.combination.pop();
                    self.sum -= self.candidates[i];
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::combination_sum(vec![2], 1),
            Vec::<Vec<i32>>::new()
        );
    }
}
