use super::*;
use std::collections::HashMap;

impl Solution {
    /// # [219. Contains Duplicate II](https://leetcode.com/problems/contains-duplicate-ii/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// Given an integer array `nums` and an integer `k`, return `true` if there are two **distinct indices**  `i` and `j` in the array such that `nums[i] == nums[j]` and `abs(i - j) <= k`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [1,2,3,1], k = 3
    /// Output: true
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [1,0,1,1], k = 1
    /// Output: true
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: nums = [1,2,3,1,2,3], k = 2
    /// Output: false
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 10^5`
    /// - `-10^9 <= nums[i] <= 10^9`
    /// - `0 <= k <= 10^5`
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut m = HashMap::new();
        for (i, e) in nums.iter().enumerate() {
            if let Some(v) = m.insert(*e, i) {
                if i - v <= k as usize {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
            true
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
            false
        );
    }
}
