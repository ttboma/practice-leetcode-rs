use crate::Solution;

impl Solution {
    /// # [78. Subsets](https://leetcode.com/problems/subsets/)
    ///
    /// Given an integer array `nums` of **unique**  elements, return all possible <div class="popover-wrapper inline-block" data-headlessui-state=""><div aria-expanded="false" data-headlessui-state="" id="headlessui-popover-button-:r18:">subsets<div style="position: fixed; z-index: 40; inset: 0px auto auto 0px; transform: translate(490px, 221px);"> (the power set).
    ///
    /// The solution set **must not**  contain duplicate subsets. Return the solution in **any order** .
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [1,2,3]
    /// Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [0]
    /// Output: [[],[0]]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 10`
    /// - `-10 <= nums[i] <= 10`
    /// - All the numbers of`nums` are **unique** .
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Subsets::new(nums).solve()
    }
}

#[derive(Default)]
struct Subsets {
    nums: Vec<i32>,
    set: Vec<i32>,
    output: Vec<Vec<i32>>,
}

impl Subsets {
    fn new(nums: Vec<i32>) -> Self {
        Subsets {
            nums,
            set: Vec::new(),
            output: vec![vec![]; 1],
        }
    }

    fn solve(mut self) -> Vec<Vec<i32>> {
        for i in 0..self.nums.len() {
            self.dfs(i as i32);
        }
        self.output
    }

    fn dfs(&mut self, i: i32) {
        self.set.push(i);
        for j in i + 1..self.nums.len() as i32 {
            self.dfs(j);
        }
        let value = self
            .set
            .iter()
            .map(|&idx| self.nums[idx as usize])
            .collect();
        self.output.push(value);
        self.set.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 3];
        assert_eq!(
            Solution::subsets(nums),
            vec![
                vec![],
                vec![1, 2, 3],
                vec![1, 2],
                vec![1, 3],
                vec![1],
                vec![2, 3],
                vec![2],
                vec![3],
            ]
        );
    }

    #[test]
    fn example2() {
        let nums = vec![0];
        assert_eq!(Solution::subsets(nums), vec![vec![], vec![0]]);
    }
}
