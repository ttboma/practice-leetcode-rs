use std::collections::HashMap;

use crate::Solution;

impl Solution {
    /// # [169. Majority Element](https://leetcode.com/problems/majority-element/)
    ///
    /// Given an array `nums` of size `n`, return the majority element.
    ///
    /// The majority element is the element that appears more than `⌊n / 2⌋` times. You may assume that the majority element always exists in the array.
    ///
    /// **Example 1:**
    ///
    /// ```
    /// Input: nums = [3,2,3]
    /// Output: 3
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```
    /// Input: nums = [2,2,1,1,1,2,2]
    /// Output: 2
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `n == nums.length`
    /// - `1 <= n <= 5 * 10^4`
    /// - `-10^9 <= nums[i] <= 10^9`
    ///  
    /// **Follow-up:**  Could you solve the problem in linear time and in `O(1)` space?
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let majority = nums.len() / 2;
        let mut count = HashMap::<i32, usize>::new();
        for element in nums {
            match count.get_mut(&element) {
                Some(n) => {
                    if *n == majority {
                        return element;
                    }
                    *n += 1;
                }
                None => {
                    count.insert(element, 1);
                }
            }
        }
        *count.iter().max_by_key(|(&key, _)| key).unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![3, 2, 3];
        let output = 3;
        assert_eq!(Solution::majority_element(nums), output);
    }

    #[test]
    fn example2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let output = 2;
        assert_eq!(Solution::majority_element(nums), output);
    }
}
