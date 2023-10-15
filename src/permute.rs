use crate::Solution;

impl Solution {
    /// # [46. Permutations](https://leetcode.com/problems/permutations/)
    ///
    /// Given an array `nums` of distinct integers, return all the possible permutations. You can return the answer in **any order** .
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [1,2,3]
    /// Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [0,1]
    /// Output: [[0,1],[1,0]]
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: nums = [1]
    /// Output: [[1]]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 6`
    /// - `-10 <= nums[i] <= 10`
    /// - All the integers of `nums` are **unique** .
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Permute::new(nums).permute()
    }
}

struct Permute {
    nums: Vec<i32>,
    mask: Vec<bool>,
    permutations: Vec<i32>,
    result: Vec<Vec<i32>>,
}

impl Permute {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        Permute {
            nums,
            mask: vec![true; n],
            permutations: vec![],
            result: vec![],
        }
    }

    fn permute(mut self) -> Vec<Vec<i32>> {
        for i in 0..self.nums.len() {
            self.backtrack(i);
        }
        self.result
    }

    fn backtrack(&mut self, index: usize) {
        let mask = &self.mask as *const Vec<bool>;
        let indexes = (0..self.mask.len()).filter(|i| unsafe { (*mask)[*i] });

        self.permutations.push(index as i32);
        self.mask[index] = false;
        if self.permutations.len() == self.mask.len() {
            self.result.push(
                self.permutations
                    .iter()
                    .map(|i| self.nums[*i as usize])
                    .collect(),
            );
        } else {
            for i in indexes {
                self.backtrack(i);
            }
        }
        self.permutations.pop();
        self.mask[index] = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 3];
        assert_eq!(
            Solution::permute(nums),
            vec![
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
        );
    }

    #[test]
    fn example2() {
        let nums = vec![0, 1];
        assert_eq!(Solution::permute(nums), vec![[0, 1], [1, 0]]);
    }

    #[test]
    fn example3() {
        let nums = vec![1];
        assert_eq!(Solution::permute(nums), vec![[1]]);
    }
}
