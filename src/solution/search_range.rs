use crate::*;
use std::cmp::Ordering;

impl Solution {
    /// # [34. Find First and Last Position of Element in Sorted Array](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an array of integers `nums` sorted in non-decreasing order, find the starting and ending position of a given `target` value.
    ///
    /// If `target` is not found in the array, return `[-1, -1]`.
    ///
    /// You must write an algorithm with`O(log n)` runtime complexity.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [5,7,7,8,8,10], target = 8
    /// Output: [3,4]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [5,7,7,8,8,10], target = 6
    /// Output: [-1,-1]
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: nums = [], target = 0
    /// Output: [-1,-1]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `0 <= nums.length <= 10^5`
    /// - `-10^9<= nums[i]<= 10^9`
    /// - `nums` is a non-decreasing array.
    /// - `-10^9<= target<= 10^9`
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        if let Some(m) = Self::search_range_recursive(&nums, 0, n, target) {
            let s = nums[0..m]
                .iter()
                .rposition(|&x| x != target)
                .map_or(0, |i| i + 1);
            let l = nums[m..n]
                .iter()
                .position(|&x| x != target)
                .map_or(n - 1, |i| m + i - 1);
            vec![s as i32, l as i32]
        } else {
            vec![-1, -1]
        }
    }

    fn search_range_recursive(
        nums: &[i32],
        start: usize,
        end: usize,
        target: i32,
    ) -> Option<usize> {
        match start.cmp(&end) {
            Ordering::Equal => None,
            Ordering::Greater => unreachable!(),
            Ordering::Less => {
                let mid = start + (end - start) / 2;
                match nums[mid].cmp(&target) {
                    Ordering::Equal => Some(mid),
                    Ordering::Less => Self::search_range_recursive(nums, mid + 1, end, target),
                    Ordering::Greater => Self::search_range_recursive(nums, start, mid, target),
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
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let result = vec![3, 4];
        assert_eq!(Solution::search_range(nums, target), result);
    }

    #[test]
    fn example2() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        let result = vec![-1, -1];
        assert_eq!(Solution::search_range(nums, target), result);
    }

    #[test]
    fn example3() {
        let nums = vec![];
        let target = 0;
        let result = vec![-1, -1];
        assert_eq!(Solution::search_range(nums, target), result);
    }

    #[test]
    fn example4() {
        let nums = vec![1];
        let target = 1;
        let result = vec![0, 0];
        assert_eq!(Solution::search_range(nums, target), result);
    }
}
