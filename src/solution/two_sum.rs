use super::*;
use std::collections::HashMap;

impl Solution {
    /// # [1. Two Sum](https://leetcode.com/problems/two-sum/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an array of integers `nums`and an integer `target`, return indices of the two numbers such that they add up to `target`.
    ///
    /// You may assume that each input would have **exactly one solution** , and you may not use the same element twice.
    ///
    /// You can return the answer in any order.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [2,7,11,15], target = 9
    /// Output: [0,1]
    /// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [3,2,4], target = 6
    /// Output: [1,2]
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: nums = [3,3], target = 6
    /// Output: [0,1]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `2 <= nums.length <= 10^4`
    /// - `-10^9 <= nums[i] <= 10^9`
    /// - `-10^9 <= target <= 10^9`
    /// - **Only one valid answer exists.**
    ///
    /// **Follow-up:** Can you come up with an algorithm that is less than `O(n^2)`time complexity?
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, usize>::new();
        for (i, &e) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - e)) {
                return vec![j as i32, i as i32];
            } else {
                map.insert(e, i);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn example2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);
    }

    #[test]
    fn example3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}
