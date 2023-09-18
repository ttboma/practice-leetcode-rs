use std::collections::HashSet;

use crate::Solution;

impl Solution {
    /// # [217. Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)
    ///
    /// Given an integer array `nums`, return `true` if any value appears **at least twice**  in the array, and return `false` if every element is distinct.
    ///
    /// **Example 1:**
    ///
    /// ```text
    /// Input: nums = [1,2,3,1]
    /// Output: true
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```text
    /// Input: nums = [1,2,3,4]
    /// Output: false
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```text
    /// Input: nums = [1,1,1,3,3,4,3,2,4,2]
    /// Output: true
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 10^5`
    /// - `-10^9 <= nums[i] <= 10^9`
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut count = HashSet::<i32>::new();
        for n in nums {
            if count.contains(&n) {
                return true;
            }
            count.insert(n);
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 3, 1];
        let output = true;
        assert_eq!(Solution::contains_duplicate(nums), output);
    }

    #[test]
    fn example2() {
        let nums = vec![1, 2, 3, 4];
        let output = false;
        assert_eq!(Solution::contains_duplicate(nums), output);
    }

    #[test]
    fn example3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let output = true;
        assert_eq!(Solution::contains_duplicate(nums), output);
    }
}
