use crate::Solution;

impl Solution {
    /// # [704. Binary Search](https://leetcode.com/problems/binary-search/)
    ///
    /// Given an array of integers `nums` which is sorted in ascending order, and an integer `target`, write a function to search `target` in `nums`. If `target` exists, then return its index. Otherwise, return `-1`.
    ///
    /// You must write an algorithm with `O(log n)` runtime complexity.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [-1,0,3,5,9,12], target = 9
    /// Output: 4
    /// Explanation: 9 exists in nums and its index is 4
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [-1,0,3,5,9,12], target = 2
    /// Output: -1
    /// Explanation: 2 does not exist in nums so return -1
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 10^4`
    /// - `-10^4 < nums[i], target < 10^4`
    /// - All the integers in `nums` are **unique** .
    /// - `nums` is sorted in ascending order.
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        let mut j = nums.len();
        if j == 0 {
            return -1;
        }
        while i != j {
            let m = i + (j - i) / 2;
            match nums[m].cmp(&target) {
                std::cmp::Ordering::Less => i = m + 1,
                std::cmp::Ordering::Greater => j = m,
                std::cmp::Ordering::Equal => {
                    return m as i32;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        assert_eq!(Solution::search(nums, target), 4);
    }

    #[test]
    fn example2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        assert_eq!(Solution::search(nums, target), -1);
    }
}
